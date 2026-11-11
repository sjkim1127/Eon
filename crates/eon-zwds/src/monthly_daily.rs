//! Zi Wei Dou Shu Monthly (Liu Yue) and Daily (Liu Ri) dynamic luck module.

use crate::transformations::get_sihua_stars;
use crate::types::{LiuRi, LiuYue, PalaceIndex};
use eon_saju::core::branch::EarthlyBranch;
use eon_saju::core::stem::HeavenlyStem;

/// Calculate monthly luck (Liu Yue) for target year and month.
///
/// * `annual_palace_idx` - Target annual palace index
/// * `annual_stem` - Target annual Heavenly Stem
/// * `target_month` - Target lunar month (1~12)
pub fn calculate_liuyue(
    annual_palace_idx: PalaceIndex,
    annual_stem: HeavenlyStem,
    target_month: u32,
) -> LiuYue {
    //    = (  + (target_month - 1)) % 12
    let palace_idx = (annual_palace_idx + (target_month as usize - 1)) % 12;

    // (月幹)  ( 五虎遁)
    let start_stem_idx = match annual_stem {
        HeavenlyStem::Jia | HeavenlyStem::Ji => 2,   // 丙寅
        HeavenlyStem::Yi | HeavenlyStem::Geng => 4,  // 戊寅
        HeavenlyStem::Bing | HeavenlyStem::Xin => 6, // 庚寅
        HeavenlyStem::Ding | HeavenlyStem::Ren => 8, // 壬寅
        HeavenlyStem::Wu | HeavenlyStem::Gui => 0,   // 甲寅
    };

    let month_stem_idx = (start_stem_idx + (target_month as usize - 1)) % 10;
    let month_stem = HeavenlyStem::from_index(month_stem_idx as i32);

    let branch_std_idx = (palace_idx + 2) % 12; // ZWDS 0=寅 -> std 2=寅
    let month_branch = EarthlyBranch::from_index(branch_std_idx as i32);

    let si_hua = get_sihua_stars(month_stem);

    LiuYue {
        month: target_month,
        palace_idx,
        stem_hanja: month_stem.hanja().to_string(),
        branch_hanja: month_branch.hanja().to_string(),
        si_hua,
    }
}

///  //  (LiuRi)  .
///
/// * `liuyue_palace_idx` -   
/// * `target_day` -    (1~30)
pub fn calculate_liuri(liuyue_palace_idx: PalaceIndex, target_day: u32) -> LiuRi {
    //    = (  + (target_day - 1)) % 12
    let palace_idx = (liuyue_palace_idx + (target_day as usize - 1)) % 12;

    let day_stem_idx = (target_day as usize - 1) % 10;
    let day_stem = HeavenlyStem::from_index(day_stem_idx as i32);

    let branch_std_idx = (palace_idx + 2) % 12;
    let day_branch = EarthlyBranch::from_index(branch_std_idx as i32);

    let si_hua = get_sihua_stars(day_stem);

    LiuRi {
        day: target_day,
        palace_idx,
        stem_hanja: day_stem.hanja().to_string(),
        branch_hanja: day_branch.hanja().to_string(),
        si_hua,
    }
}
