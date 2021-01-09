//! Logger definition.
use crate::Config;
use crate::Stream;

use log::{set_boxed_logger, set_max_level, LevelFilter, Log, Metadata, Record, SetLoggerError};
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
        metadata.level() <= Into::<LevelFilter>::into(self.config.level)
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
