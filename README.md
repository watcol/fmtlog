# fmtlog
[![crates.io](https://img.shields.io/crates/v/fmtlog)](https://crates.io/crates/fmtlog)
[![docs.rs](https://docs.rs/fmtlog/badge.svg)](https://docs.rs/fmtlog)
[![Downloads](https://img.shields.io/crates/d/fmtlog)](https://crates.io/crates/fmtlog)
[![Downloads (latest)](https://img.shields.io/crates/dv/fmtlog)](https://crates.io/crates/fmtlog)
[![License](https://img.shields.io/crates/l/fmtlog)](https://github.com/watcol/fmtlog/blob/main/LICENSE)

A simple configurable logger with format specification.

![detail1](https://github.com/watcol/fmtlog/blob/main/images/detail1.png)

For more formats, see [the Formats Collection](formats.md).

## Usage
Add to your `Cargo.toml`:
```toml
[dependencies]
log = "0.4"
fmtlog = "0.1.3"
```

These features are included by the default,
but you can remove these features.

| Feature | Description |
|---------|-------------
| `chrono` | Enable timestamps. |
| `colored` | Coloring the log. |

Like this:
```toml
[dependencies.fmtlog]
version = "0.1.3"
default-features = false
features = ["chrono"]  
```

and initialize the logger in your code:
```rust
#[macro_use]
extern crate log;
extern crate fmtlog;

fn main() {
    fmtlog::default()
        .set()
        .unwrap();

    error!("Something has failed.");

    // ...
}
```

For advanced usage, read [the API document](https://docs.rs/fmtlog).

## Features
- [x] Format Specification
- [x] Module-level Logging
- [x] Timestamps Support
- [x] Colorized Log
- [x] Logging to the File
- [x] Multiple log target

## Documents
API Documents are available on [docs.rs](https://docs.rs/fmtlog).

## Author
- ![watcol](https://github.com/watcol/icons/blob/main/32/normal.png) Watcol <<potfman@gmail.com>>

## License
This program is licensed under the MIT license.

See [LICENSE](https://github.com/watcol/fmtlog/blob/main/LICENSE) for details.
