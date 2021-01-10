//! Logger definition.
extern crate colored;
extern crate log;
extern crate thread_local;

use crate::Config;
use crate::Stream;

use log::{set_boxed_logger, set_max_level, Log, Metadata, Record, SetLoggerError};
use std::cell::RefCell;
use std::io::Write;
use thread_local::CachedThreadLocal;

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
        set_max_level(self.config.level.into());
        set_boxed_logger(Box::new(self))
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.config.level >= metadata.level()
    }

    fn log(&self, record: &Record) {
        use colored::{Color, Colorize};

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

        let level = record.level();
        let color = match level {
            log::Level::Error => Color::Red,
            log::Level::Warn => Color::Yellow,
            log::Level::Info => Color::Green,
            log::Level::Debug => Color::Cyan,
            log::Level::Trace => Color::Blue,
        };

        writeln!(
            writer,
            "{}: {}",
            level.to_string().color(color),
            record.args()
        )
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
