use eon_astro::AstroEngine;
use eon_data::cache::{ManseryukCache, SolarTermTable};
use chrono::{DateTime, Utc, TimeZone};
use std::fs::File;
use std::io::Write;

fn main() {
    let engine = AstroEngine::new();
    let mut cache = ManseryukCache::default();
    
    let start_year = 1950;
    let end_year = 2050;
    
    println!("Generating solar terms from {} to {} using Swiss Ephemeris...", start_year, end_year);
    
    for year in start_year..=end_year {
        let mut terms = Vec::new();
        for idx in 0..24 {
            // 입춘(315도) 기준으로 순차적 탐색을 위한 대략적인 시각 설정
            // 각 월별 절기(Jieqi)와 중기(Zhongqi)의 근사치
            let month = ((idx as i32 + 2) / 2) % 12;
            let actual_month = if month == 0 { 12 } else { month } as u32;
            let actual_year = if idx >= 22 && actual_month == 1 { year + 1 } else { year };
            
            let approx_day = if idx % 2 == 0 { 5 } else { 20 };
            let approx_time = Utc.with_ymd_and_hms(actual_year, actual_month, approx_day, 0, 0, 0).unwrap();
            
            if let Ok(exact_time) = engine.find_solar_term_time(approx_time, idx) {
                terms.push((idx, exact_time));
            }
        }
        
        // 정렬 보장 (시간 순)
        terms.sort_by_key(|&(_, time)| time);
        
        cache.years.insert(year, SolarTermTable { year, terms });
        if year % 10 == 0 {
            println!("Processed year: {}", year);
        }
    }
    
    let binary = bincode::serialize(&cache).expect("Failed to serialize cache");
    // eon-data 소스 디렉토리에 직접 저장 (build 시 include_bytes!로 포함시키기 위함)
    let mut file = File::create("crates/eon-data/src/manseryuk.bin").expect("Failed to create file");
    file.write_all(&binary).expect("Failed to write binary");
    
    println!("Successfully generated 'manseryuk.bin' ({} bytes)", binary.len());
    println!("Path: crates/eon-data/src/manseryuk.bin");
}
