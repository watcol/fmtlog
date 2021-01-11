use log::{LevelFilter, STATIC_MAX_LEVEL};
use std::fmt;

/// A struct to wrap [`log::LevelFilter`](https://docs.rs/log/0.4/log/enum.LevelFilter.html).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl Default for Level {
    fn default() -> Self {
        Self::Info
    }
}

impl From<LevelFilter> for Level {
    fn from(level: LevelFilter) -> Self {
        match level {
            LevelFilter::Off => Self::Off,
            LevelFilter::Error => Self::Error,
            LevelFilter::Warn => Self::Warn,
            LevelFilter::Info => Self::Info,
            LevelFilter::Debug => Self::Debug,
            LevelFilter::Trace => Self::Trace,
        }
    }
}

impl Into<LevelFilter> for Level {
    fn into(self) -> LevelFilter {
        match self {
            Self::Off => LevelFilter::Off,
            Self::Error => LevelFilter::Error,
            Self::Warn => LevelFilter::Warn,
            Self::Info => LevelFilter::Info,
            Self::Debug => LevelFilter::Debug,
            Self::Trace => LevelFilter::Trace,
        }
    }
}

impl PartialEq<LevelFilter> for Level {
    fn eq(&self, other: &LevelFilter) -> bool {
        *self == Self::from(*other)
    }
}

impl PartialOrd<LevelFilter> for Level {
    fn partial_cmp(&self, other: &LevelFilter) -> Option<std::cmp::Ordering> {
        Some((*self).cmp(&Self::from(*other)))
    }
}

impl PartialEq<log::Level> for Level {
    fn eq(&self, other: &log::Level) -> bool {
        Into::<LevelFilter>::into(*self) == *other
    }
}

impl PartialOrd<log::Level> for Level {
    fn partial_cmp(&self, other: &log::Level) -> Option<std::cmp::Ordering> {
        Into::<LevelFilter>::into(*self).partial_cmp(other)
    }
}

impl fmt::Display for Level {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "{}",
            match self {
                Self::Off => "off",
                Self::Error => "error",
                Self::Warn => "warn",
                Self::Info => "info",
                Self::Debug => "debug",
                Self::Trace => "trace",
            }
        )
    }
}

impl std::str::FromStr for Level {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "off" | "Off" | "OFF" => Ok(Self::Off),
            "error" | "Error" | "ERROR" => Ok(Self::Error),
            "warn" | "Warn" | "WARN" => Ok(Self::Warn),
            "info" | "Info" | "INFO " => Ok(Self::Info),
            "debug" | "Debug" | "DEBUG" => Ok(Self::Debug),
            "trace" | "Trace" | "TRACE" => Ok(Self::Trace),
            "max" | "Max" | "MAX" => Ok(Self::max()),
            _ => Err(String::from("Invalid string.")),
        }
    }
}

impl Level {
    /// Return the most verbose level like
    /// [`LevelFilter::max`](https://docs.rs/log/0.4/log/enum.LevelFilter.html#method.max).
    ///
    /// # Example
    ///
    /// ```rust
    /// use fmtlog::config::Level;
    ///
    /// assert_eq!(Level::max(), Level::Trace);
    /// ```
    pub fn max() -> Self {
        STATIC_MAX_LEVEL.into()
    }
}

#[cfg(feature = "serde")]
use serde::{de, Deserialize};

#[cfg(feature = "serde")]
struct LevelVisitor;

#[cfg(feature = "serde")]
impl<'de> de::Visitor<'de> for LevelVisitor {
    type Value = Level;

    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "\"off\", \"error\", \"warn\", \"info\", \"debug\", \"trace\", or \"max\"."
        )
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        use std::str::FromStr;

        Level::from_str(s).map_err(|_| de::Error::invalid_value(de::Unexpected::Str(s), &self))
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Level {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_str(LevelVisitor)
    }
}
