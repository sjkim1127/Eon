use crate::dto::{DomainTier, SajuAnalysisOutput, ScoreResult, TierGrade, TierResult, TransitAnalysisOutput, VedicAnalysisOutput};
use eon_saju::engine::linter::LintSeverity;
use eon_saju::engine::load_balancer::TrafficStatus;
use eon_vedic::analysis::gochara::SadeSatiPhase;
use eon_vedic::analysis::yogas::YogaQuality;
use eon_vedic::analysis::avasthas::BaladiAvastha;

pub fn analyze(
    saju: SajuAnalysisOutput,
    vedic: VedicAnalysisOutput,
    transit: Option<TransitAnalysisOutput>,
) -> TierResult {
    let saju_res = compute_saju_score(&saju);
    let vedic_res = compute_vedic_score(&vedic);
    let transit_res = transit.as_ref().map(compute_transit_score).unwrap_or(ScoreResult { score: 50.0, highlights: vec![] });
    let pot_res = compute_potential_score(&saju, &vedic);

    let current_age = transit.as_ref().map(|t| t.current_age).unwrap_or(30);

    let mut is_major_luck_transition = false;
    let mut is_saju_golden = false;
    
    if let Some(ml) = &saju.report.major_luck {
        for cycle in &ml.cycles {
            let diff = (cycle.start_age as i32 - current_age as i32).abs();
            if diff <= 1 {
                is_major_luck_transition = true;
                break;
            }
        }
    }
    if let Some(gt) = &saju.report.golden_time {
        if current_age >= gt.start_age && current_age <= gt.end_age {
            is_saju_golden = true;
        }
    }

    let is_vedic_benefic_dasha = {
        let benefics = ["Jupiter", "Venus", "Mercury", "Moon"];
        benefics.iter().any(|&b| vedic.report.dasha_focus.contains(b))
    };

    let has_saju_vuln = saju.vulnerability_report.as_ref().map(|v| v.total_crashes > 20).unwrap_or(false);
    let has_vedic_sati = matches!(vedic.report.sade_sati, SadeSatiPhase::Peak | SadeSatiPhase::Rising);

    let mut strengths = Vec::new();
    strengths.extend(saju_res.highlights.iter().take(2).cloned());
    strengths.extend(vedic_res.highlights.iter().take(2).cloned());
    strengths.extend(transit_res.highlights.iter().filter(|h| !h.contains("주의") && !h.contains("부하")).take(1).cloned());

    let mut weaknesses = Vec::new();
    if let Some(vr) = &saju.vulnerability_report {
        if vr.total_crashes > 30 { weaknesses.push(format!("주의 시점 {}개", vr.total_crashes)); }
    }
    if matches!(vedic.report.sade_sati, SadeSatiPhase::Peak) { weaknesses.push("사데사티 절정".to_string()); }

    let mut cross_bonus = 0.0;
    if has_saju_vuln && has_vedic_sati {
        cross_bonus -= 5.0;
        weaknesses.push("사주/베딕 복합 위기 (주의 요망)".to_string());
    }
    if is_saju_golden && is_vedic_benefic_dasha {
        cross_bonus += 5.0;
        strengths.push("사주/베딕 복합 길운 (시너지 폭발)".to_string());
    }

    let natal_raw = saju_res.score * 0.5 + vedic_res.score * 0.5;
    let mut natal_w = 0.7;
    let mut curr_w = 0.3;
    let mut profile = "balanced".to_string();

    if is_major_luck_transition {
        natal_w = 0.5;
        curr_w = 0.5;
        profile = "transition".to_string();
    } else if current_age < 35 { 
        natal_w = 0.6; curr_w = 0.4; profile = "growth".to_string(); 
    } else if current_age > 55 { 
        natal_w = 0.8; curr_w = 0.2; profile = "stable".to_string(); 
    }

    let has_transit = transit.is_some();
    let mut curr_destiny = transit_res.score;
    if has_transit { curr_destiny = curr_destiny.clamp(25.0, 90.0); }

    let natal_norm = soft_normalize(natal_raw);
    let curr_norm = if has_transit { soft_normalize(curr_destiny) } else { 0.0 };

    let mut destiny_score = if has_transit {
        natal_norm * natal_w + curr_norm * curr_w
    } else {
        natal_norm
    };

    let mut domain_tiers = Vec::new();
    let mut domain_adj: f32 = 0.0;
    let mut penalty_focus: f32 = 0.0;

    for h in &vedic.report.house_summary {
        let tier = match h.rating.as_str() {
            "Excellent" => "S",
            "Strong" => "A",
            "Average" => "B",
            "Weak" => "C",
            _ => "D",
        };
        
        let domain_name = match h.house {
            1 => "자아·건강", 2 => "재물", 3 => "형제·용기", 4 => "가정·학업",
            5 => "자녀·창작", 6 => "건강·노동", 7 => "관계·결혼", 8 => "변화·유산",
            9 => "학문·행운", 10 => "직업·명예", 11 => "소원·수입", 12 => "영성·은둔",
            _ => "기타"
        };

        domain_tiers.push(DomainTier { house: h.house, domain: domain_name.to_string(), tier: tier.to_string() });

        if tier == "S" { domain_adj += 0.5; }
        else if tier == "A" { domain_adj += 0.2; }
        else if tier == "C" { domain_adj -= 0.5; }
        else if tier == "D" { domain_adj -= 1.0; }

        if [1, 2, 6, 10, 11].contains(&h.house) {
            if tier == "C" { penalty_focus -= 0.5; }
            else if tier == "D" { penalty_focus -= 1.0; }
        }
    }

    domain_adj = (domain_adj + penalty_focus).clamp(-4.0, 5.0);
    destiny_score += domain_adj;
    destiny_score += cross_bonus;
    destiny_score = clamp_score(destiny_score);

    let mut risk_points = 0;
    if let Some(vr) = &saju.vulnerability_report {
        if vr.total_crashes >= 10 { risk_points += 1; }
        if vr.total_crashes >= 20 { risk_points += 1; }
        if vr.total_crashes >= 40 { risk_points += 1; }
    }
    if transit_res.score < 40.0 { risk_points += 1; }
    if transit_res.score < 30.0 { risk_points += 1; }
    if matches!(vedic.report.sade_sati, SadeSatiPhase::Peak | SadeSatiPhase::Rising) { risk_points += 2; }

    let risk_level = if risk_points <= 1 { "low" } else if risk_points <= 3 { "medium" } else { "high" };

    TierResult {
        natal_score: clamp_score(natal_raw.round()),
        current_score: transit_res.score,
        destiny_score: destiny_score.round(),
        destiny_tier: get_tier_from_score(destiny_score),
        potential_score: pot_res.score.round(),
        potential_tier: get_tier_from_score(pot_res.score),
        domain_tiers,
        saju_result: saju_res,
        vedic_result: vedic_res,
        transit_result: transit_res,
        strengths,
        weaknesses,
        growth_gap: (pot_res.score - destiny_score).round(),
        risk_level: risk_level.to_string(),
        profile,
        version: "v2_rust_service".to_string(),
    }
}

