//! HID input report structures

use crate::input::{Axis, AxisRange, Button, HatPosition};
use x52_device::DeviceVariant;

/// Axis values for all 11 axes
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AxisReport {
    values: [i32; Axis::COUNT],
}

impl AxisReport {
    /// Create new axis report with all zeros
    pub fn new() -> Self {
        Self {
            values: [0; Axis::COUNT],
        }
    }

    /// Get axis value by enum
    pub fn get(&self, axis: Axis) -> i32 {
        self.values[axis.index()]
    }

    /// Set axis value by enum
    pub fn set(&mut self, axis: Axis, value: i32) {
        self.values[axis.index()] = value;
    }

    /// Get axis value by index (0-10)
    pub fn get_by_index(&self, idx: usize) -> Option<i32> {
        self.values.get(idx).copied()
    }

    /// Get all axis values as a slice
    pub fn as_slice(&self) -> &[i32] {
        &self.values
    }
}

impl Default for AxisReport {
    fn default() -> Self {
        Self::new()
    }
}

/// Button states for all 40 buttons
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ButtonState {
    states: [bool; Button::COUNT],
}

impl ButtonState {
    /// Create new button state with all released
    pub fn new() -> Self {
        Self {
            states: [false; Button::COUNT],
        }
    }

    /// Check if button is pressed
    pub fn is_pressed(&self, button: Button) -> bool {
        self.states[button.index()]
    }

    /// Set button state
    pub fn set(&mut self, button: Button, pressed: bool) {
        self.states[button.index()] = pressed;
    }

    /// Get button state by index (0-39)
    pub fn get_by_index(&self, idx: usize) -> Option<bool> {
        self.states.get(idx).copied()
    }

    /// Get all button states as a slice
    pub fn as_slice(&self) -> &[bool] {
        &self.states
    }

    /// Get number of pressed buttons
    pub fn count_pressed(&self) -> usize {
        self.states.iter().filter(|&&s| s).count()
    }
}

impl Default for ButtonState {
    fn default() -> Self {
        Self::new()
    }
}

/// Complete HID input report
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputReport {
    /// Axis values
    pub axes: AxisReport,
    /// Button states
    pub buttons: ButtonState,
    /// Current mode (1, 2, or 3)
    pub mode: u8,
    /// POV hat position
    pub hat: HatPosition,
}

impl InputReport {
    /// Create new input report with default values
    pub fn new() -> Self {
        Self {
            axes: AxisReport::new(),
            buttons: ButtonState::new(),
            mode: 1,
            hat: HatPosition::default(),
        }
    }
}

impl Default for InputReport {
    fn default() -> Self {
        Self::new()
    }
}

/// Axis ranges for different device variants
pub(crate) struct AxisRanges {
    ranges: [AxisRange; Axis::COUNT],
}

impl AxisRanges {
    /// Get axis ranges for device variant
    pub fn for_variant(variant: DeviceVariant) -> Self {
        match variant {
            DeviceVariant::X52Pro => Self::x52_pro(),
            DeviceVariant::X52 => Self::x52(),
        }
    }

    /// X52 Pro axis ranges (10-bit X/Y/RZ)
    fn x52_pro() -> Self {
        Self {
            ranges: [
                AxisRange::new(0, 1023), // X (10-bit)
                AxisRange::new(0, 1023), // Y (10-bit)
                AxisRange::new(0, 1023), // RZ (10-bit)
                AxisRange::new(0, 255),  // Z (8-bit)
                AxisRange::new(0, 255),  // RX (8-bit)
                AxisRange::new(0, 255),  // RY (8-bit)
                AxisRange::new(0, 255),  // Slider (8-bit)
                AxisRange::new(0, 15),   // ThumbX (4-bit)
                AxisRange::new(0, 15),   // ThumbY (4-bit)
                AxisRange::new(-1, 1),   // HatX (derived)
                AxisRange::new(-1, 1),   // HatY (derived)
            ],
        }
    }

    /// X52 axis ranges (11-bit X/Y, 10-bit RZ)
    fn x52() -> Self {
        Self {
            ranges: [
                AxisRange::new(0, 2047), // X (11-bit)
                AxisRange::new(0, 2047), // Y (11-bit)
                AxisRange::new(0, 1023), // RZ (10-bit)
                AxisRange::new(0, 255),  // Z (8-bit)
                AxisRange::new(0, 255),  // RX (8-bit)
                AxisRange::new(0, 255),  // RY (8-bit)
                AxisRange::new(0, 255),  // Slider (8-bit)
                AxisRange::new(0, 15),   // ThumbX (4-bit)
                AxisRange::new(0, 15),   // ThumbY (4-bit)
                AxisRange::new(-1, 1),   // HatX (derived)
                AxisRange::new(-1, 1),   // HatY (derived)
            ],
        }
    }

    /// Get range for specific axis
    pub fn get(&self, axis: Axis) -> AxisRange {
        self.ranges[axis.index()]
    }
}
