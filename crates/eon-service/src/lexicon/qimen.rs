use crate::dto::SymbolInterpretation;
use eon_qimen::core::{Door, Star};

pub fn interpret_door(door: Door) -> Option<SymbolInterpretation> {
    match door {
        Door::Sheng => Some(SymbolInterpretation {
            engine: "QiMen DunJia (奇門遁甲)".to_string(),
            symbol: "Envoy Door: Sheng Men (생문 / 生門 - Life Door)".to_string(),
            classical_grounding: "《기문둔갑비급대전(奇門遁甲秘笈大全)》: '생문은 토(土)에 속하며 만물이 태동하여 번영하는 길문(吉門)이니, 재물 구함과 사업 개척, 혼인과 매매에 으뜸이다.'".to_string(),
            modern_translation: "성장 모멘텀 창출, 신규 수익 파이프라인 개발, 자본 유치 및 비즈니스 밸류에이션 극대화의 최적 문.".to_string(),
            shadow_and_caution: "안일한 낙관론, 과잉 투자로 인한 유동성 고갈 주의.".to_string(),
            tactical_remedy: "성장 기회가 왔을 때 지체 없이 행동하되, 영업 현금 흐름의 바닥선을 명확히 셋업할 것.".to_string(),
        }),
        Door::Kai => Some(SymbolInterpretation {
            engine: "QiMen DunJia (奇門遁甲)".to_string(),
            symbol: "Envoy Door: Kai Men (개문 / 開門 - Open Door)".to_string(),
            classical_grounding: "《연파조수가(煙波釣叟歌)》: '개문은 금(金)에 속하며 정당한 공적 권위와 출세의 관문이니, 관직 승진과 공적 사업에 대길하다.'".to_string(),
            modern_translation: "제도권 진출, 브랜드 런칭, 공공기관 및 대기업과의 공식 파트너십 구축, 명예로운 리더십 발휘.".to_string(),
            shadow_and_caution: "지나친 격식주의와 규정 준수로 인한 혁신 속도 저하.".to_string(),
            tactical_remedy: "공식 채널을 통해 당당하고 투명하게 제안서를 제출하고 공적 공인을 획득할 것.".to_string(),
        }),
        Door::Xiu => Some(SymbolInterpretation {
            engine: "QiMen DunJia (奇門遁甲)".to_string(),
            symbol: "Envoy Door: Xiu Men (휴문 / 休門 - Rest Door)".to_string(),
            classical_grounding: "《기문비급》: '휴문은 수(水)에 속하여 만물이 휴식하고 안정을 취하니, 귀인과의 회동과 화해, 재충전에 최선이다.'".to_string(),
            modern_translation: "전략적 재정비, 신뢰할 수 있는 파트너와의 제휴, 멘토 및 후원자 영입, 심신의 회복.".to_string(),
            shadow_and_caution: "전투력 상실, 지나친 나태와 기한 지연.".to_string(),
            tactical_remedy: "직접적인 돌파를 멈추고 네트워킹과 내적 휴식을 통해 에너지 탱크를 완충할 것.".to_string(),
        }),
        Door::Shang => Some(SymbolInterpretation {
            engine: "QiMen DunJia (奇門遁甲)".to_string(),
            symbol: "Envoy Door: Shang Men (상문 / 傷門 - Harm Door)".to_string(),
            classical_grounding: "《기문둔갑秘笈》: '상문은 목(木)에 속하며 공격과 포획에 적합하나, 스스로 다치는 흉문이니 송사와 충돌을 조심해야 한다.'".to_string(),
            modern_translation: "부실 채권 회수, 강력한 감사 및 구조조정, 경쟁사와의 치열한 영역 다툼.".to_string(),
            shadow_and_caution: "감정적 폭주로 인한 법적 분쟁, 인적 네트워크의 회복 불가능한 파괴.".to_string(),
            tactical_remedy: "공격적 액션은 법적 가이드라인 내에서만 절제되게 수행하고 사적 보복은 엄격히 금지할 것.".to_string(),
        }),
        Door::Du => Some(SymbolInterpretation {
            engine: "QiMen DunJia (奇門遁甲)".to_string(),
            symbol: "Envoy Door: Du Men (두문 / 杜門 - Delusion Door)".to_string(),
            classical_grounding: "《연파조수가》: '두문은 목(木)에 속하여 기운이 막히고 닫힌 곳이니, 숨어 피하거나 은밀히 연구함에 마땅하다.'".to_string(),
            modern_translation: "스텔스 모드 R&D, 기밀 보안 유지, 내부 인프라 고도화, 대외 노출 최소화.".to_string(),
            shadow_and_caution: "외부와의 소통 단절로 인한 고립, 시장 피드백 부재.".to_string(),
            tactical_remedy: "지금은 대외 마케팅 타이밍이 아님을 인정하고 원천 기술과 핵심 알고리즘의 완성도를 비밀리에 극대화할 것.".to_string(),
        }),
        Door::Jing => Some(SymbolInterpretation {
            engine: "QiMen DunJia (奇門遁甲)".to_string(),
            symbol: "Envoy Door: Jing Men (경문 / 景門 - Scenery Door)".to_string(),
            classical_grounding: "《기문비급》: '경문은 화(火)에 속하여 화려한 문채(文彩)를 뜻하니, 문서 계약과 언론 홍보, 시험과 명예에 길하다.'".to_string(),
            modern_translation: "대규모 마케팅 캠페인, 언론 보도 및 인터뷰, 특허 및 저작권 계약 체결, 비주얼 브랜딩.".to_string(),
            shadow_and_caution: "외화내빈(겉은 화려하나 실속 없음), 구설수 및 허위 과장 광고 리스크.".to_string(),
            tactical_remedy: "빛나는 브랜딩 이면에 실질적 제품 스펙과 계약서의 세부 독소 조항을 현미경 검증할 것.".to_string(),
        }),
        Door::Si => Some(SymbolInterpretation {
            engine: "QiMen DunJia (奇門遁甲)".to_string(),
            symbol: "Envoy Door: Si Men (사문 / 死門 - Death Door)".to_string(),
            classical_grounding: "《기문둔갑》: '사문은 토(土)에 속하여 만물이 결실을 맺고 땅에 묻히니, 부동산과 매장, 과거의 청산에 합당하다.'".to_string(),
            modern_translation: "부실 프로젝트 셧다운, 과거 레거시 시스템의 과감한 청산, 실물 부동산 및 토지 자산 매입.".to_string(),
            shadow_and_caution: "에너지의 극심한 침체, 우울감, 새로운 사업 착수의 실패.".to_string(),
            tactical_remedy: "새로운 씨앗을 뿌리려 하지 말고, 수명이 다한 것을 말끔히 장례 치르고 토양을 정화할 것.".to_string(),
        }),
        Door::Jing2 => Some(SymbolInterpretation {
            engine: "QiMen DunJia (奇門遁甲)".to_string(),
            symbol: "Envoy Door: Jing Men (경문 / 驚門 - Fear Door)".to_string(),
            classical_grounding: "《연파조수가》: '경문은 금(金)에 속하여 놀람과 의혹, 송사를 주관하니 재판과 변론, 위기 포착에 유용하다.'".to_string(),
            modern_translation: "컴플라이언스 리스크 진단, 위기 대응(Crisis PR) 가동, 변호사 및 소송 대리인을 통한 적극적 방어.".to_string(),
            shadow_and_caution: "근거 없는 공포와 패닉, 악성 루머에 휘말려 내적 평정심을 상실하는 현상.".to_string(),
            tactical_remedy: "루머나 위협에 감정적으로 동요하지 말고, 팩트 기반의 증거 자료를 철저히 수집하여 공식 반박할 것.".to_string(),
        }),
    }
}

