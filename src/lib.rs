//! A simple configurable logger with format specification.
extern crate log;

use log::*;

/// The body of fmtlog.
#[derive(Default)]
pub struct Logger;

impl Logger {
    /// Create a new instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set this logger active.
    pub fn set(self) -> Result<(), SetLoggerError> {
        set_max_level(STATIC_MAX_LEVEL);
        set_boxed_logger(Box::new(self))
    }
}

impl Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        eprintln!("{}: {}", record.level(), record.args());
    }

    fn flush(&self) {}
}
