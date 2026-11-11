//! Zi Wei Dou Shu star placement root module.
//!
//! Aggregates major, auxiliary, and minor star placements into a position map.

pub mod assistant;
pub mod location;
pub mod major;

use crate::types::{LunarBirthInfo, PalaceIndex, ZwdsStar};
use eon_saju::core::stem::HeavenlyStem;
use std::collections::HashMap;

/// Calculate positions of all ZWDS stars into a map.
///
/// * `lunar` - Lunar birth info
/// * `five_elements_val` - Five Elements Bureau value
/// * `soul_idx` -  
/// * `body_idx` -  
pub fn place_all_stars(
    lunar: &LunarBirthInfo,
    five_elements_val: u32,
    soul_idx: PalaceIndex,
    body_idx: PalaceIndex,
) -> HashMap<ZwdsStar, PalaceIndex> {
    let mut stars = HashMap::new();

    let year_stem = HeavenlyStem::from_index(lunar.year_stem_idx as i32);

    // 1. 14
    let ziwei_idx = major::get_ziwei_index(lunar.day, five_elements_val);
    let tianfu_idx = major::get_tianfu_index(ziwei_idx);
    let major_stars = major::place_14_main_stars(ziwei_idx, tianfu_idx);
    stars.extend(major_stars);

    // 2. 6
    let (wenchang, wenqu) = assistant::place_wenchang_wenqu(lunar.time_branch_idx);
    stars.insert(ZwdsStar::WenChang, wenchang);
    stars.insert(ZwdsStar::WenQu, wenqu);

    let (zuofu, youbi) = assistant::place_zuofu_youbi(lunar.month);
    stars.insert(ZwdsStar::ZuoFu, zuofu);
    stars.insert(ZwdsStar::YouBi, youbi);

    let (tiankui, tianyue) = assistant::place_tiankui_tianyue(year_stem);
    stars.insert(ZwdsStar::TianKui, tiankui);
    stars.insert(ZwdsStar::TianYue, tianyue);

    // 3. , ,
    let (lucun, qingyang, tuoluo) = location::place_lucun_qingyang_tuoluo(year_stem);
    stars.insert(ZwdsStar::LuCun, lucun);
    stars.insert(ZwdsStar::QingYang, qingyang);
    stars.insert(ZwdsStar::TuoLuo, tuoluo);

    // 4.
    let tianma = location::place_tianma(lunar.year_branch_idx);
    stars.insert(ZwdsStar::TianMa, tianma);

    // 5. ,
    let (huoxing, lingxing) =
        location::place_huoxing_lingxing(lunar.year_branch_idx, lunar.time_branch_idx);
    stars.insert(ZwdsStar::HuoXing, huoxing);
    stars.insert(ZwdsStar::LingXing, lingxing);

    // 6. ,
    let (dijie, dikong) = location::place_dijie_dikong(lunar.time_branch_idx);
    stars.insert(ZwdsStar::DiJie, dijie);
    stars.insert(ZwdsStar::DiKong, dikong);

    // 7.
    for (star, idx) in location::place_month_stars(lunar.month) {
        stars.insert(star, idx);
    }

    // 8.
    for (star, idx) in location::place_hour_stars(lunar.time_branch_idx) {
        stars.insert(star, idx);
    }

    // 9.
    for (star, idx) in location::place_day_stars(lunar.day, zuofu, youbi, wenchang, wenqu) {
        stars.insert(star, idx);
    }

    // 10.
    for (star, idx) in location::place_year_branch_stars(lunar.year_branch_idx) {
        stars.insert(star, idx);
    }

    // 11.
    for (star, idx) in
        location::place_year_stem_stars(year_stem, lunar.year_branch_idx, soul_idx, body_idx)
    {
        stars.insert(star, idx);
    }

    stars
}
