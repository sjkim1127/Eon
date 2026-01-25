//! Saju-VM: 인생 경로 에뮬레이션을 위한 가상 머신
//!
//! 사주 원국을 불변의 상태로 두고, 대운과 세운을 입력값(Instructions)으로 받아
//! 실시간 인생 에너지 상태를 시뮬레이션합니다.

use serde::{Deserialize, Serialize};
use crate::core::ganzi::GanZi;
use crate::core::pillars::FourPillars;
use crate::analysis::dynamic_luck::DynamicLuckAnalysis;
use crate::analysis::yongshin::YongshinAnalysis;
use crate::core::element::Element;
use crate::engine::signatures::{LuckSignature, SignatureScanner};
use crate::analysis::spirit_markers::{SpiritMarker, SpiritMarkerAnalysis};

/// Saju Interrupt (하드웨어 예외/인터럽트)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SajuInterrupt {
    /// 0x01: 고우선순위 시스템 충돌 (백호살 등)
    CriticalException,
    /// 0x02: 리소스 오버플로우 (괴강살 등)
    ResourceOverflow,
    /// 0x03: 시스템 정지/고독 (고신/과숙 등)
    SystemStall,
    /// 0x04: 임시 서비스 중단 (망신/겁살 등)
    ServiceInterrupt,
}

/// 오행 레지스터 (R0~R4)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct QiRegisters {
    /// R0: 목 (Wood)
    pub r0_wood: f32,
    /// R1: 화 (Fire)
    pub r1_fire: f32,
    /// R2: 토 (Earth)
    pub r2_earth: f32,
    /// R3: 금 (Metal)
    pub r3_metal: f32,
    /// R4: 수 (Water)
    pub r4_water: f32,
}

impl QiRegisters {
    pub fn new() -> Self {
        Self {
            r0_wood: 0.0,
            r1_fire: 0.0,
            r2_earth: 0.0,
            r3_metal: 0.0,
            r4_water: 0.0,
        }
    }

