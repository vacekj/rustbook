use std::thread;
use std::thread::JoinHandle;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            todo!("Create thread and store theme")
        }

        ThreadPool { threads }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

pub struct Worker {
    id: usize,
    handle: JoinHandle<()>,
}

impl Worker {}
