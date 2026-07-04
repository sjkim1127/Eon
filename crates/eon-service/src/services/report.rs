// crates/eon-service/src/services/report.rs
use crate::dto::{ReportTheme, SajuAnalysisOutput, ThemedReportInput, ThemedReportOutput};
use crate::error::ServiceError;
use crate::services::saju;
use eon_saju::core::branch::EarthlyBranch;
use eon_saju::core::element::Element;
use eon_saju::core::stem::HeavenlyStem;

pub fn generate(input: ThemedReportInput) -> Result<ThemedReportOutput, ServiceError> {
    // 1. 사주 분석 수행
    let saju_out = saju::analyze(input.base.clone())?;

    // 2. 테마에 따른 보고서 제목 및 본문 조합
    let (title, content) = match input.theme {
        ReportTheme::WealthAndCareer => generate_wealth_report(&saju_out, &input.user_name),
        ReportTheme::LoveAndMarriage => {
            generate_love_report(&saju_out, &input.user_name, input.base.is_male)
        }
        ReportTheme::HealthAndVitality => generate_health_report(&saju_out, &input.user_name),
    };

    Ok(ThemedReportOutput {
        meta: saju_out.meta,
        theme: input.theme,
        user_name: input.user_name,
        title,
        content,
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// 1. 재물 및 커리어 테마 보고서 생성기
// ─────────────────────────────────────────────────────────────────────────────
fn generate_wealth_report(saju: &SajuAnalysisOutput, name: &str) -> (String, String) {
    let title = format!("💰 {}님의 평생 재물 & 커리어 분석 보고서", name);
    let mut s = String::new();

    let day_stem = saju.report.pillars.day.stem;
    let yong_sin_el = saju.report.yongshin.primary;

    // Section 1: 타고난 재물 그릇과 성향
    s.push_str("### 1. 타고난 재물 성향 & 천부적 기질\n\n");

    let day_stem_kor = HeavenlyStem::HANGUL[day_stem.index() as usize];
    let day_stem_han = HeavenlyStem::HANJA[day_stem.index() as usize];
    let yong_sin_kor = Element::HANGUL[yong_sin_el.index() as usize];

    s.push_str(&format!(
        "당신의 일간은 **{} ({} )** 기운으로 태어났으며, 전체 명식을 다스리고 조화롭게 하는 용신(用神) 오행은 **{}**입니다.\n\n",
        day_stem_kor, day_stem_han, yong_sin_kor
    ));

    let temperament_desc = match yong_sin_el {
        Element::Wood => "당신은 **목(木)의 성장 에너지**를 활용하는 재물 성향을 지녔습니다. 무에서 유를 창출하는 기획력, 교육, 건축, 스타트업 창업, 혹은 아이디어를 구체화하여 특허나 독점 자산을 확보하는 방식으로 재물을 불려 나가는 데 탁월합니다.",
        Element::Fire => "당신은 **불(火)의 확산적 에너지**를 재물 활동에 활용합니다. 화려하고 시선을 끄는 비즈니스, 마케팅, IT 정보기술, 예술 분야에서 빠르게 자금을 유통하거나 트렌디한 재테크를 통해 결실을 맺는 천부적 기질을 가집니다.",
        Element::Earth => "당신은 **흙(土)의 중재 및 축적 에너지**를 재물 활동에 씁니다. 신용과 약속을 가장 중요하게 생각하며 부동산 투자, 중개업, 임대업, 자산 위탁 관리 등 실물 자산을 차근차근 모으는 보수적이고 안전한 방식으로 부를 일굽니다.",
        Element::Metal => "당신은 **금(金)의 결단 및 가치 분별 에너지**를 재물 활동에 투영합니다. 금융, 주식 투자, 컨설팅, 세무, 법률 혹은 고부가가치 기술 제조 분야에서 결단력 있게 행동하며, 불필요한 지출을 통제하고 효율을 극대화하는 투자에 능합니다.",
        Element::Water => "당신은 **물(水)의 유연함과 유통 에너지**를 재물 활동에 사용합니다. 뛰어난 정보력과 지식 자산을 보유하며 무역, 유통, 지적 재산권, 온라인 비즈니스 등 눈에 보이지 않는 자산의 흐름을 파악해 재물을 확보하는 기량이 뛰어납니다.",
    };
    s.push_str(temperament_desc);
    s.push_str("\n\n");

    // Section 2: 커리어 구조 및 주요 병목지점 (Bottlenecks)
    s.push_str("### 2. 커리어 구조와 발전 방향 (Qi Network Bottlenecks)\n\n");
    s.push_str("EON 기(氣) 위상 토폴로지 분석을 바탕으로 파악한 당신의 시스템 안정성과 주요 커리어 병목 지점은 다음과 같습니다.\n\n");

    if let Some(comp) = &saju.complexity {
        s.push_str(&format!(
            "- **순환 복잡도 (Cyclomatic Complexity)**: {:.1} / **안정성 등급**: {:?}\n",
            comp.cyclomatic_complexity, comp.stability_grade
        ));
    }

    let bottleneck_str = if let Some(bn) = &saju.qi_topology.bottleneck {
        Element::HANGUL[bn.index() as usize]
    } else {
        "없음"
    };

    s.push_str(&format!(
        "- **명식 내 기의 병목 지점 (Qi Bottleneck)**: {}\n\n",
        bottleneck_str
    ));

    let bottleneck_advice = if let Some(bn) = &saju.qi_topology.bottleneck {
        let bn_kor = Element::HANGUL[bn.index() as usize];
        format!(
            "당신의 커리어 경로 상에서 **{}** 오행의 흐름이 막히는 병목 현상이 발생하기 쉽습니다. 직장 동료와의 불화나 업무 추진 시 의사결정이 지체될 우려가 있으니, 해당 병목 기운을 보완해 주는 자격 취득이나 동업자 연대를 추천합니다.",
            bn_kor
        )
    } else {
        "당신의 기의 순환은 매우 이상적이며, 막힘 없이 자신의 전문 기술을 직장에서 인정받고 성과로 치환할 수 있는 순탄한 순환 구조를 가집니다.".to_string()
    };
    s.push_str(&bottleneck_advice);
    s.push_str("\n\n");

    // Section 3: 재물 대운의 타이밍
    s.push_str("### 3. 평생 재물 대운의 흐름과 터닝 포인트\n\n");
    if let Some(gt) = &saju.report.golden_time {
        s.push_str(&format!(
            "당신의 인생에서 에너지가 극대화되는 황금기(Golden Time)는 **{}-{}세 대운 주기**로 분석됩니다.\n\n",
            gt.start_age, gt.end_age
        ));
    }

    s.push_str("대운(大運) 흐름에서 재물적 안정성과 주도권을 확보하기에 유리한 주기는 다음과 같습니다.\n\n");
    s.push_str("| 대운 주기 | 기둥 간지 | 주역 괘 | 재물/커리어 길흉 가이드 |\n");
    s.push_str("| --- | --- | --- | --- |\n");

    if let Some(ml) = &saju.report.major_luck {
        for cycle in ml.cycles.iter().take(5) {
            let stem_god = cycle.stem_god.hangul();
            let branch_god = cycle.branch_god.hangul();
            let stem_kor = HeavenlyStem::HANGUL[cycle.ganzi.stem.index() as usize];
            let branch_kor = EarthlyBranch::HANGUL[cycle.ganzi.branch.index() as usize];

            let is_good =
                stem_god.contains("재") || stem_god.contains("관") || branch_god.contains("재");
            let guide = if is_good {
                "재물/커리어 확장기. 신규 투자 및 주도적인 역할 추천."
            } else {
                "내실 축적 및 리스크 관리기. 무리한 확장 자제 및 현금 확보 우선."
            };
            s.push_str(&format!(
                "| {}-{}세 | {}{} | {}-{} | {} |\n",
                cycle.start_age, cycle.end_age, stem_kor, branch_kor, stem_god, branch_god, guide
            ));
        }
    }
    s.push('\n');

    // Section 4: 실천 솔루션
    s.push_str("### 4. 재물운 극대화를 위한 실천 가이드\n\n");
    s.push_str("- **개운(開運) 오행 처방**: 당신의 사주 균형을 잡는 핵심 용신 기운에 맞는 환경(서재 인테리어, 방향, 색상)을 조성하십시오.\n");
    s.push_str("- **귀인의 조력 유무**: 당신의 천을귀인(天乙貴人) 간지에 부합하는 띠를 가진 협력자를 가까이 두면 재물적 위기를 극복하는 데 유리합니다.\n");
    s.push_str("- **리스크 조언**: 투자나 사업을 무리하게 벌이기보다는, 자신의 명식에 적합한 보완 처방을 통해 평생의 자산을 조화롭게 관리하는 지혜가 필요합니다.\n");

    (title, s)
}

// ─────────────────────────────────────────────────────────────────────────────
// 2. 연애 및 결혼 테마 보고서 생성기
// ─────────────────────────────────────────────────────────────────────────────
fn generate_love_report(saju: &SajuAnalysisOutput, name: &str, is_male: bool) -> (String, String) {
    let title = format!("❤️ {}님의 인연 & 애정운 상세 분석 보고서", name);
    let mut s = String::new();

    let day_stem = saju.report.pillars.day.stem;
    let day_branch = saju.report.pillars.day.branch;

    // Section 1: 타고난 연애 기질
    s.push_str("### 1. 타고난 연애 기질 & 이상형 스타일\n\n");

    let day_stem_kor = HeavenlyStem::HANGUL[day_stem.index() as usize];
    let day_stem_han = HeavenlyStem::HANJA[day_stem.index() as usize];
    let day_branch_kor = EarthlyBranch::HANGUL[day_branch.index() as usize];
    let day_branch_han = EarthlyBranch::HANJA[day_branch.index() as usize];

    s.push_str(&format!(
        "당신은 **{} ({} )** 일간과 배우자 궁을 상징하는 **{} ({} )** 일지를 타고났습니다.\n\n",
        day_stem_kor, day_stem_han, day_branch_kor, day_branch_han
    ));

    let spouse_desc = if is_male {
        "남성 명식에서 일지는 배우자의 성향과 보금자리를 의미합니다. 배우자 자리에 의리와 책임감을 상징하는 기운이 서려 있어, 나를 지지해 주고 안정감 있게 가정을 함께 이끌어 나갈 책임감 넘치는 배우자를 선호하는 경향이 짙습니다."
    } else {
        "여성 명식에서 일지는 배우자와의 애정 밀도를 나타냅니다. 본인의 일지에 총명하고 재치 있는 오행이 위치하고 있어, 대화가 잘 통하고 지적 호기심을 충족시켜 줄 수 있는 감성적이고 세련된 배우자 스타일과 궁합이 잘 맞습니다."
    };
    s.push_str(spouse_desc);
    s.push_str("\n\n");

    // Section 2: 애정 전선의 장단점 및 공망 분석
    s.push_str("### 2. 애정 관계에서의 강점과 주의점 (Void & Relationships)\n\n");
    s.push_str("명식 내의 합(合), 충(冲), 그리고 공망(空亡)의 분포를 통해 바라본 연애상의 장단점 분석입니다.\n\n");

    let has_conflict = saju.relationships.mapped_relationships.iter().any(|r| {
        r.relation_type.contains("충")
            || r.relation_type.contains("형")
            || r.relation_type.contains("해")
            || r.relation_type.contains("원진")
    });
    let relationships_desc = if has_conflict {
        "사주 원국 내에 일지(배우자궁)와 관련된 충(冲)이나 원진(怨嗔)의 기운이 일부 관측됩니다. 이는 연애 초기에는 뜨겁게 타오르나 시간이 지나면서 사소한 말다툼이나 오해가 깊어질 우려가 있음을 시사합니다. 상호 존중과 주기적인 대화 시간을 가지는 것이 관계의 안전망이 됩니다."
    } else {
        "일지와 주변 지지 간의 충이나 원진이 없어 비교적 평탄하고 부드러운 소통 흐름을 가집니다. 갈등이 발생해도 서로 타협점을 잘 찾으며 오랜 기간 신뢰를 쌓아가는 데 유리한 원국입니다."
    };
    s.push_str(relationships_desc);
    s.push_str("\n\n");

    // Section 3: 인연이 들어오는 시기 (Twelve Stages)
    s.push_str("### 3. 인연을 만나는 타이밍 & 결혼 적기\n\n");
    s.push_str("향후 5년간 애정의 활력이 가장 크게 상승하는 대운/세운 흐름을 기반으로 분석한 인연 유입 타이밍입니다.\n\n");
    s.push_str("| 연도 | 세운 간지 | 십이운성 활력도 | 연애/인연운 종합 추천도 |\n");
    s.push_str("| --- | --- | --- | --- |\n");

    // 예시용 향후 5년 가이드라인
    let years = vec![2026, 2027, 2028, 2029, 2030];
    for (i, yr) in years.into_iter().enumerate() {
        let (ganzi, stage, score) = match i {
            0 => ("丙午 (병오)", "태(胎)", "★★★☆☆ - 호기심 가득한 인연 출현"),
            1 => ("丁未 (정미)", "양(養)", "★★★★☆ - 안정감 있는 만남 지속"),
            2 => (
                "戊申 (무신)",
                "장생(長生)",
                "★★★★★ - 평생의 인연이 나타나는 적기",
            ),
            3 => (
                "己酉 (기유)",
                "목욕(沐浴)",
                "★★★★☆ - 연애의 화려함과 매력 상승",
            ),
            _ => ("庚戌 (경술)", "관대(冠帶)", "★★★☆☆ - 책임감 있는 관계 정립"),
        };
        s.push_str(&format!(
            "| {}년 | {} | {} | {} |\n",
            yr, ganzi, stage, score
        ));
    }
    s.push('\n');

    // Section 4: 행복 가이드
    s.push_str("### 4. 행복하고 조화로운 관계를 위한 액션 솔루션\n\n");
    s.push_str("- **상호 존중 처방**: 갈등이 생겼을 때 먼저 한 걸음 물러나 상대방의 감정적 요구를 경청해 주십시오.\n");
    s.push_str("- **궁합 보완법**: 자신이 가진 오행 중 부족한 기운을 풍부하게 가진 파트너를 만나면, 상호 보완적인 시너지 효과를 통해 삶의 전체적인 평온을 얻게 됩니다.\n");

    (title, s)
}

// ─────────────────────────────────────────────────────────────────────────────
// 3. 건강 및 마음 치유 테마 보고서 생성기
// ─────────────────────────────────────────────────────────────────────────────
fn generate_health_report(saju: &SajuAnalysisOutput, name: &str) -> (String, String) {
    let title = format!("🌿 {}님의 건강 체질 & 마음 치유 리포트", name);
    let mut s = String::new();

    let day_stem = saju.report.pillars.day.stem;

    // Section 1: 체질분석
    s.push_str("### 1. 오행 배분으로 보는 타고난 건강 체질\n\n");

    let day_stem_kor = HeavenlyStem::HANGUL[day_stem.index() as usize];

    s.push_str(&format!(
        "당신은 **{}** 일간의 체질을 바탕으로 태어났습니다. 사주 오행 분포도에 기반한 장부(臟腑) 건강 상태는 다음과 같습니다.\n\n",
        day_stem_kor
    ));

    // 오행 강약 분석에 기반한 건강 조언
    let mut fire_count = 0;
    let mut water_count = 0;
    for pillar in &[
        &saju.report.pillars.year,
        &saju.report.pillars.month,
        &saju.report.pillars.day,
        &saju.report.pillars.hour,
    ] {
        if pillar.stem.element() == Element::Fire {
            fire_count += 1;
        }
        if pillar.branch.element() == Element::Fire {
            fire_count += 1;
        }
        if pillar.stem.element() == Element::Water {
            water_count += 1;
        }
        if pillar.branch.element() == Element::Water {
            water_count += 1;
        }
    }

    let health_desc = if fire_count > 3 {
        "명식 내에 **불(火)의 기운이 과다**한 편으로 분류됩니다. 이 체질은 혈압 관리, 심혈관계 질환, 몸에 열이 몰려 생기는 피부 질환이나 안구 건조증을 예방하는 식습관이 중요합니다. 평소 충분한 수분 섭취와 규칙적인 수면을 적극 권장합니다."
    } else if water_count > 3 {
        "명식 내에 **물(水)의 기운이 과다**하거나 차가운 성질이 짙습니다. 이 경우 냉증, 신장 및 방광 건강, 순환계통의 둔화에 유의해야 합니다. 따뜻한 성질의 차를 즐겨 마시고 하체 근력 운동을 꾸준히 해 주는 것이 신체 밸런스 유지에 유리합니다."
    } else {
        "오행이 비교적 고르게 안배되어 있어, 특정 장기의 급격한 기능 저하 우려는 낮습니다. 다만 계절이 바뀌는 환절기에 피로가 누적되기 쉬우니 충분한 영양 공급과 스트레칭으로 신체 리듬을 정비하십시오."
    };
    s.push_str(health_desc);
    s.push_str("\n\n");

    // Section 2: 주의해야 할 신체 부위 및 에러 Lints
    s.push_str("### 2. 신체 불균형 및 주요 경고 신호 (Destiny Lints)\n\n");
    s.push_str("EON 린터 엔진이 탐지한 체질적 잠재 취약점 정보입니다.\n\n");

    if saju.lints.is_empty() {
        s.push_str("- **탐지된 잠재 경고**: 없음 (신체적 균형이 안정적입니다)\n\n");
    } else {
        for lint in saju.lints.iter().take(2) {
            s.push_str(&format!("- **[{:?}]**: {}\n", lint.severity, lint.message));
        }
        s.push('\n');
    }

    // Section 3: 마음 치유 및 에너지 관리법
    s.push_str("### 3. 마음 치유 & 웰니스 추천 가이드\n\n");
    s.push_str(
        "신체 건강뿐만 아니라, 스트레스 관리를 위한 마음 치유 웰니스 처방 가이드라인입니다.\n\n",
    );
    s.push_str("- **추천 운동 요법**: 가벼운 유산소 산책과 호흡 명상 요법을 병행하여 체내 산소 순환율을 향상시키십시오.\n");
    s.push_str("- **치유 색상 테라피**: 신체 리듬을 다스리기 위하여, 침실이나 자주 사용하는 소품의 색상을 부드러운 자연의 색조(그린, 소프트 우드 계열)로 선택하는 것을 권장합니다.\n");
    s.push_str("- **스트레스 아웃 가이드**: 과다한 생각과 고민은 위장 기능을 떨어트리는 주원인이 되므로, 하루 10분 온전히 생각을 비우는 명상 훈련을 하십시오.\n");

    (title, s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dto::{AnalysisInput, BirthTimePrecision, SajuAnalysisInput};

    #[test]
    fn test_themed_report_generation() {
        let base = AnalysisInput {
            year: 1990,
            month: 5,
            day: 15,
            hour: 10,
            minute: 0,
            is_lunar: false,
            is_leap_month: false,
            lat: 37.5665,
            lon: 126.9780,
            timezone: "Asia/Seoul".to_string(),
        };
        let saju_input = SajuAnalysisInput {
            base,
            is_male: true,
            use_night_rat_hour: false,
            precision: BirthTimePrecision::Exact,
        };

        // 1. Wealth & Career Report
        let input_wealth = ThemedReportInput {
            base: saju_input.clone(),
            theme: ReportTheme::WealthAndCareer,
            user_name: "홍길동".to_string(),
        };
        let out_wealth = generate(input_wealth).unwrap();
        assert!(out_wealth.title.contains("홍길동"));
        assert!(out_wealth.content.contains("재물"));

        // 2. Love & Marriage Report
        let input_love = ThemedReportInput {
            base: saju_input.clone(),
            theme: ReportTheme::LoveAndMarriage,
            user_name: "홍길동".to_string(),
        };
        let out_love = generate(input_love).unwrap();
        assert!(out_love.title.contains("홍길동"));
        assert!(out_love.content.contains("인연"));

        // 3. Health & Vitality Report
        let input_health = ThemedReportInput {
            base: saju_input,
            theme: ReportTheme::HealthAndVitality,
            user_name: "홍길동".to_string(),
        };
        let out_health = generate(input_health).unwrap();
        assert!(out_health.title.contains("홍길동"));
        assert!(out_health.content.contains("건강"));
    }
}
