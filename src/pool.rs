use std::sync::{Arc, Mutex};
use std::thread::{self, Thread};
use std::sync::mpsc::{self, Receiver};
use num_cpus;

pub struct ThreadPool {
    tx: mpsc::Sender<Job>
}

type Job = Box<dyn FnOnce() -> () + Send + 'static>;

struct Worker(thread::JoinHandle<()>);

impl Worker {
    fn new(id: usize, rx: Arc<Mutex<Receiver<Job>>>, main: Arc<Thread>) -> Worker {
        Worker (thread::spawn(move || loop {
            let job = rx.lock().unwrap().recv();
            if job.is_ok() {
                job.unwrap()();
                main.unpark();
            }
        }))
    }
}

impl ThreadPool {
    pub fn new(mut size: usize, main: Arc<Thread>) -> ThreadPool {
        if size == 0 { size = num_cpus::get(); }
        println!("using {} threads", size);
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        for i in 0..size {
            Worker::new(i, Arc::clone(&rx), Arc::clone(&main));
        }
        ThreadPool { tx }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() -> () + Send + 'static,
        {
            self.tx.send(Box::new(f)).unwrap();
        }
}