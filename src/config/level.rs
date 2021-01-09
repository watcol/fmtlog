use std::fmt;
use log::{STATIC_MAX_LEVEL, LevelFilter};

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

/// A struct to wrap `log::LevelFilter`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum Level {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace
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

impl fmt::Display for Level {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", match self {
            Self::Off => "off",
            Self::Error => "error",
            Self::Warn => "warn",
            Self::Info => "info",
            Self::Debug => "debug",
            Self::Trace => "trace",
        })
    }
}

impl std::str::FromStr for Level {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "off" | "Off" | "OFF" => Ok(Self::Off),
            "error" | "Error" | "ERROR" => Ok(Self::Error),
            "warn" | "Warn" | "WARN" => Ok(Self::Warn),
            "info" | "Info" | "INFO "=> Ok(Self::Info),
            "debug" | "Debug" | "DEBUG" => Ok(Self::Debug),
            "trace" | "Trace" | "TRACE" => Ok(Self::Trace),
            "max" | "Max" | "MAX" => Ok(Self::max()),
            _ => Err(String::from("Invalid string."))
        }
    }
}

impl Level {
    fn max() -> Self {
        STATIC_MAX_LEVEL.into()
    }
}