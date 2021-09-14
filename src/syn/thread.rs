use super::{
    message::Message,
    worker::{LogWorker, Tx, Worker},
};
use crate::{
    cli::CliOpt,
    log::{LogLevel, LogRecord, LoggerConfigureMessage},
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
    pub fn log(&self, record: LogRecord) {
        self.log_worker.sender.send(Message::Log(record)).unwrap()
    }
    pub fn log_conf(&self, conf: LoggerConfigureMessage) {
        self.log_worker
            .sender
            .send(Message::LogConfigure(conf))
            .unwrap();
    }
    pub fn size(&self) -> (usize, usize) {
        (self.workers.len(), 1)
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.log_worker
            .sender
            .send(Message::Log(LogRecord::new(
                LogLevel::Debug,
                format!("sending terminate message to all workers"),
            )))
            .unwrap();

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            self.log_worker
                .sender
                .send(Message::Log(LogRecord::new(
                    LogLevel::Debug,
                    format!("shutting down worker {}", worker.id),
                )))
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
    log_level: LogLevel,
    show_loglevel: bool,
    show_timestamp: bool,
    pool_size: usize,
    other: Vec<CliOpt>,
}

impl ThreadPoolBuilder {
    pub fn new(opts: Vec<CliOpt>) -> ThreadPoolBuilder {
        let mut pool_builder = Self::default();
        let opts_filtered = Self::filter(&opts);
        for opt in opts_filtered.0 {
            match opt {
                CliOpt::Verbosity(v) => pool_builder.log_level = v,
                CliOpt::Threads(v) => pool_builder.pool_size = v,
                CliOpt::ShowLoglevel(v) => pool_builder.show_loglevel = v,
                CliOpt::ShowTimestamp(v) => pool_builder.show_timestamp = v,
                _ => {}
            }
        }
        pool_builder.other = opts_filtered.1;

        pool_builder
    }
    pub fn thread_pool(&self) -> ThreadPool {
        let pool = ThreadPool::new(self.pool_size);
        pool.log_conf(LoggerConfigureMessage::SetLogLevel(self.log_level));
        pool.log_conf(LoggerConfigureMessage::ShowLogLevel(self.show_loglevel));
        pool.log_conf(LoggerConfigureMessage::ShowTimestamp(self.show_timestamp));
        pool
    }
    pub fn other(&self) -> Vec<CliOpt> {
        self.other.to_vec()
    }
    fn filter(opts: &Vec<CliOpt>) -> (Vec<CliOpt>, Vec<CliOpt>) {
        (
            opts.iter()
                .filter(|opt| !matches!(opt, CliOpt::Directory(_)))
                .cloned()
                .collect(),
            opts.iter()
                .filter(|opt| matches!(opt, CliOpt::Directory(_)))
                .cloned()
                .collect(),
        )
    }
}

impl Default for ThreadPoolBuilder {
    fn default() -> Self {
        ThreadPoolBuilder {
            log_level: LogLevel::default(),
            show_loglevel: true,
            show_timestamp: true,
            pool_size: Server::default_threads(),
            other: Vec::new(),
        }
    }
}
