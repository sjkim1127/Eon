use chrono::{TimeZone, Utc};
use eon_western::calculate_secondary_progression;

#[test]
fn secondary_progression_uses_one_day_per_tropical_year() {
    let birth = Utc.with_ymd_and_hms(1990, 5, 15, 10, 0, 0).unwrap();
    let target = Utc.with_ymd_and_hms(2020, 5, 15, 10, 0, 0).unwrap();
    let result = calculate_secondary_progression(birth, 37.5665, 126.978, target, 'P').unwrap();

    // The 30-year progression date is 1990-06-14 10:02:51 UTC. These are
    // Swiss Ephemeris 2.10.3.2 Moshier tropical longitudes at that instant.
    for (name, expected) in [
        ("Sun", 83.09670719497656),
        ("Moon", 331.10875165475085),
        ("Mercury", 63.862066327947936),
    ] {
        let actual = result
            .progressed_chart
            .planets
            .iter()
            .find(|planet| planet.name == name)
            .unwrap()
            .longitude;
        assert!(
            // The implementation stores the progressed instant at whole-second
            // precision, so allow the corresponding sub-arcsecond longitude
            // difference.
            (actual - expected).abs() < 1e-4,
            "{name}: actual={actual:.12}, expected={expected:.12}"
        );
    }
    assert!((result.progressed_age_years - 30.001984982580066).abs() < 1e-12);
}
