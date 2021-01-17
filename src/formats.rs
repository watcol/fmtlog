//! Formats Collection
//!
//! This module provides a list of `const` strings
//! for easy format specification.
//!
//! # Usage
//! ```rust
//! use fmtlog::Config;
//! use fmtlog::formats::SIMPLE1; // A format to use.
//!
//! fmtlog::new(Config::new().format(SIMPLE1)).set().unwrap();
//!
//! log::error!("Example Message");
//! log::warn!("Example Message");
//! log::info!("Example Message");
//! log::debug!("Example Message");
//! log::trace!("Example Message");
//! ```
//!
//! # Collection
//! ## SIMPLE1
//! A simplest format.
//!
//! <pre>
//! <font color="red"><b>ERROR</b></font>: Example Message
//! <font color="yellow"><b>WARN</b></font>: Example Message
//! <font color="green"><b>INFO</b></font>: Example Message
//! <font color="cyan"><b>DEBUG</b></font>: Example Message
//! <font color="blue"><b>TRACE</b></font>: Example Message
//! </pre>
//!
//! ## SIMPLE1_LOWER
//! A simplest format. (lowercase version)
//!
//! <pre>
//! <font color="red"><b>error</b></font>: Example Message
//! <font color="yellow"><b>warn</b></font>: Example Message
//! <font color="green"><b>info</b></font>: Example Message
//! <font color="cyan"><b>debug</b></font>: Example Message
//! <font color="blue"><b>trace</b></font>: Example Message
//! </pre>
//!
//! ## SIMPLE2
//! A simplest format. (bracket version)
//!
//! <pre>
//! [<font color="red"><b>ERROR</b></font>] Example Message
//! [<font color="yellow"><b>WARN</b></font>] Example Message
//! [<font color="green"><b>INFO</b></font>] Example Message
//! [<font color="cyan"><b>DEBUG</b></font>] Example Message
//! [<font color="blue"><b>TRACE</b></font>] Example Message
//! </pre>
//!
//! ## SIMPLE2_LOWER
//! A simplest format. (bracket lowercase version)
//!
//! <pre>
//! [<font color="red"><b>error</b></font>] Example Message
//! [<font color="yellow"><b>warn</b></font>] Example Message
//! [<font color="green"><b>info</b></font>] Example Message
//! [<font color="cyan"><b>debug</b></font>] Example Message
//! [<font color="blue"><b>trace</b></font>] Example Message
//! </pre>
//!
//! ## DETAIL1
//! A detailed version of [`SIMPLE1`](#simple1).
//!
//! <pre>
//! [2021/01/01 12:00:00 fmtlog] <font color="red"><b>ERROR</b></font>: Example Message
//! [2021/01/01 12:00:00 fmtlog] <font color="yellow"><b>WARN</b></font>: Example Message
//! [2021/01/01 12:00:00 fmtlog] <font color="green"><b>INFO</b></font>: Example Message
//! [2021/01/01 12:00:00 fmtlog] <font color="cyan"><b>DEBUG</b></font>: Example Message
//! [2021/01/01 12:00:00 fmtlog] <font color="blue"><b>TRACE</b></font>: Example Message
//! </pre>
//!
//! ## DETAIL1_LOWER
//! A detailed version of [`SIMPLE1_LOWER`](#simple1-lower).
//!
//! <pre>
//! [2021/01/01 12:00:00 fmtlog] <font color="red"><b>error</b></font>: Example Message
//! [2021/01/01 12:00:00 fmtlog] <font color="yellow"><b>warn</b></font>: Example Message
//! [2021/01/01 12:00:00 fmtlog] <font color="green"><b>info</b></font>: Example Message
//! [2021/01/01 12:00:00 fmtlog] <font color="cyan"><b>debug</b></font>: Example Message
//! [2021/01/01 12:00:00 fmtlog] <font color="blue"><b>trace</b></font>: Example Message
//! </pre>
//!
//! ## DETAIL2
//! A detailed version of [`SIMPLE2`](#simple2).
//!
//! <pre>
//! [<font color="red"><b>ERROR</b></font>] Example Message (at 2021/01/01 12:00:00 in fmtlog)
//! [<font color="yellow"><b>WARN</b></font>] Example Message (at 2021/01/01 12:00:00 in fmtlog)
//! [<font color="green"><b>INFO</b></font>] Example Message (at 2021/01/01 12:00:00 in fmtlog)
//! [<font color="cyan"><b>DEBUG</b></font>] Example Message (at 2021/01/01 12:00:00 in fmtlog)
//! [<font color="blue"><b>TRACE</b></font>] Example Message (at 2021/01/01 12:00:00 in fmtlog)
//! </pre>
//!
//! ## DETAIL2_LOWER
//! A detailed version of [`SIMPLE2_LOWER`](#simple2-lower).
//!
//! <pre>
//! [<font color="red"><b>error</b></font>] Example Message (at 2021/01/01 12:00:00 in fmtlog)
//! [<font color="yellow"><b>warn</b></font>] Example Message (at 2021/01/01 12:00:00 in fmtlog)
//! [<font color="green"><b>info</b></font>] Example Message (at 2021/01/01 12:00:00 in fmtlog)
//! [<font color="cyan"><b>debug</b></font>] Example Message (at 2021/01/01 12:00:00 in fmtlog)
//! [<font color="blue"><b>trace</b></font>] Example Message (at 2021/01/01 12:00:00 in fmtlog)
//! </pre>
//!
//! ## DEBUG1
//! A debug-specialized version of [`SIMPLE1`](#simple1).
//!
//! <pre>
//! [fmtlog (src/main.rs:10)] <font color="red"><b>ERROR</b></font>: Example Message
//! [fmtlog (src/main.rs:20)] <font color="yellow"><b>WARN</b></font>: Example Message
//! [fmtlog (src/main.rs:30)] <font color="green"><b>INFO</b></font>: Example Message
//! [fmtlog (src/main.rs:40)] <font color="cyan"><b>DEBUG</b></font>: Example Message
//! [fmtlog (src/main/rs:50)] <font color="blue"><b>TRACE</b></font>: Example Message
//! </pre>
//!
//! ## DEBUG1_LOWER
//! A debug-specialized version of [`SIMPLE1_LOWER`](#simple1-lower).
//!
//! <pre>
//! [fmtlog (src/main.rs:10)] <font color="red"><b>error</b></font>: Example Message
//! [fmtlog (src/main.rs:20)] <font color="yellow"><b>warn</b></font>: Example Message
//! [fmtlog (src/main.rs:30)] <font color="green"><b>info</b></font>: Example Message
//! [fmtlog (src/main.rs:40)] <font color="cyan"><b>debug</b></font>: Example Message
//! [fmtlog (src/main/rs:50)] <font color="blue"><b>trace</b></font>: Example Message
//! </pre>
//!
//! ## DEBUG2
//! A debug-specialized version of [`SIMPLE2`](#simple2).
//!
//! <pre>
//! [<font color="red"><b>ERROR</b></font>] Example Message (at src/main.rs:10 in fmtlog)
//! [<font color="yellow"><b>WARN</b></font>] Example Message (at src/main.rs:20 in fmtlog)
//! [<font color="green"><b>INFO</b></font>] Example Message (at src/main.rs:30 in fmtlog)
//! [<font color="cyan"><b>DEBUG</b></font>] Example Message (at src/main.rs:40 in fmtlog)
//! [<font color="blue"><b>TRACE</b></font>] Example Message (at src/main.rs:50 in fmtlog)
//! </pre>
//!
//! ## DEBUG2_LOWER
//! A debug-specialized version of [`SIMPLE2_LOWER`](#simple2-lower).
//!
//! <pre>
//! [<font color="red"><b>error</b></font>] Example Message (at src/main.rs:10 in fmtlog)
//! [<font color="yellow"><b>warn</b></font>] Example Message (at src/main.rs:20 in fmtlog)
//! [<font color="green"><b>info</b></font>] Example Message (at src/main.rs:30 in fmtlog)
//! [<font color="cyan"><b>debug</b></font>] Example Message (at src/main.rs:40 in fmtlog)
//! [<font color="blue"><b>trace</b></font>] Example Message (at src/main.rs:50 in fmtlog)
//! </pre>
//!

