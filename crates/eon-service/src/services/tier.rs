use crate::dto::{
    DestinyComponent, DomainTier, SajuAnalysisOutput, ScoreResult, TierGrade, TierResult,
    TransitAnalysisOutput, VedicAnalysisOutput,
};
use eon_vedic::analysis::avasthas::BaladiAvastha;
use eon_vedic::analysis::gochara::SadeSatiPhase;
use eon_vedic::analysis::yogas::YogaQuality;

pub fn analyze(
    saju: SajuAnalysisOutput,
    vedic: VedicAnalysisOutput,
    transit: Option<TransitAnalysisOutput>,
) -> TierResult {
    let saju_res = compute_saju_score(&saju);
    let vedic_res = compute_vedic_score(&vedic);
    let transit_res = transit
        .as_ref()
        .map(compute_transit_score)
        .unwrap_or(ScoreResult {
            score: 50.0,
            highlights: vec![],
        });
    let pot_res = compute_potential_score(&saju, &vedic);

    let current_age = transit.as_ref().map(|t| t.current_age).unwrap_or(30);

    // 12-Component Detailed Scoring
    let detailed_components =
        compute_detailed_components(&saju, &vedic, transit.as_ref(), &transit_res);
    let destiny_raw_score = detailed_components
        .iter()
        .map(|c| c.score * c.weight)
        .sum::<f32>();

    // Normalization & Tiering
    let destiny_tier_score = spread_normalize(destiny_raw_score);
    let destiny_tier = get_tier_from_score(destiny_tier_score);

    // Legacy / Compatibility fields
    let mut strengths = Vec::new();
    strengths.extend(saju_res.highlights.iter().take(2).cloned());
    strengths.extend(vedic_res.highlights.iter().take(2).cloned());
    strengths.extend(
        transit_res
            .highlights
            .iter()
            .filter(|h| !h.contains("주의") && !h.contains("부하"))
            .take(1)
            .cloned(),
    );

    let mut weaknesses = Vec::new();
    if let Some(vr) = &saju.vulnerability_report {
        if vr.total_crashes > 30 {
            weaknesses.push(format!("주의 시점 {}개", vr.total_crashes));
        }
    }
    if matches!(vedic.report.sade_sati, SadeSatiPhase::Peak) {
        weaknesses.push("사데사티 절정".to_string());
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
            1 => "자아·건강",
            2 => "재물",
            3 => "형제·용기",
            4 => "가정·학업",
            5 => "자녀·창작",
            6 => "건강·노동",
            7 => "관계·결혼",
            8 => "변화·유산",
            9 => "학문·행운",
            10 => "직업·명예",
            11 => "소원·수입",
            12 => "영성·은둔",
            _ => "기타",
        };
        domain_tiers.push(DomainTier {
            house: h.house,
            domain: domain_name.to_string(),
            tier: tier.to_string(),
        });
    }

    let potential_tier_score = spread_normalize(pot_res.score);
    let potential_tier = get_tier_from_score(potential_tier_score);

    TierResult {
        natal_score: clamp_score(destiny_tier_score),
        current_score: transit_res.score,
        destiny_score: destiny_tier_score,
        destiny_tier: destiny_tier.clone(),
        potential_score: potential_tier_score,
        potential_tier,
        domain_tiers,
        saju_result: saju_res.clone(),
        vedic_result: vedic_res.clone(),
        transit_result: transit_res.clone(),
        strengths,
        weaknesses,
        growth_gap: (potential_tier_score - destiny_tier_score).round(),
        risk_level: compute_risk_level(&saju, &vedic, &transit_res),
        profile,
        version: "v3_spread_model".to_string(),
        destiny_raw_score,
        destiny_tier_score,
        detailed_components,
        tier_model_version: "3.0.0".to_string(),
    }
}

