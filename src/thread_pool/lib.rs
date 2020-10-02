use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Worker {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                // let guard1 = receiver.lock().unwrap();
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; excuting.", id);
                        // 确保了 recv 调用过程中持有锁，而在 job.call_box() 调用前锁就被释放
                        // 只有这样才能允许并发处理多个请求
                        // drop(guard1);
                        // 不太清楚的是：在调用 call_box 前，为何 message 会被 drop 掉？ 
                        job.call_box();
                    },
                    Message::Terminate => {
                        // 平滑退出
                        println!("Worker {} was told to terminate", id);
                        break;
                    },
                }
            }
        });
        Worker { id, thread: Some(thread) }
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
    pub sender: mpsc::Sender<Message>,
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
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("shutdown all workers");
        for worker in &mut self.workers {
            println!("shut down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

pub enum Message {
    NewJob(Job),
    Terminate,
}
