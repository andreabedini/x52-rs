//! Basic input reading example
//!
//! This example demonstrates reading joystick input from the X52/X52 Pro.

use x52_hid::X52Device;
use x52_hid::{Axis, Button};

fn main() -> anyhow::Result<()> {
    smol::block_on(async {
        println!("Opening X52 device for input...");
        let mut input = X52Device::open().await?;

        println!("Device: {}", input.variant().name());
        println!("Press Ctrl+C to exit\n");

        // Read input in a loop
        loop {
            // let mut buffer = vec![0u8; 64];
            // let read_bytes = input.handle.read_input_report(&mut buffer).await?;
            // println!("Read {} bytes from HID device: {:02X?}", read_bytes, buffer);
            let report = input.read().await?;

            // Print trigger state
            if report.buttons.is_pressed(Button::Trigger) {
                println!("TRIGGER PRESSED!");
            }

            // Print fire button state
            if report.buttons.is_pressed(Button::Fire) {
                println!("FIRE!");
            }

            // Print some axes
            println!(
                "X: {:4}, Y: {:4}, Z: {:4}, RX: {:4}, RY: {:4}, RZ: {:4}, Hat: {}",
                report.axes.get(Axis::X),
                report.axes.get(Axis::Y),
                report.axes.get(Axis::Z),
                report.axes.get(Axis::RX),
                report.axes.get(Axis::RY),
                report.axes.get(Axis::RZ),
                report.hat,
            );
        }
    })
}
