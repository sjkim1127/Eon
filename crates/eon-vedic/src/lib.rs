//! # Eon Vedic
//! 
//! `eon-vedic` is the core library for Vedic astrology calculations in the Eon ecosystem.
//! It handles the construction of Vedic charts, planetary positions using sidereal zodiac,
//! ayanamsa calculations, Vargas (divisional charts), Yogas, Dashas (planetary periods),
//! and comprehensive Vedic astrology reports.
//!
//! ## Modules
//!
//! * `analysis` - Astrological interpretation logic including Yogas, Gochara (transit), and compatibility.
//! * `calc` - Mathematical and astronomical calculations like Ayanamsa, Panchanga, and Vargas.
//! * `core` - Foundational structures such as Chart, Planets, Constants, and Configuration.
//! * `prediction` - Predictive systems like Vimshottari Dasha timelines.

pub mod analysis;
pub mod calc;
pub mod core;
pub mod prediction;

// Re-export common items for easier access
pub use analysis::yogas;
pub use calc::ayanamsa;
pub use calc::panchanga;
pub use calc::varga;
pub use core::chart;
pub use core::error::VedicError;
pub use core::config;
pub use core::constants;
pub use core::names;
pub use core::planets;
pub use prediction::dasha;
pub use prediction::kalachakra;
pub use analysis::kp;
pub use analysis::matching;



