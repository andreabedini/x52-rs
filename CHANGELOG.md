# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-01-30

Initial release. Pure Rust rewrite of [libx52](https://github.com/nirenjan/libx52).

### x52-device

- Device variant detection (X52 vs X52 Pro) from USB product ID
- Shared constants: vendor/product IDs, device capabilities

### x52_hid

- Async HID input reading via `async-hid`
- HID report parsing for X52 (14 bytes, 11-bit X/Y) and X52 Pro (15 bytes, 10-bit X/Y)
- 11 axes (X, Y, RZ, Z, RX, RY, Slider, ThumbX/Y, HatX/Y) with device-specific ranges
- 40 buttons with device-specific bit mappings
- POV hat position tracking, mode selector tracking
- Timeout support

### x52-usb

- Async USB vendor control transfers via `nusb`
- MFD text display (3 lines x 16 characters)
- LED control: 11 LEDs with color support (red/amber/green for X52 Pro multicolor LEDs)
- Clock time, timezone offsets for secondary/tertiary clocks, date display
- Independent MFD and LED brightness control (0-128)
- Shift indicator, blink mode
- Batch update builder for efficient multi-command sequences
- Retry logic (3 attempts, 5s timeout per command)

### Examples

- `basic_input` - continuous joystick input reading
- `led_control` - cycling through LED colors
- `mfd_text` - MFD text display
- `batch_update` - batch updates with clock/date
