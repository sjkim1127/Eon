//! Saju-VM: 인생 경로 에뮬레이션을 위한 가상 머신
//!
//! 사주 원국을 불변의 상태로 두고, 대운과 세운을 입력값(Instructions)으로 받아
//! 실시간 인생 에너지 상태를 시뮬레이션합니다.

use crate::analysis::dynamic_luck::DynamicLuckAnalysis;
use crate::analysis::major_luck::MajorLuckAnalysis;
use crate::analysis::periodic_luck::YearlyLuck;
use crate::analysis::shinsal::{EvilSpirit, Gilsin, TwelveShinsal};
use crate::analysis::void::VoidAnalysis;
use crate::analysis::yongshin::YongshinAnalysis;
use crate::core::element::Element;
use crate::core::ganzi::GanZi;
use crate::core::pillars::FourPillars;
use crate::core::ten_gods::TenGod;
use crate::core::twelve_stages::TwelveStage;
use crate::engine::signatures::{LuckSignature, SignatureScanner};
use serde::{Deserialize, Serialize};

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

    /// 기운의 감쇠 (시간 흐름에 따른 기전 상실 표현)
    pub fn apply_decay(&mut self, factor: f32) {
        self.r0_wood *= factor;
        self.r1_fire *= factor;
        self.r2_earth *= factor;
        self.r3_metal *= factor;
        self.r4_water *= factor;
    }

    /// 기운의 정규화 (전체 합을 100%로 유지하여 상대적 균형 분석)
    pub fn normalize(&mut self) {
        let total = self.r0_wood + self.r1_fire + self.r2_earth + self.r3_metal + self.r4_water;
        if total > 0.1 {
            self.r0_wood = (self.r0_wood / total) * 100.0;
            self.r1_fire = (self.r1_fire / total) * 100.0;
            self.r2_earth = (self.r2_earth / total) * 100.0;
            self.r3_metal = (self.r3_metal / total) * 100.0;
            self.r4_water = (self.r4_water / total) * 100.0;
        }
    }
}

/// 인생의 한 지점(1년 단위)의 시뮬레이션 결과
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifeFrame {
    /// 나이
    pub age: u32,
    pub ganzi: GanZi,                   // 그 해의 세운
    pub major_ganzi: GanZi,             // 당시의 대운
    pub score: f32,                     // 운세 점수 (0~100)
    pub tags: Vec<String>,              // 주요 특징 (용신운, 충, 합 등)
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
    /// 공망 분석 결과 (고정 기준)
    pub void: VoidAnalysis,
    /// 분석 설정
    pub config: crate::core::config::AnalysisConfig,
}

impl SajuVM {
    pub fn new(natal: FourPillars) -> Self {
        Self::new_with_config(natal, crate::core::config::AnalysisConfig::default())
    }

