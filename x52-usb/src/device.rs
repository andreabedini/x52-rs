//! X52 device control

use crate::clock::{self, ClockFormat};
use crate::error::Error;
use crate::led::{self, Led, LedColor};
use crate::protocol::{Command, MAX_RETRIES, REQUEST_TIMEOUT_MS, VENDOR_REQUEST};
use crate::{ClockOffsetId, MfdLine, mfd, misc};

use anyhow::Result;

use log::{debug, error, info, trace, warn};
use nusb::transfer::{ControlOut, ControlType, Recipient};
use smol::lock::Mutex;
use std::sync::Arc;
use std::time::Duration;
use x52_device::DeviceVariant;
use x52_device::device::VENDOR_SAITEK;

/// X52/X52 Pro output control device
pub struct X52Device {
    interface: Arc<nusb::Interface>,
    variant: DeviceVariant,
    /// Internal state for deferred updates (optional)
    state: Arc<Mutex<DeviceState>>,
}

#[derive(Debug, Default)]
struct DeviceState {
    /// MFD text (3 lines, 16 chars each)
    mfd_lines: [String; 3],
    /// MFD brightness (0-128)
    mfd_brightness: u8,
    /// LED brightness (0-128)
    led_brightness: u8,
}

impl X52Device {
    /// Open first available X52/X52 Pro device
    pub async fn open() -> Result<Self> {
        Self::open_with_filter(|_| true).await
    }

    /// Open X52 device with custom filter
    pub async fn open_with_filter<F>(filter: F) -> Result<Self>
    where
        F: Fn(&nusb::DeviceInfo) -> bool,
    {
        debug!("Scanning for X52/X52 Pro devices");

        let devices = nusb::list_devices().await?;

        for dev_info in devices {
            if dev_info.vendor_id() != VENDOR_SAITEK {
                continue;
            }

            let pid = dev_info.product_id();
            let variant = match DeviceVariant::from_product_id(pid) {
                Some(v) => v,
                None => continue,
            };

            if !filter(&dev_info) {
                continue;
            }

            info!(
                "Found {} device (VID: {:04x}, PID: {:04x})",
                variant.name(),
                dev_info.vendor_id(),
                pid
            );

            let device = match dev_info.open().await {
                Ok(d) => d,
                Err(e) => {
                    warn!("Failed to open device: {}", e);
                    continue;
                }
            };

            let interface = match device.claim_interface(0).await {
                Ok(i) => i,
                Err(e) => {
                    warn!("Failed to claim interface 0: {}", e);
                    continue;
                }
            };

            info!("Successfully opened {} device", variant.name());

            return Ok(Self {
                interface: Arc::new(interface),
                variant,
                state: Arc::new(Mutex::new(DeviceState::default())),
            });
        }

        Err(Error::DeviceNotFound.into())
    }

    /// Send USB control transfer command
    async fn send_command(&self, cmd: Command) -> Result<()> {
        self.send_command_with_retry(cmd, MAX_RETRIES).await
    }

    /// Send command with retry logic
    async fn send_command_with_retry(&self, cmd: Command, retries: usize) -> Result<()> {
        trace!(
            "Sending USB command: index=0x{:04x}, value=0x{:04x}",
            cmd.index as u16, cmd.value
        );

        for attempt in 0..retries {
            match self.try_send_command(cmd).await {
                Ok(()) => return Ok(()),
                Err(e) => {
                    if attempt + 1 < retries {
                        debug!(
                            "Command failed (attempt {}/{}): {}, retrying...",
                            attempt + 1,
                            retries,
                            e
                        );
                        smol::Timer::after(Duration::from_millis(10)).await;
                    } else {
                        error!("Command failed after {} attempts: {}", retries, e);
                        return Err(e);
                    }
                }
            }
        }

        Err(Error::Other("All retries exhausted".to_owned()).into())
    }

