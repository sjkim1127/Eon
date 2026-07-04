//! # Eon Data
//!
//! 만세력 등 운명 계산에 필요한 정적 데이터를 제공합니다.

pub mod cache;
pub mod error;
pub mod manseryuk;

pub use cache::ManseryukCache;
pub use error::DataError;
pub use manseryuk::{get_day_ganzi_index, get_solar_term_datetime, LunarCalendar};
