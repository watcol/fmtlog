//! Configuration module.
#[cfg(feature = "colored")]
mod colorize;
mod output;

pub use log::LevelFilter;
pub use output::Output;

#[cfg(feature = "colored")]
pub use colorize::Colorize;

/// The logger settings.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    #[cfg(feature = "colored")]
    pub(crate) colorize: Colorize,
    pub(crate) format: String,
    pub(crate) level: LevelFilter,
    pub(crate) modules: Vec<String>,
    pub(crate) output: Vec<Output>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            #[cfg(feature = "colored")]
            colorize: Colorize::default(),

            #[cfg(feature = "chrono")]
            format: String::from(crate::formats::DETAIL1),
            #[cfg(not(feature = "chrono"))]
            format: String::from(crate::formats::SIMPLE1),

            level: LevelFilter::Info,
            modules: Vec::new(),
            output: vec![Output::default()],
        }
    }
}

impl Config {
    /// Create a new instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// [**colored**] Colorize the log.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fmtlog::Config;
    ///
    /// assert_ne!(Config::new(), Config::new().colorize(false));
    /// ```
    #[cfg(feature = "colored")]
    pub fn colorize<T: Into<Colorize>>(mut self, colorize: T) -> Self {
        self.colorize = colorize.into();
        self
    }

    /// Set the format string.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fmtlog::Config;
    ///
    /// assert_ne!(Config::new(), Config::new().format("[%L] %M\n"));
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
    /// use fmtlog::{Config, LevelFilter};
    ///
    /// assert_eq!(Config::new(), Config::new().level(LevelFilter::Info))
    /// ```
    pub fn level<T: Into<LevelFilter>>(mut self, level: T) -> Self {
        self.level = level.into();
        self
    }

    /// Set modules that enable the logger.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fmtlog::{Config, LevelFilter};
    ///
    /// assert_eq!(Config::new(), Config::new().level(LevelFilter::Info))
    /// ```
    pub fn modules<T: IntoIterator>(mut self, modules: T) -> Self
    where
        T::Item: Into<String>,
    {
        self.modules = modules.into_iter().map(|x| x.into()).collect();
        self
    }

    /// Add a module that enables the logger.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fmtlog::{Config, LevelFilter};
    ///
    /// assert_eq!(Config::new(), Config::new().level(LevelFilter::Info))
    /// ```
    pub fn module<T: Into<String>>(mut self, module: T) -> Self {
        self.modules.push(module.into());
        self
    }

    /// Set the output stream.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fmtlog::Config;
    ///
    /// assert_ne!(Config::new(), Config::new().output("log.txt"))
    /// ```
    pub fn output<T: Into<Output>>(mut self, output: T) -> Self {
        self.output = vec![output.into()];
        self
    }

    /// Append the output stream.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fmtlog::{Config, Output};
    ///
    /// assert_eq!(
    ///     Config::new(),
    ///     Config::new().outputs(Vec::<Output>::new()).add_output(Output::Stderr));
    /// ```
    pub fn add_output<T: Into<Output>>(mut self, output: T) -> Self {
        self.output.push(output.into());
        self
    }

    /// Set the output streams.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fmtlog::{Config, Output};
    ///
    /// assert_eq!(Config::new(), Config::new().outputs(vec![Output::Stderr]))
    /// ```
    pub fn outputs<T: IntoIterator>(mut self, outputs: T) -> Self
    where
        T::Item: Into<Output>,
    {
        self.output = outputs.into_iter().map(|x| x.into()).collect();
        self
    }
}
