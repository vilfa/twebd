extern crate log;
extern crate mio;
extern crate regex;
extern crate rustls;
extern crate rustls_pemfile;

pub mod cli;
pub mod net;
pub mod srv;
pub mod syn;
pub mod web;

pub use cli::run;

pub const APP_NAME: &str = "twebd";
pub const APP_VERSION: &str = "0.1.0";
pub const APP_AUTHOR: &str = "Luka Vilfan <luka.vilfan@proton.me>";
pub const APP_DESCRIPTION: &str = "A simple multi-threaded web server written in Rust.";
