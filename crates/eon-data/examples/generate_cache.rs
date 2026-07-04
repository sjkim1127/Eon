use chrono::{Datelike, TimeZone, Utc};
use eon_astro::AstroEngine;
use eon_data::cache::{LunarMonthRecord, ManseryukCache, SolarTermTable};
use std::fs::File;
use std::io::Write;

fn main() {
    let engine = AstroEngine::new();
    let mut cache = ManseryukCache::default();

    let start_year = 1950;
    let end_year = 2050;

    println!(
        "Generating solar terms/lunar from {} to {}...",
        start_year, end_year
    );

    for year in start_year..=end_year {
        let mut terms = Vec::new();
        for idx in 0..24 {
            let month = ((idx as i32 + 2) / 2) % 12;
            let actual_month = if month == 0 { 12 } else { month } as u32;
            let actual_year = if idx >= 22 && actual_month == 1 {
                year + 1
            } else {
                year
            };

            let approx_day = if idx % 2 == 0 { 5 } else { 20 };
            let approx_time = Utc
                .with_ymd_and_hms(actual_year, actual_month, approx_day, 0, 0, 0)
                .unwrap();

            if let Ok(exact_time) = engine.find_solar_term_time(approx_time, idx) {
                terms.push((idx, exact_time));
            }
        }

        terms.sort_by_key(|&(_, time)| time);
        cache.years.insert(year, SolarTermTable { year, terms });

        // 2. 음력 데이터 생성
        let mut lunar_records = Vec::new();
        let year_start = Utc.with_ymd_and_hms(year, 1, 1, 0, 0, 0).unwrap();
        let year_end = Utc.with_ymd_and_hms(year, 12, 31, 23, 59, 59).unwrap();

        let mut current_nm_t = engine.find_new_moon_before(year_start).unwrap();

        for _ in 0..15 {
            let search_t = current_nm_t + chrono::Duration::days(31);
            if let Ok(next_nm_t) = engine.find_new_moon_before(search_t) {
                if next_nm_t <= current_nm_t {
                    break;
                }
                if next_nm_t > year_end {
                    break;
                }

                let kst_offset = chrono::Duration::hours(9);
                let nm_date_kst = (next_nm_t + kst_offset).date_naive();

                if let Ok((ly, lm, _, il)) =
                    eon_data::manseryuk::LunarCalendar::from_solar_internal(nm_date_kst)
                {
                    if nm_date_kst.year() == year {
                        lunar_records.push(LunarMonthRecord {
                            new_moon_date: nm_date_kst,
                            lunar_year: ly,
                            lunar_month: lm,
                            is_leap: il,
                        });
                    }
                }
                current_nm_t = next_nm_t;
            } else {
                break;
            }
        }
        lunar_records.sort_by_key(|r| r.new_moon_date);
        cache.lunar_months.insert(year, lunar_records);

        if year % 10 == 0 {
            println!("Processed year: {}", year);
        }
    }

    let binary = bincode::serialize(&cache).expect("Failed to serialize cache");
    let mut file =
        File::create("crates/eon-data/src/manseryuk.bin").expect("Failed to create file");
    file.write_all(&binary).expect("Failed to write binary");

    println!(
        "Successfully generated 'manseryuk.bin' ({} bytes)",
        binary.len()
    );
}
