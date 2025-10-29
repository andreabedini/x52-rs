//! USB protocol definitions for X52 control

/// USB vendor request number
pub const VENDOR_REQUEST: u8 = 0x91;

/// USB request timeout (5 seconds)
pub const REQUEST_TIMEOUT_MS: u64 = 5000;

/// Maximum retry attempts
pub const MAX_RETRIES: usize = 3;

#[derive(Debug, Clone, Copy)]
#[repr(u16)]
pub enum CommandIndex {
    ChangeLed = 0xb8,
    ChangeBrightnessMfd = 0xb1,
    ChangeBrightnessLed = 0xb2,
    ClearMfdLine0 = 0xd9,
    ClearMfdLine1 = 0xda,
    ClearMfdLine2 = 0xdc,
    WriteMfdLine0 = 0xd1,
    WriteMfdLine1 = 0xd2,
    WriteMfdLine2 = 0xd4,
    SetClockTime = 0xc0,
    SetClockOffsetSecondary = 0xc1,
    SetClockOffsetTertiary = 0xc2,
    SetDateDayMonth = 0xc4,
    SetDateYear = 0xc8,
    SetShiftIndicator = 0xfd,
    SetBlinkMode = 0xb4,
}

/// USB command builder
#[derive(Debug, Clone, Copy)]
pub struct Command {
    pub index: CommandIndex,
    pub value: u16,
}
