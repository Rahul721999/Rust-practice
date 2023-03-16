use anyhow::{anyhow, Result};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

const MAX_THREAD_LIMIT: usize = 5;
struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}


impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver
                .lock()
                // have to use Mutex locking mechanism to use the data
                .expect("failed to lock the reciver") 
                .recv();

            match message{
                Ok(job) => {
                    println!("worker {id} got a job, Executing...");
                    job();
                }
                Err(_) =>{
                    println!("worker {id} disconnected; shutting down..");
                    break;
                }
            };
        });
        Worker { id: id, thread : Some(thread) }
    }
}

// struct Job;
type Job = Box<dyn FnOnce() + Send + 'static>;
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool> {
        match size {
            0 => return Err(anyhow!("zero thread cannot be created")),
            n => {
                if n >= MAX_THREAD_LIMIT {
                    return Err( anyhow!("Max Thread Spawn Limit is < 5"));
                } else {
                    // creating a mpsc channel to pass the data between thread
                    let (sender, reciever) = mpsc::channel();
                    /*  wrapping reciever therad with Arc pointer
                        cause the Job doesn't impl copy trait */
                    let reciever = Arc::new(Mutex::new(reciever));
                    let mut threads = Vec::with_capacity(size);
                    for i in 0..size {
                        // creating thread and pushing to the vector
                        threads.push(Worker::new(i, reciever.clone()));
                    }
                    Ok(ThreadPool {
                        workers: threads,
                        sender : Some(sender),
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
        self.sender.as_ref().expect("failed to get the sender as ref").send(job).expect("failed to send the job to the channel");
    }
    
}

// impl the Drop trait for ThreadPool
impl Drop for ThreadPool{
    fn drop(&mut self) {
        // drop sender before before witing for the thread to be finished
        drop(self.sender.take());
        for worker in &mut self.workers{
            println!("sutting down worker {}",worker.id);

            // worker.thread.expect("Got None value in worker.thread").join().expect("Failed to join the thread");
            if let Some(thread) = worker.thread.take() {
                thread.join().expect("failed to join the thread");
            }
        }
    }
}
