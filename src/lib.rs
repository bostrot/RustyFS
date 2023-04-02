use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

// Import logging.rs file
mod logging;
use logging::Logging;

// Import macros.rs file
#[macro_use]
mod macros;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        debug!("Sending job to thread pool");

        match self.sender.as_ref() {
            Some(sender) => {
                match sender.send(job) {
                    Ok(_) => (),
                    Err(_) => error!("Error sending job to thread pool"),
                }
            },
            None => panic!("Sender is None"),
        };
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            debug!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                match thread.join() {
                    Ok(_) => (),
                    Err(_) => error!("Joining thread"),
                }
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        
        let builder = thread::Builder::new();

        let thread = builder.spawn(move || loop {
            let message = match receiver.lock() {
                Ok(receiver) => receiver.recv(),
                Err(_) => break,
            };

            match message {
                Ok(job) => {
                    debug!("Worker {id} got a job; executing.");

                    job();
                }
                Err(_) => {
                    debug!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        let thread: Option<thread::JoinHandle<()>> = match thread {
            Ok(thread) => Some(thread),
            Err(_) => None,
        };

        Worker { id, thread }
    }
}