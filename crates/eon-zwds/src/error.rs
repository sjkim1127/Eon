use thiserror::Error;

#[derive(Debug, Error)]
pub enum ZwdsError {
    #[error("Lunar conversion failed: {0}")]
    LunarConversion(String),

    #[error("Invalid birth date: {0}")]
    InvalidBirthDate(String),

    #[error("Invalid time index: {0}")]
    InvalidTimeIndex(u32),

    #[error("Invalid lunar day: {0}")]
    InvalidLunarDay(u32),

    #[error("Internal error: {0}")]
    Internal(String),
}
