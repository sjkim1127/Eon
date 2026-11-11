use eon_service::dto::*;

#[allow(clippy::too_many_arguments)]
pub fn format_markdown_report(
    input: &AnalysisInput,
    saju: Option<&SajuAnalysisOutput>,
    vedic: Option<&VedicAnalysisOutput>,
    western: Option<&WesternAnalysisOutput>,
    zwds: Option<&ZwdsAnalysisOutput>,
    hd: Option<&HumanDesignAnalysisOutput>,
    qimen: Option<&QimenAnalysisOutput>,
    num: Option<&NumerologyAnalysisOutput>,
    tier: Option<&TierResult>,
) -> String {
    let mut md = String::new();

    md.push_str("# 🌌 Eon: 8대 운명 체계 종합 리포트 (Full 8-Engine Report)\n\n");
    md.push_str(&format!(
        "**분석 일시**: {:04}-{:02}-{:02} {:02}:{:02} | **타임존**: {} | **좌표**: ({:.4}, {:.4})\n\n",
        input.year, input.month, input.day, input.hour, input.minute, input.timezone, input.lat, input.lon
    ));

    if let Some(t) = tier {
        md.push_str("## 👑 Destiny Tier 7.0 요약\n");
        md.push_str("| 항목 | 점수 / 등급 |\n|---|---|\n");
        md.push_str(&format!(
            "| **종합 운명 티어** | **{}** ({:.1}점) |\n",
            t.destiny_tier.grade, t.destiny_tier_score
        ));
        md.push_str(&format!(
            "| **선천 원국 잠재력** | **{}** ({:.1}점) |\n",
            t.potential_tier.grade, t.potential_score
        ));
        md.push_str(&format!(
            "| **현재 트랜짓 운세** | {:.1}점 |\n",
            t.current_score
        ));
        md.push_str(&format!("| **인생 성장 갭** | +{:.1}점 |\n", t.growth_gap));
        md.push_str(&format!("| **안정성/리스크** | {} |\n\n", t.risk_level));

        if !t.detailed_components.is_empty() {
            md.push_str("### 15대 세부 구성요소 가중치\n");
            md.push_str("| 구성 요소 | 가중치 | 점수 | 주요 분석 요인 |\n|---|---|---|---|\n");
            for c in &t.detailed_components {
                let reason = c.reasons.first().cloned().unwrap_or_default();
                md.push_str(&format!(
                    "| {} | {:.0}% | {:.1} | {} |\n",
                    c.label,
                    c.weight * 100.0,
                    c.score,
                    reason
                ));
            }
            md.push('\n');
        }

        if !t.quantum_synergies.is_empty() {
            md.push_str("### ⚛️ 8D 퀀텀 크로스 시너지\n");
            for s in &t.quantum_synergies {
                let badge = if s.score_delta >= 0.0 { "🟢" } else { "🔴" };
                md.push_str(&format!(
                    "- {} **{}** ({:+.0}pt): {}\n",
                    badge, s.title, s.score_delta, s.description
                ));
            }
            md.push('\n');
        }
    }

    if let Some(s) = saju {
        md.push_str("## 📜 1. 사주 명리학 (Saju)\n");
        let p = &s.report.pillars;
        md.push_str(&format!(
            "- **원국**: 년주: {}({}{}) / 월주: {}({}{}) / 일주: {}({}{}) / 시주: {}({}{})\n",
            p.year,
            p.year.stem.hangul(),
            p.year.branch.hangul(),
            p.month,
            p.month.stem.hangul(),
            p.month.branch.hangul(),
            p.day,
            p.day.stem.hangul(),
            p.day.branch.hangul(),
            p.hour,
            p.hour.stem.hangul(),
            p.hour.branch.hangul()
        ));
        md.push_str(&format!(
            "- **신강약**: {:?} (점수: {:.1})\n",
            s.report.strength.strength_type, s.report.strength.strength_score
        ));
        md.push_str(&format!(
            "- **격국**: {:?} | **주용신**: {:?}\n\n",
            s.report.structure.structure, s.report.yongshin.primary
        ));
    }

    if let Some(v) = vedic {
        md.push_str("## 🕉️ 2. 베딕 점성학 (Vedic Astrology)\n");
        md.push_str(&format!(
            "- **상승궁 (Ascendant)**: {:?}\n",
            v.chart.ascendant
        ));
        md.push_str(&format!(
            "- **달 Nakshatra 정보**: {}\n",
            v.report.nakshatra_info
        ));
        md.push_str(&format!(
            "- **감지된 주요 요가 수**: {}개\n\n",
            v.report.yogas.len()
        ));
    }

    if let Some(w) = western {
        md.push_str("## ♈ 3. 서양 점성학 (Western Astrology)\n");
        md.push_str(&format!(
            "- **행성 수**: {}개 | **아스펙트 수**: {}개\n",
            w.result.planets.len(),
            w.result.aspects.len()
        ));
        md.push_str(&format!(
            "- **감지된 아스펙트 패턴**: {}개\n\n",
            w.result.aspect_patterns.len()
        ));
    }

    if let Some(z) = zwds {
        md.push_str("## 🔮 4. 자미두수 (Zi Wei Dou Shu)\n");
        let soul = &z.chart.palaces[z.chart.soul_idx];
        md.push_str(&format!(
            "- **명궁 (Soul Palace)**: {}\n",
            soul.name.korean()
        ));
        md.push_str(&format!("- **명궁 배치 별**: {}개\n\n", soul.stars.len()));
    }

    if let Some(h) = hd {
        md.push_str("## 🧬 5. 휴먼 디자인 (Human Design)\n");
        md.push_str(&format!(
            "- **차트 타입**: {} | **프로파일**: {}\n",
            h.result.chart_type, h.result.profile
        ));
        md.push_str(&format!(
            "- **정의된 에너지 센터**: {}개\n\n",
            h.result.defined_centers.len()
        ));
    }

    if let Some(q) = qimen {
        md.push_str("## 🧭 6. 기문둔갑 (QiMen DunJia)\n");
        md.push_str(&format!(
            "- **성반**: {} {}국 (9궁 배치 완료)\n\n",
            if q.report.pan.is_yin_ju {
                "음둔"
            } else {
                "양둔"
            },
            q.report.pan.ju_number
        ));
    }

    if let Some(n) = num {
        md.push_str("## 🔢 7. 피타고라스 수비학 (Numerology)\n");
        md.push_str(&format!(
            "- **생애여정수**: **{}**{}\n",
            n.result.core.life_path,
            if n.result.is_master_life_path {
                " (마스터 넘버)"
            } else {
                ""
            }
        ));
        md.push_str(&format!(
            "- **표현수**: {} | **영혼수**: {} | **개인 연운수**: {}\n\n",
            n.result.core.expression, n.result.core.soul_urge, n.result.personal_year
        ));
    }

    md
}
