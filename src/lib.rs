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
//! | `%%` | | Literal `%`. |
//! | `%}` | | Literal `}`. (use in `{}`.) |
//! | `%N` | `hyper` | The target of the log. |
//! | `%f` | `main.rs` | The file that the log defined. |
//! | `%S` | `main.rs:15` | The file and line that the log defined. |
//! | `%M` | `An error has occured.` | The log message. |
//! | `%l` | `info` | The log level. (lowercase) |
//! | `%L` | `INFO` | The log level. (uppercase) |
//! | `%T(<format>)` | `%T(%D %T)` -> `01/01/21 12:00:00` | The local time. (see [chrono's format specification](https://docs.rs/chrono/0.4/chrono/format/strftime)). |
//! | `%U(<format>)` | `%T(%D %T)` -> `01/01/21 12:00:00` | The UTC time. (see [chrono's format specification](https://docs.rs/chrono/0.4/chrono/format/strftime)). |
//! | `%F(<color>){...}` | | Set the foreground color. |
//! | `%F(<error>,<warn>,<info>,<debug>,<trace>){...}` | | Set the foreground color. (Branch by the log level.) |
//! | `%B(<color>){...}` | | Set the background color. |
//! | `%B(<error>,<warn>,<info>,<debug>,<trace>){...}` | | Set the background color. (Branch by the log level.) |
//! | `%b{...}` | | Bold the text. |
//! | `%d{...}` | | Dim the text color. |
//! | `%i{...}` | | Print the text in italics. |
//! | `%r{...}` | | Reverse the foreground and background color. |
//! | `%u{...}` | | Underline the text. |
//! | `%s{...}` | | Strikethrough the text. |
//!
//! ### Supported Color
//! All supported color used by `%C` and `%O` is here.
//! - `black`
//! - `red`
//! - `green`
//! - `yellow`
//! - `blue`
//! - `magenta`
//! - `cyan`
//! - `white`
//! - `bright_black`
//! - `bright_red`
//! - `bright_green`
//! - `bright_yellow`
//! - `bright_blue`
//! - `bright_magenta`
//! - `bright_cyan`
//! - `bright_white`
//! - `#ffffff` (Hexadecimal RGB)
//!
extern crate colored;
extern crate log;
extern crate thread_local;

mod config;
mod format;
mod stream;

pub use config::*;

use format::Format;
use stream::Stream;

use log::{set_boxed_logger, set_max_level, Log, Metadata, Record, SetLoggerError};
use std::cell::RefCell;
use thread_local::ThreadLocal;

/// The body of fmtlog.
pub struct Logger {
    colorize: bool,
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

        Logger {
            colorize: config.colorize.colorize(&config.output),
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

        self.format
            .write(&mut *writer, record, self.colorize)
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
