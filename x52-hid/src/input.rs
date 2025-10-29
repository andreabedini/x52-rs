//! Input-related types (axes, buttons, reports)

use std::fmt;

/// Joystick axes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Axis {
    /// Stick X axis (left/right)
    X = 0,
    /// Stick Y axis (forward/back)
    Y = 1,
    /// Stick twist (RZ)
    RZ = 2,
    /// Throttle (Z)
    Z = 3,
    /// Throttle rotary X (RX)
    RX = 4,
    /// Throttle rotary Y (RY)
    RY = 5,
    /// Throttle slider
    Slider = 6,
    /// Thumbstick X (on throttle)
    ThumbX = 7,
    /// Thumbstick Y (on throttle)
    ThumbY = 8,
    /// POV hat X (-1, 0, 1)
    HatX = 9,
    /// POV hat Y (-1, 0, 1)
    HatY = 10,
}

impl Axis {
    /// Total number of axes
    pub const COUNT: usize = 11;

    /// Get axis name
    pub fn name(&self) -> &'static str {
        match self {
            Self::X => "X",
            Self::Y => "Y",
            Self::RZ => "RZ",
            Self::Z => "Z",
            Self::RX => "RX",
            Self::RY => "RY",
            Self::Slider => "Slider",
            Self::ThumbX => "ThumbX",
            Self::ThumbY => "ThumbY",
            Self::HatX => "HatX",
            Self::HatY => "HatY",
        }
    }

    /// Get axis from index (0-10)
    pub fn from_index(idx: usize) -> Option<Self> {
        match idx {
            0 => Some(Self::X),
            1 => Some(Self::Y),
            2 => Some(Self::RZ),
            3 => Some(Self::Z),
            4 => Some(Self::RX),
            5 => Some(Self::RY),
            6 => Some(Self::Slider),
            7 => Some(Self::ThumbX),
            8 => Some(Self::ThumbY),
            9 => Some(Self::HatX),
            10 => Some(Self::HatY),
            _ => None,
        }
    }

    /// Get axis index
    pub fn index(&self) -> usize {
        *self as usize
    }
}

impl fmt::Display for Axis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Axis value range
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AxisRange {
    /// Minimum value
    pub min: i32,
    /// Maximum value
    pub max: i32,
}

impl AxisRange {
    /// Create new axis range
    pub const fn new(min: i32, max: i32) -> Self {
        Self { min, max }
    }
}

/// Joystick buttons
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Button {
    /// Primary trigger (first stage)
    Trigger = 0,
    /// Fire button
    Fire = 1,
    /// A button
    A = 2,
    /// B button
    B = 3,
    /// C button
    C = 4,
    /// Pinkie switch
    Pinkie = 5,
    /// D button
    D = 6,
    /// E button
    E = 7,
    /// Toggle 1
    T1 = 8,
    /// Toggle 2
    T2 = 9,
    /// Toggle 3
    T3 = 10,
    /// Toggle 4
    T4 = 11,
    /// Toggle 5
    T5 = 12,
    /// Toggle 6
    T6 = 13,
    /// Primary trigger (second stage)
    TriggerStage2 = 14,
    /// POV up
    PovUp = 15,
    /// POV right
    PovRight = 16,
    /// POV down
    PovDown = 17,
    /// POV left
    PovLeft = 18,
    /// POV 2 up
    Pov2Up = 19,
    /// POV 2 right
    Pov2Right = 20,
    /// POV 2 down
    Pov2Down = 21,
    /// POV 2 left
    Pov2Left = 22,
    /// Clutch (i button)
    Clutch = 23,
    /// Mouse button (primary)
    MousePrimary = 24,
    /// Mouse button (secondary)
    MouseSecondary = 25,
    /// Mouse wheel click
    MouseWheel = 26,
    /// Wheel scroll up
    WheelUp = 27,
    /// Wheel scroll down
    WheelDown = 28,
    /// Mode 1 button
    Mode1 = 29,
    /// Mode 2 button
    Mode2 = 30,
    /// Mode 3 button
    Mode3 = 31,
    /// Function button
    Function = 32,
    /// Start/Stop button
    StartStop = 33,
    /// Reset button
    Reset = 34,
    /// Page up (X52 Pro only)
    PageUp = 35,
    /// Page down (X52 Pro only)
    PageDown = 36,
    /// Up arrow (X52 Pro only)
    Up = 37,
    /// Down arrow (X52 Pro only)
    Down = 38,
    /// Select button (X52 Pro only)
    Select = 39,
}

