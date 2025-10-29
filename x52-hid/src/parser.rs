//! HID report parsing for X52 and X52 Pro

use anyhow::Result;
use log::{debug, trace, warn};

use crate::error::Error;
use crate::input::{Axis, Button, HatPosition};
use crate::report::InputReport;

/// Detect if the HID report might have a report ID byte prepended
/// Returns the offset (0 or 1) to use for data indexing
fn detect_report_id_offset(data: &[u8], expected_len: usize) -> usize {
    // If data is 1 byte longer than expected, it might have a report ID at byte 0
    if data.len() == expected_len + 1 {
        // Common report ID values are 0x00, 0x01, or small numbers
        // If byte 0 looks like a report ID, skip it
        if data[0] <= 0x0F {
            debug!(
                "Detected possible report ID byte: 0x{:02x}, offsetting data by 1",
                data[0]
            );
            return 1;
        }
    }
    0
}

/// Button mapping for X52 (14-byte report)
const X52_BUTTON_MAP: [Button; 38] = [
    Button::Trigger,        // Bit 0
    Button::TriggerStage2,  // Bit 1
    Button::Fire,           // Bit 2
    Button::A,              // Bit 3
    Button::B,              // Bit 4
    Button::C,              // Bit 5
    Button::Pinkie,         // Bit 6
    Button::D,              // Bit 7
    Button::E,              // Bit 8
    Button::T1,             // Bit 9
    Button::T2,             // Bit 10
    Button::T3,             // Bit 11
    Button::T4,             // Bit 12
    Button::T5,             // Bit 13
    Button::T6,             // Bit 14
    Button::PovUp,          // Bit 15
    Button::PovRight,       // Bit 16
    Button::PovDown,        // Bit 17
    Button::PovLeft,        // Bit 18
    Button::Pov2Up,         // Bit 19
    Button::Pov2Right,      // Bit 20
    Button::Pov2Down,       // Bit 21
    Button::Pov2Left,       // Bit 22
    Button::Clutch,         // Bit 23
    Button::MousePrimary,   // Bit 24
    Button::MouseSecondary, // Bit 25
    Button::MouseWheel,     // Bit 26
    Button::WheelUp,        // Bit 27
    Button::WheelDown,      // Bit 28
    Button::Mode1,          // Bit 29
    Button::Mode2,          // Bit 30
    Button::Mode3,          // Bit 31
    Button::Function,       // Bit 32
    Button::StartStop,      // Bit 33
    Button::Reset,          // Bit 34
    Button::PageUp,         // Bit 35 (always 0 on X52)
    Button::PageDown,       // Bit 36 (always 0 on X52)
    Button::Up,             // Bit 37 (always 0 on X52)
];

/// Button mapping for X52 Pro (15-byte report)
/// Different bit ordering than X52
const X52_PRO_BUTTON_MAP: [Button; 40] = [
    Button::Trigger,        // Bit 0
    Button::Fire,           // Bit 1
    Button::A,              // Bit 2
    Button::B,              // Bit 3
    Button::C,              // Bit 4
    Button::Pinkie,         // Bit 5
    Button::D,              // Bit 6
    Button::E,              // Bit 7
    Button::T1,             // Bit 8
    Button::T2,             // Bit 9
    Button::T3,             // Bit 10
    Button::T4,             // Bit 11
    Button::T5,             // Bit 12
    Button::T6,             // Bit 13
    Button::TriggerStage2,  // Bit 14
    Button::PovUp,          // Bit 15
    Button::PovRight,       // Bit 16
    Button::PovDown,        // Bit 17
    Button::PovLeft,        // Bit 18
    Button::Pov2Up,         // Bit 19
    Button::Pov2Right,      // Bit 20
    Button::Pov2Down,       // Bit 21
    Button::Pov2Left,       // Bit 22
    Button::Clutch,         // Bit 23
    Button::Function,       // Bit 24
    Button::StartStop,      // Bit 25
    Button::Reset,          // Bit 26
    Button::MousePrimary,   // Bit 27
    Button::WheelUp,        // Bit 28
    Button::WheelDown,      // Bit 29
    Button::MouseSecondary, // Bit 30
    Button::MouseWheel,     // Bit 31
    Button::Mode1,          // Bit 32
    Button::Mode2,          // Bit 33
    Button::Mode3,          // Bit 34
    Button::PageUp,         // Bit 35
    Button::PageDown,       // Bit 36
    Button::Up,             // Bit 37
    Button::Down,           // Bit 38
    Button::Select,         // Bit 39
];