fn clamp_score(score: f32) -> f32 {
    score.clamp(0.0, 100.0)
}

fn soft_normalize(score: f32) -> f32 {
    let clamped = clamp_score(score);
    if clamped <= 50.0 { return clamped; }
    let excess = (clamped - 50.0) / 50.0;
    let compressed = excess.powf(1.4) * 35.0;
    (50.0 + compressed).round()
}

fn get_tier_from_score(score: f32) -> TierGrade {
    let s = score.round() as i32;
    if s >= 93 { TierGrade { grade: "S+".to_string(), label: "천기".to_string(), desc: "사주와 별운이 완전히 일치하는 극희귀 최상의 조합".to_string() } }
    else if s >= 85 { TierGrade { grade: "S".to_string(), label: "천운".to_string(), desc: "사주와 별운이 모두 유리한 극상의 조합".to_string() } }
    else if s >= 77 { TierGrade { grade: "A+".to_string(), label: "대길상".to_string(), desc: "용신·대운·요가가 거의 완벽하게 지원하는 강운".to_string() } }
    else if s >= 69 { TierGrade { grade: "A".to_string(), label: "대길".to_string(), desc: "전반적으로 아주 강한 기운의 조합".to_string() } }
    else if s >= 61 { TierGrade { grade: "B+".to_string(), label: "길상".to_string(), desc: "균형이 잡히고 강점이 뚜렷하게 빛나는 운세".to_string() } }
    else if s >= 53 { TierGrade { grade: "B".to_string(), label: "길".to_string(), desc: "전반적으로 안정적이고 활용 가능한 운세".to_string() } }
    else if s >= 45 { TierGrade { grade: "C+".to_string(), label: "중상".to_string(), desc: "보통 이상의 기운, 노력으로 충분히 도약 가능".to_string() } }
    else if s >= 37 { TierGrade { grade: "C".to_string(), label: "중평".to_string(), desc: "일부 어려움이 있으나 극복 가능한 조합".to_string() } }
    else if s >= 29 { TierGrade { grade: "D+".to_string(), label: "성장예비".to_string(), desc: "성장 여지가 많으며 조건이 갖춰지면 빠른 상향 가능".to_string() } }
    else { TierGrade { grade: "D".to_string(), label: "다다익선".to_string(), desc: "성장 여지가 많은 시기, 주의 시점 활용 권장".to_string() } }
}

