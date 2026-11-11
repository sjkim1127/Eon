use crate::dto::{
    DestinyComponent, DomainTier, SajuAnalysisOutput, ScoreResult, TierGrade, TierResult,
    TransitAnalysisOutput, VedicAnalysisOutput,
};
use chrono::Duration;
use eon_vedic::analysis::avasthas::BaladiAvastha;
use eon_vedic::analysis::gochara::SadeSatiPhase;
use eon_vedic::analysis::yogas::YogaQuality;

pub fn analyze(
    saju: SajuAnalysisOutput,
    vedic: VedicAnalysisOutput,
    transit: Option<TransitAnalysisOutput>,
) -> TierResult {
    // #6 Fix: Delegate to analyze_omni to eliminate code duplication
    let omni_input = crate::dto::OmniDestinyTierInput {
        saju,
        vedic,
        western: None,
        zwds: None,
        human_design: None,
        qimen: None,
        numerology: None,
        transit,
    };
    analyze_omni(omni_input)
}

pub fn analyze_omni(input: crate::dto::OmniDestinyTierInput) -> TierResult {
    let saju = &input.saju;
    let vedic = &input.vedic;
    let western = input.western.as_ref();
    let zwds = input.zwds.as_ref();
    let human_design = input.human_design.as_ref();
    let qimen = input.qimen.as_ref();
    let numerology = input.numerology.as_ref();
    let transit = input.transit.as_ref();

    let saju_res = compute_saju_score(saju);
    let vedic_res = compute_vedic_score(vedic);
    let transit_res = transit.map(compute_transit_score).unwrap_or(ScoreResult {
        score: 50.0,
        highlights: vec![],
    });
    let pot_res = compute_potential_score(saju, vedic);

    let current_age = transit.map(|t| t.current_age).unwrap_or(30);

    let detailed_components = compute_omni_detailed_components(
        saju,
        vedic,
        western,
        zwds,
        human_design,
        qimen,
        numerology,
        transit,
        &transit_res,
    );

    let quantum_synergies =
        compute_quantum_synergies(saju, vedic, western, zwds, human_design, qimen, numerology);
    let component_raw: f32 = detailed_components.iter().map(|c| c.score * c.weight).sum();
    let synergy_delta: f32 = quantum_synergies.iter().map(|s| s.score_delta).sum();
    let destiny_raw_score = (component_raw + synergy_delta).clamp(0.0, 100.0);

    let destiny_tier_score = spread_normalize(destiny_raw_score);
    let destiny_tier = get_tier_from_score(destiny_tier_score);

    let mut strengths = Vec::new();
    strengths.extend(saju_res.highlights.iter().take(2).cloned());
    strengths.extend(vedic_res.highlights.iter().take(2).cloned());
    if let Some(w) = western {
        strengths.push(format!(
            "Western Aspect Patterns: {} configurations",
            w.result.aspect_patterns.len()
        ));
    }
    if let Some(z) = zwds {
        strengths.push(format!(
            "Zi Wei Dou Shu Life Palace: {}",
            z.chart.palaces[z.chart.soul_idx].name.english()
        ));
    }
    if let Some(hd) = human_design {
        strengths.push(format!(
            "Human Design: Type {}, Profile {}",
            hd.result.chart_type, hd.result.profile
        ));
    }
    if let Some(qm) = qimen {
        strengths.push(format!(
            "QiMen DunJia: Ju {} ({})",
            qm.report.pan.ju_number,
            if qm.report.pan.is_yin_ju {
                "Yin Dun"
            } else {
                "Yang Dun"
            }
        ));
    }
    if let Some(num) = numerology {
        strengths.push(format!(
            "Pythagorean Numerology: Life Path {}, Expression {}",
            num.result.core.life_path, num.result.core.expression
        ));
    }

    let mut weaknesses = Vec::new();
    if let Some(vr) = &saju.vulnerability_report {
        if vr.total_crashes > 30 {
            weaknesses.push(format!("Cautionary timing points: {}", vr.total_crashes));
        }
    }
    if matches!(vedic.report.sade_sati, SadeSatiPhase::Peak) {
        weaknesses.push("Sade Sati Peak Phase".to_string());
    }

    let mut profile = "balanced".to_string();
    if current_age < 35 {
        profile = "growth".to_string();
    } else if current_age > 55 {
        profile = "stable".to_string();
    }

    let mut domain_tiers = Vec::new();
    for h in &vedic.report.house_summary {
        let tier = match h.rating.as_str() {
            "Excellent" => "S",
            "Strong" => "A",
            "Average" => "B",
            "Weak" => "C",
            _ => "D",
        };
        let domain_name = match h.house {
            1 => "Self & Vitality",
            2 => "Wealth & Assets",
            3 => "Courage & Siblings",
            4 => "Home & Foundations",
            5 => "Children & Creativity",
            6 => "Health & Service",
            7 => "Partnership & Marriage",
            8 => "Transformation & Longevity",
            9 => "Wisdom & Fortune",
            10 => "Career & Status",
            11 => "Aspirations & Gains",
            12 => "Spirituality & Solitude",
            _ => "General",
        };
        domain_tiers.push(DomainTier {
            house: h.house,
            domain: domain_name.to_string(),
            tier: tier.to_string(),
        });
    }

    let potential_tier_score = spread_normalize(pot_res.score);
    let potential_tier = get_tier_from_score(potential_tier_score);

    let domain_radar =
        compute_8d_domain_radar(saju, vedic, western, zwds, human_design, qimen, numerology);
    let tier_trajectory = compute_temporal_tier_trajectory(saju, vedic);

    TierResult {
        natal_score: clamp_score(destiny_tier_score),
        current_score: transit_res.score,
        destiny_score: destiny_tier_score,
        destiny_tier: destiny_tier.clone(),
        potential_score: potential_tier_score,
        potential_tier,
        domain_tiers,
        saju_result: saju_res,
        vedic_result: vedic_res,
        transit_result: transit_res.clone(),
        strengths,
        weaknesses,
        growth_gap: (potential_tier_score - destiny_tier_score).round(),
        risk_level: compute_risk_level(saju, vedic, &transit_res),
        profile,
        version: "v7.0_omni_8engine_model".to_string(),
        destiny_raw_score,
        destiny_tier_score,
        detailed_components,
        tier_model_version: "7.0.0".to_string(),
        quantum_synergies,
        domain_radar,
        tier_trajectory,
    }
}

