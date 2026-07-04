use super::{ExpectedSaju, ExpectedVedic, OracleFixture};
use crate::dto::AnalysisInput;

pub fn get_v1_fixtures() -> Vec<OracleFixture> {
    vec![
        // 1. Standard Case (Male, Seoul)
        OracleFixture {
            id: "std-seoul-1995-11-27".to_string(),
            description: "Standard Case: 1995-11-27 11:30 KST (Male, Seoul)".to_string(),
            input: AnalysisInput {
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
            },
            is_male: true,
            expected_saju: ExpectedSaju {
                year_stem: "Yi".to_string(),
                year_branch: "Hai".to_string(),
                month_stem: "Ding".to_string(),
                month_branch: "Hai".to_string(),
                day_stem: "Ren".to_string(),
                day_branch: "Xu".to_string(),
                hour_stem: "Yi".to_string(),
                hour_branch: "Si".to_string(),
                day_master: "Ren".to_string(),
            },
            expected_vedic: ExpectedVedic {
                ascendant_rasi: 10,
                moon_rasi: 10,
                moon_nakshatra: 22,
            },
        },
        // 2. Lunar Month Edge Case
        OracleFixture {
            id: "lunar-edge-case".to_string(),
            description: "Lunar Edge Case: 1988 Lunar 2-15 10:00 (Standard Solar 1988-04-01)"
                .to_string(),
            input: AnalysisInput {
                year: 1988,
                month: 2,
                day: 15,
                hour: 10,
                minute: 0,
                is_lunar: true,
                is_leap_month: false,
                lat: 37.5665,
                lon: 126.9780,
                timezone: "Asia/Seoul".to_string(),
            },
            is_male: true,
            expected_saju: ExpectedSaju {
                year_stem: "Wu".to_string(),
                year_branch: "Chen".to_string(),
                month_stem: "Yi".to_string(),
                month_branch: "Mao".to_string(),
                day_stem: "Bing".to_string(),
                day_branch: "Xu".to_string(),
                hour_stem: "Gui".to_string(),
                hour_branch: "Si".to_string(),
                day_master: "Bing".to_string(),
            },
            expected_vedic: ExpectedVedic {
                ascendant_rasi: 2,
                moon_rasi: 6,
                moon_nakshatra: 12, // Hasta
            },
        },
        // 3. Overseas Timezone (NY)
        OracleFixture {
            id: "ny-timezone-case".to_string(),
            description: "New York Case: 1990-05-20 15:30 (EDT)".to_string(),
            input: AnalysisInput {
                year: 1990,
                month: 5,
                day: 20,
                hour: 15,
                minute: 30,
                is_lunar: false,
                is_leap_month: false,
                lat: 40.7128,
                lon: -74.0060,
                timezone: "America/New_York".to_string(),
            },
            is_male: false,
            expected_saju: ExpectedSaju {
                year_stem: "Geng".to_string(),
                year_branch: "Wu".to_string(),
                month_stem: "Xin".to_string(),
                month_branch: "Si".to_string(),
                day_stem: "Yi".to_string(),
                day_branch: "You".to_string(),
                hour_stem: "Jia".to_string(),
                hour_branch: "Shen".to_string(),
                day_master: "Yi".to_string(),
            },
            expected_vedic: ExpectedVedic {
                ascendant_rasi: 6,  // Virgo
                moon_rasi: 12,      // Pisces
                moon_nakshatra: 26, // Uttara Bhadrapada
            },
        },
    ]
}
