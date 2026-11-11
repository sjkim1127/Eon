#![allow(clippy::vec_init_then_push)]
//! # eon-zwds — Zi Wei Dou Shu Engine
//!
//! Pure-Rust calculation engine for traditional Chinese Zi Wei Dou Shu astrology.
//! Algorithm reference: [SylarLong/iztro](https://github.com/SylarLong/iztro) (TypeScript, MIT)
//!
//! ## Key Features
//! - 12-palace chart placement calculation
//! - 14(主星) + 6 +  ~50
//! - (四化: 化祿·化權·化科·化忌)
//! - (大限, 10 ) / (流年, 1 )
//! - (命主) / (身主)

pub mod annual;
pub mod brightness;
pub mod calendar;
pub mod chart;
pub mod collisions_zihua;
pub mod decadal;
pub mod destiny_patterns;
pub mod error;
pub mod monthly_daily;
pub mod palace;
pub mod stars;
pub mod transformations;
pub mod types;

pub use chart::build_chart;
pub use error::ZwdsError;
pub use types::*;
