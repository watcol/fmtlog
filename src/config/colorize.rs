use super::Output;
use std::fmt;

/// Colorize the output
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Colorize {
    /// Never colorize.
    Off,
    /// Detect by the output target.
    Auto,
    /// Always colorize.
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
            // colorize when the "output" is standard stream.
            Self::Auto => matches!(output, Output::Stdout | Output::Stderr),
            Self::On => true,
        }
    }
}
