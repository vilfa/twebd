use super::message::Message;
use crate::log::{
    config::Configure,
    logger::{Log, Logger},
    LogLevel, LoggerConfigureMessage,
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
                        .send(Message::Log(
                            LogLevel::Debug,
                            format!("worker {} got a job. executing", id),
                        ))
                        .unwrap();
                    job();
                }
                Message::Terminate => {
                    logger
                        .send(Message::Log(
                            LogLevel::Debug,
                            format!("worker {} got a terminate message. terminating", id),
                        ))
                        .unwrap();
                    break;
                }
                _ => {} // Message::Log(log_level, msg) => {
                        //     logger
                        //         .send(Message::Log(
                        //             LogLevel::Debug,
                        //             format!("worker {} got a log. sending", id),
                        //         ))
                        //         .unwrap();
                        //     logger.send(Message::Log(log_level, msg)).unwrap();
                        // }
                        // Message::LogConfigure(conf) => {
                        //     logger
                        //         .send(Message::Log(
                        //             LogLevel::Debug,
                        //             format!("worker {} got a log config. sending", id),
                        //         ))
                        //         .unwrap();
                        //     logger.send(Message::LogConfigure(conf)).unwrap();
                        // }
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
                Message::Log(log_level, msg) => logger.log(log_level, msg),
                Message::LogConfigure(c) => match c {
                    LoggerConfigureMessage::SetLogLevel(v) => {
                        logger.info(format!("configure logger. setting log level: `{:?}`", &v));
                        logger.set_log_level(v)
                    }
                    LoggerConfigureMessage::ShowLogLevel(v) => {
                        logger.info(format!(
                            "configure logger. setting log level visibility: `{:?}`",
                            &v
                        ));
                        logger.show_log_level(v)
                    }
                    LoggerConfigureMessage::ShowTimestamp(v) => {
                        logger.info(format!(
                            "configure logger. setting timestamp visibility: `{:?}`",
                            &v
                        ));
                        logger.show_timestamp(v)
                    }
                },
                Message::Terminate => {
                    logger.debug(format!("log worker got a terminate message. terminating",));
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
