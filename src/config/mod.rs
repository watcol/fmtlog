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
use serde::Deserialize;

/// The logger settings.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
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

    /// **[conf-json]** Create a new instance from JSON.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fmtlog::config::Config;
    ///
    /// let json = r#"{
    ///     "colorize":"auto",
    ///     "level":"info",
    ///     "output":"stderr"
    /// }"#;
    ///
    /// assert_eq!(Config::from_json(json).unwrap(), Config::new());
    /// ```
    #[cfg(feature = "conf-json")]
    pub fn from_json<T: AsRef<str>>(s: T) -> serde_json::Result<Self> {
        serde_json::from_str(s.as_ref())
    }

    /// **[conf-yaml]** Create a new instance from YAML.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fmtlog::config::Config;
    ///
    /// let yaml = r#"
    ///     colorize: auto
    ///     level: info
    ///     output: stderr
    /// "#;
    ///
    /// assert_eq!(Config::from_yaml(yaml).unwrap(), Config::new());
    /// ```
    #[cfg(feature = "conf-yaml")]
    pub fn from_yaml<T: AsRef<str>>(s: T) -> serde_yaml::Result<Self> {
        serde_yaml::from_str(s.as_ref())
    }

    /// **[conf-toml]** Create a new instance from TOML.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fmtlog::config::Config;
    ///
    /// let toml = r#"
    ///     colorize = "auto"
    ///     level = "info"
    ///     output = "stderr"
    /// "#;
    ///
    /// assert_eq!(Config::from_toml(toml).unwrap(), Config::new());
    /// ```
    #[cfg(feature = "conf-toml")]
    pub fn from_toml<T: AsRef<str>>(s: T) -> Result<Self, toml::de::Error> {
        toml::from_str(s.as_ref())
    }

    /// Colorize the log.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fmtlog::config::Config;
    ///
    /// assert_ne!(Config::new(), Config::new().colorize(false));
    /// ```
    pub fn colorize<T: Into<Colorize>>(mut self, colorize: T) -> Self {
        self.colorize = colorize.into();
        self
    }

    /// Set the log level.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fmtlog::config::{Config, Level};
    ///
    /// assert_eq!(Config::new(), Config::new().level(Level::Info))
    /// ```
    pub fn level<T: Into<Level>>(mut self, level: T) -> Self {
        self.level = level.into();
        self
    }

    /// Set the output stream.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fmtlog::config::Config;
    ///
    /// assert_ne!(Config::new(), Config::new().output("log.txt"))
    /// ```
    pub fn output<T: Into<Output>>(mut self, output: T) -> Self {
        self.output = output.into();
        self
    }
}
