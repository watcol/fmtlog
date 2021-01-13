use std::io;
use log::Record;
use crate::Stream;

/// The format structure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Format(Vec<Element>);

impl Format {
    pub(crate) fn parse<T: AsRef<str>>(s: T) -> Result<Format, String> {
        let mut s = s.as_ref().chars().peekable();
        let mut res = Vec::new();

        let mut const_str = String::new();
        loop {
            match s.next() {
                Some('%') => match s.next() {
                    Some('%') => const_str.push('%'),
                    Some(c) => {
                        res.push(Element::Const(const_str.clone()));
                        const_str.clear();
                        res.push(Element::Special(Kind::from_char(c)?));
                    },
                    None => return Err(String::from("Unnexpected end.")),
                },
                Some(c) => const_str.push(c),
                None => {
                    res.push(Element::Const(const_str));
                    break;
                },
            };
        }
        Ok(Format(res))
    }

    pub(crate) fn write(&self, writer: &mut Stream, record: &Record) -> io::Result<()> {
        for elem in self.0.iter() {
            elem.write(writer, record)?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Element {
    Const(String),
    Special(Kind),
}

impl Element {
    fn write(&self, writer: &mut Stream, record: &Record) -> io::Result<()> {
        use io::Write;

        match self {
            Self::Const(s) => write!(writer, "{}", s),
            Self::Special(kind) => write!(writer, "{}", kind.to_str(record)),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    Body,
    LogLevelUpper,
    LogLevelLower,
}

impl Kind {
    fn from_char(c: char) -> Result<Self, String> {
        match c {
            'B' => Ok(Kind::Body),
            'l' => Ok(Kind::LogLevelLower),
            'L' => Ok(Kind::LogLevelUpper),
            _ => Err(String::from("Invalid specifier.")),
        }
    }

    fn to_str(&self, record: &Record) -> String {
        match self {
            Self::Body => format!("{}", record.args()),
            Self::LogLevelLower => record.level().to_string().to_lowercase(),
            Self::LogLevelUpper => record.level().to_string(),
        }
    }
}