fn compute_omni_detailed_components(
    saju: &SajuAnalysisOutput,
    vedic: &VedicAnalysisOutput,
    western: Option<&crate::dto::WesternAnalysisOutput>,
    zwds: Option<&crate::dto::ZwdsAnalysisOutput>,
    human_design: Option<&crate::dto::HumanDesignAnalysisOutput>,
    qimen: Option<&crate::dto::QimenAnalysisOutput>,
    numerology: Option<&crate::dto::NumerologyAnalysisOutput>,
    _transit_opt: Option<&TransitAnalysisOutput>,
    transit_res: &ScoreResult,
) -> Vec<DestinyComponent> {
    let mut components = Vec::new();

    // 1. Saju Core Strength (Weight: 0.10)
    let saju_strength_score = saju.report.strength.strength_score.abs().min(50.0) * 2.0;
    components.push(DestinyComponent {
        key: "saju_strength".to_string(),
        label: "Saju Day Master Strength".to_string(),
        score: saju_strength_score,
        weight: 0.10,
        reasons: vec![format!(
            "Day Master strength index: {:.1}",
            saju.report.strength.strength_score
        )],
    });

    // 2. Element Flow (Weight: 0.07)
    components.push(DestinyComponent {
        key: "element_flow".to_string(),
        label: "Five Elements Flow Efficiency".to_string(),
        score: saju.qi_topology.throughput * 100.0,
        weight: 0.07,
        reasons: vec![format!(
            "Five Elements circulation efficiency: {:.1}%",
            saju.qi_topology.throughput * 100.0
        )],
    });

    // 3. Structure & Yongshin (Weight: 0.08)
    let structure_score = if matches!(
        saju.report.structure.structure,
        eon_saju::analysis::structure::StructureType::Follower
    ) {
        40.0
    } else {
        85.0
    };
    components.push(DestinyComponent {
        key: "structure".to_string(),
        label: "Saju Structure & Yongshin".to_string(),
        score: structure_score,
        weight: 0.08,
        reasons: vec![format!("Structure: {:?}", saju.report.structure.structure)],
    });

    // 4. Spirit Markers (Weight: 0.05)
    let spirit_score = (saju.report.spirit_markers.auspicious.len() as f32 * 20.0).min(100.0)
        - (saju.report.spirit_markers.inauspicious.len() as f32 * 10.0).max(0.0);
    components.push(DestinyComponent {
        key: "spirit_markers".to_string(),
        label: "Saju Auspicious & Inauspicious Stars".to_string(),
        score: spirit_score.clamp(0.0, 100.0),
        weight: 0.05,
        reasons: vec![format!(
            "{} auspicious, {} inauspicious stars",
            saju.report.spirit_markers.auspicious.len(),
            saju.report.spirit_markers.inauspicious.len()
        )],
    });

    // 5. Vedic House Score (Weight: 0.09)
    let house_score = vedic.report.overall_strength_score as f32 / 6.0;
    components.push(DestinyComponent {
        key: "vedic_houses".to_string(),
        label: "Vedic 12 House Capacity".to_string(),
        score: house_score.clamp(0.0, 100.0),
        weight: 0.09,
        reasons: vec![format!("House average strength: {:.1}", house_score)],
    });

    // 6. Vedic Yogas (Weight: 0.08)
    let yoga_score = (vedic
        .report
        .yogas
        .iter()
        .filter(|y| matches!(y.quality, YogaQuality::VeryHigh))
        .count() as f32
        * 30.0)
        .min(100.0);
    components.push(DestinyComponent {
        key: "vedic_yogas".to_string(),
        label: "Vedic Classical Yogas (108 Major)".to_string(),
        score: yoga_score,
        weight: 0.08,
        reasons: vec![format!(
            "{} top-tier yogas detected",
            vedic
                .report
                .yogas
                .iter()
                .filter(|y| matches!(y.quality, YogaQuality::VeryHigh))
                .count()
        )],
    });

    // 7. Planet Status (Avasthas) (Weight: 0.05)
    let avastha_score = (vedic
        .chart
        .avasthas
        .iter()
        .filter(|a| matches!(a.baladi, BaladiAvastha::Bala | BaladiAvastha::Yuva))
        .count() as f32
        / 7.0)
        * 100.0;
    components.push(DestinyComponent {
        key: "planet_status".to_string(),
        label: "Vedic Planetary Vitality (Avasthas)".to_string(),
        score: avastha_score.clamp(0.0, 100.0),
        weight: 0.05,
        reasons: vec![format!(
            "{} active state planets",
            vedic
                .chart
                .avasthas
                .iter()
                .filter(|a| matches!(a.baladi, BaladiAvastha::Bala | BaladiAvastha::Yuva))
                .count()
        )],
    });

    // 8. Western Dignities & Aspects (Weight: 0.09)
    let west_score = if let Some(w) = western {
        let pattern_bonus = w.result.aspect_patterns.len() as f32 * 15.0;
        let aspect_score = (w.result.aspects.len() as f32 * 2.0).min(50.0);
        (50.0 + pattern_bonus + aspect_score).min(100.0)
    } else {
        55.0
    };
    components.push(DestinyComponent {
        key: "western_astrology".to_string(),
        label: "Western Aspects & Essential Dignities".to_string(),
        score: west_score,
        weight: 0.09,
        reasons: vec![if let Some(w) = western {
            format!(
                "{} aspects, {} aspect patterns detected",
                w.result.aspects.len(),
                w.result.aspect_patterns.len()
            )
        } else {
            "Default Western astrology module applied".to_string()
        }],
    });

    // 9. ZWDS Pattern & Sihua (Weight: 0.09)
    let zwds_score = if let Some(z) = zwds {
        let pat_score = (z.chart.destiny_patterns.len() as f32 * 20.0).min(60.0);
        let collision_penalty = z.chart.collisions.len() as f32 * 10.0;
        (50.0 + pat_score - collision_penalty).clamp(20.0, 100.0)
    } else {
        55.0
    };
    components.push(DestinyComponent {
        key: "zwds_harmony".to_string(),
        label: "ZWDS Formations & Flying SiHua Clashes".to_string(),
        score: zwds_score,
        weight: 0.09,
        reasons: vec![if let Some(z) = zwds {
            format!(
                "{} destiny formations, {} SiHua collisions",
                z.chart.destiny_patterns.len(),
                z.chart.collisions.len()
            )
        } else {
            "Default Zi Wei Dou Shu module applied".to_string()
        }],
    });

    // 10. Human Design Dynamics (Weight: 0.07)
    let hd_score = if let Some(hd) = human_design {
        let center_bonus = hd.result.defined_centers.len() as f32 * 10.0;
        let channel_bonus = hd.result.active_channels.len() as f32 * 12.0;
        (30.0 + center_bonus + channel_bonus).clamp(20.0, 100.0)
    } else {
        55.0
    };
    components.push(DestinyComponent {
        key: "human_design_dynamics".to_string(),
        label: "Human Design Centers & Active Channels".to_string(),
        score: hd_score,
        weight: 0.07,
        reasons: vec![if let Some(hd) = human_design {
            format!(
                "{} defined centers, {} active channels, Type: {}",
                hd.result.defined_centers.len(),
                hd.result.active_channels.len(),
                hd.result.chart_type
            )
        } else {
            "Default Human Design module applied".to_string()
        }],
    });

    // 11. Qimen Harmony (Weight: 0.06)
    let qimen_score = if let Some(qm) = qimen {
        let ju_base = (qm.report.pan.ju_number as f32 * 8.0).min(70.0);
        let palace_bonus = (qm.report.pan.palaces.len() as f32 * 3.0).min(30.0);
        (20.0 + ju_base + palace_bonus).clamp(20.0, 100.0)
    } else {
        55.0
    };
    components.push(DestinyComponent {
        key: "qimen_harmony".to_string(),
        label: "QiMen 9 Palaces & Auspicious Doors".to_string(),
        score: qimen_score,
        weight: 0.06,
        reasons: vec![if let Some(qm) = qimen {
            format!(
                "QiMen DunJia Ju {} ({}), Value Chief: {:?}, Envoy Door: {:?}",
                qm.report.pan.ju_number,
                if qm.report.pan.is_yin_ju {
                    "Yin Dun"
                } else {
                    "Yang Dun"
                },
                qm.report
                    .pan
                    .value_chief_star
                    .as_ref()
                    .map(|s| format!("{:?}", s))
                    .unwrap_or_default(),
                qm.report
                    .pan
                    .value_envoy_door
                    .as_ref()
                    .map(|d| format!("{:?}", d))
                    .unwrap_or_default()
            )
        } else {
            "Default QiMen DunJia module applied".to_string()
        }],
    });

    // 12. Numerology Vibration (Weight: 0.06)
    let num_score = if let Some(num) = numerology {
        let master_bonus = if num.result.is_master_life_path {
            20.0
        } else {
            0.0
        };
        let lp_val = (num.result.core.life_path as f32 * 6.0).min(50.0);
        (30.0 + lp_val + master_bonus).clamp(20.0, 100.0)
    } else {
        55.0
    };
    components.push(DestinyComponent {
        key: "numerology_vibration".to_string(),
        label: "Pythagorean Numerology Vibrations".to_string(),
        score: num_score,
        weight: 0.06,
        reasons: vec![if let Some(num) = numerology {
            format!(
                "Life Path: {}, Expression: {}, Soul Urge: {}",
                num.result.core.life_path, num.result.core.expression, num.result.core.soul_urge
            )
        } else {
            "Default Numerology module applied".to_string()
        }],
    });

    // 13. Luck Cycle & Transits (Weight: 0.05)
    let luck_score = transit_res.score;
    components.push(DestinyComponent {
        key: "luck_cycle".to_string(),
        label: "Current Transit Dynamics".to_string(),
        score: luck_score,
        weight: 0.05,
        reasons: vec![format!("Transit composite score: {:.1}", luck_score)],
    });

    // 14. Stability & System Safety (Weight: 0.03)
    let risk_inv = 100.0
        - (saju
            .vulnerability_report
            .as_ref()
            .map(|v| v.total_crashes)
            .unwrap_or(0) as f32
            * 2.0)
            .min(100.0);
    components.push(DestinyComponent {
        key: "stability".to_string(),
        label: "Destiny Resilience & Risk Prevention".to_string(),
        score: risk_inv,
        weight: 0.03,
        reasons: vec![format!(
            "Vulnerability crash index: {}",
            saju.vulnerability_report
                .as_ref()
                .map(|v| v.total_crashes)
                .unwrap_or(0)
        )],
    });

    // 15. Golden Time (Weight: 0.03)
    let golden_score = saju
        .report
        .golden_time
        .as_ref()
        .map(|gt| gt.average_score)
        .unwrap_or(50.0);
    components.push(DestinyComponent {
        key: "golden_time".to_string(),
        label: "Golden Era Timing Metric".to_string(),
        score: golden_score,
        weight: 0.03,
        reasons: vec![saju
            .report
            .golden_time
            .as_ref()
            .map(|gt| format!("Optimal golden era: ages {}~{}", gt.start_age, gt.end_age))
            .unwrap_or_else(|| "Golden era undetermined".to_string())],
    });

    components
}

