mod color;
mod pallet;

use colored::Colorize;
use chrono::{Local, Utc};
use log::Record;
use std::io;

use color::Color;
use pallet::Pallet;

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
    Time(String),
    UtcTime(String),
    FgColor(Color, Format),
    FgColorBranch(Pallet, Format),
    BgColor(Color, Format),
    BgColorBranch(Pallet, Format),
    Bold(Format),
    Dimmed(Format),
    Italic(Format),
    Reversed(Format),
    Underline(Format),
    StrikeThrough(Format),
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
            'T' => {
                if s.next() != Some('(') {
                    return Err(String::from("Missing time format."));
                }

                let format = s.take_while(|c| *c != ')').collect();

                Ok(Self::Time(format))
            }
            'U' => {
                if s.next() != Some('(') {
                    return Err(String::from("Missing time format."));
                }

                let format = s.take_while(|c| *c != ')').collect();

                Ok(Self::UtcTime(format))
            }
            'F' => {
                use std::str::FromStr;

                if s.next() != Some('(') {
                    return Err(String::from("Missing color specifier."));
                }

                let mut color = String::new();
                let branch = loop {
                    match s.next() {
                        Some(')') => break false,
                        Some(',') => break true,
                        Some(c) => color.push(c),
                        None => return Err(String::from("Unnexpected end.")),
                    }
                };

                let color = Color::from_str(&color)?;

                if branch {
                    let error = color;
                    let warn = Color::parse_until(s, ',')?;
                    let info = Color::parse_until(s, ',')?;
                    let debug = Color::parse_until(s, ',')?;
                    let trace = Color::parse_until(s, ')')?;

                    if s.next() != Some('{') {
                        return Err(String::from("Missing the body."));
                    }

                    let format = Format::parse_until(s, '}')?;

                    Ok(Self::FgColorBranch(
                        Pallet {
                            error,
                            warn,
                            info,
                            debug,
                            trace,
                        },
                        format,
                    ))
                } else {
                    if s.next() != Some('{') {
                        return Err(String::from("Missing the body."));
                    }

                    let format = Format::parse_until(s, '}')?;

                    Ok(Self::FgColor(color, format))
                }
            }
            'B' => {
                use std::str::FromStr;

                if s.next() != Some('(') {
                    return Err(String::from("Missing color specifier."));
                }

                let mut color = String::new();
                let branch = loop {
                    match s.next() {
                        Some(')') => break false,
                        Some(',') => break true,
                        Some(c) => color.push(c),
                        None => return Err(String::from("Unnexpected end.")),
                    }
                };

                let color = Color::from_str(&color)?;

                if branch {
                    let error = color;
                    let warn = Color::parse_until(s, ',')?;
                    let info = Color::parse_until(s, ',')?;
                    let debug = Color::parse_until(s, ',')?;
                    let trace = Color::parse_until(s, ')')?;

                    if s.next() != Some('{') {
                        return Err(String::from("Missing the body."));
                    }

                    let format = Format::parse_until(s, '}')?;

                    Ok(Self::BgColorBranch(
                        Pallet {
                            error,
                            warn,
                            info,
                            debug,
                            trace,
                        },
                        format,
                    ))
                } else {
                    if s.next() != Some('{') {
                        return Err(String::from("Missing the body."));
                    }

                    let format = Format::parse_until(s, '}')?;

                    Ok(Self::BgColor(color, format))
                }
            }
            'b' => {
                if s.next() != Some('{') {
                    return Err(String::from("Missing the body."));
                }

                let format = Format::parse_until(s, '}')?;

                Ok(Self::Bold(format))
            }
            'd' => {
                if s.next() != Some('{') {
                    return Err(String::from("Missing the body."));
                }

                let format = Format::parse_until(s, '}')?;

                Ok(Self::Dimmed(format))
            }
            'i' => {
                if s.next() != Some('{') {
                    return Err(String::from("Missing the body."));
                }

                let format = Format::parse_until(s, '}')?;

                Ok(Self::Italic(format))
            }
            'r' => {
                if s.next() != Some('{') {
                    return Err(String::from("Missing the body."));
                }

                let format = Format::parse_until(s, '}')?;

                Ok(Self::Reversed(format))
            }
            'u' => {
                if s.next() != Some('{') {
                    return Err(String::from("Missing the body."));
                }

                let format = Format::parse_until(s, '}')?;

                Ok(Self::Underline(format))
            }
            's' => {
                if s.next() != Some('{') {
                    return Err(String::from("Missing the body."));
                }

                let format = Format::parse_until(s, '}')?;

                Ok(Self::StrikeThrough(format))
            }
            _ => Err(String::from("Invalid specifier.")),
        }
    }

    fn write<W: io::Write>(
        &self,
        writer: &mut W,
        record: &Record,
        colorize: bool,
    ) -> io::Result<()> {
        match self {
            Self::Percent => write!(writer, "%"),
            Self::Close => write!(writer, "}}"),
            Self::Message => write!(writer, "{}", record.args()),
            Self::LogLevelUpper => write!(writer, "{}", record.level()),
            Self::LogLevelLower => write!(writer, "{}", record.level().to_string().to_lowercase()),
            Self::Time(format) => write!(writer, "{}", Local::now().format(format)),
            Self::UtcTime(format) => write!(writer, "{}", Utc::now().format(format)),
            Self::FgColor(color, format) => {
                let s = format.to_str(record, colorize)?;

                if colorize {
                    write!(writer, "{}", s.color(*color))
                } else {
                    write!(writer, "{}", s)
                }
            }
            Self::FgColorBranch(pallet, format) => {
                let s = format.to_str(record, colorize)?;

                if colorize {
                    write!(writer, "{}", s.color(pallet.select(record.level())))
                } else {
                    write!(writer, "{}", s)
                }
            }
            Self::BgColor(color, format) => {
                let s = format.to_str(record, colorize)?;

                if colorize {
                    write!(writer, "{}", s.on_color(*color))
                } else {
                    write!(writer, "{}", s)
                }
            }
            Self::BgColorBranch(pallet, format) => {
                let s = format.to_str(record, colorize)?;

                if colorize {
                    write!(writer, "{}", s.on_color(pallet.select(record.level())))
                } else {
                    write!(writer, "{}", s)
                }
            }
            Self::Bold(format) => {
                let s = format.to_str(record, colorize)?;

                if colorize {
                    write!(writer, "{}", s.bold())
                } else {
                    write!(writer, "{}", s)
                }
            }
            Self::Dimmed(format) => {
                let s = format.to_str(record, colorize)?;

                if colorize {
                    write!(writer, "{}", s.dimmed())
                } else {
                    write!(writer, "{}", s)
                }
            }
            Self::Italic(format) => {
                let s = format.to_str(record, colorize)?;

                if colorize {
                    write!(writer, "{}", s.italic())
                } else {
                    write!(writer, "{}", s)
                }
            }
            Self::Reversed(format) => {
                let s = format.to_str(record, colorize)?;

                if colorize {
                    write!(writer, "{}", s.reversed())
                } else {
                    write!(writer, "{}", s)
                }
            }
            Self::Underline(format) => {
                let s = format.to_str(record, colorize)?;

                if colorize {
                    write!(writer, "{}", s.underline())
                } else {
                    write!(writer, "{}", s)
                }
            }
            Self::StrikeThrough(format) => {
                let s = format.to_str(record, colorize)?;

                if colorize {
                    write!(writer, "{}", s.strikethrough())
                } else {
                    write!(writer, "{}", s)
                }
            }
        }
    }
}
