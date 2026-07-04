//! 자미두수 궁(Palace) 연산 모듈
//!
//! 명궁(命宮)과 신궁(身宮) 계산, 오행국(五行局) 결정, 12궁명 매핑을 처리합니다.

use crate::types::{FiveElementsClass, PalaceIndex, PalaceName};
use eon_saju::core::branch::EarthlyBranch;
use eon_saju::core::stem::HeavenlyStem;

/// 인덱스 보정 (0~11 범위)
pub fn fix_index(i: i32) -> usize {
    let r = i % 12;
    if r < 0 {
        (r + 12) as usize
    } else {
        r as usize
    }
}

/// ZWDS 지지 인덱스 (0=寅) -> 표준 지지 인덱스 (0=子)
pub fn zwds_idx_to_std_idx(zwds_idx: usize) -> usize {
    (zwds_idx + 2) % 12
}

/// 표준 지지 인덱스 (0=子) -> ZWDS 지지 인덱스 (0=寅)
pub fn std_idx_to_zwds_idx(std_idx: usize) -> usize {
    (std_idx + 10) % 12
}

/// 명궁 및 신궁 인덱스 계산 (ZWDS 인덱스 반환)
///
/// * `lunar_month` - 음력 월 (1~12)
/// * `time_branch_std_idx` - 태어난 시의 표준 지지 인덱스 (0=子)
pub fn get_soul_and_body_index(
    lunar_month: u32,
    time_branch_std_idx: usize,
) -> (PalaceIndex, PalaceIndex) {
    let month_idx = (lunar_month - 1) as i32;
    let time_idx = time_branch_std_idx as i32;

    let soul = fix_index(month_idx - time_idx);
    let body = fix_index(month_idx + time_idx);

    (soul, body)
}

/// 각 궁의 천간 계산 (五虎遁年起月법 적용)
///
/// * `year_stem` - 출생년도의 천간
/// * `palace_zwds_idx` - ZWDS 지지 인덱스 (0=寅, 1=卯...)
pub fn get_palace_stem(year_stem: HeavenlyStem, palace_zwds_idx: PalaceIndex) -> HeavenlyStem {
    let start_stem_idx = match year_stem {
        HeavenlyStem::Jia | HeavenlyStem::Ji => 2,   // 丙
        HeavenlyStem::Yi | HeavenlyStem::Geng => 4,  // 戊
        HeavenlyStem::Bing | HeavenlyStem::Xin => 6, // 庚
        HeavenlyStem::Ding | HeavenlyStem::Ren => 8, // 壬
        HeavenlyStem::Wu | HeavenlyStem::Gui => 0,   // 甲
    };
    let stem_idx = (start_stem_idx + palace_zwds_idx) % 10;
    HeavenlyStem::from_index(stem_idx as i32)
}

/// 오행국(Five Elements Class) 결정
///
/// * `soul_stem` - 명궁의 천간
/// * `soul_zwds_idx` - 명궁의 ZWDS 지지 인덱스
pub fn get_five_elements(soul_stem: HeavenlyStem, soul_zwds_idx: PalaceIndex) -> FiveElementsClass {
    let stem_num = match soul_stem {
        HeavenlyStem::Jia | HeavenlyStem::Yi => 1,
        HeavenlyStem::Bing | HeavenlyStem::Ding => 2,
        HeavenlyStem::Wu | HeavenlyStem::Ji => 3,
        HeavenlyStem::Geng | HeavenlyStem::Xin => 4,
        HeavenlyStem::Ren | HeavenlyStem::Gui => 5,
    };

    let soul_branch_std_idx = zwds_idx_to_std_idx(soul_zwds_idx);
    let branch = EarthlyBranch::from_index(soul_branch_std_idx as i32);

    let branch_num = match branch {
        EarthlyBranch::Zi | EarthlyBranch::Wu | EarthlyBranch::Chou | EarthlyBranch::Wei => 1,
        EarthlyBranch::Yin | EarthlyBranch::Shen | EarthlyBranch::Mao | EarthlyBranch::You => 2,
        EarthlyBranch::Chen | EarthlyBranch::Xu | EarthlyBranch::Si | EarthlyBranch::Hai => 3,
    };

    let sum = stem_num + branch_num;
    let rem = sum % 5;

    match rem {
        1 => FiveElementsClass::Wood3,
        2 => FiveElementsClass::Metal4,
        3 => FiveElementsClass::Water2,
        4 => FiveElementsClass::Fire6,
        0 => FiveElementsClass::Earth5,
        _ => unreachable!(),
    }
}

