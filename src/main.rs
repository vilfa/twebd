use twebd::cli::runner;

// TODO
// DONE - *start the logger in its own thread and use message passing*
// DONE - implement a thread pool;
// DONE - implement actual listeners
// DONE - implement a message queue for log messages for when the logger is uninitialized;
// - modify all http structs to use Option<T>, to enable universal request and response usage;
// - implement simple http requests and responses;
// - implement crypto module;
// -

fn main() {
    runner::run();
}
