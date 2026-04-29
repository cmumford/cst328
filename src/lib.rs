#![no_std]

mod cst328;

pub use cst328::{Error, TouchData, TouchPoint};

#[cfg(not(any(feature = "use_sync", feature = "use_async")))]
compile_error!(
    r#"You must enable exactly one of the features `use_sync` or `use_async`.

Most users want the blocking version:
    cst328 = { version = "0.x", features = ["use_sync"] }

For async (Embassy):
    cst328 = { version = "0.x", features = ["use_async"] }"#
);

#[cfg(all(feature = "use_sync", feature = "use_async"))]
compile_error!(
    r#"Features `use_sync` and `use_async` cannot be enabled at the same time.

Please choose only one:
- `use_sync`   (recommended for most applications, blocking I2C)
- `use_async`  (for use with Embassy / async runtimes)"#
);

#[cfg(feature = "use_sync")]
pub use crate::cst328::Cst328Sync;

#[cfg(feature = "use_async")]
pub use crate::cst328::Cst328Async;

#[cfg(all(feature = "use_sync", not(feature = "use_async")))]
pub use crate::cst328::Cst328Sync as Cst328;

#[cfg(all(feature = "use_async", not(feature = "use_sync")))]
pub use crate::cst328::Cst328Async as Cst328;
