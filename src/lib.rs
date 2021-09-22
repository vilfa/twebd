#[macro_export]
macro_rules! logf {
    ($lvl: expr, $fmt_str: expr $(, $fmt_arg: expr)*) => {
        $crate::log::native::LogRecord::new($lvl, format!($fmt_str, $($fmt_arg),*))
    };
}

extern crate regex;
extern crate rustls;

pub mod cli;
pub mod log;
pub mod net;
pub mod srv;
pub mod syn;
pub mod web;

pub const APP_NAME: &str = "twebd";
pub const APP_VERSION: &str = "0.1.0";
pub const APP_AUTHOR: &str = "Luka Vilfan <luka.vilfan@protonmail.com>";
pub const APP_DESCRIPTION: &str = "A simple multi-threaded web server written in Rust.";
