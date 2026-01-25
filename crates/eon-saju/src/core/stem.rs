//! 천간(天干, Heavenly Stems) - 10개의 천간
//!
//! 甲乙丙丁戊己庚辛壬癸

use serde::{Deserialize, Serialize};
use crate::core::element::{Element, Polarity};

/// 천간(天干) - 10개
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum HeavenlyStem {
    /// 甲 (갑) - 양목
    Jia = 0,
    /// 乙 (을) - 음목
    Yi = 1,
    /// 丙 (병) - 양화
    Bing = 2,
    /// 丁 (정) - 음화
    Ding = 3,
    /// 戊 (무) - 양토
    Wu = 4,
    /// 己 (기) - 음토
    Ji = 5,
    /// 庚 (경) - 양금
    Geng = 6,
    /// 辛 (신) - 음금
    Xin = 7,
    /// 壬 (임) - 양수
    Ren = 8,
    /// 癸 (계) - 음수
    Gui = 9,
}

impl HeavenlyStem {
    /// 모든 천간 배열
    pub const ALL: [HeavenlyStem; 10] = [
        Self::Jia, Self::Yi, Self::Bing, Self::Ding, Self::Wu,
        Self::Ji, Self::Geng, Self::Xin, Self::Ren, Self::Gui,
    ];

    /// 한자 표기
    pub const HANJA: [&'static str; 10] = [
        "甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"
    ];

    /// 한글 표기
    pub const HANGUL: [&'static str; 10] = [
        "갑", "을", "병", "정", "무", "기", "경", "신", "임", "계"
    ];

    /// 인덱스 (0-9)
    #[inline]
    pub const fn index(self) -> u8 {
        self as u8
    }

    /// 인덱스로부터 천간 생성 (0-9, 그 외는 모듈로 연산)
    #[inline]
    pub const fn from_index(idx: i32) -> Self {
        Self::ALL[idx.rem_euclid(10) as usize]
    }

    /// 오행 반환
    #[inline]
    pub const fn element(self) -> Element {
        match self {
            Self::Jia | Self::Yi => Element::Wood,
            Self::Bing | Self::Ding => Element::Fire,
            Self::Wu | Self::Ji => Element::Earth,
            Self::Geng | Self::Xin => Element::Metal,
            Self::Ren | Self::Gui => Element::Water,
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

    /// 다음 천간
    #[inline]
    pub const fn next(self) -> Self {
        Self::from_index(self.index() as i32 + 1)
    }

    /// 이전 천간
    #[inline]
    pub const fn prev(self) -> Self {
        Self::from_index(self.index() as i32 - 1)
    }
}

impl std::fmt::Display for HeavenlyStem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hanja())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stem_index() {
        assert_eq!(HeavenlyStem::Jia.index(), 0);
        assert_eq!(HeavenlyStem::Gui.index(), 9);
    }

    #[test]
    fn test_stem_from_index() {
        assert_eq!(HeavenlyStem::from_index(0), HeavenlyStem::Jia);
        assert_eq!(HeavenlyStem::from_index(10), HeavenlyStem::Jia); // 순환
        assert_eq!(HeavenlyStem::from_index(-1), HeavenlyStem::Gui); // 음수
    }

    #[test]
    fn test_stem_element() {
        assert_eq!(HeavenlyStem::Jia.element(), Element::Wood);
        assert_eq!(HeavenlyStem::Bing.element(), Element::Fire);
        assert_eq!(HeavenlyStem::Wu.element(), Element::Earth);
        assert_eq!(HeavenlyStem::Geng.element(), Element::Metal);
        assert_eq!(HeavenlyStem::Ren.element(), Element::Water);
    }

    #[test]
    fn test_stem_polarity() {
        assert_eq!(HeavenlyStem::Jia.polarity(), Polarity::Yang);
        assert_eq!(HeavenlyStem::Yi.polarity(), Polarity::Yin);
    }
}
