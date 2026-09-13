use chrono::{TimeZone, Utc};
use eon_vedic::config::VedicYearType;
use eon_vedic::planets::VedicPlanet;
use eon_vedic::prediction::dasha::Vimshottari;

/// Reference input: Moon longitude from Swiss Ephemeris 2.10.3.2 with Lahiri
/// ayanamsa at 2024-04-15 12:00 UTC (88.3618787076 deg sidereal).
#[test]
fn vimshottari_birth_balance_matches_independent_reference() {
    let birth = Utc.with_ymd_and_hms(2024, 4, 15, 12, 0, 0).unwrap();
    let timeline = Vimshottari::calculate(88.36187870759836, birth, 1, VedicYearType::Gregorian);

    let first = &timeline[0];
    assert_eq!(first.planet, VedicPlanet::Jupiter);
    assert!((first.duration_years - 5.9657455509).abs() < 1e-9);
    assert_eq!(first.start_date, birth);

    let expected_seconds = (5.9657455509 * 365.2425 * 86_400.0) as i64;
    let actual_seconds = first
        .end_date
        .signed_duration_since(first.start_date)
        .num_seconds();
    assert!((actual_seconds - expected_seconds).abs() <= 1);
}
