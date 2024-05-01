// I'll try to describe each process inside this file cause is actually complex
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>, // this sender
                                       // will send every job we get from any request
}
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of thread is the pool.
    ///
    /// #Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        // creates a vec with the size we want
        let mut workers = Vec::with_capacity(size);
        // creates a channel
        let (sender, receiver) = mpsc::channel();
        // we override/shadow the receiver variable for it contains
        // a conable and mutable reciever, since recievers by default
        // can only exists once a time
        let receiver = Arc::new(Mutex::new(receiver));
        for id in 0..=size {
            // creates multiple owned references of a reciever for threads to be able
            // of listening on the same channel
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        // return the pool
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }
    // pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
    //     todo!();
    // }
}

impl ThreadPool {
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // we store an the arrived job/request
        let job = Box::new(f);
        // we send throuhg our channel the job
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // creates a the thread and saves the handler of that thread
        let thread = thread::spawn(move || loop {
            // message as an analogy of listenig a channel
            // but in fact we are recieving the job
            // I'll inquire more here cause this is the key part of our program for it to work
            let message = receiver.lock().unwrap().recv(); // calling
                                                           // lock() will lock the channel for our thread starts
                                                           // listening the channel, then
                                                           // unwrap will retrieve the mutex of our reciever in the channel
                                                           // then we wait for the reciever to hold something calling recv()
                                                           // recv() will wait till something to be inside the channel
                                                           // once our channel/reciever holds a job
                                                           // the job will be moved out of the channel
                                                           // so other threads will be waiting again
                                                           // for our reciever/channel to holds a job
                                                           // recv() will also unlock the reciever for other threads
                                                           // start listening the reciever and wait for it
                                                           // holds a job too

            match message {
                // so if we got a job from the reciever we execute the job
                // job will be executed asynchronously cause we are calling the job conainted
                // in our channel before
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job(); // we make the call to our job
                }
                Err(_) => {
                    // we break the loop if there is an error
                    println!("Worker {id} disconnected; shutting down...");
                    break;
                }
            };
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take()); // we drop our sender
                                  // so no more jobs for our threads
        for worker in &mut self.workers {
            // we shut down our workers
            println!("Shutting down worker {}", worker.id);
            // taking the ownerwhip of our thread
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap(); // waiting for our thread to
                                        // evaluate an error inside it and breaks the loop
            }
        }
    }
}
