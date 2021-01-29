#[cfg(feature = "colored")]
mod color;
#[cfg(feature = "colored")]
mod pallet;


#[cfg(feature = "colored")]
use colored::Colorize;
#[cfg(feature = "colored")]
use color::Color;
#[cfg(feature = "colored")]
use pallet::Pallet;

#[cfg(feature = "chrono")]
use chrono::{Local, Utc};

use log::Record;
use std::io;

/// The format structure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Format(Vec<Element>);

impl Format {
    pub(crate) fn new<T: AsRef<str>>(s: T) -> Result<Self, String> {
        Self::parse(&mut s.as_ref().chars())
    }

    #[allow(dead_code)]
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

    #[allow(unused_variables)]
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

    #[allow(unused_variables)]
    #[allow(dead_code)]
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
    #[allow(unused_variables)]
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
    Name,
    SourceFile,
    SourceFileWithLine,
    Module,
    Message,
    LogLevelLower,
    LogLevelUpper,

    #[cfg(feature = "chrono")]
    Time(String),
    #[cfg(feature = "chrono")]
    UtcTime(String),

    #[cfg(feature = "colored")]
    FgColor(Color, Format),
    #[cfg(feature = "colored")]
    FgColorBranch(Pallet, Format),
    #[cfg(feature = "colored")]
    BgColor(Color, Format),
    #[cfg(feature = "colored")]
    BgColorBranch(Pallet, Format),
    #[cfg(feature = "colored")]
    Bold(Format),
    #[cfg(feature = "colored")]
    Dimmed(Format),
    #[cfg(feature = "colored")]
    Italic(Format),
    #[cfg(feature = "colored")]
    Reversed(Format),
    #[cfg(feature = "colored")]
    Underline(Format),
    #[cfg(feature = "colored")]
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
            'N' => Ok(Self::Name),
            'f' => Ok(Self::SourceFile),
            'S' => Ok(Self::SourceFileWithLine),
            'm' => Ok(Self::Module),
            'M' => Ok(Self::Message),
            'l' => Ok(Self::LogLevelLower),
            'L' => Ok(Self::LogLevelUpper),

            #[cfg(feature = "chrono")]
            'T' => {
                if s.next() != Some('(') {
                    return Err(String::from("Missing time format."));
                }

                let format = s.take_while(|c| *c != ')').collect();

                Ok(Self::Time(format))
            }

            #[cfg(feature = "chrono")]
            'U' => {
                if s.next() != Some('(') {
                    return Err(String::from("Missing time format."));
                }

                let format = s.take_while(|c| *c != ')').collect();

                Ok(Self::UtcTime(format))
            }

            #[cfg(feature = "colored")]
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
            #[cfg(feature = "colored")]
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
            #[cfg(feature = "colored")]
            'b' => {
                if s.next() != Some('{') {
                    return Err(String::from("Missing the body."));
                }

                let format = Format::parse_until(s, '}')?;

                Ok(Self::Bold(format))
            }
            #[cfg(feature = "colored")]
            'd' => {
                if s.next() != Some('{') {
                    return Err(String::from("Missing the body."));
                }

                let format = Format::parse_until(s, '}')?;

                Ok(Self::Dimmed(format))
            }
            #[cfg(feature = "colored")]
            'i' => {
                if s.next() != Some('{') {
                    return Err(String::from("Missing the body."));
                }

                let format = Format::parse_until(s, '}')?;

                Ok(Self::Italic(format))
            }
            #[cfg(feature = "colored")]
            'r' => {
                if s.next() != Some('{') {
                    return Err(String::from("Missing the body."));
                }

                let format = Format::parse_until(s, '}')?;

                Ok(Self::Reversed(format))
            }
            #[cfg(feature = "colored")]
            'u' => {
                if s.next() != Some('{') {
                    return Err(String::from("Missing the body."));
                }

                let format = Format::parse_until(s, '}')?;

                Ok(Self::Underline(format))
            }
            #[cfg(feature = "colored")]
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

    #[allow(unused_variables)]
    fn write<W: io::Write>(
        &self,
        writer: &mut W,
        record: &Record,
        colorize: bool,
    ) -> io::Result<()> {
        match self {
            Self::Percent => write!(writer, "%"),
            Self::Close => write!(writer, "}}"),
            Self::Name => write!(writer, "{}", record.target()),
            Self::SourceFile => write!(writer, "{}", record.file().unwrap_or_default()),
            Self::SourceFileWithLine => match record.file() {
                Some(s) => match record.line() {
                    Some(l) => write!(writer, "{}:{}", s, l),
                    None => write!(writer, "{}", s),
                },
                None => Ok(()),
            },
            Self::Module => write!(writer, "{}", record.module_path().unwrap_or_default()),
            Self::Message => write!(writer, "{}", record.args()),
            Self::LogLevelUpper => write!(writer, "{}", record.level()),
            Self::LogLevelLower => write!(writer, "{}", record.level().to_string().to_lowercase()),

            #[cfg(feature = "chrono")]
            Self::Time(format) => write!(writer, "{}", Local::now().format(format)),
            #[cfg(feature = "chrono")]
            Self::UtcTime(format) => write!(writer, "{}", Utc::now().format(format)),

            #[cfg(feature = "colored")]
            Self::FgColor(color, format) => {
                let s = format.to_str(record, colorize)?;

                if colorize {
                    write!(writer, "{}", s.color(*color))
                } else {
                    write!(writer, "{}", s)
                }
            }
            #[cfg(feature = "colored")]
            Self::FgColorBranch(pallet, format) => {
                let s = format.to_str(record, colorize)?;

                if colorize {
                    write!(writer, "{}", s.color(pallet.select(record.level())))
                } else {
                    write!(writer, "{}", s)
                }
            }
            #[cfg(feature = "colored")]
            Self::BgColor(color, format) => {
                let s = format.to_str(record, colorize)?;

                if colorize {
                    write!(writer, "{}", s.on_color(*color))
                } else {
                    write!(writer, "{}", s)
                }
            }
            #[cfg(feature = "colored")]
            Self::BgColorBranch(pallet, format) => {
                let s = format.to_str(record, colorize)?;

                if colorize {
                    write!(writer, "{}", s.on_color(pallet.select(record.level())))
                } else {
                    write!(writer, "{}", s)
                }
            }
            #[cfg(feature = "colored")]
            Self::Bold(format) => {
                let s = format.to_str(record, colorize)?;

                if colorize {
                    write!(writer, "{}", s.bold())
                } else {
                    write!(writer, "{}", s)
                }
            }
            #[cfg(feature = "colored")]
            Self::Dimmed(format) => {
                let s = format.to_str(record, colorize)?;

                if colorize {
                    write!(writer, "{}", s.dimmed())
                } else {
                    write!(writer, "{}", s)
                }
            }
            #[cfg(feature = "colored")]
            Self::Italic(format) => {
                let s = format.to_str(record, colorize)?;

                if colorize {
                    write!(writer, "{}", s.italic())
                } else {
                    write!(writer, "{}", s)
                }
            }
            #[cfg(feature = "colored")]
            Self::Reversed(format) => {
                let s = format.to_str(record, colorize)?;

                if colorize {
                    write!(writer, "{}", s.reversed())
                } else {
                    write!(writer, "{}", s)
                }
            }
            #[cfg(feature = "colored")]
            Self::Underline(format) => {
                let s = format.to_str(record, colorize)?;

                if colorize {
                    write!(writer, "{}", s.underline())
                } else {
                    write!(writer, "{}", s)
                }
            }
            #[cfg(feature = "colored")]
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