fn compute_saju_score(saju: &SajuAnalysisOutput) -> ScoreResult {
    let mut score = 0.0;
    let mut highlights = Vec::new();

    let r = &saju.report;
    let st = &r.strength;
    
    let strength_norm = clamp_score(st.strength_score * 2.0);
    score += strength_norm * 0.28;

    let acquired = vec![st.deuk_ryeong.acquired, st.deuk_ji.acquired, st.deuk_si.acquired]
        .into_iter().filter(|&x| x).count() as f32;
    score += acquired * 4.0;
    if acquired == 3.0 { highlights.push("삼득(득령·득지·득시) 완성".to_string()); }
    else if acquired >= 2.0 { highlights.push(format!("득령·득지·득시 {}개 달성", acquired)); }

    let support_pct = if st.deuk_se.support_ratio > 1.0 { st.deuk_se.support_ratio } else { st.deuk_se.support_ratio * 100.0 };
    score += (support_pct / 100.0) * 10.0;
    if support_pct > 60.0 { highlights.push(format!("득세 지지비율 {:.0}% (우세)", support_pct)); }

    let topo = &saju.qi_topology;
    score += topo.throughput * 12.0;
    if topo.throughput > 0.75 { highlights.push(format!("오행 흐름 {:.0}% (원활)", topo.throughput * 100.0)); }
    if topo.bottleneck.is_some() { score -= 2.0; }

    let struc = &r.structure;
    let name = format!("{:?}", struc.structure);
    if name.contains("Follower") { score -= 3.0; }
    else { score += 2.0; }

    let sm = &r.spirit_markers;
    let aus = sm.auspicious.len() as f32;
    let inaus = sm.inauspicious.len() as f32;
    score += 5.0_f32.min(aus * 1.0);
    score -= 4.0_f32.min(inaus * 0.8);
    if aus > 0.0 { highlights.push(format!("길신 {}개", aus)); }
    if inaus >= 3.0 { highlights.push(format!("흉신 {}개 주의", inaus)); }

    if let Some(gt) = &r.golden_time {
        let len = (gt.end_age - gt.start_age) as f32;
        score += 6.0_f32.min(len * 0.4);
        highlights.push(format!("골든타임 {}~{}세 ({}년)", gt.start_age, gt.end_age, len));
    }

    if !r.simulation_frames.is_empty() {
        let avg: f32 = r.simulation_frames.iter().map(|f| f.score).sum::<f32>() / r.simulation_frames.len() as f32;
        let good_pct = r.simulation_frames.iter().filter(|f| f.score >= 65.0).count() as f32 / r.simulation_frames.len() as f32;
        score += 8.0_f32.min((avg - 50.0) * 0.16);
        score += good_pct * 5.0;
        if avg >= 70.0 { highlights.push(format!("시뮬레이션 평균 {:.0}점", avg)); }
    }

    if let Some(vr) = &saju.vulnerability_report {
        if vr.total_crashes == 0 {
            score += 4.0;
            highlights.push("충돌 주의 시점 없음".to_string());
        } else if vr.total_crashes > 40 {
            score -= 5.0;
            highlights.push(format!("주의 시점 {}개", vr.total_crashes));
        } else if vr.total_crashes > 20 {
            score -= 2.0;
        }
    }

    let entropy_score = saju.entropy.score;
    let is_high_entropy = entropy_score > 1.5;

    if let Some(cx) = &saju.complexity {
        if cx.stability_grade.starts_with("A") { score += 4.0; highlights.push("안정성 A등급".to_string()); }
        else if cx.stability_grade.starts_with("B") { score += 2.0; }
        else if cx.stability_grade.starts_with("D") { score -= if is_high_entropy { 1.5 } else { 3.0 }; }
    }

    let errs = saju.lints.iter().filter(|l| matches!(l.severity, LintSeverity::Error)).count() as f32;
    let warns = saju.lints.iter().filter(|l| matches!(l.severity, LintSeverity::Warning)).count() as f32;
    if errs == 0.0 && warns == 0.0 {
        score += 2.0; highlights.push("사주 구조 클린".to_string());
    }
    score -= 5.0_f32.min(errs * 1.5 + warns * 0.4);

    if entropy_score < 1.0 { score += 2.0; highlights.push("운명 패턴 안정".to_string()); }
    else if entropy_score > 2.0 { score -= 2.0; highlights.push("운명 변수 많음".to_string()); }

    let ys = &r.yongshin;
    if ys.recommendations.len() >= 2 { score += 2.0; }

    highlights.truncate(6);
    ScoreResult { score: clamp_score(score.round()), highlights }
}

