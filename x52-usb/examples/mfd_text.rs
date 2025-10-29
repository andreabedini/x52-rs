//! MFD text display example
//!
//! This example demonstrates displaying text on the MFD.

use x52_usb::X52Device;

fn main() -> anyhow::Result<()> {
    smol::block_on(async {
        println!("Opening X52 device for MFD control...");
        let device = X52Device::open().await?;

        println!("Device: {}", device.variant().name());
        println!("\nSetting MFD text...\n");

        // Set brightness
        device.set_brightness(128).await?;

        // Display text on all 3 lines
        device
            .set_text(x52_usb::MfdLine::Line0, "  libx52-rs     ")
            .await?;
        device
            .set_text(x52_usb::MfdLine::Line1, " Rust Rewrite  ")
            .await?;
        device
            .set_text(x52_usb::MfdLine::Line2, "  Hello X52!   ")
            .await?;

        println!("MFD text set successfully!");
        println!("Check your X52 MFD display");

        Ok(())
    })
}
