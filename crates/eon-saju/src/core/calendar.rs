//! 절기(節氣) 및 달력 관련 유틸리티
//!
//! 24절기 중 12절기를 사용하여 월주를 결정합니다.

use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use serde::{Deserialize, Serialize};

/// 12절기 (월 구분용)
///
/// 12절기는 월의 시작을 나타내며, 12중기는 월의 중간을 나타냅니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SolarTerm {
    /// 입춘(立春) - 寅월 시작 (대략 2/4)
    Lichun,
    /// 경칩(驚蟄) - 卯월 시작 (대략 3/6)
    Jingzhe,
    /// 청명(清明) - 辰월 시작 (대략 4/5)
    Qingming,
    /// 입하(立夏) - 巳월 시작 (대략 5/6)
    Lixia,
    /// 망종(芒種) - 午월 시작 (대략 6/6)
    Mangzhong,
    /// 소서(小暑) - 未월 시작 (대략 7/7)
    Xiaoshu,
    /// 입추(立秋) - 申월 시작 (대략 8/8)
    Liqiu,
    /// 백로(白露) - 酉월 시작 (대략 9/8)
    Bailu,
    /// 한로(寒露) - 戌월 시작 (대략 10/8)
    Hanlu,
    /// 입동(立冬) - 亥월 시작 (대략 11/7)
    Lidong,
    /// 대설(大雪) - 子월 시작 (대략 12/7)
    Daxue,
    /// 소한(小寒) - 丑월 시작 (대략 1/6)
    Xiaohan,
}

impl SolarTerm {
    /// 모든 12절기 배열 (寅월부터 순서대로)
    pub const ALL: [SolarTerm; 12] = [
        Self::Lichun,    // 寅월
        Self::Jingzhe,   // 卯월
        Self::Qingming,  // 辰월
        Self::Lixia,     // 巳월
        Self::Mangzhong, // 午월
        Self::Xiaoshu,   // 未월
        Self::Liqiu,     // 申월
        Self::Bailu,     // 酉월
        Self::Hanlu,     // 戌월
        Self::Lidong,    // 亥월
        Self::Daxue,     // 子월
        Self::Xiaohan,   // 丑월
    ];

    /// 한자 표기
    pub const HANJA: [&'static str; 12] = [
        "立春", "驚蟄", "清明", "立夏", "芒種", "小暑", "立秋", "白露", "寒露", "立冬", "大雪",
        "小寒",
    ];

    /// 한글 표기
    pub const HANGUL: [&'static str; 12] = [
        "입춘", "경칩", "청명", "입하", "망종", "소서", "입추", "백로", "한로", "입동", "대설",
        "소한",
    ];

    /// 인덱스 (0-11)
    #[inline]
    pub const fn index(self) -> u8 {
        match self {
            Self::Lichun => 0,
            Self::Jingzhe => 1,
            Self::Qingming => 2,
            Self::Lixia => 3,
            Self::Mangzhong => 4,
            Self::Xiaoshu => 5,
            Self::Liqiu => 6,
            Self::Bailu => 7,
            Self::Hanlu => 8,
            Self::Lidong => 9,
            Self::Daxue => 10,
            Self::Xiaohan => 11,
        }
    }

    /// 인덱스로부터 절기 생성
    #[inline]
    pub const fn from_index(idx: i32) -> Self {
        Self::ALL[idx.rem_euclid(12) as usize]
    }

    /// 해당 절기가 시작하는 월의 지지 인덱스 (寅=2부터)
    #[inline]
    pub const fn month_branch_index(self) -> u8 {
        // 입춘 = 寅월(2), 경칩 = 卯월(3), ...
        (self.index() + 2) % 12
    }

    /// 지지 인덱스로부터 해당 월의 시작 절기 반환
    #[inline]
    pub const fn from_month_branch_index(branch_idx: u8) -> Self {
        Self::from_index((branch_idx as i32 - 2).rem_euclid(12))
    }

    /// 한자 표기
    #[inline]
    pub const fn hanja(self) -> &'static str {
        Self::HANJA[self.index() as usize]
    }

    /// 한글 표기
    #[inline]
    pub const fn hangul(self) -> &'static str {
        Self::HANGUL[self.index() as usize]
    }
}

impl std::fmt::Display for SolarTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hanja())
    }
}

/// 절기 시각 데이터
#[derive(Debug, Clone, Copy)]
pub struct SolarTermTime {
    pub term: SolarTerm,
    pub datetime: DateTime<Utc>,
}

/// 특정 시점을 기준으로 해당 절기의 정확한 시작 시각 계산 (eon-astro 활용)
pub fn get_solar_term_time(dt: DateTime<Utc>, term: SolarTerm) -> DateTime<Utc> {
    use eon_astro::AstroEngine;
    let engine = AstroEngine::new();

    // AstroEngine의 term_idx: 0=입춘(315도), 1=우수, 2=경칩...
    // SolarTerm::index: 0=입춘, 1=경칩...
    // 월의 시작 절기는 짝수 인덱스
    engine
        .find_solar_term_time(dt, (term.index() * 2) as u8)
        .unwrap_or(dt)
}

