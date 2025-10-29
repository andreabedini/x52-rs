use thiserror::Error;

/// Error type for X52 operations
#[derive(Debug, Error)]
pub enum Error {
    #[error("Device not found")]
    DeviceNotFound,

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Not supported: {0}")]
    NotSupported(String),

    #[error("Value out of range: {0}")]
    OutOfRange(String),

    #[error("Other error: {0}")]
    Other(String),
}
