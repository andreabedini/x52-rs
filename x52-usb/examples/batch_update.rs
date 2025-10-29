//! Batch update example
//!
//! This example demonstrates using the batch update API for efficient
//! control of multiple features at once.

use chrono::{Datelike, Local, Timelike};
use x52_usb::{ClockFormat, Led, LedColor, MfdLine, X52Device};

fn main() -> anyhow::Result<()> {
    smol::block_on(async {
        println!("Opening X52 device...");
        let device = X52Device::open().await?;

        println!("Device: {}", device.variant().name());
        println!("\nApplying batch update...\n");

        let now = Local::now();

        // Build and apply a batch update
        device
            .batch()
            .text(MfdLine::Line0, "  Batch Update  ")
            .text(MfdLine::Line1, " Multiple Cmds ")
            .text(MfdLine::Line2, "    @ Once!    ")
            .mfd_brightness(100)
            .led_brightness(80)
            .led(Led::Fire, LedColor::On)
            .led(Led::A, LedColor::Green)
            .led(Led::B, LedColor::Amber)
            .clock(now.hour() as u8, now.minute() as u8, ClockFormat::Hours24)
            .date(now.day() as u8, now.month() as u8, (now.year() % 100) as u8)
            .apply()
            .await?;

        println!("Batch update completed successfully!");
        println!("Check your X52 device for the changes");

        Ok(())
    })
}
