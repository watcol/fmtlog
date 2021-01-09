//! Configuration module.
mod output;

pub use output::Output;
pub use log::LevelFilter;

use serde::{Deserialize, Serialize};

fn default_level() -> LevelFilter {
    LevelFilter::Info
}

/// The logger settings.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub(crate) output: Output,
    #[serde(default = "default_level")]
    pub(crate) level: LevelFilter,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            output: Output::default(),
            level: default_level(),
        }
    }
}

impl Config {
    /// Create a new instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new instance from toml.
    pub fn from_toml(s: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(s)
    }

    /// Create a new instance from toml.
    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string(self)
    }

    /// Set the output stream.
    pub fn output(mut self, output: Output) -> Self {
        self.output = output;
        self
    }

    /// Set the log level.
    pub fn level(mut self, level: LevelFilter) -> Self {
        self.level = level;
        self
    }
}
