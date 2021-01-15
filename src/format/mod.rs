mod style;

use colored::{Color, Colorize};
use log::Record;
use std::io;

pub use style::Style;

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
        style: &mut Style,
        colorize: bool,
    ) -> io::Result<()> {
        for elem in self.0.iter() {
            elem.write(writer, record, style, colorize)?;
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
        style: &mut Style,
        colorize: bool,
    ) -> io::Result<()> {
        let s = match self {
            Self::Const(s) => s.clone(),
            Self::Special(spec) => spec.to_str(record, style),
        };

        if !colorize {
            return write!(writer, "{}", s);
        }

        let mut s = colored::ColoredString::from(s.as_ref());

        if let Some(c) = style.fg {
            s = s.color(c);
        }

        if let Some(c) = style.bg {
            s = s.on_color(c);
        }

        write!(writer, "{}", s)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Special {
    Literal,
    Message,
    LogLevelLower,
    LogLevelUpper,
    Color(Color),
    NoColor,
    OnColor(Color),
    NoOnColor,
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
            'C' => {
                use std::str::FromStr;

                if s.next() != Some('(') {
                    return Err(String::from("Missing color specifier."));
                }

                let mut color = String::new();
                loop {
                    match s.next() {
                        Some(')') => break,
                        Some(c) => color.push(c),
                        None => return Err(String::from("Unnexpected end.")),
                    }
                }

                Ok(Self::Color(
                    Color::from_str(&color).map_err(|_| "Unnexpected color specifier.")?,
                ))
            },
            'c' => Ok(Self::NoColor),
            'O' => {
                use std::str::FromStr;

                if s.next() != Some('(') {
                    return Err(String::from("Missing color specifier."));
                }

                let mut color = String::new();
                loop {
                    match s.next() {
                        Some(')') => break,
                        Some(c) => color.push(c),
                        None => return Err(String::from("Unnexpected end.")),
                    }
                }

                Ok(Self::OnColor(
                    Color::from_str(&color).map_err(|_| "Unnexpected color specifier.")?,
                ))
            },
            'o' => Ok(Self::NoOnColor),
            _ => Err(String::from("Invalid specifier.")),
        }
    }

    fn to_str(&self, record: &Record, style: &mut Style) -> String {
        match self {
            Self::Literal => String::from("%"),
            Self::Message => record.args().to_string(),
            Self::LogLevelLower => record.level().to_string().to_lowercase(),
            Self::LogLevelUpper => record.level().to_string(),
            Self::Color(c) => {
                style.fg = Some(*c);
                String::new()
            },
            Self::NoColor => {
                style.fg = None;
                String::new()
            }
            Self::OnColor(c) => {
                style.bg = Some(*c);
                String::new()
            },
            Self::NoOnColor => {
                style.bg = None;
                String::new()
            }
        }
    }
}