impl Button {
    /// Total number of buttons
    pub const COUNT: usize = 40;

    /// Get button name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Trigger => "Trigger",
            Self::Fire => "Fire",
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
            Self::Pinkie => "Pinkie",
            Self::D => "D",
            Self::E => "E",
            Self::T1 => "T1",
            Self::T2 => "T2",
            Self::T3 => "T3",
            Self::T4 => "T4",
            Self::T5 => "T5",
            Self::T6 => "T6",
            Self::TriggerStage2 => "Trigger Stage 2",
            Self::PovUp => "POV Up",
            Self::PovRight => "POV Right",
            Self::PovDown => "POV Down",
            Self::PovLeft => "POV Left",
            Self::Pov2Up => "POV2 Up",
            Self::Pov2Right => "POV2 Right",
            Self::Pov2Down => "POV2 Down",
            Self::Pov2Left => "POV2 Left",
            Self::Clutch => "Clutch",
            Self::MousePrimary => "Mouse Primary",
            Self::MouseSecondary => "Mouse Secondary",
            Self::MouseWheel => "Mouse Wheel",
            Self::WheelUp => "Wheel Up",
            Self::WheelDown => "Wheel Down",
            Self::Mode1 => "Mode 1",
            Self::Mode2 => "Mode 2",
            Self::Mode3 => "Mode 3",
            Self::Function => "Function",
            Self::StartStop => "Start/Stop",
            Self::Reset => "Reset",
            Self::PageUp => "Page Up",
            Self::PageDown => "Page Down",
            Self::Up => "Up",
            Self::Down => "Down",
            Self::Select => "Select",
        }
    }

    /// Get button from index (0-39)
    pub fn from_index(idx: usize) -> Option<Self> {
        if idx < Self::COUNT {
            // SAFETY: We just checked the range
            Some(unsafe { std::mem::transmute::<u8, Button>(idx as u8) })
        } else {
            None
        }
    }

    /// Get button index
    pub fn index(&self) -> usize {
        *self as usize
    }
}

impl fmt::Display for Button {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// POV hat position (raw value 0-8)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum HatPosition {
    /// Centered (neutral)
    #[default]
    Center = 0,
    /// North (up)
    North = 1,
    /// North-east
    NorthEast = 2,
    /// East (right)
    East = 3,
    /// South-east
    SouthEast = 4,
    /// South (down)
    South = 5,
    /// South-west
    SouthWest = 6,
    /// West (left)
    West = 7,
    /// North-west
    NorthWest = 8,
}

impl HatPosition {
    /// Get hat position from raw value
    pub fn from_raw(value: u8) -> Self {
        match value {
            1 => Self::North,
            2 => Self::NorthEast,
            3 => Self::East,
            4 => Self::SouthEast,
            5 => Self::South,
            6 => Self::SouthWest,
            7 => Self::West,
            8 => Self::NorthWest,
            _ => Self::Center,
        }
    }

    /// Convert to axis values (x, y)
    /// Returns (-1, 0, 1) for each axis
    pub fn to_axis_values(&self) -> (i32, i32) {
        match self {
            Self::Center => (0, 0),
            Self::North => (0, -1),
            Self::NorthEast => (1, -1),
            Self::East => (1, 0),
            Self::SouthEast => (1, 1),
            Self::South => (0, 1),
            Self::SouthWest => (-1, 1),
            Self::West => (-1, 0),
            Self::NorthWest => (-1, -1),
        }
    }

    /// Get raw value
    pub fn raw(&self) -> u8 {
        *self as u8
    }
}

impl fmt::Display for HatPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Center => "Center",
            Self::North => "N",
            Self::NorthEast => "NE",
            Self::East => "E",
            Self::SouthEast => "SE",
            Self::South => "S",
            Self::SouthWest => "SW",
            Self::West => "W",
            Self::NorthWest => "NW",
        };
        write!(f, "{}", name)
    }
}