    pub fn new_with_config(
        natal: FourPillars,
        config: crate::core::config::AnalysisConfig,
    ) -> Self {
        let yongshin = natal.yongshin_with_config(&config);
        let void = natal.void_analysis();
        Self {
            natal,
            yongshin,
            void,
            config,
        }
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
        let mut score = self.config.vm.base_score;

        // 0. 하드웨어 인터럽트 체크 (Interrupt Vector Table Scan)
        self.handle_interrupts(dynamic, tags, esil_trace, registers, &mut score);

        // 1. 파이프라인 에뮬레이션 (Instruction Pipeline Execution)
        // 대운(Fetch) -> 세운(Decode) -> 월운(Execute) -> 일운(Memory) -> 시운(WriteBack)
        self.execute_pipeline(dynamic, tags, esil_trace, registers, &mut score);

        let primary_yongshin = self.yongshin.primary;
        let assistant_yongshin = self.yongshin.assistant;
        let thermal_index =
            crate::analysis::yongshin::calculate_thermal_index(&self.natal, &self.config);

        // 2. 공망(Void) 동적 감지 및 탈공(Escaping Void) 분석
        // 운에서 들어온 지지가 공망인지 확인
        for (luck_branch, label) in [
            (
                dynamic.major_influence.as_ref().map(|i| i.ganzi.branch),
                "대운",
            ),
            (
                dynamic.yearly_influence.as_ref().map(|i| i.ganzi.branch),
                "세운",
            ),
        ] {
            if let Some(b) = luck_branch {
                if self.void.void_branches.contains(&b) {
                    // 공망 발생! 기본적으로 흉(비효율, 헛됨)으로 간주
                    // 하지만 합(육합, 삼합, 방합)이나 충이 발생하면 '탈공(전실)'되어 공망이 해소됨

                    let mut is_escaped = false;
                    let _b_hangul = b.hangul();

                    // 탈공 조건 1: 지지 충
                    for (_, p1, p2) in &dynamic.combined_relations.branch_clashes {
                        if p1.contains(label) || p2.contains(label) {
                            if (p1.contains(label)
                                && self.get_branch_by_path(p1, dynamic) == Some(b))
                                || (p2.contains(label)
                                    && self.get_branch_by_path(p2, dynamic) == Some(b))
                            {
                                is_escaped = true;
                                tags.push(format!("탈공:충({})", label));
                                esil_trace.push_str(&format!(
                                    "void_escape:{}_clash,restore:10.0; ",
                                    label
                                ));
                                break;
                            }
                        }
                    }

                    // 탈공 조건 2: 합 (육합, 삼합, 방합)
                    if !is_escaped {
                        // 육합 체크
                        for (_, p1, p2) in &dynamic.combined_relations.six_combinations {
                            if p1.contains(label) || p2.contains(label) {
                                is_escaped = true;
                                tags.push(format!("탈공:육합({})", label));
                                break;
                            }
                        }
                        // 삼합/방합 체크 (이미 combined_relations에 포함됨)
                        if !is_escaped {
                            for trip in &dynamic.combined_relations.triple_combinations {
                                if trip.branches().contains(&b) {
                                    is_escaped = true;
                                    tags.push(format!("탈공:삼합({})", label));
                                    break;
                                }
                            }
                        }
                        if !is_escaped {
                            for season in &dynamic.combined_relations.seasonal_combinations {
                                if season.branches().contains(&b) {
                                    is_escaped = true;
                                    tags.push(format!("탈공:방합({})", label));
                                    break;
                                }
                            }
                        }
                    }

                    if is_escaped {
                        // 탈공됨 (전실): 전화위복
                        score += 5.0; // 오히려 좋아질 수 있음
                    } else {
                        // 진공(眞空): 공망의 흉의가 그대로 작용
                        let penalty = 10.0;
                        score -= penalty;
                        tags.push(format!("운성공망:{}", label));
                        esil_trace
                            .push_str(&format!("void_luck:{},penalty:-{:.1}; ", label, penalty));
                    }
                }
            }
        }

        // 2.5 신살(Shinsal) 동적 분석 (Phase 12)
        // 12신살, 길신(천을/문창), 흉살(원진/귀문) 체크

        let natal_day_branch = self.natal.day.branch;
        let natal_year_branch = self.natal.year.branch;
        let day_master = self.natal.day_master();

        for (luck_branch, label) in [
            (
                dynamic.major_influence.as_ref().map(|i| i.ganzi.branch),
                "대운",
            ),
            (
                dynamic.yearly_influence.as_ref().map(|i| i.ganzi.branch),
                "세운",
            ),
        ] {
            if let Some(b) = luck_branch {
                // A. 12신살 (일지 기준)
                let shinsal_day = TwelveShinsal::calculate(natal_day_branch, b);
                let (s_score, _s_tag) = match shinsal_day {
                    TwelveShinsal::Jangseongsal | TwelveShinsal::Banansal => (5.0, "길신"),
                    TwelveShinsal::Geopsal | TwelveShinsal::Jaesal | TwelveShinsal::Cheonsal => {
                        (-5.0, "흉신")
                    }
                    TwelveShinsal::Yeokmasal | TwelveShinsal::Jisal => (2.0, "이동"), // 역마/지살은 활동성 증가
                    _ => (0.0, "평"),
                };
                if s_score != 0.0 {
                    score += s_score;
                    tags.push(format!("신살:{}({})", shinsal_day.hangul(), label));
                    esil_trace.push_str(&format!(
                        "shinsal:{},score:{:.1}; ",
                        shinsal_day.hangul(),
                        s_score
                    ));
                }

                // B. 길신 (천을귀인, 문창귀인)
                let cheoneul = Gilsin::cheoneul_branches(day_master);
                if cheoneul.contains(&b) {
                    let bonus = 15.0;
                    score += bonus;
                    tags.push(format!("길신:천을귀인({})", label));
                    esil_trace.push_str(&format!("gilsin:cheoneul,bonus:{:.1}; ", bonus));
                }

                // C. 흉살 (원진, 귀문) - 일지/년지와 관계
                for (target, target_name) in
                    [(natal_day_branch, "일지"), (natal_year_branch, "년지")]
                {
                    if let Some(wonjin) = EvilSpirit::check_wonjin(target, b) {
                        score -= 5.0;
                        tags.push(format!(
                            "흉살:{}({}-{})",
                            wonjin.hangul(),
                            target_name,
                            label
                        ));
                    }
                    if let Some(gwimun) = EvilSpirit::check_gwimun(target, b) {
                        score -= 3.0;
                        tags.push(format!(
                            "흉살:{}({}-{})",
                            gwimun.hangul(),
                            target_name,
                            label
                        ));
                    }
                }
            }
        }

        // 2.6 동적 12운성 (Lifecycle Analysis) - Phase 13
        // 일간이 운에서 만나는 지지에 따라 에너지 상태 변화
        for (luck_branch, label) in [
            (
                dynamic.major_influence.as_ref().map(|i| i.ganzi.branch),
                "대운",
            ),
            (
                dynamic.yearly_influence.as_ref().map(|i| i.ganzi.branch),
                "세운",
            ),
        ] {
            if let Some(b) = luck_branch {
                let stage = crate::core::twelve_stages::calculate_twelve_stage(day_master, b);

                // 왕상휴수사(旺相休囚死)와 유사한 에너지 레벨
                let (stage_score, _description) = match stage {
                    TwelveStage::Changsheng
                    | TwelveStage::Guandai
                    | TwelveStage::Jianlu
                    | TwelveStage::Diwang => (10.0, "왕성"),
                    TwelveStage::Shuai | TwelveStage::Bing | TwelveStage::Si | TwelveStage::Mu => {
                        (-5.0, "쇠퇴")
                    }
                    TwelveStage::Jue | TwelveStage::Tai | TwelveStage::Yang => (-2.0, "불안/잉태"),
                    TwelveStage::Muyu => (-2.0, "목욕/불안"),
                };

                if stage_score != 0.0 {
                    score += stage_score;
                    esil_trace.push_str(&format!(
                        "lifecycle:{}({}),score:{:.1}; ",
                        stage.hangul(),
                        label,
                        stage_score
                    ));
                    tags.push(format!("운성:{}({})", stage.hangul(), label));
                }
            }
        }

        // 2.7 십신 복합 패턴 (Combinatorial Ten Gods) - Phase 13
        // 예: 상관견관(Hurting Officer + Direct Officer) -> 위화백단
        for (luck_stem, label) in [
            (
                dynamic.major_influence.as_ref().map(|i| i.ganzi.stem),
                "대운",
            ),
            (
                dynamic.yearly_influence.as_ref().map(|i| i.ganzi.stem),
                "세운",
            ),
        ] {
            if let Some(s) = luck_stem {
                let ten_god = TenGod::from_stems(day_master, s);

                // A. 상관견관 (상관 운 + 원국 정관)
                if ten_god == TenGod::Shangguan {
                    // 원국 천간에 정관이 있는지 확인
                    let has_zhengguan = [
                        self.natal.year.stem,
                        self.natal.month.stem,
                        self.natal.hour.stem,
                    ]
                    .iter()
                    .any(|&hz| TenGod::from_stems(day_master, hz) == TenGod::Zhengguan);

                    if has_zhengguan {
                        let penalty = 15.0;
                        score -= penalty;
                        tags.push(format!("패턴:상관견관({})", label));
                        esil_trace.push_str(&format!(
                            "pattern:shangguan_gyeongwan,penalty:-{:.1}; ",
                            penalty
                        ));
                    }
                }

                // B. 식신생재 (식신 운 + 원국 재성)
                if ten_god == TenGod::Shishen {
                    let has_wealth = [
                        self.natal.year.stem,
                        self.natal.month.stem,
                        self.natal.hour.stem,
                    ]
                    .iter()
                    .any(|&hz| {
                        let god = TenGod::from_stems(day_master, hz);
                        god == TenGod::Zhengcai || god == TenGod::Piancai
                    });

                    if has_wealth {
                        let bonus = 10.0;
                        score += bonus;
                        tags.push(format!("패턴:식신생재({})", label));
                        esil_trace
                            .push_str(&format!("pattern:shishen_saengjae,bonus:{:.1}; ", bonus));
                    }
                }
            }
        }

        // 3. 지지 합충 분석 (Structural Stability)
        for (clash, p1, p2) in &dynamic.combined_relations.branch_clashes {
            if p1.contains("세운")
                || p2.contains("세운")
                || p1.contains("대운")
                || p2.contains("대운")
            {
                let b1 = self.get_branch_by_path(p1, dynamic);
                let b2 = self.get_branch_by_path(p2, dynamic);

                if let (Some(b1_obj), Some(b2_obj)) = (b1, b2) {
                    let e1 = b1_obj.element();
                    let e2 = b2_obj.element();

                    let p1_priority = self.get_element_priority(
                        e1,
                        primary_yongshin,
                        assistant_yongshin,
                        thermal_index,
                    );
                    let p2_priority = self.get_element_priority(
                        e2,
                        primary_yongshin,
                        assistant_yongshin,
                        thermal_index,
                    );

                    // 정통 명리학: 희신(좋은 오행)을 충하면 흉, 기신(나쁜 오행)을 충하면 길
                    // 가중치 합산: 우선순위가 높을수록(용신) 충돌 시 감점이 큼
                    let total_impact = p1_priority + p2_priority;
                    let score_change = if total_impact > 1.0 {
                        // 희신/용신 충돌 (감점)
                        self.config.vm.clash_bad
                    } else if total_impact < -0.5 {
                        // 기신/구신 충돌 (가점 - 개고 효과 등)
                        self.config.vm.clash_good
                    } else {
                        // 일반적인 충돌 (약한 감점)
                        self.config.vm.clash_default
                    };

                    score += score_change;
                    esil_trace
                        .push_str(&format!("clash:{}-{},impact:{:.1}; ", p1, p2, score_change));
                }

                // 지장간 메모리 덤프 (Memory Dump of Hidden Stems)
                // 충돌이 발생한 지지의 지장간 기운을 해방하여 시스템에 반영
                let b1_obj = self.get_branch_by_path(p1, dynamic);
                let b2_obj = self.get_branch_by_path(p2, dynamic);

                for b in [b1_obj, b2_obj].iter().flatten() {
                    let hidden_stems = b.jijanggan();
                    for stem in hidden_stems {
                        let el = stem.element();
                        let weight = self.get_element_priority(
                            el,
                            primary_yongshin,
                            assistant_yongshin,
                            thermal_index,
                        );

                        // 지장간 기운은 겉으로 드러난 기운보다 영향력은 작지만(가중치 0.3)
                        // 충에 의해 해방될 때 시스템에 변수를 제공함
                        let bonus = 10.0 * self.config.vm.memory_dump_weight * weight;
                        score += bonus;
                        registers.update(el, bonus);

                        if weight.abs() > 0.1 {
                            esil_trace.push_str(&format!(
                                "mem_dump:{}({}),bonus:{:.1}; ",
                                b.hangul(),
                                stem.hangul(),
                                bonus
                            ));
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
            esil_trace.push_str(&format!(
                "mem_corrupt:{}-{},penalty:-{:.1}; ",
                p1, p2, penalty
            ));
            tags.push(pun.hangul());
        }

        // 5. 지지 해 (害) - Race Condition
        for (harm, p1, p2) in &dynamic.combined_relations.branch_harms {
            let penalty = 3.0;
            score -= penalty;
            esil_trace.push_str(&format!(
                "race_cond:{}-{},penalty:-{:.1}; ",
                p1, p2, penalty
            ));
            tags.push(harm.hangul().to_string());
        }

        // 6. 지지 파 (破) - IO Error
        for (dest, p1, p2) in &dynamic.combined_relations.branch_destructions {
            let penalty = 2.0;
            score -= penalty;
            esil_trace.push_str(&format!("io_error:{}-{},penalty:-{:.1}; ", p1, p2, penalty));
            tags.push(dest.hangul().to_string());
        }

        // 6.5 육합 (Six Combinations) - Stable Connection
        for (six, p1, p2) in &dynamic.combined_relations.six_combinations {
            // 원국 내의 육합은 이미 정적 분석에 포함됨, 여기서는 운과의 결합을 중시
            if p1.contains("운") || p2.contains("운") {
                let bonus = 8.0;
                score += bonus;

                // 합화 오행 기운 상승
                let element = six.transformed_element(); // 육합의 화기(예: 자축합토)
                if let Some(transformed_el) = element {
                    let weight = self.get_element_priority(
                        transformed_el,
                        primary_yongshin,
                        assistant_yongshin,
                        thermal_index,
                    );
                    score += weight * 3.0;
                    registers.update(transformed_el, weight * 3.0);
                }

                esil_trace.push_str(&format!("six_combo:{}-{},bonus:{:.1}; ", p1, p2, bonus));
                tags.push(format!("육합:{}", six.hangul()));
            }
        }

        // 6.6 천간충 (Stem Clashes) - Mental Stress
        for (clash, p1, p2) in &dynamic.combined_relations.stem_clashes {
            let penalty = 5.0;
            score -= penalty;
            esil_trace.push_str(&format!(
                "stem_clash:{}-{},penalty:-{:.1}; ",
                p1, p2, penalty
            ));
            tags.push(format!("천간충:{}", clash.hangul()));
        }

        // 7. 동적 회합(Triple/Seasonal) 완성 분석 - Dynamic Combination Completion
        // 원국에 없던 삼합/방합이 대운/세운에 의해 완성될 때 강력한 시너지 보너스 부여
        self.evaluate_dynamic_combinations(dynamic, tags, esil_trace, registers, &mut score);

        // 8. 신강/신약 태깅
        let strength = self.natal.strength();
        tags.push(format!("신강약:{}", strength.strength_type.hangul()));
        if strength.deuk_ryeong.acquired {
            tags.push("득령".to_string());
        }
        if strength.deuk_ji.acquired {
            tags.push("득지".to_string());
        }
        if strength.deuk_si.acquired {
            tags.push("득시".to_string());
        }
        if strength.deuk_se.acquired {
            tags.push("득세".to_string());
        }

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
            SajuInterrupt::CriticalException => (
                self.config.vm.irq_critical,
                format!("IRQ_0x00:{}(CRITICAL)", marker_name),
            ),
            SajuInterrupt::ResourceOverflow => (
                self.config.vm.irq_overflow,
                format!("IRQ_0x02:{}", marker_name),
            ),
            SajuInterrupt::SystemStall => (
                self.config.vm.irq_stall,
                format!("IRQ_0x03:{}", marker_name),
            ),
            SajuInterrupt::ServiceInterrupt => (
                self.config.vm.irq_service,
                format!("IRQ_0x04:{}", marker_name),
            ),
        };

        *score -= penalty;
        esil_trace.push_str(&format!("irq_handle:{:?},impact:-{:.1}; ", irq, penalty));
        tags.push(tag);

        // 인터럽트 발생 시 특정 레지스터 강제 변조 (Kernel Panic 효과)
        if matches!(irq, SajuInterrupt::CriticalException) {
            registers.r2_earth += self.config.vm.irq_critical; // 토(Earth) 기운 급증으로 인한 시스템 정체
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
        let thermal_index =
            crate::analysis::yongshin::calculate_thermal_index(&self.natal, &self.config);

        // 스테이지 정의
        let stages = [
            (
                dynamic.major_influence.as_ref().map(|i| i.ganzi),
                "대운",
                self.config.vm.pipeline_major,
            ),
            (
                dynamic.yearly_influence.as_ref().map(|i| i.ganzi),
                "세운",
                self.config.vm.pipeline_yearly,
            ),
            (
                dynamic.monthly_influence.as_ref().map(|i| i.ganzi),
                "월운",
                self.config.vm.pipeline_monthly,
            ),
            (
                dynamic.daily_influence.as_ref().map(|i| i.ganzi),
                "일운",
                self.config.vm.pipeline_daily,
            ),
            (
                dynamic.hourly_influence.as_ref().map(|i| i.ganzi),
                "시운",
                self.config.vm.pipeline_hourly,
            ),
        ];

        let mut prev_el: Option<Element> = None;
        let mut forwarding_buffer: Option<Element> = None;

        for (opt_ganzi, label, base_weight) in stages {
            if let Some(ganzi) = opt_ganzi {
                // 천간 합에 의한 변환된 오행 확인 (Forwarding 체크)
                let stem_label = format!("{}천간", label);
                let eff_el = self
                    .get_effective_stem(dynamic, &stem_label)
                    .unwrap_or(ganzi.stem.element());

                let mut weight = self.get_element_priority(
                    eff_el,
                    primary_yongshin,
                    assistant_yongshin,
                    thermal_index,
                );

                // 1. Data Hazard (Stall): 앞선 단계와 현재 단계가 상극일 경우
                if let Some(prev) = prev_el {
                    if prev.controls() == eff_el {
                        let stall_penalty = self.config.vm.stall_penalty;
                        weight *= stall_penalty;
                        esil_trace.push_str(&format!(
                            "pipeline_stall:{}_by_{}; ",
                            label,
                            prev.hangul()
                        ));
                    }
                }

                // 2. Bypassing (Forwarding): 합에 의해 생성된 기운이 용신일 경우 보너스 가중치
                if let Some(fwd) = forwarding_buffer {
                    if fwd == eff_el || fwd.generates() == eff_el {
                        let fwd_bonus = self.config.vm.forwarding_bonus;
                        weight *= fwd_bonus;
                        esil_trace.push_str(&format!(
                            "pipeline_forwarding:{}_aided_by_{}; ",
                            label,
                            fwd.hangul()
                        ));
                    }
                }

                let increment = base_weight * weight;
                *score += increment;

                // VM 레지스터 업데이트 전 감쇠 및 후 정규화
                registers.apply_decay(self.config.vm.decay_factor);
                registers.update(eff_el, increment);
                registers.normalize();

                esil_trace.push_str(&format!(
                    "{}_infl:{},weight:{:.1},score+={:.1}; ",
                    label.to_lowercase(),
                    eff_el.hangul(),
                    weight,
                    increment
                ));

                // 태깅
                if label == "대운" || label == "세운" {
                    if weight >= 1.5 {
                        tags.push(format!("{}핵심운", label));
                    } else if weight >= 1.0 {
                        tags.push(format!("{}길운", label));
                    } else if weight <= -0.5 {
                        tags.push(format!("{}기신운", label));
                    }
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
    fn get_element_priority(
        &self,
        el: Element,
        primary: Element,
        assistant: Element,
        thermal: i32,
    ) -> f32 {
        let mut multiplier = 0.0;

        if el == primary {
            multiplier += 1.0;
        } else if el == assistant {
            multiplier += 0.5;
        } else if el == primary.controlled_by() {
            multiplier -= 0.8;
        }

        if thermal <= -30 && el == Element::Fire {
            multiplier *= 2.0;
            if multiplier == 0.0 {
                multiplier = 1.2;
            }
        }
        if thermal >= 30 && el == Element::Water {
            multiplier *= 2.0;
            if multiplier == 0.0 {
                multiplier = 1.2;
            }
        }

        multiplier
    }

    /// 경로 문자열로부터 실제 Branch 객체 추출
    fn get_branch_by_path(
        &self,
        path: &str,
        dynamic: &DynamicLuckAnalysis,
    ) -> Option<crate::core::branch::EarthlyBranch> {
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

    /// 동적 회합(삼합/방합) 완성 평가
    fn evaluate_dynamic_combinations(
        &self,
        dynamic: &DynamicLuckAnalysis,
        tags: &mut Vec<String>,
        esil_trace: &mut String,
        registers: &mut QiRegisters,
        score: &mut f32,
    ) {
        let natal_triples = &dynamic.natal_relations.triple_combinations;
        let natal_seasonals = &dynamic.natal_relations.seasonal_combinations;

        // 1. 삼합 완성 체크
        for triple in &dynamic.combined_relations.triple_combinations {
            if !natal_triples.contains(triple) {
                // 원국에는 없던 삼합이 완성됨!
                let element = triple.element();
                let priority = self.get_element_priority(
                    element,
                    self.yongshin.primary,
                    self.yongshin.assistant,
                    0,
                );

                let bonus = 15.0 * priority;
                *score += bonus;
                registers.update(element, bonus.abs());

                let label = if priority > 0.0 {
                    "삼합완성(吉)"
                } else {
                    "삼합완성(凶)"
                };
                tags.push(label.to_string());
                esil_trace.push_str(&format!(
                    "dynamic_triple:{},bonus:{:.1}; ",
                    triple.hangul(),
                    bonus
                ));
            }
        }

        // 2. 방합 완성 체크
        for seasonal in &dynamic.combined_relations.seasonal_combinations {
            if !natal_seasonals.contains(seasonal) {
                let element = seasonal.element();
                let priority = self.get_element_priority(
                    element,
                    self.yongshin.primary,
                    self.yongshin.assistant,
                    0,
                );

                let bonus = 20.0 * priority; // 방합은 세력이 더 강력함
                *score += bonus;
                registers.update(element, bonus.abs());

                let label = if priority > 0.0 {
                    "방합완성(吉)"
                } else {
                    "방합완성(凶)"
                };
                tags.push(label.to_string());
                esil_trace.push_str(&format!(
                    "dynamic_seasonal:{},bonus:{:.1}; ",
                    seasonal.hangul(),
                    bonus
                ));
            }
        }
    }

    /// 인생 시뮬레이션 (Rayon 병렬 처리 지원)
    pub fn simulate_life(&self, start_age: u32, end_age: u32) -> Vec<LifeFrame> {
        use rayon::prelude::*;

        // 1. 대운 흐름 계산 (교운기 포함 정밀 분석)
        let luck_analysis = MajorLuckAnalysis::calculate_astro(
            &self.natal,
            self.natal.gender,
            self.natal.raw_input.year,
            self.natal.raw_input.month,
            self.natal.raw_input.day,
            self.natal.raw_input.hour,
            self.natal.raw_input.minute,
        )
        .ok();

        // 출생 연도 (세운 계산 기준)
        let birth_year = self.natal.raw_input.year;

        (start_age..=end_age)
            .into_par_iter()
            .map(|age| {
                // 2. 해당 나이의 대운 간지 추출
                let major_ganzi = luck_analysis
                    .as_ref()
                    .and_then(|a| a.at_age(age))
                    .map(|m| m.ganzi)
                    .unwrap_or_else(|| GanZi::from_index(0));

                // 3. 해당 나이의 세운 간지 추출 (한국식 나이 또는 만 나이 기준 보정 필요)
                // 여기서는 단순하게 생년 + (나이-1) 전후의 세운을 계산
                let target_year = birth_year + (age as i32) - 1; // 대략적인 나이 보정
                let yearly_ganzi = YearlyLuck::calculate(target_year, &self.natal).ganzi;

                // 4. 정밀한 step 수행
                self.step(age, major_ganzi, yearly_ganzi, None, None, None)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::pillars::SajuInput;

    fn test_vm() -> SajuVM {
        // 갑자년 갑자월 갑자일 갑자시 (가장 심플한 케이스)
        let input = SajuInput::new_solar(1984, 1, 1, 0, 0);
        // Note: 1984 is GapJa year
        let pillars = FourPillars::calculate(&input).unwrap();
        SajuVM::new(pillars)
    }

    #[test]
    fn test_pipeline_stall() {
        let vm = test_vm();
        // This requires detailed setup of DynamicLuckAnalysis which is complex.
        // For now, we will verify that the constants are correctly loaded.
        assert_eq!(vm.config.vm.stall_penalty, 0.5);
    }
}
