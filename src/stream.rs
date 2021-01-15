use std::{fs, io};

/// Stream
#[derive(Debug)]
pub enum Stream {
    Stdout(io::Stdout),
    Stderr(io::Stderr),
    File(fs::File),
}

impl From<io::Stdout> for Stream {
    fn from(s: io::Stdout) -> Self {
        Stream::Stdout(s)
    }
}

impl From<io::Stderr> for Stream {
    fn from(s: io::Stderr) -> Self {
        Stream::Stderr(s)
    }
}

impl From<fs::File> for Stream {
    fn from(s: fs::File) -> Self {
        Stream::File(s)
    }
}

impl io::Write for Stream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Stream::Stdout(w) => w.write(buf),
            Stream::Stderr(w) => w.write(buf),
            Stream::File(w) => w.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            Stream::Stdout(w) => w.flush(),
            Stream::Stderr(w) => w.flush(),
            Stream::File(w) => w.flush(),
        }
    }
}
