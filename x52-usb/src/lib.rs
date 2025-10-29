//! Async USB control library for Saitek X52/X52 Pro HOTAS
//!
//! This library provides asynchronous control of the MFD, LEDs, and clocks.

#![warn(missing_docs)]

mod clock;
mod device;
mod error;
mod led;
mod mfd;
mod misc;
mod protocol;

pub use device::{BatchUpdate, X52Device};

pub use clock::{ClockFormat, ClockOffsetId, DateFormat, set_date_day_month, set_date_year};
pub use led::{Led, LedColor, set_brightness as set_led_brightness, set_color as set_led_color};
pub use mfd::{MfdLine, clear_line, set_brightness as set_mfd_brightness, write_chars};
