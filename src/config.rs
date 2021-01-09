//! Configuration module.
mod output;

pub use output::Output;

/// The logger settings.
#[derive(Default, Clone, Debug)]
pub struct Config {
    pub(crate) output: Output,
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
}
