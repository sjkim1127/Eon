//! Saju-VM: 인생 경로 에뮬레이션을 위한 가상 머신
//!
//! 사주 원국을 불변의 상태로 두고, 대운과 세운을 입력값(Instructions)으로 받아
//! 실시간 인생 에너지 상태를 시뮬레이션합니다.

use serde::{Deserialize, Serialize};
use crate::ganzi::GanZi;
use crate::pillars::FourPillars;
use crate::dynamic_luck::DynamicLuckAnalysis;
use crate::yongshin::YongshinAnalysis;
use crate::element::Element;
use crate::signatures::{LuckSignature, SignatureScanner};

/// 인생의 한 지점(1년 단위)의 시뮬레이션 결과
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifeFrame {
    /// 나이
    pub age: u32,
    pub ganzi: GanZi,         // 그 해의 세운
    pub major_ganzi: GanZi,   // 당시의 대운
    pub score: f32,           // 운세 점수 (0~100)
    pub tags: Vec<String>,    // 주요 특징 (용신운, 충, 합 등)
    pub signatures: Vec<LuckSignature>, // 탐지된 운세 패턴
}

/// Saju Virtual Machine
#[derive(Debug, Clone)]
pub struct SajuVM {
    /// 원국 (고정 컨텍스트)
    pub natal: FourPillars,
    /// 용신 분석 결과 (고정 기준)
    pub yongshin: YongshinAnalysis,
}

impl SajuVM {
    pub fn new(natal: FourPillars) -> Self {
        let yongshin = natal.yongshin();
        Self { natal, yongshin }
    }

    pub fn step(&self, age: u32, major_ganzi: GanZi, yearly_ganzi: GanZi) -> LifeFrame {
        let dynamic = DynamicLuckAnalysis::analyze(&self.natal, Some(major_ganzi), Some(yearly_ganzi));
        
        let mut tags = Vec::new();
        let score = self.evaluate_score(&dynamic, &mut tags);
        
        let mut frame = LifeFrame {
            age,
            ganzi: yearly_ganzi,
            major_ganzi,
            score,
            tags,
            signatures: Vec::new(),
        };

        // 시그니처 스캔
        frame.signatures = SignatureScanner::scan(&frame, &dynamic);

        frame
    }

    /// 점수 평가 로직 (Precision Tuning)
    /// - Data Flow Analysis처럼 원국의 상태와 운의 상호작용을 정밀 추적
    fn evaluate_score(&self, dynamic: &DynamicLuckAnalysis, tags: &mut Vec<String>) -> f32 {
        let mut score = 50.0;
        
        let primary_yongshin = self.yongshin.primary;
        let assistant_yongshin = self.yongshin.assistant;

        // 1. 조후 컨텍스트 계산 (가중치 매트릭스)
        let thermal_index = crate::yongshin::calculate_thermal_index(&self.natal);
        
        // 2. 실질 오행(Effective Elements) 추출 
        // 합(Hap)에 의해 변한 기운이 있으면 그 기운을 기준으로 평가 (Data Flow)
        let major_stem_eff = self.get_effective_stem(dynamic, "대운천간");
        let yearly_stem_eff = self.get_effective_stem(dynamic, "세운천간");

        if let Some(major) = dynamic.major_influence.as_ref() {
            let el = major_stem_eff.unwrap_or(major.ganzi.stem.element());
            let weight = self.get_element_priority(el, primary_yongshin, assistant_yongshin, thermal_index);
            
            let increment = 10.0 * weight;
            score += increment;
            
            if weight >= 1.5 { tags.push("대운핵심운".to_string()); }
            else if weight >= 1.0 { tags.push("대운길운".to_string()); }
            else if weight <= -0.5 { tags.push("대운기신운".to_string()); }
        }
        
        if let Some(yearly) = dynamic.yearly_influence.as_ref() {
            let el = yearly_stem_eff.unwrap_or(yearly.ganzi.stem.element());
            let weight = self.get_element_priority(el, primary_yongshin, assistant_yongshin, thermal_index);
            
            let increment = 15.0 * weight;
            score += increment;

            if weight >= 1.5 { tags.push("세운핵심운".to_string()); }
            else if weight >= 1.0 { tags.push("세운길운".to_string()); }
            else if weight <= -0.5 { tags.push("세운기신운".to_string()); }
        }

        // 3. 지지 합충 분석 (Structural Stability)
        for (clash, p1, p2) in &dynamic.combined_relations.branch_clashes {
            if p1.contains("세운") || p2.contains("세운") || p1.contains("대운") || p2.contains("대운") {
                // 용신을 치는 충은 페널티가 훨씬 큼
                let is_yongshin_clash = self.is_involving_yongshin(p1, p2, primary_yongshin);
                let penalty = if is_yongshin_clash { 20.0 } else { 8.0 };
                score -= penalty;
                tags.push(clash.hangul().to_string());
            }
        }

        // 4. 합에 의한 기운 안정화
        if !dynamic.combined_relations.stem_combinations.is_empty() {
            score += 5.0;
            // 합화가 용신으로 변했다면 추가 가점
            if major_stem_eff == Some(primary_yongshin) || yearly_stem_eff == Some(primary_yongshin) {
                score += 10.0;
                tags.push("용신합화".to_string());
            }
        }

        score.clamp(0.0, 100.0)
    }

