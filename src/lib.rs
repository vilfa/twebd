// pub static LOGGER: crate::log::Logger = crate::log::Logger::new();
// pub static LOG_INIT: std::sync::Once = std::sync::Once::new();

// #[macro_export]
// macro_rules! logf {
//     ($lvl: expr, $fmt_str: expr $(, $fmt_arg: expr)*) => {
//         $crate::log::native::LogRecord::new($lvl, format!($fmt_str, $($fmt_arg),*))
//     };
// }

// #[macro_export]
// macro_rules! err {
//     ($fmt_str: expr $(, $fmt_arg: expr)*) => {
//         crate::LOGGER.log(logf!(crate::log::native::LogLevel::Error, "{}", format!($fmt_str, $($fmt_arg),*)));
//     };
// }

// #[macro_export]
// macro_rules! warn {
//     ($fmt_str: expr $(, $fmt_arg: expr)*) => {
//         crate::LOGGER.log(logf!(crate::log::native::LogLevel::Warning, "{}", format!($fmt_str, $($fmt_arg),*)));
//     };
// }

// #[macro_export]
// macro_rules! info {
//     ($fmt_str: expr $(, $fmt_arg: expr)*) => {
//         crate::LOGGER.log(logf!(crate::log::native::LogLevel::Info, "{}", format!($fmt_str, $($fmt_arg),*)));
//     };
// }

// #[macro_export]
// macro_rules! dbg {
//     ($fmt_str: expr $(, $fmt_arg: expr)*) => {
//         crate::LOGGER.log(logf!(crate::log::native::LogLevel::Debug, "{}", format!($fmt_str, $($fmt_arg),*)));
//     };
// }

static LOGGER: app::Logger = app::Logger;
fn init_logger(log_level: log::LevelFilter) -> Result<(), log::SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(log_level))
}

extern crate log;
extern crate regex;
extern crate rustls;

pub mod app;
pub mod cli;
// pub mod log;
pub mod net;
pub mod srv;
pub mod syn;
pub mod web;

pub use cli::run;
