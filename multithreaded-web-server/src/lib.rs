use anyhow::{anyhow, Result};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

const MAX_THREAD_LIMIT: usize = 5;
struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}


impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let job = receiver
                .lock()
                .expect("failed to lock the reciver") // have to use Mutex locking mechanism
                .recv()
                .expect("failed on recv method");

            println!("Worker {id} got a job; executing...");
            job();
        });
        Worker { id: id, thread }
    }
}

// struct Job;
type Job = Box<dyn FnOnce() + Send + 'static>;
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, anyhow::Error> {
        match size {
            0 => return Err(anyhow!("zero thread cannot be created")),
            n => {
                if n >= MAX_THREAD_LIMIT {
                    return Err(anyhow!("Max Thread Spawn Limit is < 5"));
                } else {
                    // creating a mpsc channel to pass the data between thread
                    let (sender, reciever) = mpsc::channel();
                    // wrapping reciever therad with Arc pointer
                    // cause the Job doesn't impl copy trait
                    let reciever = Arc::new(Mutex::new(reciever));
                    let mut threads = Vec::with_capacity(size);
                    for i in 0..size {
                        // creating thread and pushing to the vector
                        threads.push(Worker::new(i, reciever.clone()));
                    }
                    Ok(ThreadPool {
                        workers: threads,
                        sender,
                    })
                }
            }
        }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + 'static + Send,
    {
        let job = Box::new(f);
        self.sender.send(job).expect("failed to send the job to the channel");
    }
}
