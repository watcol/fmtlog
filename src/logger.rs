//! Logger definition.
extern crate colored;
extern crate log;
extern crate thread_local;

use crate::Config;
use crate::Format;
use crate::Stream;

use log::{set_boxed_logger, set_max_level, Log, Metadata, Record, SetLoggerError};
use std::cell::RefCell;
use std::io::Write;
use thread_local::ThreadLocal;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub(crate) struct Style {
    pub fg: Option<colored::Color>,
    pub bg: Option<colored::Color>,
}

/// The body of fmtlog.
pub struct Logger {
    colorize: bool,
    style: ThreadLocal<RefCell<Style>>,
    format: Format,
    level: log::LevelFilter,
    writer: ThreadLocal<RefCell<Stream>>,
}

impl Logger {
    /// Create a new instance.
    pub fn new(config: Config) -> Logger {
        let writer = ThreadLocal::new();
        writer
            .get_or(|| RefCell::new(config.output.to_stream().expect("Failed to open the file.")));

        let style = ThreadLocal::new();
        style.get_or(|| RefCell::new(Style::default()));

        Logger {
            colorize: config.colorize.colorize(&config.output),
            style,
            format: Format::parse(config.format).expect("Invalid Format."),
            level: config.level.into(),
            writer,
        }
    }

    /// Set this logger active.
    ///
    /// # Example
    ///
    /// ```rust
    /// #[macro_use]
    /// extern crate log;
    ///
    /// use fmtlog::{Logger, Config};
    ///
    /// fn main() {
    ///     Logger::new(Config::new()).set().unwrap();
    ///     info!("Hello!") // INFO: Hello!
    /// }
    /// ```
    pub fn set(self) -> Result<(), SetLoggerError> {
        set_max_level(self.level);
        set_boxed_logger(Box::new(self))
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.level >= metadata.level()
    }

    fn log(&self, record: &Record) {
        let mut writer = self.writer.get().unwrap().borrow_mut();
        let mut style = self.style.get().unwrap().borrow_mut();

        self.format
            .write(&mut *writer, record, &mut *style, self.colorize)
            .expect("Failed to write.");
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
