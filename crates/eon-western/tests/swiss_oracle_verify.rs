use chrono::{TimeZone, Utc};
use eon_western::calculate_western;

fn approx(actual: f64, expected: f64, label: &str) {
    assert!(
        (actual - expected).abs() < 1e-7,
        "{label}: actual={actual:.12}, expected={expected:.12}"
    );
}

#[test]
fn tropical_positions_and_placidus_angles_match_swiss_ephemeris() {
    // Swiss Ephemeris 2.10.3.2, Moshier, UT 1990-05-15 10:00:00,
    // geocentric tropical positions at 37.5665N, 126.9780E.
    let time = Utc.with_ymd_and_hms(1990, 5, 15, 10, 0, 0).unwrap();
    let result = calculate_western(time, 37.5665, 126.9780, 'P').unwrap();

    for (name, expected) in [
        ("Sun", 54.31576531225126),
        ("Moon", 296.13312884567046),
        ("Mercury", 38.02275135624734),
        ("Venus", 12.723269093229508),
        ("Mars", 348.27196014868935),
        ("Jupiter", 99.52593045558825),
        ("Saturn", 295.25084708158704),
        ("True Node", 310.25474801529924),
        ("Lilith", 231.46957523669772),
    ] {
        let planet = result.planets.iter().find(|p| p.name == name).unwrap();
        approx(planet.longitude, expected, name);
    }

    approx(result.ascendant, 228.43590655342115, "ASC");
    approx(result.midheaven, 147.6641891810983, "MC");
    approx(
        result.houses[0].cusp_longitude,
        228.43590655342115,
        "House 1",
    );
    approx(
        result.houses[3].cusp_longitude,
        327.6641891810983,
        "House 4",
    );
    approx(
        result.houses[9].cusp_longitude,
        147.6641891810983,
        "House 10",
    );
}
