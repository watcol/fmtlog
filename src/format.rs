use log::Record;
use std::io;

/// The format structure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Format(Vec<Element>);

impl Format {
    pub(crate) fn parse<T: AsRef<str>>(s: T) -> Result<Format, String> {
        let mut s = s.as_ref().chars();
        let mut res = Vec::new();

        let mut const_str = String::new();
        loop {
            match s.next() {
                Some('%') => {
                    res.push(Element::Const(const_str.clone()));
                    const_str.clear();
                    res.push(Element::Special(Special::parse(&mut s)?));
                }
                Some(c) => const_str.push(c),
                None => {
                    res.push(Element::Const(const_str));
                    break;
                }
            };
        }
        Ok(Format(res))
    }

    pub(crate) fn write<W: io::Write>(&self, writer: &mut W, record: &Record) -> io::Result<()> {
        for elem in self.0.iter() {
            elem.write(writer, record)?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Element {
    Const(String),
    Special(Special),
}

impl Element {
    fn write<W: io::Write>(&self, writer: &mut W, record: &Record) -> io::Result<()> {
        match self {
            Self::Const(s) => write!(writer, "{}", s),
            Self::Special(spec) => spec.write(writer, record),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Special {
    kind: Kind,
    body: Option<Format>,
}

impl Special {
    fn parse(s: &mut std::str::Chars) -> Result<Self, String> {
        let kind = match s.next() {
            Some(c) => Kind::from_char(c)?,
            None => return Err(String::from("Unnexpected end.")),
        };

        match kind {
            Kind::Green => {}
            _ => return Ok(Special { kind, body: None }),
        }

        let body = match s.next() {
            Some('{') => {
                let mut format = String::new();
                loop {
                    match s.next() {
                        Some('}') => break,
                        Some(c) => format.push(c),
                        None => return Err(String::from("Unnexpected end.")),
                    }
                }
                Format::parse(format)?
            }
            _ => return Err(String::from("Missing the body.")),
        };

        Ok(Special { kind, body: Some(body) })
    }

    fn write<W: io::Write>(&self, writer: &mut W, record: &Record) -> io::Result<()> {
        match self.kind {
            Kind::Literal => write!(writer, "%"),
            Kind::Body => write!(writer, "{}", record.args()),
            Kind::LogLevelLower => write!(writer, "{}", record.level().to_string().to_lowercase()),
            Kind::LogLevelUpper => write!(writer, "{}", record.level().to_string()),
            Kind::Green => {
                use colored::Colorize;

                let mut buf = Vec::new();
                self.body.as_ref().unwrap().write(&mut buf, record)?;
                write!(writer, "{}", String::from_utf8(buf).unwrap().green())
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    Literal,
    Body,
    LogLevelLower,
    LogLevelUpper,
    Green,
}

impl Kind {
    fn from_char(c: char) -> Result<Self, String> {
        match c {
            '%' => Ok(Kind::Literal),
            'B' => Ok(Kind::Body),
            'l' => Ok(Kind::LogLevelLower),
            'L' => Ok(Kind::LogLevelUpper),
            'G' => Ok(Kind::Green),
            _ => Err(String::from("Invalid specifier.")),
        }
    }
}
