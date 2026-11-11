//! Zi Wei Dou Shu 14 Major Stars placement module.
//!
//! Calculates positions of ZiWei and TianFu, placing the 14 major stars accordingly.

use crate::palace::fix_index;
use crate::types::{PalaceIndex, ZwdsStar};
use std::collections::HashMap;

/// Calculate ZiWei ZWDS branch index (0=Yin)
///
/// * `lunar_day` - Lunar day (1~30)
/// * `five_elements_val` - Five Elements Bureau value (2, 3, 4, 5, 6)
pub fn get_ziwei_index(lunar_day: u32, five_elements_val: u32) -> PalaceIndex {
    let mut offset: i32 = -1;
    loop {
        offset += 1;
        let divisor = lunar_day as i32 + offset;
        let quotient = divisor / five_elements_val as i32;
        let remainder = divisor % five_elements_val as i32;
        if remainder == 0 {
            let mut ziwei_idx = quotient % 12 - 1;
            if offset % 2 == 0 {
                ziwei_idx += offset;
            } else {
                ziwei_idx -= offset;
            }
            return fix_index(ziwei_idx);
        }
    }
}

/// (天府星) ZWDS   
///
///   寅-申  .
/// (ziwei_index + tianfu_index) % 12 == 0
pub fn get_tianfu_index(ziwei_idx: PalaceIndex) -> PalaceIndex {
    fix_index(12 - ziwei_idx as i32)
}

///    14   
pub fn place_14_main_stars(
    ziwei_idx: PalaceIndex,
    tianfu_idx: PalaceIndex,
) -> HashMap<ZwdsStar, PalaceIndex> {
    let mut stars = HashMap::new();

    // 1.  (:   ,  )
    let z = ziwei_idx as i32;
    stars.insert(ZwdsStar::ZiWei, fix_index(z));
    stars.insert(ZwdsStar::TianJi, fix_index(z - 1));
    stars.insert(ZwdsStar::TaiYang, fix_index(z - 3));
    stars.insert(ZwdsStar::WuQu, fix_index(z - 4));
    stars.insert(ZwdsStar::TianTong, fix_index(z - 5));
    stars.insert(ZwdsStar::LianZhen, fix_index(z - 8));

    // 2.  (:  ,  )
    let f = tianfu_idx as i32;
    stars.insert(ZwdsStar::TianFu, fix_index(f));
    stars.insert(ZwdsStar::TaiYin, fix_index(f + 1));
    stars.insert(ZwdsStar::TanLang, fix_index(f + 2));
    stars.insert(ZwdsStar::JuMen, fix_index(f + 3));
    stars.insert(ZwdsStar::TianXiang, fix_index(f + 4));
    stars.insert(ZwdsStar::TianLiang, fix_index(f + 5));
    stars.insert(ZwdsStar::QiSha, fix_index(f + 6));
    stars.insert(ZwdsStar::PoJun, fix_index(f + 10));

    stars
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ziwei_pos() {
        //  16, (2) -> quotient = 16/2 = 8, rem = 0, offset = 0.
        // ziwei = 8 % 12 - 1 = 7 (酉). offset=0 ziwei = 7 (酉).
        assert_eq!(get_ziwei_index(16, 2), 7); // 酉

        //  16, (5) -> divisor = 16 + 4 = 20 (offset = 4).
        // quotient = 20 / 5 = 4. ziwei = 4 % 12 - 1 = 3. offset=4 ziwei = 3 + 4 = 7 (酉).
        assert_eq!(get_ziwei_index(16, 5), 7); // 酉
    }

    #[test]
    fn test_tianfu_pos() {
        //  寅(0) ->  寅(0)
        assert_eq!(get_tianfu_index(0), 0);
        //  子(10) ->  辰(2)
        assert_eq!(get_tianfu_index(10), 2);
    }
}
