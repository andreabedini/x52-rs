//! MFD (Multi-Function Display) control module
//!
//! This module is currently a placeholder for future MFD-specific functionality
//! like character encoding and text formatting.

use crate::protocol::{Command, CommandIndex};

/// MFD display line selector
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MfdLine {
    /// Top line
    Line0,
    /// Middle line
    Line1,
    /// Bottom line
    Line2,
}

/// MFD brightness control (index: 0xb1)
pub fn set_brightness(level: u8) -> Command {
    let index = CommandIndex::ChangeBrightnessMfd;
    let value = level.min(128) as u16;
    Command { index, value }
}

/// Clear MFD line (0xd9, 0xda, 0xdc for lines 0, 1, 2)
pub fn clear_line(line: MfdLine) -> Command {
    let index = match line {
        MfdLine::Line0 => CommandIndex::ClearMfdLine0,
        MfdLine::Line1 => CommandIndex::ClearMfdLine1,
        MfdLine::Line2 => CommandIndex::ClearMfdLine2,
    };
    let value = 0u16;
    Command { index, value }
}

/// Write character pair to MFD line (0xd1, 0xd2, 0xd4 for lines 0, 1, 2)
pub fn write_chars(line: MfdLine, char1: u8, char2: u8) -> Command {
    let index = match line {
        MfdLine::Line0 => CommandIndex::WriteMfdLine0,
        MfdLine::Line1 => CommandIndex::WriteMfdLine1,
        MfdLine::Line2 => CommandIndex::WriteMfdLine2,
    };
    // Characters in little-endian: char1 | (char2 << 8)
    let value = (char1 as u16) | ((char2 as u16) << 8);
    Command { index, value }
}
