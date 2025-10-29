//! Shared protocol definitions for Saitek X52/X52 Pro HOTAS
//!
//! This crate provides common types, constants, and utilities used by both
//! the input (x52io) and output (x52) libraries.

pub mod device;

// Re-export commonly used types
pub use device::DeviceVariant;