    /// Try to send command once
    async fn try_send_command(&self, cmd: Command) -> Result<()> {
        // USB Vendor Control Transfer
        // bmRequestType: VENDOR | DEVICE | OUT
        // bRequest: 0x91
        // wValue: command-specific
        // wIndex: command-specific
        // wLength: 0 (no data phase)

        let timeout = Duration::from_millis(REQUEST_TIMEOUT_MS);

        let result = self
            .interface
            .control_out(
                ControlOut {
                    control_type: ControlType::Vendor,
                    recipient: Recipient::Device,
                    request: VENDOR_REQUEST,
                    value: cmd.value,
                    index: cmd.index as u16,
                    data: &[],
                },
                timeout,
            )
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    /// Send multiple commands
    async fn send_commands(&self, commands: &[Command]) -> Result<()> {
        for cmd in commands {
            self.send_command(*cmd).await?;
        }
        Ok(())
    }

    /// Get device variant
    pub fn variant(&self) -> DeviceVariant {
        self.variant
    }

    /// Check if device supports LED control
    pub fn supports_led_control(&self) -> bool {
        self.variant.supports_led_control()
    }

    /// Set LED state
    pub async fn set_led_color(&self, led: Led, color: LedColor) -> Result<()> {
        // Validate LED control support
        if !self.supports_led_control() && !matches!(led, Led::Fire | Led::Throttle) {
            return Err(Error::NotSupported(format!(
                "{} does not support {led} LED control",
                self.variant.name(),
            ))
            .into());
        }

        // Validate color for single-color LEDs
        if !led.is_multicolor() && !color.is_valid_for_single_color() {
            return Err(Error::InvalidParameter(format!(
                "{led} LED only supports On/Off, not {color}",
            ))
            .into());
        }

        let commands = led::set_color(led, color);
        self.send_commands(&commands).await
    }

    /// Set brightness
    pub async fn set_brightness(&self, level: u8) -> Result<()> {
        if level > 128 {
            return Err(Error::OutOfRange("Brightness must be 0-128".to_owned()).into());
        }

        let cmd = led::set_brightness(level);
        self.send_command(cmd).await?;

        // Update internal state
        let mut state = self.state.lock().await;
        state.led_brightness = level;

        Ok(())
    }

    /// Set MFD backlight brightness (0-128)
    pub async fn set_mfd_brightness(&self, level: u8) -> Result<()> {
        if level > 128 {
            return Err(Error::OutOfRange("Brightness must be 0-128".to_owned()).into());
        }

        let cmd = mfd::set_brightness(level);
        self.send_command(cmd).await?;

        // Update internal state
        let mut state = self.state.lock().await;
        state.mfd_brightness = level;

        Ok(())
    }

    /// Set MFD text line
    pub async fn set_text(&self, line: mfd::MfdLine, text: &str) -> Result<()> {
        // Truncate or pad to 16 characters
        let mut chars = [b' '; 16];
        for (i, b) in text.bytes().take(16).enumerate() {
            chars[i] = b;
        }

        // Clear line first
        self.send_command(mfd::clear_line(line)).await?;

        // Send character pairs (little-endian)
        for chunk in chars.chunks(2) {
            let char1 = chunk[0];
            let char2 = chunk.get(1).copied().unwrap_or(b' ');
            self.send_command(mfd::write_chars(line, char1, char2))
                .await?;
        }

        // Update internal state
        let mut state = self.state.lock().await;
        state.mfd_lines[line as usize] = text.to_string();

        Ok(())
    }

    /// Set clock time
    pub async fn set_clock(&self, hour: u8, minute: u8, format: ClockFormat) -> Result<()> {
        if hour >= 24 {
            return Err(Error::OutOfRange("Hour must be 0-23".to_owned()).into());
        }
        if minute >= 60 {
            return Err(Error::OutOfRange("Minute must be 0-59".to_owned()).into());
        }

        self.send_command(clock::set_time(hour, minute, format))
            .await
    }

    /// Set clock timezone offset (for secondary/tertiary clocks)
    pub async fn set_clock_offset(
        &self,
        clock: ClockOffsetId,
        offset_minutes: i32,
        format: ClockFormat,
    ) -> Result<()> {
        if offset_minutes.abs() > 1440 {
            return Err(Error::OutOfRange(
                "Offset must be within ±1440 minutes (24 hours)".to_owned(),
            )
            .into());
        }

        let cmd = clock::set_offset(clock, offset_minutes, format);
        self.send_command(cmd).await
    }

    /// Set date
    pub async fn set_date(&self, day: u8, month: u8, year: u8) -> Result<()> {
        if day == 0 || day > 31 {
            return Err(Error::OutOfRange("Day must be 1-31".to_owned()).into());
        }
        if month == 0 || month > 12 {
            return Err(Error::OutOfRange("Month must be 1-12".to_owned()).into());
        }
        if year > 99 {
            return Err(Error::OutOfRange("Year must be 0-99".to_owned()).into());
        }

        self.send_command(clock::set_date_day_month(day, month))
            .await?;
        self.send_command(clock::set_date_year(year)).await?;

        Ok(())
    }

    /// Set shift indicator
    pub async fn set_shift(&self, state: bool) -> Result<()> {
        let cmd = misc::set_shift(state);
        self.send_command(cmd).await
    }

    /// Set blink mode
    pub async fn set_blink(&self, state: bool) -> Result<()> {
        let cmd = misc::set_blink(state);
        self.send_command(cmd).await
    }

    /// Create a batch update builder for sending multiple commands at once
    pub fn batch(&self) -> BatchUpdate {
        BatchUpdate::new(self.interface.clone())
    }
}

/// Builder for sending multiple USB commands in a single batch
pub struct BatchUpdate {
    device: Arc<nusb::Interface>,
    commands: Vec<Command>,
}

impl BatchUpdate {
    /// Create new batch update
    pub(crate) fn new(device: Arc<nusb::Interface>) -> Self {
        Self {
            device,
            commands: Vec::new(),
        }
    }