/// 양력 날짜/시각 + Timezone offset으로부터 해당 월의 절기 지지 인덱스 계산
///
/// # Arguments
/// * `year` - 양력 년도
/// * `month` - 양력 월 (1-12)
/// * `day` - 양력 일 (1-31)
/// * `hour` - 시 (0-23)
/// * `minute` - 분 (0-59)
/// * `tz_offset_minutes` - 시간대 오프셋 **분 단위** (예: KST=540, IST=330, UTC=0)
///
/// # Returns
/// 월지 인덱스 (0=子, 1=丑, 2=寅, ...)
pub fn get_month_branch_index(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    tz_offset_minutes: i32,
) -> Result<u8, CalendarError> {
    // Timezone offset 적용하여 FixedOffset 생성
    let offset_secs = tz_offset_minutes * 60;
    let tz = FixedOffset::east_opt(offset_secs)
        .ok_or_else(|| CalendarError::InvalidTimezone(tz_offset_minutes))?;

    // 로컬 시간 생성 후 UTC로 변환
    // FixedOffset은 DST 변화가 없으므로 Ambiguous는 발생하지 않음
    let dt_utc = tz
        .with_ymd_and_hms(year, month, day, hour, minute, 0)
        .single()
        .ok_or(CalendarError::InvalidDateTime {
            year,
            month,
            day,
            hour,
            minute,
        })?
        .with_timezone(&Utc);

    Ok(get_month_branch_index_from_dt(dt_utc))
}

/// DateTime<Utc>로부터 해당 월의 절기 지지 인덱스 계산
///
/// 이 함수는 이미 UTC로 변환된 시간을 받으므로 timezone 문제가 없습니다.
pub fn get_month_branch_index_from_dt(dt: DateTime<Utc>) -> u8 {
    use eon_astro::AstroEngine;

    let engine = AstroEngine::new();

    // 태양 황경으로부터 현재 어떤 절기권에 있는지 계산
    // AstroEngine::get_solar_term_index는 24절기(0~23)를 반환
    let term_24_idx = engine.get_solar_term_index(dt);

    // 12절기(월 구분용) 인덱스로 변환 (0: 입춘, 1: 경칩, 2: 청명...)
    let term_12_idx = term_24_idx / 2;

    // 해당 절기가 시작하는 월의 지지 인덱스 (寅=2부터)
    SolarTerm::from_index(term_12_idx as i32).month_branch_index()
}

/// 달력 관련 에러
#[derive(Debug, Clone)]
pub enum CalendarError {
    /// 유효하지 않은 날짜/시간
    InvalidDateTime {
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
    },
    /// 유효하지 않은 시간대
    InvalidTimezone(i32),
}

impl std::fmt::Display for CalendarError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidDateTime {
                year,
                month,
                day,
                hour,
                minute,
            } => {
                write!(
                    f,
                    "Invalid datetime: {}-{:02}-{:02} {:02}:{:02}",
                    year, month, day, hour, minute
                )
            }
            Self::InvalidTimezone(offset) => {
                write!(f, "Invalid timezone offset: {} minutes", offset)
            }
        }
    }
}

impl std::error::Error for CalendarError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solar_term_month_branch() {
        // 입춘 = 寅월(2)
        assert_eq!(SolarTerm::Lichun.month_branch_index(), 2);
        // 경칩 = 卯월(3)
        assert_eq!(SolarTerm::Jingzhe.month_branch_index(), 3);
        // 대설 = 子월(0)
        assert_eq!(SolarTerm::Daxue.month_branch_index(), 0);
        // 소한 = 丑월(1)
        assert_eq!(SolarTerm::Xiaohan.month_branch_index(), 1);
    }

    #[test]
    fn test_get_month_branch_index_kst() {
        // 2024년 2월 3일 KST 12:00 = 입춘 전 = 丑월(1)
        let result = get_month_branch_index(2024, 2, 3, 12, 0, 540).unwrap();
        assert_eq!(result, 1);

        // 2024년 2월 5일 KST 12:00 = 입춘 후 = 寅월(2)
        let result = get_month_branch_index(2024, 2, 5, 12, 0, 540).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_timezone_boundary() {
        // 2024년 2월 4일 입춘 시각: 17:27 KST (08:27 UTC)

        // KST 17:00 (입춘 전) = 丑월
        let before = get_month_branch_index(2024, 2, 4, 17, 0, 540).unwrap();
        assert_eq!(before, 1, "입춘 직전(17:00)은 丑월이어야 함");

        // KST 18:00 (입춘 후) = 寅월
        let after = get_month_branch_index(2024, 2, 4, 18, 0, 540).unwrap();
        assert_eq!(after, 2, "입춘 직후(18:00)는 寅월이어야 함");

        // UTC로 같은 시각 테스트 (08:00 UTC = 17:00 KST, 입춘 전)
        let utc_before = get_month_branch_index(2024, 2, 4, 8, 0, 0).unwrap();
        assert_eq!(utc_before, 1, "UTC 08:00 (입춘 전)은 丑월이어야 함");

        // UTC 09:00 = KST 18:00 (입춘 후)
        let utc_after = get_month_branch_index(2024, 2, 4, 9, 0, 0).unwrap();
        assert_eq!(utc_after, 2, "UTC 09:00 (입춘 후)은 寅월이어야 함");
    }
}
