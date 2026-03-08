//! 신강신약(身强身弱, Day Master Strength) 판정
//!
//! 일간(日干)의 강약을 판단합니다.
//!
//! ## 판정 기준
//!
//! 1. **득령(得令)** - 월지가 일간을 생(生) 또는 비(比)하는가
//! 2. **득지(得地)** - 지지에 일간의 뿌리(통근)가 있는가
//! 3. **득시(得時)** - 시지가 일간을 생(生) 또는 비(比)하는가
//! 4. **득세(得勢)** - 비겁(比劫)/인성(印星)이 많은가
//!
//! ## 강약 판정
//!
//! - 4가지 중 2가지 이상 만족 → 신강(身强)
//! - 4가지 중 1가지 이하 만족 → 신약(身弱)

use serde::{Deserialize, Serialize};
use crate::core::stem::HeavenlyStem;
use crate::core::branch::EarthlyBranch;
use crate::core::element::Element;
use crate::core::pillars::FourPillars;
use crate::core::ten_gods::TenGod;
use crate::analysis::relationships::RelationshipAnalysis;

use crate::core::config::AnalysisConfig;

/// 강약 유형
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StrengthType {
    /// 신강(身强) - 강한 사주
    Strong,
    /// 신약(身弱) - 약한 사주
    Weak,
    /// 중화(中和) - 균형 잡힌 사주 (드묾)
    Balanced,
}

impl StrengthType {
    pub const fn hangul(&self) -> &'static str {
        match self {
            Self::Strong => "신강",
            Self::Weak => "신약",
            Self::Balanced => "중화",
        }
    }

    pub const fn hanja(&self) -> &'static str {
        match self {
            Self::Strong => "身强",
            Self::Weak => "身弱",
            Self::Balanced => "中和",
        }
    }

    pub const fn description(&self) -> &'static str {
        match self {
            Self::Strong => "일간의 힘이 강하므로 식상/재성/관성이 필요합니다.",
            Self::Weak => "일간의 힘이 약하므로 인성/비겁이 필요합니다.",
            Self::Balanced => "일간과 다른 기운이 균형을 이루고 있습니다.",
        }
    }
}

impl std::fmt::Display for StrengthType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hangul())
    }
}

/// 득령(得令) 판정 - 월지와 일간의 관계
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeukRyeong {
    /// 득령 여부
    pub acquired: bool,
    /// 월지
    pub month_branch: EarthlyBranch,
    /// 월지 오행
    pub month_element: Element,
    /// 일간 오행
    pub day_master_element: Element,
}

impl DeukRyeong {
    /// 득령 판정
    /// 월지 오행이 일간을 생하거나 같으면 득령
    pub fn check(day_master: HeavenlyStem, month_branch: EarthlyBranch) -> Self {
        let day_element = day_master.element();
        let month_element = month_branch.element();
        
        // 월지가 일간을 생(生)하거나 비(比)하면 득령
        let is_generating = month_element.generates() == day_element;
        let is_same = month_element == day_element;
        let acquired = is_generating || is_same;

        Self {
            acquired,
            month_branch,
            month_element,
            day_master_element: day_element,
        }
    }

    /// 실질 오행을 반영한 득령 판정
    pub fn check_effective(day_master: HeavenlyStem, month_branch: EarthlyBranch, month_eff: Element) -> Self {
        let day_element = day_master.element();
        
        // 월지의 실질 오행이 일간을 생(生)하거나 비(比)하면 득령
        let is_generating = month_eff.generates() == day_element;
        let is_same = month_eff == day_element;
        let acquired = is_generating || is_same;

        Self {
            acquired,
            month_branch,
            month_element: month_eff,
            day_master_element: day_element,
        }
    }

    /// 관계 설명
    pub fn relation(&self) -> &'static str {
        let is_generating = self.month_element.generates() == self.day_master_element;
        let is_same = self.month_element == self.day_master_element;
        
        if is_generating {
            "월지가 일간을 생함 (生)"
        } else if is_same {
            "월지와 일간이 같은 오행 (比)"
        } else if self.day_master_element.generates() == self.month_element {
            "일간이 월지를 생함 (泄)"
        } else if self.day_master_element.controls() == self.month_element {
            "일간이 월지를 극함 (剋)"
        } else {
            "월지가 일간을 극함 (剋)"
        }
    }
}

