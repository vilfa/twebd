use crate::{
    cli::{Build, CliOpt, Other},
    log::{LogLevel, LogRecord, LoggerConfigureMessage},
    srv::default_threads,
    syn::{LogWorker, Message, ThreadPoolError, Tx, Worker},
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
    _other: Vec<CliOpt>,
}

impl Build<Self, ThreadPool, ThreadPoolError> for ThreadPoolBuilder {
    fn new(opts: Vec<CliOpt>) -> Self {
        let mut thread_pool_builder = Self::default();
        for opt in opts {
            match opt {
                CliOpt::Verbosity(v) => thread_pool_builder.log_level = v,
                CliOpt::Threads(v) => thread_pool_builder.pool_size = v,
                CliOpt::ShowLoglevel(v) => thread_pool_builder.show_loglevel = v,
                CliOpt::ShowTimestamp(v) => thread_pool_builder.show_timestamp = v,
                cli_opt => thread_pool_builder.add_other(cli_opt.to_owned()),
            }
        }
        thread_pool_builder
    }
    fn build(&self) -> Result<ThreadPool, ThreadPoolError> {
        let thread_pool = ThreadPool::new(self.pool_size);
        thread_pool.log_conf(LoggerConfigureMessage::SetLogLevel(self.log_level));
        thread_pool.log_conf(LoggerConfigureMessage::ShowLogLevel(self.show_loglevel));
        thread_pool.log_conf(LoggerConfigureMessage::ShowTimestamp(self.show_timestamp));
        Ok(thread_pool)
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
            log_level: LogLevel::default(),
            show_loglevel: true,
            show_timestamp: true,
            pool_size: default_threads(),
            _other: Vec::new(),
        }
    }
}
