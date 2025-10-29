//! Device identification and variant detection

/// Saitek vendor ID
pub const VENDOR_SAITEK: u16 = 0x06a3;

/// X52 Pro product ID
pub const PRODUCT_X52_PRO: u16 = 0x0762;
/// X52 (v1) product ID
pub const PRODUCT_X52_1: u16 = 0x0255;
/// X52 (v2) product ID
pub const PRODUCT_X52_2: u16 = 0x075C;

/// All supported product IDs
pub const PRODUCT_IDS: [u16; 3] = [PRODUCT_X52_PRO, PRODUCT_X52_1, PRODUCT_X52_2];

/// Device variant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeviceVariant {
    /// X52 Pro - Full LED control
    X52Pro,
    /// X52 (original) - No LED control
    X52,
}

impl DeviceVariant {
    /// Detect device variant from product ID
    pub fn from_product_id(pid: u16) -> Option<Self> {
        match pid {
            PRODUCT_X52_PRO => Some(Self::X52Pro),
            PRODUCT_X52_1 | PRODUCT_X52_2 => Some(Self::X52),
            _ => None,
        }
    }

    /// Check if this device supports LED control
    pub fn supports_led_control(&self) -> bool {
        matches!(self, Self::X52Pro)
    }

    /// Get device name
    pub fn name(&self) -> &'static str {
        match self {
            Self::X52Pro => "X52 Pro",
            Self::X52 => "X52",
        }
    }
}