fn compute_vedic_score(vedic: &VedicAnalysisOutput) -> ScoreResult {
    let mut score = 0.0;
    let mut highlights = Vec::new();

    let r = &vedic.report;
    score += clamp_score(r.overall_strength_score as f32 / 6.0) * 0.35;

    let vh_yogas = r.yogas.iter().filter(|y| matches!(y.quality, YogaQuality::VeryHigh)).count() as f32;
    let h_yogas = r.yogas.iter().filter(|y| matches!(y.quality, YogaQuality::High)).count() as f32;
    score += 12.0_f32.min(vh_yogas * 4.0 + h_yogas * 2.0);
    if vh_yogas > 0.0 { highlights.push(format!("최상급 요가 {}개", vh_yogas)); }
    else if h_yogas > 0.0 { highlights.push(format!("우수 요가 {}개", h_yogas)); }

    let ex_houses = r.house_summary.iter().filter(|h| h.rating == "Excellent").count() as f32;
    let st_houses = r.house_summary.iter().filter(|h| h.rating == "Strong").count() as f32;
    let wk_houses = r.house_summary.iter().filter(|h| h.rating == "Weak").count() as f32;
    score += 10.0_f32.min(ex_houses * 2.0 + st_houses * 0.8);
    score -= 6.0_f32.min(wk_houses * 1.2);
    if ex_houses >= 4.0 { highlights.push(format!("최강 하우스 {}개", ex_houses)); }
    else if st_houses + ex_houses >= 6.0 { highlights.push(format!("강한 하우스 {}개", st_houses + ex_houses)); }

    let c = &vedic.chart;
    let mut key_house_bonus = 0.0;
    for h in [1, 5, 9, 10] {
        if let Some(bh) = c.bhava_strengths.iter().find(|b| b.house == h) {
            if bh.total_score >= 60.0 { key_house_bonus += 1.5; }
            else if bh.total_score >= 40.0 { key_house_bonus += 0.5; }
        }
    }
    score += 6.0_f32.min(key_house_bonus);
    if key_house_bonus >= 4.0 { highlights.push("핵심 하우스(1·5·9·10) 강화".to_string()); }

    let strong_sav = c.sav.points.iter().filter(|&&p| p >= 28).count() as f32;
    let weak_sav = c.sav.points.iter().filter(|&&p| p <= 22).count() as f32;
    score += 5.0_f32.min(strong_sav * 0.8);
    score -= 4.0_f32.min(weak_sav * 0.8);
    if strong_sav >= 6.0 { highlights.push(format!("SAV 강점 하우스 {}개", strong_sav)); }

    if !c.vimshopaka_scores.is_empty() {
        let mut sum = 0.0;
        for (_, v) in &c.vimshopaka_scores {
            sum += v.shadvarga_score + v.shodashavarga_score;
        }
        let avg = (sum / 2.0) as f32 / c.vimshopaka_scores.len() as f32;
        score += 6.0_f32.min((avg / 20.0) * 6.0);
        if avg >= 14.0 { highlights.push(format!("빔쇼파카 평균 {:.1}", avg)); }
    }

    let retro = c.planets.iter().filter(|p| p.is_retrograde).count() as f32;
    let combust = c.planets.iter().filter(|p| p.is_combust).count() as f32;
    score -= 4.0_f32.min(retro * 0.8);
    score -= 3.0_f32.min(combust * 0.8);
    if retro + combust >= 3.0 { highlights.push(format!("역행 {}+연소 {}개", retro, combust)); }

    let bala = c.avasthas.iter().filter(|a| matches!(a.baladi, BaladiAvastha::Bala | BaladiAvastha::Yuva)).count() as f32;
    let mrta = c.avasthas.iter().filter(|a| matches!(a.baladi, BaladiAvastha::Mrita | BaladiAvastha::Vriddha)).count() as f32;
    score += 3.0_f32.min(bala * 0.6);
    score -= 3.0_f32.min(mrta * 0.7);
    if bala >= 4.0 { highlights.push(format!("활성 행성 {}개", bala)); }

    match r.sade_sati {
        SadeSatiPhase::None => { score += 5.0; highlights.push("사데사티 비해당".to_string()); },
        SadeSatiPhase::Peak => { score -= 6.0; highlights.push("사데사티 절정".to_string()); },
        SadeSatiPhase::Rising => { score -= 3.0; highlights.push("사데사티 상승".to_string()); },
        SadeSatiPhase::Setting => { score -= 1.0; },
    }

    let benefics = ["Jupiter", "Venus", "Mercury", "Moon"];
    if benefics.iter().any(|&b| r.dasha_focus.contains(b)) {
        score += 4.0;
        highlights.push("길성 다샤".to_string());
    }

    highlights.truncate(6);
    ScoreResult { score: clamp_score(score.round()), highlights }
}

