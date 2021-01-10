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
//!         output = { stream = "stdout" }
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
//! ## Text-base Configuration
//! This crate supports configuration by JSON, YAML, and TOML.
//!
//! #### [JSON](https://en.wikipedia.org/wiki/JSON) (Requires feature `conf-json`)
//! ```json
//! {
//!     "colorize": "auto",
//!     "level": "info",
//!     "output": {
//!         "stream": "file",
//!         "path": "log.txt"
//!     }
//! }
//! ```
//!
//! #### [YAML](https://en.wikipedia.org/wiki/YAML) (Requires feature `conf-yaml`)
//! ```yaml
//! colorize: auto
//! level: info
//! output:
//!   stream: file
//!   path: log.txt
//! ```
//!
//! #### [TOML](https://en.wikipedia.org/wiki/TOML) (Requires feature `conf-toml`)
//! ```toml
//! colorize = "auto"
//! level = "info"
//!
//! [output]
//! stream = "file"
//! path = "log.txt"
//! ```
//!
//! Available values are there. (If the value is not present, the default will be chosen.)
//!
//! ### Data Format
//! | Key | Default | Value | Description |
//! |-----|:-------:|-------|-------------|
//! | [`colorize`](config/enum.Colorize.html) | `auto` | `on`, `auto`, `off` | Colorize the log if the value is `on`. |
//! | [`level`](config/enum.Level.html) | `info` | `off`, `error`, `warn`, `info`, `debug`, `trace` | Specify the log level. See [this](https://docs.rs/log) for the information. |
//! | [`output`](config/enum.Output.html) | - | The following format | Specify the log destination. |
//!
//! - The [`output`](config/enum.Output.html) format
//!
//! | Key | Default | Value | Description |
//! |-----|:-------:|-------|-------------|
//! | `stream` | `stderr` | `stdout`, `stderr`, `file` | The output stream. |
//! | `path` | - | Valid file path | Specify the file path when `stream` is `file`. |
//!
pub mod config;
mod logger;
mod stream;

pub use logger::Logger;
pub(crate) use stream::Stream;

use config::Config;

/// Create a logger by default settings.
pub fn default() -> Logger {
    Logger::new(Config::default())
}

/// Create a logger by custom settings.
pub fn new(config: Config) -> Logger {
    Logger::new(config)
}

/// **[conf-json]** Create a logger from JSON file.
#[cfg(feature = "conf-json")]
pub fn from_json<T: AsRef<str>>(s: T) -> serde_json::Result<Logger> {
    Ok(Logger::new(Config::from_json(s)?))
}

/// **[conf-yaml]** Create a logger from YAML file.
#[cfg(feature = "conf-yaml")]
pub fn from_yaml<T: AsRef<str>>(s: T) -> serde_yaml::Result<Logger> {
    Ok(Logger::new(Config::from_yaml(s)?))
}

/// **[conf-toml]** Create a logger from TOML file.
#[cfg(feature = "conf-toml")]
pub fn from_toml<T: AsRef<str>>(s: T) -> Result<Logger, toml::de::Error> {
    Ok(Logger::new(Config::from_toml(s)?))
}
