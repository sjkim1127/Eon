//! Behaviour of every engine when only the birth date is known.

use crate::dto::*;
use crate::error::ServiceError;
use crate::facade;

fn date_only() -> AnalysisInput {
    AnalysisInput::new(1990, 5, 15, 12, 0, "Asia/Seoul", 37.5665, 126.978).with_unknown_time(true)
}

#[test]
fn saju_runs_with_three_pillars_and_unknown_precision() {
    let input = SajuAnalysisInput::new(date_only(), true, false, None);
    assert_eq!(input.precision, BirthTimePrecision::UnknownTimeNoonProxy);

    let out = facade::analyze_saju(input).expect("three-pillar Saju must succeed");
    let p = &out.report.pillars;
    assert!(p.hour.is_none());
    assert!(out.report.strength.deuk_si.is_none());
    assert!(p.supplementary_pillars.myeonggung.is_none());
}

#[test]
fn time_dependent_engines_refuse_without_birth_time() {
    let base = date_only();
    assert!(matches!(
        facade::analyze_vedic(VedicAnalysisInput::new(base.clone(), None, None)),
        Err(ServiceError::BirthTimeRequired(_))
    ));
    assert!(matches!(
        facade::analyze_western(WesternAnalysisInput::new(base.clone(), "Placidus".into())),
        Err(ServiceError::BirthTimeRequired(_))
    ));
    assert!(matches!(
        facade::analyze_zwds(ZwdsAnalysisInput::new(base.clone(), true, None)),
        Err(ServiceError::BirthTimeRequired(_))
    ));
    assert!(matches!(
        facade::analyze_qimen(QimenAnalysisInput::new(base.clone(), true)),
        Err(ServiceError::BirthTimeRequired(_))
    ));
    assert!(matches!(
        facade::analyze_human_design(HumanDesignAnalysisInput::new(base.clone())),
        Err(ServiceError::BirthTimeRequired(_))
    ));
    assert!(matches!(
        facade::analyze_iching(SajuAnalysisInput::new(base, true, false, None)),
        Err(ServiceError::BirthTimeRequired(_))
    ));
}

#[test]
fn omni_analysis_reports_skipped_engines_and_renormalises_tier() {
    let omni = facade::analyze_all_synthesized(date_only(), true, Some("John Smith".into()))
        .expect("omni analysis must degrade gracefully");

    assert!(omni.vedic.is_none());
    assert!(omni.western.is_none());
    assert!(omni.zwds.is_none());
    assert!(omni.human_design.is_none());
    assert!(omni.qimen.is_none());
    assert!(
        omni.numerology.is_some(),
        "numerology does not need the birth time"
    );
    assert!(omni.tier.vedic_result.is_none());

    let skipped: Vec<&str> = omni
        .skipped_engines
        .iter()
        .map(|s| s.engine.as_str())
        .collect();
    for engine in [
        "Vedic astrology",
        "Western astrology",
        "Zi Wei Dou Shu",
        "Human Design",
        "QiMen DunJia",
    ] {
        assert!(
            skipped.contains(&engine),
            "{engine} should be reported as skipped"
        );
    }

    let total_weight: f32 = omni.tier.detailed_components.iter().map(|c| c.weight).sum();
    assert!(
        (total_weight - 1.0).abs() < 1e-3,
        "active weights must sum to 1.0"
    );
    assert!(omni
        .tier
        .detailed_components
        .iter()
        .all(|c| !c.key.starts_with("vedic_") || c.weight == 0.0));
}

#[test]
fn known_time_is_unaffected() {
    let known = AnalysisInput::new(1990, 5, 15, 12, 0, "Asia/Seoul", 37.5665, 126.978);
    let omni = facade::analyze_all_synthesized(known, true, None).unwrap();
    assert!(omni.vedic.is_some());
    assert!(omni.skipped_engines.is_empty());
    assert!(omni.saju.report.pillars.hour.is_some());
}
