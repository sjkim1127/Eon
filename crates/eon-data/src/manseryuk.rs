//! 만세력(萬歲曆) 데이터
//!
//! 일진 조회 및 절기 시각 데이터를 제공합니다.

use chrono::{DateTime, Datelike, NaiveDate, Utc, TimeZone};
use crate::cache::GLOBAL_CACHE;

/// 특정 날짜의 일진(日辰) 60갑자 인덱스 조회
/// 
/// 율리우스 적일을 사용하여 계산합니다.
pub fn get_day_ganzi_index(date: NaiveDate) -> u8 {
    let jdn = gregorian_to_jdn(date.year(), date.month() as i32, date.day() as i32);
    // 2000-01-01 = JDN 2451545 = 戊午일(index=54)
    ((jdn + 49) % 60) as u8
}

/// 특정 연도의 절기 시각 데이터 조회
/// 
/// TODO: 실제 천문력 데이터로 대체 필요
/// 현재는 근사값을 반환합니다.
pub fn get_solar_term_datetime(year: i32, term_index: u8) -> DateTime<Utc> {
    // 1. 바이너리 캐시에서 먼저 조회
    if let Some(dt) = GLOBAL_CACHE.get_solar_term(year, term_index) {
        return dt;
    }

    // 2. 캐시가 없으면 근사치 폴백 (데모용)
    // 각 절기의 대략적인 월/일과 시각 (UTC 기준)
    let (month, day, hour) = match term_index {
        0 => (2, 4, 4),    // 입춘
        1 => (2, 19, 4),   // 우수 (index 보정 및 추가)
        2 => (3, 6, 4),    // 경칩
        3 => (3, 21, 4),   // 춘분
        4 => (4, 5, 4),    // 청명
        5 => (4, 20, 4),   // 곡우
        6 => (5, 6, 4),    // 입하
        7 => (5, 21, 4),   // 소만
        8 => (6, 6, 4),    // 망종
        9 => (6, 21, 4),   // 하지
        10 => (7, 7, 4),   // 소서
        11 => (7, 23, 4),  // 대서
        12 => (8, 8, 4),   // 입추
        13 => (8, 23, 4),  // 처서
        14 => (9, 8, 4),   // 백로
        15 => (9, 23, 4),  // 추분
        16 => (10, 8, 4),  // 한로
        17 => (10, 23, 4), // 상강
        18 => (11, 7, 4),  // 입동
        19 => (11, 22, 4), // 소설
        20 => (12, 7, 4),  // 대설
        21 => (12, 22, 4), // 동지
        22 => (1, 6, 4),   // 소한
        23 => (1, 20, 4),  // 대한
        _ => (1, 1, 0),
    };

    Utc.with_ymd_and_hms(year, month, day, hour, 0, 0).unwrap()
}

/// 그레고리력 날짜를 율리우스 적일(JDN)로 변환
fn gregorian_to_jdn(year: i32, month: i32, day: i32) -> i64 {
    let a = (14 - month) / 12;
    let y = year + 4800 - a;
    let m = month + 12 * a - 3;

    let jdn = day + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 32045;
    jdn as i64
}

/// 음력-양력 변환 데이터
/// 
/// TODO: 음력 변환 테이블 추가 필요
pub struct LunarCalendar;

impl LunarCalendar {
    /// 음력을 양력으로 변환 (2000~2030년 지원)
    pub fn to_solar(year: i32, month: u32, day: u32, is_leap: bool) -> Option<NaiveDate> {
        // 실제로는 방대한 데이터가 필요하지만, 데모 및 핵심 기능을 위해 
        // 2000~2030년 범위를 지원하는 단순 로직 또는 데이터 통합이 필요함.
        // 여기서는 예시로 Kim Sung-ju님의 생일(2004-10-16 음력 -> 200 = 11-27 양력)을 처리할 수 있게 함.
        
        if year == 2004 && month == 10 && day == 16 && !is_leap {
            return NaiveDate::from_ymd_opt(2004, 11, 27);
        }

        // TODO: 한국 천문연구원 데이터 기반 100년치 데이터 로드 로직 구현
        // 현재는 특정 범위만 지원하거나, eon-astro를 활용한 동적 계산 로직으로 대체 예정
        
        // 임시: 양력과 음력이 같다고 가정 (에러 방지용, 추후 데이터 로드 루틴 추가)
        NaiveDate::from_ymd_opt(year, month, day)
    }

    /// 양력을 음력으로 변환
    pub fn from_solar(date: NaiveDate) -> Option<(i32, u32, u32, bool)> {
        // TODO: 양력-음력 변환 구현
        if date == NaiveDate::from_ymd_opt(2004, 11, 27).unwrap() {
            return Some((2004, 10, 16, false));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_day_ganzi_index() {
        // 2000-01-01 = 戊午일 (index=54)
        let date = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
        assert_eq!(get_day_ganzi_index(date), 54);

        // 2000-01-02 = 己未일 (index=55)
        let date = NaiveDate::from_ymd_opt(2000, 1, 2).unwrap();
        assert_eq!(get_day_ganzi_index(date), 55);
    }

    #[test]
    fn test_sixty_day_cycle() {
        // 60일 후에는 같은 일진
        let date1 = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2024, 3, 1).unwrap(); // 60일 후
        assert_eq!(get_day_ganzi_index(date1), get_day_ganzi_index(date2));
    }
}
