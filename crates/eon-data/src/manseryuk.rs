//! 만세력(萬歲曆) 데이터
//!
//! 일진 조회 및 절기 시각 데이터를 제공합니다.

use chrono::{DateTime, Datelike, NaiveDate, Utc, TimeZone};

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
    // 각 절기의 대략적인 월/일과 시각 (UTC 기준)
    let (month, day, hour) = match term_index {
        0 => (2, 4, 4),    // 입춘
        1 => (3, 6, 4),    // 경칩
        2 => (4, 5, 4),    // 청명
        3 => (5, 6, 4),    // 입하
        4 => (6, 6, 4),    // 망종
        5 => (7, 7, 4),    // 소서
        6 => (8, 8, 4),    // 입추
        7 => (9, 8, 4),    // 백로
        8 => (10, 8, 4),   // 한로
        9 => (11, 7, 4),   // 입동
        10 => (12, 7, 4),  // 대설
        11 => (1, 6, 4),   // 소한
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
    /// 음력을 양력으로 변환
    pub fn to_solar(_year: i32, _month: u32, _day: u32, _is_leap: bool) -> Option<NaiveDate> {
        // TODO: 음력-양력 변환 구현
        // 현재는 미구현
        None
    }

    /// 양력을 음력으로 변환
    pub fn from_solar(_date: NaiveDate) -> Option<(i32, u32, u32, bool)> {
        // TODO: 양력-음력 변환 구현
        // 현재는 미구현
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