fn compute_detailed_components(
    saju: &SajuAnalysisOutput,
    vedic: &VedicAnalysisOutput,
    _transit_opt: Option<&TransitAnalysisOutput>,
    transit_res: &ScoreResult,
) -> Vec<DestinyComponent> {
    compute_omni_detailed_components(
        saju,
        vedic,
        None,
        None,
        None,
        None,
        None,
        _transit_opt,
        transit_res,
    )
}

fn clamp_score(score: f32) -> f32 {
    score.clamp(0.0, 100.0)
}

/// Deterministic Spread Normalization
/// Expands the score distribution to cover the full 0-100 range more effectively.
fn spread_normalize(score: f32) -> f32 {
    let s = clamp_score(score);
    // Sigmoid-like stretch around 50 to pull clusters away from the middle
    let normalized = if s < 50.0 {
        // [0, 50] -> [0, 50] with downward curve
        50.0 * (s / 50.0).powf(1.4)
    } else {
        // [50, 100] -> [50, 100] with upward curve
        50.0 + 50.0 * ((s - 50.0) / 50.0).powf(0.7) // Squareroot-like to boost high-mid scores
    };
    normalized.round()
}

fn get_tier_from_score(score: f32) -> TierGrade {
    let s = score.round() as i32;
    if s >= 97 {
        TierGrade {
            grade: "S+".to_string(),
            label: "Celestial Sovereign".to_string(),
            desc: "A destiny of supreme nobility anchored by celestial harmony.".to_string(),
        }
    } else if s >= 90 {
        TierGrade {
            grade: "S".to_string(),
            label: "Heavenly Fortune".to_string(),
            desc: "Great noble destiny blessed by heaven and illuminated by earth.".to_string(),
        }
    } else if s >= 83 {
        TierGrade {
            grade: "A+".to_string(),
            label: "Great Auspice".to_string(),
            desc: "Abundant fortune and commanding momentum admired by all.".to_string(),
        }
    } else if s >= 75 {
        TierGrade {
            grade: "A".to_string(),
            label: "High Auspice".to_string(),
            desc: "Highly auspicious destiny destined for clear, sustained achievements."
                .to_string(),
        }
    } else if s >= 67 {
        TierGrade {
            grade: "B+".to_string(),
            label: "Favorable Auspice".to_string(),
            desc: "Gentle momentum allowing talents to unfold smoothly.".to_string(),
        }
    } else if s >= 59 {
        TierGrade {
            grade: "B".to_string(),
            label: "Auspicious".to_string(),
            desc: "Stable, well-grounded destiny with steady prosperity.".to_string(),
        }
    } else if s >= 51 {
        TierGrade {
            grade: "C+".to_string(),
            label: "Moderate Auspice".to_string(),
            desc: "Above-average resilience where diligent effort yields bright results."
                .to_string(),
        }
    } else if s >= 43 {
        TierGrade {
            grade: "C".to_string(),
            label: "Moderate Equilibrium".to_string(),
            desc: "Experiences life fluctuations but fully equipped to navigate through."
                .to_string(),
        }
    } else if s >= 35 {
        TierGrade {
            grade: "D+".to_string(),
            label: "Moderate Caution".to_string(),
            desc: "Frequent turning points requiring patience to carve out pathways.".to_string(),
        }
    } else if s >= 27 {
        TierGrade {
            grade: "D".to_string(),
            label: "Cautionary".to_string(),
            desc: "Demands careful attention, prudent conduct, and strategic timing.".to_string(),
        }
    } else if s >= 18 {
        TierGrade {
            grade: "E".to_string(),
            label: "Perseverance".to_string(),
            desc: "A journey of profound patience and endurance before blossoming.".to_string(),
        }
    } else {
        TierGrade {
            grade: "F".to_string(),
            label: "Adversity".to_string(),
            desc: "Rugged terrain requiring inner cultivation and wisdom to avert hardship."
                .to_string(),
        }
    }
}