/// 득지(得地) 판정 - 지지에 통근 여부
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeukJi {
    /// 득지 여부
    pub acquired: bool,
    /// 통근(通根) 개수
    pub root_count: u8,
    /// 통근(通根) 가중치 점수 합계
    pub root_score: f32,
    /// 통근 위치들
    pub root_positions: Vec<String>,
    /// 강한 12운성 개수 (A급 또는 B급)
    pub strong_stage_count: u8,
    /// 12운성 가중치 합계 (A급=1.0, B급=0.5, C급=0.0)
    pub stage_weight_sum: f32,
}

impl DeukJi {
    /// 득지 판정 (기존 호환성 유지)
    pub fn check(pillars: &FourPillars, config: &AnalysisConfig) -> Self {
        let relations = RelationshipAnalysis::from_pillars(pillars);
        Self::check_with_relations(pillars, &relations, config)
    }

    /// 득지 판정 (합충 관계 반영)
    pub fn check_with_relations(pillars: &FourPillars, relations: &RelationshipAnalysis, config: &AnalysisConfig) -> Self {
        let day_element = pillars.day_master().element();
        
        let branches = [
            ("년지", pillars.year.branch),
            ("월지", pillars.month.branch),
            ("일지", pillars.day.branch),
            ("시지", pillars.hour.branch),
        ];
        
        let mut root_count = 0;
        let mut root_score = 0.0;
        let mut root_positions = Vec::new();
        let mut strong_stage_count = 0;
        let mut stage_weight_sum = 0.0_f32;
        
        // 충돌 정보를 위치별로 매핑 (충 종류 포함)
        use std::collections::HashMap;
        let mut clash_info: HashMap<&str, &crate::analysis::relationships::BranchClash> = HashMap::new();
        for (clash, pos1, pos2) in &relations.branch_clashes {
            clash_info.insert(pos1.as_str(), clash);
            clash_info.insert(pos2.as_str(), clash);
        }
        
        for (name, branch) in &branches {
            // 위치별 기본 가중치
            let base_weight = match *name {
                "월지" => config.weights.month_branch,
                "일지" => config.weights.day_branch,
                _ => config.weights.other_branch, // 년지, 시지
            };
            
            // 충(Clash) 발생 시 충 종류에 따른 차등 감산율 적용
            let (weight, clash_label) = if let Some(clash) = clash_info.get(*name) {
                let damage = clash.damage_ratio();
                let adjusted = base_weight * (1.0 - damage);
                (adjusted, Some(format!("{} ({})", name, clash.clash_type().hangul())))
            } else {
                (base_weight, None)
            };

            // 지장간(Hidden Stems) 전체를 확인하여 통근 여부 및 세기 분석
            let hidden_stems = branch.hidden_stems();
            let mut branch_root_score = 0.0;
            let saryeong = if *name == "월지" { Some(pillars.saryeong()) } else { None };

            for (idx, stem) in hidden_stems.iter().enumerate() {
                if stem.element() == day_element {
                    // 기운의 종류에 따른 가중치 (정기: 1.0, 중기: 0.6, 여기: 0.3)
                    let part_weight = if idx == hidden_stems.len() - 1 { 
                        config.root.main_root
                    } else if hidden_stems.len() == 3 && idx == 1 {
                        config.root.middle_root
                    } else {
                        config.root.remain_root
                    };

                    // 사령(Commanding) 여부 확인 (월지만 해당, 가중치 20% 보정)
                    let saryeong_multiplier = if let Some(ref s) = saryeong {
                        if s.commanding_stem == *stem { config.root.saryeong_bonus } else { 1.0 }
                    } else { 1.0 };

                    branch_root_score += (weight * part_weight * saryeong_multiplier).min(weight);
                }
            }
            
            if branch_root_score > 0.0 {
                root_count += 1;
                root_score += branch_root_score;
                if let Some(label) = clash_label {
                    root_positions.push(label);
                } else {
                    root_positions.push(name.to_string());
                }
            }
            
            // 12운성 확인 및 가중치 계산
            let stage = crate::core::twelve_stages::calculate_twelve_stage(
                pillars.day_master(), 
                *branch
            );
            
            // 12운성 가중치 누적 (A급=1.0, B급=0.5, C급=0.0)
            stage_weight_sum += stage.root_weight();
            
            // 강한 12운성 개수 (A급 또는 B급)
            if stage.is_strong() {
                strong_stage_count += 1;
            }
        }
        
        // 득지 판정: 
        // 1) 12운성 가중치 합이 1.0 이상 (A급 1개 또는 B급 2개 이상)
        // 2) 또는 통근 점수가 설정된 최소 점수 이상
        let acquired = stage_weight_sum >= 1.0 || root_score >= config.root.min_deuk_ji_score;
        
        Self {
            acquired,
            root_count,
            root_score,
            root_positions,
            strong_stage_count,
            stage_weight_sum,
        }
    }
}

