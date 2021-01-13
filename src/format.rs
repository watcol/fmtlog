use colored::{Color, Colorize};
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

    pub(crate) fn write<W: io::Write>(
        &self,
        writer: &mut W,
        record: &Record,
        color: &mut Option<Color>,
        colorize: bool,
    ) -> io::Result<()> {
        for elem in self.0.iter() {
            elem.write(writer, record, color, colorize)?;
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
    fn write<W: io::Write>(
        &self,
        writer: &mut W,
        record: &Record,
        color: &mut Option<Color>,
        colorize: bool,
    ) -> io::Result<()> {
        let s = match self {
            Self::Const(s) => s.clone(),
            Self::Special(spec) => spec.to_str(record, color),
        };

        match color {
            Some(c) if colorize => write!(writer, "{}", s.color(*c)),
            None => write!(writer, "{}", s),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Special {
    kind: Kind,
}

impl Special {
    fn parse(s: &mut std::str::Chars) -> Result<Self, String> {
        let kind = match s.next() {
            Some(c) => Kind::from_char(c)?,
            None => return Err(String::from("Unnexpected end.")),
        };

        Ok(Special { kind })
    }

    fn to_str(&self, record: &Record, color: &mut Option<Color>) -> String {
        match self.kind {
            Kind::Literal => String::from("%"),
            Kind::Body => record.args().to_string(),
            Kind::LogLevelLower => record.level().to_string().to_lowercase(),
            Kind::LogLevelUpper => record.level().to_string(),
            Kind::Green => {
                *color = Some(Color::Green);
                String::new()
            }
            Kind::Red => {
                *color = Some(Color::Red);
                String::new()
            }
            Kind::Plain => {
                *color = None;
                String::new()
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
    Red,
    Plain,
}

impl Kind {
    fn from_char(c: char) -> Result<Self, String> {
        match c {
            '%' => Ok(Kind::Literal),
            'B' => Ok(Kind::Body),
            'l' => Ok(Kind::LogLevelLower),
            'L' => Ok(Kind::LogLevelUpper),
            'G' => Ok(Kind::Green),
            'R' => Ok(Kind::Red),
            'P' => Ok(Kind::Plain),
            _ => Err(String::from("Invalid specifier.")),
        }
    }
}
