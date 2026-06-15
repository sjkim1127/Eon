use thiserror::Error;
use serde::Serialize;
use eon_astro::AstroError;

#[derive(Debug, Error, Serialize, Clone, PartialEq, Eq)]
#[serde(tag = "type", content = "message")]
pub enum DataError {
    #[error("Astro engine error: {0}")]
    Astro(#[from] AstroError),

    #[error("Cache deserialization error: {0}")]
    Deserialization(String),

    #[error("Invalid date parameters")]
    InvalidDate,

    #[error("Date calculation overflow")]
    DateCalculationOverflow,
}
