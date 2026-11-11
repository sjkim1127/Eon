use eon_service::dto::*;

fn zodiac_sign_name(idx: usize) -> &'static str {
    match idx % 12 {
        0 => "Aries (양자리)",
        1 => "Taurus (황소자리)",
        2 => "Gemini (쌍둥이자리)",
        3 => "Cancer (게자리)",
        4 => "Leo (사자자리)",
        5 => "Virgo (처녀자리)",
        6 => "Libra (천칭자리)",
        7 => "Scorpio (전갈자리)",
        8 => "Sagittarius (사수자리)",
        9 => "Capricorn (염소자리)",
        10 => "Aquarius (물병자리)",
        11 => "Pisces (물고기자리)",
        _ => "Unknown",
    }
}

#[allow(clippy::too_many_arguments)]
pub fn format_agent_context(
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
    let mut out = String::new();

    out.push_str("# 🔮 [DESTINY INTELLIGENCE PROFILE FOR AI AGENT]\n\n");
    out.push_str(&format!(
        "> **생년월일시**: {:04}-{:02}-{:02} {:02}:{:02} ({}, Lat: {:.4}, Lon: {:.4})\n\n",
        input.year,
        input.month,
        input.day,
        input.hour,
        input.minute,
        input.timezone,
        input.lat,
        input.lon
    ));

    // 1. Destiny Tier 7.0 (종합 운명 티어 & 퀀텀 시너지)
    if let Some(t) = tier {
        out.push_str("## 🌌 1. 종합 운명 티어 (Omni Destiny Tier 7.0)\n");
        out.push_str(&format!(
            "- **종합 등급**: **{}** (선천 원국: {:.1}pt / 종합 점수: {:.1}pt / 잠재력: **{}** {:.1}pt)\n",
            t.destiny_tier.grade, t.destiny_raw_score, t.destiny_tier_score, t.potential_tier.grade, t.potential_score
        ));
        out.push_str(&format!(
            "- **프로필 유형**: {} | **성장 갭**: +{:.1}pt\n",
            t.profile, t.growth_gap
        ));

        if !t.quantum_synergies.is_empty() {
            out.push_str("- **⚛️ 8D 퀀텀 크로스 시너지 (Cross-Engine Synergies)**:\n");
            for s in &t.quantum_synergies {
                let sign = if s.score_delta >= 0.0 { "+" } else { "" };
                out.push_str(&format!(
                    "  * [{}] {}{:.0}pt — {}\n",
                    s.title, sign, s.score_delta, s.description
                ));
            }
        }

        if !t.strengths.is_empty() {
            out.push_str(&format!("- **핵심 강점**: {}\n", t.strengths.join(", ")));
        }
        if !t.weaknesses.is_empty() {
            out.push_str(&format!("- **주의/취약점**: {}\n", t.weaknesses.join(", ")));
        }
        out.push('\n');
    }

    // 2. 사주 명리학 (Saju)
    if let Some(s) = saju {
        out.push_str("## 📜 2. 사주 명리학 (Four Pillars of Destiny)\n");
        let pillars = &s.report.pillars;
        out.push_str(&format!(
            "- **사주 팔자**: {}({}{})년  {}({}{})월  {}({}{})일  {}({}{})시\n",
            pillars.year,
            pillars.year.stem.hangul(),
            pillars.year.branch.hangul(),
            pillars.month,
            pillars.month.stem.hangul(),
            pillars.month.branch.hangul(),
            pillars.day,
            pillars.day.stem.hangul(),
            pillars.day.branch.hangul(),
            pillars.hour,
            pillars.hour.stem.hangul(),
            pillars.hour.branch.hangul()
        ));
        out.push_str(&format!(
            "- **신강약**: {:?} (신강 지수: {:.1})\n",
            s.report.strength.strength_type, s.report.strength.strength_score
        ));
        out.push_str(&format!(
            "- **격국 및 용신**: 격국: {:?}, 주용신: {:?}\n",
            s.report.structure.structure, s.report.yongshin.primary
        ));
        out.push_str(&format!(
            "- **오행 유통성 (Throughput)**: {:.1}%\n",
            s.qi_topology.throughput * 100.0
        ));
        if let Some(gt) = &s.report.golden_time {
            out.push_str(&format!(
                "- **인생 골든타임**: {}세 ~ {}세 (평균 점수: {:.1}pt)\n",
                gt.start_age, gt.end_age, gt.average_score
            ));
        }
        out.push('\n');
    }

    // 3. 베딕 점성학 (Vedic Astrology)
    if let Some(v) = vedic {
        out.push_str("## 🕉️ 3. 베딕 점성학 (Vedic Astrology / Jyotish)\n");
        out.push_str(&format!("- **Lagna (상승궁)**: {:?}\n", v.chart.ascendant));
        out.push_str(&format!(
            "- **달 Nakshatra 정보**: {}\n",
            v.report.nakshatra_info
        ));
        out.push_str(&format!(
            "- **사데 사티 (Sade Sati)**: {:?}\n",
            v.report.sade_sati
        ));

        let top_yogas: Vec<_> = v.report.yogas.iter().take(4).collect();
        if !top_yogas.is_empty() {
            out.push_str("- **핵심 요가 (Key Yogas)**:\n");
            for y in top_yogas {
                out.push_str(&format!(
                    "  * **{}** ({:?}): {}\n",
                    y.name, y.quality, y.description
                ));
            }
        }
        out.push('\n');
    }

    // 4. 서양 점성학 (Western Astrology)
    if let Some(w) = western {
        out.push_str("## ♈ 4. 서양 점성학 (Western Astrology)\n");
        let sun = w.result.planets.iter().find(|p| p.name == "Sun");
        let moon = w.result.planets.iter().find(|p| p.name == "Moon");
        let asc = w.result.houses.first();

        if let (Some(s), Some(m), Some(a)) = (sun, moon, asc) {
            out.push_str(&format!(
                "- **Big 3**: 태양({:?}, H{}), 달({:?}, H{}), 상승궁({:?})\n",
                zodiac_sign_name(s.sign_index),
                s.house_number,
                zodiac_sign_name(m.sign_index),
                m.house_number,
                zodiac_sign_name(a.sign_index)
            ));
        }
        if !w.result.aspect_patterns.is_empty() {
            let patterns: Vec<String> = w
                .result
                .aspect_patterns
                .iter()
                .map(|p| format!("{:?}", p.pattern_type))
                .collect();
            out.push_str(&format!(
                "- **아스펙트 복합 패턴**: {}\n",
                patterns.join(", ")
            ));
        }
        out.push('\n');
    }

    // 5. 자미두수 (Zi Wei Dou Shu)
    if let Some(z) = zwds {
        out.push_str("## 🔮 5. 자미두수 (Zi Wei Dou Shu)\n");
        let soul_palace = &z.chart.palaces[z.chart.soul_idx];
        let stars: Vec<String> = soul_palace
            .stars
            .iter()
            .map(|s| s.star.korean().to_string())
            .collect();
        out.push_str(&format!(
            "- **명궁 (Life Palace)**: {} (주요 별: {})\n",
            soul_palace.name.korean(),
            if stars.is_empty() {
                "공궁 (명궁 무주성)".to_string()
            } else {
                stars.join(", ")
            }
        ));
        if !z.chart.destiny_patterns.is_empty() {
            let pats: Vec<String> = z
                .chart
                .destiny_patterns
                .iter()
                .map(|p| p.name_korean.clone())
                .collect();
            out.push_str(&format!("- **성반 주요 격국**: {}\n", pats.join(", ")));
        }
        out.push('\n');
    }

    // 6. 휴먼 디자인 (Human Design)
    if let Some(h) = hd {
        out.push_str("## 🧬 6. 휴먼 디자인 (Human Design)\n");
        out.push_str(&format!(
            "- **Type**: {} | **Profile**: {} | **Strategy**: {}\n",
            h.result.chart_type, h.result.profile, h.result.strategy
        ));
        out.push_str(&format!(
            "- **정의 센터 (Defined Centers)**: {}개 ({:?})\n",
            h.result.defined_centers.len(),
            h.result.defined_centers
        ));
        out.push_str(&format!(
            "- **활성 채널 (Active Channels)**: {}개\n",
            h.result.active_channels.len()
        ));
        out.push('\n');
    }

    // 7. 기문둔갑 (QiMen DunJia)
    if let Some(q) = qimen {
        out.push_str("## 🧭 7. 기문둔갑 (QiMen DunJia)\n");
        out.push_str(&format!(
            "- **국수 (Ju)**: {} {}국\n",
            if q.report.pan.is_yin_ju {
                "음둔"
            } else {
                "양둔"
            },
            q.report.pan.ju_number
        ));
        if let Some(chief) = &q.report.pan.value_chief_star {
            out.push_str(&format!("- **직부성 (Chief Star)**: {:?}\n", chief));
        }
        if let Some(envoy) = &q.report.pan.value_envoy_door {
            out.push_str(&format!("- **직사문 (Envoy Door)**: {:?}\n", envoy));
        }
        out.push('\n');
    }

    // 8. 피타고라스 수비학 (Pythagorean Numerology)
    if let Some(n) = num {
        out.push_str("## 🔢 8. 피타고라스 수비학 (Numerology)\n");
        let core = &n.result.core;
        out.push_str(&format!(
            "- **생애여정수 (Life Path)**: **{}**{}\n",
            core.life_path,
            if n.result.is_master_life_path {
                " (🌟 Master Number)"
            } else {
                ""
            }
        ));
        out.push_str(&format!(
            "- **5대 코어 지표**: 표현수: {}, 영혼수: {}, 인격수: {}, 생일수: {}\n",
            core.expression, core.soul_urge, core.personality, core.birthday
        ));
        out.push_str(&format!(
            "- **개인 연운수 (Personal Year)**: {}년 주기 진동\n",
            n.result.personal_year
        ));
        out.push('\n');
    }

    out.push_str("---\n");
    out.push_str("*💡 Instructions for AI Agent: 위 8대 엔진의 교차 분석 데이터를 기반으로 내담자의 운명 구조, 강점, 대운 흐름, 시너지 및 최적의 실천 전략을 명확하고 지혜롭게 통찰하십시오.*\n");

    out
}
