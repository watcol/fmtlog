//! A simple configurable logger with format specification.
//!
//! This crate provides an implementation of [`log` crate](https://docs.rs/log), which
//! provides integrated logging interface.
//!
//! ## Examples
//! #### Basic
//!
//! ```rust
//! #[macro_use]
//! extern crate log;
//! extern crate fmtlog;
//!
//! fn main() {
//!     fmtlog::default().set().unwrap();
//!
//!     info!("Hello!"); // INFO: Hello!
//! }
//! ```
//! See also [the function `default`](fn.default.html).
//!
//! #### Configure in Code
//!
//! ```rust
//! #[macro_use]
//! extern crate log;
//! extern crate fmtlog;
//!
//! use fmtlog::{Config, LevelFilter};
//!
//! fn main() {
//!     fmtlog::new(Config::new().level(LevelFilter::Trace))
//!         .set()
//!         .unwrap();
//!
//!     info!("Hello!"); // INFO: Hello!
//! }
//! ```
//! See also [the struct `Config`](struct.Config.html).
//!
//! ## Format Specification
//! The format string is basically a string, but the following specifiers will converted into
//! another string.
//!
//! | Spec. | Example | Description |
//! |-------|---------|-------------|
//! | `%%` | `%` | Literal `%`. |
//! | `%,` | `%` | Literal `,` (use in branching). |
//! | `%)` | `%` | Literal `)`. (use in branching.) |
//! | `%(<error>,<warn>,<info>,<debug>,<trace>)` | `%(%C(red),%C(yellow),%C(green),%C(cyan),%C(yellow))` | Branching by the log level. |
//! | `%M` | `An error has occured.` | The log message. |
//! | `%l` | `info` | The log level. (lowercase) |
//! | `%L` | `INFO` | The log level. (uppercase) |
//! | `%C(<color>)` | `%C(green)` | Set the foreground color. |
//! | `%c` | | Reset the foreground color. |
//! | `%O(<color>)` | `%O(green)` | Set the background color. |
//! | `%o` | | Reset the background color. |
//! | `%B` | | Set the character bold. |
//! | `%b` | | Unset the bold character. |
//! | `%U` | | Enable underline. |
//! | `%u` | | Disable underline. |
//!
extern crate colored;
extern crate log;
extern crate thread_local;

mod config;
mod format;
mod stream;

pub use config::*;

use format::{Format, Style};
use stream::Stream;

use log::{set_boxed_logger, set_max_level, Log, Metadata, Record, SetLoggerError};
use std::cell::RefCell;
use thread_local::ThreadLocal;

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
            format: Format::new(config.format).expect("Invalid Format."),
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
        use std::io::Write;

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
///
/// This function wraps [`Config::default`](struct.Config.html#impl-Default).
///
/// # Example
///
/// ```rust
/// #[macro_use]
/// extern crate log;
/// extern crate fmtlog;
///
/// fn main() {
///     fmtlog::default().set().unwrap();
///
///     info!("Hello!"); // INFO: Hello!
/// }
/// ```
pub fn default() -> Logger {
    Logger::new(Config::default())
}

/// Create a logger by custom settings.
///
/// This function wraps [`Logger::new`](struct.Logger.html#method.new).
///
/// # Example
///
/// ```rust
/// #[macro_use]
/// extern crate log;
/// extern crate fmtlog;
///
/// use fmtlog::Config;
///
/// fn main() {
///     fmtlog::new(Config::new()).set().unwrap();
///
///     info!("Hello!"); // INFO: Hello!
/// }
/// ```
pub fn new(config: Config) -> Logger {
    Logger::new(config)
}
