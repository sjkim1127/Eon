//! # Eon Core
//!
//! 모든 운명 시스템에서 공통으로 사용되는 타입과 유틸리티를 제공합니다.
//!
//! - [`Location`] - 지리적 위치 및 지역시 보정
//! - [`BirthInfo`] - 출생 정보 (생년월일시, 위치, 성별)
//! - [`EonError`] - 공통 에러 타입

pub mod error;
pub mod location;
pub mod birth;

pub use error::EonError;
pub use location::Location;
pub use birth::{BirthInfo, Gender, CalendarType};
