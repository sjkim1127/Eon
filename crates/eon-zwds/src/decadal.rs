//! 자미두수 대한(大限, 10년 대운) 연산 모듈
//!
//! 성별과 생년 천간의 음양에 따라 순행/역행 방향을 정하고, 12개 궁에 대한의 나이 범위와 대간(大干)을 설정합니다.

use eon_core::birth::Gender;
use eon_saju::core::stem::HeavenlyStem;
use eon_saju::core::branch::EarthlyBranch;
use crate::types::{PalaceIndex, FiveElementsClass, DaXian};
use crate::palace::{fix_index, get_palace_stem, zwds_idx_to_std_idx};

/// 대한 목록을 생성하여 반환합니다.
///
/// * `gender` - 성별 (남성/여성)
/// * `year_stem` - 출생년도 천간
/// * `soul_idx` - 명궁의 ZWDS 지지 인덱스
/// * `five_elements` - 오행국
pub fn calculate_daxian(
    gender: Gender,
    year_stem: HeavenlyStem,
    soul_idx: PalaceIndex,
    five_elements: FiveElementsClass,
) -> Vec<DaXian> {
    let mut daxian_list = Vec::new();

    // 1. 순행/역행 방향 결정
    // 양남음녀(陽男陰女) -> 순행, 음남양녀(陰男陽女) -> 역행
    // 양천간: 甲, 丙, 戊, 庚, 壬 (index % 2 == 0)
    // 음천간: 乙, 丁, 己, 辛, 癸 (index % 2 == 1)
    let is_yang_stem = year_stem.index() % 2 == 0;
    let is_male = matches!(gender, Gender::Male);

    let is_forward = if is_male {
        is_yang_stem // 양남 -> 순행, 음남 -> 역행
    } else {
        !is_yang_stem // 음녀 -> 순행, 양녀 -> 역행
    };

    let step = if is_forward { 1 } else { -1 };
    let start_age = five_elements.starting_age() as u32;

    for i in 0..12 {
        let age_start = start_age + i * 10;
        let age_end = age_start + 9;

        // 명궁(soul_idx)에서 시작하여 순행/역행
        let p_idx = fix_index(soul_idx as i32 + step * i as i32);

        // 해당 궁의 천간과 지지 구하기
        let stem = get_palace_stem(year_stem, p_idx);
        let branch_std = zwds_idx_to_std_idx(p_idx);
        let branch = EarthlyBranch::from_index(branch_std as i32);

        daxian_list.push(DaXian {
            index: i,
            age_start,
            age_end,
            palace_idx: p_idx,
            stem_hanja: stem.hanja().to_string(),
            branch_hanja: branch.hanja().to_string(),
        });
    }

    daxian_list
}
