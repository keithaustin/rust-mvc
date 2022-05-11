// Imports
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

// Import other lib modules
use crate::request::Request;
pub mod request;
use crate::response::Response;
pub mod response;
use crate::router::Router;
pub mod router;
use crate::server::Server;
pub mod server;


/// Message types for sending jobs to workers
enum Message {
    NewJob(Job),
    Terminate
}

/// Worker creates a thread and awaits jobs to perform
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// Creates a new Worker
    ///
    /// id = ID of the Worker
    /// receiver = Receiving side of channel for getting messages
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        // Spawn thread
        let thread = thread::spawn(move || loop {
            // Get message
            let message = receiver.lock().unwrap().recv().unwrap();

            // Perform task from message
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job! Executing now.", id);
                    job();
                },
                Message::Terminate => {
                    println!("Worker {} terminating.", id);
                    break;
                }
            }
        });

        Worker { id, thread: Some(thread) }
    }
}

// Type alias for execute closures
type Job = Box<dyn FnOnce() + Send + 'static>;

/// ThreadPool contains and sends tasks to Workers
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    /// Create a new Thread Pool
    ///
    /// size = Number of threads in the pool.
    ///
    /// # Panics
    /// 
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        // Ensure size is at least 1
        assert!(size > 0);

        // Create a channel
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        // Create workers
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    /// Executes a request with the given closure
    /// 
    /// f = The closure to execute
    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    /// Stops and removes threads from Workers
    fn drop(&mut self) {
        println!("Shutting down all workers.");

        // Send terminate message to all Workers
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        // Take posession of each worker's thread and join it
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
