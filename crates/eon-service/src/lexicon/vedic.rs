use crate::dto::SymbolInterpretation;

pub fn interpret_yoga(yoga_name: &str) -> Option<SymbolInterpretation> {
    let name_lower = yoga_name.to_lowercase();
    if name_lower.contains("gaja") || name_lower.contains("kesari") {
        Some(SymbolInterpretation {
            engine: "Vedic Astrology (Jyotish)".to_string(),
            symbol: "Gaja Kesari Yoga (Elephant-Lion Royal Union / 가자 케사리 요가)".to_string(),
            classical_grounding: "《Brihat Parashara Hora Shastra(BPHS)》: '목성(지혜)이 달(마음)로부터 케ند라(1, 4, 7, 10실)에 거하면 모든 장애를 파하고 왕의 신망과 영원한 명성을 얻는다.'".to_string(),
            modern_translation: "품격 있는 지성, 대중적 신뢰와 인망, 위기 상황에서 강력한 멘토와 후원자(귀인)의 조력을 이끌어내는 인품.".to_string(),
            shadow_and_caution: "도덕적 우월감, 타인의 미숙함을 훈계하려는 설교벽, 실질적 행동보다 조언자 역할에 머무는 태만.".to_string(),
            tactical_remedy: "조언에 머무르지 말고 지혜를 교육 플랫폼, 저술, 전략 자문 계약 등 공적 시스템으로 구체화할 것.".to_string(),
        })
    } else if name_lower.contains("raja")
        || name_lower.contains("l1")
        || name_lower.contains("l9")
        || name_lower.contains("l10")
    {
        Some(SymbolInterpretation {
            engine: "Vedic Astrology (Jyotish)".to_string(),
            symbol: format!("Raja Yoga (Royal Authority & Power / 라자 요가) — {}", yoga_name),
            classical_grounding: "《BPHS》: '켄드라(활동의 중심)와 트리코나(신의 축복)의 지배자가 서로 결합할 때 최고의 권위와 세속적 권력을 획득한다.'".to_string(),
            modern_translation: "정상급 임원, 권한을 위임받는 핵심 디렉터, 자신의 능력에 상응하는 공적 권한과 사회적 지위의 상승.".to_string(),
            shadow_and_caution: "권력 남용, 권한에 걸맞은 무거운 책임감으로 인한 스트레스, 조직 내 질투와 정적 생성.".to_string(),
            tactical_remedy: "권한을 자신만을 위해 쓰지 말고 조직 구성원에게 권익을 나누어주는 포용적 리더십을 견지할 것.".to_string(),
        })
    } else if name_lower.contains("dhana") {
        Some(SymbolInterpretation {
            engine: "Vedic Astrology (Jyotish)".to_string(),
            symbol: format!("Dhana Yoga (Wealth Magnetism & Accumulation / 다나 요가) — {}", yoga_name),
            classical_grounding: "《BPHS》: '재물의 궁(2실, 11실)과 행운의 궁(1실, 5실, 9실)이 연결되면 마르지 않는 샘물처럼 자산이 축적된다.'".to_string(),
            modern_translation: "자산 스케일업 능력, 전문성과 명성이 고수익 현금 흐름 및 투자 수익으로 전환되는 강력한 재물 운용력.".to_string(),
            shadow_and_caution: "물질적 성공에 대한 집착, 재정 관리 소홀 시 급격한 누수, 돈으로 모든 인간관계를 계산하려는 태도.".to_string(),
            tactical_remedy: "축적된 유동 자산을 안전한 우량 부동산이나 인덱스 자산으로 고정화하여 영속적 패시브 인컴 구조를 세울 것.".to_string(),
        })
    } else if name_lower.contains("budhaditya") {
        Some(SymbolInterpretation {
            engine: "Vedic Astrology (Jyotish)".to_string(),
            symbol: "Budhaditya Yoga (Mercury-Sun Intellectual Brilliance / 부다디티야 요가)".to_string(),
            classical_grounding: "《Phaladeepika》: '태양과 수성이 동궁하면 총명하고 기민하며 학문과 언변에서 만인의 칭송을 받는다.'".to_string(),
            modern_translation: "예리한 분석력, 논리적인 스피치 및 설득력, 복잡한 지식을 단순명료하게 구조화하는 데이터 커뮤니케이터.".to_string(),
            shadow_and_caution: "신경과민, 지나친 계산과 분석 마비(Analysis Paralysis), 말실수로 인한 불필요한 마찰.".to_string(),
            tactical_remedy: "명상과 아날로그 휴식으로 과열된 뇌를 식히고, 직관적 감수성을 보완할 것.".to_string(),
        })
    } else if name_lower.contains("sunapha")
        || name_lower.contains("anapha")
        || name_lower.contains("durdhara")
    {
        Some(SymbolInterpretation {
            engine: "Vedic Astrology (Jyotish)".to_string(),
            symbol: format!("Lunar Wealth & Self-Sufficiency Yoga — {}", yoga_name),
            classical_grounding: "《BPHS》: '달의 전후에 길성이 포진하면 스스로의 노력과 인품으로 자립하여 부귀를 일군다.'".to_string(),
            modern_translation: "자수성가형 자산가, 감정적 안정과 실물 자산의 비례, 타인에게 의존하지 않는 독립적 재정 기반.".to_string(),
            shadow_and_caution: "고립주의, 타인을 쉽게 믿지 못해 모든 실무를 홀로 짊어지려는 과부하.".to_string(),
            tactical_remedy: "신뢰할 수 있는 전문 파트너십을 점진적으로 구축하여 사업을 레버리지할 것.".to_string(),
        })
    } else {
        None
    }
}

pub fn interpret_nakshatra(nakshatra_info: &str) -> Option<SymbolInterpretation> {
    Some(SymbolInterpretation {
        engine: "Vedic Astrology (Jyotish)".to_string(),
        symbol: format!("Moon Nakshatra: {}", nakshatra_info),
        classical_grounding: "《타이티리야 브라마나(Taittiriya Brahmana)》: '낙샤트라는 영혼의 본래 성소(Mansion)이자 깊은 무의식의 본능적 반응 양식이다.'".to_string(),
        modern_translation: "무의식적 본능, 정서적 안정감을 느끼는 필수 환경, 영혼이 세상과 상호작용하는 근원적인 감수성.".to_string(),
        shadow_and_caution: "낙샤트라의 본능적 욕구가 채워지지 않을 때 나타나는 무의식적 방어기제와 관계 단절.".to_string(),
        tactical_remedy: "자신의 본질적 별자리(Nakshatra)가 요구하는 환경(독립성, 탐구, 소통, 치유 등)을 일상 루틴에 필수적으로 배치할 것.".to_string(),
    })
}
