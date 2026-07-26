use chrono::{DateTime, Utc};
use eon_vedic::{
    analysis::report::VedicAnalysisReport, analysis::yogas::YogaEngine, chart::VedicChartCalculator,
};

fn get_test_date() -> DateTime<Utc> {
    "1990-05-15T10:00:00Z".parse::<DateTime<Utc>>().unwrap()
}

#[test]
fn test_yoga_engine_detection() {
    let birth_time = get_test_date();
    let lat = 37.5665;
    let lon = 126.9780;

    let calc = VedicChartCalculator::new();
    let chart = calc.calculate(birth_time, lat, lon).unwrap();
    let mut yogas = YogaEngine::check_yogas(&chart);

    assert!(!yogas.is_empty(), "Should detect Vedic Yogas in chart");

    for y in &yogas {
        assert!(!y.name.is_empty());
        assert!(!y.description.is_empty());
        assert!(y.strength_percentage >= 0.0);
    }

    // Attach Dasha activations
    YogaEngine::attach_dasha_activations(&chart, &mut yogas, birth_time);

    let active_yogas: Vec<_> = yogas
        .iter()
        .filter(|y| !y.activation_periods.is_empty())
        .collect();
    assert!(
        !active_yogas.is_empty(),
        "Should attach Vimshottari Dasha activation periods to detected Yogas"
    );

    for y in active_yogas {
        for act in &y.activation_periods {
            assert!(act.start_year <= act.end_year);
            assert!(!act.period_summary.is_empty());
        }
    }
}

#[test]
fn test_vedic_analysis_report_yoga_dasha_integration() {
    let birth_time = get_test_date();
    let lat = 37.5665;
    let lon = 126.9780;

    let calc = VedicChartCalculator::new();
    let chart = calc.calculate(birth_time, lat, lon).unwrap();
    let report = VedicAnalysisReport::generate(&chart, birth_time, chart.ascendant.rasi);

    assert!(!report.yogas.is_empty());
    let yogas_with_dasha: Vec<_> = report
        .yogas
        .iter()
        .filter(|y| !y.activation_periods.is_empty())
        .collect();

    assert!(
        !yogas_with_dasha.is_empty(),
        "VedicAnalysisReport should include Dasha activation periods in yogas"
    );
}
