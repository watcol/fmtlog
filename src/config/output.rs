use crate::Stream;
use std::{fmt, fs, io, path};

#[cfg(feature = "serde")]
use serde::Deserialize;

/// The Output type
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "stream", content = "path"))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum Output {
    /// Standard Output
    Stdout,
    /// Standard Error
    Stderr,
    /// File Stream
    File(path::PathBuf),
}

impl fmt::Display for Output {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "{}",
            match self {
                Self::Stdout => "<stdout>",
                Self::Stderr => "<stderr>",
                Self::File(path) => path.to_str().unwrap_or("<???>"),
            }
        )
    }
}

impl Default for Output {
    fn default() -> Self {
        Self::Stderr
    }
}

impl<T: Into<path::PathBuf>> From<T> for Output {
    fn from(path: T) -> Self {
        Output::File(path.into())
    }
}

fn new_file(path: path::PathBuf) -> io::Result<fs::File> {
    fs::OpenOptions::new()
        .append(true)
        .open(path.clone())
        .or(fs::File::create(path))
}

impl Output {
    /// Create `Stream` from `Output`.
    pub(crate) fn to_stream(&self) -> io::Result<Stream> {
        Ok(match self.clone() {
            Self::Stdout => Stream::from(io::stdout()),
            Self::Stderr => Stream::from(io::stderr()),
            Self::File(path) => Stream::from(new_file(path)?),
        })
    }
}
