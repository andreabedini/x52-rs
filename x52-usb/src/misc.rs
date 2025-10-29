use crate::protocol::{Command, CommandIndex};

/// Set shift indicator (index: 0xfd, value: 0x51=on, 0x50=off)
pub fn set_shift(state: bool) -> Command {
    let index = CommandIndex::SetShiftIndicator;
    let value = if state { 0x51 } else { 0x50 };
    Command { index, value }
}

/// Set blink mode (index: 0xb4, value: 0x51=on, 0x50=off)
pub fn set_blink(state: bool) -> Command {
    let index = CommandIndex::SetBlinkMode;
    let value = if state { 0x51 } else { 0x50 };
    Command { index, value }
}
