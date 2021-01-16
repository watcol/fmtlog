mod color;

use colored::Colorize;
use log::{Level, Record};
use std::io;

use color::Color;

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
        colorize: bool,
    ) -> io::Result<()> {
        for elem in self.0.iter() {
            elem.write(writer, record, colorize)?;
        }

        Ok(())
    }

    fn to_str(&self, record: &Record, colorize: bool) -> io::Result<String> {
        let mut buf: Vec<u8> = Vec::new();
        self.write(&mut buf, record, colorize)?;
        Ok(String::from_utf8(buf).unwrap())
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
        colorize: bool,
    ) -> io::Result<()> {
        match self {
            Self::Const(s) => write!(writer, "{}", s),
            Self::Special(spec) => spec.write(writer, record, colorize),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Special {
    Percent,
    Close,
    Message,
    LogLevelLower,
    LogLevelUpper,
    FgColor(Color, Format),
    FgColorBranch(Color, Color, Color, Color, Color, Format),
    BgColor(Color, Format),
    BgColorBranch(Color, Color, Color, Color, Color, Format),
    Bold(Format),
    Underline(Format),
}

impl Special {
    fn parse<T: Iterator<Item = char>>(s: &mut T) -> Result<Self, String> {
        let kind = match s.next() {
            Some(c) => c,
            None => return Err(String::from("Unnexpected end.")),
        };

        match kind {
            '%' => Ok(Self::Percent),
            '}' => Ok(Self::Close),
            'M' => Ok(Self::Message),
            'l' => Ok(Self::LogLevelLower),
            'L' => Ok(Self::LogLevelUpper),
            'f' => {
                if s.next() != Some('(') {
                    return Err(String::from("Missing color specifier."));
                }

                let color = Color::parse_until(s, ')')?;

                if s.next() != Some('{') {
                    return Err(String::from("Missing the body."));
                }

                let format = Format::parse_until(s, '}')?;

                Ok(Self::FgColor(color, format))
            }
            'F' => {
                if s.next() != Some('(') {
                    return Err(String::from("Missing color specifier."));
                }

                let e = Color::parse_until(s, ',')?;
                let w = Color::parse_until(s, ',')?;
                let i = Color::parse_until(s, ',')?;
                let d = Color::parse_until(s, ',')?;
                let t = Color::parse_until(s, ')')?;

                if s.next() != Some('{') {
                    return Err(String::from("Missing the body."));
                }

                let format = Format::parse_until(s, '}')?;

                Ok(Self::FgColorBranch(e, w, i, d, t, format))
            }
            'b' => {
                if s.next() != Some('(') {
                    return Err(String::from("Missing color specifier."));
                }

                let color = Color::parse_until(s, ')')?;

                if s.next() != Some('{') {
                    return Err(String::from("Missing the body."));
                }

                let format = Format::parse_until(s, '}')?;

                Ok(Self::BgColor(color, format))
            }
            'B' => {
                if s.next() != Some('(') {
                    return Err(String::from("Missing color specifier."));
                }

                let e = Color::parse_until(s, ',')?;
                let w = Color::parse_until(s, ',')?;
                let i = Color::parse_until(s, ',')?;
                let d = Color::parse_until(s, ',')?;
                let t = Color::parse_until(s, ')')?;

                if s.next() != Some('{') {
                    return Err(String::from("Missing the body."));
                }

                let format = Format::parse_until(s, '}')?;

                Ok(Self::BgColorBranch(e, w, i, d, t, format))
            }
            'O' => {
                if s.next() != Some('{') {
                    return Err(String::from("Missing the body."));
                }

                let format = Format::parse_until(s, '}')?;

                Ok(Self::Bold(format))
            }
            'U' => {
                if s.next() != Some('{') {
                    return Err(String::from("Missing the body."));
                }

                let format = Format::parse_until(s, '}')?;

                Ok(Self::Underline(format))
            }
            _ => Err(String::from("Invalid specifier.")),
        }
    }

    fn write<W: io::Write>(&self, writer: &mut W, record: &Record, colorize: bool) -> io::Result<()> {
        match self {
            Self::Percent => write!(writer, "%"),
            Self::Close => write!(writer, "}}"),
            Self::Message => write!(writer, "{}", record.args()),
            Self::LogLevelUpper => write!(writer, "{}", record.level()),
            Self::LogLevelLower => write!(writer, "{}", record.level().to_string().to_lowercase()),
            Self::FgColor(color, format) => {
                let s = format.to_str(record, colorize)?;

                if colorize  {
                    write!(writer, "{}", s.color(*color))
                } else {
                    write!(writer, "{}", s)
                }
            }
            Self::FgColorBranch(e, w, i, d, t, format) => {
                let s = format.to_str(record, colorize)?;

                let color = *match record.level() {
                    Level::Error => e,
                    Level::Warn => w,
                    Level::Info => i,
                    Level::Debug => d,
                    Level::Trace => t,
                };

                if colorize {
                    write!(writer, "{}", s.color(color))
                } else {
                    write!(writer, "{}", s)
                }
            }
            Self::BgColor(color, format) => {
                let s = format.to_str(record, colorize)?;

                if colorize  {
                    write!(writer, "{}", s.on_color(*color))
                } else {
                    write!(writer, "{}", s)
                }
            }
            Self::BgColorBranch(e, w, i, d, t, format) => {
                let s = format.to_str(record, colorize)?;

                let color = *match record.level() {
                    Level::Error => e,
                    Level::Warn => w,
                    Level::Info => i,
                    Level::Debug => d,
                    Level::Trace => t,
                };

                if colorize {
                    write!(writer, "{}", s.on_color(color))
                } else {
                    write!(writer, "{}", s)
                }
            }
            Self::Bold(format) => {
                let s = format.to_str(record, colorize)?;

                if colorize  {
                    write!(writer, "{}", s.bold())
                } else {
                    write!(writer, "{}", s)
                }
            }
            Self::Underline(format) => {
                let s = format.to_str(record, colorize)?;

                if colorize  {
                    write!(writer, "{}", s.underline())
                } else {
                    write!(writer, "{}", s)
                }
            }
        }
    }
}