/// Parse X52 HID report (14 bytes)
pub(crate) fn parse_x52(data: &[u8], report: &mut InputReport) -> Result<()> {
    // Check for report ID byte and adjust offset
    let offset = detect_report_id_offset(data, 14);
    let data = &data[offset..];

    if data.len() < 14 {
        let hint = if data.len() == 13 {
            " (Try setting RUST_LOG=trace to see raw bytes and check for report ID offset issues)"
        } else {
            ""
        };
        return Err(Error::InvalidParameter(format!(
            "X52 report too short: {} bytes (expected 14){}",
            data.len(),
            hint
        ))
        .into());
    }

    // Debug logging: show raw HID report bytes
    trace!(
        "X52 raw HID report ({} bytes, offset={}): {:02x?}",
        data.len(),
        offset,
        &data[..data.len().min(16)]
    );

    // Parse packed axes from bytes 0-3 (32 bits)
    // X: 11 bits, Y: 11 bits, RZ: 10 bits
    let packed = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);

    trace!(
        "X52 packed axes: 0x{:08x} (bytes: {:02x} {:02x} {:02x} {:02x})",
        packed, data[0], data[1], data[2], data[3]
    );

    // Validate that we're getting sensible data
    if packed == 0 && data[4] != 0 {
        warn!(
            "Packed axes are all zero but Z axis (throttle) is not - possible byte offset issue!"
        );
        warn!("Raw data: {:02x?}", &data[..14.min(data.len())]);
    }

    let x = (packed & 0x7FF) as i32; // Bits 0-10 (11 bits)
    let y = ((packed >> 11) & 0x7FF) as i32; // Bits 11-21 (11 bits)
    let z = data[4] as i32; // Byte 4 (8 bits)

    report.axes.set(Axis::X, x);
    report.axes.set(Axis::Y, y);
    report.axes.set(Axis::Z, z);

    let rx = data[5] as i32; // Byte 5 (8 bits)
    let ry = data[6] as i32; // Byte 6 (8 bits
    let rz = ((packed >> 22) & 0x3FF) as i32; // Bits 22-31 (10 bits)

    report.axes.set(Axis::RZ, rz);
    report.axes.set(Axis::RX, rx);
    report.axes.set(Axis::RY, ry);

    let slider = data[7] as i32; // Byte 7 (8 bits)
    report.axes.set(Axis::Slider, slider);

    // Hat position (upper nibble of byte 12 for X52)
    // Note: This differs from X52 Pro which uses byte 13
    let hat_value = (data[12] >> 4) & 0x0F;
    report.hat = HatPosition::from_raw(hat_value);
    let (hat_x, hat_y) = report.hat.to_axis_values();
    report.axes.set(Axis::HatX, hat_x);
    report.axes.set(Axis::HatY, hat_y);

    trace!(
        "X52 axes: X={}, Y={}, Z={}, RX={}, RY={}, RZ={}, Slider={}, Hat={:?}",
        x, y, z, rx, ry, rz, slider, report.hat
    );

    // Thumbstick for X52: byte 13
    // Lower nibble = X axis, upper nibble = Y axis
    // (Hat is in byte 12, so no conflict)
    let thumb_x = (data[13] & 0x0F) as i32;
    let thumb_y = ((data[13] >> 4) & 0x0F) as i32;
    report.axes.set(Axis::ThumbX, thumb_x);
    report.axes.set(Axis::ThumbY, thumb_y);

    trace!(
        "X52 thumbstick: X={}, Y={} (byte[13]=0x{:02x})",
        thumb_x, thumb_y, data[13]
    );

    // Parse buttons (bytes 8-12, 40 bits)
    let button_bits = u64::from_le_bytes([
        data[8], data[9], data[10], data[11], data[12], 0, 0, 0, // Pad to 64 bits
    ]);

    for (bit_idx, &button) in X52_BUTTON_MAP.iter().enumerate() {
        let pressed = (button_bits & (1 << bit_idx)) != 0;
        report.buttons.set(button, pressed);
    }

    // Detect current mode
    update_mode(report);

    Ok(())
}

