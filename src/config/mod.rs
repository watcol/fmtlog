//! Configuration module.
#[cfg(feature = "serde")]
extern crate serde;
#[cfg(feature = "conf-json")]
extern crate serde_json;
#[cfg(feature = "conf-yaml")]
extern crate serde_yaml;
#[cfg(feature = "conf-toml")]
extern crate toml;

mod colorize;
mod level;
mod output;

pub use colorize::Colorize;
pub use level::Level;
pub use output::Output;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The logger settings.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Config {
    #[cfg_attr(feature = "serde", serde(default))]
    pub(crate) colorize: Colorize,
    #[cfg_attr(feature = "serde", serde(default))]
    pub(crate) level: Level,
    #[cfg_attr(feature = "serde", serde(default))]
    pub(crate) output: Output,
}

impl Config {
    /// Create a new instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new instance from JSON.
    ///
    /// **Requires `conf-json` feature.**
    #[cfg(feature = "conf-json")]
    pub fn from_json<T: AsRef<str>>(s: T) -> serde_json::Result<Self> {
        serde_json::from_str(s.as_ref())
    }

    /// Output JSON from the configuration.
    ///
    /// **Requires `conf-json` feature.**
    #[cfg(feature = "conf-json")]
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }

    /// Create a new instance from YAML.
    ///
    /// **Requires `conf-yaml` feature.**
    #[cfg(feature = "conf-yaml")]
    pub fn from_yaml<T: AsRef<str>>(s: T) -> serde_yaml::Result<Self> {
        serde_yaml::from_str(s.as_ref())
    }

    /// Output YAML from the configuration.
    ///
    /// **Requires `conf-yaml` feature.**
    #[cfg(feature = "conf-yaml")]
    pub fn to_yaml(&self) -> serde_yaml::Result<String> {
        serde_yaml::to_string(self)
    }

    /// Create a new instance from TOML.
    ///
    /// **Requires `conf-toml` feature.**
    #[cfg(feature = "conf-toml")]
    pub fn from_toml<T: AsRef<str>>(s: T) -> Result<Self, toml::de::Error> {
        toml::from_str(s.as_ref())
    }

    /// Output TOML from the configuration.
    ///
    /// **Requires `conf-toml` feature.**
    #[cfg(feature = "conf-toml")]
    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string(self)
    }

    /// Set the log level.
    pub fn colorize<T: Into<Colorize>>(mut self, colorize: T) -> Self {
        self.colorize = colorize.into();
        self
    }

    /// Set the log level.
    pub fn level<T: Into<Level>>(mut self, level: T) -> Self {
        self.level = level.into();
        self
    }

    /// Set the output stream.
    pub fn output<T: Into<Output>>(mut self, output: T) -> Self {
        self.output = output.into();
        self
    }
}