fn compute_risk_level(
    saju: &SajuAnalysisOutput,
    vedic: &VedicAnalysisOutput,
    transit_res: &ScoreResult,
) -> String {
    let mut risk_points = 0;
    if let Some(vr) = &saju.vulnerability_report {
        if vr.total_crashes >= 10 {
            risk_points += 1;
        }
        if vr.total_crashes >= 30 {
            risk_points += 2;
        }
    }
    if transit_res.score < 40.0 {
        risk_points += 1;
    }
    if matches!(
        vedic.report.sade_sati,
        SadeSatiPhase::Peak | SadeSatiPhase::Rising
    ) {
        risk_points += 2;
    }

    if risk_points >= 4 {
        "high".to_string()
    } else if risk_points >= 2 {
        "medium".to_string()
    } else {
        "low".to_string()
    }
}

// Keep helper functions for computing sub-scores (legacy/compatibility)
fn compute_saju_score(saju: &SajuAnalysisOutput) -> ScoreResult {
    let mut score = 0.0;
    let mut highlights = Vec::new();

    let r = &saju.report;
    let st = &r.strength;

    let strength_norm = clamp_score(st.strength_score * 2.0);
    score += strength_norm * 0.28;

    let acquired = vec![
        st.deuk_ryeong.acquired,
        st.deuk_ji.acquired,
        st.deuk_si.acquired,
    ]
    .into_iter()
    .filter(|&x| x)
    .count() as f32;
    score += acquired * 4.0;
    if acquired == 3.0 {
        highlights.push("Triple Acquisition (Season, Position, Time) complete".to_string());
    }

    let support_pct = if st.deuk_se.support_ratio > 1.0 {
        st.deuk_se.support_ratio
    } else {
        st.deuk_se.support_ratio * 100.0
    };
    score += (support_pct / 100.0) * 10.0;

    let topo = &saju.qi_topology;
    score += topo.throughput * 12.0;

    let sm = &r.spirit_markers;
    let aus = sm.auspicious.len() as f32;
    let inaus = sm.inauspicious.len() as f32;
    score += 5.0_f32.min(aus * 1.0);
    score -= 4.0_f32.min(inaus * 0.8);

    if let Some(gt) = &r.golden_time {
        let len = (gt.end_age - gt.start_age) as f32;
        score += 6.0_f32.min(len * 0.4);
    }

    ScoreResult {
        score: clamp_score(score.round()),
        highlights,
    }
}

