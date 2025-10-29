use thiserror::Error;

/// Error type for X52 operations
#[derive(Debug, Error)]
pub enum Error {
    /// Device not found
    #[error("X52 device not found")]
    DeviceNotFound,

    /// Invalid parameter
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// Feature not supported by this device variant
    #[error("Feature not supported by {0}")]
    NotSupported(String),

    /// Value out of range
    #[error("Value out of range: {0}")]
    OutOfRange(String),

    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Timeout
    #[error("Operation timed out")]
    Timeout,

    /// Device disconnected
    #[error("Device disconnected")]
    Disconnected,

    /// USB error
    #[error("USB error: {0}")]
    Usb(String),

    /// HID error
    #[error("HID error: {0}")]
    HidError(async_hid::HidError),

    /// Other error
    #[error("{0}")]
    Other(String),
}

impl From<async_hid::HidError> for Error {
    fn from(err: async_hid::HidError) -> Self {
        Error::HidError(err)
    }
}
