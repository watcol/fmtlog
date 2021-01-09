//! Configuration module.
mod output;

pub use output::Output;
pub use log::LevelFilter;

/// The logger settings.
#[derive(Clone, Debug)]
pub struct Config {
    pub(crate) output: Output,
    pub(crate) level: LevelFilter,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            output: Output::default(),
            level: LevelFilter::Info,
        }
    }
}

impl Config {
    /// Create a new instance.
    pub fn new() -> Self {
        Self::default()
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