pub const SIMPLE1: &str = "%F(red,yellow,green,cyan,blue){%b{%L}}: %M\n";
pub const SIMPLE1_LOWER: &str = "%F(red,yellow,green,cyan,blue){%b{%l}}: %M\n";

pub const SIMPLE2: &str = "[%F(red,yellow,green,cyan,blue){%b{%L}}] %M\n";
pub const SIMPLE2_LOWER: &str = "[%F(red,yellow,green,cyan,blue){%b{%l}}] %M\n";


pub const DETAIL1: &str =
    "[%T(%Y/%m/%d %T) %N] %F(red,yellow,green,cyan,blue){%b{%L}}: %M\n";
pub const DETAIL1_LOWER: &str =
    "[%T(%Y/%m/%d %T) %N] %F(red,yellow,green,cyan,blue){%b{%l}}: %M\n";

pub const DETAIL2: &str =
    "[%F(red,yellow,green,cyan,blue){%b{%L}}] %M (at %T(%Y/%m/%d %T) in %N))\n";
pub const DETAIL2_LOWER: &str =
    "[%F(red,yellow,green,cyan,blue){%b{%l}}] %M (at %T(%Y/%m/%d %T) in %N))\n";


pub const DEBUG1: &str =
    "[%N (%S)] %F(red,yellow,green,cyan,blue){%b{%L}}: %M\n";
pub const DEBUG1_LOWER: &str =
    "[%N (%S)] %F(red,yellow,green,cyan,blue){%b{%l}}: %M\n";

pub const DEBUG2: &str =
    "[%F(red,yellow,green,cyan,blue){%b{%L}}] %M (at %S in %N))\n";
pub const DEBUG2_LOWER: &str =
    "[%F(red,yellow,green,cyan,blue){%b{%l}}] %M (at %S in %N))\n";
