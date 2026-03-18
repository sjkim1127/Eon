use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("birth info construction failed: {0}")]
    BirthInfo(String),

    #[error("saju analysis failed: {0}")]
    Saju(String),

    #[error("vedic analysis failed: {0}")]
    Vedic(String),

    #[error("transit analysis failed: {0}")]
    Transit(String),

    #[error("compatibility analysis failed: {0}")]
    Compatibility(String),

    #[error("ai audit failed: {0}")]
    AiAudit(String),

    #[error("serialization failed: {0}")]
    Serialization(String),
}
