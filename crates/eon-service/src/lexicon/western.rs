use crate::dto::SymbolInterpretation;

pub fn interpret_aspect_pattern(pattern_name: &str) -> Option<SymbolInterpretation> {
    let name_lower = pattern_name.to_lowercase();
    if name_lower.contains("tsquare") || name_lower.contains("t-square") {
        Some(SymbolInterpretation {
            engine: "Western Astrology".to_string(),
            symbol: "T-Square Aspect Configuration (T-스퀘어 복합 각도)".to_string(),
            classical_grounding: "심리 점성학(Psychological Astrology): '두 행성의 180도 충돌과 이를 90도로 찌르는 초점 행성(Focal Planet)이 형성하는 극도의 긴장.'".to_string(),
            modern_translation: "결핍이 낳은 비범한 성취 엔진. 안주하지 못하는 내적 결핍과 압박감이 폭발적인 성장과 돌파구를 창출하는 원동력.".to_string(),
            shadow_and_caution: "만성적인 내적 불안, 스스로를 채찍질하는 완벽주의, 초점 행성의 영역에서 발생하는 반복적인 좌절.".to_string(),
            tactical_remedy: "초점 행성의 정반대 지점(빈 공간 / Empty Leg)의 에너지를 의도적으로 개발하여 긴장감을 분산시키고 쉼표를 찍을 것.".to_string(),
        })
    } else if name_lower.contains("grandtrine") || name_lower.contains("grand-trine") {
        Some(SymbolInterpretation {
            engine: "Western Astrology".to_string(),
            symbol: "Grand Trine Aspect Configuration (그랜드 트라인 / 대삼합)".to_string(),
            classical_grounding: "헬레니즘 점성학: '동일 원소의 세 행성이 120도로 조화롭게 순환하여 저절로 흘러넘치는 천부적 재능.'".to_string(),
            modern_translation: "노력하지 않아도 주어지는 고유의 재능, 직관적인 흐름 장악력, 위기 속에서도 저절로 탈출구가 열리는 행운.".to_string(),
            shadow_and_caution: "극단적인 나태와 매너리즘, 저항이 없는 환경에 길들여져 도전과 압박을 견디지 못하는 회피성.".to_string(),
            tactical_remedy: "자연스러운 재능에 엄격한 마감 기한과 외부적 도전을 부과하여 실질적이고 가시적인 상업적 성과물로 빚어낼 것.".to_string(),
        })
    } else if name_lower.contains("grandcross") || name_lower.contains("grand-cross") {
        Some(SymbolInterpretation {
            engine: "Western Astrology".to_string(),
            symbol: "Grand Cross Aspect Configuration (그랜드 크로스 / 대십자)".to_string(),
            classical_grounding: "점성학: '네 행성이 십자가 형태로 팽팽히 맞서 삶의 전방위적 영역에서 피할 수 없는 시험을 부과함.'".to_string(),
            modern_translation: "강철을 담금질하는 극한의 시련과 거인의 탄생. 모든 역경을 극복해 낸 후 대체 불가능한 역사적 족적을 남기는 인물.".to_string(),
            shadow_and_caution: "사방에서 옥죄어오는 압박감으로 인한 심신 붕괴, 극단적 좌절감, 분노 조절 실패.".to_string(),
            tactical_remedy: "한 번에 모든 전선을 해결하려 하지 말고 우선순위를 명확히 세워 하나씩 각개격파하는 전략적 침묵을 유지할 것.".to_string(),
        })
    } else if name_lower.contains("stellium") {
        Some(SymbolInterpretation {
            engine: "Western Astrology".to_string(),
            symbol: "Stellium (스텔리움 / 행성 집중군)".to_string(),
            classical_grounding: "점성학: '한 하우스 또는 별자리에 3개 이상의 행성이 융합되어 특정 인생 영역에 운명의 모든 빛이 쏟아짐.'".to_string(),
            modern_translation: "한 분야의 초전문가(Hyper-Specialist), 레이저처럼 응축된 집중력, 특정 인생 무대에서 독점적 영향력 행사.".to_string(),
            shadow_and_caution: "시야 협착, 해당 영역의 문제에 인생 전체가 휘둘리는 편중성, 균형 감각의 상실.".to_string(),
            tactical_remedy: "집중된 영역의 반대편 하우스(보완 영역)를 주기적으로 점검하여 라이프 밸런스를 붕괴시키지 않도록 할 것.".to_string(),
        })
    } else if name_lower.contains("yod") {
        Some(SymbolInterpretation {
            engine: "Western Astrology".to_string(),
            symbol: "Yod (Finger of God / 신의 손가락)".to_string(),
            classical_grounding: "점성학: '두 행성의 섹스타일(60도)이 초점 행성을 150도(퀸컹크스)로 겨누어 피할 수 없는 운명적 소명을 부여함.'".to_string(),
            modern_translation: "운명적 궤도 수정, 특정한 영적·사회적 사명을 위해 끊임없이 재조정(Adjustment)을 겪으며 도달하는 독보적 경지.".to_string(),
            shadow_and_caution: "일상의 사소한 집착에서 벗어나지 못할 때 겪는 설명하기 힘든 불편감과 지속적인 우회로 탐색의 피로.".to_string(),
            tactical_remedy: "통제하려는 욕망을 내려놓고 사건이 이끄는 방향을 수용하며 세밀한 태도 조정을 반복할 것.".to_string(),
        })
    } else {
        None
    }
}
