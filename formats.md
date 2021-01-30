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

<pre>
<font color="red"><b>ERROR</b></font>: Example Message
<font color="yellow"><b>WARN</b></font>: Example Message
<font color="green"><b>INFO</b></font>: Example Message
<font color="cyan"><b>DEBUG</b></font>: Example Message
<font color="blue"><b>TRACE</b></font>: Example Message
</pre>

## SIMPLE1_LOWER
A simplest format. (lowercase version)

<pre>
<font color="red"><b>error</b></font>: Example Message
<font color="yellow"><b>warn</b></font>: Example Message
<font color="green"><b>info</b></font>: Example Message
<font color="cyan"><b>debug</b></font>: Example Message
<font color="blue"><b>trace</b></font>: Example Message
</pre>

## SIMPLE2
A simplest format. (bracket version)

<pre>
[<font color="red"><b>ERROR</b></font>] Example Message
[<font color="yellow"><b>WARN</b></font>] Example Message
[<font color="green"><b>INFO</b></font>] Example Message
[<font color="cyan"><b>DEBUG</b></font>] Example Message
[<font color="blue"><b>TRACE</b></font>] Example Message
</pre>

## SIMPLE2_LOWER
A simplest format. (bracket lowercase version)

<pre>
[<font color="red"><b>error</b></font>] Example Message
[<font color="yellow"><b>warn</b></font>] Example Message
[<font color="green"><b>info</b></font>] Example Message
[<font color="cyan"><b>debug</b></font>] Example Message
[<font color="blue"><b>trace</b></font>] Example Message
</pre>

