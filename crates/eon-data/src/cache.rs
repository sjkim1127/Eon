use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc, Datelike};
use once_cell::sync::Lazy;

/// 절기 시각 테이블 (1년치)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolarTermTable {
    pub year: i32,
    /// (절기 인덱스 0~23, 시각) 목록
    pub terms: Vec<(u8, DateTime<Utc>)>,
}

/// 만세력 바이너리 캐시 구조체
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManseryukCache {
    /// 연도별 절기 데이터
    pub years: HashMap<i32, SolarTermTable>,
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
        
        // 시각 기준 내림차순 정렬되어 있다고 가정하거나, 적절히 탐색
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
}

/// 전역 공유 캐시 인스턴스
pub static GLOBAL_CACHE: Lazy<ManseryukCache> = Lazy::new(|| {
    // 빌드 시 포함된 바이너리 데이터 로드
    let bytes = include_bytes!("manseryuk.bin");
    ManseryukCache::from_binary(bytes).unwrap_or_else(|e| {
        eprintln!("Failed to load manseryuk cache: {}", e);
        ManseryukCache::default()
    })
});
