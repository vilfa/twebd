use crate::{cli::CliOpt, syn::thread::ThreadPool};

pub struct Server {
    opts: Vec<CliOpt>,
    // socket: ,
    thread_pool: ThreadPool,
}

impl Server {}
