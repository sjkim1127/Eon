//! # Eon Data
//!
//! Provides static data and binary caches for astronomical calendar calculations.

pub mod cache;
pub mod error;
pub mod manseryuk;

pub use cache::ManseryukCache;
pub use error::DataError;
pub use manseryuk::{get_day_ganzi_index, get_solar_term_datetime, LunarCalendar};
