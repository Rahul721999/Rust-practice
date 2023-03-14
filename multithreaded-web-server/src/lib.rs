use std::{thread::{JoinHandle, self}};
use anyhow::{Result, anyhow};

const MAX_THREAD_LIMIT : usize = 4;
pub struct ThreadPool{
    thread : Vec<thread::JoinHandle<()>>
}

impl ThreadPool{
    pub fn new(size: usize) -> Result<ThreadPool, anyhow::Error>{
        match size{
            0 => return Err(anyhow!("zero thread cannot be created")),
            n => {
                    if n >= MAX_THREAD_LIMIT{
                    return Err(anyhow!("Thread is not available"));
                }
                else{
                    let mut threads = Vec::with_capacity(size);
                    for n in 0..size{
                        let t = ThreadPool::spawn(size);
                        threads.push(t);
                    }
                    return Ok(ThreadPool { thread: threads})
                }
            }
        }
    }
    pub fn execute<F>(&self, f: F)
    where
        F : FnOnce() + 'static + Send,
        {

        }
    
    pub fn spawn<F, T>(f : F) -> JoinHandle<T>
    where  
        F : FnOnce() -> T,
        F : Send + 'static,
        T : Send + 'static,
    {

    } 
}