    /// Add LED command
    pub fn led(mut self, led: Led, color: LedColor) -> Self {
        let cmds = led::set_color(led, color);
        self.commands.extend(cmds);
        self
    }

    /// Add brightness command
    pub fn led_brightness(mut self, level: u8) -> Self {
        self.commands.push(led::set_brightness(level));
        self
    }
    /// Add MFD brightness command
    pub fn mfd_brightness(mut self, level: u8) -> Self {
        self.commands.push(mfd::set_brightness(level));
        self
    }

    /// Add MFD text command
    pub fn text(mut self, line: MfdLine, text: &str) -> Self {
        // Truncate or pad to 16 characters
        let mut chars = [b' '; 16];
        for (i, b) in text.bytes().take(16).enumerate() {
            chars[i] = b;
        }

        // Clear line
        self.commands.push(mfd::clear_line(line));

        // Add character pairs
        for chunk in chars.chunks(2) {
            let char1 = chunk[0];
            let char2 = chunk.get(1).copied().unwrap_or(b' ');
            self.commands.push(mfd::write_chars(line, char1, char2));
        }

        self
    }

    /// Add clock time command
    pub fn clock(mut self, hour: u8, minute: u8, format: ClockFormat) -> Self {
        if hour < 24 && minute < 60 {
            self.commands.push(clock::set_time(hour, minute, format));
        }
        self
    }

    /// Add clock offset command
    pub fn clock_offset(
        mut self,
        clock: ClockOffsetId,
        offset_minutes: i32,
        format: ClockFormat,
    ) -> Self {
        if offset_minutes.abs() <= 1440 {
            self.commands
                .push(clock::set_offset(clock, offset_minutes, format));
        }
        self
    }

    /// Add date command
    pub fn date(mut self, day: u8, month: u8, year: u8) -> Self {
        if day > 0 && day <= 31 && month > 0 && month <= 12 && year <= 99 {
            self.commands.push(clock::set_date_day_month(day, month));
            self.commands.push(clock::set_date_year(year));
        }
        self
    }

    /// Add shift indicator command
    pub fn shift(mut self, state: bool) -> Self {
        self.commands.push(misc::set_shift(state));
        self
    }

    /// Add blink command
    pub fn blink(mut self, state: bool) -> Self {
        self.commands.push(misc::set_blink(state));
        self
    }

    /// Apply all commands
    pub async fn apply(self) -> Result<()> {
        debug!(
            "Applying batch update with {} commands",
            self.commands.len()
        );

        let timeout = Duration::from_millis(REQUEST_TIMEOUT_MS);

        for cmd in self.commands {
            let result = self
                .device
                .control_out(
                    ControlOut {
                        control_type: ControlType::Vendor,
                        recipient: Recipient::Device,
                        request: VENDOR_REQUEST,
                        value: cmd.value,
                        index: cmd.index as u16,
                        data: &[],
                    },
                    timeout,
                )
                .await;

            match result {
                Ok(_) => continue,
                Err(e) => return Err(e.into()),
            }
        }

        debug!("Batch update completed successfully");
        Ok(())
    }
}
