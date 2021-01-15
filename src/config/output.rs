use crate::Stream;
use std::{fmt, fs, io, path};

/// The Output type
#[derive(Clone, Debug, PartialEq, Eq)]
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

impl std::str::FromStr for Output {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(match s {
            "stdout" | "Stdout" | "StdOut" | "STDOUT" | "<stdout>" => Self::Stdout,
            "stderr" | "Stderr" | "StdErr" | "STDERR" | "<stderr>" => Self::Stderr,
            path => Self::File(path.into()),
        })
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
