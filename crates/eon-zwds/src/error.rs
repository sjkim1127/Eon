use thiserror::Error;

#[derive(Debug, Error)]
pub enum ZwdsError {
    #[error("음력 변환 실패: {0}")]
    LunarConversion(String),

    #[error("유효하지 않은 생년월일: {0}")]
    InvalidBirthDate(String),

    #[error("유효하지 않은 시간 인덱스: {0}")]
    InvalidTimeIndex(u32),

    #[error("유효하지 않은 음력 일수: {0}")]
    InvalidLunarDay(u32),

    #[error("내부 오류: {0}")]
    Internal(String),
}
