//! Formats Collection (See [the document](https://github.com/watcol/fmtlog/blob/main/formats.md).)
//!
//! This module provides a list of `const` strings
//! for easy format specification.
//!
//! For the details, see [the document](https://github.com/watcol/fmtlog/blob/main/formats.md).
//!

#[cfg(feature = "chrono")]
pub const TOML: &str = "[%T(%+)]\ntarget = \"%N\"\nlevel = \"%L\"\ninfo = \"%M\"\n\n";
#[cfg(feature = "chrono")]
pub const YAML: &str = "- date: %T(%+)\n  target: %N\n  level: %L\n  info: %M\n\n";

#[cfg(feature = "colored")]
pub use self::colored::*;
#[cfg(not(feature = "colored"))]
pub use self::normal::*;

#[allow(dead_code)]
mod normal {
    pub const SIMPLE1: &str = "%L: %M\n";
    pub const SIMPLE1_LOWER: &str = "%l: %M\n";

    pub const SIMPLE2: &str = "[%L] %M\n";
    pub const SIMPLE2_LOWER: &str = "[%l] %M\n";

    #[cfg(feature = "chrono")]
    pub const DETAIL1: &str = "[%T(%Y/%m/%d %T) %N] %L: %M\n";
    #[cfg(feature = "chrono")]
    pub const DETAIL1_LOWER: &str = "[%T(%Y/%m/%d %T) %N] %l: %M\n";

    #[cfg(feature = "chrono")]
    pub const DETAIL2: &str = "[%L] %M (at %T(%b %d %T) in %N)\n";
    #[cfg(feature = "chrono")]
    pub const DETAIL2_LOWER: &str = "[%l] %M (at %T(%b %d %T) in %N)\n";

    pub const DEBUG1: &str = "[%N (%S)] %L: %M\n";
    pub const DEBUG1_LOWER: &str = "[%N (%S)] %l: %M\n";

    pub const DEBUG2: &str = "[%L] %M (at %S in %N)\n";
    pub const DEBUG2_LOWER: &str = "[%l] %M (at %S in %N)\n";

    pub const PRETTY_ENV_LOGGER: &str = " %L %N > %M\n";
    pub const FLEXI_LOGGER: &str = "%L [%m] %M\n";
    pub const STDERRLOG: &str = "%L - %M\n";

    #[cfg(feature = "chrono")]
    pub const ENV_LOGGER: &str = "[%T(%Y-%m-%dT%TZ) %L %N] %M\n";
    #[cfg(feature = "chrono")]
    pub const FLEXI_LOGGER2: &str = "%T(%Y-%m-%d %T%.6f %:z) %L [%m] %S: %M\n";
    #[cfg(feature = "chrono")]
    pub const SIMPLE_LOGGER: &str = "%T(%Y-%m-%d %T) %L [%N] %M\n";
    #[cfg(feature = "chrono")]
    pub const SIMPLELOG: &str = "%T(%T) [%L] %M\n";
    #[cfg(feature = "chrono")]
    pub const STDERRLOG2: &str = "%T(%Y-%m-%dT%T%:z) - %L - %M\n";
}

#[allow(dead_code)]
mod colored {
    pub const SIMPLE1: &str = "%F(red,yellow,green,purple,blue){%b{%L}}: %M\n";
    pub const SIMPLE1_LOWER: &str = "%F(red,yellow,green,purple,blue){%b{%l}}: %M\n";

    pub const SIMPLE2: &str = "[%F(red,yellow,green,purple,blue){%b{%L}}] %M\n";
    pub const SIMPLE2_LOWER: &str = "[%F(red,yellow,green,purple,blue){%b{%l}}] %M\n";

    #[cfg(feature = "chrono")]
    pub const DETAIL1: &str = "[%T(%Y/%m/%d %T) %N] %F(red,yellow,green,purple,blue){%b{%L}}: %M\n";
    #[cfg(feature = "chrono")]
    pub const DETAIL1_LOWER: &str =
        "[%T(%Y/%m/%d %T) %N] %F(red,yellow,green,purple,blue){%b{%l}}: %M\n";

    #[cfg(feature = "chrono")]
    pub const DETAIL2: &str =
        "[%F(red,yellow,green,purple,blue){%b{%L}}] %M (at %T(%b %d %T) in %N)\n";
    #[cfg(feature = "chrono")]
    pub const DETAIL2_LOWER: &str =
        "[%F(red,yellow,green,purple,blue){%b{%l}}] %M (at %T(%b %d %T) in %N)\n";

    pub const DEBUG1: &str = "[%N (%S)] %F(red,yellow,green,purple,blue){%b{%L}}: %M\n";
    pub const DEBUG1_LOWER: &str = "[%N (%S)] %F(red,yellow,green,purple,blue){%b{%l}}: %M\n";

    pub const DEBUG2: &str = "[%F(red,yellow,green,purple,blue){%b{%L}}] %M (at %S in %N)\n";
    pub const DEBUG2_LOWER: &str = "[%F(red,yellow,green,purple,blue){%b{%l}}] %M (at %S in %N)\n";

    pub const PRETTY_ENV_LOGGER: &str = " %F(red,yellow,green,blue,purple){%L} %b{%N} > %M\n";
    pub const FLEXI_LOGGER: &str =
        "%F(red,yellow,white,white,black){%b{%L}} [%m] %F(red,yellow,white,white,black){%b{%M}}\n";
    pub const STDERRLOG: &str = "%F(red,purple,yellow,cyan,blue){%L - %M}\n";

    #[cfg(feature = "chrono")]
    pub const ENV_LOGGER: &str =
        "%F(bright black){[}%T(%Y-%m-%dT%TZ) %F(red,yellow,green,blue,cyan){%L} %N%F(bright black){]} %M\n";
    #[cfg(feature = "chrono")]
    pub const FLEXI_LOGGER2: &str = "%F(red,yellow,white,white,black){%b{%T(%Y-%m-%d %T%.6f %:z) %L}} [%m] %S: %F(red,yellow,white,white,black){%b{%M}}\n";
    #[cfg(feature = "chrono")]
    pub const SIMPLE_LOGGER: &str =
        "%T(%Y-%m-%d %T) %F(red,yellow,cyan,purple,white){%L} [%N] %M\n";
    #[cfg(feature = "chrono")]
    pub const SIMPLELOG: &str = "%T(%T) [%L] %M\n";
    #[cfg(feature = "chrono")]
    pub const STDERRLOG2: &str = "%F(red,purple,yellow,cyan,blue){%T(%Y-%m-%dT%T%:z) - %L - %M}\n";
}
