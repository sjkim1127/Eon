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

    for (name, expected_longitude, expected_speed) in [
        ("Sun", 54.31576531225126, 0.9642460691081437),
        ("Moon", 296.13312884567046, 12.335417934648275),
        ("Mercury", 38.02275135624734, -0.1265739875170262),
        ("Venus", 12.723269093229508, 1.1405030101364602),
        ("Mars", 348.27196014868935, 0.7427464005365598),
        ("Jupiter", 99.52593045558825, 0.1890889768004081),
        ("Saturn", 295.25084708158704, -0.01676909954981326),
        ("True Node", 310.25474801529924, -0.02479813056294504),
        ("Lilith", 231.46957523669772, 0.11204669788820393),
    ] {
        let planet = result.planets.iter().find(|p| p.name == name).unwrap();
        approx(planet.longitude, expected_longitude, name);
        approx(planet.speed, expected_speed, &format!("{name} speed"));
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