fn compute_vedic_score(vedic: &VedicAnalysisOutput) -> ScoreResult {
    let mut score = 0.0;
    let mut highlights = Vec::new();

    let r = &vedic.report;
    score += clamp_score(r.overall_strength_score as f32 / 6.0) * 0.35;

    let vh_yogas: Vec<_> = r
        .yogas
        .iter()
        .filter(|y| matches!(y.quality, YogaQuality::VeryHigh | YogaQuality::High))
        .collect();
    let vh_count = vh_yogas
        .iter()
        .filter(|y| matches!(y.quality, YogaQuality::VeryHigh))
        .count() as f32;
    score += 12.0_f32.min(vh_count * 4.0);

    // #7 Fix: Populate highlights with meaningful vedic info
    for yoga in vh_yogas.iter().take(3) {
        highlights.push(format!("{} ({:?})", yoga.name, yoga.yoga_type));
    }

    let ex_houses = r
        .house_summary
        .iter()
        .filter(|h| h.rating == "Excellent")
        .count() as f32;
    score += 10.0_f32.min(ex_houses * 2.0);

    if ex_houses > 0.0 {
        highlights.push(format!("{} Excellent rating houses", ex_houses as u32));
    }

    if matches!(r.sade_sati, SadeSatiPhase::Peak) {
        highlights.push("Sade Sati Peak Phase active".to_string());
    } else if matches!(r.sade_sati, SadeSatiPhase::None) {
        highlights.push("Sade Sati inactive (Stable)".to_string());
    }

    ScoreResult {
        score: clamp_score(score.round()),
        highlights,
    }
}

