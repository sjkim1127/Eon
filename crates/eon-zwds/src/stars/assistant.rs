//! 자미두수 6대 보조성(輔星) 배치 모듈
//!
//! 문창, 문곡, 좌보, 우필, 천괴, 천월의 위치를 계산합니다.

use crate::palace::fix_index;
use crate::types::PalaceIndex;
use eon_saju::core::stem::HeavenlyStem;

/// 문창(文昌)과 문곡(文曲)의 ZWDS 지지 인덱스 계산
///
/// * `time_branch_std_idx` - 표준 지지 인덱스 기준의 출생시 (0=子)
pub fn place_wenchang_wenqu(time_branch_std_idx: usize) -> (PalaceIndex, PalaceIndex) {
    let t = time_branch_std_idx as i32;
    // 문창: 戌궁(8)에서 역행
    let wenchang = fix_index(8 - t);
    // 문곡: 辰궁(2)에서 순행
    let wenqu = fix_index(2 + t);
    (wenchang, wenqu)
}

/// 좌보(左輔)와 우필(右弼)의 ZWDS 지지 인덱스 계산
///
/// * `lunar_month` - 음력 월 (1~12)
pub fn place_zuofu_youbi(lunar_month: u32) -> (PalaceIndex, PalaceIndex) {
    let m = lunar_month as i32;
    // 좌보: 辰궁(2)에서 음력 월수만큼 순행
    let zuofu = fix_index(2 + m - 1);
    // 우필: 戌궁(8)에서 음력 월수만큼 역행
    let youbi = fix_index(8 - m + 1);
    (zuofu, youbi)
}

/// 천괴(天魁)와 천鉞(天鉞)의 ZWDS 지지 인덱스 계산
///
/// * `year_stem` - 출생년도의 천간
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
        // 亥시 (time_branch_std_idx = 11)
        // 문창: 8 - 11 = -3 -> 9 (亥)
        // 문곡: 2 + 11 = 13 -> 1 (卯)
        let (wc, wq) = place_wenchang_wenqu(11);
        assert_eq!(wc, 9);
        assert_eq!(wq, 1);
    }

    #[test]
    fn test_zuofu_youbi() {
        // 음력 10월
        // 좌보: 2 + 10 - 1 = 11 (丑)
        // 우필: 8 - 10 + 1 = -1 -> 11 (丑)
        let (zf, yb) = place_zuofu_youbi(10);
        assert_eq!(zf, 11);
        assert_eq!(yb, 11);
    }
}
