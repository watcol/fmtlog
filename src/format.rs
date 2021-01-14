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

        match *color {
            Some(c) if colorize => write!(writer, "{}", s.color(c)),
            _ => write!(writer, "{}", s),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Special {
    Literal,
    Message,
    LogLevelLower,
    LogLevelUpper,
}

impl Special {
    fn parse(s: &mut std::str::Chars) -> Result<Self, String> {
        let kind = match s.next() {
            Some(c) => c,
            None => return Err(String::from("Unnexpected end.")),
        };

        match kind {
            '%' => Ok(Self::Literal),
            'l' => Ok(Self::LogLevelLower),
            'L' => Ok(Self::LogLevelUpper),
            'M' => Ok(Self::Message),
            _ => Err(String::from("Invalid specifier.")),
        }
    }

    fn to_str(&self, record: &Record, _color: &mut Option<Color>) -> String {
        match self {
            Self::Literal => String::from("%"),
            Self::Message => record.args().to_string(),
            Self::LogLevelLower => record.level().to_string().to_lowercase(),
            Self::LogLevelUpper => record.level().to_string(),
        }
    }
}