fn compute_transit_score(transit: &TransitAnalysisOutput) -> ScoreResult {
    let score = if let Some(f) = &transit.current_frame {
        clamp_score(f.score)
    } else {
        50.0
    };
    ScoreResult {
        score: clamp_score(score.round()),
        highlights: vec![],
    }
}

fn compute_potential_score(saju: &SajuAnalysisOutput, vedic: &VedicAnalysisOutput) -> ScoreResult {
    let mut score = 50.0;
    let topo = &saju.qi_topology;
    score += topo.throughput * 12.0;

    if let Some(gt) = &saju.report.golden_time {
        let len = (gt.end_age - gt.start_age) as f32;
        score += 12.0_f32.min(len * 0.5);
    }

    let high_yogas = vedic
        .report
        .yogas
        .iter()
        .filter(|y| matches!(y.quality, YogaQuality::VeryHigh | YogaQuality::High))
        .count() as f32;
    score += 10.0_f32.min(high_yogas * 3.0);

    ScoreResult {
        score: clamp_score(score.round()),
        highlights: vec![],
    }
}

fn compute_quantum_synergies(
    saju: &SajuAnalysisOutput,
    vedic: &VedicAnalysisOutput,
    western: Option<&crate::dto::WesternAnalysisOutput>,
    zwds: Option<&crate::dto::ZwdsAnalysisOutput>,
    human_design: Option<&crate::dto::HumanDesignAnalysisOutput>,
    qimen: Option<&crate::dto::QimenAnalysisOutput>,
    numerology: Option<&crate::dto::NumerologyAnalysisOutput>,
) -> Vec<crate::dto::QuantumSynergyItem> {
    let mut items = Vec::new();

    let saju_ausp_count = saju.report.spirit_markers.auspicious.len();
    let vedic_yoga_count = vedic
        .report
        .yogas
        .iter()
        .filter(|y| matches!(y.quality, YogaQuality::VeryHigh))
        .count();
    if saju_ausp_count >= 2 && vedic_yoga_count >= 1 {
        items.push(crate::dto::QuantumSynergyItem {
            title: "Noble Alignment (East-West Auspicious Synergy)".to_string(),
            engines: vec!["Saju".to_string(), "Vedic".to_string()],
            score_delta: 12.0,
            is_positive: true,
            description: "Saju auspicious spirit markers and top-tier Vedic yogas align in cross-engine synergy, confirming powerful celestial momentum.".to_string(),
        });
    }

    if let (Some(w), Some(z)) = (western, zwds) {
        if !w.result.aspect_patterns.is_empty() && !z.chart.destiny_patterns.is_empty() {
            items.push(crate::dto::QuantumSynergyItem {
                title: "Geometric Resonance (Western Aspects & ZWDS Formations)".to_string(),
                engines: vec!["Western".to_string(), "ZWDS".to_string()],
                score_delta: 15.0,
                is_positive: true,
                description: "Western planetary aspect geometry resonates directly with Zi Wei Dou Shu primary palace formations.".to_string(),
            });
        }
    }

    if let (Some(hd), Some(qm)) = (human_design, qimen) {
        if hd.result.defined_centers.len() >= 3 && qm.report.pan.ju_number > 0 {
            items.push(crate::dto::QuantumSynergyItem {
                title: "Quantum Energy Alignment (Human Design Centers & QiMen 9 Palaces)".to_string(),
                engines: vec!["Human Design".to_string(), "QiMen".to_string()],
                score_delta: 14.0,
                is_positive: true,
                description: "Human Design defined center flows align with QiMen DunJia Nine Palaces auspicious door layouts.".to_string(),
            });
        }
    }

    if let Some(num) = numerology {
        if num.result.is_master_life_path && (saju_ausp_count >= 1 || vedic_yoga_count >= 1) {
            items.push(crate::dto::QuantumSynergyItem {
                title: "Master Number Resonance (Pythagorean Vibration & Universal Blessing)".to_string(),
                engines: vec!["Numerology".to_string(), "Saju/Vedic".to_string()],
                score_delta: 16.0,
                is_positive: true,
                description: "High-order vibrations of Pythagorean Master Numbers (11/22/33) unify with East-West noble auspicious markers.".to_string(),
            });
        }
    }

    if saju_ausp_count >= 1
        && vedic_yoga_count >= 1
        && western
            .map(|w| !w.result.aspects.is_empty())
            .unwrap_or(false)
        && zwds
            .map(|z| !z.chart.destiny_patterns.is_empty())
            .unwrap_or(false)
        && human_design
            .map(|hd| !hd.result.defined_centers.is_empty())
            .unwrap_or(false)
        && qimen
            .map(|qm| !qm.report.pan.palaces.is_empty())
            .unwrap_or(false)
        && numerology
            .map(|n| n.result.core.life_path > 0)
            .unwrap_or(false)
    {
        items.push(crate::dto::QuantumSynergyItem {
            title: "8-Engine Universal Harmony (Full Cross-Engine Alignment)".to_string(),
            engines: vec!["Saju".to_string(), "Vedic".to_string(), "Western".to_string(), "ZWDS".to_string(), "HD".to_string(), "QiMen".to_string(), "Numerology".to_string()],
            score_delta: 22.0,
            is_positive: true,
            description: "All 8 destiny engines (Saju, Vedic, Western, ZWDS, Human Design, QiMen, Numerology) harmonize in unanimous celestial alignment.".to_string(),
        });
    }

    if saju
        .vulnerability_report
        .as_ref()
        .map(|v| v.total_crashes)
        .unwrap_or(0)
        > 20
    {
        items.push(crate::dto::QuantumSynergyItem {
            title: "Inflection Point Collision Alert (Timing Risk Warning)".to_string(),
            engines: vec!["Saju VM".to_string()],
            score_delta: -10.0,
            is_positive: false,
            description: "Temporal simulation detects concentrated clash/punishment inflection points, advising strategic timing and caution.".to_string(),
        });
    }

    items
}

