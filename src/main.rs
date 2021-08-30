use twebd::cli::runner;

// TODO
// DONE - *start the logger in its own thread and use message passing*
// DONE - implement a thread pool;
// DONE - implement actual listeners
// - implement a message queue for log messages for when the logger is uninitialized;
// - implement simple http requests and responses;
// - implement crypto module;
// -

fn main() {
    runner::run();
}
