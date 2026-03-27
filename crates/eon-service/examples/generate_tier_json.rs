use eon_service::dto::*;
use eon_service::facade;
use chrono::{Utc, TimeZone};

fn main() {
    let birth = AnalysisInput {
        year: 1985,
        month: 11,
        day: 27,
        hour: 14,
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

    let now = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    
    let vedic_input = VedicAnalysisInput::new(
        birth.year, birth.month, birth.day, birth.hour, birth.minute,
        birth.is_lunar, birth.is_leap_month,
        birth.lat, birth.lon, birth.timezone.clone(),
        Some(false), Some(now)
    );

    let transit_input = TransitAnalysisInput::new(
        birth.year, birth.month, birth.day, birth.hour, birth.minute,
        birth.is_lunar, birth.is_leap_month,
        saju_input.is_male, saju_input.use_night_rat_hour,
        birth.lon, birth.lat, birth.timezone.clone(),
        Some(false), Some(now)
    );

    let saju_res = facade::analyze_saju(saju_input).expect("Saju failed");
    let vedic_res = facade::analyze_vedic(vedic_input).expect("Vedic failed");
    let transit_res = facade::analyze_transit(transit_input).ok();

    let tier_res = facade::analyze_destiny_tier(saju_res, vedic_res, transit_res).expect("Tier analysis failed");

    println!("{}", serde_json::to_string_pretty(&tier_res).unwrap());
}
