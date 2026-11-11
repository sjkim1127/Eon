//! # Eon Core
//!
//! Provides common primitives, types, and utilities used across all destiny engines.
//!
//! - [`Location`] - Geographical coordinates and longitude-based local solar time correction
//! - [`BirthInfo`] - Comprehensive birth data (datetime, location, timezone, gender)
//! - [`timezones`] - Commonly used IANA timezone constants
//! - [`EonError`] - Common error types
//!
//! ## Timezone & Historical DST Support
//!
//! Employs `chrono-tz` to automatically handle historical Daylight Saving Time (DST) changes
//! (e.g., Korean DST during the 1988 Seoul Olympics).

pub mod birth;
pub mod error;
pub mod location;
pub mod timezone;

pub use birth::{timezones, BirthInfo, CalendarType, Gender};
pub use error::{CoreError, EonError};
pub use location::{lookup_city, CityInfo, Location, BUILTIN_CITIES};
pub use timezone::standard_meridian_from_tz;