pub fn interpret_star(star: Star) -> Option<SymbolInterpretation> {
    match star {
        Star::Fu => Some(SymbolInterpretation {
            engine: "QiMen DunJia (奇門遁甲)".to_string(),
            symbol: "Chief Star: Tian Fu (천보성 / 天輔星 - Assistant Star)".to_string(),
            classical_grounding: "《기문비급》: '천보성은 문곡(文曲)의 정기이니 학문과 교육, 문화 예술과 인재 육성에 가장 뛰어난 상길성이다.'".to_string(),
            modern_translation: "학술 연구, 지식재산권(IP) 구축, 교육 및 코칭 비즈니스, 전략적 고문(Advisory) 역할 수행.".to_string(),
            shadow_and_caution: "현실적 실행력 부족, 이론과 이상에 치우쳐 사업적 수익화를 놓칠 위험.".to_string(),
            tactical_remedy: "고도화된 지식을 즉시 디지털 콘텐츠나 비즈니스 아카데미 모델로 체계화할 것.".to_string(),
        }),
        Star::Qin => Some(SymbolInterpretation {
            engine: "QiMen DunJia (奇門遁甲)".to_string(),
            symbol: "Chief Star: Tian Qin (천금성 / 天禽星 - Central Sovereign Star)".to_string(),
            classical_grounding: "《기문둔갑》: '천금성은 중궁(中宮)의 으뜸 황극이니 모든 방위를 통어하며 백 가지 일을 도모함에 거리낌이 없다.'".to_string(),
            modern_translation: "중앙 통제실 운영, 플랫폼 비즈니스의 허브 역할, 거버넌스와 통합 조정의 최고 사령탑.".to_string(),
            shadow_and_caution: "중앙 집권적 과부하, 모든 책임을 혼자 지려는 영웅주의.".to_string(),
            tactical_remedy: "중앙 거버넌스 원칙만 확립하고 실행 권한은 각 기능 부서에 신속히 분산할 것.".to_string(),
        }),
        Star::Xin => Some(SymbolInterpretation {
            engine: "QiMen DunJia (奇門遁甲)".to_string(),
            symbol: "Chief Star: Tian Xin (천심성 / 天心星 - Strategist Star)".to_string(),
            classical_grounding: "《기문둔갑》: '천심성은 지혜와 의술, 군략의 별이니 의약의 치료와 큰 계책을 세움에 대길하다.'".to_string(),
            modern_translation: "전략 컨설팅, 헬스케어 및 바이오, 복잡한 시스템의 병목을 진단하고 치료하는 마스터 아키텍트.".to_string(),
            shadow_and_caution: "타인의 결점을 지나치게 해부하여 인간적 신뢰를 잃을 위험.".to_string(),
            tactical_remedy: "진단에 그치지 말고 상대방이 안심하고 회복할 수 있는 구체적 실행 솔루션을 패키지로 제공할 것.".to_string(),
        }),
        _ => None,
    }
}
