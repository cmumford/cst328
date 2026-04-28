#![no_std]

mod cst328;

pub use cst328::{Error, TouchData, TouchPoint};

// Conditional re-export so `Cst328` always exists for users
#[cfg(feature = "use_sync")]
pub use cst328::Cst328Sync as Cst328;

#[cfg(feature = "use_async")]
pub use cst328::Cst328Async as Cst328;
