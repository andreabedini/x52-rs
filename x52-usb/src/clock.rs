//! Clock and date control module
//!
//! This module is currently a placeholder for future clock-specific functionality
//! like timezone calculations and date formatting.

use derive_more::Display;

use crate::protocol::{Command, CommandIndex};

/// Clock identifier for timezone offset configuration
#[derive(Display, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ClockOffsetId {
    /// Secondary clock
    Secondary = 1,
    /// Tertiary clock
    Tertiary = 2,
}

/// Clock display format
#[derive(Display, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClockFormat {
    /// 12-hour format
    Hours12,
    /// 24-hour format
    Hours24,
}

/// Date display format
#[derive(Display, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DateFormat {
    /// DD-MM-YY
    DayMonthYear,
    /// MM-DD-YY
    MonthDayYear,
    /// YY-MM-DD
    YearMonthDay,
}

/// Set time on primary clock (index: 0xc0)
pub fn set_time(hour: u8, minute: u8, format: ClockFormat) -> Command {
    let index = CommandIndex::SetClockTime;
    let format_bit = match format {
        ClockFormat::Hours24 => 1u16 << 15,
        ClockFormat::Hours12 => 0,
    };
    let h = (hour as u16 & 0x7F) << 8;
    let m = minute as u16 & 0xFF;
    let value = format_bit | h | m;
    Command { index, value }
}

/// Set timezone offset for secondary/tertiary clock (0xc1, 0xc2)
pub fn set_offset(clock: ClockOffsetId, offset_minutes: i32, format: ClockFormat) -> Command {
    let index = match clock {
        ClockOffsetId::Secondary => CommandIndex::SetClockOffsetSecondary,
        ClockOffsetId::Tertiary => CommandIndex::SetClockOffsetTertiary,
    };

    let format_bit = match format {
        ClockFormat::Hours24 => 1u16 << 15,
        ClockFormat::Hours12 => 0,
    };

    let negative = offset_minutes < 0;
    let abs_offset = offset_minutes.unsigned_abs() as u16 & 0x3FF; // 10 bits max
    let neg_bit = if negative { 1u16 << 10 } else { 0 };
    let value = format_bit | neg_bit | abs_offset;

    Command { index, value }
}

/// Set date (index: 0xc4 for day/month, 0xc8 for year)
pub fn set_date_day_month(day: u8, month: u8) -> Command {
    let index = CommandIndex::SetDateDayMonth;
    let d = day as u16 & 0xFF;
    let m = (month as u16 & 0xFF) << 8;
    let value = m | d;
    Command { index, value }
}

/// Set year (index: 0xc8)
pub fn set_date_year(year: u8) -> Command {
    let index = CommandIndex::SetDateYear;
    let value = year as u16 & 0xFF;
    Command { index, value }
}
