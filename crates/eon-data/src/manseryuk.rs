//! 만세력(萬歲曆) 데이터
//!
//! 일진 조회 및 절기 시각 데이터를 제공합니다.

use crate::cache::GlobalCache;
use chrono::{DateTime, Datelike, NaiveDate, TimeZone, Utc};

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
/// 바이너리 캐시에서 먼저 조회하고, 없으면 eon-astro를 통해 실시간 계산합니다.
pub fn get_solar_term_datetime(
    year: i32,
    term_index: u8,
) -> Result<DateTime<Utc>, crate::error::DataError> {
    // 1. 바이너리 캐시에서 먼저 조회
    if let Some(dt) = GlobalCache::get_solar_term(year, term_index) {
        return Ok(dt);
    }

    // 2. 캐시가 없으면 eon-astro를 사용하여 실시간 정밀 계산
    let engine = eon_astro::AstroEngine::new();

    // 절기별 대략적인 위치 (입춘=315도)
    // 2월 초순(입춘)부터 약 15일 간격으로 24절기가 배치됨
    // 대략적인 시작 시점을 2월 1일로 잡고 검색
    let approx_start = Utc
        .with_ymd_and_hms(year, 2, 1, 0, 0, 0)
        .single()
        .ok_or(crate::error::DataError::InvalidDate)?;

    match engine.find_solar_term_time(approx_start, term_index) {
        Ok(dt) => Ok(dt),
        Err(_) => {
            // 엔진 실패시 최후의 폴백 (근사값 계산)
            // 입춘(0번)은 대략 2월 4일
            // 각 절기/중기는 약 15.22일 간격
            let base_date = Utc
                .with_ymd_and_hms(year, 2, 4, 0, 0, 0)
                .single()
                .ok_or(crate::error::DataError::InvalidDate)?;
            let offset_days = (term_index as f64 * 15.218).round() as i64;
            Ok(base_date + chrono::Duration::days(offset_days))
        }
    }
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
pub struct LunarCalendar;

impl LunarCalendar {
    /// 양력을 음력으로 변환 (KST 기준 정밀 계산) - 캐시 사용
    pub fn from_solar(date: NaiveDate) -> Result<(i32, u32, u32, bool), crate::error::DataError> {
        // 1. 캐시에서 먼저 조회
        if let Some(res) = GlobalCache::get_lunar_date(date) {
            return Ok(res);
        }

        // 2. 캐시가 없으면 실시간 정밀 계산
        Self::from_solar_internal(date)
    }

    /// 양력을 음력으로 변환 (실시간 천문 계산 엔진 사용)
    pub fn from_solar_internal(
        date: NaiveDate,
    ) -> Result<(i32, u32, u32, bool), crate::error::DataError> {
        let engine = eon_astro::AstroEngine::new();
        let kst_offset = chrono::Duration::hours(9);

        // 1. KST 당일의 끝 시점의 UTC 변환
        let dt_utc = Utc
            .with_ymd_and_hms(date.year(), date.month(), date.day(), 23, 59, 59)
            .single()
            .ok_or(crate::error::DataError::InvalidDate)?
            - kst_offset;

        // 2. 현재 달 혹은 당일의 합삭(New Moon) 시각 찾기
        let nm_t = engine.find_new_moon_before(dt_utc)?;
        let nm_date_kst = (nm_t + kst_offset).date_naive();
        let lunar_day = (date - nm_date_kst).num_days() as u32 + 1;

        // 3. 월 및 연도 결정을 위한 기준점(동지) 찾기
        // 현재 합삭 시점 이전 혹은 포함된 동지(Winter Solstice)를 찾음
        let ws_t = engine.find_winter_solstice_before(nm_t + chrono::Duration::days(1))?;
        let ws_nm_t = engine.find_new_moon_before(ws_t + chrono::Duration::minutes(1))?;

        // 동지가 포함된 달은 무조건 음력 11월
        let mut curr_nm = ws_nm_t;
        let mut month_num = 11;
        let mut is_leap = false;
        let mut lunar_year = (ws_t + kst_offset).year();
        // 동지가 12월 말에 있으므로, 이 달이 속한 음력 연도의 1월은 보통 다음 해로 넘어감
        if (ws_t + kst_offset).month() == 12 {
            // 동지가 12월이면 그해 1월은 이미 지났으므로, 동지 달(11월)은 해당 연도의 음력으로 침
        }

        // 4. 동지 달(11월)부터 현재 합삭 달까지 진행하며 월 번호 부여
        while (nm_t - curr_nm).num_minutes() > 4320 {
            let next_nm = engine.find_new_moon_before(curr_nm + chrono::Duration::days(45))?;

            // 중기(Zhong-qi) 포함 여부 확인
            let mut has_zq = false;
            for i in 1..=23 {
                if i % 2 == 1 {
                    // 중기 인덱스 (1, 3, 5, ...)
                    if let Ok(zq_t) = engine.find_solar_term_time(curr_nm, i) {
                        // 중기가 합삭일 당일에 있어도 포함된 것으로 간주 (KST 기준)
                        let zq_date = (zq_t + kst_offset).date_naive();
                        let curr_nm_date = (curr_nm + kst_offset).date_naive();
                        let next_nm_date = (next_nm + kst_offset).date_naive();

                        if zq_date >= curr_nm_date && zq_date < next_nm_date {
                            has_zq = true;
                            break;
                        }
                    }
                }
            }

            if !has_zq && !is_leap {
                // 이번 달에 중기가 없고, 이번 무중치윤 구간에서 아직 윤달을 넣지 않았다면 윤달 지정
                is_leap = true;
            } else {
                month_num += 1;
                if month_num > 12 {
                    month_num = 1;
                    lunar_year += 1;
                }
                is_leap = false;
            }
            curr_nm = next_nm;
        }

        Ok((lunar_year, month_num as u32, lunar_day, is_leap))
    }

    /// 음력을 양력으로 변환
    pub fn to_solar(
        year: i32,
        month: u32,
        day: u32,
        is_leap: bool,
    ) -> Result<NaiveDate, crate::error::DataError> {
        let test_date =
            NaiveDate::from_ymd_opt(year, month, 1).ok_or(crate::error::DataError::InvalidDate)?;

        // 해당 연도 전후 6개월 범위를 스캔 (음력 1월이 양력 2월경이므로 폭넓게 스캔)
        for i in -60..360 {
            let date = test_date + chrono::Duration::days(i);
            if let Ok((ly, lm, ld, il)) = Self::from_solar(date) {
                if ly == year && lm == month && ld == day && il == is_leap {
                    return Ok(date);
                }
            }
        }
        Err(crate::error::DataError::InvalidDate)
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

    #[test]
    fn test_lunar_conversion() {
        // 김성주님 생일: 양력 2004-11-27 -> 음력 2004-10-16
        let solar = NaiveDate::from_ymd_opt(2004, 11, 27).unwrap();
        let lunar = LunarCalendar::from_solar(solar).unwrap();
        println!("DEBUG: Lunar result for 2004-11-27: {:?}", lunar);

        assert_eq!(lunar.0, 2004, "Year should be 2004");
        assert_eq!(lunar.1, 10, "Month should be 10");
        assert_eq!(lunar.2, 16, "Day should be 16");
        assert!(!lunar.3, "Should not be a leap month");

        // 역변환 확인
        let solar_back = LunarCalendar::to_solar(2004, 10, 16, false).unwrap();
        assert_eq!(solar_back, solar);
    }
}
