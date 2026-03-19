#[cfg(test)]
mod tests {
    use crate::dto::*;
    use crate::facade;
    use chrono::{Utc, TimeZone};

    #[test]
    fn test_unified_analysis_parity() {
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

        // 1. Saju
        let saju_res = facade::analyze_saju(saju_input.clone()).expect("Saju analysis failed");
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
}
