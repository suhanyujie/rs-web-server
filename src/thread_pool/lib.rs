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
                let guard1 = receiver.lock().unwrap();
                match guard1.recv() {
                    Ok(job) => {
                        println!("Worker {} got a job; excuting.", id);
                        // 确保了 recv 调用过程中持有锁，而在 job.call_box() 调用前锁就被释放
                        // 只有这样才能允许并发处理多个请求
                        drop(guard1);
                        job.call_box();
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

pub trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<Self>) {
        (*self)();
    }
}

pub struct PoolCreationError;
type Job = Box<FnBox + Send + 'static>;

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
