//! Features that exist in the engines and are exposed through the facade for the CLI and MCP.

use crate::dto::*;
use crate::error::ServiceError;
use crate::facade;
use crate::services::western::parse_house_system;
use chrono::{TimeZone, Utc};

fn person_a() -> AnalysisInput {
    AnalysisInput::new(1985, 11, 27, 14, 30, "Asia/Seoul", 37.5665, 126.9780)
}

fn person_b() -> AnalysisInput {
    AnalysisInput::new(1990, 5, 15, 10, 30, "Asia/Seoul", 37.5665, 126.9780)
}

#[test]
fn house_system_names_and_letters_resolve() {
    assert_eq!(parse_house_system("Placidus").unwrap(), 'P');
    assert_eq!(parse_house_system("koch").unwrap(), 'K');
    assert_eq!(parse_house_system("Whole Sign").unwrap(), 'W');
    assert_eq!(parse_house_system("whole-sign").unwrap(), 'W');
    assert_eq!(parse_house_system("Equal").unwrap(), 'E');
    assert_eq!(parse_house_system("Porphyry").unwrap(), 'O');
    assert_eq!(parse_house_system("Regiomontanus").unwrap(), 'R');
    assert_eq!(parse_house_system("Campanus").unwrap(), 'C');
    // Legacy single-letter spelling keeps working.
    assert_eq!(parse_house_system("P").unwrap(), 'P');
    assert_eq!(parse_house_system("k").unwrap(), 'K');
}

#[test]
fn unknown_house_system_is_rejected_instead_of_falling_back() {
    // "Porphyry" used to become 'P' (Placidus) because only the first character was read.
    for bad in ["", "Bogus", "Pomegranate"] {
        match parse_house_system(bad) {
            Err(ServiceError::InvalidInput(msg)) => assert!(msg.contains("house system")),
            other => panic!("{bad:?} must be rejected, got {other:?}"),
        }
    }
    let res = facade::analyze_western(WesternAnalysisInput::new(person_a(), "Nope".to_string()));
    assert!(matches!(res, Err(ServiceError::InvalidInput(_))));
}

#[test]
fn porphyry_differs_from_placidus() {
    // Far from the equator the two systems place the intermediate cusps differently.
    let birth = AnalysisInput::new(1985, 11, 27, 14, 30, "Europe/Oslo", 59.9139, 10.7522);
    let placidus =
        facade::analyze_western(WesternAnalysisInput::new(birth.clone(), "Placidus".into()))
            .unwrap();
    let porphyry =
        facade::analyze_western(WesternAnalysisInput::new(birth, "Porphyry".into())).unwrap();

    let cusp = |r: &WesternAnalysisOutput, house: usize| {
        r.result
            .houses
            .iter()
            .find(|h| h.house_number == house)
            .unwrap()
            .cusp_longitude
    };
    // Angles agree, the intermediate cusps do not.
    assert!((cusp(&placidus, 1) - cusp(&porphyry, 1)).abs() < 1e-6);
    assert!((cusp(&placidus, 11) - cusp(&porphyry, 11)).abs() > 0.01);
}

#[test]
fn western_synastry_and_composite() {
    let out = facade::analyze_western_synastry(
        WesternAnalysisInput::new(person_a(), "Placidus".into()),
        WesternAnalysisInput::new(person_b(), "Placidus".into()),
        "A",
        "B",
    )
    .expect("synastry");

    assert_eq!(out.synastry.person_a_name, "A");
    assert_eq!(out.synastry.person_b_name, "B");
    assert!(!out.synastry.inter_aspects.is_empty());
    assert!((0.0..=100.0).contains(&out.synastry.harmony_score));
    assert!(!out.composite.composite_chart.planets.is_empty());
}

#[test]
fn western_progression_runs_and_rejects_dates_before_birth() {
    let input = WesternAnalysisInput::new(person_a(), "Placidus".into());
    let target = Utc.with_ymd_and_hms(2026, 6, 1, 0, 0, 0).unwrap();
    let out = facade::analyze_western_progression(input.clone(), target).expect("progression");

    assert!((out.result.progressed_age_years - 40.5).abs() < 0.2);
    assert_eq!(
        out.result.progressed_chart.planets.len(),
        out.result.birth_chart.planets.len()
    );

    let before_birth = Utc.with_ymd_and_hms(1980, 1, 1, 0, 0, 0).unwrap();
    assert!(matches!(
        facade::analyze_western_progression(input, before_birth),
        Err(ServiceError::InvalidInput(_))
    ));
}

#[test]
fn vedic_ayanamsa_is_selectable_and_validated() {
    let now = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let run = |ayanamsa: Option<&str>| {
        let mut input = VedicAnalysisInput::new(person_a(), Some(false), Some(now));
        input.ayanamsa = ayanamsa.map(str::to_string);
        facade::analyze_vedic(input)
    };

    let lahiri = run(None).expect("default");
    let explicit = run(Some("Lahiri")).expect("explicit Lahiri");
    let raman = run(Some("raman")).expect("Raman");

    assert!((lahiri.chart.ayanamsa - explicit.chart.ayanamsa).abs() < 1e-9);
    // Raman's ayanamsa is about 1.4 degrees smaller than Lahiri's.
    assert!((lahiri.chart.ayanamsa - raman.chart.ayanamsa).abs() > 1.0);

    assert!(matches!(
        run(Some("Bogus")),
        Err(ServiceError::InvalidInput(_))
    ));
}

#[test]
fn saju_compatibility_audit_is_bounded() {
    let a = SajuAnalysisInput::new(person_a(), true, false, None);
    let b = SajuAnalysisInput::new(person_b(), false, false, None);
    let out = facade::analyze_saju_compatibility(a, b).expect("saju compatibility");

    assert!((0.0..=100.0).contains(&out.audit.sync_score));
    assert!(
        !out.audit.synergies.is_empty()
            || !out.audit.conflicts.is_empty()
            || !out.audit.deadlocks.is_empty()
            || !out.audit.merged_esil_trace.is_empty()
    );
}
