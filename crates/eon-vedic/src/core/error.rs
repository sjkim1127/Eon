use thiserror::Error;
use serde::Serialize;
use eon_astro::AstroError;

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
