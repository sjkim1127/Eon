use eon_astro::AstroError;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize, Clone, PartialEq, Eq)]
#[serde(tag = "type", content = "message")]
pub enum VedicError {
    #[error("Astronomical calculation error: {0}")]
    Astro(#[from] AstroError),

    #[error("Calculation error: {0}")]
    CalculationError(String),

    #[error("Invalid parameters: {0}")]
    InvalidParameters(String),
}
