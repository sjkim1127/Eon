use crate::common::VedicTestFixture;
use eon_vedic::planets::VedicPlanet;
use eon_vedic::config::AyanamsaSystem;

pub const FIXTURES: &[VedicTestFixture] = &[
    VedicTestFixture {
        name: "B.V. Raman",
        year: 1912,
        month: 8,
        day: 8,
        hour: 14,
        minute: 13,
        latitude: 13.0,
        longitude: 77.5,
        ayanamsa: AyanamsaSystem::Raman,
        expected_ak: VedicPlanet::Moon,
        expected_al_rasi: 9,
        expected_navamsa_lagna: 10,
        expected_dasamsa_lagna: 2,
        expected_current_yogini: "Sankata",
    },
    VedicTestFixture {
        name: "Amitabh Bachchan",
        year: 1942,
        month: 10,
        day: 11,
        hour: 10,
        minute: 30,
        latitude: 25.45,
        longitude: 81.85,
        ayanamsa: AyanamsaSystem::Lahiri,
        expected_ak: VedicPlanet::Sun,
        expected_al_rasi: 9,
        expected_navamsa_lagna: 1,
        expected_dasamsa_lagna: 6,
        expected_current_yogini: "Pingala",
    }
];
