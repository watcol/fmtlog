//! A simple configurable logger with format specification.
extern crate log;
extern crate thread_local;

pub mod config;
mod logger;
mod stream;

pub use logger::Logger;
pub(crate) use stream::Stream;

use config::Config;

/// Create a logger by default settings.
pub fn default() -> Logger {
    Logger::new(Config::default())
}

/// Create a logger by custom settings.
pub fn new(config: Config) -> Logger {
    Logger::new(config)
}
