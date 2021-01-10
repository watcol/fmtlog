//! A simple configurable logger with format specification.
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

/// Create a logger from JSON file.
///
/// **Requires feature `conf-json`.**
#[cfg(feature = "conf-json")]
pub fn from_json<T: AsRef<str>>(s: T) -> serde_json::Result<Logger> {
    Ok(Logger::new(Config::from_json(s)?))
}

/// Create a logger from YAML file.
///
/// **Requires feature `conf-yaml`.**
#[cfg(feature = "conf-yaml")]
pub fn from_yaml<T: AsRef<str>>(s: T) -> serde_yaml::Result<Logger> {
    Ok(Logger::new(Config::from_yaml(s)?))
}

/// Create a logger from TOML file.
///
/// **Requires feature `conf-toml`.**
#[cfg(feature = "conf-toml")]
pub fn from_toml<T: AsRef<str>>(s: T) -> Result<Logger, toml::de::Error> {
    Ok(Logger::new(Config::from_toml(s)?))
}
