# cst328
[![CI](https://github.com/cmumford/cst328/actions/workflows/ci.yml/badge.svg)](https://github.com/cmumford/cst328/actions/workflows/ci.yml)
[![Crates.io Version](https://img.shields.io/crates/v/cst328)](https://crates.io/crates/cst328)
[![docs.rs](https://docs.rs/cst328/badge.svg)](https://docs.rs/cst328)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A pure Rust driver for the CST328 touch controller device
([datasheet](docs/CST328-DataSheet-V2.2.pdf)). This driver supports the
following:

* Various hardware via [`embedded-hal`](https://github.com/rust-embedded/embedded-hal)
* A `no_std` environment
* both sync and async API's

This library supports either (but **not both**) synchronous (blocking) or
asynchronous (non-blocking) API's. One and only one of the following
features must be defined: `use_sync` or `use_async`.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
cst328 = { version = "1.0", features = ["use_sync"] }   # or "use_async"
```

## Example

```rs
use cst328::{Cst328, Mode};
use log::{info, error};

let i2c = // Create I2C device.
let mut dev = Cst328::new(i2c);
match dev.read_finger(0).await {
    Ok(finger) => info!("Touch Data: {finger:?}"),
    Err(e) => error!("Failed to read finger data: {e:?}"),
}
```

See the full working examples in the project `///examples` directory.