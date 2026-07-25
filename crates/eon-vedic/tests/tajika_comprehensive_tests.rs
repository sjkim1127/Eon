mod common;

use common::create_test_chart;
use eon_vedic::analysis::report::TajikaReport;
use eon_vedic::analysis::tajika::TajikaEngine;

#[test]
fn test_36_sahams_completeness_and_bounds() {
    let chart = create_test_chart(1990, 5, 15, 12, 37.5, 127.0);
    let sahams = TajikaEngine::calculate_all_sahams(&chart);

    assert_eq!(
        sahams.len(),
        36,
        "Should calculate exactly 36 classical Tajika Sahams"
    );

    for saham in &sahams {
        assert!(
            saham.longitude >= 0.0 && saham.longitude < 360.0,
            "Saham longitude out of bounds: {}",
            saham.longitude
        );
        assert!(
            saham.rasi >= 1 && saham.rasi <= 12,
            "Saham rasi out of bounds: {}",
            saham.rasi
        );
        assert!(
            saham.house >= 1 && saham.house <= 12,
            "Saham house out of bounds: {}",
            saham.house
        );
        assert!(!saham.name.is_empty());
        assert!(!saham.name_kr.is_empty());
        assert!(!saham.formula.is_empty());
    }
}

#[test]
fn test_mudda_dasha_total_duration_invariants() {
    let chart = create_test_chart(1990, 5, 15, 12, 37.5, 127.0);
    let mudda = TajikaEngine::calculate_mudda_dasha(&chart);

    assert_eq!(
        mudda.len(),
        9,
        "Mudda Dasha must contain all 9 Vimshottari planets"
    );

    let total_days: f64 = mudda.iter().map(|m| m.duration_days).sum();
    let expected_days = 365.25;

    assert!(
        (total_days - expected_days).abs() < 1.0,
        "Total Mudda Dasha days ({}) should match annual solar year (~365.25)",
        total_days
    );

    let last_period = mudda.last().unwrap();
    assert!((last_period.end_day_offset - expected_days).abs() < 1.0);
}

#[test]
fn test_pancha_vargeeya_bala_range() {
    let chart = create_test_chart(1990, 5, 15, 12, 37.5, 127.0);
    let bala = TajikaEngine::calculate_pancha_vargeeya_bala(&chart);

    assert_eq!(
        bala.len(),
        7,
        "Pancha Vargeeya Bala calculates 7 traditional planets"
    );

    for pvb in &bala {
        assert!(pvb.kshetra_bala >= 0.0 && pvb.kshetra_bala <= 30.0);
        assert!(pvb.uchcha_bala >= 0.0 && pvb.uchcha_bala <= 20.0);
        assert!(pvb.hadda_bala >= 0.0 && pvb.hadda_bala <= 15.0);
        assert!(pvb.drekkana_bala >= 0.0 && pvb.drekkana_bala <= 10.0);
        assert!(pvb.navamsha_bala >= 0.0 && pvb.navamsha_bala <= 10.0);
        assert!(pvb.total_virupas >= 0.0 && pvb.total_virupas <= 25.0);
        assert!(!pvb.grade.is_empty());
    }
}

#[test]
fn test_tajika_yogas_detection() {
    let chart = create_test_chart(1990, 5, 15, 12, 37.5, 127.0);
    let yogas = TajikaEngine::detect_tajika_yogas(&chart);

    assert!(
        !yogas.is_empty(),
        "Should detect at least one Tajika yoga in sample chart"
    );

    for yoga in &yogas {
        assert!(!yoga.name.is_empty());
        assert!(!yoga.name_kr.is_empty());
        assert!(!yoga.description.is_empty());
    }
}

#[test]
fn test_muntha_12_houses_progression() {
    let chart = create_test_chart(1990, 5, 15, 12, 37.5, 127.0);

    for age in 1..=12 {
        let muntha = TajikaEngine::analyze_muntha(&chart, 1, age);
        assert_eq!(muntha.muntha_rasi, (age - 1) as u8 % 12 + 1);
        assert!(!muntha.summary.is_empty());
        assert!(!muntha.details.is_empty());
    }
}

#[test]
fn test_tajika_report_generate_integration() {
    let chart = create_test_chart(1990, 5, 15, 12, 37.5, 127.0);
    let report = TajikaReport::generate(&chart, 1, 20);

    assert_eq!(report.sahams.len(), 36);
    assert_eq!(report.mudda_dasha.len(), 9);
    assert_eq!(report.pancha_vargeeya_bala.len(), 7);
    assert!(report.muntha_analysis.is_some());
    assert!(report.year_lord.is_some());
}
