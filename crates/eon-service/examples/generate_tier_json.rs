use chrono::{TimeZone, Utc};
use eon_service::dto::*;
use eon_service::facade;

fn main() {
    let birth = AnalysisInput::new(1985, 11, 27, 14, 30, "Asia/Seoul", 37.5665, 126.9780);

    let saju_input = SajuAnalysisInput {
        base: birth.clone(),
        is_male: true,
        use_night_rat_hour: false,
        precision: BirthTimePrecision::Exact,
    };

    let now = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();

    let vedic_input = VedicAnalysisInput::new(birth.clone(), Some(false), Some(now));

    let transit_input = TransitAnalysisInput::new(saju_input.clone(), Some(now));

    let saju_res = facade::analyze_saju(saju_input).expect("Saju failed");
    let vedic_res = facade::analyze_vedic(vedic_input).expect("Vedic failed");
    let transit_res = facade::analyze_transit(transit_input).ok();

    let tier_res = facade::analyze_destiny_tier(saju_res, vedic_res, transit_res)
        .expect("Tier analysis failed");

    println!("{}", serde_json::to_string_pretty(&tier_res).unwrap());
}
