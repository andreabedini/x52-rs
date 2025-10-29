//! LED control module
//!
//! This module is currently a placeholder for future LED-specific functionality.

use derive_more::Display;

use crate::protocol::{Command, CommandIndex};

/// LED identifier on the X52/X52 Pro
#[derive(Display, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Led {
    /// Fire button LED (single color)
    Fire,
    /// A button LED (multicolor)
    A,
    /// B button LED (multicolor)
    B,
    /// D button LED (multicolor)
    D,
    /// E button LED (multicolor)
    E,
    /// T1 button LED (multicolor)
    T1,
    /// T2 button LED (multicolor)
    T2,
    /// T3 button LED (multicolor)
    T3,
    /// POV LED (multicolor)
    Pov,
    /// Clutch/i button LED (multicolor)
    Clutch,
    /// Throttle LED (single color)
    Throttle,
}

impl Led {
    /// Check if LED is multicolor
    pub fn is_multicolor(&self) -> bool {
        !matches!(self, Led::Fire | Led::Throttle)
    }
}

/// LED color/state
#[derive(Display, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LedColor {
    /// Off
    Off,
    /// On (Fire/Throttle only)
    On,
    /// Red
    Red,
    /// Amber (Red + Green)
    Amber,
    /// Green
    Green,
}

impl LedColor {
    /// Convert color to (red, green) bit values for protocol encoding
    pub fn to_bits(&self) -> (u8, u8) {
        match self {
            Self::Red => (1, 0),
            Self::Green => (0, 1),
            Self::Amber => (1, 1),
            _ => (0, 0), // Off and On do not apply here
        }
    }
    /// Check if valid for single-color LEDs
    pub fn is_valid_for_single_color(&self) -> bool {
        matches!(self, Self::Off | Self::On)
    }
}

/// Get LED base number for protocol
fn led_base_number(led: Led) -> u8 {
    match led {
        Led::Fire => 1,
        Led::A => 2,
        Led::B => 4,
        Led::D => 6,
        Led::E => 8,
        Led::T1 => 10,
        Led::T2 => 12,
        Led::T3 => 14,
        Led::Pov => 16,
        Led::Clutch => 18,
        Led::Throttle => 20,
    }
}

/// LED control command (index: 0xb8)
pub fn set_color(led: Led, color: LedColor) -> Vec<Command> {
    let base = led_base_number(led) as u16;

    let (red, green) = color.to_bits();
    if led.is_multicolor() {
        // Multicolor LED
        vec![
            Command {
                index: CommandIndex::ChangeLed,
                value: (base << 8) | red as u16,
            },
            Command {
                index: CommandIndex::ChangeLed,
                value: ((base + 1) << 8) | green as u16,
            },
        ]
    } else {
        // Single color LED
        match color {
            LedColor::Off => vec![Command {
                index: CommandIndex::ChangeLed,
                value: base << 8,
            }],
            // Map any other color to "On"
            _ => vec![Command {
                index: CommandIndex::ChangeLed,
                value: (base << 8) | 0x01,
            }],
        }
    }
}

/// LED brightness control (index: 0xb2)
pub fn set_brightness(level: u8) -> Command {
    let clamped = level.min(128);
    Command {
        index: CommandIndex::ChangeBrightnessLed,
        value: clamped as u16,
    }
}
