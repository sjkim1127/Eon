//! 지지(地支, Earthly Branches) - 12개의 지지
//!
//! 子丑寅卯辰巳午未申酉戌亥

use serde::{Deserialize, Serialize};
use crate::element::{Element, Polarity};

/// 지지(地支) - 12개
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum EarthlyBranch {
    /// 子 (자) - 쥐 - 양수
    Zi = 0,
    /// 丑 (축) - 소 - 음토
    Chou = 1,
    /// 寅 (인) - 호랑이 - 양목
    Yin = 2,
    /// 卯 (묘) - 토끼 - 음목
    Mao = 3,
    /// 辰 (진) - 용 - 양토
    Chen = 4,
    /// 巳 (사) - 뱀 - 음화
    Si = 5,
    /// 午 (오) - 말 - 양화
    Wu = 6,
    /// 未 (미) - 양 - 음토
    Wei = 7,
    /// 申 (신) - 원숭이 - 양금
    Shen = 8,
    /// 酉 (유) - 닭 - 음금
    You = 9,
    /// 戌 (술) - 개 - 양토
    Xu = 10,
    /// 亥 (해) - 돼지 - 음수
    Hai = 11,
}

impl EarthlyBranch {
    /// 모든 지지 배열
    pub const ALL: [EarthlyBranch; 12] = [
        Self::Zi, Self::Chou, Self::Yin, Self::Mao, Self::Chen, Self::Si,
        Self::Wu, Self::Wei, Self::Shen, Self::You, Self::Xu, Self::Hai,
    ];

    /// 한자 표기
    pub const HANJA: [&'static str; 12] = [
        "子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"
    ];

    /// 한글 표기
    pub const HANGUL: [&'static str; 12] = [
        "자", "축", "인", "묘", "진", "사", "오", "미", "신", "유", "술", "해"
    ];

    /// 띠 동물 (한글)
    pub const ZODIAC_HANGUL: [&'static str; 12] = [
        "쥐", "소", "호랑이", "토끼", "용", "뱀", "말", "양", "원숭이", "닭", "개", "돼지"
    ];

    /// 인덱스 (0-11)
    #[inline]
    pub const fn index(self) -> u8 {
        self as u8
    }

    /// 인덱스로부터 지지 생성 (0-11, 그 외는 모듈로 연산)
    #[inline]
    pub const fn from_index(idx: i32) -> Self {
        Self::ALL[idx.rem_euclid(12) as usize]
    }

    /// 오행 반환 (지지는 장간의 정기 기준)
    #[inline]
    pub const fn element(self) -> Element {
        match self {
            Self::Yin | Self::Mao => Element::Wood,
            Self::Si | Self::Wu => Element::Fire,
            Self::Chou | Self::Chen | Self::Wei | Self::Xu => Element::Earth,
            Self::Shen | Self::You => Element::Metal,
            Self::Hai | Self::Zi => Element::Water,
        }
    }

    /// 음양 반환
    #[inline]
    pub const fn polarity(self) -> Polarity {
        match self.index() % 2 {
            0 => Polarity::Yang,
            _ => Polarity::Yin,
        }
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

    /// 띠 동물 (한글)
    #[inline]
    pub const fn zodiac(self) -> &'static str {
        Self::ZODIAC_HANGUL[self.index() as usize]
    }

    /// 시주 계산용: 시간(0-23)으로부터 지지 반환
    /// 
    /// 자시(子時): 23:00-00:59 → Zi
    /// 축시(丑時): 01:00-02:59 → Chou
    /// ...
    #[inline]
    pub const fn from_hour(hour: u8) -> Self {
        // 23시-00시는 자시, 1-2시는 축시, ...
        let adjusted = (hour + 1) % 24;
        Self::from_index((adjusted / 2) as i32)
    }

    /// 월주 계산용: 월(1-12, 음력 기준)으로부터 지지 반환
    /// 
    /// 인월(1월) → 寅, 묘월(2월) → 卯, ...
    #[inline]
    pub const fn from_month(month: u8) -> Self {
        // 음력 1월 = 인월(寅)
        Self::from_index((month as i32 + 1) % 12)
    }

    /// 다음 지지
    #[inline]
    pub const fn next(self) -> Self {
        Self::from_index(self.index() as i32 + 1)
    }

    /// 이전 지지
    #[inline]
    pub const fn prev(self) -> Self {
        Self::from_index(self.index() as i32 - 1)
    }
}

impl std::fmt::Display for EarthlyBranch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hanja())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_branch_index() {
        assert_eq!(EarthlyBranch::Zi.index(), 0);
        assert_eq!(EarthlyBranch::Hai.index(), 11);
    }

    #[test]
    fn test_branch_from_index() {
        assert_eq!(EarthlyBranch::from_index(0), EarthlyBranch::Zi);
        assert_eq!(EarthlyBranch::from_index(12), EarthlyBranch::Zi); // 순환
        assert_eq!(EarthlyBranch::from_index(-1), EarthlyBranch::Hai); // 음수
    }

    #[test]
    fn test_branch_from_hour() {
        assert_eq!(EarthlyBranch::from_hour(23), EarthlyBranch::Zi); // 자시
        assert_eq!(EarthlyBranch::from_hour(0), EarthlyBranch::Zi);  // 자시
        assert_eq!(EarthlyBranch::from_hour(1), EarthlyBranch::Chou); // 축시
        assert_eq!(EarthlyBranch::from_hour(12), EarthlyBranch::Wu);  // 오시
    }

    #[test]
    fn test_branch_element() {
        assert_eq!(EarthlyBranch::Yin.element(), Element::Wood);
        assert_eq!(EarthlyBranch::Wu.element(), Element::Fire);
        assert_eq!(EarthlyBranch::Chen.element(), Element::Earth);
        assert_eq!(EarthlyBranch::Shen.element(), Element::Metal);
        assert_eq!(EarthlyBranch::Zi.element(), Element::Water);
    }

    #[test]
    fn test_branch_zodiac() {
        assert_eq!(EarthlyBranch::Zi.zodiac(), "쥐");
        assert_eq!(EarthlyBranch::Yin.zodiac(), "호랑이");
        assert_eq!(EarthlyBranch::Chen.zodiac(), "용");
    }
}