fn compute_transit_score(transit: &TransitAnalysisOutput) -> ScoreResult {
    let mut highlights = Vec::new();
    let mut score = if let Some(f) = &transit.current_frame { clamp_score(f.score) } else { 50.0 };

    if score >= 75.0 { highlights.push("현재 운세 긍정적".to_string()); }
    else if score >= 55.0 { highlights.push("현재 운세 보통".to_string()); }
    else if score < 40.0 { highlights.push("현재 운세 주의".to_string()); }

    let bad = transit.nearby_diagnostics.iter().filter(|d| matches!(d.status, TrafficStatus::Overloaded | TrafficStatus::SystemDown)).count() as f32;
    let down = transit.nearby_diagnostics.iter().filter(|d| matches!(d.status, TrafficStatus::SystemDown)).count() as f32;
    if bad > 0.0 { score -= 15.0_f32.min(bad * 5.0); highlights.push(format!("부하 구간 {}개", bad)); }
    score -= down * 3.0;

    let ts = transit.yearly_luck.twelve_stage.as_ref().map(|s| s.as_str()).unwrap_or("");
    if ["장생", "건록", "제왕", "관대", "목욕"].contains(&ts) { score += 5.0; highlights.push(format!("세운: {} (길)", ts)); }
    else if ["절", "묘", "태"].contains(&ts) { score -= 5.0; highlights.push(format!("세운: {} (흉)", ts)); }

    if !transit.yearly_luck.special_events.is_empty() {
        score += 4.0_f32.min(transit.yearly_luck.special_events.len() as f32 * 1.5);
        highlights.push(format!("길조 이벤트 {}개", transit.yearly_luck.special_events.len()));
    }

    if let Some(inf) = &transit.yearly_luck.influence {
        let rels = &inf.relations_with_natal;
        let good = rels.iter().filter(|r| r.contains("합") || r.contains("록") || r.contains("귀인")).count() as f32;
        let bad = rels.iter().filter(|r| r.contains("충") || r.contains("형") || r.contains("해") || r.contains("파")).count() as f32;
        score += 4.0_f32.min(good * 1.5);
        score -= 4.0_f32.min(bad * 1.5);
    }

    let ts_m = transit.monthly_luck.twelve_stage.as_ref().map(|s| s.as_str()).unwrap_or("");
    if ["장생", "건록", "제왕", "관대", "목욕"].contains(&ts_m) { score += 3.0; }
    else if ["절", "묘", "태"].contains(&ts_m) { score -= 3.0; }

    highlights.truncate(4);
    ScoreResult { score: clamp_score(score.round()), highlights }
}

