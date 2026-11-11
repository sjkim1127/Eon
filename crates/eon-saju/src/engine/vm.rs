//! Saju-VM: Virtual machine for life path trajectory emulation
//!
//! Treats natal Four Pillars as an immutable base state, consuming luck cycles
//! (Major, Yearly, etc.) as instructions to simulate real-time energy flow.

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
use crate::engine::trace_tag::{
    AuspiciousSpiritName, InauspiciousSpiritName, InterruptType, LuckPeriod, PillarPosition,
    StrengthTypeName, TraceTag,
};
use serde::{Deserialize, Serialize};

/// Saju Interrupt (hardware exception/interrupt)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SajuInterrupt {
    /// 0x01: High-priority system clash (Baekho, etc.)
    CriticalException,
    /// 0x02: Resource overflow (Goegang, etc.)
    ResourceOverflow,
    /// 0x03: System stall / isolation (Gosin/Gwasuk, etc.)
    SystemStall,
    /// 0x04: Temporary service interruption (Mangshin/Geop, etc.)
    ServiceInterrupt,
}

/// Five Elements Registers (R0~R4)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QiRegisters {
    /// R0: Wood
    pub r0_wood: f32,
    /// R1: Fire
    pub r1_fire: f32,
    /// R2: Earth
    pub r2_earth: f32,
    /// R3: Metal
    pub r3_metal: f32,
    /// R4: Water
    pub r4_water: f32,
}

impl Default for QiRegisters {
    fn default() -> Self {
        Self::new()
    }
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

    /// Energy decay over time
    pub fn apply_decay(&mut self, factor: f32) {
        self.r0_wood *= factor;
        self.r1_fire *= factor;
        self.r2_earth *= factor;
        self.r3_metal *= factor;
        self.r4_water *= factor;
    }

    /// Energy normalization (maintains total sum at 100% for relative balance)
    pub fn normalize(&mut self) {
        self.r0_wood = self.r0_wood.max(0.0);
        self.r1_fire = self.r1_fire.max(0.0);
        self.r2_earth = self.r2_earth.max(0.0);
        self.r3_metal = self.r3_metal.max(0.0);
        self.r4_water = self.r4_water.max(0.0);

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

/// Simulation result for a single life frame (1-year interval)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LifeFrame {
    /// Age
    pub age: u32,
    pub ganzi: GanZi,       // Yearly luck stem-branch
    pub major_ganzi: GanZi, // Major luck stem-branch
    pub score: f32,         // Luck score (0~100)
    /// Key features (yongshin luck, clash, combination, etc.)
    pub tags: Vec<TraceTag>,
    pub signatures: Vec<LuckSignature>, // Detected luck patterns
    /// ESIL trace (instruction execution log)
    pub esil_trace: String,
    /// Register state after event resolution
    pub register_state: QiRegisters,
}

impl LifeFrame {
    /// Convert tags to string vector (legacy compatibility)
    pub fn tags_as_strings(&self) -> Vec<String> {
        crate::engine::trace_tag::tags_to_strings(&self.tags)
    }
}

/// Saju Virtual Machine
#[derive(Debug, Clone)]
pub struct SajuVM {
    /// Natal chart (fixed context)
    pub natal: FourPillars,
    /// Yongshin analysis outcome (fixed reference)
    pub yongshin: YongshinAnalysis,
    /// Void analysis outcome (fixed reference)
    pub void: VoidAnalysis,
    /// Engine configuration
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

        // Signature scan
        frame.signatures = SignatureScanner::scan(&frame, &dynamic);

        frame
    }

