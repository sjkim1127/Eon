// crates/eon-saju/src/analysis/heluo.rs

use serde::{Deserialize, Serialize};
use crate::core::stem::HeavenlyStem;
use crate::core::branch::EarthlyBranch;
use crate::core::ganzi::GanZi;
use crate::core::element::{Element, ElementRelation};

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
pub struct NaJiaYao {
    pub stem: HeavenlyStem,
    pub branch: EarthlyBranch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YaoShinsal {
    pub is_noble: bool,
    pub is_void: bool,
    pub is_rok: bool,
    pub is_horse: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TiYongRelation {
    pub ti_element: Element,
    pub yong_element: Element,
    pub relationship: ElementRelation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YearlyHexagramResult {
    pub age: u32,
    pub hexagram_index: u8,
    pub yearly_line: u8,
    pub monthly_hexagrams: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeLuoResult {
    pub pre_natal_hexagram: u8,
    pub post_natal_hexagram: u8,
    pub yuan_dang_yao: u8,
    pub lifetime_cycles: Vec<HeLuoCycle>,
    // 확장 필드
    pub yuan_qi: bool,
    pub hua_gong: bool,
    pub se_yao: u8,
    pub ying_yao: u8,
    pub najia_lines: Vec<NaJiaYao>,
    pub shinsal_lines: Vec<YaoShinsal>,
    pub ti_yong: TiYongRelation,
    pub yearly_hexagrams: Vec<YearlyHexagramResult>,
}

/// 천간의 낙서 수리 배정 (후천팔괘 납갑수)
fn stem_to_num(stem: HeavenlyStem) -> u32 {
    match stem {
        HeavenlyStem::Jia => 6, // 乾
        HeavenlyStem::Yi => 2,  // 坤
        HeavenlyStem::Bing => 8, // 艮
        HeavenlyStem::Ding => 7, // 兌
        HeavenlyStem::Wu => 1,   // 坎
        HeavenlyStem::Ji => 9,   // 離
        HeavenlyStem::Geng => 3, // 震
        HeavenlyStem::Xin => 4,  // 巽
        HeavenlyStem::Ren => 6,  // 乾
        HeavenlyStem::Gui => 2,  // 坤
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

/// 5의 기궁(寄宮) 처리
fn get_gigung_trigram(era: Era, is_male: bool, is_yang_male_or_yin_female: bool) -> u8 {
    match era {
        Era::SangWon => {
            if is_male { 8 } else { 2 } // 남기간(8), 여기관(2)
        }
        Era::JungWon => {
            if is_yang_male_or_yin_female { 8 } else { 2 } // 양남음녀기간(8), 음남양녀기관(2)
        }
        Era::HaWon => {
            if is_male { 9 } else { 7 } // 남기리(9), 여기태(7)
        }
    }
}

fn reduce_heaven(sum: u32, is_male: bool, is_yang_male_or_yin_female: bool, era: Era) -> u8 {
    let mut remainder = sum;
    while remainder > 25 {
        remainder -= 25;
    }
    if remainder == 0 {
        remainder = 25;
    }
    
    // 去十不用 (십의 자리는 버림)
    let val = if remainder == 10 {
        1
    } else if remainder == 20 {
        2
    } else {
        (remainder % 10) as u8
    };
    
    if val == 5 {
        get_gigung_trigram(era, is_male, is_yang_male_or_yin_female)
    } else {
        val
    }
}

fn reduce_earth(sum: u32, is_male: bool, is_yang_male_or_yin_female: bool, era: Era) -> u8 {
    let mut remainder = sum;
    while remainder > 30 {
        remainder -= 30;
    }
    if remainder == 0 {
        remainder = 30;
    }
    
    // 去十不用 (십의 자리는 버림)
    let val = if remainder == 10 {
        1
    } else if remainder == 20 {
        2
    } else if remainder == 30 {
        3
    } else {
        (remainder % 10) as u8
    };
    
    if val == 5 {
        get_gigung_trigram(era, is_male, is_yang_male_or_yin_female)
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
    let odd_trigram_num = reduce_heaven(sum_odd, is_male, is_yang_male_or_yin_female, era);
    let even_trigram_num = reduce_earth(sum_even, is_male, is_yang_male_or_yin_female, era);
    
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
    
    // 태어난 시의 음양 및 인덱스 (자~사=양시(0..5), 오~해=음시(6..11))
    let hour_branch_idx = hour_ganzi.branch.index() as usize;
    let is_yang_hour = hour_branch_idx <= 5;
    let hi = hour_branch_idx % 6;
    
    // 원당효 계산을 위해 매칭/타겟 라인 분리
    let mut matching_indices = Vec::new();
    let mut other_indices = Vec::new();
    for i in 0..6 {
        if pre_lines[i] == is_yang_hour {
            matching_indices.push(i);
        } else {
            other_indices.push(i);
        }
    }
    
    let k = matching_indices.len();
    let yuan_dang_idx = if k == 0 || k == 6 {
        // 순수 괘상(전부 양이거나 전부 음)일 때의 순수 원당 규칙
        let base_line = if is_yang_hour {
            (hi % 3) + 1
        } else {
            (hi % 3) + 4
        };
        
        let is_yang_ling = month_ganzi.branch.index() <= 5;
        let mut reverse = false;
        if pre_natal_hexagram == 1 { // 乾
            if !is_male && is_yang_ling {
                reverse = true;
            }
        } else if pre_natal_hexagram == 2 { // 坤
            if is_male && !is_yang_ling {
                reverse = true;
            }
        }
        
        if reverse {
            6 - base_line
        } else {
            base_line - 1
        }
    } else {
        // 일반적인 괘상
        let mut slots = Vec::new();
        if k <= 3 {
            slots.extend(&matching_indices);
            slots.extend(&matching_indices);
            slots.extend(&other_indices);
        } else {
            slots.extend(&matching_indices);
            slots.extend(&other_indices);
        }
        slots[hi]
    };
    
    let yuan_dang_yao = (yuan_dang_idx + 1) as u8;
    
    // 후천괘 만들기
    let is_yang_ling = month_ganzi.branch.index() <= 5;
    
    // 삼지존괘 (坎 29, 屯 3, 蹇 39) 조건 체크
    let is_three_sovereign = pre_natal_hexagram == 29 || pre_natal_hexagram == 3 || pre_natal_hexagram == 39;
    let is_special_sovereign_case = is_three_sovereign && (
        (yuan_dang_yao == 5 && !is_yang_ling) || // 9五 이며 음령일 때
        (yuan_dang_yao == 6 && is_yang_ling)     // 上六 이며 양령일 때
    );
    
    let mut post_lines = pre_lines;
    post_lines[yuan_dang_idx] = !pre_lines[yuan_dang_idx];
    
    let (post_upper, post_lower, post_yuan_dang_idx) = if is_special_sovereign_case {
        // 변이불이 (변하되 바뀌지 않음): 내외괘(상하괘)를 서로 교환하지 않음, 원당효 위치 유지
        let post_lower_lines = [post_lines[0], post_lines[1], post_lines[2]];
        let post_upper_lines = [post_lines[3], post_lines[4], post_lines[5]];
        let pl = lines_to_trigram_num(post_lower_lines);
        let pu = lines_to_trigram_num(post_upper_lines);
        (pu, pl, yuan_dang_idx)
    } else {
        // 일반 케이스: 내외괘(상하괘) 교환 및 원당효 위치 3칸 시프트
        let post_lower_lines = [post_lines[3], post_lines[4], post_lines[5]];
        let post_upper_lines = [post_lines[0], post_lines[1], post_lines[2]];
        let pl = lines_to_trigram_num(post_lower_lines);
        let pu = lines_to_trigram_num(post_upper_lines);
        
        let pyd = if yuan_dang_idx < 3 {
            yuan_dang_idx + 3
        } else {
            yuan_dang_idx - 3
        };
        
        (pu, pl, pyd)
    };
    
    let post_natal_hexagram = get_king_wen_index(post_upper, post_lower);
    
    // 평생 대운 타임라인 생성
    let mut lifetime_cycles = Vec::new();
    let mut current_age = 1;
    
    // 1. 선천괘 주기 (원당효부터 항상 위로 순행)
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
        line_ptr = (line_ptr + 1) % 6; // 항상 위쪽 방향(초->상)으로 진행
    }
    
    // 2. 후천괘 주기 (후천괘 원당효부터 항상 위로 순행)
    let mut post_line_ptr = post_yuan_dang_idx;
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
        post_line_ptr = (post_line_ptr + 1) % 6; // 항상 위쪽 방향으로 진행
    }
    
    // 확장 필드 연산
    let (se_yao, ying_yao) = get_se_ying_yao(pre_natal_hexagram);
    let ti_yong = analyze_ti_yong(pre_upper, pre_lower, yuan_dang_yao);
    
    // 납갑(NaJia) 6효 계산
    let mut najia_lines = Vec::new();
    for i in 1..=6 {
        let (stem, branch) = get_najia_for_line(pre_upper, pre_lower, i);
        najia_lines.push(NaJiaYao { stem, branch });
    }
    
    // 신살(Shinsal) 6효 계산
    let (void_branches, _) = crate::analysis::void::calculate_void_branches(*day_ganzi);
    let mut shinsal_lines = Vec::new();
    for i in 0..6 {
        let branch = najia_lines[i].branch;
        let is_noble = matches!(
            (day_ganzi.stem, branch),
            (HeavenlyStem::Jia | HeavenlyStem::Wu | HeavenlyStem::Geng, EarthlyBranch::Chou | EarthlyBranch::Wei) |
            (HeavenlyStem::Yi | HeavenlyStem::Ji, EarthlyBranch::Zi | EarthlyBranch::Shen) |
            (HeavenlyStem::Bing | HeavenlyStem::Ding, EarthlyBranch::Hai | EarthlyBranch::You) |
            (HeavenlyStem::Ren | HeavenlyStem::Gui, EarthlyBranch::Si | EarthlyBranch::Mao) |
            (HeavenlyStem::Xin, EarthlyBranch::Wu | EarthlyBranch::Yin)
        );
        let is_void = void_branches.contains(&branch);
        let is_rok = matches!(
            (day_ganzi.stem, branch),
            (HeavenlyStem::Jia, EarthlyBranch::Yin) |
            (HeavenlyStem::Yi, EarthlyBranch::Mao) |
            (HeavenlyStem::Bing | HeavenlyStem::Wu, EarthlyBranch::Si) |
            (HeavenlyStem::Ding | HeavenlyStem::Ji, EarthlyBranch::Wu) |
            (HeavenlyStem::Geng, EarthlyBranch::Shen) |
            (HeavenlyStem::Xin, EarthlyBranch::You) |
            (HeavenlyStem::Ren, EarthlyBranch::Hai) |
            (HeavenlyStem::Gui, EarthlyBranch::Zi)
        );
        let is_horse = matches!(
            (year_ganzi.branch, branch),
            (EarthlyBranch::Shen | EarthlyBranch::Zi | EarthlyBranch::Chen, EarthlyBranch::Yin) |
            (EarthlyBranch::Yin | EarthlyBranch::Wu | EarthlyBranch::Xu, EarthlyBranch::Shen) |
            (EarthlyBranch::Si | EarthlyBranch::You | EarthlyBranch::Chou, EarthlyBranch::Hai) |
            (EarthlyBranch::Hai | EarthlyBranch::Mao | EarthlyBranch::Wei, EarthlyBranch::Si)
        );
        shinsal_lines.push(YaoShinsal { is_noble, is_void, is_rok, is_horse });
    }
    
    // 원기(YuanQi) / 화공(HuaGong) 판정
    let yd_idx = (yuan_dang_yao - 1) as usize;
    let yd_branch = najia_lines[yd_idx].branch;
    let yd_element = yd_branch.element();
    
    let nayin_el = year_ganzi.nayin().element();
    let yuan_qi = matches!(
        nayin_el.relation_to(yd_element),
        ElementRelation::Same | ElementRelation::Generates | ElementRelation::GeneratedBy
    );
    
    let month_el = month_ganzi.branch.element();
    let hua_gong = matches!(
        month_el.relation_to(yd_element),
        ElementRelation::Same | ElementRelation::GeneratedBy
    );
    
    let yearly_hexagrams = calculate_yearly_hexagrams(
        pre_natal_hexagram,
        post_natal_hexagram,
        &lifetime_cycles,
        pre_lines,
        post_lines,
    );

    HeLuoResult {
        pre_natal_hexagram,
        post_natal_hexagram,
        yuan_dang_yao,
        lifetime_cycles,
        yuan_qi,
        hua_gong,
        se_yao,
        ying_yao,
        najia_lines,
        shinsal_lines,
        ti_yong,
        yearly_hexagrams,
    }
}

pub fn get_najia_for_line(upper: u8, lower: u8, line_idx: u8) -> (HeavenlyStem, EarthlyBranch) {
    if line_idx <= 3 {
        let idx = (line_idx - 1) as usize;
        match lower {
            1 => (HeavenlyStem::Wu, [EarthlyBranch::Yin, EarthlyBranch::Chen, EarthlyBranch::Wu][idx]),
            2 => (HeavenlyStem::Yi, [EarthlyBranch::Wei, EarthlyBranch::Si, EarthlyBranch::Mao][idx]),
            3 => (HeavenlyStem::Geng, [EarthlyBranch::Zi, EarthlyBranch::Yin, EarthlyBranch::Chen][idx]),
            4 => (HeavenlyStem::Xin, [EarthlyBranch::Chou, EarthlyBranch::Hai, EarthlyBranch::You][idx]),
            6 => (HeavenlyStem::Jia, [EarthlyBranch::Zi, EarthlyBranch::Yin, EarthlyBranch::Chen][idx]),
            7 => (HeavenlyStem::Ding, [EarthlyBranch::Si, EarthlyBranch::Mao, EarthlyBranch::Chou][idx]),
            8 => (HeavenlyStem::Bing, [EarthlyBranch::Chen, EarthlyBranch::Wu, EarthlyBranch::Shen][idx]),
            9 => (HeavenlyStem::Ji, [EarthlyBranch::Mao, EarthlyBranch::Chou, EarthlyBranch::Hai][idx]),
            _ => (HeavenlyStem::Wu, EarthlyBranch::Zi),
        }
    } else {
        let idx = (line_idx - 4) as usize;
        match upper {
            1 => (HeavenlyStem::Wu, [EarthlyBranch::Shen, EarthlyBranch::Xu, EarthlyBranch::Zi][idx]),
            2 => (HeavenlyStem::Gui, [EarthlyBranch::Chou, EarthlyBranch::Hai, EarthlyBranch::You][idx]),
            3 => (HeavenlyStem::Geng, [EarthlyBranch::Wu, EarthlyBranch::Shen, EarthlyBranch::Xu][idx]),
            4 => (HeavenlyStem::Xin, [EarthlyBranch::Wei, EarthlyBranch::Si, EarthlyBranch::Mao][idx]),
            6 => (HeavenlyStem::Ren, [EarthlyBranch::Wu, EarthlyBranch::Shen, EarthlyBranch::Xu][idx]),
            7 => (HeavenlyStem::Ding, [EarthlyBranch::Hai, EarthlyBranch::You, EarthlyBranch::Wei][idx]),
            8 => (HeavenlyStem::Bing, [EarthlyBranch::Xu, EarthlyBranch::Zi, EarthlyBranch::Yin][idx]),
            9 => (HeavenlyStem::Ji, [EarthlyBranch::You, EarthlyBranch::Wei, EarthlyBranch::Si][idx]),
            _ => (HeavenlyStem::Wu, EarthlyBranch::Zi),
        }
    }
}

pub fn get_se_ying_yao(hexagram_index: u8) -> (u8, u8) {
    let se = match hexagram_index {
        1 | 2 | 29 | 30 | 51 | 52 | 57 | 58 => 6,
        9 | 16 | 24 | 44 | 47 | 56 | 60 => 1,
        3 | 13 | 19 | 33 | 37 | 40 | 45 | 46 | 50 => 2,
        7 | 8 | 11 | 12 | 14 | 17 | 18 | 31 | 32 | 41 | 42 | 54 | 53 | 63 | 64 => 3,
        4 | 5 | 6 | 20 | 25 | 26 | 27 | 28 | 34 | 35 | 36 | 38 | 39 | 49 | 59 | 61 | 62 => 4,
        10 | 15 | 21 | 22 | 23 | 43 | 48 | 55 => 5,
        _ => 6,
    };
    let ying = if se <= 3 { se + 3 } else { se - 3 };
    (se, ying)
}

fn get_trigram_element(trigram_num: u8) -> Element {
    match trigram_num {
        1 => Element::Water,
        2 => Element::Earth,
        3 => Element::Wood,
        4 => Element::Wood,
        6 => Element::Metal,
        7 => Element::Metal,
        8 => Element::Earth,
        9 => Element::Fire,
        _ => Element::Earth,
    }
}

pub fn analyze_ti_yong(upper: u8, lower: u8, yuan_dang_yao: u8) -> TiYongRelation {
    let (ti_trigram, yong_trigram) = if yuan_dang_yao <= 3 {
        (upper, lower)
    } else {
        (lower, upper)
    };
    
    let ti_element = get_trigram_element(ti_trigram);
    let yong_element = get_trigram_element(yong_trigram);
    let relationship = ti_element.relation_to(yong_element);
    
    TiYongRelation {
        ti_element,
        yong_element,
        relationship,
    }
}

pub fn calculate_yearly_hexagrams(
    _pre_natal_hexagram: u8,
    _post_natal_hexagram: u8,
    lifetime_cycles: &[HeLuoCycle],
    pre_lines: [bool; 6],
    post_lines: [bool; 6],
) -> Vec<YearlyHexagramResult> {
    let mut results = Vec::new();
    
    for age in 1..=100 {
        if let Some(cycle) = lifetime_cycles.iter().find(|c| age >= c.start_age && age <= c.end_age) {
            let offset = age - cycle.start_age;
            let start_line = cycle.line_index;
            
            let yearly_line_idx = ((start_line - 1) as u32 + offset) % 6;
            let yearly_line = (yearly_line_idx + 1) as u8;
            
            let base_lines = if cycle.is_pre_natal { pre_lines } else { post_lines };
            
            let mut changed_lines = base_lines;
            changed_lines[yearly_line_idx as usize] = !base_lines[yearly_line_idx as usize];
            
            let lower_lines = [changed_lines[0], changed_lines[1], changed_lines[2]];
            let upper_lines = [changed_lines[3], changed_lines[4], changed_lines[5]];
            let pl = lines_to_trigram_num(lower_lines);
            let pu = lines_to_trigram_num(upper_lines);
            let yearly_hex_idx = get_king_wen_index(pu, pl);
            
            let mut monthly_hexagrams = Vec::new();
            for m in 1..=12 {
                let m_line_idx = (yearly_line_idx + (m - 1) as u32) % 6;
                let mut m_lines = changed_lines;
                m_lines[m_line_idx as usize] = !changed_lines[m_line_idx as usize];
                
                let m_lower = [m_lines[0], m_lines[1], m_lines[2]];
                let m_upper = [m_lines[3], m_lines[4], m_lines[5]];
                let ml = lines_to_trigram_num(m_lower);
                let mu = lines_to_trigram_num(m_upper);
                monthly_hexagrams.push(get_king_wen_index(mu, ml));
            }
            
            results.push(YearlyHexagramResult {
                age,
                hexagram_index: yearly_hex_idx,
                yearly_line,
                monthly_hexagrams,
            });
        }
    }
    
    results
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

    #[test]
    fn test_heluo_case_1964_04_18() {
        // 1964년 음력 3월 7일 巳시 (양력 1964년 4월 18일)
        // 갑진년 무진월 정유일 을사시 (시두법 상 정일 기사시가 아닌 을사시가 맞음)
        let year_gz = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Chen);
        let month_gz = GanZi::new(HeavenlyStem::Wu, EarthlyBranch::Chen);
        let day_gz = GanZi::new(HeavenlyStem::Ding, EarthlyBranch::You);
        let hour_gz = GanZi::new(HeavenlyStem::Yi, EarthlyBranch::Si);

        let res = calculate_heluo(1964, true, &year_gz, &month_gz, &day_gz, &hour_gz);
        
        // 선천괘와 후천괘 출력
        println!("1964-04-18 Pre-natal: {}, Post-natal: {}, Yuan Dang: {}", 
                 res.pre_natal_hexagram, res.post_natal_hexagram, res.yuan_dang_yao);

        assert_eq!(res.pre_natal_hexagram, 50); // 화풍정 (火風鼎)
        assert_eq!(res.post_natal_hexagram, 9);  // 풍천소축 (風天小畜)
        assert_eq!(res.yuan_dang_yao, 5);
    }
}
