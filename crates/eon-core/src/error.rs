//! Eon 공통 에러 타입

use serde::Serialize;
use thiserror::Error;

/// Eon 플랫폼 공통 에러 타입
#[derive(Debug, Error, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum EonError {
    /// 잘못된 날짜/시간
    #[error("Invalid datetime: {0}")]
    InvalidDateTime(String),

    /// 잘못된 인덱스
    #[error("Invalid index: {0}")]
    InvalidIndex(i32),

    /// 데이터 조회 실패
    #[error("Data not found: {0}")]
    DataNotFound(String),

    /// 계산 오류
    #[error("Calculation error: {0}")]
    CalculationError(String),
}

/// Core module errors
#[derive(Debug, thiserror::Error, serde::Serialize, Clone, PartialEq, Eq)]
#[serde(tag = "type", content = "message")]
pub enum CoreError {
    /// 잘못된 날짜/시간
    #[error("Invalid date or time parameters")]
    InvalidDateTime,

    /// 로컬 시간이 대상 타임존에 존재하지 않음
    #[error("Local date/time is non-existent in the target timezone")]
    NonExistentLocalTime,

    /// 잘못된 타임존 형식
    #[error("Invalid timezone format: {0}")]
    InvalidTimezone(String),
}