fn compute_potential_score(saju: &SajuAnalysisOutput, vedic: &VedicAnalysisOutput) -> ScoreResult {
    let mut highlights = Vec::new();
    let mut score = 50.0;

    let r = &saju.report;
    let topo = &saju.qi_topology;
    score += topo.throughput * 12.0;
    if topo.throughput > 0.7 { highlights.push("오행 흐름 원활 (잠재력 발현)".to_string()); }
    
    if let Some(cx) = &saju.complexity {
        if cx.stability_grade.starts_with("A") { score += 8.0; highlights.push("안정성 A등급".to_string()); }
        else if cx.stability_grade.starts_with("B") { score += 4.0; }
        else if cx.stability_grade.starts_with("D") { score -= 4.0; }
    }
    if let Some(gt) = &r.golden_time {
        let len = (gt.end_age - gt.start_age) as f32;
        score += 12.0_f32.min(len * 0.5);
        highlights.push(format!("골든타임 {}년", len));
    }
    let ys = &r.yongshin;
    if ys.recommendations.len() >= 2 { score += 4.0; }
    if let Some(vr) = &saju.vulnerability_report {
        if vr.total_crashes == 0 { score += 5.0; }
        else if vr.total_crashes < 10 { score += 3.0; }
        else if vr.total_crashes > 40 { score -= 5.0; }
    }
    let sm = &r.spirit_markers;
    score += 5.0_f32.min(sm.auspicious.len() as f32 * 1.2);
    
    let errs = saju.lints.iter().filter(|l| matches!(l.severity, LintSeverity::Error)).count() as f32;
    if errs == 0.0 { score += 3.0; } else { score -= errs * 2.0; }
    
    if !r.simulation_frames.is_empty() {
        let good_pct = r.simulation_frames.iter().filter(|f| f.score >= 70.0).count() as f32 / r.simulation_frames.len() as f32;
        score += good_pct * 8.0;
    }

    let high_yogas = vedic.report.yogas.iter().filter(|y| matches!(y.quality, YogaQuality::VeryHigh | YogaQuality::High)).count() as f32;
    score += 10.0_f32.min(high_yogas * 3.0);
    if high_yogas > 0.0 { highlights.push(format!("상급 요가 {}개", high_yogas)); }

    let ex_houses = vedic.report.house_summary.iter().filter(|h| h.rating == "Excellent").count() as f32;
    score += 8.0_f32.min(ex_houses * 2.5);

    if matches!(vedic.report.sade_sati, SadeSatiPhase::None) { score += 3.0; }
    else if matches!(vedic.report.sade_sati, SadeSatiPhase::Peak) { score -= 5.0; }

    highlights.truncate(5);
    ScoreResult { score: clamp_score(score.round()), highlights }
}
