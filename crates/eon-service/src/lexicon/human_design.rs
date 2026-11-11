use crate::dto::SymbolInterpretation;

pub fn interpret_type_strategy(chart_type: &str, strategy: &str) -> Option<SymbolInterpretation> {
    let type_lower = chart_type.to_lowercase();
    if type_lower.contains("projector") || type_lower.contains("프로젝터") {
        Some(SymbolInterpretation {
            engine: "Human Design System".to_string(),
            symbol: format!("Type: Projector | Strategy: {}", strategy),
            classical_grounding: "《The Definitive Book of Human Design》: '프로젝터는 비에너지 타입이자 차세대 가이드이니, 직접 모터를 돌리기보다 타인의 에너지를 인도할 때 빛난다.'".to_string(),
            modern_translation: "시스템 조율자, 마스터 컨설턴트, 다른 사람의 잠재력을 꿰뚫어보고 최적의 배치와 전략을 제시하는 최고 가이드.".to_string(),
            shadow_and_caution: "인정받지 못했는데 먼저 덤벼들어 조언하다 거절당하는 쓴맛(Bitterness), 에너지 고갈과 만성 피로.".to_string(),
            tactical_remedy: "절대 먼저 밀어붙이지 말고, 자신의 전문성을 조용히 가꾸며 '공식적인 인정과 초대(Invitation)'를 기다린 후 개입할 것.".to_string(),
        })
    } else if type_lower.contains("generator") || type_lower.contains("제너레이터") {
        Some(SymbolInterpretation {
            engine: "Human Design System".to_string(),
            symbol: format!("Type: Generator | Strategy: {}", strategy),
            classical_grounding: "《Human Design》: '천골(Sacral)의 무한한 생명력 에너지를 지닌 인류의 위대한 건설자이자 장인.'".to_string(),
            modern_translation: "지치지 않는 실행력의 소유자, 좋아하는 일에 몰입할 때 거대한 현실적 결과물을 창출해 내는 마스터 빌더.".to_string(),
            shadow_and_caution: "머리로 해야 한다고 생각하는 일에 억지로 매달리다 마주하는 깊은 좌절감(Frustration).".to_string(),
            tactical_remedy: "머리로 결론짓지 말고, 외부 자극에 대해 배 속(천골)에서 올라오는 직관적인 '응/아니'의 소리에 반응하여 움직일 것.".to_string(),
        })
    } else if type_lower.contains("manifestor") || type_lower.contains("매니페스터") {
        Some(SymbolInterpretation {
            engine: "Human Design System".to_string(),
            symbol: format!("Type: Manifestor | Strategy: {}", strategy),
            classical_grounding: "《Human Design》: '역사를 시작하고 새로운 패러다임을 발화시키는 유일한 순수 이니시에이터(Initiator).'".to_string(),
            modern_translation: "새로운 프로젝트를 독자적으로 개시하는 퍼스트 무버, 누구의 허락도 필요 없는 독립적 혁신가.".to_string(),
            shadow_and_caution: "타인을 알리지 않고 혼자 폭주하다 마주하는 주변의 저항과 고립, 깊은 분노(Anger).".to_string(),
            tactical_remedy: "행동하기 전 영향을 받을 사람들에게 반드시 사전에 '알리기(Informing)'를 통해 저항을 평화로 바꿀 것.".to_string(),
        })
    } else if type_lower.contains("reflector") || type_lower.contains("리플렉터") {
        Some(SymbolInterpretation {
            engine: "Human Design System".to_string(),
            symbol: format!("Type: Reflector | Strategy: {}", strategy),
            classical_grounding: "《Human Design》: '9개 센터가 모두 열린 달의 존재이니, 공동체와 환경의 건강 상태를 완벽하게 반영하는 거울이다.'".to_string(),
            modern_translation: "시대의 흐름과 조직의 영적·신체적 상태를 정확히 진단하는 궁극의 평가관이자 영적 카운슬러.".to_string(),
            shadow_and_caution: "부적절한 환경과 탁한 에너지에 노출되어 영혼이 병드는 실망감(Disappointment).".to_string(),
            tactical_remedy: "중대한 결정 시 달의 주기(28.5일) 동안 숙고하며 환경을 무엇보다 까다롭게 고를 것.".to_string(),
        })
    } else {
        None
    }
}

pub fn interpret_profile(profile: &str) -> Option<SymbolInterpretation> {
    if profile.contains("6/2") {
        Some(SymbolInterpretation {
            engine: "Human Design System".to_string(),
            symbol: "Profile: 6/2 (Role Model / Hermit — 롤모델 / 은둔자)".to_string(),
            classical_grounding: "《Human Design Profile》: '1단계(30세까지) 시행착오의 시기를 거쳐 지붕 위로 올라가 관조한 뒤 마침내 살아있는 롤모델로 하산하는 영웅의 여정.'".to_string(),
            modern_translation: "젊은 날의 온갖 풍파와 경험을 지혜로 승화시켜, 높은 곳에서 전체를 조망하며 타인에게 영감을 주는 살아있는 롤모델.".to_string(),
            shadow_and_caution: "완벽하지 않은 세상에 대한 냉소와 환멸, 자기만의 동굴에 지나치게 오래 갇혀 하산하지 않으려는 고립.".to_string(),
            tactical_remedy: "은둔의 시간(2라인)으로 에너지를 충전하되, 준비가 되었을 때 자신의 통찰을 세상과 나누며 모범을 보일 것.".to_string(),
        })
    } else if profile.contains("1/3") {
        Some(SymbolInterpretation {
            engine: "Human Design System".to_string(),
            symbol: "Profile: 1/3 (Investigator / Martyr — 탐구자 / 순교자)".to_string(),
            classical_grounding: "《Human Design Profile》: '밑바닥 기초를 파고드는 철저한 탐구와 수많은 시행착오(Trial & Error)를 통해 진짜 작동하는 진실만을 걸러내는 구조.'".to_string(),
            modern_translation: "어설픈 것은 용납하지 않는 팩트체커, 실패를 통해 시장과 시스템의 취약점을 완벽하게 찾아내는 최고 감사관.".to_string(),
            shadow_and_caution: "실패에 대한 자책감, 불확실성에 대한 두려움으로 인한 연구 중독과 행동 지연.".to_string(),
            tactical_remedy: "실패를 오류가 아닌 '작동하지 않는 것을 발견한 값진 데이터'로 정의하고 과감하게 테스트를 지속할 것.".to_string(),
        })
    } else if profile.contains("3/5") {
        Some(SymbolInterpretation {
            engine: "Human Design System".to_string(),
            symbol: "Profile: 3/5 (Martyr / Heretic — 순교자 / 이단아)".to_string(),
            classical_grounding: "《Human Design Profile》: '부딪히고 깨지며 얻은 실전 노하우로 대중의 위기를 구원하는 파격적인 솔루션을 제시하는 해결사.'".to_string(),
            modern_translation: "위기 현장의 트러블슈터(Troubleshooter), 대중의 투사를 받아 문제를 근본적으로 해결해 내는 카리스마 넘치는 이단아.".to_string(),
            shadow_and_caution: "타인의 비현실적인 기대(투사)에 부응하지 못했을 때 쏟아지는 비난과 상처, 관계의 돌발 단절.".to_string(),
            tactical_remedy: "해결할 수 있는 현실적 약속만을 명확히 제한하고, 투사가 깨지기 전에 명확한 결과물을 제시하고 퇴장할 것.".to_string(),
        })
    } else {
        None
    }
}
