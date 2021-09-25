// TODO
// DONE - *start the logger in its own thread and use message passing*
// DONE - implement a thread pool;
// DONE - implement actual listeners
// DONE - implement a message queue for log messages for when the logger is uninitialized;
// DONE - modify all http structs to use Option<T>, to enable universal request and response usage;
// DONE - implement simple http requests and responses;
// DONE - generate self-signed certificates for tls testing;
// - implement https module;
// - implement https module error handling;
// DONE - make socket operations actually multithreaded;
// DONE - restructure and make logger better;
// DONE - look into once for logger;
// DONE - get rid of udp

fn main() {
    twebd::run();
}
