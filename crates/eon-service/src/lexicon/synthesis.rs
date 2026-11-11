use crate::dto::CrossEngineSynthesisGuideline;

pub fn generate_synthesis(
    day_master_name: Option<&str>,
    sun_sign_name: Option<&str>,
    hd_type: Option<&str>,
    numerology_lp: Option<u8>,
    numerology_py: Option<u8>,
    is_seven_killings_or_intense: bool,
    has_wealth_yoga_or_structure: bool,
) -> (
    String,
    String,
    Vec<CrossEngineSynthesisGuideline>,
    Vec<String>,
    Vec<String>,
) {
    let dm_clean = day_master_name.unwrap_or("Unknown");
    let sun_clean = sun_sign_name.unwrap_or("Unknown");
    let hd_clean = hd_type.unwrap_or("Unknown");
    let lp_val = numerology_lp.unwrap_or(1);
    let py_val = numerology_py.unwrap_or(1);

    // 1. Archetypal Persona Determination
    let persona_title = if dm_clean.contains("Jia")
        || dm_clean.contains("Geng")
        || lp_val == 1
        || lp_val == 8
        || hd_clean.to_lowercase().contains("manifestor")
    {
        "The Sovereign Architect (주권적 설계자 & 문명 개척자)"
    } else if dm_clean.contains("Bing")
        || dm_clean.contains("Ding")
        || lp_val == 3
        || lp_val == 5
        || sun_clean.contains("Leo")
        || sun_clean.contains("Aries")
    {
        "The Radiant Catalyst (빛의 혁신가 & 패러다임 촉매자)"
    } else if dm_clean.contains("Ren")
        || dm_clean.contains("Gui")
        || lp_val == 7
        || lp_val == 11
        || hd_clean.to_lowercase().contains("projector")
    {
        "The Intuitive Strategist (직관적 전략가 & 심연의 네비게이터)"
    } else if dm_clean.contains("Wu") || dm_clean.contains("Ji") || lp_val == 4 || lp_val == 22 {
        "The Master Builder (현실적 거장 & 대지적 토대 구축자)"
    } else {
        "The Harmonizing Alchemist (조화의 연금술사 & 보편적 가치 전달자)"
    };

    let archetypal_persona = format!(
        "{} [Day Master: {} | Sun: {} | Type: {} | LP: {}]",
        persona_title, dm_clean, sun_clean, hd_clean, lp_val
    );

    // 2. Core Essence Summary
    let core_essence_summary = format!(
        "본 차트는 동양의 음양오행 및 사주명리 격국 체계, 베딕 점성학의 카르마적 요가, 서양 점성학의 행성 원형 각도, 그리고 휴먼디자인 및 피타고라스 수비학이 정밀하게 일치하는 다차원 구조를 지닙니다. 일간(Day Master) '{}'의 기운과 서양 태양 '{}', 휴먼디자인 '{}' 타입 및 수비학 생애여정수 '{}'이 융합되어, 타인의 압박에 종속되지 않고 스스로 거버넌스와 실행 시스템을 장악할 때 폭발적인 현실적 권능을 발휘하는 천부적 리더십 구조를 형성하고 있습니다.",
        dm_clean, sun_clean, hd_clean, lp_val
    );

    // 3. Cross-Engine Synthesis Guidelines
    let career_guideline = if has_wealth_yoga_or_structure {
        CrossEngineSynthesisGuideline {
            domain: "Career & Wealth Architecture (커리어 및 자본 증식)".to_string(),
            combined_insight: "사주의 강력한 격국/용신 기운과 베딕 점성학의 재물 요가(Dhana/Raja Yoga), 수비학의 추진력이 상호 증폭되고 있습니다. 단순 노동 소득을 넘어 고유한 지적 자산과 자본 레버리지를 융합하는 사업적 성취가 명확히 약속된 구조입니다.".to_string(),
            strategic_action: "단기적 잔파도에 일희일비하지 말고, 복리 효과를 창출하는 독점적 플랫폼이나 핵심 IP(지적재산권) 시스템을 선제 구축하십시오. 전문 인력과 자본을 통솔하는 거버넌스 확립에 집중해야 합니다.".to_string(),
        }
    } else {
        CrossEngineSynthesisGuideline {
            domain: "Career & Wealth Architecture (커리어 및 자본 증식)".to_string(),
            combined_insight: "전문적 연구 역량, 독창적 콘텐츠 기획력, 그리고 독보적인 기술력이 현실적 가치로 전환되는 지식 자본 중심의 구조입니다. 형식적인 조직의 틀보다 전문성을 입증할 수 있는 공인 라이선스나 특허가 핵심 자산입니다.".to_string(),
            strategic_action: "기술이나 전문 연구 성과를 시장의 유통 채널(재성)과 직접 연결할 수 있는 공식 비즈니스 파이프라인을 구축하십시오. 전문성을 가진 파트너와의 분업이 성공의 열쇠입니다.".to_string(),
        }
    };

    let relationship_guideline = CrossEngineSynthesisGuideline {
        domain: "Interpersonal Chemistry & Leadership (대인 관계, 파트너십 및 거버넌스)".to_string(),
        combined_insight: "통찰력이 깊고 주권 의식이 강하여 상대방의 위선이나 비효율을 빠르게 간파하는 경향이 있습니다. 이는 뛰어난 리더십의 원천이나, 감정적 타협이 어려워 불필요한 마찰을 초래할 수 있습니다.".to_string(),
        strategic_action: "모든 사람을 교정하려 하지 말고, 공식적인 인정과 초대를 받은 영역에서만 조언을 제공하십시오. 명확한 상호 역할 분담(R&R)과 심리적 완충 공간(Boundaries)을 둘 때 최상의 시너지가 유지됩니다.".to_string(),
    };

    let timing_guideline = CrossEngineSynthesisGuideline {
        domain: "Operational Rhythm & Timing Strategy (실행 리듬, 위기 대응 및 주기 타이밍)".to_string(),
        combined_insight: format!(
            "현재 피타고라스 수비학 Personal Year {}의 진동수와 기문둔갑 및 대운의 주기적 흐름이 맞물려 있습니다. 에너지의 썰물과 밀물을 분별하여 행동할 때 최소한의 마찰로 최대의 결실을 거둘 수 있습니다.",
            py_val
        ),
        strategic_action: match py_val {
            1 | 8 => "강력한 주도권을 쥐고 신규 프로젝트의 착수 또는 대규모 자본 집행을 단행하기에 최적인 수확/개척기입니다. 과감한 결단과 실행을 권장합니다.".to_string(),
            2 | 6 => "동반자와의 전략적 제휴, 팀워크 결속, 내부 인프라 및 거버넌스 보강에 집중할 때입니다. 정면 충돌을 피하고 연대 전선을 구축하십시오.".to_string(),
            3 | 5 => "대외 마케팅, 브랜드 홍보, 해외 진출 및 비즈니스 모델 피벗(Pivot)에 유리합니다. 창의적 에너지를 세상에 적극적으로 발산하십시오.".to_string(),
            4 | 7 => "외부 확장을 절제하고 R&D, 시스템 고도화, 특허 출원, 그리고 심신의 에너지 완충에 집중해야 하는 내적 성찰기입니다. 무리한 투자는 보류하십시오.".to_string(),
            _ => "장기 프로젝트를 매듭짓고 불필요한 부실 자산과 한계 관계를 정리하여 다음 주기를 맞이할 공간을 비워내는 대단원의 정리 시기입니다.".to_string(),
        },
    };

    let synthesis_guidelines = vec![career_guideline, relationship_guideline, timing_guideline];

    // 4. Tactical DOs and DON'Ts
    let mut tactical_dos = vec![
        "자신만의 고유 영역(Niche)과 거버넌스 시스템을 독립적으로 구축할 것.".to_string(),
        "두뇌의 과도한 계산보다 신체의 직접적 반응(천골/직관)과 공식 초대(Strategy)에 따라 의사결정할 것.".to_string(),
        "지적재산권, 특허, 전문 노하우 등 진입장벽이 높은 무형 자산을 조기에 자본화할 것.".to_string(),
        "주간 일정에 반드시 단독 사유 및 충전 시간(Solo Buffer)을 의무 배치하여 신경계 피로를 예방할 것.".to_string(),
    ];

    let mut tactical_donts = vec![
        "공식적인 요청이나 인정이 없는 상태에서 타인의 일에 먼저 개입하여 쓴맛(Bitterness)을 자초하지 말 것.".to_string(),
        "체면과 주변의 평판에 흔들려 본인의 핵심 원칙과 장기 비전을 헐값에 양보하지 말 것.".to_string(),
        "모든 전선의 세부 실행을 혼자 감당하려는 마이크로매니지먼트와 영웅주의에 빠지지 말 것.".to_string(),
        "감정이 격앙되었거나 조급함이 극에 달했을 때 대규모 자본 투자나 중대 계약을 즉흥적으로 체결하지 말 것.".to_string(),
    ];

    if is_seven_killings_or_intense {
        tactical_dos.insert(
            0,
            "극단적 압박 상황에서 감정적으로 반응하지 말고, 48시간의 전략적 숙고 지대를 둘 것."
                .to_string(),
        );
        tactical_donts.insert(
            0,
            "위기 상황에서 독단적 분노를 표출하여 핵심 협력자와의 신뢰 자산을 일시에 태워버리지 말 것.".to_string(),
        );
    }

    (
        archetypal_persona,
        core_essence_summary,
        synthesis_guidelines,
        tactical_dos,
        tactical_donts,
    )
}