/// 득시(得時) 판정 - 시지와 일간의 관계
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeukSi {
    /// 득시 여부
    pub acquired: bool,
    /// 시지
    pub hour_branch: EarthlyBranch,
    /// 시지 오행
    pub hour_element: Element,
    /// 일간 오행
    pub day_master_element: Element,
}

impl DeukSi {
    /// 득시 판정
    /// 시지 오행이 일간을 생하거나 같으면 득시
    pub fn check(day_master: HeavenlyStem, hour_branch: EarthlyBranch) -> Self {
        let day_element = day_master.element();
        let hour_element = hour_branch.element();
        
        // 시지가 일간을 생(生)하거나 비(比)하면 득시
        let is_generating = hour_element.generates() == day_element;
        let is_same = hour_element == day_element;
        let acquired = is_generating || is_same;

        Self {
            acquired,
            hour_branch,
            hour_element,
            day_master_element: day_element,
        }
    }

    /// 실질 오행을 반영한 득시 판정
    pub fn check_effective(day_master: HeavenlyStem, hour_branch: EarthlyBranch, hour_eff: Element) -> Self {
        let day_element = day_master.element();
        
        // 시지의 실질 오행이 일간을 생(生)하거나 비(比)하면 득시
        let is_generating = hour_eff.generates() == day_element;
        let is_same = hour_eff == day_element;
        let acquired = is_generating || is_same;

        Self {
            acquired,
            hour_branch,
            hour_element: hour_eff,
            day_master_element: day_element,
        }
    }

    /// 관계 설명
    pub fn relation(&self) -> &'static str {
        let is_generating = self.hour_element.generates() == self.day_master_element;
        let is_same = self.hour_element == self.day_master_element;
        
        if is_generating {
            "시지가 일간을 생함 (生)"
        } else if is_same {
            "시지와 일간이 같은 오행 (比)"
        } else if self.day_master_element.generates() == self.hour_element {
            "일간이 시지를 생함 (泄)"
        } else if self.day_master_element.controls() == self.hour_element {
            "일간이 시지를 극함 (剋)"
        } else {
            "시지가 일간을 극함 (剋)"
        }
    }
}

/// 득세(得勢) 판정 - 비겁/인성의 숫자
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeukSe {
    /// 득세 여부
    pub acquired: bool,
    /// 비겁(比肩+劫財) 개수
    pub bijie_count: u8,
    /// 인성(正印+偏印) 개수
    pub yinxing_count: u8,
    /// 식상(食神+傷官) 개수
    pub shishang_count: u8,
    /// 재성(正財+偏財) 개수
    pub caisheng_count: u8,
    /// 관성(正官+偏官) 개수
    pub guanxing_count: u8,
    /// 비겁+인성 비율 (%)
    pub support_ratio: f32,
}

impl DeukSe {
    /// 득세 판정
    /// 비겁+인성이 식상+재성+관성보다 많으면 득세
    pub fn check(pillars: &FourPillars, config: &AnalysisConfig) -> Self {
        Self::check_with_options(pillars, false, config)
    }

