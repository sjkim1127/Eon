//! Zi Wei Dou Shu Annual Luck (Liu Nian) module.
//!
//! Computes annual Tai Sui stem/branch, annual palace, and annual Si Hua.

use crate::palace::std_idx_to_zwds_idx;
use crate::transformations::get_sihua_stars;
use crate::types::LiuNian;
use eon_saju::core::branch::EarthlyBranch;
use eon_saju::core::stem::HeavenlyStem;

/// Calculate annual luck (Liu Nian) for a target year.
///
/// * `target_year` - Target annual year (e.g. 2026)
pub fn calculate_liunian(target_year: i32) -> LiuNian {
    // Calculate GanZi based on 60-cycle (Jia-Zi = 4 AD)
    let stem_idx = (target_year - 4).rem_euclid(10) as usize;
    let branch_std_idx = (target_year - 4).rem_euclid(12) as usize;

    let stem = HeavenlyStem::from_index(stem_idx as i32);
    let branch = EarthlyBranch::from_index(branch_std_idx as i32);

    let palace_idx = std_idx_to_zwds_idx(branch_std_idx);
    let si_hua = get_sihua_stars(stem);

    let (liu_lu, liu_yang, liu_tuo) = crate::stars::location::place_lucun_qingyang_tuoluo(stem);
    let (liu_chang, liu_qu) = match stem {
        HeavenlyStem::Jia => (3, 7),
        HeavenlyStem::Yi => (4, 6),
        HeavenlyStem::Bing => (6, 4),
        HeavenlyStem::Ding => (7, 3),
        HeavenlyStem::Wu => (6, 4),
        HeavenlyStem::Ji => (7, 3),
        HeavenlyStem::Geng => (9, 1),
        HeavenlyStem::Xin => (10, 0),
        HeavenlyStem::Ren => (0, 10),
        HeavenlyStem::Gui => (1, 9),
    };

    LiuNian {
        year: target_year,
        palace_idx,
        stem_hanja: stem.hanja().to_string(),
        branch_hanja: branch.hanja().to_string(),
        si_hua,
        liu_lu,
        liu_yang,
        liu_tuo,
        liu_chang,
        liu_qu,
    }
}
