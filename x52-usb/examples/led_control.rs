//! LED control example
//!
//! This example demonstrates controlling the LEDs on the X52 Pro.

use smol::Timer;
use std::time::Duration;
use x52_usb::{self, Led, LedColor, X52Device};

fn main() -> anyhow::Result<()> {
    smol::block_on(async {
        println!("Opening X52 device for control...");
        let device = X52Device::open().await?;

        println!("Device: {}", device.variant().name());

        if !device.supports_led_control() {
            println!("Warning: This device does not support full LED control");
            println!("Only Fire and Throttle LEDs are available");
        }

        println!("\nCycling through LED colors...\n");

        // Cycle through colors on various LEDs
        let leds = [Led::A, Led::B, Led::T1, Led::T2, Led::T3];
        let colors = [
            LedColor::Red,
            LedColor::Amber,
            LedColor::Green,
            LedColor::Off,
        ];

        let mut fire_color = LedColor::On;

        for color in &colors {
            println!("Setting LEDs to {color}");

            device.set_led_color(Led::Fire, fire_color).await?;
            fire_color = match fire_color {
                LedColor::On => LedColor::Off,
                LedColor::Off => LedColor::On,
                _ => LedColor::On,
            };

            for led in &leds {
                if let Err(e) = device.set_led_color(*led, *color).await {
                    eprintln!("Failed to set {led} LED: {e}");
                }
            }

            Timer::after(Duration::from_secs(1)).await;
        }

        println!("\nDone!");
        Ok(())
    })
}
