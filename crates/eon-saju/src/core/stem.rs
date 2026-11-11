//! Heavenly Stems (天干) - 10 Heavenly Stems
//!
//! Jia, Yi, Bing, Ding, Wu, Ji, Geng, Xin, Ren, Gui

use crate::core::element::{Element, Polarity};
use serde::{Deserialize, Serialize};

/// Heavenly Stems (天干) - 10 stems
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum HeavenlyStem {
    /// Jia (甲) - Yang Wood
    Jia = 0,
    /// Yi (乙) - Yin Wood
    Yi = 1,
    /// Bing (丙) - Yang Fire
    Bing = 2,
    /// Ding (丁) - Yin Fire
    Ding = 3,
    /// Wu (戊) - Yang Earth
    Wu = 4,
    /// Ji (己) - Yin Earth
    Ji = 5,
    /// Geng (庚) - Yang Metal
    Geng = 6,
    /// Xin (辛) - Yin Metal
    Xin = 7,
    /// Ren (壬) - Yang Water
    Ren = 8,
    /// Gui (癸) - Yin Water
    Gui = 9,
}

impl HeavenlyStem {
    /// All stems array
    pub const ALL: [HeavenlyStem; 10] = [
        Self::Jia,
        Self::Yi,
        Self::Bing,
        Self::Ding,
        Self::Wu,
        Self::Ji,
        Self::Geng,
        Self::Xin,
        Self::Ren,
        Self::Gui,
    ];

    /// Hanja notation
    pub const HANJA: [&'static str; 10] =
        ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];

    /// Romanized/English notation
    pub const HANGUL: [&'static str; 10] = [
        "Jia", "Yi", "Bing", "Ding", "Wu", "Ji", "Geng", "Xin", "Ren", "Gui",
    ];

    /// Index (0-9)
    #[inline]
    pub const fn index(self) -> u8 {
        self as u8
    }

    /// Construct from index (0-9, modulo wrapping)
    #[inline]
    pub const fn from_index(idx: i32) -> Self {
        Self::ALL[idx.rem_euclid(10) as usize]
    }

    /// Associated Five Element
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

    /// Associated Yin/Yang Polarity
    #[inline]
    pub const fn polarity(self) -> Polarity {
        match self.index() % 2 {
            0 => Polarity::Yang,
            _ => Polarity::Yin,
        }
    }

    /// Hanja notation
    #[inline]
    pub const fn hanja(self) -> &'static str {
        Self::HANJA[self.index() as usize]
    }

    /// Romanized/English notation
    #[inline]
    pub const fn hangul(self) -> &'static str {
        Self::HANGUL[self.index() as usize]
    }

    /// English name alias
    #[inline]
    pub const fn english(self) -> &'static str {
        Self::HANGUL[self.index() as usize]
    }

    /// Next stem
    #[inline]
    pub const fn next(self) -> Self {
        Self::from_index(self.index() as i32 + 1)
    }

    /// Previous stem
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
        assert_eq!(HeavenlyStem::from_index(10), HeavenlyStem::Jia); // wrap
        assert_eq!(HeavenlyStem::from_index(-1), HeavenlyStem::Gui); // negative
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
