// crates/eon-saju/src/analysis/heluo.rs

use serde::{Deserialize, Serialize};
use crate::core::stem::HeavenlyStem;
use crate::core::branch::EarthlyBranch;
use crate::core::ganzi::GanZi;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Era {
    SangWon, // 1864 ~ 1923
    JungWon, // 1924 ~ 1983
    HaWon,   // 1984 ~ 2043
}

impl Era {
    pub fn from_year(year: i32) -> Self {
        if year < 1924 {
            Self::SangWon
        } else if year < 1984 {
            Self::JungWon
        } else {
            Self::HaWon
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeLuoCycle {
    pub start_age: u32,
    pub end_age: u32,
    pub hexagram_index: u8, // King Wen index (1..64)
    pub line_index: u8,      // 1..6
    pub is_pre_natal: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeLuoResult {
    pub pre_natal_hexagram: u8,
    pub post_natal_hexagram: u8,
    pub yuan_dang_yao: u8,
    pub lifetime_cycles: Vec<HeLuoCycle>,
}

/// 천간의 낙서 수리 배정
fn stem_to_num(stem: HeavenlyStem) -> u32 {
    match stem {
        HeavenlyStem::Jia => 3,
        HeavenlyStem::Yi => 8,
        HeavenlyStem::Bing => 2,
        HeavenlyStem::Ding => 7,
        HeavenlyStem::Wu => 5,
        HeavenlyStem::Ji => 10,
        HeavenlyStem::Geng => 4,
        HeavenlyStem::Xin => 9,
        HeavenlyStem::Ren => 1,
        HeavenlyStem::Gui => 6,
    }
}

/// 지지의 오행 배송 하도 수리 배정 (생수와 성수 쌍)
fn branch_to_nums(branch: EarthlyBranch) -> Vec<u32> {
    match branch {
        EarthlyBranch::Zi | EarthlyBranch::Hai => vec![1, 6],
        EarthlyBranch::Yin | EarthlyBranch::Mao => vec![3, 8],
        EarthlyBranch::Si | EarthlyBranch::Wu => vec![2, 7],
        EarthlyBranch::Shen | EarthlyBranch::You => vec![4, 9],
        EarthlyBranch::Chen | EarthlyBranch::Xu | EarthlyBranch::Chou | EarthlyBranch::Wei => vec![5, 10],
    }
}

/// 5와 10의 기궁(寄宮) 처리
fn get_gigung_trigram(era: Era, is_yang_male_or_yin_female: bool) -> u8 {
    match era {
        Era::SangWon => {
            if is_yang_male_or_yin_female { 8 } else { 2 } // 艮 / 坤
        }
        Era::JungWon => {
            if is_yang_male_or_yin_female { 4 } else { 3 } // 巽 / 震
        }
        Era::HaWon => {
            if is_yang_male_or_yin_female { 9 } else { 7 } // 離 / 兌
        }
    }
}

fn reduce_heaven(sum: u32, is_yang_male_or_yin_female: bool, era: Era) -> u8 {
    let mut remainder = sum;
    while remainder > 25 {
        remainder -= 25;
    }
    if remainder == 0 {
        remainder = 25;
    }
    
    let mut val = (remainder % 10) as u8;
    if val == 0 {
        val = 10;
    }
    
    if val == 5 || val == 10 {
        get_gigung_trigram(era, is_yang_male_or_yin_female)
    } else {
        val
    }
}

fn reduce_earth(sum: u32, is_yang_male_or_yin_female: bool, era: Era) -> u8 {
    let mut remainder = sum;
    while remainder > 30 {
        remainder -= 30;
    }
    if remainder == 0 {
        remainder = 30;
    }
    
    let mut val = (remainder % 10) as u8;
    if val == 0 {
        val = 10;
    }
    
    if val == 5 || val == 10 {
        get_gigung_trigram(era, is_yang_male_or_yin_female)
    } else {
        val
    }
}

pub fn get_trigram_lines(num: u8) -> [bool; 3] {
    match num {
        1 => [false, true, false],  // 坎 ☵
        2 => [false, false, false], // 坤 ☷
        3 => [true, false, false],  // 震 ☳
        4 => [false, true, true],   // 巽 ☴
        6 => [true, true, true],    // 乾 ☰
        7 => [true, true, false],   // 兌 ☱
        8 => [false, false, true],  // 艮 ☶
        9 => [true, false, true],   // 離 ☲
        _ => [false, false, false],
    }
}

pub fn get_king_wen_index(upper: u8, lower: u8) -> u8 {
    match (upper, lower) {
        (6, 6) => 1,  // 乾
        (2, 2) => 2,  // 坤
        (1, 3) => 3,  // 屯
        (8, 1) => 4,  // 蒙
        (6, 1) => 5,  // 需
        (1, 6) => 6,  // 訟
        (2, 1) => 7,  // 師
        (1, 2) => 8,  // 比
        (4, 6) => 9,  // 小畜
        (6, 7) => 10, // 履
        (2, 6) => 11, // 泰
        (6, 2) => 12, // 否
        (6, 9) => 13, // 同인
        (9, 6) => 14, // 大有
        (2, 8) => 15, // 謙
        (3, 2) => 16, // 豫
        (7, 3) => 17, // 隨
        (8, 4) => 18, // 蠱
        (2, 7) => 19, // 臨
        (4, 2) => 20, // 觀
        (9, 3) => 21, // 噬嗑
        (8, 9) => 22, // 賁
        (8, 2) => 23, // 剝
        (2, 3) => 24, // 復
        (6, 3) => 25, // 無妄
        (8, 6) => 26, // 大畜
        (8, 3) => 27, // 頤
        (7, 4) => 28, // 大過
        (1, 1) => 29, // 坎
        (9, 9) => 30, // 離
        (7, 8) => 31, // 咸
        (3, 4) => 32, // 恆
        (6, 8) => 33, // 遯
        (3, 6) => 34, // 大壯
        (9, 2) => 35, // 晉
        (2, 9) => 36, // 明夷
        (4, 9) => 37, // 家人
        (9, 7) => 38, // 睽
        (1, 8) => 39, // 蹇
        (3, 1) => 40, // 解
        (8, 7) => 41, // 損
        (4, 3) => 42, // 益
        (7, 6) => 43, // 夬
        (6, 4) => 44, // 姤
        (7, 2) => 45, // 萃
        (2, 4) => 46, // 升
        (7, 1) => 47, // 困
        (1, 4) => 48, // 井
        (7, 9) => 49, // 革
        (9, 4) => 50, // 鼎
        (3, 3) => 51, // 震
        (8, 8) => 52, // 艮
        (4, 8) => 53, // 漸
        (3, 7) => 54, // 歸妹
        (3, 9) => 55, // 豐
        (9, 8) => 56, // 旅
        (4, 4) => 57, // 巽
        (7, 7) => 58, // 兌
        (4, 1) => 59, // 渙
        (1, 7) => 60, // 節
        (4, 7) => 61, // 中孚
        (3, 8) => 62, // 小過
        (1, 9) => 63, // 既濟
        (9, 1) => 64, // 未濟
        _ => 1,
    }
}

pub fn calculate_heluo(
    birth_year: i32,
    is_male: bool,
    year_ganzi: &GanZi,
    month_ganzi: &GanZi,
    day_ganzi: &GanZi,
    hour_ganzi: &GanZi,
) -> HeLuoResult {
    let era = Era::from_year(birth_year);
    
    // 1. 천간 수리 치환 및 합산
    let stem_nums = vec![
        stem_to_num(year_ganzi.stem),
        stem_to_num(month_ganzi.stem),
        stem_to_num(day_ganzi.stem),
        stem_to_num(hour_ganzi.stem),
    ];
    
    // 2. 지지 수리 치환 및 합산
    let mut branch_nums = Vec::new();
    branch_nums.extend(branch_to_nums(year_ganzi.branch));
    branch_nums.extend(branch_to_nums(month_ganzi.branch));
    branch_nums.extend(branch_to_nums(day_ganzi.branch));
    branch_nums.extend(branch_to_nums(hour_ganzi.branch));
    
    let mut all_nums = Vec::new();
    all_nums.extend(stem_nums);
    all_nums.extend(branch_nums);
    
    // 천수(홀수) 및 지수(짝수) 분류 및 합산
    let mut sum_odd = 0;
    let mut sum_even = 0;
    for num in all_nums {
        if num % 2 == 1 {
            sum_odd += num;
        } else {
            sum_even += num;
        }
    }
    
    // 양남음녀 판단
    let is_year_yang = matches!(
        year_ganzi.stem,
        HeavenlyStem::Jia | HeavenlyStem::Bing | HeavenlyStem::Wu | HeavenlyStem::Geng | HeavenlyStem::Ren
    );
    let is_yang_male_or_yin_female = (is_year_yang && is_male) || (!is_year_yang && !is_male);
    
    // 상괘 / 하괘 번호 도출
    let odd_trigram_num = reduce_heaven(sum_odd, is_yang_male_or_yin_female, era);
    let even_trigram_num = reduce_earth(sum_even, is_yang_male_or_yin_female, era);
    
    let (pre_upper, pre_lower) = if is_yang_male_or_yin_female {
        (odd_trigram_num, even_trigram_num)
    } else {
        (even_trigram_num, odd_trigram_num)
    };
    
    let pre_natal_hexagram = get_king_wen_index(pre_upper, pre_lower);
    
    // 선천괘의 6효 구조 만들기
    let upper_lines = get_trigram_lines(pre_upper);
    let lower_lines = get_trigram_lines(pre_lower);
    let mut pre_lines = [false; 6];
    pre_lines[0] = lower_lines[0]; // 초효
    pre_lines[1] = lower_lines[1]; // 2효
    pre_lines[2] = lower_lines[2]; // 3효
    pre_lines[3] = upper_lines[0]; // 4효
    pre_lines[4] = upper_lines[1]; // 5효
    pre_lines[5] = upper_lines[2]; // 상효
    
    // 태어난 시의 음양 및 인덱스
    let hour_branch_idx = hour_ganzi.branch.index() as usize;
    let is_yang_hour = hour_branch_idx <= 5; // 자~사 시
    
    // 원당효 계산
    let mut target_indices = Vec::new();
    for i in 0..6 {
        if pre_lines[i] == is_yang_hour {
            target_indices.push(i);
        }
    }
    
    let yuan_dang_idx = if target_indices.is_empty() {
        let slot = match hour_branch_idx {
            0 | 1 => 0, // 子丑
            2 | 6 => 1, // 寅 or 午
            3 | 7 => 2, // 卯 or 未
            4 | 8 => 3, // 辰 or 申
            5 | 9 => 4, // 巳 or 酉
            _ => 5,     // 戌 or 亥
        };
        slot
    } else {
        let slot = match hour_branch_idx {
            0 | 1 => 0,
            2 | 6 => 1,
            3 | 7 => 2,
            4 | 8 => 3,
            5 | 9 => 4,
            _ => 5,
        };
        let idx_in_targets = slot % target_indices.len();
        target_indices[idx_in_targets]
    };
    
    let yuan_dang_yao = (yuan_dang_idx + 1) as u8;
    
    // 후천괘 만들기 (원당효가 위치한 효를 반전)
    let mut post_lines = pre_lines;
    post_lines[yuan_dang_idx] = !pre_lines[yuan_dang_idx];
    
    // 반전된 효들로부터 후천 상괘/하괘 도출
    let post_lower_lines = [post_lines[0], post_lines[1], post_lines[2]];
    let post_upper_lines = [post_lines[3], post_lines[4], post_lines[5]];
    
    let post_lower = lines_to_trigram_num(post_lower_lines);
    let post_upper = lines_to_trigram_num(post_upper_lines);
    let post_natal_hexagram = get_king_wen_index(post_upper, post_lower);
    
    // 평생 대운 타임라인 생성
    let mut lifetime_cycles = Vec::new();
    let mut current_age = 1;
    
    // 1. 선천괘 주기 (원당효부터 순행/역행)
    let is_forward = is_yang_male_or_yin_female;
    let mut line_ptr = yuan_dang_idx;
    
    for _ in 0..6 {
        let is_yang = pre_lines[line_ptr];
        let duration = if is_yang { 9 } else { 6 };
        let end_age = current_age + duration - 1;
        
        lifetime_cycles.push(HeLuoCycle {
            start_age: current_age,
            end_age,
            hexagram_index: pre_natal_hexagram,
            line_index: (line_ptr + 1) as u8,
            is_pre_natal: true,
        });
        
        current_age += duration;
        if is_forward {
            line_ptr = (line_ptr + 1) % 6;
        } else {
            line_ptr = if line_ptr == 0 { 5 } else { line_ptr - 1 };
        }
    }
    
    // 2. 후천괘 주기 (후천괘 원당효부터 순행/역행)
    let mut post_line_ptr = yuan_dang_idx; // 동일한 효 위치를 기준으로 삼음
    for _ in 0..6 {
        let is_yang = post_lines[post_line_ptr];
        let duration = if is_yang { 9 } else { 6 };
        let end_age = current_age + duration - 1;
        
        lifetime_cycles.push(HeLuoCycle {
            start_age: current_age,
            end_age,
            hexagram_index: post_natal_hexagram,
            line_index: (post_line_ptr + 1) as u8,
            is_pre_natal: false,
        });
        
        current_age += duration;
        if is_forward {
            post_line_ptr = (post_line_ptr + 1) % 6;
        } else {
            post_line_ptr = if post_line_ptr == 0 { 5 } else { post_line_ptr - 1 };
        }
    }
    
    HeLuoResult {
        pre_natal_hexagram,
        post_natal_hexagram,
        yuan_dang_yao,
        lifetime_cycles,
    }
}

fn lines_to_trigram_num(lines: [bool; 3]) -> u8 {
    match lines {
        [false, true, false] => 1,  // 坎
        [false, false, false] => 2, // 坤
        [true, false, false] => 3,  // 震
        [false, true, true] => 4,   // 巽
        [true, true, true] => 6,    // 乾
        [true, true, false] => 7,   // 兌
        [false, false, true] => 8,  // 艮
        [true, false, true] => 9,   // 離
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::stem::HeavenlyStem;
    use crate::core::branch::EarthlyBranch;

    #[test]
    fn test_heluo_calculation() {
        let year_gz = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Zi);
        let month_gz = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Zi);
        let day_gz = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Zi);
        let hour_gz = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Zi);

        let res = calculate_heluo(1984, true, &year_gz, &month_gz, &day_gz, &hour_gz);
        assert!(res.pre_natal_hexagram > 0 && res.pre_natal_hexagram <= 64);
        assert!(res.post_natal_hexagram > 0 && res.post_natal_hexagram <= 64);
        assert!(res.yuan_dang_yao >= 1 && res.yuan_dang_yao <= 6);
        assert!(!res.lifetime_cycles.is_empty());
    }
}
