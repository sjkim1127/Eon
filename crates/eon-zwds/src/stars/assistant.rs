//! Zi Wei Dou Shu 6 Auxiliary Stars placement module.
//!
//! Positions WenChang, WenQu, ZuoFu, YouBi, TianKui, TianYue.

use crate::palace::fix_index;
use crate::types::PalaceIndex;
use eon_saju::core::stem::HeavenlyStem;

/// Calculate WenChang and WenQu branch index
///
/// * `time_branch_std_idx` - Standard birth hour branch index (0=Zi)
pub fn place_wenchang_wenqu(time_branch_std_idx: usize) -> (PalaceIndex, PalaceIndex) {
    let t = time_branch_std_idx as i32;
    // WenChang: Reverse from Xu (8)
    let wenchang = fix_index(8 - t);
    // : 辰(2)
    let wenqu = fix_index(2 + t);
    (wenchang, wenqu)
}

/// (左輔) (右弼) ZWDS   
///
/// * `lunar_month` -   (1~12)
pub fn place_zuofu_youbi(lunar_month: u32) -> (PalaceIndex, PalaceIndex) {
    let m = lunar_month as i32;
    // : 辰(2)
    let zuofu = fix_index(2 + m - 1);
    // : 戌(8)
    let youbi = fix_index(8 - m + 1);
    (zuofu, youbi)
}

/// (天魁) 鉞(天鉞) ZWDS   
///
/// * `year_stem` -  
pub fn place_tiankui_tianyue(year_stem: HeavenlyStem) -> (PalaceIndex, PalaceIndex) {
    match year_stem {
        HeavenlyStem::Jia | HeavenlyStem::Wu | HeavenlyStem::Geng => (11, 5), // 丑, 未
        HeavenlyStem::Yi | HeavenlyStem::Ji => (10, 6),                       // 子, 申
        HeavenlyStem::Xin => (4, 0),                                          // 午, 寅
        HeavenlyStem::Bing | HeavenlyStem::Ding => (9, 7),                    // 亥, 酉
        HeavenlyStem::Ren | HeavenlyStem::Gui => (1, 3),                      // 卯, 巳
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wenchang_wenqu() {
        // 亥 (time_branch_std_idx = 11)
        // : 8 - 11 = -3 -> 9 (亥)
        // : 2 + 11 = 13 -> 1 (卯)
        let (wc, wq) = place_wenchang_wenqu(11);
        assert_eq!(wc, 9);
        assert_eq!(wq, 1);
    }

    #[test]
    fn test_zuofu_youbi() {
        //  10
        // : 2 + 10 - 1 = 11 (丑)
        // : 8 - 10 + 1 = -1 -> 11 (丑)
        let (zf, yb) = place_zuofu_youbi(10);
        assert_eq!(zf, 11);
        assert_eq!(yb, 11);
    }
}
