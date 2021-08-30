use super::message::Message;
use crate::log::{
    config::Configure,
    logger::{Log, Logger},
    LogLevel, LogRecord, LoggerConfigureMessage,
};
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub type Job = Box<dyn FnOnce() + Send + 'static>;
pub type Rx = Arc<Mutex<mpsc::Receiver<Message>>>;
pub type Tx = mpsc::Sender<Message>;

pub struct Worker {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Rx, logger: Tx) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver
                .lock()
                .expect("failed to acquire thread message lock!")
                .recv()
                .unwrap();

            match message {
                Message::Job(job) => {
                    logger
                        .send(Message::Log(LogRecord::new(
                            LogLevel::Debug,
                            format!("worker {} got a job. executing", id),
                        )))
                        .unwrap();
                    job();
                }
                Message::Terminate => {
                    logger
                        .send(Message::Log(LogRecord::new(
                            LogLevel::Debug,
                            format!("worker {} got a terminate message. terminating", id),
                        )))
                        .unwrap();
                    break;
                }
                _ => {}
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

pub struct LogWorker {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
    pub sender: Tx,
}

impl LogWorker {
    pub fn new(id: usize) -> LogWorker {
        let (sender, receiver) = mpsc::channel::<Message>();
        let mut logger = Logger::new();
        let thread = thread::spawn(move || loop {
            let message = receiver.recv().unwrap();
            match message {
                Message::Log(record) => logger.log(record),
                Message::LogConfigure(c) => match c {
                    LoggerConfigureMessage::SetLogLevel(v) => {
                        logger.log(LogRecord::new(
                            LogLevel::Info,
                            format!("configure logging. setting log level: `{:?}`", &v),
                        ));
                        logger.set_log_level(v)
                    }
                    LoggerConfigureMessage::ShowLogLevel(v) => {
                        logger.log(LogRecord::new(
                            LogLevel::Info,
                            format!(
                                "configure logging. setting log level visibility: `{:?}`",
                                &v
                            ),
                        ));
                        logger.show_log_level(v)
                    }
                    LoggerConfigureMessage::ShowTimestamp(v) => {
                        logger.log(LogRecord::new(
                            LogLevel::Info,
                            format!(
                                "configure logging. setting timestamp visibility: `{:?}`",
                                &v
                            ),
                        ));
                        logger.show_timestamp(v)
                    }
                },
                Message::Terminate => {
                    logger.log(LogRecord::new(
                        LogLevel::Debug,
                        format!("log worker got a terminate message. terminating"),
                    ));
                    break;
                }
                _ => {}
            }
        });
        LogWorker {
            id,
            thread: Some(thread),
            sender,
        }
    }
}
