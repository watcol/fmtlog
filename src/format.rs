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

        loop {
            let mut const_str = String::new();
            while let Some(c) = s.peek() {
                if *c != '%' {
                    const_str.push(s.next().unwrap());
                } else {
                    break;
                }
            }

            if const_str.len() != 0 {
                res.push(Element::Const(const_str));
            }

            let mut spec = String::new();

            match s.next() {
                Some('%') => match s.next() {
                    Some('%') => res.push(Element::Const(String::from("%"))),
                    Some('[') => {
                        while let Some(c) = s.peek() {
                            if *c != ']' {
                                spec.push(s.next().unwrap());
                            }
                        }

                        if s.next() != Some(']') {
                            return Err(String::from("Invalid syntax."));
                        }
                    }
                    Some(c) => spec.push(c),
                    None => return Err(String::from("Invalid syntax."))
                },
                None => break,
                _ => unreachable!(),
            }

            res.push(Element::Special(Kind::from_string(spec)?));
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
    fn from_string(s: String) -> Result<Self, String> {
        match s.as_str() {
            "B" => Ok(Kind::Body),
            "l" => Ok(Kind::LogLevelLower),
            "L" => Ok(Kind::LogLevelUpper),
            _ => Err(String::from("Invalid string.")),
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
