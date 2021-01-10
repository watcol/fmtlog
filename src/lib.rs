//! A simple configurable logger with format specification.
//!
//! ## Text-base Configuration
//! This crate supports configuration by JSON, YAML, and TOML.
//! - [JSON](https://en.wikipedia.org/wiki/JSON) (Requires feature `conf-json`)
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
//! - [YAML](https://en.wikipedia.org/wiki/YAML) (Requires feature `conf-yaml`)
//! ```yaml
//! colorize: auto
//! level: info
//! output:
//!   stream: file
//!   path: log.txt
//! ```
//!
//! - [TOML](https://en.wikipedia.org/wiki/TOML) (Requires feature `conf-toml`)
//! ```toml
//! colorize = "auto"
//! level = "info"
//!
//! [output]
//! stream = "file"
//! path = "log.txt"
//! ```
//!
//! Available key and value are there.
//!
//! ### Data Format
//! | Key | Value | Description |
//! |-----|-------|-------------|
//! | `colorize` | `on`, `auto`, `off` | Colorize the log if this value is `on` |
//! | `level` | `off`, `error`, `warn`, `info`, `debug`, `trace` | Specify the log level. See [this](https://docs.rs/log) for the information. |
//! | `output` | The following format | Specify the log destination. |
//!
//! - The `output` format
//!
//! | Key | Value | Description |
//! |-----|-------|-------------|
//! | `stream` | `stdout`, `stderr`, `file` | The output stream. |
//! | `path` | Valid file path | Specify the file path when `stream` is `file`. |
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
