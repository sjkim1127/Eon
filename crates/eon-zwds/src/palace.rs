//! Zi Wei Dou Shu Palace calculation module.
//!
//! Computes Ming/Body palaces, Five Elements Bureau, and palace naming.

use crate::types::{FiveElementsClass, PalaceIndex, PalaceName};
use eon_saju::core::branch::EarthlyBranch;
use eon_saju::core::stem::HeavenlyStem;

/// Normalize index to 0..11 range
pub fn fix_index(i: i32) -> usize {
    let r = i % 12;
    if r < 0 {
        (r + 12) as usize
    } else {
        r as usize
    }
}

/// ZWDS branch index (0=Yin) -> Standard branch index (0=Zi)
pub fn zwds_idx_to_std_idx(zwds_idx: usize) -> usize {
    (zwds_idx + 2) % 12
}

/// Standard branch index (0=Zi) -> ZWDS branch index (0=Yin)
pub fn std_idx_to_zwds_idx(std_idx: usize) -> usize {
    (std_idx + 10) % 12
}

/// Calculate Ming (Soul) and Shen (Body) palace indices in ZWDS coordinates.
///
/// * `lunar_month` - Lunar birth month (1~12)
/// * `time_branch_std_idx` - Standard birth hour branch index (0=Zi)
pub fn get_soul_and_body_index(
    lunar_month: u32,
    time_branch_std_idx: usize,
) -> (PalaceIndex, PalaceIndex) {
    let month_idx = (lunar_month - 1) as i32;
    let time_idx = time_branch_std_idx as i32;

    let soul = fix_index(month_idx - time_idx);
    let body = fix_index(month_idx + time_idx);

    (soul, body)
}

/// Determine Heavenly Stem of each palace using Five Tigers method (Wu Hu Dun).
///
/// * `year_stem` - Birth year Heavenly Stem
/// * `palace_zwds_idx` - ZWDS palace branch index (0=Yin, 1=Mao...)
pub fn get_palace_stem(year_stem: HeavenlyStem, palace_zwds_idx: PalaceIndex) -> HeavenlyStem {
    let start_stem_idx = match year_stem {
        HeavenlyStem::Jia | HeavenlyStem::Ji => 2,   // Bing
        HeavenlyStem::Yi | HeavenlyStem::Geng => 4,  // Wu
        HeavenlyStem::Bing | HeavenlyStem::Xin => 6, // Geng
        HeavenlyStem::Ding | HeavenlyStem::Ren => 8, // Ren
        HeavenlyStem::Wu | HeavenlyStem::Gui => 0,   // Jia
    };
    let stem_idx = (start_stem_idx + palace_zwds_idx) % 10;
    HeavenlyStem::from_index(stem_idx as i32)
}

/// Calculate Five Elements Bureau (Wu Xing Ju).
///
/// * `soul_stem` - Ming palace Heavenly Stem
/// * `soul_zwds_idx` - Ming palace ZWDS branch index
pub fn get_five_elements(soul_stem: HeavenlyStem, soul_zwds_idx: PalaceIndex) -> FiveElementsClass {
    let stem_num = match soul_stem {
        HeavenlyStem::Jia | HeavenlyStem::Yi => 1,
        HeavenlyStem::Bing | HeavenlyStem::Ding => 2,
        HeavenlyStem::Wu | HeavenlyStem::Ji => 3,
        HeavenlyStem::Geng | HeavenlyStem::Xin => 4,
        HeavenlyStem::Ren | HeavenlyStem::Gui => 5,
    };

    let soul_branch_std_idx = zwds_idx_to_std_idx(soul_zwds_idx);
    let branch = EarthlyBranch::from_index(soul_branch_std_idx as i32);

    let branch_num = match branch {
        EarthlyBranch::Zi | EarthlyBranch::Wu | EarthlyBranch::Chou | EarthlyBranch::Wei => 1,
        EarthlyBranch::Yin | EarthlyBranch::Shen | EarthlyBranch::Mao | EarthlyBranch::You => 2,
        EarthlyBranch::Chen | EarthlyBranch::Xu | EarthlyBranch::Si | EarthlyBranch::Hai => 3,
    };

    let sum = stem_num + branch_num;
    let rem = sum % 5;

    match rem {
        1 => FiveElementsClass::Wood3,
        2 => FiveElementsClass::Metal4,
        3 => FiveElementsClass::Water2,
        4 => FiveElementsClass::Fire6,
        0 => FiveElementsClass::Earth5,
        _ => unreachable!(),
    }
}

/// Determine PalaceName from Ming palace offset.
///
/// * `soul_zwds_idx` - Ming palace ZWDS branch index
/// * `palace_zwds_idx` - Target palace ZWDS branch index
pub fn get_palace_name(soul_zwds_idx: PalaceIndex, palace_zwds_idx: PalaceIndex) -> PalaceName {
    let offset = fix_index(palace_zwds_idx as i32 - soul_zwds_idx as i32);
    PalaceName::ALL[offset]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soul_body_calc() {
        // Lunar month 10, Hai hour (lunar_month = 10, time_branch_std_idx = 11)
        let (soul, body) = get_soul_and_body_index(10, 11);
        // soul = 9 - 11 = -2 -> 10 (Zi)
        // body = 9 + 11 = 20 -> 8 (Xu)
        assert_eq!(soul, 10);
        assert_eq!(body, 8);
    }

    #[test]
    fn test_palace_stem() {
        // Jia year, Yin (0) palace -> Bing
        assert_eq!(get_palace_stem(HeavenlyStem::Jia, 0), HeavenlyStem::Bing);
        assert_eq!(get_palace_stem(HeavenlyStem::Jia, 10), HeavenlyStem::Bing);
    }

    #[test]
    fn test_five_elements() {
        // Ming stem: Bing, Ming ZWDS: 10 (Zi)
        let class = get_five_elements(HeavenlyStem::Bing, 10);
        assert_eq!(class, FiveElementsClass::Water2);
    }
}
