//! Zi Wei Dou Shu Decadal Luck (Da Xian) module.
//!
//! Determines forward/reverse direction based on gender and stem polarity, setting age ranges and decadal stems.

use crate::palace::{fix_index, get_palace_stem, zwds_idx_to_std_idx};
use crate::types::{DaXian, FiveElementsClass, PalaceIndex};
use eon_core::birth::Gender;
use eon_saju::core::branch::EarthlyBranch;
use eon_saju::core::stem::HeavenlyStem;

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

    // 1. /
    // (陽男陰女) -> , (陰男陽女) ->
    // : 甲, 丙, 戊, 庚, 壬 (index % 2 == 0)
    // : 乙, 丁, 己, 辛, 癸 (index % 2 == 1)
    let is_yang_stem = year_stem.index().is_multiple_of(2);
    let is_male = matches!(gender, Gender::Male);

    let is_forward = if is_male {
        is_yang_stem // Yang Male -> forward, Yin Male -> reverse
    } else {
        !is_yang_stem // Yin Female -> forward, Yang Female -> reverse
    };

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
