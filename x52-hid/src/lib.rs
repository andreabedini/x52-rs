//! Async HID input library for Saitek X52/X52 Pro HOTAS
//!
//! This library provides asynchronous input reading from X52/X52 Pro flight controllers.

#![warn(missing_docs)]

mod device;
mod error;
mod input;
mod parser;
mod report;

pub use device::*;
pub use error::*;
pub use input::*;
pub use report::*;
