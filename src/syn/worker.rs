use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub type Job = Box<dyn FnOnce() + Send + 'static>;

pub enum Message {
    NewJob(Job),
    Terminate,
}

pub struct Worker {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // TODO: write all these messages to the log.
            let message = receiver
                .lock()
                .expect("failed to acquire thread message lock!")
                .recv()
                .unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("worker {} got a job. executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("worker {} got a terminate message. terminating", id);
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