/// 각 ZWDS 인덱스의 궁명(PalaceName) 결정
///
/// * `soul_zwds_idx` - 명궁의 ZWDS 지지 인덱스
/// * `palace_zwds_idx` - 현재 궁의 ZWDS 지지 인덱스
pub fn get_palace_name(soul_zwds_idx: PalaceIndex, palace_zwds_idx: PalaceIndex) -> PalaceName {
    let offset = fix_index(palace_zwds_idx as i32 - soul_zwds_idx as i32);
    PalaceName::ALL[offset]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soul_body_calc() {
        // 음력 10월, 亥시생 (lunar_month = 10, time_branch_std_idx = 11)
        let (soul, body) = get_soul_and_body_index(10, 11);
        // soul = 9 - 11 = -2 -> 10 (子궁)
        // body = 9 + 11 = 20 -> 8 (戌궁)
        assert_eq!(soul, 10);
        assert_eq!(body, 8);
    }

    #[test]
    fn test_palace_stem() {
        // 甲년, ZWDS 0(寅)의 천간 -> 丙
        assert_eq!(get_palace_stem(HeavenlyStem::Jia, 0), HeavenlyStem::Bing);
        // 甲년, ZWDS 10(子)의 천간 -> 甲 (start 2 + 10 = 12 % 10 = 2 -> 丙인데? 아! 10 % 10 = 0 -> 丙 + 10 = 丙(2) + 10 = 12 -> 丙?)
        // 잠시, 공식이 맞는지 검증해보자.
        // start_stem_idx:
        // 甲/己年 -> 寅궁(0) 천간 = 丙 (2)
        // 卯궁(1) 천간 = 丁 (3)
        // 辰궁(2) 천간 = 戊 (4)
        // 巳궁(3) 천간 = 己 (5)
        // 午궁(4) 천간 = 庚 (6)
        // 未궁(5) 천간 = 辛 (7)
        // 申궁(6) 천간 = 壬 (8)
        // 酉궁(7) 천간 = 癸 (9)
        // 戌궁(8) 천간 = 甲 (0)
        // 亥궁(9) 천간 = 乙 (1)
        // 子궁(10) 천간 = 丙 (2)
        // 丑궁(11) 천간 = 丁 (3)
        // 공식: (start_stem_idx + palace_zwds_idx) % 10
        // (2 + 0) % 10 = 2 -> 丙. 맞음.
        // (2 + 10) % 10 = 2 -> 丙. 맞음! (子궁의 천간은 丙이다. 五虎遁법에 의하면 甲己년의 자궁 천간은 丙이 맞음).
        // (2 + 8) % 10 = 0 -> 甲. 戌궁의 천간은 甲. 맞음!
        assert_eq!(get_palace_stem(HeavenlyStem::Jia, 10), HeavenlyStem::Bing);
    }

    #[test]
    fn test_five_elements() {
        // 명궁 천간: 丙, 명궁 ZWDS: 10(子)
        // stem_num = 丙(2)
        // ZWDS 10 -> 표준 0(子) -> branch_num = 1
        // sum = 3 -> rem = 3 -> Water2 (수이국)
        let class = get_five_elements(HeavenlyStem::Bing, 10);
        assert_eq!(class, FiveElementsClass::Water2);
    }
}
