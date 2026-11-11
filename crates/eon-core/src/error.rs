//! Common error types for Eon Core.

use serde::Serialize;
use thiserror::Error;

/// Common platform error type for Eon
#[derive(Debug, Error, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum EonError {
    /// Invalid date or time
    #[error("Invalid datetime: {0}")]
    InvalidDateTime(String),

    /// Invalid index
    #[error("Invalid index: {0}")]
    InvalidIndex(i32),

    /// Data query failure
    #[error("Data not found: {0}")]
    DataNotFound(String),

    /// Calculation error
    #[error("Calculation error: {0}")]
    CalculationError(String),
}

/// Core module errors
#[derive(Debug, thiserror::Error, serde::Serialize, Clone, PartialEq, Eq)]
#[serde(tag = "type", content = "message")]
pub enum CoreError {
    /// Invalid date or time parameters
    #[error("Invalid date or time parameters")]
    InvalidDateTime,

    /// Local date/time does not exist in target timezone
    #[error("Local date/time is non-existent in the target timezone")]
    NonExistentLocalTime,

    /// Invalid timezone format
    #[error("Invalid timezone format: {0}")]
    InvalidTimezone(String),
}
