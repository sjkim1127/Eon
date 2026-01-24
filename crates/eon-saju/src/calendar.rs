//! 절기(節氣) 및 달력 관련 유틸리티
//!
//! 24절기 중 12절기를 사용하여 월주를 결정합니다.

use chrono::{DateTime, Utc};
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
        Self::Lichun,   // 寅월
        Self::Jingzhe,  // 卯월
        Self::Qingming, // 辰월
        Self::Lixia,    // 巳월
        Self::Mangzhong,// 午월
        Self::Xiaoshu,  // 未월
        Self::Liqiu,    // 申월
        Self::Bailu,    // 酉월
        Self::Hanlu,    // 戌월
        Self::Lidong,   // 亥월
        Self::Daxue,    // 子월
        Self::Xiaohan,  // 丑월
    ];

    /// 한자 표기
    pub const HANJA: [&'static str; 12] = [
        "立春", "驚蟄", "清明", "立夏", "芒種", "小暑",
        "立秋", "白露", "寒露", "立冬", "大雪", "小寒",
    ];

    /// 한글 표기
    pub const HANGUL: [&'static str; 12] = [
        "입춘", "경칩", "청명", "입하", "망종", "소서",
        "입추", "백로", "한로", "입동", "대설", "소한",
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

/// 특정 연도와 월을 기준으로 대략적인 절기 시작일 계산
/// 
/// 주의: 이 함수는 근사값을 반환합니다. 정확한 절기 시각은
/// 만세력 데이터(`eon-data` 크레이트)를 사용해야 합니다.
pub fn approximate_solar_term_day(year: i32, term: SolarTerm) -> (u32, u32) {
    // 각 절기의 대략적인 월/일 (평균값)
    match term {
        SolarTerm::Xiaohan => (1, 6),   // 소한: 1월 6일경
        SolarTerm::Lichun => (2, 4),    // 입춘: 2월 4일경
        SolarTerm::Jingzhe => (3, 6),   // 경칩: 3월 6일경
        SolarTerm::Qingming => (4, 5),  // 청명: 4월 5일경
        SolarTerm::Lixia => (5, 6),     // 입하: 5월 6일경
        SolarTerm::Mangzhong => (6, 6), // 망종: 6월 6일경
        SolarTerm::Xiaoshu => (7, 7),   // 소서: 7월 7일경
        SolarTerm::Liqiu => (8, 8),     // 입추: 8월 8일경
        SolarTerm::Bailu => (9, 8),     // 백로: 9월 8일경
        SolarTerm::Hanlu => (10, 8),    // 한로: 10월 8일경
        SolarTerm::Lidong => (11, 7),   // 입동: 11월 7일경
        SolarTerm::Daxue => (12, 7),    // 대설: 12월 7일경
    }
}

/// 양력 날짜로부터 해당 월의 절기 지지 인덱스 계산at
/// 
/// 절기를 기준으로 월주의 지지를 결정합니다.
/// 예: 2월 4일(입춘) 이전은 丑월, 이후는 寅월
pub fn get_month_branch_index(year: i32, month: u32, day: u32) -> u8 {
    // 현재 월에 해당하는 절기 찾기
    let term_idx = match month {
        1 => 11,  // 1월: 소한(丑월) 또는 대설(子월)
        2 => 0,   // 2월: 입춘(寅월) 또는 소한(丑월)
        3 => 1,   // 3월: 경칩(卯월) 또는 입춘(寅월)
        4 => 2,   // 4월: 청명(辰월) 또는 경칩(卯월)
        5 => 3,   // 5월: 입하(巳월) 또는 청명(辰월)
        6 => 4,   // 6월: 망종(午월) 또는 입하(巳월)
        7 => 5,   // 7월: 소서(未월) 또는 망종(午월)
        8 => 6,   // 8월: 입추(申월) 또는 소서(未월)
        9 => 7,   // 9월: 백로(酉월) 또는 입추(申월)
        10 => 8,  // 10월: 한로(戌월) 또는 백로(酉월)
        11 => 9,  // 11월: 입동(亥월) 또는 한로(戌월)
        12 => 10, // 12월: 대설(子월) 또는 입동(亥월)
        _ => 0,
    };

    let term = SolarTerm::from_index(term_idx);
    let (_, term_day) = approximate_solar_term_day(year, term);

    // 절기일 이전이면 이전 월의 지지, 이후면 현재 월의 지지
    if day < term_day {
        // 이전 절기의 월 지지
        SolarTerm::from_index(term_idx - 1).month_branch_index()
    } else {
        // 현재 절기의 월 지지
        term.month_branch_index()
    }
}

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
    fn test_get_month_branch_index() {
        // 2월 3일 = 입춘 전 = 丑월(1)
        assert_eq!(get_month_branch_index(2024, 2, 3), 1);
        // 2월 5일 = 입춘 후 = 寅월(2)
        assert_eq!(get_month_branch_index(2024, 2, 5), 2);
    }
}