    /// 옵션에 따른 득세 판정 (Input vs Output 에너지량 비교 추상화)
    pub fn check_with_options(pillars: &FourPillars, apply_transform: bool, config: &AnalysisConfig) -> Self {
        let day_master = pillars.day_master();
        let dm_element = day_master.element();
        let eff_map = if apply_transform { 
            Some(pillars.effective_elements()) 
        } else { None };
        
        let mut total_support_score = 0.0f32;
        let mut bijie_count = 0u8;
        let mut yinxing_count = 0u8;
        let mut shishang_count = 0u8;
        let mut caisheng_count = 0u8;
        let mut guanxing_count = 0u8;

        // 일간 자신 점수 가산
        total_support_score += config.weights.stem;

        // 1. 천간 분석 (일간 제외)
        let stems = [
            (pillars.year.stem, 0),
            (pillars.month.stem, 1),
            (pillars.hour.stem, 3),
        ];

        for (stem, idx) in &stems {
            let element = if let Some(map) = eff_map { map[*idx].1 } else { stem.element() };
            let weight = config.weights.stem;
            
            // 내 편(Input: 인성+비겁) 점수 합산
            if element == dm_element || element == dm_element.generated_by() {
                total_support_score += weight;
            }
            
            // 십성 통계 (기존 로직 유지)
            let god = TenGod::from_stems(day_master, *stem); // 통계는 원국 기준
            match god {
                TenGod::Bijian | TenGod::Jiecai => bijie_count += 1,
                TenGod::Zhengyin | TenGod::Pianyin => yinxing_count += 1,
                TenGod::Shishen | TenGod::Shangguan => shishang_count += 1,
                TenGod::Zhengcai | TenGod::Piancai => caisheng_count += 1,
                TenGod::Zhengguan | TenGod::Pianguan => guanxing_count += 1,
            }
        }
        
        // 2. 지지 분석
        let branch_indices = [
            (pillars.year.branch, config.weights.other_branch, 4),
            (pillars.month.branch, config.weights.month_branch, 5),
            (pillars.day.branch, config.weights.day_branch, 6),
            (pillars.hour.branch, config.weights.other_branch, 7),
        ];

        for (branch, weight, idx) in &branch_indices {
            let element = if let Some(map) = eff_map { map[*idx].1 } else { branch.element() };
            
            // 내 편 점수 합산
            if element == dm_element || element == dm_element.generated_by() {
                total_support_score += *weight;
            }

            let god = TenGod::from_stem_and_branch(day_master, *branch);
            match god {
                TenGod::Bijian | TenGod::Jiecai => bijie_count += 1,
                TenGod::Zhengyin | TenGod::Pianyin => yinxing_count += 1,
                TenGod::Shishen | TenGod::Shangguan => shishang_count += 1,
                TenGod::Zhengcai | TenGod::Piancai => caisheng_count += 1,
                TenGod::Zhengguan | TenGod::Pianguan => guanxing_count += 1,
            }
        }
        
        let support_ratio = (total_support_score / config.weights.total_weight) * 100.0;
        let acquired = total_support_score >= config.strength.deuk_se_threshold;
        
        Self {
            acquired,
            bijie_count,
            yinxing_count,
            shishang_count,
            caisheng_count,
            guanxing_count,
            support_ratio,
        }
    }
}

/// 신강신약 분석 결과
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrengthAnalysis {
    /// 일간
    pub day_master: HeavenlyStem,
    /// 강약 유형
    pub strength_type: StrengthType,
    /// 득령 판정
    pub deuk_ryeong: DeukRyeong,
    /// 득지 판정
    pub deuk_ji: DeukJi,
    /// 득시 판정
    pub deuk_si: DeukSi,
    /// 득세 판정
    pub deuk_se: DeukSe,
    /// 만족 조건 개수 (0-4)
    pub acquired_count: u8,
    /// 신강/신약 점수 (0-100, 50 기준)
    pub strength_score: f32,
}

impl StrengthAnalysis {
    /// 사주로부터 신강신약 분석
    pub fn from_pillars(pillars: &FourPillars) -> Self {
        // 기본적으로 합화 반영
        Self::from_pillars_with_options(pillars, true, &AnalysisConfig::default())
    }

