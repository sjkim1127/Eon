#[cfg(test)]
mod tests {
    use crate::dto::*;
    use crate::facade;
    use chrono::{Utc, TimeZone};

    #[test]
    fn test_unified_analysis_parity() {
        let (_birth, saju_input, transit_input) = setup_test_inputs();

        // 1. Saju
        let saju_res = facade::analyze_saju(saju_input.clone()).expect("Saju analysis failed");
        
        // Verify precise Saju calculation outputs (TST + DST consideration)
        assert_eq!(format!("{:?}", saju_res.report.pillars.year.stem), "Yi");
        assert_eq!(format!("{:?}", saju_res.report.pillars.year.branch), "Hai");
        assert_eq!(format!("{:?}", saju_res.report.pillars.day.stem), "Ren");
        assert_eq!(format!("{:?}", saju_res.report.strength.day_master), "Ren");
        
        assert!(saju_res.meta.corrected_time.contains(":"), "Corrected time should be present");
        
        // 2. Transit
        let transit_res = facade::analyze_transit(transit_input).expect("Transit analysis failed");
        assert_eq!(transit_res.meta.analysis_timezone, "Asia/Seoul");
        assert!(transit_res.monthly_lucks.len() > 0, "Should have multiple monthly lucks");

        // 3. AI Audit
        let ai_audit_res = facade::analyze_ai_audit(saju_input).expect("AI Audit failed");
        assert_eq!(ai_audit_res.meta.precision, BirthTimePrecision::Exact);
        assert!(!ai_audit_res.context_dump.is_empty(), "Context dump should not be empty");
    }

    #[test]
    fn test_vedic_analysis_parity() {
        let (birth, _, _) = setup_test_inputs();
        let vedic_input = VedicAnalysisInput {
            base: birth.clone(),
            precision: BirthTimePrecision::Exact,
            current: Some(CurrentContext {
                now_utc: Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
                analysis_timezone: "Asia/Seoul".to_string(),
            }),
        };

        let vedic_res = facade::analyze_vedic(vedic_input).expect("Vedic analysis failed");
        
        // Verify precise Vedic calculation outputs (Standard Time -> UTC directly, without TST)
        assert_eq!(vedic_res.chart.ascendant.rasi, 10); // Capricorn
        assert_eq!(vedic_res.chart.ascendant.nakshatra, 21); // Uttarashadha
        
        if let Some(moon) = vedic_res.chart.planets.iter().find(|p| p.planet == eon_vedic::planets::VedicPlanet::Moon) {
            assert_eq!(moon.rasi, 10);
            assert_eq!(moon.nakshatra, 22); // Shravana
        } else {
            panic!("Moon not found in Vedic chart");
        }
        
        assert_eq!(vedic_res.meta.precision, BirthTimePrecision::Exact);
        assert!(!vedic_res.chart.planets.is_empty(), "Chart planets should not be empty");
        assert!(!vedic_res.gochara.transits.is_empty(), "Gochara transits should not be empty");
    }

    #[test]
    fn test_compatibility_parity() {
        let (_, saju_input, _) = setup_test_inputs();
        let person2 = SajuAnalysisInput {
            base: AnalysisInput {
                year: 1996,
                month: 5,
                day: 5,
                hour: 12,
                minute: 0,
                is_lunar: false,
                is_leap_month: false,
                lat: 35.6895,
                lon: 139.6917,
                timezone: "Asia/Tokyo".to_string(),
            },
            is_male: false,
            use_night_rat_hour: false,
            precision: BirthTimePrecision::Exact,
        };

        let comp_input = CompatibilityInput {
            person1: saju_input,
            person2,
        };

        let comp_res = facade::analyze_compatibility(comp_input).expect("Compatibility failed");
        assert!(comp_res.saju.sync_score > 0.0, "Saju compatibility score should be > 0");
        assert!(comp_res.vedic.total_score >= 0.0, "Vedic compatibility score should be >= 0");
    }

    #[test]
    fn test_tier_consistency() {
        let (birth, saju_input, transit_input) = setup_test_inputs();
        let vedic_input = VedicAnalysisInput {
            base: birth.clone(),
            precision: BirthTimePrecision::Exact,
            current: Some(CurrentContext {
                now_utc: Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
                analysis_timezone: "Asia/Seoul".to_string(),
            }),
        };

        let saju = facade::analyze_saju(saju_input).expect("Saju res failed");
        let vedic = facade::analyze_vedic(vedic_input).expect("Vedic res failed");
        let transit = Some(facade::analyze_transit(transit_input).expect("Transit res failed"));

        let tier = facade::analyze_destiny_tier(saju, vedic, transit).expect("Tier analysis failed");
        assert!(tier.destiny_score >= 0.0, "Destiny score should be valid");
        assert!(!tier.destiny_tier.grade.is_empty(), "Tier grade should be present");
    }

    fn setup_test_inputs() -> (AnalysisInput, SajuAnalysisInput, TransitAnalysisInput) {
        let birth = AnalysisInput {
            year: 1995,
            month: 11,
            day: 27,
            hour: 11,
            minute: 30,
            is_lunar: false,
            is_leap_month: false,
            lat: 37.5665,
            lon: 126.9780,
            timezone: "Asia/Seoul".to_string(),
        };

        let saju_input = SajuAnalysisInput {
            base: birth.clone(),
            is_male: true,
            use_night_rat_hour: false,
            precision: BirthTimePrecision::Exact,
        };

        let now_utc = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let transit_input = TransitAnalysisInput {
            base: saju_input.clone(),
            current: CurrentContext {
                now_utc,
                analysis_timezone: "Asia/Seoul".to_string(),
            },
        };

        (birth, saju_input, transit_input)
    }
}
