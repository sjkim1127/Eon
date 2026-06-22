use thiserror::Error;
use serde::Serialize;
use ts_rs::TS;

#[derive(Debug, Error, Serialize, TS)]
#[serde(tag = "type", content = "message", rename_all = "camelCase")]
#[ts(export)]
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

    #[error("zwds analysis failed: {0}")]
    Zwds(String),

    #[error("serialization failed: {0}")]
    Serialization(String),
}
