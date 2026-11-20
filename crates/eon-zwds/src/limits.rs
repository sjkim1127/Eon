//! 小限 (annual minor limit) and 斗君 (annual "Dou Jun" palace).
//!
//! Both are pure palace look-ups for a target year; the chart-aware wrappers
//! ([`crate::ZwdsChart::xiaoxian`] and [`crate::ZwdsChart::dou_jun`]) add names, stems and 四化.
//!
//! ## 小限
//! 《紫微斗數全書》安小限訣: 寅午戌人辰上起, 申子辰人自戌宮, 巳酉丑人未宮始, 亥卯未人起丑宮;
//! 男順女逆, 一歲一宮. Age 1 (虛歲, nominal age counted from the lunar birth year) sits in the start
//! palace; males then advance along the branch order (辰 → 巳 → 午 ...), females go backwards.
//!
//! ## 斗君
//! 《紫微斗數全書》斗君訣: 以流年太歲所在宮為正月, 逆數至生月, 再從該宮起子時, 順數至生時, 即斗君.
//! In ZWDS coordinates (寅 = 0): `dou_jun = sui − (birth_month − 1) + hour_branch`, where `sui` is
//! the palace of the year branch and `hour_branch` is the standard hour index (子 = 0).
//! Schools differ on leap months (here the leap month uses its month number) and on whether the
//! late-子 hour (23:00–24:00) belongs to the next day; see `docs/CALCULATION.md`.

use crate::palace::{fix_index, std_idx_to_zwds_idx};
use crate::types::{PalaceIndex, PalaceName, ZwdsStar};
use serde::{Deserialize, Serialize};

/// 小限 for one target year.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XiaoXian {
    /// Target (流年) year.
    pub year: i32,
    /// Nominal age (虛歲) in the target year: `target_year − lunar_birth_year + 1`.
    pub age: u32,
    /// Palace the 小限 falls in (ZWDS index, 寅 = 0).
    pub palace_idx: PalaceIndex,
    /// Natal name of that palace (e.g. 命宮, 財帛宮).
    pub palace_name: PalaceName,
    pub stem_hanja: String,
    pub branch_hanja: String,
    /// 四化 stars of the 小限 palace stem: [化祿, 化權, 化科, 化忌].
    pub si_hua: [ZwdsStar; 4],
}

/// 斗君 for one target year.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DouJun {
    /// Target (流年) year.
    pub year: i32,
    /// Palace of the 斗君 (ZWDS index, 寅 = 0).
    pub palace_idx: PalaceIndex,
    /// Natal name of that palace.
    pub palace_name: PalaceName,
    pub stem_hanja: String,
    pub branch_hanja: String,
}

/// Palace where 小限 age 1 starts, by birth-year branch (standard index, 子 = 0).
pub fn xiaoxian_start(year_branch_std_idx: usize) -> PalaceIndex {
    match year_branch_std_idx % 12 {
        2 | 6 | 10 => 2, // 寅午戌 → 辰
        8 | 0 | 4 => 8,  // 申子辰 → 戌
        5 | 9 | 1 => 5,  // 巳酉丑 → 未
        _ => 11,         // 亥卯未 → 丑
    }
}

/// 小限 palace for a nominal age (>= 1).
pub fn xiaoxian_palace(year_branch_std_idx: usize, is_male: bool, age: u32) -> PalaceIndex {
    let steps = age.saturating_sub(1) as i32;
    let dir = if is_male { 1 } else { -1 };
    fix_index(xiaoxian_start(year_branch_std_idx) as i32 + dir * steps)
}

/// 斗君 palace for a target-year branch (standard index), lunar birth month and birth-hour branch
/// (standard index, 子 = 0).
pub fn dou_jun_palace(
    target_branch_std_idx: usize,
    lunar_month: u32,
    time_branch_std_idx: usize,
) -> PalaceIndex {
    let sui = std_idx_to_zwds_idx(target_branch_std_idx % 12) as i32;
    fix_index(sui - (lunar_month as i32 - 1) + time_branch_std_idx as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xiaoxian_start_by_year_branch() {
        // 寅午戌 → 辰, 申子辰 → 戌, 巳酉丑 → 未, 亥卯未 → 丑 (ZWDS idx: 辰 2, 戌 8, 未 5, 丑 11).
        assert_eq!(xiaoxian_start(2), 2);
        assert_eq!(xiaoxian_start(6), 2);
        assert_eq!(xiaoxian_start(10), 2);
        assert_eq!(xiaoxian_start(0), 8);
        assert_eq!(xiaoxian_start(4), 8);
        assert_eq!(xiaoxian_start(8), 8);
        assert_eq!(xiaoxian_start(1), 5);
        assert_eq!(xiaoxian_start(5), 5);
        assert_eq!(xiaoxian_start(9), 5);
        assert_eq!(xiaoxian_start(3), 11);
        assert_eq!(xiaoxian_start(7), 11);
        assert_eq!(xiaoxian_start(11), 11);
    }

    #[test]
    fn xiaoxian_male_goes_forward_female_backward() {
        // 午 year: age 1 in 辰 (2). Male age 2 → 巳 (3), age 13 → back to 辰, age 37 → 辰.
        assert_eq!(xiaoxian_palace(6, true, 1), 2);
        assert_eq!(xiaoxian_palace(6, true, 2), 3);
        assert_eq!(xiaoxian_palace(6, true, 13), 2);
        assert_eq!(xiaoxian_palace(6, true, 37), 2);
        // 子 year female: age 1 in 戌 (8), age 2 → 酉 (7), age 3 → 申 (6), age 43 = 1 + 42 →
        // 42 mod 12 = 6 steps back from 戌 → 辰 (2).
        assert_eq!(xiaoxian_palace(0, false, 1), 8);
        assert_eq!(xiaoxian_palace(0, false, 2), 7);
        assert_eq!(xiaoxian_palace(0, false, 3), 6);
        assert_eq!(xiaoxian_palace(0, false, 43), 2);
    }

    #[test]
    fn dou_jun_reference() {
        // Hand-computed (《全書》斗君訣): year 丙午 (午 = std 6 → ZWDS 4), born lunar month 4,
        // 巳 hour (std 5): 午宮起正月, 逆數至四月 → 4 − 3 = 1 (卯), 再順數至巳時 → 1 + 5 = 6 (申).
        assert_eq!(dou_jun_palace(6, 4, 5), 6);
        // Born in month 1 at 子 hour the 斗君 is the 太歲 palace itself.
        assert_eq!(dou_jun_palace(6, 1, 0), 4);
        // Mirror identity: 斗君 + 命宮 ≡ 太歲 palace (all in 寅 = 0 coordinates), because
        // 命宮 = (month − 1) − hour. Check on the same birth: 命宮 = 3 − 5 → 10 (子).
        let ming = crate::palace::get_soul_and_body_index(4, 5).0;
        assert_eq!(fix_index(dou_jun_palace(6, 4, 5) as i32 + ming as i32), 4);
    }
}
