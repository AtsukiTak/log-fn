logfn
===

This crate provides the `logfn` attribute macro for inserting logging code into your function.

Currently we support 2 types of logging.

- Pre logging
- Post logging

And we have a plan to add `Time logging` and `Count logging` type.

Each `logfn` attribute injects a single logging code. You can put as many `logfn` as you want.

```rust
# use logfn::logfn;
# use std::num::ParseIntError;
#[logfn(Pre, Debug, "{fn} will be executed")]
#[logfn(Post, Debug, "{fn} is executed", if = "Result::is_ok")]
#[logfn(Post, Error, "Error {fn}: {ret:?}", if = "Result::is_err")]
fn atoi(a: &str) -> Result<usize, ParseIntError> {
    usize::from_str_radix(a, 10)
}
```

The detail is documented below.

## Pre logging

The following attribute injects logging code **before** function is called.

```rust
use logfn::logfn;

#[logfn(Pre, Info, "executing {fn}...")]
fn add(a: usize, b: usize) -> usize {
    a + b
}
```

The resulting code will looks like

```rust
fn add(a: usize, b: usize) -> usize {
    log::info!("executing add...");

    {
        a + b
    }
}
```

## Post logging

You also be able to inject logging code **after** function is called.

```rust
use logfn::logfn;

#[logfn(Post, Info, "executed {fn}!")]
fn add(a: usize, b: usize) -> usize {
    a + b
}
```

The resulting code will looks like

```rust
fn add(a: usize, b: usize) -> usize {
    let res = (move || {
        a + b
    })();

    log::info!("executed add!");

    res
}
```

## Conditional logging

You can configure the condition on which logging code is fired.
To do that, please add `if` argument with a path to the function which takes reference to
returned value and returns `true` when you want to fire the logging code.

## Note
Conditional logging is only supported in post logging for now.

```rust
use logfn::logfn;

#[logfn(Post, Warn, "checked add is failed!!", if = "Option::is_none")]
fn checked_add(a: usize, b: usize) -> Option<usize> {
    a.checked_add(b)
}
```

## Message formatting

We support below format patterns.

- "{fn}" interpolates function name
- "{ret:?}" or "{ret}" interpolates returned value

```rust
# use std::num::ParseIntError;
use logfn::logfn;

#[logfn(Post, Error, "Error while {fn}: {ret:?}", if = "Result::is_err")]
fn atoi(s: &str) -> Result<usize, ParseIntError> {
    usize::from_str_radix(s, 10)
}
```

## Async function

Async function is also supported.

```rust
use logfn::logfn;

#[logfn(Post, Debug, "{fn} is executed")]
async fn add_fut(a: usize, b: usize) -> usize {
    a + b
}
```
