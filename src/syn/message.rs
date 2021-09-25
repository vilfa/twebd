use crate::syn::Job;

pub enum Message {
    Job(Job),
    Terminate,
}
