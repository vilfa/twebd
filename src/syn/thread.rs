use super::{
    message::Message,
    worker::{LogWorker, Tx, Worker},
};
use crate::{
    cli::CliOpt,
    log::{LogLevel, LoggerConfigureMessage},
    srv::server::Server,
};
use std::sync::{mpsc, Arc, Mutex};

pub struct ThreadPool {
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
    pub fn log(&self, log_level: LogLevel, msg: String) {
        self.log_worker
            .sender
            .send(Message::Log(log_level, msg))
            .unwrap();
    }
    pub fn log_conf(&self, conf: LoggerConfigureMessage) {
        self.log_worker
            .sender
            .send(Message::LogConfigure(conf))
            .unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.log_worker
            .sender
            .send(Message::Log(
                LogLevel::Debug,
                format!("sending terminate message to all workers"),
            ))
            .unwrap();

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            self.log_worker
                .sender
                .send(Message::Log(
                    LogLevel::Debug,
                    format!("shutting down worker {}", worker.id),
                ))
                .unwrap();
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

pub struct ThreadPoolBuilder {
    logworker_level: LogLevel,
    pool_size: usize,
}

impl ThreadPoolBuilder {
    pub fn new(opts: Vec<CliOpt>) -> ThreadPoolBuilder {
        let mut pool_builder = ThreadPoolBuilder::default();
        for opt in opts {
            match opt {
                CliOpt::Verbosity(v) => pool_builder.logworker_level = v,
                CliOpt::Threads(t) => pool_builder.pool_size = t,
                _ => {}
            }
        }

        pool_builder
    }
    pub fn thread_pool(&self) -> ThreadPool {
        let pool = ThreadPool::new(self.pool_size);
        pool.log_conf(LoggerConfigureMessage::SetLogLevel(self.logworker_level));
        pool
    }
}

impl Default for ThreadPoolBuilder {
    fn default() -> Self {
        ThreadPoolBuilder {
            logworker_level: LogLevel::default(),
            pool_size: Server::default_threads(),
        }
    }
}
