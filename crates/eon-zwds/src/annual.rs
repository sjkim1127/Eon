//! 자미두수 유년(流年, 1년 운) 연산 모듈
//!
//! 특정 연도의 태세(천간/지지)를 구하고, 지지에 해당하는 궁을 유년 궁으로 지정하며, 천간에 따른 유년 사화를 계산합니다.

use crate::palace::std_idx_to_zwds_idx;
use crate::transformations::get_sihua_stars;
use crate::types::LiuNian;
use eon_saju::core::branch::EarthlyBranch;
use eon_saju::core::stem::HeavenlyStem;

/// 특정 연도에 대한 유년(LiuNian) 정보를 계산합니다.
///
/// * `target_year` - 운세를 보려는 연도 (예: 2026)
pub fn calculate_liunian(target_year: i32) -> LiuNian {
    // 60갑자 기반 천간, 지지 계산 (甲子년 = 서기 4년 기준)
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
