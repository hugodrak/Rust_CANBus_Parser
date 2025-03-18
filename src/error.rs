// error.rs

/// Custom error type for CAN bus parsing.
#[derive(Debug)]
pub enum CanError {
    /// Indicates that the CAN frame is invalid (e.g., not enough bytes).
    InvalidFrame(String),
    /// Indicates an unknown or unhandled CAN ID.
    UnknownId(u32),
    /// Generic catch-all error type for anything else.
    Other(String),
}

impl std::fmt::Display for CanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CanError::InvalidFrame(details) => write!(f, "Invalid CAN frame: {}", details),
            CanError::UnknownId(id) => write!(f, "Unknown CAN ID: {}", id),
            CanError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for CanError {}
