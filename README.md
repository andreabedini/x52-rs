# x52

A Rust library for async control and input handling of the Saitek/MadCatz X52 and X52 Pro flight control system (HOTAS). Clean-room rewrite of [libx52](https://github.com/nirenjan/libx52) with no C dependencies.

## Architecture

This workspace contains three crates:

| Crate | Lib name | Purpose |
|-------|----------|---------|
| `x52-device` | `x52_device` | Shared types: device detection, variant identification |
| `x52-hid` | `x52_hid` | Async HID input (axes, buttons, hat) via `async-hid` |
| `x52-usb` | `x52_usb` | Async USB output (MFD, LEDs, clocks, brightness) via `nusb` |

The X52 uses different USB mechanisms for input vs output:

- **Input** (`x52-hid`): Standard HID input reports via `/dev/hidraw*`, read by `async-hid`
- **Output** (`x52-usb`): USB vendor control requests to endpoint 0, sent by `nusb`

This lets both coexist without conflicts -- the kernel HID driver stays attached for hidraw access while vendor control transfers bypass it entirely.

## Quick Start

### Reading Input

```rust
use x52_hid::{X52Device, Axis, Button};

fn main() -> anyhow::Result<()> {
    smol::block_on(async {
        let mut input = X52Device::open().await?;
        loop {
            let report = input.read().await?;
            let x = report.axes.get(Axis::X);
            let y = report.axes.get(Axis::Y);
            if report.buttons.is_pressed(Button::Trigger) {
                println!("FIRE! x={x} y={y}");
            }
        }
    })
}
```

### Controlling Output

```rust
use x52_usb::{X52Device, Led, LedColor, MfdLine};

fn main() -> anyhow::Result<()> {
    smol::block_on(async {
        let device = X52Device::open().await?;
        device.set_text(MfdLine::Line0, "Hello X52!").await?;
        device.set_led_color(Led::A, LedColor::Green).await?;
        device.set_brightness(100).await?;
        Ok(())
    })
}
```

### Batch Updates

```rust
use x52_usb::{X52Device, Led, LedColor, MfdLine, ClockFormat};

fn main() -> anyhow::Result<()> {
    smol::block_on(async {
        let device = X52Device::open().await?;
        device.batch()
            .text(MfdLine::Line0, "  Batch Mode   ")
            .text(MfdLine::Line1, " Many Commands ")
            .text(MfdLine::Line2, "    @ Once!    ")
            .mfd_brightness(100)
            .led_brightness(80)
            .led(Led::Fire, LedColor::On)
            .led(Led::A, LedColor::Green)
            .clock(12, 30, ClockFormat::Hours24)
            .apply()
            .await?;
        Ok(())
    })
}
```

## Examples

```bash
cargo run --example basic_input -p x52-hid      # HID input reading
cargo run --example led_control -p x52-usb       # LED control
cargo run --example mfd_text -p x52-usb          # MFD text display
cargo run --example batch_update -p x52-usb      # Batch updates
```

Set `RUST_LOG=debug` for verbose output.

## Capabilities

### Input (x52-hid)

- 11 axes: X, Y, RZ (stick), Z (throttle), RX/RY (rotaries), Slider, ThumbX/Y, HatX/Y
- 40 buttons: Trigger (2-stage), Fire, A-E, T1-T6, POV (2x 4-way), Mouse, Mode selectors, X52 Pro extras
- Device-specific HID report parsing: X52 (14 bytes, 11-bit X/Y) and X52 Pro (15 bytes, 10-bit X/Y)
- Timeout support, mode tracking

### Output (x52-usb)

- MFD text: 3 lines x 16 characters
- LEDs: Fire/Throttle (on/off), A/B/D/E/T1-T3/POV/Clutch (red/amber/green, X52 Pro only)
- Clock/date: 3 clocks with timezone offsets, date display
- Brightness: independent MFD and LED control (0-128)
- Shift indicator, blink mode
- Batch update builder for efficient multi-command sequences
- Retry logic (3 attempts, 5s timeout per command)

## Supported Devices

| Device | Vendor ID | Product ID | LED Control |
|--------|-----------|------------|-------------|
| X52 Pro | 0x06a3 | 0x0762 | Full (multicolor) |
| X52 (v1) | 0x06a3 | 0x0255 | Fire/Throttle only |
| X52 (v2) | 0x06a3 | 0x075C | Fire/Throttle only |

## Permissions

### Linux (udev)

Create `/etc/udev/rules.d/99-x52.rules`:
```
SUBSYSTEM=="usb", ATTRS{idVendor}=="06a3", ATTRS{idProduct}=="0762", MODE="0666"
SUBSYSTEM=="usb", ATTRS{idVendor}=="06a3", ATTRS{idProduct}=="0255", MODE="0666"
SUBSYSTEM=="usb", ATTRS{idVendor}=="06a3", ATTRS{idProduct}=="075c", MODE="0666"
```

Then reload: `sudo udevadm control --reload-rules && sudo udevadm trigger`

## Building

```bash
cargo check --all             # Type-check
cargo build --all             # Build
cargo test --all              # Test
cargo build --examples --all  # Build examples
cargo clippy --all            # Lint
cargo doc --no-deps --open    # Generate API docs
```

Requires Rust edition 2024 (resolver v3).

## License

MIT

## Credits

- Original [libx52](https://github.com/nirenjan/libx52) by Nirenjan Krishnan
- [nusb](https://github.com/kevinmehall/nusb) pure-Rust USB library
- [async-hid](https://github.com/sidit77/async-hid) async HID library
