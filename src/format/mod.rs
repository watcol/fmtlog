mod style;

use colored::{Color, Colorize};
use log::Record;
use std::io;

pub use style::Style;

/// The format structure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Format(Vec<Element>);

impl Format {
    pub(crate) fn new<T: AsRef<str>>(s: T) -> Result<Self, String> {
        Self::parse(&mut s.as_ref().chars())
    }

    fn parse_until<T: Iterator<Item = char>>(s: &mut T, ch: char) -> Result<Self, String> {
        let mut res = Vec::new();

        let mut const_str = String::new();
        loop {
            match s.next() {
                Some('%') => {
                    res.push(Element::Const(const_str.clone()));
                    const_str.clear();
                    res.push(Element::Special(Special::parse(s)?));
                }
                Some(c) if c == ch => {
                    res.push(Element::Const(const_str));
                    break;
                }
                None => {
                    res.push(Element::Const(const_str));
                    break;
                }
                Some(c) => const_str.push(c),
            };
        }
        Ok(Format(res))
    }

    fn parse<T: Iterator<Item = char>>(s: &mut T) -> Result<Self, String> {
        let mut res = Vec::new();

        let mut const_str = String::new();
        loop {
            match s.next() {
                Some('%') => {
                    res.push(Element::Const(const_str.clone()));
                    const_str.clear();
                    res.push(Element::Special(Special::parse(s)?));
                }
                None => {
                    res.push(Element::Const(const_str));
                    break;
                }
                Some(c) => const_str.push(c),
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

        if style.bold {
            s = s.bold();
        }

        if style.underline {
            s = s.underline();
        }

        write!(writer, "{}", s)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Special {
    Percent,
    Close,
    Comma,
    Branch(Format, Format, Format, Format, Format),
    Message,
    LogLevelLower,
    LogLevelUpper,
    Color(Color),
    NoColor,
    OnColor(Color),
    NoOnColor,
    Bold,
    NoBold,
    Underline,
    NoUnderline,
}

impl Special {
    fn parse<T: Iterator<Item = char>>(s: &mut T) -> Result<Self, String> {
        let kind = match s.next() {
            Some(c) => c,
            None => return Err(String::from("Unnexpected end.")),
        };

        match kind {
            '%' => Ok(Self::Percent),
            ')' => Ok(Self::Close),
            ',' => Ok(Self::Comma),
            '(' => {
                let e = Format::parse_until(s, ',')?;
                let w = Format::parse_until(s, ',')?;
                let i = Format::parse_until(s, ',')?;
                let d = Format::parse_until(s, ',')?;
                let t = Format::parse_until(s, ')')?;

                Ok(Self::Branch(e, w, i, d, t))
            }
            'M' => Ok(Self::Message),
            'l' => Ok(Self::LogLevelLower),
            'L' => Ok(Self::LogLevelUpper),
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
            }
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
            }
            'o' => Ok(Self::NoOnColor),
            'B' => Ok(Self::Bold),
            'b' => Ok(Self::NoBold),
            'U' => Ok(Self::Underline),
            'u' => Ok(Self::NoUnderline),
            _ => Err(String::from("Invalid specifier.")),
        }
    }

    fn to_str(&self, record: &Record, style: &mut Style) -> String {
        match self {
            Self::Percent => String::from("%"),
            Self::Close => String::from(")"),
            Self::Comma => String::from(","),
            Self::Branch(e, w, i, d, t) => {
                use log::Level;
                let mut buf: Vec<u8> = Vec::new();
                match record.level() {
                    Level::Error => e.write(&mut buf, record, style, false),
                    Level::Warn => w.write(&mut buf, record, style, false),
                    Level::Info => i.write(&mut buf, record, style, false),
                    Level::Debug => d.write(&mut buf, record, style, false),
                    Level::Trace => t.write(&mut buf, record, style, false),
                }.expect("Failed to write");

                String::from_utf8(buf).unwrap()
            }
            Self::Message => record.args().to_string(),
            Self::LogLevelLower => record.level().to_string().to_lowercase(),
            Self::LogLevelUpper => record.level().to_string(),
            Self::Color(c) => {
                style.fg = Some(*c);
                String::new()
            }
            Self::NoColor => {
                style.fg = None;
                String::new()
            }
            Self::OnColor(c) => {
                style.bg = Some(*c);
                String::new()
            }
            Self::NoOnColor => {
                style.bg = None;
                String::new()
            }
            Self::Bold => {
                style.bold = true;
                String::new()
            }
            Self::NoBold => {
                style.bold = false;
                String::new()
            }
            Self::Underline => {
                style.underline = true;
                String::new()
            }
            Self::NoUnderline => {
                style.underline = false;
                String::new()
            }
        }
    }
}
