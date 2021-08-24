use twebd::cli::runner;
use twebd::log::logger::Logger;

// TODO
// - *start the logger in its own thread and use message passing*
// - implement a thread pool;
// - implement actual listeners;
// - implement simple http requests and responses;
// - implement crypto module;
// -

fn main() {
    let mut logger = Logger::init();
    runner::run(&mut logger);
}
