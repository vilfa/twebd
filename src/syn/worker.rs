use crate::syn::Message;
use log::debug;
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
    pub fn new(id: usize, receiver: Rx) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver
                .lock()
                .expect("failed to acquire thread message lock!")
                .recv()
                .unwrap();

            match message {
                Message::Job(job) => {
                    debug!("worker {} got a job", id);
                    job();
                }
                Message::Terminate => {
                    debug!("worker {} got a terminate message", id);
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
