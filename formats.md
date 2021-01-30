# Formats Collection
Here is the list of preset formats of fmtlog.

**WARNING: Log will not be colored when feature `colored` is missing.**

These formats can be used like this:
```rust
use fmtlog::Config;
use fmtlog::formats::SIMPLE1; // A format to use.

fmtlog::new(Config::new().format(SIMPLE1)).set().unwrap();

log::error!("Example Message");
log::warn!("Example Message");
log::info!("Example Message");
log::debug!("Example Message");
log::trace!("Example Message");
```

## SIMPLE1
A simplest format.

![simple1](images/simple1.png)

## SIMPLE1_LOWER
A simplest format. (lowercase version)

![simple1-lower](images/simple1-lower.png)

## SIMPLE2
A simplest format. (bracket version)

![simple2](images/simple2.png)

## SIMPLE2_LOWER
A simplest format. (bracket lowercase version)

![simple2-lower](images/simple2-lower.png)

## DETAIL1 (Requires feature: `chrono`)
A detailed version of [`SIMPLE1`](#simple1).

![detail1](images/detail1.png)

## DETAIL1_LOWER (Requires feature: `chrono`)
A detailed version of [`SIMPLE1_LOWER`](#simple1-lower).

![detail1-lower](images/detail1-lower.png)

## DETAIL2 (Requires feature: `chrono`)
A detailed version of [`SIMPLE2`](#simple2).

![detail2](images/detail2.png)

## DETAIL2_LOWER (Requires feature: `chrono`)
A detailed version of [`SIMPLE2_LOWER`](#simple2-lower).

![detail2-lower](images/detail2-lower.png)

## DEBUG1
A debug-specialized version of [`SIMPLE1`](#simple1).

![debug1](images/debug1.png)

## DEBUG1_LOWER
A debug-specialized version of [`SIMPLE1_LOWER`](#simple1-lower).

![debug1-lower](images/debug1-lower.png)

## DEBUG2
A debug-specialized version of [`SIMPLE2`](#simple2).

![debug2](images/debug2.png)

## DEBUG2_LOWER
A debug-specialized version of [`SIMPLE2_LOWER`](#simple2-lower).

![debug2-lower](images/debug2-lower.png)

## TOML (Requires feature: `chrono`)
Output logs as TOML format.

![toml](images/toml.png)

## YAML (Requires feature: `chrono`)
Output logs as YAML format.

![yaml](images/yaml.png)

### ENV_LOGGER (Requires feature: `chrono`)
The default format of [`env_logger`](https://docs.rs/env_logger).

![env-logger](images/env-logger.png)

### PRETTY_ENV_LOGGER
The default format of [`pretty_env_logger`](https://docs.rs/pretty_env_logger).

![pretty-env-logger](images/pretty-env-logger.png)

### FLEXI_LOGGER
The default format of [`flexi_logger`](https://docs.rs/flexi_logger) with timestamps.

![flexi-logger](images/flexi-logger.png)

### FLEXI_LOGGER2 (Requires feature: `chrono`)
The default(detailed) format of [`flexi_logger`](https://docs.rs/flexi_logger).

![flexi-logger2](images/flexi-logger.png)

### SIMPLE_LOGGER (Requires feature: `chrono`)
The default format of [`simple_logger`](https://docs.rs/simple_logger).

![simple-logger](images/simple-logger.png)

### SIMPLELOG (Requires feature: `chrono`)
The default format of [`simplelog`](https://docs.rs/simplelog).

![simplelog](images/simplelog.png)

### STDERRLOG
The default format of [`stderrlog`](https://docs.rs/stderrlog).

![stderrlog](images/stderrlog.png)

### STDERRLOG2 (Requires feature: `chrono`)
The default format of [`stderrlog`](https://docs.rs/stderrlog) with timestamps.

![stderrlog2](images/stderrlog2.png)
