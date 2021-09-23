use crate::{
    cli::{Build, CliOpt, Other},
    srv::default_threads,
    syn::{Message, ThreadPoolError, Tx, Worker},
};
use log::debug;
use std::sync::{mpsc, Arc, Mutex};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Tx,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel::<Message>();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 1..size + 1 {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::Job(job)).unwrap();
    }
    pub fn size(&self) -> usize {
        self.workers.len()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        debug!("sending terminate message to all workers");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            debug!("shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

pub struct ThreadPoolBuilder {
    pool_size: usize,
    _other: Vec<CliOpt>,
}

impl Build<Self, ThreadPool, ThreadPoolError> for ThreadPoolBuilder {
    fn new(opts: Vec<CliOpt>) -> Self {
        let mut thread_pool_builder = Self::default();
        for opt in opts {
            match opt {
                CliOpt::Threads(v) => thread_pool_builder.pool_size = v,
                cli_opt => thread_pool_builder.add_other(cli_opt.to_owned()),
            }
        }
        thread_pool_builder
    }
    fn build(&self) -> Result<ThreadPool, ThreadPoolError> {
        Ok(ThreadPool::new(self.pool_size))
    }
}

impl Other for ThreadPoolBuilder {
    fn add_other(&mut self, o: CliOpt) {
        self._other.push(o);
    }
    fn other(&self) -> Vec<CliOpt> {
        self._other.to_vec()
    }
}

impl Default for ThreadPoolBuilder {
    fn default() -> Self {
        ThreadPoolBuilder {
            pool_size: default_threads(),
            _other: Vec::new(),
        }
    }
}