/// Parse X52 Pro HID report (15 bytes)
pub(crate) fn parse_x52_pro(data: &[u8], report: &mut InputReport) -> Result<()> {
    // Check for report ID byte and adjust offset
    let offset = detect_report_id_offset(data, 15);
    let data = &data[offset..];

    if data.len() < 15 {
        let hint = if data.len() == 14 {
            " (Try setting RUST_LOG=trace to see raw bytes and check for report ID offset issues)"
        } else {
            ""
        };
        return Err(Error::InvalidParameter(format!(
            "X52 Pro report too short: {} bytes (expected 15){}",
            data.len(),
            hint
        ))
        .into());
    }

    // Debug logging: show raw HID report bytes
    trace!(
        "X52 Pro raw HID report ({} bytes, offset={}): {:02x?}",
        data.len(),
        offset,
        &data[..data.len().min(16)]
    );

    // Parse packed axes from bytes 0-3 (32 bits)
    // X: 10 bits, Y: 10 bits, RZ: 10 bits
    let packed = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);

    trace!(
        "X52 Pro packed axes: 0x{:08x} (bytes: {:02x} {:02x} {:02x} {:02x})",
        packed, data[0], data[1], data[2], data[3]
    );

    // Validate that we're getting sensible data
    if packed == 0 && data[4] != 0 {
        warn!(
            "Packed axes are all zero but Z axis (throttle) is not - possible byte offset issue!"
        );
        warn!("Raw data: {:02x?}", &data[..15.min(data.len())]);
    }

    let x = (packed & 0x3FF) as i32; // Bits 0-9 (10 bits)
    let y = ((packed >> 10) & 0x3FF) as i32; // Bits 10-19 (10 bits)
    let z = data[4] as i32; // Byte 4 (8 bits)

    report.axes.set(Axis::X, x);
    report.axes.set(Axis::Y, y);
    report.axes.set(Axis::Z, z);

    let rx = data[5] as i32; // Byte 5 (8 bits)
    let ry = data[6] as i32; // Byte 6 (8 bits)
    let rz = ((packed >> 20) & 0x3FF) as i32; // Bits 20-29 (10 bits)

    report.axes.set(Axis::RX, rx);
    report.axes.set(Axis::RY, ry);
    report.axes.set(Axis::RZ, rz);

    let slider = data[7] as i32; // Byte 7 (8 bits)
    report.axes.set(Axis::Slider, slider);

    // Hat position (upper nibble of byte 13 for X52 Pro)
    let hat_value = (data[13] >> 4) & 0x0F;
    report.hat = HatPosition::from_raw(hat_value);
    let (hat_x, hat_y) = report.hat.to_axis_values();
    report.axes.set(Axis::HatX, hat_x);
    report.axes.set(Axis::HatY, hat_y);

    // Thumbstick (byte 14: lower 4 bits = X, upper 4 bits = Y)
    let thumb_x = (data[14] & 0x0F) as i32;
    let thumb_y = ((data[14] >> 4) & 0x0F) as i32;
    report.axes.set(Axis::ThumbX, thumb_x);
    report.axes.set(Axis::ThumbY, thumb_y);

    trace!(
        "X52 Pro axes: X={}, Y={}, Z={}, RX={}, RY={}, RZ={}, Slider={}, Hat={:?}, Thumb: X={}, Y={}",
        x, y, z, rx, ry, rz, slider, report.hat, thumb_x, thumb_y
    );

    // Parse buttons (bytes 8-12, 40 bits)
    let button_bits = u64::from_le_bytes([
        data[8], data[9], data[10], data[11], data[12], 0, 0, 0, // Pad to 64 bits
    ]);

    for (bit_idx, &button) in X52_PRO_BUTTON_MAP.iter().enumerate() {
        let pressed = (button_bits & (1 << bit_idx)) != 0;
        report.buttons.set(button, pressed);
    }

    // Detect current mode
    update_mode(report);

    Ok(())
}

/// Update mode based on mode button states
/// If multiple mode buttons are pressed (transient state), mode is not updated
fn update_mode(report: &mut InputReport) {
    let pressed = [
        report.buttons.is_pressed(Button::Mode1),
        report.buttons.is_pressed(Button::Mode2),
        report.buttons.is_pressed(Button::Mode3),
    ];

    // Count pressed mode buttons
    let pressed_count = pressed.iter().filter(|&&b| b).count();
    let mode = pressed.iter().position(|&b| b).map(|i| i + 1);

    if let Some(mode) = mode
        && pressed_count == 1
    {
        report.mode = mode as u8;
    }
    // If 0 or >1 mode buttons pressed, keep current mode (transient state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_x52_length_check() {
        let mut report = InputReport::new();
        let short_data = [0u8; 10];
        assert!(parse_x52(&short_data, &mut report).is_err());
    }

    #[test]
    fn test_parse_x52_pro_length_check() {
        let mut report = InputReport::new();
        let short_data = [0u8; 10];
        assert!(parse_x52_pro(&short_data, &mut report).is_err());
    }

    #[test]
    fn test_parse_x52_axes() {
        let mut report = InputReport::new();
        // Simple test data with known values
        let mut data = [0u8; 14];

        // Set X=100, Y=200, RZ=300 in packed format
        // X: bits 0-10, Y: bits 11-21, RZ: bits 22-31
        let packed = 100u32 | (200u32 << 11) | (300u32 << 22);
        data[0..4].copy_from_slice(&packed.to_le_bytes());

        parse_x52(&data, &mut report).unwrap();

        assert_eq!(report.axes.get(Axis::X), 100);
        assert_eq!(report.axes.get(Axis::Y), 200);
        assert_eq!(report.axes.get(Axis::RZ), 300);
    }

    #[test]
    fn test_hat_position_conversion() {
        let pos = HatPosition::NorthEast;
        let (x, y) = pos.to_axis_values();
        assert_eq!(x, 1);
        assert_eq!(y, -1);
    }
}
