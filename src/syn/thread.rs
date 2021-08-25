use super::{
    message::Message,
    worker::{LogWorker, Tx, Worker},
};
use crate::log::LogLevel;
use std::sync::{mpsc, Arc, Mutex};

struct ThreadPool {
    workers: Vec<Worker>,
    log_worker: LogWorker,
    sender: Tx,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let log_worker = LogWorker::new(0);
        let (sender, receiver) = mpsc::channel::<Message>();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 1..size + 1 {
            workers.push(Worker::new(
                id,
                Arc::clone(&receiver),
                log_worker.sender.clone(),
            ));
        }

        ThreadPool {
            workers,
            log_worker,
            sender,
        }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::Job(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.log_worker.sender.send(Message::Log(
            LogLevel::Debug,
            format!("sending terminate message to all workers"),
        ));

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            self.log_worker.sender.send(Message::Log(
                LogLevel::Debug,
                format!("shutting down worker {}", worker.id),
            ));
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }

        self.log_worker.sender.send(Message::Terminate).unwrap();
        if let Some(thread) = self.log_worker.thread.take() {
            thread.join().unwrap();
        }
    }
}
