//! A simple configurable logger with format specification.
extern crate log;

use log::*;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Output {
    Stdout,
    Stderr,
}

impl fmt::Display for Output {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "{}",
            match self {
                Self::Stdout => "<stdout>",
                Self::Stderr => "<stderr>",
            }
        )
    }
}

impl Default for Output {
    fn default() -> Self {
        Self::Stderr
    }
}

/// The body of fmtlog.
#[derive(Default)]
pub struct Logger {
    output: Output,
}

impl Logger {
    /// Create a new instance.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn output(&mut self, output: Output) -> &mut Self {
        self.output = output;
        self
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
        match self.output {
            Output::Stdout => println!("{}: {}", record.level(), record.args()),
            Output::Stderr => eprintln!("{}: {}", record.level(), record.args()),
        }
    }

    fn flush(&self) {}
}
