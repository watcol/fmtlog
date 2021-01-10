//! Logger definition.
extern crate colored;
extern crate log;
extern crate thread_local;

use crate::Config;
use crate::Stream;

use log::{set_boxed_logger, set_max_level, Log, Metadata, Record, SetLoggerError};
use std::cell::RefCell;
use std::io::Write;
use thread_local::ThreadLocal;

/// The body of fmtlog.
pub struct Logger {
    colorize: bool,
    level: log::LevelFilter,
    writer: ThreadLocal<RefCell<Stream>>,
}

impl Logger {
    /// Create a new instance.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fmtlog::{Logger, config::Config};
    ///
    /// fn main() {
    ///     Logger::new(Config::new());
    /// }
    /// ```
    pub fn new(config: Config) -> Logger {
        let writer = ThreadLocal::new();
        writer
            .get_or(|| RefCell::new(config.output.to_stream().expect("Failed to open the file.")));
        Logger {
            colorize: config.colorize.colorize(&config.output),
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
    /// use fmtlog::{Logger, config::Config};
    ///
    /// fn main() {
    ///     Logger::new(Config::new()).set().unwrap();
    ///
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
        use colored::{Color, Colorize};

        let mut writer = self.writer.get().unwrap().borrow_mut();

        let level = record.level();
        let color = match level {
            log::Level::Error => Color::Red,
            log::Level::Warn => Color::Yellow,
            log::Level::Info => Color::Green,
            log::Level::Debug => Color::Cyan,
            log::Level::Trace => Color::Blue,
        };

        let level_str = if self.colorize {
            level.to_string().color(color)
        } else {
            level.to_string().normal()
        };

        writeln!(writer, "{}: {}", level_str, record.args()).expect("Failed to write.");
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