fn compute_8d_domain_radar(
    saju: &SajuAnalysisOutput,
    vedic: &VedicAnalysisOutput,
    western: Option<&crate::dto::WesternAnalysisOutput>,
    zwds: Option<&crate::dto::ZwdsAnalysisOutput>,
    human_design: Option<&crate::dto::HumanDesignAnalysisOutput>,
    qimen: Option<&crate::dto::QimenAnalysisOutput>,
    numerology: Option<&crate::dto::NumerologyAnalysisOutput>,
) -> Vec<crate::dto::DomainRadarTier> {
    let mut domains = Vec::new();
    let saju_str = saju.report.strength.strength_score.abs().min(50.0) * 2.0;

    let domain_defs: [(&str, &str, &[u8], f64); 8] = [
        ("self_health", "Self & Vitality", &[1, 6], 0.15),
        ("wealth_asset", "Wealth & Assets", &[2, 11], 0.15),
        ("career_honor", "Career & Honor", &[10, 6], 0.15),
        ("knowledge_wisdom", "Knowledge & Wisdom", &[9, 3], 0.10),
        (
            "relationship_marriage",
            "Partnership & Marriage",
            &[7, 5],
            0.12,
        ),
        ("lineage_descendants", "Lineage & Children", &[5, 4], 0.10),
        ("network_social", "Network & Influence", &[11, 3], 0.11),
        (
            "spirituality_intuition",
            "Spirituality & Intuition",
            &[12, 8],
            0.12,
        ),
    ];

    for (key, name, house_nums, weight) in domain_defs {
        let mut house_scores: Vec<f32> = Vec::new();
        for &hn in house_nums {
            let v_h = vedic.report.house_summary.iter().find(|h| h.house == hn);
            let hs = match v_h.map(|h| h.rating.as_str()).unwrap_or("Average") {
                "Excellent" => 92.0,
                "Strong" => 80.0,
                "Average" => 65.0,
                "Weak" => 48.0,
                _ => 35.0,
            };
            house_scores.push(hs);
        }
        let base_score = if house_scores.is_empty() {
            65.0
        } else {
            house_scores.iter().sum::<f32>() / house_scores.len() as f32
        };

        let w_bonus = western
            .map(|w| {
                let relevant = w
                    .result
                    .planets
                    .iter()
                    .filter(|p| house_nums.iter().any(|&hn| p.house_number == hn as usize))
                    .count();
                (relevant as f32 * 8.0).min(15.0)
            })
            .unwrap_or(3.0);

        let z_bonus = zwds
            .map(|z| {
                let mut bonus = 0.0f32;
                for &hn in house_nums {
                    if let Some(palace) = z.chart.palaces.get(hn as usize % 12) {
                        bonus += palace.stars.len() as f32 * 3.0;
                    }
                }
                bonus.min(15.0)
            })
            .unwrap_or(3.0);

        let hd_bonus = human_design
            .map(|hd| (hd.result.defined_centers.len() as f32 * 2.5).min(15.0))
            .unwrap_or(3.0);

        let qm_bonus = qimen
            .map(|qm| (qm.report.pan.ju_number as f32 * 1.5).min(10.0))
            .unwrap_or(2.0);

        let num_bonus = numerology
            .map(|n| (n.result.core.life_path as f32 * 1.2).min(10.0))
            .unwrap_or(2.0);

        let final_score = (base_score * 0.45
            + saju_str * 0.10
            + w_bonus
            + z_bonus
            + hd_bonus
            + qm_bonus
            + num_bonus)
            .clamp(20.0, 99.0);
        let tier = get_tier_from_score(final_score).grade;

        let primary_house = house_nums[0];
        let primary_rating = vedic
            .report
            .house_summary
            .iter()
            .find(|h| h.house == primary_house)
            .map(|h| h.rating.as_str())
            .unwrap_or("N/A");

        domains.push(crate::dto::DomainRadarTier {
            domain_key: key.to_string(),
            domain_name: name.to_string(),
            score: final_score.round(),
            tier,
            key_factors: vec![
                format!("Vedic House {} rating: {}", primary_house, primary_rating),
                format!("Eastern Element weight: {:.1}%", weight * 100.0),
            ],
        });
    }

    domains
}

