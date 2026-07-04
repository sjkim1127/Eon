//! Varga Nakshatra Report integration test.
//! Verifies D1/D9/D10/D108 nakshatra reports for a known chart.

use chrono::{TimeZone, Utc};
use eon_vedic::analysis::varga_nakshatra_report::build_varga_nakshatra_reports;
use eon_vedic::chart::VedicChartCalculator;
use eon_vedic::core::config::{AyanamsaSystem, NodeCalculation, VedicConfig};

#[test]
fn test_varga_nakshatra_report_d1_ansan() {
    // 27 Nov 2004 22:00 KST (13:00 UTC), Ansan-si (37.3167, 126.8167)
    let time = Utc.with_ymd_and_hms(2004, 11, 27, 13, 0, 0).unwrap();
    let config = VedicConfig {
        ayanamsa: AyanamsaSystem::Lahiri,
        node_calc: NodeCalculation::MeanNode,
        ..Default::default()
    };
    let calculator = VedicChartCalculator::with_config(config);
    let chart = calculator.calculate(time, 37.3167, 126.8167).unwrap();

    let reports = build_varga_nakshatra_reports(&chart);

    // reports map should contain all 22 vargas
    let expected_keys: [&str; 22] = [
        "rasi",
        "hora",
        "drekkana",
        "chaturthamsha",
        "panchamsa",
        "saptamsa",
        "ashtamsa",
        "navamsa",
        "dasamsa",
        "rudramsa",
        "dwadasamsa",
        "shodashamsa",
        "vimsamsa",
        "chaturvimshamsa",
        "saptavimsamsa",
        "trimsamsa",
        "khavedamsa",
        "akshavedamsa",
        "shashtyamsa",
        "navanavamsa",
        "ashtottaramsa",
        "dwadasdwadasamsa",
    ];
    for key in &expected_keys {
        assert!(
            reports.reports.contains_key(*key),
            "reports should contain key: {}",
            key
        );
    }

    let d1 = &reports.reports["rasi"];
    // D1 checks - known values from user verification
    assert_eq!(d1.lagna_rasi, 4, "Lagna Cancer");
    assert_eq!(d1.rows.len(), 13, "9 planets + ASC + IC + DSC + MC");

    let sun_row = d1.rows.iter().find(|r| r.planet == "Sun").unwrap();
    assert_eq!(sun_row.nakshatra_name, "Anuradha");
    assert_eq!(sun_row.pada, 3);
    assert_eq!(sun_row.sign, 8, "Sun in Scorpio");

    let moon_row = d1.rows.iter().find(|r| r.planet == "Moon").unwrap();
    assert_eq!(moon_row.nakshatra_name, "Rohini");
    assert_eq!(moon_row.pada, 3);
    assert_eq!(moon_row.sign, 2, "Moon in Taurus");

    let asc_row = d1.rows.iter().find(|r| r.planet == "ASC").unwrap();
    assert_eq!(asc_row.nakshatra_name, "Pushya");
    assert_eq!(asc_row.pada, 4);
    assert_eq!(asc_row.sign, 4, "ASC Cancer");

    // D9/D10/D108 should have same row count
    let d9 = &reports.reports["navamsa"];
    let d10 = &reports.reports["dasamsa"];
    let d108 = &reports.reports["ashtottaramsa"];
    assert_eq!(d9.rows.len(), 13);
    assert_eq!(d10.rows.len(), 13);
    assert_eq!(d108.rows.len(), 13);

    // D108: Lagna Taurus (from user's D108 table)
    assert_eq!(d108.lagna_rasi, 2, "D108 Lagna Taurus");
}
