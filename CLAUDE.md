# CLAUDE.md

## Build Commands

```bash
cargo check --all          # Type-check all crates
cargo build --all          # Build all crates
cargo test --all           # Run all tests
cargo build --examples --all  # Build all examples
cargo clippy --all         # Lint
```

### Running Examples

Examples live in each crate's `examples/` directory (auto-discovered):

```bash
cargo run --example basic_input -p x52-hid    # HID input reading
cargo run --example led_control -p x52-usb    # LED control
cargo run --example mfd_text -p x52-usb       # MFD text display
cargo run --example batch_update -p x52-usb   # Batch updates
```

Use `RUST_LOG=debug` for verbose output.

## Architecture

Rust workspace with three crates for controlling Saitek/MadCatz X52/X52 Pro HOTAS:

| Crate | Lib name | Purpose |
|-------|----------|---------|
| `x52-device` | `x52_device` | Shared types and protocol definitions (DeviceVariant) |
| `x52-hid` | `x52_hid` | Async HID input reading (axes, buttons, hat) via `async-hid` |
| `x52-usb` | `x52_usb` | Async USB output control (MFD, LEDs, clocks) via `nusb` |

Input and output use different USB mechanisms by design:
- **Input** (`x52-hid`): Standard HID reports via `/dev/hidraw*`
- **Output** (`x52-usb`): USB vendor control requests to endpoint 0

This lets both coexist without driver conflicts.

## Key Patterns

- **Async runtime**: `smol`. All device I/O is async.
- **Edition**: Rust 2024 (`resolver = "3"`)
- **Error handling**: `thiserror` for library errors, `anyhow` in examples/binaries
- **Re-exports**: Each crate's `lib.rs` re-exports public API from internal modules
- **Supported devices**: X52 Pro (0x06a3:0x0762), X52 v1 (0x06a3:0x0255), X52 v2 (0x06a3:0x075C)
