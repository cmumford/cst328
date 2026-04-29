#![no_std]

mod cst328;

pub use cst328::{Error, TouchData, TouchPoint};

#[cfg(feature = "use_sync")]
pub use crate::cst328::Cst328Sync;

#[cfg(feature = "use_async")]
pub use crate::cst328::Cst328Async;

#[cfg(all(feature = "use_sync", not(feature = "use_async")))]
pub use crate::cst328::Cst328Sync as Cst328;

#[cfg(all(feature = "use_async", not(feature = "use_sync")))]
pub use crate::cst328::Cst328Async as Cst328;
