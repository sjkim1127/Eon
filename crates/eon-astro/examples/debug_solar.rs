use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use eon_astro::AstroEngine;

fn get_solar_term_index(dt: DateTime<Utc>) -> u8 {
    let engine = AstroEngine::new();
    let sun_long = engine.get_sun_longitude(dt).unwrap();
    let adjusted = (sun_long - 315.0 + 360.0) % 360.0;
    let index = (adjusted / 15.0).floor() as u8;
    println!(
        "sun_long: {}, adjusted: {}, index: {}, term_12: {}",
        sun_long,
        adjusted,
        index % 24,
        (index % 24) / 2
    );
    index % 24
}

fn main() {
    let tz = FixedOffset::east_opt(540 * 60).unwrap();

    // 2024년 2월 3일 KST 12:00
    let dt1 = tz
        .with_ymd_and_hms(2024, 2, 3, 12, 0, 0)
        .single()
        .unwrap()
        .with_timezone(&Utc);
    println!("2024-02-03 12:00 KST: {:?}", dt1);
    get_solar_term_index(dt1);

    // 2024년 2월 4일 KST 17:00
    let dt2 = tz
        .with_ymd_and_hms(2024, 2, 4, 17, 0, 0)
        .single()
        .unwrap()
        .with_timezone(&Utc);
    println!("2024-02-04 17:00 KST: {:?}", dt2);
    get_solar_term_index(dt2);
}
