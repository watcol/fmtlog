//! A simple configurable logger with format specification.
extern crate log;

use log::*;

/// The body of fmtlog.
#[derive(Default)]
pub struct FmtLog;

impl FmtLog {
    /// Create a new instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set this logger active.
    pub fn set(self) -> Result<(), SetLoggerError> {
        set_boxed_logger(Box::new(self))
    }
}

impl Log for FmtLog {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, _record: &Record) {
        eprintln!("Unimplemented");
    }

    fn flush(&self) {}
}
