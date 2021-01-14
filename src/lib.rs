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
//! use fmtlog::config::{Config, Level};
//!
//! fn main() {
//!     fmtlog::new(Config::new().level(Level::Trace))
//!         .set()
//!         .unwrap();
//!
//!     info!("Hello!"); // INFO: Hello!
//! }
//! ```
//! See also [the struct `Config`](config/struct.Config.html).
//!
//! #### Configure Using TOML (Requires feature `conf-toml`)
//!
//! ```rust
//! #[macro_use]
//! extern crate log;
//! extern crate fmtlog;
//!
//! # #[cfg(feature = "conf-toml")]
//! fn main() {
//!     let toml = r#"
//!         level = "trace"
//!         output = "stdout"
//!     "#;
//!
//!     fmtlog::from_toml(toml).unwrap().set().unwrap();
//!
//!     info!("Hello!"); // INFO: Hello!
//! }
//! #
//! # #[cfg(not(feature = "conf-toml"))]
//! # fn main() {}
//! ```
//! See also [the following](#text-base-configuration) and [the function `from_toml`](fn.from_toml.html).
//!
//! ## Format Specification
//! The format string is basically a string, but the following specifiers will converted into
//! another string.
//!
//! | Spec. | Example | Description |
//! |-------|---------|-------------|
//! | `%%` | `%` | Literal `%`. |
//! | `%M` | `An error has occured.` | The log message. |
//! | `%l` | `info` | The log level. (lowercase) |
//! | `%L` | `INFO` | The log level. (uppercase) |
//!
//! ## Text-base Configuration
//! This crate supports configuration by JSON, YAML, and TOML.
//!
//! #### [JSON](https://en.wikipedia.org/wiki/JSON) (Requires feature `conf-json`)
//! ```json
//! {
//!     "colorize": "auto",
//!     "level": "info",
//!     "output": "log.txt"
//! }
//! ```
//!
//! #### [YAML](https://en.wikipedia.org/wiki/YAML) (Requires feature `conf-yaml`)
//! ```yaml
//! colorize: auto
//! level: info
//! output: log.txt
//! ```
//!
//! #### [TOML](https://en.wikipedia.org/wiki/TOML) (Requires feature `conf-toml`)
//! ```toml
//! colorize = "auto"
//! level = "info"
//! output = "log.txt"
//! ```
//!
//! Available values are there. (If the value is not present, the default will be chosen.)
//!
//! ### Data Format
//! | Key | Default | Value | Description |
//! |-----|:-------:|-------|-------------|
//! | [`colorize`](config/enum.Colorize.html) | `auto` | `on`, `auto`, `off`, `true`, `false` | Colorize the log if the value is `on` (or `true`). |
//! | `format` | `%l: %M\n` | A string | Logger format specified in [the previous](#format-specification). |
//! | [`level`](config/enum.Level.html) | `info` | `off`, `error`, `warn`, `info`, `debug`, `trace` | Specify the log level. See [this](https://docs.rs/log) for the information. |
//! | [`output`](config/enum.Output.html) | `stderr` | `stdout`, `stderr`, or a valid file path. | Specify the log destination. |
//!
pub mod config;
mod format;
mod logger;
mod stream;

pub(crate) use format::Format;
pub use logger::Logger;
pub(crate) use stream::Stream;

use config::Config;

/// Create a logger by default settings.
///
/// This function wraps [`Config::default`](config/struct.Config.html#impl-Default).
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
/// use fmtlog::config::Config;
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

/// **[conf-json]** Create a logger from JSON file.
///
/// This function wraps [`Config::from_json`](config/struct.Config.html#method.from_json).
///
/// # Example
///
/// ```rust
/// #[macro_use]
/// extern crate log;
/// extern crate fmtlog;
///
/// fn main() {
///     let json = r#"
///         {
///             "colorize": true,
///             "level": "trace",
///             "output": "stdout"
///         }
///     "#;
///
///     fmtlog::from_json(json).unwrap().set().unwrap();
///
///     info!("Hello!"); // INFO: Hello!
/// }
/// ```
#[cfg(feature = "conf-json")]
pub fn from_json<T: AsRef<str>>(s: T) -> serde_json::Result<Logger> {
    Ok(Logger::new(Config::from_json(s)?))
}

/// **[conf-yaml]** Create a logger from YAML file.
///
/// This function wraps [`Config::from_yaml`](config/struct.Config.html#method.from_yaml).
///
/// # Example
///
/// ```rust
/// #[macro_use]
/// extern crate log;
/// extern crate fmtlog;
///
/// fn main() {
///     let yaml = r#"
///         colorize: true
///         level: trace
///         output: stdout
///     "#;
///
///     fmtlog::from_yaml(yaml).unwrap().set().unwrap();
///
///     info!("Hello!"); // INFO: Hello!
/// }
/// ```
#[cfg(feature = "conf-yaml")]
pub fn from_yaml<T: AsRef<str>>(s: T) -> serde_yaml::Result<Logger> {
    Ok(Logger::new(Config::from_yaml(s)?))
}

/// **[conf-toml]** Create a logger from TOML file.
///
/// This function wraps [`Config::from_toml`](config/struct.Config.html#method.from_toml).
///
/// # Example
///
/// ```rust
/// #[macro_use]
/// extern crate log;
/// extern crate fmtlog;
///
/// fn main() {
///     let toml = r#"
///         colorize = true
///         level = "trace"
///         output = "stdout"
///     "#;
///
///     fmtlog::from_toml(toml).unwrap().set().unwrap();
///
///     info!("Hello!"); // INFO: Hello!
/// }
/// ```
#[cfg(feature = "conf-toml")]
pub fn from_toml<T: AsRef<str>>(s: T) -> Result<Logger, toml::de::Error> {
    Ok(Logger::new(Config::from_toml(s)?))
}