fn compute_temporal_tier_trajectory(
    saju: &SajuAnalysisOutput,
    vedic: &VedicAnalysisOutput,
) -> Vec<crate::dto::TemporalTierPoint> {
    let mut trajectory = Vec::new();
    let frames = &saju.report.simulation_frames;

    // #3 Fix: Build a vedic Dasha favorability lookup by approximate age
    let dasha_timeline = &vedic.report.dasha_timeline;

    if !frames.is_empty() {
        for frame in frames.iter().step_by(10) {
            // Find the Dasha period active around this age (approximate)
            let dasha_bonus = compute_dasha_bonus_for_age(dasha_timeline, frame.age);

            // Blend saju simulation (60%) + vedic dasha favorability (40%)
            let blended_score = (frame.score * 0.6 + dasha_bonus * 0.4).clamp(0.0, 100.0);

            let grade = get_tier_from_score(blended_score).grade;
            let dasha_lord_name = find_dasha_lord_at_age(dasha_timeline, frame.age);
            let note = if blended_score >= 80.0 {
                format!("Apex Era (Peak Luck Phase) — Dasha: {}", dasha_lord_name)
            } else if blended_score >= 60.0 {
                format!(
                    "Growth Era (Accumulation & Expansion Phase) — Dasha: {}",
                    dasha_lord_name
                )
            } else {
                format!(
                    "Restoration Era (Inner Cultivation Phase) — Dasha: {}",
                    dasha_lord_name
                )
            };
            trajectory.push(crate::dto::TemporalTierPoint {
                age: frame.age,
                score: blended_score.round(),
                tier_grade: grade,
                note,
            });
        }
    } else {
        // Fallback: use Dasha timeline alone if no simulation frames
        for age in (10..=80).step_by(10) {
            let dasha_bonus = compute_dasha_bonus_for_age(dasha_timeline, age);
            let score = (50.0 + dasha_bonus * 0.5).clamp(20.0, 99.0);
            let grade = get_tier_from_score(score).grade;
            let lord = find_dasha_lord_at_age(dasha_timeline, age);
            trajectory.push(crate::dto::TemporalTierPoint {
                age,
                score: score.round(),
                tier_grade: grade,
                note: format!("Dasha estimation: {}", lord),
            });
        }
    }

    trajectory
}

/// Returns a favorability score (0..100) for the Dasha period active at a given age
fn compute_dasha_bonus_for_age(
    timeline: &[eon_vedic::analysis::dasha::DashaPeriod],
    age: u32,
) -> f32 {
    let Some(first) = timeline.first() else {
        return 60.0;
    };
    let target = first.start_time + Duration::seconds((age as f64 * 365.2425 * 86_400.0) as i64);
    for (i, period) in timeline.iter().enumerate() {
        if target >= period.start_time && target < period.end_time {
            if let Some(favorable) = period.is_favorable {
                if favorable {
                    // Stagger scores slightly by period index for variety
                    return (75.0 + (i as f32 * 2.0) % 20.0).min(95.0);
                } else {
                    return (35.0 + (i as f32 * 3.0) % 15.0).min(50.0);
                }
            }
            return 60.0;
        }
    }
    // Neutral default
    60.0
}

/// Returns the name of the Dasha lord active at a given age
fn find_dasha_lord_at_age(
    timeline: &[eon_vedic::analysis::dasha::DashaPeriod],
    age: u32,
) -> String {
    let Some(first) = timeline.first() else {
        return "N/A".to_string();
    };
    let target = first.start_time + Duration::seconds((age as f64 * 365.2425 * 86_400.0) as i64);
    timeline
        .iter()
        .find(|period| target >= period.start_time && target < period.end_time)
        .map(|period| format!("{:?}", period.lord))
        .unwrap_or_else(|| format!("{:?}", first.lord))
}

#[cfg(test)]
mod dasha_trajectory_tests {
    use super::{compute_dasha_bonus_for_age, find_dasha_lord_at_age};
    use chrono::{Duration, TimeZone, Utc};
    use eon_vedic::analysis::dasha::DashaPeriod;
    use eon_vedic::planets::VedicPlanet;

    #[test]
    fn selects_period_by_age_instead_of_always_using_first_period() {
        let birth = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
        let first_end = birth + Duration::days(7 * 365);
        let second_end = first_end + Duration::days(20 * 365);
        let timeline = vec![
            DashaPeriod {
                lord: VedicPlanet::Ketu,
                start_time: birth,
                end_time: first_end,
                sub_dashas: vec![],
                name: None,
                interpretation: None,
                is_favorable: Some(false),
            },
            DashaPeriod {
                lord: VedicPlanet::Venus,
                start_time: first_end,
                end_time: second_end,
                sub_dashas: vec![],
                name: None,
                interpretation: None,
                is_favorable: Some(true),
            },
        ];

        assert_eq!(find_dasha_lord_at_age(&timeline, 1), "Ketu");
        assert_eq!(find_dasha_lord_at_age(&timeline, 10), "Venus");
        assert!(compute_dasha_bonus_for_age(&timeline, 10) > 70.0);
    }
}
