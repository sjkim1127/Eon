use crate::core::config::AyanamsaSystem;
use chrono::{DateTime, Utc};
use eon_astro::AstroEngine;

/// Calculate Ayanamsa (Precession correction) using Swiss Ephemeris
pub fn get_ayanamsa(astro: &AstroEngine, time: DateTime<Utc>, method: AyanamsaSystem) -> f64 {
    // 1. Set Sidereal Mode based on config
    let method_id = match method {
        AyanamsaSystem::Lahiri => 1,       // SE_SIDM_LAHIRI
        AyanamsaSystem::FaganBradley => 0, // SE_SIDM_FAGAN_BRADLEY
        AyanamsaSystem::Raman => 3,        // SE_SIDM_RAMAN
        AyanamsaSystem::Krishnamurti => 5, // SE_SIDM_KRISHNAMURTI
    };

    // Keep mode selection and calculation in one critical section because
    // Swiss Ephemeris stores the sidereal mode process-wide.
    astro.get_ayanamsa_ut_for_mode(time, method_id, 0.0, 0.0)
}

/// Helper for default Lahiri Ayanamsa
pub fn get_lahiri_ayanamsa(astro: &AstroEngine, time: DateTime<Utc>) -> f64 {
    get_ayanamsa(astro, time, AyanamsaSystem::Lahiri)
}
