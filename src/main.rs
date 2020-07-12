use std::net::TcpListener;
use std::net::{TcpStream};
use std::io::{Read, Write};
use std::fs;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Mutex, Arc};

fn main() {
    server();
}

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            //  create some worker
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        
        Ok(ThreadPool {
            workers,
            sender,
        })
    }

    fn execute<F>(&self, f: F) 
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move ||{
            while let job =  receiver.lock().unwrap().recv().unwrap() {
                println!("Worker {} got a job; excuting.", id);
                job();
            }
        });
        Worker { id, thread}
    }
}

struct PoolCreationError;
type Job = Box<dyn FnOnce() + Send + 'static>;

fn server() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    let pool = if let Ok(pool) = ThreadPool::new(4) {
        pool
    } else {
        panic!("new threadpool failed.")
    };
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // thread::spawn(|| {
        //     handle_connection(stream);
        // });
        pool.execute(|| {
            handle_connection(stream);
        })
    }
}

fn handle_connection(mut stream: TcpStream) {
    let get = b"GET / HTTP/1.1\n";
    let sleep = b"GET /sleep HTTP/1.1\n";

    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")   
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string("hello.html").unwrap();
    let response = format!("HTTP/1.1 200 OK\n\n{}", contents);
    println!("{}", response);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
