use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Worker {
    pub id: usize,
    pub thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                match receiver.lock().unwrap().recv() {
                    Ok(job) => {
                        println!("Worker {} got a job; excuting.", id);
                        job();
                    },
                    Err(e) => {
                        panic!(e)
                    },
                }
            }
        });
        Worker { id, thread }
    }
}

pub struct PoolCreationError;
type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    pub workers: Vec<Worker>,
    pub sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// 实例化线程池。
    /// `size<=0` 时没有意义，会触发 panic 
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            //  create some worker
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool { workers, sender })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
        println!("has sended...");
    }
}
