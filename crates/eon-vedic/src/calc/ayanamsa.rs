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

    // t0=0, ayan_t0=0 for standard methods
    astro.set_sidereal_mode(method_id, 0.0, 0.0);

    // 2. Calculate
    astro.get_ayanamsa_ut(time)
}

/// Helper for default Lahiri Ayanamsa
pub fn get_lahiri_ayanamsa(astro: &AstroEngine, time: DateTime<Utc>) -> f64 {
    get_ayanamsa(astro, time, AyanamsaSystem::Lahiri)
}
