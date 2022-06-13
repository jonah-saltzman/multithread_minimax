use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::{self, Receiver};
use num_cpus;

pub struct ThreadPool {
    workers: Vec<Worker>,
    tx: mpsc::Sender<Job>
}

type Job = Box<dyn FnOnce() -> () + Send + 'static>;

struct Worker {
    id: usize,
    handle: thread::JoinHandle<()>
}

impl Worker {
    fn new(id: usize, rx: Arc<Mutex<Receiver<Job>>>) -> Worker {
        Worker { id, handle: thread::spawn(move || loop { 
            let job = rx.lock().unwrap().recv().unwrap();
            job();
        }) }
    }
}

impl ThreadPool {
    pub fn new(mut size: usize) -> ThreadPool {
        if size == 0 { size = num_cpus::get(); }
        let mut workers = Vec::with_capacity(size);
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&rx)))
        }
        ThreadPool { workers, tx }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() -> () + Send + 'static,
        {
            self.tx.send(Box::new(f)).unwrap();
        }
}