use chrono::{DateTime, Utc};
use eon_western::{
    calculate_composite, calculate_secondary_progression, calculate_synastry, calculate_western,
    AspectDynamics,
};

fn get_test_date(year: i32, month: u32, day: u32, hour: u32) -> DateTime<Utc> {
    format!("{:04}-{:02}-{:02}T{:02}:00:00Z", year, month, day, hour)
        .parse::<DateTime<Utc>>()
        .unwrap()
}

#[test]
fn test_western_minor_aspects_and_dynamics() {
    let birth_date = get_test_date(1990, 5, 15, 10);
    let lat = 37.5665;
    let lon = 126.9780;

    let res = calculate_western(birth_date, lat, lon, 'P').unwrap();

    // Verify minor aspects exist and dynamics (Applying/Separating/Exact) are populated
    let minor_aspects: Vec<_> = res.aspects.iter().filter(|a| !a.is_major).collect();
    assert!(!minor_aspects.is_empty(), "Should calculate minor aspects");

    for asp in &res.aspects {
        assert!(asp.orb <= asp.aspect_type.standard_orb());
        assert!(matches!(
            asp.dynamics,
            AspectDynamics::Applying | AspectDynamics::Separating | AspectDynamics::Exact
        ));
    }
}

#[test]
fn test_western_essential_dignities_scoring() {
    let birth_date = get_test_date(1990, 5, 15, 10);
    let lat = 37.5665;
    let lon = 126.9780;

    let res = calculate_western(birth_date, lat, lon, 'P').unwrap();

    assert!(!res.dignities.is_empty());
    for d in &res.dignities {
        assert!(!d.planet_name.is_empty());
        assert!(!d.status_summary.is_empty());
    }
}

#[test]
fn test_western_house_rulership_network() {
    let birth_date = get_test_date(1990, 5, 15, 10);
    let lat = 37.5665;
    let lon = 126.9780;

    let res = calculate_western(birth_date, lat, lon, 'P').unwrap();

    assert_eq!(res.house_rulerships.len(), 12);
    for hr in &res.house_rulerships {
        assert!(hr.house_number >= 1 && hr.house_number <= 12);
        assert!(hr.ruler_in_house >= 1 && hr.ruler_in_house <= 12);
        assert!(!hr.ruler_planet.is_empty());
        assert!(!hr.interpretation.is_empty());
    }
}

#[test]
fn test_western_arabian_parts() {
    let birth_date = get_test_date(1990, 5, 15, 10);
    let lat = 37.5665;
    let lon = 126.9780;

    let res = calculate_western(birth_date, lat, lon, 'P').unwrap();

    assert_eq!(res.arabian_parts.len(), 3);
    for part in &res.arabian_parts {
        assert!(part.longitude >= 0.0 && part.longitude < 360.0);
        assert!(part.sign_index < 12);
        assert!(part.house_number >= 1 && part.house_number <= 12);
        assert!(!part.formula.is_empty());
    }
}

#[test]
fn test_western_synastry_composite_progression() {
    let date_a = get_test_date(1990, 5, 15, 10);
    let date_b = get_test_date(1992, 8, 20, 14);
    let lat = 37.5665;
    let lon = 126.9780;

    let res_a = calculate_western(date_a, lat, lon, 'P').unwrap();
    let res_b = calculate_western(date_b, lat, lon, 'P').unwrap();

    // 1. Synastry test
    let syn = calculate_synastry(&res_a, &res_b, "Person A", "Person B");
    assert!(syn.harmony_score >= 0.0 && syn.harmony_score <= 100.0);
    assert!(!syn.summary.is_empty());

    // 2. Composite test
    let comp = calculate_composite(&res_a, &res_b).unwrap();
    assert_eq!(comp.composite_chart.planets.len(), 18);
    assert_eq!(comp.composite_chart.houses.len(), 12);

    // 3. Progression test (Age ~ 30 years later)
    let target_date = get_test_date(2020, 5, 15, 10);
    let prog = calculate_secondary_progression(date_a, lat, lon, target_date, 'P').unwrap();
    assert!((prog.progressed_age_years - 30.0).abs() < 0.5);
    assert_eq!(prog.progressed_chart.planets.len(), 18);
}
