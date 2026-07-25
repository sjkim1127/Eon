mod common;

use chrono::Utc;
use common::create_test_chart;
use eon_vedic::analysis::jaimini::JaiminiEngine;
use eon_vedic::analysis::report::VedicAnalysisReport;

#[test]
fn test_chara_dasha_2_tier_sub_periods() {
    let chart = create_test_chart(1990, 5, 15, 12, 37.5, 127.0);
    let chara_dasha = JaiminiEngine::calculate_chara_dasha(&chart);

    assert_eq!(
        chara_dasha.len(),
        12,
        "Chara Dasha should have 12 Mahadashas"
    );

    for dasha in &chara_dasha {
        assert!(
            dasha.rasi >= 1 && dasha.rasi <= 12,
            "Mahadasha rasi out of bounds: {}",
            dasha.rasi
        );
        assert!(dasha.years >= 1 && dasha.years <= 12);
        assert_eq!(
            dasha.sub_periods.len(),
            12,
            "Each Mahadasha must have 12 Antardasha sub-periods"
        );

        for sub in &dasha.sub_periods {
            assert!(
                sub.rasi >= 1 && sub.rasi <= 12,
                "Antardasha rasi out of bounds: {}",
                sub.rasi
            );
            assert!(sub.duration_days > 0.0);
            assert!(sub.start_time < sub.end_time);
        }
    }
}

#[test]
fn test_karakamsha_and_ishta_devata_calculation() {
    let chart = create_test_chart(1990, 5, 15, 12, 37.5, 127.0);
    let karakamsha = JaiminiEngine::analyze_karakamsha(&chart);

    assert!(
        karakamsha.karakamsha_rasi >= 1 && karakamsha.karakamsha_rasi <= 12,
        "Karakamsha rasi out of bounds: {}",
        karakamsha.karakamsha_rasi
    );
    assert!(karakamsha.ishta_devata_planet.is_some());
    assert!(!karakamsha.ishta_devata_deity.is_empty());
    assert!(!karakamsha.spiritual_summary.is_empty());
    assert!(!karakamsha.career_talent_summary.is_empty());
}

#[test]
fn test_arudha_padas_12_houses() {
    let chart = create_test_chart(1990, 5, 15, 12, 37.5, 127.0);
    let arudha_padas = JaiminiEngine::analyze_arudha_padas(&chart);

    assert_eq!(
        arudha_padas.len(),
        12,
        "Should calculate Arudha Padas for all 12 houses"
    );

    for (idx, ap) in arudha_padas.iter().enumerate() {
        assert_eq!(ap.house, (idx + 1) as u8);
        assert!(
            ap.rasi >= 1 && ap.rasi <= 12,
            "Arudha rasi out of bounds: {}",
            ap.rasi
        );
        assert!(!ap.name.is_empty());
        assert!(!ap.interpretation.is_empty());
    }

    let al = arudha_padas.iter().find(|p| p.house == 1).unwrap();
    assert!(al.interpretation.contains("아루다 라그나(AL)"));

    let ul = arudha_padas.iter().find(|p| p.house == 12).unwrap();
    assert!(ul.interpretation.contains("우파파다 라그나(UL/A12)"));
}

#[test]
fn test_argala_and_virodhargala_matrix() {
    let chart = create_test_chart(1990, 5, 15, 12, 37.5, 127.0);
    let argala_matrix = JaiminiEngine::analyze_argala(&chart);

    assert_eq!(
        argala_matrix.len(),
        12,
        "Argala matrix should cover all 12 rasis"
    );

    for (idx, arg) in argala_matrix.iter().enumerate() {
        assert_eq!(arg.rasi, (idx + 1) as u8);
        assert!(arg.primary_argala_score >= 0.0);
        assert!(arg.virodhargala_score >= 0.0);
        assert!(!arg.status.is_empty());
    }
}

#[test]
fn test_jaimini_report_integration() {
    let chart = create_test_chart(1990, 5, 15, 12, 37.5, 127.0);
    let now = Utc::now();
    let report = VedicAnalysisReport::generate(&chart, now, 1);

    assert!(report.jaimini_report.is_some());
    let j_report = report.jaimini_report.as_ref().unwrap();

    assert_eq!(j_report.chara_dasha.len(), 12);
    assert_eq!(j_report.arudha_padas.len(), 12);
    assert_eq!(j_report.argala_matrix.len(), 12);
    assert!(!j_report.karakamsha_analysis.ishta_devata_deity.is_empty());
}
