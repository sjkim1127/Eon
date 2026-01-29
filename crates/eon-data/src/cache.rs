use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc, Datelike};
use std::sync::OnceLock;

/// 절기 시각 테이블 (1년치)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolarTermTable {
    pub year: i32,
    /// (절기 인덱스 0~23, 시각) 목록
    pub terms: Vec<(u8, DateTime<Utc>)>,
}

/// 음력 월 레코드
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LunarMonthRecord {
    /// 합삭일 (KST 기준)
    pub new_moon_date: chrono::NaiveDate,
    pub lunar_year: i32,
    pub lunar_month: u32,
    pub is_leap: bool,
}

/// 만세력 바이너리 캐시 구조체
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManseryukCache {
    /// 연도별 절기 데이터
    pub years: HashMap<i32, SolarTermTable>,
    /// 연도별 음력 데이터 (해당 연도에 시작하는 음력 월 목록)
    pub lunar_months: HashMap<i32, Vec<LunarMonthRecord>>,
}

impl ManseryukCache {
    /// 바이너리 데이터로부터 캐시 로드
    pub fn from_binary(bytes: &[u8]) -> Result<Self, String> {
        bincode::deserialize(bytes).map_err(|e| e.to_string())
    }

    /// 특정 연도/절기의 시각 조회
    pub fn get_solar_term(&self, year: i32, term_idx: u8) -> Option<DateTime<Utc>> {
        self.years.get(&year).and_then(|table| {
            table.terms.iter()
                .find(|(idx, _)| *idx == term_idx)
                .map(|(_, time)| *time)
        })
    }

    /// 특정 시점이 포함된 절기 인덱스 조회 (정적 데이터 기반 고속 검색)
    pub fn get_term_index_at(&self, time: DateTime<Utc>) -> Option<u8> {
        let year = time.year();
        let table = self.years.get(&year)?;
        
        let mut best_idx = 0;
        let mut last_time = DateTime::<Utc>::MIN_UTC;
        
        for (idx, term_time) in &table.terms {
            if time >= *term_time && *term_time > last_time {
                best_idx = *idx;
                last_time = *term_time;
            }
        }
        Some(best_idx)
    }

    /// 특정 양력 날짜에 대응하는 음력 데이터 조회
    pub fn get_lunar_date(&self, solar_date: chrono::NaiveDate) -> Option<(i32, u32, u32, bool)> {
        let year = solar_date.year();
        // 현재 해와 이전 해 데이터 스캔 (음력 1월이 양력 2월경이므로 이전 해 데이터 필요)
        let mut all_months = Vec::new();
        if let Some(m) = self.lunar_months.get(&(year - 1)) {
            all_months.extend(m);
        }
        if let Some(m) = self.lunar_months.get(&year) {
            all_months.extend(m);
        }
        
        // solar_date보다 작거나 같은 가장 가까운 합삭일 찾기
        let record = all_months.iter()
            .filter(|r| r.new_moon_date <= solar_date)
            .max_by_key(|r| r.new_moon_date)?;
            
        let day = (solar_date - record.new_moon_date).num_days() as u32 + 1;
        Some((record.lunar_year, record.lunar_month, day, record.is_leap))
    }
}

/// 전역 공유 캐시 인스턴스 (Lazy initialization)
static GLOBAL_CACHE_INTERNAL: OnceLock<ManseryukCache> = OnceLock::new();

pub struct GlobalCache;

impl GlobalCache {
    pub fn get() -> &'static ManseryukCache {
        GLOBAL_CACHE_INTERNAL.get_or_init(|| {
            // 빌드 시 포함된 바이너리 데이터 로드
            let bytes = include_bytes!("manseryuk.bin");
            ManseryukCache::from_binary(bytes).unwrap_or_else(|e| {
                eprintln!("Failed to load manseryuk cache: {}", e);
                ManseryukCache::default()
            })
        })
    }

    pub fn get_solar_term(year: i32, term_idx: u8) -> Option<DateTime<Utc>> {
        Self::get().get_solar_term(year, term_idx)
    }

    pub fn get_lunar_date(solar_date: chrono::NaiveDate) -> Option<(i32, u32, u32, bool)> {
        Self::get().get_lunar_date(solar_date)
    }
}
