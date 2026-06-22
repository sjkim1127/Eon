//! 자미두수 14주성(主星) 배치 모듈
//!
//! 자미성과 천부성의 위치를 계산하고, 이를 기준으로 14주성을 각 궁에 배치합니다.

use std::collections::HashMap;
use crate::types::{PalaceIndex, ZwdsStar, FiveElementsClass};
use crate::palace::fix_index;

/// 자미성(紫微星)의 ZWDS 지지 인덱스 (0=寅) 계산
///
/// * `lunar_day` - 음력 일 (1~30)
/// * `five_elements_val` - 오행국 수치 (2, 3, 4, 5, 6)
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

/// 천부성(天府星)의 ZWDS 지지 인덱스 계산
///
/// 자미성과 천부성은 寅-申 축 대칭입니다.
/// (ziwei_index + tianfu_index) % 12 == 0
pub fn get_tianfu_index(ziwei_idx: PalaceIndex) -> PalaceIndex {
    fix_index(12 - ziwei_idx as i32)
}

/// 자미성과 천부성을 기준으로 14주성의 위치를 맵으로 리턴
pub fn place_14_main_stars(ziwei_idx: PalaceIndex, tianfu_idx: PalaceIndex) -> HashMap<ZwdsStar, PalaceIndex> {
    let mut stars = HashMap::new();

    // 1. 자미성계 (역행: 시계 반대 방향, 인덱스 감소)
    let z = ziwei_idx as i32;
    stars.insert(ZwdsStar::ZiWei,    fix_index(z));
    stars.insert(ZwdsStar::TianJi,   fix_index(z - 1));
    stars.insert(ZwdsStar::TaiYang,  fix_index(z - 3));
    stars.insert(ZwdsStar::WuQu,     fix_index(z - 4));
    stars.insert(ZwdsStar::TianTong, fix_index(z - 5));
    stars.insert(ZwdsStar::LianZhen, fix_index(z - 8));

    // 2. 천부성계 (순행: 시계 방향, 인덱스 증가)
    let f = tianfu_idx as i32;
    stars.insert(ZwdsStar::TianFu,    fix_index(f));
    stars.insert(ZwdsStar::TaiYin,    fix_index(f + 1));
    stars.insert(ZwdsStar::TanLang,   fix_index(f + 2));
    stars.insert(ZwdsStar::JuMen,    fix_index(f + 3));
    stars.insert(ZwdsStar::TianXiang, fix_index(f + 4));
    stars.insert(ZwdsStar::TianLiang, fix_index(f + 5));
    stars.insert(ZwdsStar::QiSha,     fix_index(f + 6));
    stars.insert(ZwdsStar::PoJun,     fix_index(f + 10));

    stars
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ziwei_pos() {
        // 음력 16일, 수이국(2) -> quotient = 16/2 = 8, rem = 0, offset = 0.
        // ziwei = 8 % 12 - 1 = 7 (酉). offset=0이므로 ziwei = 7 (酉).
        assert_eq!(get_ziwei_index(16, 2), 7); // 酉

        // 음력 16일, 토오국(5) -> divisor = 16 + 4 = 20 (offset = 4).
        // quotient = 20 / 5 = 4. ziwei = 4 % 12 - 1 = 3. offset=4이므로 ziwei = 3 + 4 = 7 (酉).
        assert_eq!(get_ziwei_index(16, 5), 7); // 酉
    }

    #[test]
    fn test_tianfu_pos() {
        // 자미가 寅(0) -> 천부는 寅(0)
        assert_eq!(get_tianfu_index(0), 0);
        // 자미가 子(10) -> 천부는 辰(2)
        assert_eq!(get_tianfu_index(10), 2);
    }
}
