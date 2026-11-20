//! Zi Wei Dou Shu Decadal Luck (Da Xian) module.
//!
//! Determines forward/reverse direction based on gender and stem polarity, setting age ranges and decadal stems.

use crate::palace::{fix_index, get_palace_stem, zwds_idx_to_std_idx};
use crate::types::{DaXian, FiveElementsClass, PalaceIndex};
use eon_core::birth::Gender;
use eon_saju::core::branch::EarthlyBranch;
use eon_saju::core::stem::HeavenlyStem;

/// Whether a chart runs 順行 (following the branch order) for 大限 and the 博士/長生 gods:
/// 陽男·陰女 forward, 陰男·陽女 backward. Yang stems are 甲丙戊庚壬 (even index).
pub fn is_forward_direction(is_male: bool, year_stem: HeavenlyStem) -> bool {
    let is_yang_stem = year_stem.index().is_multiple_of(2);
    if is_male {
        is_yang_stem
    } else {
        !is_yang_stem
    }
}

/// Generate decadal luck periods list.
///
/// * `gender` - Gender (Male/Female)
/// * `year_stem` - Birth year Heavenly Stem
/// * `soul_idx` -  ZWDS  
/// * `five_elements` -
pub fn calculate_daxian(
    gender: Gender,
    year_stem: HeavenlyStem,
    soul_idx: PalaceIndex,
    five_elements: FiveElementsClass,
) -> Vec<DaXian> {
    let mut daxian_list = Vec::new();

    let is_forward = is_forward_direction(matches!(gender, Gender::Male), year_stem);

    let step = if is_forward { 1 } else { -1 };
    let start_age = five_elements.starting_age() as u32;

    for i in 0..12 {
        let age_start = start_age + i * 10;
        let age_end = age_start + 9;

        // (soul_idx)  /
        let p_idx = fix_index(soul_idx as i32 + step * i as i32);

        //
        let stem = get_palace_stem(year_stem, p_idx);
        let branch_std = zwds_idx_to_std_idx(p_idx);
        let branch = EarthlyBranch::from_index(branch_std as i32);

        daxian_list.push(DaXian {
            index: i,
            age_start,
            age_end,
            palace_idx: p_idx,
            stem_hanja: stem.hanja().to_string(),
            branch_hanja: branch.hanja().to_string(),
        });
    }

    daxian_list
}