    /// 옵션 및 설정을 기반으로 한 신강신약 분석
    pub fn from_pillars_with_options(pillars: &FourPillars, apply_transform: bool, config: &AnalysisConfig) -> Self {
        let day_master = pillars.day_master();
        let relations = RelationshipAnalysis::from_pillars(pillars);
        let eff_map = if apply_transform { Some(pillars.effective_elements()) } else { None };

        let deuk_ryeong = if let Some(map) = eff_map {
            DeukRyeong::check_effective(day_master, pillars.month.branch, map[5].1)
        } else {
            DeukRyeong::check(day_master, pillars.month.branch)
        };

        let deuk_ji = DeukJi::check_with_relations(pillars, &relations, config);
        
        let deuk_si = if let Some(map) = eff_map {
            DeukSi::check_effective(day_master, pillars.hour.branch, map[7].1)
        } else {
            DeukSi::check(day_master, pillars.hour.branch)
        };

        let deuk_se = DeukSe::check_with_options(pillars, apply_transform, config);
        
        // 만족 조건 개수 (4가지)
        let acquired_count = 
            (deuk_ryeong.acquired as u8) + 
            (deuk_ji.acquired as u8) + 
            (deuk_si.acquired as u8) +
            (deuk_se.acquired as u8);
        
        // 강약 판정
        let strength_type = match acquired_count {
            2..=4 => StrengthType::Strong,
            1 | 0 => StrengthType::Weak,
            _ => StrengthType::Balanced,
        };
        
        // 종합 점수 계산
        let score_ryeong = if deuk_ryeong.acquired { config.strength.criteria_score } else { 0.0 };
        let score_ji = if deuk_ji.acquired { config.strength.criteria_score } else { 0.0 };
        let score_si = if deuk_si.acquired { config.strength.criteria_score } else { 0.0 };
        let score_se = deuk_se.support_ratio * config.strength.deuk_se_weight;
        let strength_score = score_ryeong + score_ji + score_si + score_se;
        
        Self {
            day_master,
            strength_type,
            deuk_ryeong,
            deuk_ji,
            deuk_si,
            deuk_se,
            acquired_count,
            strength_score,
        }
    }

    /// 용신(用神) 추천 (오행 그룹 반환)
    pub fn recommend_yongshin(&self) -> Vec<Element> {
        let dm_element = self.day_master.element();
        match self.strength_type {
            StrengthType::Strong => {
                // 식상 / 재성 / 관성 오행들
                vec![
                    dm_element.generates(),       // 식상
                    dm_element.controls(),        // 재성
                    dm_element.controlled_by(),   // 관성
                ]
            }
            StrengthType::Weak => {
                // 인성 / 비겁 오행들
                vec![
                    dm_element.generated_by(),    // 인성
                    dm_element,                   // 비겁
                ]
            }
            StrengthType::Balanced => {
                vec![]
            }
        }
    }
}

impl std::fmt::Display for StrengthAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "【신강/신약 분석】")?;
        writeln!(f, "─────────────────────────────────")?;
        writeln!(f, "일간: {} ({})", self.day_master.hanja(), self.day_master.element().hangul())?;
        writeln!(f)?;
        
        // 득령
        let ryeong_mark = if self.deuk_ryeong.acquired { "○" } else { "✗" };
        writeln!(f, "득령(得令): {} - {}", ryeong_mark, self.deuk_ryeong.relation())?;
        
        // 득지
        let ji_mark = if self.deuk_ji.acquired { "○" } else { "✗" };
        writeln!(f, "득지(得地): {} - 통근 {}개, 강한 운성 {}개",
            ji_mark,
            self.deuk_ji.root_count,
            self.deuk_ji.strong_stage_count)?;
        
        // 득시
        let si_mark = if self.deuk_si.acquired { "○" } else { "✗" };
        writeln!(f, "득시(得時): {} - {}", si_mark, self.deuk_si.relation())?;
        
        // 득세
        let se_mark = if self.deuk_se.acquired { "○" } else { "✗" };
        writeln!(f, "득세(得勢): {} - 비겁+인성 {:.1}%",
            se_mark,
            self.deuk_se.support_ratio)?;
        
        writeln!(f)?;
        writeln!(f, "【판정 결과】")?;
        writeln!(f, "조건 충족: {}/4", self.acquired_count)?;
        writeln!(f, "종합 점수: {:.1}%", self.strength_score)?;
        writeln!(f)?;
        writeln!(f, "▶ {} ({})", self.strength_type.hangul(), self.strength_type.hanja())?;
        writeln!(f, "  {}", self.strength_type.description())?;
        
        writeln!(f)?;
        let yongshin_names: Vec<String> = self.recommend_yongshin().iter()
            .map(|e| e.hangul().to_string())
            .collect();
        writeln!(f, "용신(추천 오행): {}", yongshin_names.join(", "))?;
        
        Ok(())
    }
}

