//! Eon Vedic: Sidereal Astrology Calculation Engine
//!
//! Provides calculations based on the Sidereal Zodiac (Lahiri Ayanamsa by default).

pub mod ayanamsa;
pub mod planets;
pub mod chart;
pub mod names;
pub mod config;
pub mod varga;
pub mod dasha;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ZodiacSystem {
    Tropical,
    Sidereal,
}

pub use config::AyanamsaSystem as AyanamsaMethod;