    pub fn update(&mut self, el: Element, value: f32) {
        match el {
            Element::Wood => self.r0_wood += value,
            Element::Fire => self.r1_fire += value,
            Element::Earth => self.r2_earth += value,
            Element::Metal => self.r3_metal += value,
            Element::Water => self.r4_water += value,
        }
    }
}

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
    /// ESIL 트레이스 (명령어 실행 로그)
    pub esil_trace: String,
    /// 사건 종료 후의 레지스터 상태
    pub register_state: QiRegisters,
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

    pub fn step(
        &self, 
        age: u32, 
        major_ganzi: GanZi, 
        yearly_ganzi: GanZi,
        monthly_ganzi: Option<GanZi>,
        daily_ganzi: Option<GanZi>,
        hourly_ganzi: Option<GanZi>,
    ) -> LifeFrame {
        let dynamic = DynamicLuckAnalysis::analyze(
            &self.natal, 
            Some(major_ganzi), 
            Some(yearly_ganzi),
            monthly_ganzi,
            daily_ganzi,
            hourly_ganzi,
        );
        
        let mut tags = Vec::new();
        let mut esil_trace = String::new();
        let mut registers = QiRegisters::new();
        
        let score = self.evaluate_score(&dynamic, &mut tags, &mut esil_trace, &mut registers);
        
        let mut frame = LifeFrame {
            age,
            ganzi: yearly_ganzi,
            major_ganzi,
            score,
            tags,
            signatures: Vec::new(),
            esil_trace,
            register_state: registers,
        };

        // 시그니처 스캔
        frame.signatures = SignatureScanner::scan(&frame, &dynamic);

        frame
    }

    /// 점수 평가 로직 (Precision Tuning)
    /// - Data Flow Analysis처럼 원국의 상태와 운의 상호작용을 정밀 추적
    fn evaluate_score(
        &self, 
        dynamic: &DynamicLuckAnalysis, 
        tags: &mut Vec<String>,
        esil_trace: &mut String,
        registers: &mut QiRegisters,
    ) -> f32 {
        let mut score = 50.0;
        
        // 0. 하드웨어 인터럽트 체크 (Interrupt Vector Table Scan)
        self.handle_interrupts(dynamic, tags, esil_trace, registers, &mut score);

        // 1. 파이프라인 에뮬레이션 (Instruction Pipeline Execution)
        // 대운(Fetch) -> 세운(Decode) -> 월운(Execute) -> 일운(Memory) -> 시운(WriteBack)
        self.execute_pipeline(dynamic, tags, esil_trace, registers, &mut score);

        let primary_yongshin = self.yongshin.primary;
        let assistant_yongshin = self.yongshin.assistant;
        let thermal_index = crate::analysis::yongshin::calculate_thermal_index(&self.natal);

        // 3. 지지 합충 분석 (Structural Stability)
        for (clash, p1, p2) in &dynamic.combined_relations.branch_clashes {
            if p1.contains("세운") || p2.contains("세운") || p1.contains("대운") || p2.contains("대운") {
                // 용신을 치는 충은 페널티가 훨씬 큼
                let is_yongshin_clash = self.is_involving_yongshin(p1, p2, primary_yongshin);
                let penalty = if is_yongshin_clash { 20.0 } else { 8.0 };
                score -= penalty;
                
                esil_trace.push_str(&format!("clash:{}-{},penalty:-{:.1}; ", p1, p2, penalty));
                
                // 지장간 메모리 덤프 (Memory Dump of Hidden Stems)
                // 충돌이 발생한 지지의 지장간 기운을 해방하여 시스템에 반영
                let b1_obj = self.get_branch_by_path(p1, dynamic);
                let b2_obj = self.get_branch_by_path(p2, dynamic);
                
                for b in [b1_obj, b2_obj].iter().flatten() {
                    let hidden_stems = b.jijanggan();
                    for stem in hidden_stems {
                        let el = stem.element();
                        let weight = self.get_element_priority(el, primary_yongshin, assistant_yongshin, thermal_index);
                        
                        // 지장간 기운은 겉으로 드러난 기운보다 영향력은 작지만(가중치 0.3) 
                        // 충에 의해 해방될 때 시스템에 변수를 제공함
                        let bonus = 3.0 * weight;
                        score += bonus;
                        registers.update(el, bonus);
                        
                        if weight.abs() > 0.1 {
                            esil_trace.push_str(&format!("mem_dump:{}({}),bonus:{:.1}; ", b.hangul(), stem.hangul(), bonus));
                        }
                    }
                }
                tags.push(clash.hangul().to_string());
            }
        }

        // 4. 지지 형 (刑) - Memory Corruption
        for (pun, p1, p2) in &dynamic.combined_relations.branch_punishments {
            let penalty = 5.0;
            score -= penalty;
            esil_trace.push_str(&format!("mem_corrupt:{}-{},penalty:-{:.1}; ", p1, p2, penalty));
            tags.push(pun.hangul());
        }

        // 5. 지지 해 (害) - Race Condition
        for (harm, p1, p2) in &dynamic.combined_relations.branch_harms {
            let penalty = 3.0;
            score -= penalty;
            esil_trace.push_str(&format!("race_cond:{}-{},penalty:-{:.1}; ", p1, p2, penalty));
            tags.push(harm.hangul().to_string());
        }

        // 6. 지지 파 (破) - IO Error
        for (dest, p1, p2) in &dynamic.combined_relations.branch_destructions {
            let penalty = 2.0;
            score -= penalty;
            esil_trace.push_str(&format!("io_error:{}-{},penalty:-{:.1}; ", p1, p2, penalty));
            tags.push(dest.hangul().to_string());
        }

        // 7. 신강/신약 태깅
        let strength = self.natal.strength();
        tags.push(format!("신강약:{}", strength.strength_type.hangul()));
        if strength.deuk_ryeong.acquired { tags.push("득령".to_string()); }
        if strength.deuk_ji.acquired { tags.push("득지".to_string()); }
        if strength.deuk_si.acquired { tags.push("득시".to_string()); }
        if strength.deuk_se.acquired { tags.push("득세".to_string()); }
        
        score.clamp(0.0, 100.0)
    }



    /// 인터럽트 벡터 테이블(IVT) 및 핸들링 로직
    fn handle_interrupts(
        &self,
        dynamic: &DynamicLuckAnalysis,
        tags: &mut Vec<String>,
        esil_trace: &mut String,
        registers: &mut QiRegisters,
        score: &mut f32,
    ) {
        // 현재 활성화된 신살 스캔 (원국 + 운의 상호작용)
        // 100년 전수 조사를 위해 가벼운 체크 로직 적용
        
        let yearly_ganzi = dynamic.yearly_influence.as_ref().map(|i| i.ganzi);
        
        if let Some(y) = yearly_ganzi {
            // 1. 고우선순위 인터럽트: 백호살 (Baihu)
            // 지지 충돌 시 백호대살이 겹치면 '심각한 하드웨어 예외' 발생
            if crate::analysis::spirit_markers::SpiritMarkerAnalysis::is_baihu(y) {
                let irq = SajuInterrupt::CriticalException;
                self.trigger_irq(irq, "백호대살", tags, esil_trace, registers, score);
            }

            // 2. 리소스 오버플로우: 괴강살 (Kuigang)
            if crate::analysis::spirit_markers::SpiritMarkerAnalysis::is_kuigang(y) {
                let irq = SajuInterrupt::ResourceOverflow;
                self.trigger_irq(irq, "괴강특수운", tags, esil_trace, registers, score);
            }
        }
    }

    /// 인터럽트 발생 (Handler Execution)
    fn trigger_irq(
        &self,
        irq: SajuInterrupt,
        marker_name: &str,
        tags: &mut Vec<String>,
        esil_trace: &mut String,
        registers: &mut QiRegisters,
        score: &mut f32,
    ) {
        let (penalty, tag) = match irq {
            SajuInterrupt::CriticalException => (25.0, format!("IRQ_0x01:{}", marker_name)),
            SajuInterrupt::ResourceOverflow => (-15.0, format!("IRQ_0x02:{}", marker_name)), // 괴강은 양면성, 일단 변동성으로 처리
            SajuInterrupt::SystemStall => (10.0, format!("IRQ_0x03:{}", marker_name)),
            SajuInterrupt::ServiceInterrupt => (7.0, format!("IRQ_0x04:{}", marker_name)),
        };

        *score -= penalty;
        esil_trace.push_str(&format!("irq_handle:{:?},impact:-{:.1}; ", irq, penalty));
        tags.push(tag);

        // 인터럽트 발생 시 특정 레지스터 강제 변조 (Kernel Panic 효과)
        if matches!(irq, SajuInterrupt::CriticalException) {
            registers.r2_earth += 20.0; // 토(Earth) 기운 급증으로 인한 시스템 정체
            esil_trace.push_str("kernel_panic:earth_overflow; ");
        }
    }

    /// 파이프라인 명령어 실행 루프
    fn execute_pipeline(
        &self,
        dynamic: &DynamicLuckAnalysis,
        tags: &mut Vec<String>,
        esil_trace: &mut String,
        registers: &mut QiRegisters,
        score: &mut f32,
    ) {
        let primary_yongshin = self.yongshin.primary;
        let assistant_yongshin = self.yongshin.assistant;
        let thermal_index = crate::analysis::yongshin::calculate_thermal_index(&self.natal);

        // 스테이지 정의
        let stages = [
            (dynamic.major_influence.as_ref().map(|i| i.ganzi), "대운", 10.0),
            (dynamic.yearly_influence.as_ref().map(|i| i.ganzi), "세운", 15.0),
            (dynamic.monthly_influence.as_ref().map(|i| i.ganzi), "월운", 5.0),
            (dynamic.daily_influence.as_ref().map(|i| i.ganzi), "일운", 2.0),
            (dynamic.hourly_influence.as_ref().map(|i| i.ganzi), "시운", 1.0),
        ];

        let mut prev_el: Option<Element> = None;
        let mut forwarding_buffer: Option<Element> = None;

        for (opt_ganzi, label, base_weight) in stages {
            if let Some(ganzi) = opt_ganzi {
                // 천간 합에 의한 변환된 오행 확인 (Forwarding 체크)
                let stem_label = format!("{}천간", label);
                let eff_el = self.get_effective_stem(dynamic, &stem_label).unwrap_or(ganzi.stem.element());

                let mut weight = self.get_element_priority(eff_el, primary_yongshin, assistant_yongshin, thermal_index);
                
                // 1. Data Hazard (Stall): 앞선 단계와 현재 단계가 상극일 경우
                if let Some(prev) = prev_el {
                    if prev.controls() == eff_el {
                        let stall_penalty = 0.5;
                        weight *= stall_penalty;
                        esil_trace.push_str(&format!("pipeline_stall:{}_by_{}; ", label, prev.hangul()));
                    }
                }

                // 2. Bypassing (Forwarding): 합에 의해 생성된 기운이 용신일 경우 보너스 가중치
                if let Some(fwd) = forwarding_buffer {
                    if fwd == eff_el || fwd.generates() == eff_el {
                        let fwd_bonus = 1.2;
                        weight *= fwd_bonus;
                        esil_trace.push_str(&format!("pipeline_forwarding:{}_aided_by_{}; ", label, fwd.hangul()));
                    }
                }

                let increment = base_weight * weight;
                *score += increment;
                registers.update(eff_el, increment);
                esil_trace.push_str(&format!("{}_infl:{},weight:{:.1},score+={:.1}; ", 
                    label.to_lowercase(), eff_el.hangul(), weight, increment));

                // 태깅
                if label == "대운" || label == "세운" {
                    if weight >= 1.5 { tags.push(format!("{}핵심운", label)); }
                    else if weight >= 1.0 { tags.push(format!("{}길운", label)); }
                    else if weight <= -0.5 { tags.push(format!("{}기신운", label)); }
                }

                // 다음 단계를 위해 현재 기운 저장
                prev_el = Some(eff_el);
                
                // 합화된 기운이 있다면 포워딩 버퍼에 저장
                if self.get_effective_stem(dynamic, &stem_label).is_some() {
                    forwarding_buffer = Some(eff_el);
                } else {
                    forwarding_buffer = None;
                }
            }
        }
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
        else if el == primary.controlled_by() { multiplier -= 0.8; }
        
        if thermal <= -30 && el == Element::Fire {
            multiplier *= 2.0;
            if multiplier == 0.0 { multiplier = 1.2; }
        }
        if thermal >= 30 && el == Element::Water {
            multiplier *= 2.0;
            if multiplier == 0.0 { multiplier = 1.2; }
        }

        multiplier
    }

    /// 경로 문자열로부터 실제 Branch 객체 추출
    fn get_branch_by_path(&self, path: &str, dynamic: &DynamicLuckAnalysis) -> Option<crate::core::branch::EarthlyBranch> {
        match path {
            "년지" => Some(self.natal.year.branch),
            "월지" => Some(self.natal.month.branch),
            "일지" => Some(self.natal.day.branch),
            "시지" => Some(self.natal.hour.branch),
            "대운지지" => dynamic.major_influence.as_ref().map(|i| i.ganzi.branch),
            "세운지지" => dynamic.yearly_influence.as_ref().map(|i| i.ganzi.branch),
            "월운지지" => dynamic.monthly_influence.as_ref().map(|i| i.ganzi.branch),
            "일운지지" => dynamic.daily_influence.as_ref().map(|i| i.ganzi.branch),
            "시운지지" => dynamic.hourly_influence.as_ref().map(|i| i.ganzi.branch),
            _ => None,
        }
    }
}
