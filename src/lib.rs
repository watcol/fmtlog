//! A simple configurable logger with format specification.
extern crate log;
extern crate thread_local;

use log::*;
use thread_local::ThreadLocal;
use std::cell::RefCell;
use std::{fmt, fs};
use std::io::{
    self, Write
};

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

#[derive(Debug)]
enum Stream {
    Stdout(io::Stdout),
    Stderr(io::Stderr),
    File(fs::File)
}

impl Write for Stream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Stream::Stdout(w) => w.write(buf),
            Stream::Stderr(w) => w.write(buf),
            Stream::File(w) => w.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            Stream::Stdout(w) => w.flush(),
            Stream::Stderr(w) => w.flush(),
            Stream::File(w) => w.flush(),
        }
    }
}

/// The logger settings.
#[derive(Default, Clone, Debug)]
pub struct Config {
    output: Output
}

impl Config {
    /// Create a new instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the output stream.
    pub fn output(&mut self, output: Output) -> &mut Self {
        self.output = output;
        self
    }
}

/// The body of fmtlog.
pub struct Logger {
    config: Config,
    writer: ThreadLocal<RefCell<Stream>>,
}

impl Logger {
    /// Create a new instance.
    pub fn new(config: Config) -> io::Result<Logger> {

        Ok(Logger {
            config,
            writer: ThreadLocal::new(),
        })
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
        let writer = self.writer.get_or(|| {
            let tmp = match self.config.output.clone() {
                Output::Stdout => Stream::Stdout(io::stdout()),
                Output::Stderr => Stream::Stderr(io::stderr()),
                Output::File(path) => Stream::File(fs::File::open(path)
                                                   .expect("Failed to read the file"))
            };
            RefCell::new(tmp)
        });

        let mut writer = writer.borrow_mut();
        write!(writer, "{}: {}", record.level(), record.args())
            .expect("Failed to write.");
    }

    fn flush(&self) {}
}
