use crate::config::Output;
use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Colorize the output
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum Colorize {
    Off,
    Auto,
    On,
}

impl Default for Colorize {
    fn default() -> Self {
        Colorize::Auto
    }
}

impl fmt::Display for Colorize {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "{}",
            match self {
                Self::Off => "off",
                Self::Auto => "auto",
                Self::On => "on",
            }
        )
    }
}

impl std::str::FromStr for Colorize {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "off" | "Off" | "OFF" | "false" | "False" | "FALSE" => Ok(Self::Off),
            "auto" | "Auto" | "AUTO" => Ok(Self::Auto),
            "on" | "On" | "ON" | "true" | "True" | "TRUE" => Ok(Self::On),
            e => Err(format!("Inavlid string:\"{}\"", e)),
        }
    }
}

impl From<bool> for Colorize {
    fn from(b: bool) -> Self {
        match b {
            false => Self::Off,
            true => Self::On,
        }
    }
}

impl Colorize {
    /// Check whether the logger should colorize output or not.
    pub(crate) fn colorize(&self, output: &Output) -> bool {
        match self {
            Self::Off => false,
            Self::Auto => match output {
                Output::Stdout | Output::Stderr => true,
                _ => false,
            },
            Self::On => true,
        }
    }
}