//! Sexagenary Cycle (干支, GanZi) - 60 Pillar Combinations
//!
//! Combines 10 Heavenly Stems and 12 Earthly Branches into 60 pillars.
//! Cycles from JiaZi (甲子) to GuiHai (癸亥).

use crate::core::branch::EarthlyBranch;
use crate::core::element::{Element, Polarity};
use crate::core::stem::HeavenlyStem;
use serde::{Deserialize, Serialize};

/// Sexagenary Cycle Pair (干支) - Combination of Heavenly Stem and Earthly Branch
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GanZi {
    /// Heavenly Stem
    pub stem: HeavenlyStem,
    /// Earthly Branch
    pub branch: EarthlyBranch,
}

impl GanZi {
    /// 60 Cycle array (JiaZi ~ GuiHai)
    pub const SIXTY_CYCLE: [(HeavenlyStem, EarthlyBranch); 60] = Self::generate_sixty_cycle();

    /// Generate 60 cycle array (const fn)
    const fn generate_sixty_cycle() -> [(HeavenlyStem, EarthlyBranch); 60] {
        let mut result = [(HeavenlyStem::Jia, EarthlyBranch::Zi); 60];
        let mut i = 0;
        while i < 60 {
            result[i] = (
                HeavenlyStem::from_index(i as i32),
                EarthlyBranch::from_index(i as i32),
            );
            i += 1;
        }
        result
    }

    /// Construct new GanZi
    #[inline]
    pub const fn new(stem: HeavenlyStem, branch: EarthlyBranch) -> Self {
        Self { stem, branch }
    }

    /// Construct from 60-cycle index (0-59, cyclic)
    #[inline]
    pub const fn from_index(idx: i32) -> Self {
        let normalized = idx.rem_euclid(60);
        let (stem, branch) = Self::SIXTY_CYCLE[normalized as usize];
        Self { stem, branch }
    }

    /// Construct yearly GanZi from solar year
    ///
    /// Epoch: 4 CE = JiaZi year (index 0)
    #[inline]
    pub const fn from_year(year: i32) -> Self {
        Self::from_index(year - 4)
    }

    /// Return index within 60 cycle (0-59)
    ///
    /// JiaZi=0, YiChou=1, ..., GuiHai=59
    pub const fn index(self) -> u8 {
        let stem_idx = self.stem.index() as i32;
        let branch_idx = self.branch.index() as i32;
        let n = (6 * stem_idx - 5 * branch_idx).rem_euclid(60);
        n as u8
    }

    /// Stem element
    #[inline]
    pub const fn stem_element(self) -> Element {
        self.stem.element()
    }

    /// Branch element (primary stem)
    #[inline]
    pub const fn branch_element(self) -> Element {
        self.branch.element()
    }

    /// Polarity (matches between stem and branch)
    #[inline]
    pub const fn polarity(self) -> Polarity {
        self.stem.polarity()
    }

    /// Hanja notation (e.g. "甲子")
    pub fn hanja(&self) -> String {
        format!("{}{}", self.stem.hanja(), self.branch.hanja())
    }

    /// English name notation (e.g. "JiaZi")
    pub fn hangul(&self) -> String {
        format!("{}{}", self.stem.hangul(), self.branch.hangul())
    }

    /// English name alias
    pub fn english(&self) -> String {
        self.hangul()
    }

    /// Next GanZi (cyclic)
    #[inline]
    pub const fn next(self) -> Self {
        Self::from_index(self.index() as i32 + 1)
    }

    /// Previous GanZi (cyclic)
    #[inline]
    pub const fn prev(self) -> Self {
        Self::from_index(self.index() as i32 - 1)
    }

    /// Compute distance to another GanZi
    #[inline]
    pub const fn distance_to(self, other: Self) -> i32 {
        (other.index() as i32 - self.index() as i32).rem_euclid(60)
    }
}

impl std::fmt::Display for GanZi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.stem, self.branch)
    }
}

impl Default for GanZi {
    fn default() -> Self {
        Self::new(HeavenlyStem::Jia, EarthlyBranch::Zi)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ganzi_index() {
        // JiaZi = 0
        let jiazi = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Zi);
        assert_eq!(jiazi.index(), 0);

        // YiChou = 1
        let yichou = GanZi::new(HeavenlyStem::Yi, EarthlyBranch::Chou);
        assert_eq!(yichou.index(), 1);

        // GuiHai = 59
        let guihai = GanZi::new(HeavenlyStem::Gui, EarthlyBranch::Hai);
        assert_eq!(guihai.index(), 59);

        // JiaYin = 50
        let jiayin = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Yin);
        assert_eq!(jiayin.index(), 50);
    }

    #[test]
    fn test_ganzi_from_index() {
        assert_eq!(
            GanZi::from_index(0),
            GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Zi)
        );
        assert_eq!(
            GanZi::from_index(59),
            GanZi::new(HeavenlyStem::Gui, EarthlyBranch::Hai)
        );
        assert_eq!(
            GanZi::from_index(60),
            GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Zi)
        );
        assert_eq!(
            GanZi::from_index(-1),
            GanZi::new(HeavenlyStem::Gui, EarthlyBranch::Hai)
        );
    }

    #[test]
    fn test_ganzi_sixty_cycle() {
        for i in 0..60 {
            let ganzi = GanZi::from_index(i);
            assert_eq!(ganzi.index() as i32, i);
        }
    }

    #[test]
    fn test_ganzi_next_prev() {
        let jiazi = GanZi::from_index(0);
        assert_eq!(jiazi.next(), GanZi::from_index(1));
        assert_eq!(jiazi.prev(), GanZi::from_index(59));
    }

    #[test]
    fn test_ganzi_display() {
        let jiazi = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Zi);
        assert_eq!(jiazi.to_string(), "甲子");
        assert_eq!(jiazi.hangul(), "JiaZi");
    }

    #[test]
    fn test_ganzi_polarity() {
        let jiazi = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Zi);
        assert_eq!(jiazi.polarity(), Polarity::Yang);

        let yichou = GanZi::new(HeavenlyStem::Yi, EarthlyBranch::Chou);
        assert_eq!(yichou.polarity(), Polarity::Yin);
    }
}
