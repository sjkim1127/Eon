use thiserror::Error;
use serde::Serialize;

#[derive(Debug, Error, Serialize, Clone, PartialEq, Eq)]
#[serde(tag = "type", content = "message")]
pub enum AiError {
    #[error("JSON error: {0}")]
    Json(String),
}

impl From<serde_json::Error> for AiError {
    fn from(e: serde_json::Error) -> Self {
        AiError::Json(e.to_string())
    }
}
