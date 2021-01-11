use crate::config::Output;
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
            Self::Auto => match output {
                Output::Stdout | Output::Stderr => true,
                _ => false,
            },
            Self::On => true,
        }
    }
}

#[cfg(feature = "serde")]
use serde::{de, Deserialize};

#[cfg(feature = "serde")]
struct ColorizeVisitor;

#[cfg(feature = "serde")]
impl<'de> de::Visitor<'de> for ColorizeVisitor {
    type Value = Colorize;

    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "a boolean, \"off\", \"auto\", or \"on\".")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Colorize::from(value))
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        use std::str::FromStr;

        Colorize::from_str(s)
            .map_err(|_| de::Error::invalid_value(de::Unexpected::Str(s), &self))
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Colorize {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(ColorizeVisitor)
    }
}
