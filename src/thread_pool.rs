use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self, JoinHandle},
};

type Receiver = Arc<Mutex<mpsc::Receiver<Message>>>;
type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    /// Initializes a new worker thread.
    ///
    /// ## Parameters
    ///
    /// `id` is the identifier of the worker thread.
    /// `receiver` is the receiving end of the channel used to communicate between threads.
    fn new(id: usize, receiver: Receiver) -> Self {
        let thread = thread::spawn(move || loop {
            // The call to recv blocks, so if there is no job yet,
            // the current thread will wait until a job becomes available.
            let message = receiver
                .lock()
                .expect("failed to acquire mutex")
                .recv()
                .expect("channel got hung up");

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} received a job. Executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

/// Message sent between threads using channels.
///
/// A message can either be:
/// - `NewJob` if a worker thread is consuming a new job, or
/// - `Terminate` if a worker thread is told by the thread pool to stop working
enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    /// Initializes a new thread pool.
    ///
    /// # Parameters
    ///
    /// `size`: The size of the thread pool
    ///
    /// # Panics
    ///
    /// Panics if the size of the thread pool equals 0.
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    /// Executes a job provided as a closure.
    ///
    /// The closure is sent from one thread to another using the `Send` trait.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender
            .send(Message::NewJob(job))
            .expect("failed to send job");
    }
}

/// Cleans up the thread pool.
///
/// The worker threads are told to terminate, and their threads are joined.
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            self.sender
                .send(Message::Terminate)
                .unwrap_or_else(|_| panic!("failed to terminate worker {}", worker.id));
        }

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread
                    .join()
                    .expect(&format!("failed to join worker thread {}", worker.id)[..]);
            }
        }
    }
}