## DETAIL1 (Requires feature: `chrono`)
A detailed version of [`SIMPLE1`](#simple1).

<pre>
[2021/01/01 12:00:00 fmtlog] <font color="red"><b>ERROR</b></font>: Example Message
[2021/01/01 12:00:00 fmtlog] <font color="yellow"><b>WARN</b></font>: Example Message
[2021/01/01 12:00:00 fmtlog] <font color="green"><b>INFO</b></font>: Example Message
[2021/01/01 12:00:00 fmtlog] <font color="cyan"><b>DEBUG</b></font>: Example Message
[2021/01/01 12:00:00 fmtlog] <font color="blue"><b>TRACE</b></font>: Example Message
</pre>

## DETAIL1_LOWER (Requires feature: `chrono`)
A detailed version of [`SIMPLE1_LOWER`](#simple1-lower).

<pre>
[2021/01/01 12:00:00 fmtlog] <font color="red"><b>error</b></font>: Example Message
[2021/01/01 12:00:00 fmtlog] <font color="yellow"><b>warn</b></font>: Example Message
[2021/01/01 12:00:00 fmtlog] <font color="green"><b>info</b></font>: Example Message
[2021/01/01 12:00:00 fmtlog] <font color="cyan"><b>debug</b></font>: Example Message
[2021/01/01 12:00:00 fmtlog] <font color="blue"><b>trace</b></font>: Example Message
</pre>

## DETAIL2 (Requires feature: `chrono`)
A detailed version of [`SIMPLE2`](#simple2).

<pre>
[<font color="red"><b>ERROR</b></font>] Example Message (at Jan 01 12:00:00 in fmtlog)
[<font color="yellow"><b>WARN</b></font>] Example Message (at Jan 01 12:00:00 in fmtlog)
[<font color="green"><b>INFO</b></font>] Example Message (at  Jan 01 12:00:00 in fmtlog)
[<font color="cyan"><b>DEBUG</b></font>] Example Message (at Jan 01 12:00:00 in fmtlog)
[<font color="blue"><b>TRACE</b></font>] Example Message (at Jan 01 12:00:00 in fmtlog)
</pre>

## DETAIL2_LOWER (Requires feature: `chrono`)
A detailed version of [`SIMPLE2_LOWER`](#simple2-lower).

<pre>
[<font color="red"><b>error</b></font>] Example Message (at Jan 01 12:00:00 in fmtlog)
[<font color="yellow"><b>warn</b></font>] Example Message (at Jan 01 12:00:00 in fmtlog)
[<font color="green"><b>info</b></font>] Example Message (at Jan 01 12:00:00 in fmtlog)
[<font color="cyan"><b>debug</b></font>] Example Message (at Jan 01 12:00:00 in fmtlog)
[<font color="blue"><b>trace</b></font>] Example Message (at Jan 01 12:00:00 in fmtlog)
</pre>

## DEBUG1
A debug-specialized version of [`SIMPLE1`](#simple1).

<pre>
[fmtlog (src/main.rs:10)] <font color="red"><b>ERROR</b></font>: Example Message
[fmtlog (src/main.rs:20)] <font color="yellow"><b>WARN</b></font>: Example Message
[fmtlog (src/main.rs:30)] <font color="green"><b>INFO</b></font>: Example Message
[fmtlog (src/main.rs:40)] <font color="cyan"><b>DEBUG</b></font>: Example Message
[fmtlog (src/main/rs:50)] <font color="blue"><b>TRACE</b></font>: Example Message
</pre>

## DEBUG1_LOWER
A debug-specialized version of [`SIMPLE1_LOWER`](#simple1-lower).

<pre>
[fmtlog (src/main.rs:10)] <font color="red"><b>error</b></font>: Example Message
[fmtlog (src/main.rs:20)] <font color="yellow"><b>warn</b></font>: Example Message
[fmtlog (src/main.rs:30)] <font color="green"><b>info</b></font>: Example Message
[fmtlog (src/main.rs:40)] <font color="cyan"><b>debug</b></font>: Example Message
[fmtlog (src/main/rs:50)] <font color="blue"><b>trace</b></font>: Example Message
</pre>

## DEBUG2
A debug-specialized version of [`SIMPLE2`](#simple2).

<pre>
[<font color="red"><b>ERROR</b></font>] Example Message (at src/main.rs:10 in fmtlog)
[<font color="yellow"><b>WARN</b></font>] Example Message (at src/main.rs:20 in fmtlog)
[<font color="green"><b>INFO</b></font>] Example Message (at src/main.rs:30 in fmtlog)
[<font color="cyan"><b>DEBUG</b></font>] Example Message (at src/main.rs:40 in fmtlog)
[<font color="blue"><b>TRACE</b></font>] Example Message (at src/main.rs:50 in fmtlog)
</pre>

## DEBUG2_LOWER
A debug-specialized version of [`SIMPLE2_LOWER`](#simple2-lower).

<pre>
[<font color="red"><b>error</b></font>] Example Message (at src/main.rs:10 in fmtlog)
[<font color="yellow"><b>warn</b></font>] Example Message (at src/main.rs:20 in fmtlog)
[<font color="green"><b>info</b></font>] Example Message (at src/main.rs:30 in fmtlog)
[<font color="cyan"><b>debug</b></font>] Example Message (at src/main.rs:40 in fmtlog)
[<font color="blue"><b>trace</b></font>] Example Message (at src/main.rs:50 in fmtlog)
</pre>

## TOML (Requires feature: `chrono`)
Output logs as TOML format.

```toml
[2021-01-01T12:00:00.123456789+09:00]
target = "fmtlog"
level = "ERROR"
info = "Example Message"

[2021-01-01T12:00:00.123456789+09:00]
target = "fmtlog"
level = "WARN"
info = "Example Message"

[2021-01-01T12:00:00.123456789+09:00]
target = "fmtlog"
level = "INFO"
info = "Example Message"

[2021-01-01T12:00:00.123456789+09:00]
target = "fmtlog"
level = "DEBUG"
info = "Example Message"

[2021-01-01T12:00:00.123456789+09:00]
target = "fmtlog"
level = "TRACE"
info = "Example Message"

```

## YAML (Requires feature: `chrono`)
Output logs as YAML format.

```yaml
- date: 2021-01-01T12:00:00.123456789+09:00
  target: fmtlog
  level: ERROR
  info: Example Message

- date: 2021-01-01T12:00:00.123456789+09:00
  target: fmtlog
  level: WARN
  info: Example Message

- date: 2021-01-01T12:00:00.123456789+09:00
  target: fmtlog
  level: INFO
  info: Example Message

- date: 2021-01-01T12:00:00.123456789+09:00
  target: fmtlog
  level: DEBUG
  info: Example Message

- date: 2021-01-01T12:00:00.123456789+09:00
  target: fmtlog
  level: TRACE
  info: Example Message

```

### ENV_LOGGER (Requires feature: `chrono`)
The default format of [`env_logger`](https://docs.rs/env_logger).

<pre>
<font color="black">[</font>2021-01-01T12:00:00Z <font color="red">ERROR</font> fmtlog<font color="black">]</font> Example Message
<font color="black">[</font>2021-01-01T12:00:00Z <font color="yellow">WARN</font> fmtlog<font color="black">]</font> Example Message
<font color="black">[</font>2021-01-01T12:00:00Z <font color="green">INFO</font> fmtlog<font color="black">]</font> Example Message
<font color="black">[</font>2021-01-01T12:00:00Z <font color="blue">DEBUG</font> fmtlog<font color="black">]</font> Example Message
<font color="black">[</font>2021-01-01T12:00:00Z <font color="cyan">TRACE</font> fmtlog<font color="black">]</font> Example Message
</pre>

### PRETTY_ENV_LOGGER
The default format of [`pretty_env_logger`](https://docs.rs/pretty_env_logger).

<pre>
 <font color="red">ERROR</font> <b>fmtlog</b> > Example Message
 <font color="yellow">WARN</font> <b>fmtlog</b> > Example Message
 <font color="green">INFO</font> <b>fmtlog</b> > Example Message
 <font color="blue">DEBUG</font> <b>fmtlog</b> > Example Message
 <font color="purple">TRACE</font> <b>fmtlog</b> > Example Message
</pre>

### FLEXI_LOGGER
The default format of [`flexi_logger`](https://docs.rs/flexi_logger) with timestamps.

<pre>
<font color="red"><b>ERROR</b></font> [fmtlog::module] <font color="red"><b>Example Message</b></font>
<font color="yellow"><b>WARN</b></font> [fmtlog::module] <font color="yellow"><b>Example Message</b></font>
<font color="white"><b>INFO</b></font> [fmtlog::module] <font color="white"><b>Example Message</b></font>
<font color="white"><b>DEBUG</b></font> [fmtlog::module] <font color="white"><b>Example Message</b></font>
<font color="black"><b>TRACE</b></font> [fmtlog::module] <font color="white"><b>Example Message</b></font>
</pre>

### FLEXI_LOGGER2 (Requires feature: `chrono`)
The default(detailed) format of [`flexi_logger`](https://docs.rs/flexi_logger).

<pre>
<font color="red"><b>2021-01-01 12:00:00:00.000000 +09:00 ERROR</b></font> [fmtlog::module] src/main.rs:10: <font color="red"><b>Example Message</b></font>
<font color="yellow"><b>2021-01-01 12:00:00:00.000000 +09:00 WARN</b></font> [fmtlog::module] src/main.rs:20: <font color="yellow"><b>Example Message</b></font>
<font color="white"><b>2021-01-01 12:00:00:00.000000 +09:00 INFO</b></font> [fmtlog::module] src/main.rs:30: <font color="white"><b>Example Message</b></font>
<font color="white"><b>2021-01-01 12:00:00:00.000000 +09:00 DEBUG</b></font> [fmtlog::module] src/main.rs:40: <font color="white"><b>Example Message</b></font>
<font color="black"><b>2021-01-01 12:00:00:00.000000 +09:00 TRACE</b></font> [fmtlog::module] src/main.rs:50: <font color="white"><b>Example Message</b></font>
</pre>

### SIMPLE_LOGGER (Requires feature: `chrono`)
The default format of [`simple_logger`](https://docs.rs/simple_logger).

<pre>
2021-01-01 12:00:00 <font color="red">ERROR</font> [fmtlog] Example Message
2021-01-01 12:00:00 <font color="yellow">WARN</font> [fmtlog] Example Message
2021-01-01 12:00:00 <font color ="cyan">INFO</font> [fmtlog] Example Message
2021-01-01 12:00:00 <font color="purple">DEBUG</font> [fmtlog] Example Message
2021-01-01 12:00:00 TRACE [fmtlog] Example Message
</pre>

### SIMPLELOG (Requires feature: `chrono`)
The default format of [`simplelog`](https://docs.rs/simplelog).

```text
12:00:00 [ERROR] Example Message
12:00:00 [WARN] Example Message
12:00:00 [INFO] Example Message
12:00:00 [DEBUG] Example Message
12:00:00 [TRACE] Example Message
```

### STDERRLOG
The default format of [`stderrlog`](https://docs.rs/stderrlog).

<pre>
<font color="red">ERROR - Example Message</font>
<font color="purple">WARN - Example Message</font>
<font color="yellow">INFO - Example Message</font>
<font color="cyan">DEBUG - Example Message</font>
<font color="blue">TRACE - Example Message</font>
</pre>

### STDERRLOG2 (Requires feature: `chrono`)
The default format of [`stderrlog`](https://docs.rs/stderrlog) with timestamps.

<pre>
<font color="red">2021-01-01T12:00:00+09:00 - ERROR - Example Message</font>
<font color="purple">2021-01-01T12:00:00+09:00 - WARN - Example Message</font>
<font color="yellow">2021-01-01T12:00:00+09:00 - INFO - Example Message</font>
<font color="cyan">2021-01-01T12:00:00+09:00 - DEBUG - Example Message</font>
<font color="blue">2021-01-01T12:00:00+09:00 - TRACE - Example Message</font>
</pre>

