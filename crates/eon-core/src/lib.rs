//! # Eon Core
//!
//! 모든 운명 시스템에서 공통으로 사용되는 타입과 유틸리티를 제공합니다.
//!
//! - [`Location`] - 지리적 위치 및 경도 기반 지역시 보정
//! - [`BirthInfo`] - 출생 정보 (생년월일시, 위치, 타임존, 성별)
//! - [`timezones`] - 자주 사용되는 IANA 타임존 상수
//! - [`EonError`] - 공통 에러 타입
//!
//! ## 타임존 및 DST 지원
//!
//! `chrono-tz`를 활용하여 역사적 썸머타임(DST)을 자동 처리합니다.
//! 예: 1988년 서울 올림픽 기간의 썸머타임 적용

pub mod error;
pub mod location;
pub mod birth;

pub use error::EonError;
pub use location::Location;
pub use birth::{BirthInfo, Gender, CalendarType, timezones};
