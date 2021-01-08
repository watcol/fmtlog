//! A simple configurable logger with format specification.
extern crate log;
extern crate thread_local;

use log::*;
use std::cell::RefCell;
use std::io::{self, Write};
use std::{fmt, fs};
use thread_local::CachedThreadLocal;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Output {
    Stdout,
    Stderr,
    File(std::path::PathBuf),
}

impl fmt::Display for Output {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "{}",
            match self {
                Self::Stdout => "<stdout>",
                Self::Stderr => "<stderr>",
                Self::File(path) => path.to_str().unwrap_or("<???>"),
            }
        )
    }
}

impl Default for Output {
    fn default() -> Self {
        Self::Stderr
    }
}

impl<T: Into<std::path::PathBuf>> From<T> for Output {
    fn from(path: T) -> Self {
        Output::File(path.into())
    }
}

impl Output {
    /// Create `Stream` from `Output`.
    fn to_stream(&self) -> io::Result<Stream> {
        Ok(match self.clone() {
            Self::Stdout => Stream::from(io::stdout()),
            Self::Stderr => Stream::from(io::stderr()),
            Self::File(path) => {
                Stream::from(fs::File::open(path.clone()).or(fs::File::create(path.clone()))?)
            }
        })
    }
}

#[derive(Debug)]
enum Stream {
    Stdout(io::Stdout),
    Stderr(io::Stderr),
    File(fs::File),
}

impl From<io::Stdout> for Stream {
    fn from(s: io::Stdout) -> Self {
        Stream::Stdout(s)
    }
}

impl From<io::Stderr> for Stream {
    fn from(s: io::Stderr) -> Self {
        Stream::Stderr(s)
    }
}

impl From<fs::File> for Stream {
    fn from(s: fs::File) -> Self {
        Stream::File(s)
    }
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
    output: Output,
}

impl Config {
    /// Create a new instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the output stream.
    pub fn output(mut self, output: Output) -> Self {
        self.output = output;
        self
    }
}

/// The body of fmtlog.
pub struct Logger {
    config: Config,
    writer: CachedThreadLocal<RefCell<Stream>>,
}

impl Logger {
    /// Create a new instance.
    pub fn new(config: Config) -> Logger {
        Logger {
            config,
            writer: CachedThreadLocal::new(),
        }
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
        let mut writer = self
            .writer
            .get_or(|| {
                RefCell::new(
                    self.config
                        .output
                        .to_stream()
                        .expect("Failed to open a file."),
                )
            })
            .borrow_mut();
        writeln!(writer, "{}: {}", record.level(), record.args()).expect("Failed to write.");
    }

    fn flush(&self) {
        match self.writer.get() {
            Some(writer) => {
                writer
                    .borrow_mut()
                    .flush()
                    .expect("Failed to flush the stream.");
            }
            None => {}
        }
    }
}

/// Create a logger by default settings.
pub fn default() -> Logger {
    Logger::new(Config::default())
}

/// Create a logger by custom settings.
pub fn new(config: Config) -> Logger {
    Logger::new(config)
}
