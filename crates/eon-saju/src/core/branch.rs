//! Earthly Branches (地支) - 12 Earthly Branches
//!
//! Zi, Chou, Yin, Mao, Chen, Si, Wu, Wei, Shen, You, Xu, Hai

use crate::core::element::{Element, Polarity};
use serde::{Deserialize, Serialize};

/// Earthly Branches (地支) - 12 branches
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum EarthlyBranch {
    /// Zi (子) - Rat - Yang Water
    Zi = 0,
    /// Chou (丑) - Ox - Yin Earth
    Chou = 1,
    /// Yin (寅) - Tiger - Yang Wood
    Yin = 2,
    /// Mao (卯) - Rabbit - Yin Wood
    Mao = 3,
    /// Chen (辰) - Dragon - Yang Earth
    Chen = 4,
    /// Si (巳) - Snake - Yin Fire
    Si = 5,
    /// Wu (午) - Horse - Yang Fire
    Wu = 6,
    /// Wei (未) - Goat - Yin Earth
    Wei = 7,
    /// Shen (申) - Monkey - Yang Metal
    Shen = 8,
    /// You (酉) - Rooster - Yin Metal
    You = 9,
    /// Xu (戌) - Dog - Yang Earth
    Xu = 10,
    /// Hai (亥) - Pig - Yin Water
    Hai = 11,
}

impl EarthlyBranch {
    /// All branches array
    pub const ALL: [EarthlyBranch; 12] = [
        Self::Zi,
        Self::Chou,
        Self::Yin,
        Self::Mao,
        Self::Chen,
        Self::Si,
        Self::Wu,
        Self::Wei,
        Self::Shen,
        Self::You,
        Self::Xu,
        Self::Hai,
    ];

    /// Hanja notation
    pub const HANJA: [&'static str; 12] = [
        "子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥",
    ];

    /// Romanized/English notation
    pub const HANGUL: [&'static str; 12] = [
        "Zi", "Chou", "Yin", "Mao", "Chen", "Si", "Wu", "Wei", "Shen", "You", "Xu", "Hai",
    ];

    /// Zodiac animal (English)
    pub const ZODIAC: [&'static str; 12] = [
        "Rat", "Ox", "Tiger", "Rabbit", "Dragon", "Snake", "Horse", "Goat", "Monkey", "Rooster",
        "Dog", "Pig",
    ];

    /// Zodiac alias
    pub const ZODIAC_HANGUL: [&'static str; 12] = Self::ZODIAC;

    /// Index (0-11)
    #[inline]
    pub const fn index(self) -> u8 {
        self as u8
    }

    /// Construct from index (0-11, modulo wrapping)
    #[inline]
    pub const fn from_index(idx: i32) -> Self {
        Self::ALL[idx.rem_euclid(12) as usize]
    }

    /// Associated Five Element (primary stem based)
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

    /// Zodiac animal
    #[inline]
    pub const fn zodiac(self) -> &'static str {
        Self::ZODIAC[self.index() as usize]
    }

    /// Hour branch calculation from solar hour (0-23)
    ///
    /// 23:00-00:59 -> Zi
    /// 01:00-02:59 -> Chou
    /// ...
    #[inline]
    pub const fn from_hour(hour: u8) -> Self {
        let adjusted = (hour + 1) % 24;
        Self::from_index((adjusted / 2) as i32)
    }

    /// Month branch calculation from lunar/solar month (1-12)
    ///
    /// 1st month -> Yin (寅), 2nd month -> Mao (卯), ...
    #[inline]
    pub const fn from_month(month: u8) -> Self {
        Self::from_index((month as i32 + 1) % 12)
    }

    /// Next branch
    #[inline]
    pub const fn next(self) -> Self {
        Self::from_index(self.index() as i32 + 1)
    }

    /// Previous branch
    #[inline]
    pub const fn prev(self) -> Self {
        Self::from_index(self.index() as i32 - 1)
    }

    /// Returns Hidden Stems (地藏干) in order: Remnant (餘氣), Middle (中氣), Main (正氣).
    pub fn jijanggan(self) -> Vec<crate::core::stem::HeavenlyStem> {
        use crate::core::stem::HeavenlyStem::*;
        match self {
            Self::Zi => vec![Ren, Gui],
            Self::Chou => vec![Gui, Xin, Ji],
            Self::Yin => vec![Wu, Bing, Jia],
            Self::Mao => vec![Jia, Yi],
            Self::Chen => vec![Yi, Gui, Wu],
            Self::Si => vec![Wu, Geng, Bing],
            Self::Wu => vec![Bing, Ji, Ding],
            Self::Wei => vec![Ding, Yi, Ji],
            Self::Shen => vec![Wu, Ren, Geng],
            Self::You => vec![Geng, Xin],
            Self::Xu => vec![Xin, Ding, Wu],
            Self::Hai => vec![Wu, Jia, Ren],
        }
    }

    /// Hidden Stems alias
    pub fn hidden_stems(self) -> Vec<crate::core::stem::HeavenlyStem> {
        self.jijanggan()
    }

    /// Remnant Stem (餘氣)
    pub fn yeogi(self) -> crate::core::stem::HeavenlyStem {
        self.jijanggan()[0]
    }

    /// Middle Stem (中氣)
    pub fn junggi(self) -> crate::core::stem::HeavenlyStem {
        let stems = self.jijanggan();
        if stems.len() == 3 {
            stems[1]
        } else {
            stems[stems.len() - 1]
        }
    }

    /// Main Stem (正氣)
    pub fn jeonggi(self) -> crate::core::stem::HeavenlyStem {
        let stems = self.jijanggan();
        stems[stems.len() - 1]
    }

    /// Primary stem alias
    pub fn primary_stem(self) -> crate::core::stem::HeavenlyStem {
        self.jeonggi()
    }

    /// Season
    pub fn season(self) -> Season {
        match self {
            Self::Yin | Self::Mao | Self::Chen => Season::Spring,
            Self::Si | Self::Wu | Self::Wei => Season::Summer,
            Self::Shen | Self::You | Self::Xu => Season::Autumn,
            Self::Hai | Self::Zi | Self::Chou => Season::Winter,
        }
    }
}

/// Season
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
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
        assert_eq!(EarthlyBranch::from_index(12), EarthlyBranch::Zi); // wrap
        assert_eq!(EarthlyBranch::from_index(-1), EarthlyBranch::Hai); // negative
    }

    #[test]
    fn test_branch_from_hour() {
        assert_eq!(EarthlyBranch::from_hour(23), EarthlyBranch::Zi);
        assert_eq!(EarthlyBranch::from_hour(0), EarthlyBranch::Zi);
        assert_eq!(EarthlyBranch::from_hour(1), EarthlyBranch::Chou);
        assert_eq!(EarthlyBranch::from_hour(12), EarthlyBranch::Wu);
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
        assert_eq!(EarthlyBranch::Zi.zodiac(), "Rat");
        assert_eq!(EarthlyBranch::Yin.zodiac(), "Tiger");
        assert_eq!(EarthlyBranch::Chen.zodiac(), "Dragon");
    }
}
