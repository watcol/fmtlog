//! Configuration module.
#[cfg(feature = "serde")]
extern crate serde;
#[cfg(feature = "conf-toml")]
extern crate toml;
#[cfg(feature = "conf-json")]
extern crate serde_json;

mod output;
mod level;

pub use output::Output;
pub use level::Level;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The logger settings.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Config {
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

    /// Create a new instance from TOML.
    #[cfg(feature = "conf-toml")]
    pub fn from_toml(s: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(s)
    }

    /// Output TOML from the configuration.
    #[cfg(feature = "conf-toml")]
    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string(self)
    }

    /// Create a new instance from JSON.
    #[cfg(feature = "conf-json")]
    pub fn from_json(s: &str) -> serde_json::Result<Self> {
        serde_json::from_str(s)
    }

    /// Output JSON from the configuration.
    #[cfg(feature = "conf-json")]
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }

    /// Set the output stream.
    pub fn output(mut self, output: Output) -> Self {
        self.output = output;
        self
    }

    /// Set the log level.
    pub fn level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }
}
