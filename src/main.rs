use twebd::cli::runner;

// TODO
// DONE - *start the logger in its own thread and use message passing*
// DONE - implement a thread pool;
// DONE - implement actual listeners
// DONE - implement a message queue for log messages for when the logger is uninitialized;
// DONE - modify all http structs to use Option<T>, to enable universal request and response usage;
// DONE - implement simple http requests and responses;
// DONE - generate self-signed certificates for tls testing;
// - implement https module;

fn main() -> std::result::Result<(), twebd::srv::server::ServerError> {
    runner::run()
}