    /// Scoring evaluation logic (Precision Tuning)
    /// - Tracks interaction between natal state and luck cycles akin to Data Flow Analysis
    fn evaluate_score(
        &self,
        dynamic: &DynamicLuckAnalysis,
        tags: &mut Vec<TraceTag>,
        esil_trace: &mut String,
        registers: &mut QiRegisters,
    ) -> f32 {
        let mut score = self.config.vm.base_score;

        let power_res = self
            .natal
            .integrated_analysis(Default::default(), &self.config);
        for (el, pct, _) in power_res.element_scores {
            registers.update(el, pct);
        }

        // 0. Check hardware interrupts (Interrupt Vector Table scan)
        self.handle_interrupts(dynamic, tags, esil_trace, registers, &mut score);

        // 1. Pipeline execution emulation:
        // Major(Fetch) -> Yearly(Decode) -> Monthly(Execute) -> Daily(Memory) -> Hourly(WriteBack)
        self.execute_pipeline(dynamic, tags, esil_trace, registers, &mut score);

        let primary_yongshin = self.yongshin.primary;
        let assistant_yongshin = self.yongshin.assistant;
        let thermal_index =
            crate::analysis::yongshin::calculate_thermal_index(&self.natal, &self.config);

        // 2. Dynamic void detection and escaping void analysis
        for (luck_branch, label) in [
            (
                dynamic.major_influence.as_ref().map(|i| i.ganzi.branch),
                "Major",
            ),
            (
                dynamic.yearly_influence.as_ref().map(|i| i.ganzi.branch),
                "Yearly",
            ),
        ] {
            if let Some(b) = luck_branch {
                if self.void.void_branches.contains(&b) {
                    // Void occurrence: fundamentally considered inauspicious unless rescued by clash or combination

                    let mut is_escaped = false;
                    let _b_hangul = b.hangul();

                    // Rescued void condition 1: Branch clash
                    for (_, p1, p2) in &dynamic.combined_relations.branch_clashes {
                        if (p1.contains(&label.to_lowercase())
                            || p2.contains(&label.to_lowercase()))
                            && ((p1.contains(&label.to_lowercase())
                                && self.get_branch_by_path(p1, dynamic) == Some(b))
                                || (p2.contains(&label.to_lowercase())
                                    && self.get_branch_by_path(p2, dynamic) == Some(b)))
                        {
                            is_escaped = true;
                            let period = LuckPeriod::from_label(label);
                            tags.push(TraceTag::EscapedVoidClash { period });
                            esil_trace
                                .push_str(&format!("void_escape:{}_clash,restore:10.0; ", label));
                            break;
                        }
                    }

                    // Rescued void condition 2: Combinations (Six, Triple, Seasonal)
                    if !is_escaped {
                        // Check Six Combination
                        for (_, p1, p2) in &dynamic.combined_relations.six_combinations {
                            if p1.contains(&label.to_lowercase())
                                || p2.contains(&label.to_lowercase())
                            {
                                is_escaped = true;
                                let period = LuckPeriod::from_label(label);
                                tags.push(TraceTag::EscapedVoidSixCombo { period });
                                break;
                            }
                        }
                        if !is_escaped {
                            for (_, p1, p2) in &dynamic.combined_relations.branch_clashes {
                                if p1.contains(&label.to_lowercase())
                                    || p2.contains(&label.to_lowercase())
                                {
                                    is_escaped = true;
                                    let period = LuckPeriod::from_label(label);
                                    tags.push(TraceTag::EscapedVoidClash { period });
                                    break;
                                }
                            }
                        }
                        // Check Triple / Seasonal Alliances
                        if !is_escaped {
                            for trip in &dynamic.combined_relations.triple_combinations {
                                if trip.branches().contains(&b) {
                                    is_escaped = true;
                                    let period = LuckPeriod::from_label(label);
                                    tags.push(TraceTag::EscapedVoidTriple { period });
                                    break;
                                }
                            }
                        }
                        if !is_escaped {
                            for season in &dynamic.combined_relations.seasonal_combinations {
                                if season.branches().contains(&b) {
                                    is_escaped = true;
                                    let period = LuckPeriod::from_label(label);
                                    tags.push(TraceTag::EscapedVoidSeasonal { period });
                                    break;
                                }
                            }
                        }
                    }

                    if is_escaped {
                        // Rescued void: turned into opportunity
                        score += 5.0;
                    } else {
                        // True void: unrescued void penalty
                        let penalty = 10.0;
                        score -= penalty;
                        tags.push(TraceTag::VoidLuck {
                            period: LuckPeriod::from_label(label),
                        });
                        esil_trace
                            .push_str(&format!("void_luck:{},penalty:-{:.1}; ", label, penalty));
                    }
                }
            }
        }

        // 2.5 Dynamic Shinsal analysis: 12 Shinsal, Auspicious spirits, Inauspicious spirits

        let natal_day_branch = self.natal.day.branch;
        let natal_year_branch = self.natal.year.branch;
        let day_master = self.natal.day_master();

        for (luck_branch, label) in [
            (
                dynamic.major_influence.as_ref().map(|i| i.ganzi.branch),
                "Major",
            ),
            (
                dynamic.yearly_influence.as_ref().map(|i| i.ganzi.branch),
                "Yearly",
            ),
        ] {
            if let Some(b) = luck_branch {
                // A. 12 Shinsal (day branch reference)
                let shinsal_day = TwelveShinsal::calculate(natal_day_branch, b);
                let (s_score, _s_tag) = match shinsal_day {
                    TwelveShinsal::Jangseongsal | TwelveShinsal::Banansal => (5.0, "Auspicious"),
                    TwelveShinsal::Geopsal | TwelveShinsal::Jaesal | TwelveShinsal::Cheonsal => {
                        (-5.0, "Inauspicious")
                    }
                    TwelveShinsal::Yeokmasal | TwelveShinsal::Jisal => (2.0, "Movement"),
                    _ => (0.0, "Neutral"),
                };
                if s_score != 0.0 {
                    score += s_score;
                    tags.push(TraceTag::TwelveShinsal {
                        name: shinsal_day.into(),
                        period: LuckPeriod::from_label(label),
                    });
                    esil_trace.push_str(&format!(
                        "shinsal:{},score:{:.1}; ",
                        shinsal_day.hangul(),
                        s_score
                    ));
                }

                // B. Auspicious spirits (Cheoneul, Munchang)
                let cheoneul = Gilsin::cheoneul_branches(day_master);
                if cheoneul.contains(&b) {
                    let bonus = 15.0;
                    score += bonus;
                    tags.push(TraceTag::AuspiciousSpirit {
                        name: AuspiciousSpiritName::Cheoneul,
                        period: LuckPeriod::from_label(label),
                    });
                    esil_trace.push_str(&format!("gilsin:cheoneul,bonus:{:.1}; ", bonus));
                }

                // C. Inauspicious spirits (Wonjin, Gwimun)
                for (target, target_name) in [
                    (natal_day_branch, "day_branch"),
                    (natal_year_branch, "year_branch"),
                ] {
                    if let Some(_wonjin) = EvilSpirit::check_wonjin(target, b) {
                        score -= 5.0;
                        tags.push(TraceTag::InauspiciousSpirit {
                            name: InauspiciousSpiritName::Wonjin,
                            position: PillarPosition::from_label(target_name),
                            period: LuckPeriod::from_label(label),
                        });
                    }
                    if let Some(_gwimun) = EvilSpirit::check_gwimun(target, b) {
                        score -= 3.0;
                        tags.push(TraceTag::InauspiciousSpirit {
                            name: InauspiciousSpiritName::Gwimun,
                            position: PillarPosition::from_label(target_name),
                            period: LuckPeriod::from_label(label),
                        });
                    }
                }
            }
        }

        // 2.6 Dynamic 12 Life Stages analysis
        for (luck_branch, label) in [
            (
                dynamic.major_influence.as_ref().map(|i| i.ganzi.branch),
                "Major",
            ),
            (
                dynamic.yearly_influence.as_ref().map(|i| i.ganzi.branch),
                "Yearly",
            ),
        ] {
            if let Some(b) = luck_branch {
                let stage = crate::core::twelve_stages::calculate_twelve_stage(day_master, b);

                // Energy levels analogous to the Five Seasonal Qi States
                let (stage_score, _description) = match stage {
                    TwelveStage::Changsheng
                    | TwelveStage::Guandai
                    | TwelveStage::Jianlu
                    | TwelveStage::Diwang => (10.0, "Prosperous"),
                    TwelveStage::Shuai | TwelveStage::Bing | TwelveStage::Si | TwelveStage::Mu => {
                        (-5.0, "Declining")
                    }
                    TwelveStage::Jue | TwelveStage::Tai | TwelveStage::Yang => (-2.0, "Unstable"),
                    TwelveStage::Muyu => (-2.0, "Cleansing/Volatile"),
                };

                if stage_score != 0.0 {
                    score += stage_score;
                    esil_trace.push_str(&format!(
                        "lifecycle:{}({}),score:{:.1}; ",
                        stage.hangul(),
                        label,
                        stage_score
                    ));
                    tags.push(TraceTag::LifeStage {
                        stage: stage.into(),
                        period: LuckPeriod::from_label(label),
                    });
                }
            }
        }

        // 2.7 Combinatorial Ten Gods patterns
        for (luck_stem, label) in [
            (
                dynamic.major_influence.as_ref().map(|i| i.ganzi.stem),
                "Major",
            ),
            (
                dynamic.yearly_influence.as_ref().map(|i| i.ganzi.stem),
                "Yearly",
            ),
        ] {
            if let Some(s) = luck_stem {
                let ten_god = TenGod::from_stems(day_master, s);

                // A. Hurting Officer meets Direct Officer
                if ten_god == TenGod::Shangguan {
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
                        tags.push(TraceTag::HurtingOfficerMeetsOfficer {
                            period: LuckPeriod::from_label(label),
                        });
                        esil_trace.push_str(&format!(
                            "pattern:shangguan_gyeongwan,penalty:-{:.1}; ",
                            penalty
                        ));
                    }
                }

                // B. Eating God produces Wealth
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
                        tags.push(TraceTag::EatingGodProducesWealth {
                            period: LuckPeriod::from_label(label),
                        });
                        esil_trace
                            .push_str(&format!("pattern:shishen_saengjae,bonus:{:.1}; ", bonus));
                    }
                }
            }
        }

        // 3. Branch Clash Analysis (Structural Stability)
        for (clash, p1, p2) in &dynamic.combined_relations.branch_clashes {
            if p1.contains("luck") || p2.contains("luck") {
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

                    // Clash evaluation: clashing auspicious elements is unfavorable, clashing inauspicious is favorable
                    let total_impact = p1_priority + p2_priority;
                    let score_change = if total_impact > 1.0 {
                        // Clashing favorable elements
                        self.config.vm.clash_bad
                    } else if total_impact < -0.5 {
                        // Clashing unfavorable elements
                        self.config.vm.clash_good
                    } else {
                        // Neutral clash
                        self.config.vm.clash_default
                    };

                    score += score_change;
                    esil_trace
                        .push_str(&format!("clash:{}-{},impact:{:.1}; ", p1, p2, score_change));
                }

                // Memory dump of hidden stems: releasing unsealed qi into the system
                let b1_obj = self.get_branch_by_path(p1, dynamic);
                let b2_obj = self.get_branch_by_path(p2, dynamic);

                for b in [b1_obj, b2_obj].iter().flatten() {
                    let hidden_stems = b.jijanggan();
                    for stem in hidden_stems {
                        // Skip stems that will be scored by GaeGo events in Section 6.7 to prevent double scoring
                        let is_gaego_unsealed = dynamic
                            .gaego_events
                            .iter()
                            .any(|ev| ev.branch == *b && ev.unsealed_stems.contains(&stem));
                        if is_gaego_unsealed {
                            continue;
                        }

                        let el = stem.element();
                        let weight = self.get_element_priority(
                            el,
                            primary_yongshin,
                            assistant_yongshin,
                            thermal_index,
                        );

                        // Hidden stem qi released upon clash
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
                tags.push(TraceTag::BranchClash {
                    clash_type: clash.hangul().into(),
                });
            }
        }

        // 4. Branch Punishment - Memory Corruption
        for (pun, p1, p2) in &dynamic.combined_relations.branch_punishments {
            let penalty = 5.0;
            score -= penalty;
            esil_trace.push_str(&format!(
                "mem_corrupt:{}-{},penalty:-{:.1}; ",
                p1, p2, penalty
            ));
            tags.push(TraceTag::Punishment {
                punishment_type: pun.hangul().into(),
            });
        }

        // 5. Branch Harm - Race Condition
        for (harm, p1, p2) in &dynamic.combined_relations.branch_harms {
            let penalty = 3.0;
            score -= penalty;
            esil_trace.push_str(&format!(
                "race_cond:{}-{},penalty:-{:.1}; ",
                p1, p2, penalty
            ));
            tags.push(TraceTag::Harm {
                harm_type: harm.hangul().into(),
            });
        }

        // 6. Branch Destruction - IO Error
        for (dest, p1, p2) in &dynamic.combined_relations.branch_destructions {
            let penalty = 2.0;
            score -= penalty;
            esil_trace.push_str(&format!("io_error:{}-{},penalty:-{:.1}; ", p1, p2, penalty));
            tags.push(TraceTag::Destruction {
                destruction_type: dest.hangul().into(),
            });
        }

        // 6.5 Six Combinations - Stable Connection
        for (six, p1, p2) in &dynamic.combined_relations.six_combinations {
            // Evaluates six combinations involving luck pillars
            if p1.contains("luck") || p2.contains("luck") {
                let bonus = 8.0;
                score += bonus;

                // Boost transformed element energy
                let element = six.transformed_element();
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
                tags.push(TraceTag::SixCombination {
                    combo_type: six.hangul().into(),
                });
            }
        }

        // 6.6 Stem Clashes - Mental Stress
        for (clash, p1, p2) in &dynamic.combined_relations.stem_clashes {
            let penalty = 5.0;
            score -= penalty;
            esil_trace.push_str(&format!(
                "stem_clash:{}-{},penalty:-{:.1}; ",
                p1, p2, penalty
            ));
            tags.push(TraceTag::StemClash {
                clash_type: clash.hangul().into(),
            });
        }

        // 6.7 Storage Unsealing (GaeGo)
        for event in &dynamic.gaego_events {
            for stem in &event.unsealed_stems {
                let el = stem.element();
                let weight = self.get_element_priority(
                    el,
                    primary_yongshin,
                    assistant_yongshin,
                    thermal_index,
                );
                let bonus = 10.0 * self.config.vm.memory_dump_weight * weight;
                score += bonus;
                registers.update(el, bonus);
                esil_trace.push_str(&format!(
                    "gaego:{}({}),bonus:{:.1}; ",
                    event.branch.hangul(),
                    stem.hangul(),
                    bonus
                ));
                tags.push(TraceTag::GaeGo {
                    branch: event.branch,
                    unsealed_stem: *stem,
                });
            }
        }

        // 6.8 Storage Trapping (IpMyo)
        for event in &dynamic.ipmyo_events {
            let el = event.element;
            let penalty = 8.0;
            score -= penalty;
            registers.update(el, -penalty);
            esil_trace.push_str(&format!(
                "ipmyo:{},tomb:{},penalty:-{:.1}; ",
                el.hangul(),
                event.tomb_branch.hangul(),
                penalty
            ));
            tags.push(TraceTag::IpMyo {
                element: el,
                tomb_branch: event.tomb_branch,
            });
        }

        // 6.9 Dynamic Structure State Transition
        let gyeok_state = &dynamic.structure_state;
        let gyeok_score = match gyeok_state.status {
            crate::analysis::dynamic_luck::GyeokStatus::Fulfilled => 15.0,
            crate::analysis::dynamic_luck::GyeokStatus::Transformed => 8.0,
            crate::analysis::dynamic_luck::GyeokStatus::Broken => -15.0,
            crate::analysis::dynamic_luck::GyeokStatus::Stable => 0.0,
        };
        if gyeok_score != 0.0 {
            score += gyeok_score;
            esil_trace.push_str(&format!(
                "gyeok_state:{},status:{},score:{:.1}; ",
                gyeok_state.active_structure.hangul(),
                gyeok_state.status.hangul(),
                gyeok_score
            ));
        }
        tags.push(TraceTag::DynamicGyeok {
            active_structure: gyeok_state.active_structure,
            status: gyeok_state.status,
        });

        // 7. Dynamic Combination Completion (Triple / Seasonal Alliances)
        self.evaluate_dynamic_combinations(dynamic, tags, esil_trace, registers, &mut score);

        // 8. Strength / Weakness tagging
        let strength = self.natal.strength();
        let strength_name = match strength.strength_type {
            crate::analysis::strength::StrengthType::Strong => StrengthTypeName::Strong,
            crate::analysis::strength::StrengthType::Weak => StrengthTypeName::Weak,
            crate::analysis::strength::StrengthType::Balanced => StrengthTypeName::Balanced,
        };
        tags.push(TraceTag::StrengthType {
            strength: strength_name,
        });
        if strength.deuk_ryeong.acquired {
            tags.push(TraceTag::DeukRyeong);
        }
        if strength.deuk_ji.acquired {
            tags.push(TraceTag::DeukJi);
        }
        if strength.deuk_si.acquired {
            tags.push(TraceTag::DeukSi);
        }
        if strength.deuk_se.acquired {
            tags.push(TraceTag::DeukSe);
        }

        self.apply_dynamic_transformation(registers, esil_trace);

        let mut final_score = score;
        if final_score > 100.0 {
            final_score = 100.0;
        } else if final_score < 15.0 {
            // Asymptotic decay below 15.0 to prevent hard 0.0 clamp
            final_score = 15.0 * ((final_score - 15.0) / 15.0).exp();
        }

        final_score
    }

    /// Dynamic element energy conversion (Conservation of Energy)
    fn apply_dynamic_transformation(&self, registers: &mut QiRegisters, esil_trace: &mut String) {
        // Generating cycle simulation: Wood -> Fire -> Earth -> Metal -> Water -> Wood
        // Energy drained from parent increases child element (energy conservation)
        let transfer_rate = 0.1; // 10% energy transfer

        let wood_transfer = registers.r0_wood * transfer_rate;
        let fire_transfer = registers.r1_fire * transfer_rate;
        let earth_transfer = registers.r2_earth * transfer_rate;
        let metal_transfer = registers.r3_metal * transfer_rate;
        let water_transfer = registers.r4_water * transfer_rate;

        // Wood -> Fire
        registers.r0_wood -= wood_transfer;
        registers.r1_fire += wood_transfer;

        // Fire -> Earth
        registers.r1_fire -= fire_transfer;
        registers.r2_earth += fire_transfer;

        // Earth -> Metal
        registers.r2_earth -= earth_transfer;
        registers.r3_metal += earth_transfer;

        // Metal -> Water
        registers.r3_metal -= metal_transfer;
        registers.r4_water += metal_transfer;

        // Water -> Wood
        registers.r4_water -= water_transfer;
        registers.r0_wood += water_transfer;

        registers.normalize();
        esil_trace.push_str("qi_conservation:applied; ");
    }

    /// Interrupt Vector Table (IVT) and handling logic
    fn handle_interrupts(
        &self,
        dynamic: &DynamicLuckAnalysis,
        tags: &mut Vec<TraceTag>,
        esil_trace: &mut String,
        registers: &mut QiRegisters,
        score: &mut f32,
    ) {
        // Scan active spirits (natal + luck interaction)

        let yearly_ganzi = dynamic.yearly_influence.as_ref().map(|i| i.ganzi);

        if let Some(y) = yearly_ganzi {
            // 1. High-priority interrupt: Baekho (Baihu)
            let el = y.stem.element();
            let thermal_index =
                crate::analysis::yongshin::calculate_thermal_index(&self.natal, &self.config);
            let p = self.get_element_priority(
                el,
                self.yongshin.primary,
                self.yongshin.assistant,
                thermal_index,
            );

            if crate::analysis::spirit_markers::SpiritMarkerAnalysis::is_baihu(y) {
                let mut irq = SajuInterrupt::CriticalException;
                if p > 0.5 {
                    irq = SajuInterrupt::ServiceInterrupt; // Yongshin mitigation
                    esil_trace.push_str("irq_mitigate:yongshin_protect; ");
                }
                self.trigger_irq(irq, "BaekhoDaeSal", tags, esil_trace, registers, score);
            }

            // 2. Resource overflow: Goegang (Kuigang)
            if crate::analysis::spirit_markers::SpiritMarkerAnalysis::is_kuigang(y) {
                let mut irq = SajuInterrupt::ResourceOverflow;
                if p > 0.5 {
                    irq = SajuInterrupt::ServiceInterrupt; // Yongshin mitigation
                    esil_trace.push_str("irq_mitigate:yongshin_protect; ");
                }
                self.trigger_irq(
                    irq,
                    "GoegangSpecialLuck",
                    tags,
                    esil_trace,
                    registers,
                    score,
                );
            }
        }
    }

    /// Interrupt execution handler
    fn trigger_irq(
        &self,
        irq: SajuInterrupt,
        marker_name: &'static str,
        tags: &mut Vec<TraceTag>,
        esil_trace: &mut String,
        registers: &mut QiRegisters,
        score: &mut f32,
    ) {
        let (penalty, irq_type) = match irq {
            SajuInterrupt::CriticalException => (
                self.config.vm.irq_critical,
                InterruptType::CriticalException,
            ),
            SajuInterrupt::ResourceOverflow => {
                (self.config.vm.irq_overflow, InterruptType::ResourceOverflow)
            }
            SajuInterrupt::SystemStall => (self.config.vm.irq_stall, InterruptType::SystemStall),
            SajuInterrupt::ServiceInterrupt => {
                (self.config.vm.irq_service, InterruptType::ServiceInterrupt)
            }
        };

        *score -= penalty;
        esil_trace.push_str(&format!("irq_handle:{:?},impact:-{:.1}; ", irq, penalty));
        tags.push(TraceTag::Interrupt {
            irq_type,
            marker: marker_name.into(),
        });

        // Force register modification upon interrupt (Kernel Panic effect)
        if matches!(irq, SajuInterrupt::CriticalException) {
            registers.r2_earth += self.config.vm.irq_critical; // Earth energy surge causing system congestion
            esil_trace.push_str("kernel_panic:earth_overflow; ");
        }
    }

    /// Pipeline instruction execution loop
    fn execute_pipeline(
        &self,
        dynamic: &DynamicLuckAnalysis,
        tags: &mut Vec<TraceTag>,
        esil_trace: &mut String,
        registers: &mut QiRegisters,
        score: &mut f32,
    ) {
        let primary_yongshin = self.yongshin.primary;
        let assistant_yongshin = self.yongshin.assistant;
        let thermal_index =
            crate::analysis::yongshin::calculate_thermal_index(&self.natal, &self.config);

        // Stage definitions
        let stages = [
            (
                dynamic.major_influence.as_ref().map(|i| i.ganzi),
                "Major",
                self.config.vm.pipeline_major,
            ),
            (
                dynamic.yearly_influence.as_ref().map(|i| i.ganzi),
                "Yearly",
                self.config.vm.pipeline_yearly,
            ),
            (
                dynamic.monthly_influence.as_ref().map(|i| i.ganzi),
                "Monthly",
                self.config.vm.pipeline_monthly,
            ),
            (
                dynamic.daily_influence.as_ref().map(|i| i.ganzi),
                "Daily",
                self.config.vm.pipeline_daily,
            ),
            (
                dynamic.hourly_influence.as_ref().map(|i| i.ganzi),
                "Hourly",
                self.config.vm.pipeline_hourly,
            ),
        ];

        let mut prev_el: Option<Element> = None;
        let mut forwarding_buffer: Option<Element> = None;

        for (opt_ganzi, label, base_weight) in stages {
            if let Some(ganzi) = opt_ganzi {
                // Forwarding check: transformed element from stem combination
                let stem_label = format!("{}_luck_stem", label.to_lowercase());
                let eff_el = self
                    .get_effective_stem(dynamic, &stem_label)
                    .unwrap_or(ganzi.stem.element());

                let mut weight = self.get_element_priority(
                    eff_el,
                    primary_yongshin,
                    assistant_yongshin,
                    thermal_index,
                );

                // 1. Data Hazard (Stall): conflict between previous and current stage
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

                // 2. Bypassing (Forwarding): bonus when transformed element aligns with yongshin
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

                // Energy decay and normalization
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

                // Tagging
                if label == "Major" || label == "Yearly" {
                    let period = LuckPeriod::from_label(label);
                    if weight >= 1.5 {
                        tags.push(TraceTag::CoreLuck { period });
                    } else if weight >= 1.0 {
                        tags.push(TraceTag::GoodLuck { period });
                    } else if weight <= -0.5 {
                        tags.push(TraceTag::BadLuck { period });
                    }
                }

                // Store energy for next stage
                prev_el = Some(eff_el);

                // Store transformed energy in forwarding buffer
                if self.get_effective_stem(dynamic, &stem_label).is_some() {
                    forwarding_buffer = Some(eff_el);
                } else {
                    forwarding_buffer = None;
                }
            }
        }
    }

    /// Extract effective stem element after combination transformation
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

    /// Element priority weighting matrix
    pub fn get_element_priority(
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

    /// Extract branch object from position string
    fn get_branch_by_path(
        &self,
        path: &str,
        dynamic: &DynamicLuckAnalysis,
    ) -> Option<crate::core::branch::EarthlyBranch> {
        match path {
            "year_branch" => Some(self.natal.year.branch),
            "month_branch" => Some(self.natal.month.branch),
            "day_branch" => Some(self.natal.day.branch),
            "hour_branch" => Some(self.natal.hour.branch),
            "major_luck_branch" => dynamic.major_influence.as_ref().map(|i| i.ganzi.branch),
            "yearly_luck_branch" => dynamic.yearly_influence.as_ref().map(|i| i.ganzi.branch),
            "monthly_luck_branch" => dynamic.monthly_influence.as_ref().map(|i| i.ganzi.branch),
            "daily_luck_branch" => dynamic.daily_influence.as_ref().map(|i| i.ganzi.branch),
            "hourly_luck_branch" => dynamic.hourly_influence.as_ref().map(|i| i.ganzi.branch),
            _ => None,
        }
    }

    /// Evaluate dynamic combination completion (Triple / Seasonal)
    fn evaluate_dynamic_combinations(
        &self,
        dynamic: &DynamicLuckAnalysis,
        tags: &mut Vec<TraceTag>,
        esil_trace: &mut String,
        registers: &mut QiRegisters,
        score: &mut f32,
    ) {
        let natal_triples = &dynamic.natal_relations.triple_combinations;
        let natal_seasonals = &dynamic.natal_relations.seasonal_combinations;

        // 1. Triple Alliance completion check
        for triple in &dynamic.combined_relations.triple_combinations {
            if !natal_triples.contains(triple) {
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

                let is_beneficial = priority > 0.0;
                tags.push(TraceTag::TripleCombination {
                    element,
                    is_beneficial,
                });
                esil_trace.push_str(&format!(
                    "dynamic_triple:{},bonus:{:.1}; ",
                    triple.hangul(),
                    bonus
                ));
            }
        }

        // 2. Seasonal Alliance completion check
        for seasonal in &dynamic.combined_relations.seasonal_combinations {
            if !natal_seasonals.contains(seasonal) {
                let element = seasonal.element();
                let priority = self.get_element_priority(
                    element,
                    self.yongshin.primary,
                    self.yongshin.assistant,
                    0,
                );

                let bonus = 20.0 * priority;
                *score += bonus;
                registers.update(element, bonus.abs());

                let is_beneficial = priority > 0.0;
                tags.push(TraceTag::SeasonalCombination {
                    element,
                    is_beneficial,
                });
                esil_trace.push_str(&format!(
                    "dynamic_seasonal:{},bonus:{:.1}; ",
                    seasonal.hangul(),
                    bonus
                ));
            }
        }
    }

    /// Life simulation
    pub fn simulate_life(&self, start_age: u32, end_age: u32) -> Vec<LifeFrame> {
        // 1. Calculate major luck flow
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

        // Birth year (reference for yearly luck)
        let birth_year = self.natal.raw_input.year;

        let mapper = |age: u32| {
            // 2. Extract major luck for age
            let major_ganzi = luck_analysis
                .as_ref()
                .and_then(|a| a.at_age(age))
                .map(|m| m.ganzi)
                .unwrap_or_else(|| GanZi::from_index(0));

            // 3. Extract yearly luck for age
            let target_year = birth_year + (age as i32) - 1;
            let yearly_ganzi = YearlyLuck::calculate(target_year, &self.natal).ganzi;

            // 4. Execute precision step
            self.step(age, major_ganzi, yearly_ganzi, None, None, None)
        };

        #[cfg(feature = "parallel")]
        {
            use rayon::prelude::*;
            (start_age..=end_age).into_par_iter().map(mapper).collect()
        }

        #[cfg(not(feature = "parallel"))]
        {
            (start_age..=end_age).map(mapper).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::pillars::SajuInput;

    fn test_vm() -> SajuVM {
        // Jia-Zi year, Jia-Zi month, Jia-Zi day, Jia-Zi hour
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
