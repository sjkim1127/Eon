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

fn natal() -> WesternAnalysisInput {
    WesternAnalysisInput::new(person_a(), "Placidus".into())
}

#[test]
fn western_transit_overlays_the_sky_on_natal_houses() {
    let target = Utc.with_ymd_and_hms(2026, 11, 20, 0, 0, 0).unwrap();
    let out = facade::analyze_western_transit(natal(), target).expect("transit");

    assert_eq!(out.result.target_time, target);
    assert!(out.result.planets.iter().any(|p| p.name == "Saturn"));
    assert!(out
        .result
        .planets
        .iter()
        .all(|p| (1..=12).contains(&p.natal_house)));
    assert!(out
        .result
        .aspects
        .iter()
        .all(|a| a.body_a_name.starts_with("Transit ") && a.body_b_name.starts_with("Natal ")));
}

#[test]
fn western_returns_are_found_and_validated() {
    let solar = facade::analyze_western_solar_return(natal(), 2027, None).expect("solar return");
    // Born 1985-11-27: the 2027 return is at the end of November 2027.
    assert_eq!(
        solar.result.exact_time.format("%Y-%m").to_string(),
        "2027-11"
    );

    // Relocating changes the angles but not the moment.
    let relocated =
        facade::analyze_western_solar_return(natal(), 2027, Some((40.7128, -74.006))).unwrap();
    assert_eq!(relocated.result.exact_time, solar.result.exact_time);
    assert!(
        (relocated.result.return_chart.ascendant - solar.result.return_chart.ascendant).abs() > 1.0
    );

    assert!(matches!(
        facade::analyze_western_solar_return(natal(), 1985, None),
        Err(ServiceError::InvalidInput(_))
    ));

    let after = Utc.with_ymd_and_hms(2026, 11, 20, 0, 0, 0).unwrap();
    let lunar = facade::analyze_western_lunar_return(natal(), after, None).expect("lunar return");
    assert!(lunar.result.exact_time >= after);
    assert!((lunar.result.exact_time - after).num_days() <= 28);
}

#[test]
fn western_solar_arc_and_profection() {
    let target = Utc.with_ymd_and_hms(2026, 11, 20, 0, 0, 0).unwrap();
    let arc = facade::analyze_western_solar_arc(natal(), target).expect("solar arc");
    assert!(
        (arc.result.arc_degrees - 41.0).abs() < 2.0,
        "{}",
        arc.result.arc_degrees
    );
    assert!(arc.result.directed.iter().any(|d| d.name == "Ascendant"));

    let profection = facade::analyze_western_profection(natal(), target).expect("profection");
    assert_eq!(profection.result.age, 40);
    assert_eq!(profection.result.profected_house, 5); // 40 mod 12 = 4 -> 5th house
    assert!(!profection.result.year_lord.is_empty());

    let before_birth = Utc.with_ymd_and_hms(1980, 1, 1, 0, 0, 0).unwrap();
    assert!(facade::analyze_western_solar_arc(natal(), before_birth).is_err());
    assert!(facade::analyze_western_profection(natal(), before_birth).is_err());
}

#[test]
fn natal_chart_carries_antiscia_receptions_and_midpoints() {
    let chart = facade::analyze_western(natal()).unwrap().result;
    // These are computed for every chart, so they must be well formed.
    assert!(chart.midpoints.windows(2).all(|w| w[0].orb <= w[1].orb));
    assert!(chart.antiscia.iter().all(|c| c.orb <= 1.0));
    assert!(chart.midpoints.iter().all(|c| c.orb <= 1.0));
    for r in &chart.mutual_receptions {
        assert_ne!(r.planet_a, r.planet_b);
    }
}

#[test]
fn qimen_report_carries_insights_and_question_readings() {
    use eon_qimen::analysis::question::QimenQuestion;

    let input = QimenAnalysisInput::new(person_a(), true);
    let plain = facade::analyze_qimen(input.clone()).expect("qimen");
    let insights = &plain.report.insights;
    assert_eq!(insights.palaces.len(), 9);
    assert!(insights.horse_palace.is_some());
    assert_eq!(insights.hour_void.branches.len(), 2);
    assert!(insights.self_palace.is_some());
    assert!(insights.palaces.iter().all(|p| p.patterns.len() <= 6));

    let out = facade::analyze_qimen_question(input.clone(), QimenQuestion::Wealth).expect("wealth");
    assert_eq!(out.reading.question, QimenQuestion::Wealth);
    assert_eq!(out.report.pan.ju_number, plain.report.pan.ju_number);
    assert!(!out.reading.advice_ko.is_empty());

    // The same moment always gives the same plate and reading.
    let again = facade::analyze_qimen_question(input, QimenQuestion::Wealth).unwrap();
    assert_eq!(out.reading, again.reading);
    assert_eq!(out.report.insights, again.report.insights);
}

#[test]
fn qimen_still_needs_a_known_birth_time() {
    let mut date_only = person_a();
    date_only.unknown_time = true;
    assert!(matches!(
        facade::analyze_qimen(QimenAnalysisInput::new(date_only, true)),
        Err(ServiceError::BirthTimeRequired(_))
    ));
}