fn compute_detailed_components(
    saju: &SajuAnalysisOutput,
    vedic: &VedicAnalysisOutput,
    _transit_opt: Option<&TransitAnalysisOutput>,
    transit_res: &ScoreResult,
) -> Vec<DestinyComponent> {
    let mut components = Vec::new();

    // 1. Saju Core Strength (Weight: 0.12)
    let saju_strength_score = saju.report.strength.strength_score.abs().min(50.0) * 2.0; // Normalized 0-100
    components.push(DestinyComponent {
        key: "saju_strength".to_string(),
        label: "사주 원국 강점".to_string(),
        score: saju_strength_score,
        weight: 0.12,
        reasons: vec![format!(
            "신강약 지수: {:.1}",
            saju.report.strength.strength_score
        )],
    });

    // 2. Element Flow (Weight: 0.08)
    components.push(DestinyComponent {
        key: "element_flow".to_string(),
        label: "오행 유통성".to_string(),
        score: saju.qi_topology.throughput * 100.0,
        weight: 0.08,
        reasons: vec![format!(
            "오행 흐름 효율: {:.1}%",
            saju.qi_topology.throughput * 100.0
        )],
    });

    // 3. Structure & Yongshin (Weight: 0.10)
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
        label: "격국 및 용신".to_string(),
        score: structure_score,
        weight: 0.10,
        reasons: vec![format!("격국: {:?}", saju.report.structure.structure)],
    });

    // 4. Spirit Markers (Weight: 0.06)
    let spirit_score = (saju.report.spirit_markers.auspicious.len() as f32 * 20.0).min(100.0)
        - (saju.report.spirit_markers.inauspicious.len() as f32 * 10.0).max(0.0);
    components.push(DestinyComponent {
        key: "spirit_markers".to_string(),
        label: "길흉신 분포".to_string(),
        score: spirit_score.clamp(0.0, 100.0),
        weight: 0.06,
        reasons: vec![format!(
            "길신 {}개, 흉신 {}개",
            saju.report.spirit_markers.auspicious.len(),
            saju.report.spirit_markers.inauspicious.len()
        )],
    });

    // 5. Vedic House Score (Weight: 0.12)
    let house_score = vedic.report.overall_strength_score as f32 / 6.0; // Already roughly 0-100 scale? (max is 600/6=100)
    components.push(DestinyComponent {
        key: "vedic_houses".to_string(),
        label: "베딕 하우스 역량".to_string(),
        score: house_score.clamp(0.0, 100.0),
        weight: 0.12,
        reasons: vec![format!("전체 하우스 평균 강도: {:.1}", house_score)],
    });

    // 6. Vedic Yogas (Weight: 0.10)
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
        label: "행성 조합(요가)".to_string(),
        score: yoga_score,
        weight: 0.10,
        reasons: vec![format!(
            "최상급 요가 {}개 감지",
            vedic
                .report
                .yogas
                .iter()
                .filter(|y| matches!(y.quality, YogaQuality::VeryHigh))
                .count()
        )],
    });

    // 7. Planet Status (Avasthas) (Weight: 0.07)
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
        label: "행성 활성도".to_string(),
        score: avastha_score.clamp(0.0, 100.0),
        weight: 0.07,
        reasons: vec![format!(
            "활성 상태(Bala/Yuva) 행성 {}개",
            vedic
                .chart
                .avasthas
                .iter()
                .filter(|a| matches!(a.baladi, BaladiAvastha::Bala | BaladiAvastha::Yuva))
                .count()
        )],
    });

    // 8. Luck Cycle (Weight: 0.12)
    let luck_score = transit_res.score;
    components.push(DestinyComponent {
        key: "luck_cycle".to_string(),
        label: "현재 운 흐름".to_string(),
        score: luck_score,
        weight: 0.12,
        reasons: vec![format!("트랜짓 종합 점수: {:.1}", luck_score)],
    });

    // 9. Stability & Risk (Weight: 0.08)
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
        label: "인생 안정성".to_string(),
        score: risk_inv,
        weight: 0.08,
        reasons: vec![format!(
            "취약점(Crash) 지수: {}",
            saju.vulnerability_report
                .as_ref()
                .map(|v| v.total_crashes)
                .unwrap_or(0)
        )],
    });

    // 10. Potential (Weight: 0.05)
    components.push(DestinyComponent {
        key: "potential".to_string(),
        label: "발전 가능성".to_string(),
        score: saju.entropy.score.min(2.0) * 50.0, // Entropy as potential
        weight: 0.05,
        reasons: vec![format!("운명 엔트로피: {:.2}", saju.entropy.score)],
    });

    // 11. Golden Time (Weight: 0.05)
    let golden_score = saju
        .report
        .golden_time
        .as_ref()
        .map(|gt| gt.average_score)
        .unwrap_or(50.0);
    components.push(DestinyComponent {
        key: "golden_time".to_string(),
        label: "인생 골든타임".to_string(),
        score: golden_score,
        weight: 0.05,
        reasons: vec![saju
            .report
            .golden_time
            .as_ref()
            .map(|gt| format!("최상의 시기: {}~{}세", gt.start_age, gt.end_age))
            .unwrap_or("골든타임 미정".to_string())],
    });

    // 12. Holistic Balance (Weight: 0.05)
    let balance_score = 100.0 - (saju.lints.len() as f32 * 5.0).min(50.0);
    components.push(DestinyComponent {
        key: "holistic_balance".to_string(),
        label: "종합적 조화".to_string(),
        score: balance_score,
        weight: 0.05,
        reasons: vec![format!("사주 구조 오류/린트 {}건", saju.lints.len())],
    });

    components
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
            label: "천기".to_string(),
            desc: "하늘의 기틀이 잡힌 극귀(極貴)의 운명".to_string(),
        }
    } else if s >= 90 {
        TierGrade {
            grade: "S".to_string(),
            label: "천운".to_string(),
            desc: "하늘이 돕고 땅이 비추는 대귀(大貴)의 운명".to_string(),
        }
    } else if s >= 83 {
        TierGrade {
            grade: "A+".to_string(),
            label: "대길상".to_string(),
            desc: "복이 넘치며 만인이 부러워할 기세의 운명".to_string(),
        }
    } else if s >= 75 {
        TierGrade {
            grade: "A".to_string(),
            label: "대길".to_string(),
            desc: "크게 길하며 성취가 확실한 운명".to_string(),
        }
    } else if s >= 67 {
        TierGrade {
            grade: "B+".to_string(),
            label: "길상".to_string(),
            desc: "순한 기세 속에서 재능을 펼치는 운명".to_string(),
        }
    } else if s >= 59 {
        TierGrade {
            grade: "B".to_string(),
            label: "길".to_string(),
            desc: "안정적이고 무난하게 번영할 운명".to_string(),
        }
    } else if s >= 51 {
        TierGrade {
            grade: "C+".to_string(),
            label: "중상".to_string(),
            desc: "보통 이상의 저력이 있으며 노력이 빛을 발함".to_string(),
        }
    } else if s >= 43 {
        TierGrade {
            grade: "C".to_string(),
            label: "중평".to_string(),
            desc: "굴곡이 있으나 능히 헤쳐 나갈 수 있는 운명".to_string(),
        }
    } else if s >= 35 {
        TierGrade {
            grade: "D+".to_string(),
            label: "중하".to_string(),
            desc: "고비가 잦으나 인내로써 길을 열어야 함".to_string(),
        }
    } else if s >= 27 {
        TierGrade {
            grade: "D".to_string(),
            label: "하평".to_string(),
            desc: "많은 주의를 요하며 신중한 처세가 필요한 시기".to_string(),
        }
    } else if s >= 18 {
        TierGrade {
            grade: "E".to_string(),
            label: "하하".to_string(),
            desc: "크나큰 인고와 역경 뒤에 겨우 싹을 틔울 운명".to_string(),
        }
    } else {
        TierGrade {
            grade: "F".to_string(),
            label: "난국".to_string(),
            desc: "길이 험난하니 수양과 지혜로 화를 피해야 함".to_string(),
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
        highlights.push("삼득(득령·득지·득시) 완성".to_string());
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
    let highlights = Vec::new();

    let r = &vedic.report;
    score += clamp_score(r.overall_strength_score as f32 / 6.0) * 0.35;

    let vh_yogas = r
        .yogas
        .iter()
        .filter(|y| matches!(y.quality, YogaQuality::VeryHigh))
        .count() as f32;
    score += 12.0_f32.min(vh_yogas * 4.0);

    let ex_houses = r
        .house_summary
        .iter()
        .filter(|h| h.rating == "Excellent")
        .count() as f32;
    score += 10.0_f32.min(ex_houses * 2.0);

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
