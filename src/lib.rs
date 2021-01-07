//! A simple configurable logger with format specification.
extern crate log;

use log::*;
use std::{fmt, fs};
use std::io::Write;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Output {
    Stdout,
    Stderr,
    File(std::path::PathBuf)
}

impl fmt::Display for Output {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "{}",
            match self {
                Self::Stdout => "<stdout>",
                Self::Stderr => "<stderr>",
                Self::File(path) => path.to_str().unwrap_or("<???>")
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
        match self.output.clone() {
            Output::Stdout => println!("{}: {}", record.level(), record.args()),
            Output::Stderr => eprintln!("{}: {}", record.level(), record.args()),
            Output::File(path) => {
                let mut file = fs::File::open(path)
                    .expect("Failed to open the log file.");
                write!(file, "{}: {}", record.level(), record.args())
                    .expect("Failed to write the file.");
            }
        }
    }

    fn flush(&self) {}
}