    /// 특정 간지가 용신에 해당하는지 체크
    fn is_involving_yongshin(&self, p1: &str, p2: &str, yongshin: Element) -> bool {
        let get_el = |p: &str| {
            match p {
                "년간" => Some(self.natal.year.stem.element()),
                "년지" => Some(self.natal.year.branch.element()),
                "월간" => Some(self.natal.month.stem.element()),
                "월지" => Some(self.natal.month.branch.element()),
                "일간" => Some(self.natal.day.stem.element()),
                "일지" => Some(self.natal.day.branch.element()),
                "시간" => Some(self.natal.hour.stem.element()),
                "시지" => Some(self.natal.hour.branch.element()),
                _ => None
            }
        };
        get_el(p1) == Some(yongshin) || get_el(p2) == Some(yongshin)
    }

    /// 합화에 의한 실질 천간 오행 추출
    fn get_effective_stem(&self, dynamic: &DynamicLuckAnalysis, target: &str) -> Option<Element> {
        for (combo, p1, p2) in &dynamic.combined_relations.stem_combinations {
            if p1 == target || p2 == target {
                // 월지가 합화되는 오행을 돕는지 확인 (TransformationAnalysis 로직과 유사)
                let transformed = combo.transformed_element();
                let month_el = self.natal.month.branch.element();
                if month_el == transformed || month_el.generates() == transformed {
                    return Some(transformed);
                }
            }
        }
        None
    }

    /// 오행별 우선순위 가중치 (Thermal + Yongshin Matrix)
    fn get_element_priority(&self, el: Element, primary: Element, assistant: Element, thermal: i32) -> f32 {
        let mut multiplier = 0.0;

        if el == primary { multiplier += 1.0; }
        else if el == assistant { multiplier += 0.5; }
        else if el == primary.controlled_by() { multiplier -= 0.8; } // 기신 (용신을 극함)
        
        // 조후 가중치 보정
        // 추운 사주(-30 이하)에서 火는 가중치 2배
        if thermal <= -30 && el == Element::Fire {
            multiplier *= 2.0;
            if multiplier == 0.0 { multiplier = 1.2; } // 용신은 아니지만 조후로 필요한 경우
        }
        // 더운 사주(30 이상)에서 水는 가중치 2배
        if thermal >= 30 && el == Element::Water {
            multiplier *= 2.0;
            if multiplier == 0.0 { multiplier = 1.2; }
        }

        multiplier
    }
}
