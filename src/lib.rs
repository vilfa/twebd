pub static LOGGER: crate::log::Logger = crate::log::Logger::new();
pub static LOG_INIT: std::sync::Once = std::sync::Once::new();

#[macro_export]
macro_rules! logf {
    ($lvl: expr, $fmt_str: expr $(, $fmt_arg: expr)*) => {
        $crate::log::native::LogRecord::new($lvl, format!($fmt_str, $($fmt_arg),*))
    };
}

#[macro_export]
macro_rules! err {
    ($fmt_str: expr $(, $fmt_arg: expr)*) => {
        crate::LOGGER.log(logf!(crate::log::native::LogLevel::Error, "{}", format!($fmt_str, $($fmt_arg),*)));
    };
}

#[macro_export]
macro_rules! warn {
    ($fmt_str: expr $(, $fmt_arg: expr)*) => {
        crate::LOGGER.log(logf!(crate::log::native::LogLevel::Warning, "{}", format!($fmt_str, $($fmt_arg),*)));
    };
}

#[macro_export]
macro_rules! info {
    ($fmt_str: expr $(, $fmt_arg: expr)*) => {
        crate::LOGGER.log(logf!(crate::log::native::LogLevel::Info, "{}", format!($fmt_str, $($fmt_arg),*)));
    };
}

#[macro_export]
macro_rules! dbg {
    ($fmt_str: expr $(, $fmt_arg: expr)*) => {
        crate::LOGGER.log(logf!(crate::log::native::LogLevel::Debug, "{}", format!($fmt_str, $($fmt_arg),*)));
    };
}

extern crate regex;
extern crate rustls;

pub(crate) mod app;
pub(crate) mod cli;
pub(crate) mod log;
pub(crate) mod net;
pub(crate) mod srv;
pub(crate) mod syn;
pub(crate) mod web;