// ============================================
// FourPillars 편의 메서드
// ============================================

impl FourPillars {
    /// 신강/신약 분석
    pub fn strength(&self) -> StrengthAnalysis {
        StrengthAnalysis::from_pillars(self)
    }

    /// 설정을 포함한 신강/신약 분석
    pub fn strength_with_config(&self, config: &AnalysisConfig) -> StrengthAnalysis {
        StrengthAnalysis::from_pillars_with_options(self, true, config)
    }

    /// 신강 여부
    pub fn is_strong(&self) -> bool {
        self.strength().strength_type == StrengthType::Strong
    }

    /// 신약 여부
    pub fn is_weak(&self) -> bool {
        self.strength().strength_type == StrengthType::Weak
    }
}

use crate::analysis::Analyzable;

impl Analyzable for StrengthAnalysis {
    type Output = StrengthAnalysis;
    fn analyze(pillars: &FourPillars, config: &AnalysisConfig) -> Self::Output {
        StrengthAnalysis::from_pillars_with_options(pillars, true, config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::pillars::SajuInput;

    #[test]
    fn test_user_strength() {
        // 김성주님 사주: 甲申年 乙亥月 庚戌日 丁亥時
        // 일간 庚(금), 월지 亥(수), 시지 亥(수)
        let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
        let pillars = FourPillars::calculate(&input).unwrap();
        
        let analysis = pillars.strength();
        
        println!("{}", analysis);
        
        // 사용자 만세력 결과: 신약
        // 득령: ✗ (亥水가 庚金을 설기)
        // 득지: ○ (申에 건록)
        // 득시: ✗ (亥水가 庚金을 설기)
        // 득세: ✗
        assert!(!analysis.deuk_ryeong.acquired);
        assert!(analysis.deuk_ji.acquired);
        assert!(!analysis.deuk_si.acquired);
        assert_eq!(analysis.strength_type, StrengthType::Weak);
    }

    #[test]
    fn test_deuk_ryeong() {
        // 庚일간, 申월 → 득령 (같은 금)
        let dr = DeukRyeong::check(HeavenlyStem::Geng, EarthlyBranch::Shen);
        assert!(dr.acquired);
        
        // 庚일간, 亥월 → 비득령 (금 → 수 설기)
        let dr2 = DeukRyeong::check(HeavenlyStem::Geng, EarthlyBranch::Hai);
        assert!(!dr2.acquired);
    }

    #[test]
    fn test_deuk_si() {
        // 庚일간, 申시 → 득시 (같은 금)
        let ds = DeukSi::check(HeavenlyStem::Geng, EarthlyBranch::Shen);
        assert!(ds.acquired);
        
        // 庚일간, 亥시 → 비득시 (금 → 수 설기)
        let ds2 = DeukSi::check(HeavenlyStem::Geng, EarthlyBranch::Hai);
        assert!(!ds2.acquired);
    }

    #[test]
    fn test_four_criteria() {
        let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
        let pillars = FourPillars::calculate(&input).unwrap();
        let analysis = pillars.strength();
        
        // 4가지 기준 모두 확인
        println!("득령: {}", analysis.deuk_ryeong.acquired);
        println!("득지: {}", analysis.deuk_ji.acquired);
        println!("득시: {}", analysis.deuk_si.acquired);
        println!("득세: {}", analysis.deuk_se.acquired);
        println!("총 충족: {}/4", analysis.acquired_count);
    }
}
