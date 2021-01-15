//! Configuration module.
mod colorize;
mod output;

pub use colorize::Colorize;
pub use output::Output;
pub use log::LevelFilter;

fn default_format() -> String {
    String::from("%l: %M\n")
}

/// The logger settings.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub(crate) colorize: Colorize,
    pub(crate) format: String,
    pub(crate) level: LevelFilter,
    pub(crate) output: Output,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            colorize: Colorize::default(),
            format: default_format(),
            level: LevelFilter::Info,
            output: Output::default(),
        }
    }
}

impl Config {
    /// Create a new instance.
    pub fn new() -> Self {
        Self::default()
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

    /// Set the output format.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fmtlog::config::Config;
    ///
    /// assert_ne!(Config::new(), Config::new().colorize(false));
    /// ```
    pub fn format<T: Into<String>>(mut self, format: T) -> Self {
        self.format = format.into();
        self
    }

    /// Set the log level.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fmtlog::config::{Config, LevelFilter};
    ///
    /// assert_eq!(Config::new(), Config::new().level(LevelFilter::Info))
    /// ```
    pub fn level<T: Into<LevelFilter>>(mut self, level: T) -> Self {
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
