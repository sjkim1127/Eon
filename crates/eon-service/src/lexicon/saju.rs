use crate::dto::SymbolInterpretation;

pub fn interpret_structure(structure_name: &str) -> Option<SymbolInterpretation> {
    let name_lower = structure_name.to_lowercase();
    if name_lower.contains("pianguan")
        || name_lower.contains("sevenkilling")
        || name_lower.contains("chil-sal")
    {
        Some(SymbolInterpretation {
            engine: "Saju (Four Pillars / BaZi)".to_string(),
            symbol: format!("Structure: {} (Seven Killings / 偏官·七殺格)", structure_name),
            classical_grounding: "《자평진전(子平眞詮)》: '편관은 극렬한 기운이니 제압(식신제살)하거나 순화(살인상생)하여 다스릴 때 비로소 거대한 권위와 명예로 화한다.'".to_string(),
            modern_translation: "위기 관리자(Crisis Manager), 혁신적 문제 해결사, 리스크를 감수하고 권한과 영역을 확장하는 개척형 리더.".to_string(),
            shadow_and_caution: "극단적 통제 욕구, 과도한 자기 억압, 타인의 미숙함을 견디지 못하는 독단적 조급함, 번아웃 주의.".to_string(),
            tactical_remedy: "전략적 완충 지대(인성/휴식) 확보. 모든 전선에서 홀로 싸우려 하지 말고 권한을 위임하고 명확한 규율을 시스템화할 것.".to_string(),
        })
    } else if name_lower.contains("zhengguan")
        || name_lower.contains("directofficer")
        || name_lower.contains("jeong-gwan")
    {
        Some(SymbolInterpretation {
            engine: "Saju (Four Pillars / BaZi)".to_string(),
            symbol: format!("Structure: {} (Direct Officer / 正官格)", structure_name),
            classical_grounding: "《자평진전》: '정관은 귀함의 으뜸이니, 순수함을 보존하고 상관의 손상과 형충을 피해야 조직의 동량이 된다.'".to_string(),
            modern_translation: "제도권의 핵심 엘리트, 공적 신뢰를 바탕으로 한 조직 운영자, 거버넌스와 브랜드 가치를 극대화하는 관리자.".to_string(),
            shadow_and_caution: "형식주의와 관료적 경직성, 체면과 평판에 대한 과도한 불안, 급변하는 환경에서의 유연성 결여.".to_string(),
            tactical_remedy: "원칙의 엄격함을 유지하되 하위 팀원의 변칙성을 수용할 수 있는 심리적 여백과 유연한 비상 대책을 마련할 것.".to_string(),
        })
    } else if name_lower.contains("shishen")
        || name_lower.contains("eatinggod")
        || name_lower.contains("sik-sin")
    {
        Some(SymbolInterpretation {
            engine: "Saju (Four Pillars / BaZi)".to_string(),
            symbol: format!("Structure: {} (Eating God / 食神格)", structure_name),
            classical_grounding: "《자평진전》: '식신은 본래 나의 수기를 자연스럽게 설기하여 재성을 생하니 수복(壽福)이 무궁하다.'".to_string(),
            modern_translation: "전문 R&D 연구자, 독창적 콘텐츠 크리에이터, 고유 기술이나 노하우를 경제적 가치로 전환하는 지식 생산자.".to_string(),
            shadow_and_caution: "현실 안주, 압박감이 없을 때 나태해지는 경향, 상업적 마케팅보다 자기만족적 탐구에 치우칠 위험.".to_string(),
            tactical_remedy: "탁월한 연구/창작물을 현실 세계의 비즈니스 플랫폼(재성)과 체계적으로 연결하는 유통 파이프라인 구축.".to_string(),
        })
    } else if name_lower.contains("shangguan")
        || name_lower.contains("hurtingofficer")
        || name_lower.contains("sang-gwan")
    {
        Some(SymbolInterpretation {
            engine: "Saju (Four Pillars / BaZi)".to_string(),
            symbol: format!("Structure: {} (Hurting Officer / 傷官格)", structure_name),
            classical_grounding: "《적천수(滴天髓)》: '상관은 비범한 총명함을 지녔으니, 재성으로 기운을 빼거나 인성으로 제어하면 천하의 기재가 된다.'".to_string(),
            modern_translation: "파괴적 혁신가(Disruptor), 날카로운 언론·법률가, 기성 질서의 모순을 타파하고 트렌드를 선도하는 전략가.".to_string(),
            shadow_and_caution: "독설과 오만함으로 인한 불필요한 적 생성, 기존 권위와의 극단적 충돌, 감정 기복에 따른 프로젝트 파기.".to_string(),
            tactical_remedy: "재능을 방어할 공인된 면허/지적재산권(인성)을 확보하고, 비판을 대안 제시와 상업적 제품화(재성)로 승화시킬 것.".to_string(),
        })
    } else if name_lower.contains("piancai")
        || name_lower.contains("indirectwealth")
        || name_lower.contains("pyeon-jae")
    {
        Some(SymbolInterpretation {
            engine: "Saju (Four Pillars / BaZi)".to_string(),
            symbol: format!("Structure: {} (Indirect Wealth / 偏財格)", structure_name),
            classical_grounding: "《삼명통회(三命通會)》: '편재는 천하의 공공 재물이니 스케일이 웅장하고 유동성을 장악하여 거부를 이룬다.'".to_string(),
            modern_translation: "대규모 자본 운용가, 벤처 캐피털리스트, 시장의 흐름과 공간적 네트워크를 연결하는 비즈니스 개척자.".to_string(),
            shadow_and_caution: "과도한 레버리지 투기, 작은 실속을 경시하는 허장성세, 감당할 수 없는 확장으로 인한 유동성 위기.".to_string(),
            tactical_remedy: "현금 흐름의 바닥선(Safety Margin)을 항시 방어하고, 공격적 자산 투자 시 반드시 전문 리스크 매니저를 둘 것.".to_string(),
        })
    } else if name_lower.contains("zhengcai")
        || name_lower.contains("directwealth")
        || name_lower.contains("jeong-jae")
    {
        Some(SymbolInterpretation {
            engine: "Saju (Four Pillars / BaZi)".to_string(),
            symbol: format!("Structure: {} (Direct Wealth / 正財格)", structure_name),
            classical_grounding: "《자평진전》: '정재는 정직한 근로와 신용의 대가이니, 성실함과 치밀함으로 견고한 부의 성을 쌓는다.'".to_string(),
            modern_translation: "철저한 데이터 기반 재무 전략가, 안정적 캐시카우 자산가, 예측 가능한 가치를 창출하는 실무형 총괄자.".to_string(),
            shadow_and_caution: "기회비용을 놓치는 지나친 보수주의, 불확실성에 대한 극도의 회피, 단기 손실에 대한 과민 반응.".to_string(),
            tactical_remedy: "핵심 자산의 안정성을 유지하되 자본의 10~20%는 고성장 섹터에 배분하여 비대칭 상방 기회를 열어둘 것.".to_string(),
        })
    } else if name_lower.contains("zhengyin")
        || name_lower.contains("directresource")
        || name_lower.contains("jeong-in")
    {
        Some(SymbolInterpretation {
            engine: "Saju (Four Pillars / BaZi)".to_string(),
            symbol: format!("Structure: {} (Direct Resource / 正印格)", structure_name),
            classical_grounding: "《자평진전》: '인성은 나를 낳고 기르는 자애로운 기운이니, 학문과 덕망으로 만인의 존경을 받는다.'".to_string(),
            modern_translation: "학문적 권위자, 브레인 및 전략 참모, 공인된 자격과 원천 특허·라이선스를 보유한 지식 생태계의 구심점.".to_string(),
            shadow_and_caution: "지나친 이상주의, 행동 결핍과 결정 장애, 현실 자본주의의 냉혹한 거래 법칙에 대한 면역력 부족.".to_string(),
            tactical_remedy: "지식을 머릿속에만 두지 말고 계약과 저작권, 라이선스 비즈니스로 즉각 문서화·제도화하여 현실 권력을 쥘 것.".to_string(),
        })
    } else if name_lower.contains("pianyin")
        || name_lower.contains("indirectresource")
        || name_lower.contains("pyeon-in")
    {
        Some(SymbolInterpretation {
            engine: "Saju (Four Pillars / BaZi)".to_string(),
            symbol: format!("Structure: {} (Indirect Resource / 偏印格)", structure_name),
            classical_grounding: "《적천수》: '편인은 신비와 직관의 별이니 특수한 학술과 비범한 기술에서 남들이 흉내 낼 수 없는 극치를 이룬다.'".to_string(),
            modern_translation: "특수 기술 아키텍트, 딥테크/AI 엔지니어, 철학·심리학·메타인지 분야에서 대체 불가능한 통찰을 뿜어내는 전략가.".to_string(),
            shadow_and_caution: "변덕과 권태, 세상과의 고립, 시작은 창대하나 마무리를 회피하는 용두사미 습관(도식 / 倒食).".to_string(),
            tactical_remedy: "자신만의 고립된 방에서 나와 결과물을 시장에 검증받는 루틴을 강제하고, 현실적 재무 파트너와 공조할 것.".to_string(),
        })
    } else if name_lower.contains("jianlu")
        || name_lower.contains("yangren")
        || name_lower.contains("bi-jian")
    {
        Some(SymbolInterpretation {
            engine: "Saju (Four Pillars / BaZi)".to_string(),
            symbol: format!("Structure: {} (Self-Dominant / 祿刃格)", structure_name),
            classical_grounding: "《삼명통회》: '녹(祿)과 인(刃)은 나의 기운이 하늘을 찌르는 것이니 자수성가하여 스스로 깃발을 꽂는 패기다.'".to_string(),
            modern_translation: "자수성가형 독립 사업가, 강력한 추진력과 뚝심으로 맨땅에서 왕국을 세우는 독립적 솔로프리너.".to_string(),
            shadow_and_caution: "타협을 모르는 고집, 동업자와의 지분 분쟁 및 재물 탕진, 타인의 조언을 배척하는 독불장군 성향.".to_string(),
            tactical_remedy: "이익을 나눌 줄 아는 분배의 룰을 사전에 확립하고, 엄격한 회계 및 법률 자문가를 옆에 두어 자만을 경계할 것.".to_string(),
        })
    } else {
        None
    }
}

pub fn interpret_yongshin(element: &str) -> Option<SymbolInterpretation> {
    let elem_upper = element.to_uppercase();
    if elem_upper.contains("WATER") || elem_upper.contains("수") {
        Some(SymbolInterpretation {
            engine: "Saju (Four Pillars / BaZi)".to_string(),
            symbol: "Primary Yongshin: Water (수 / 水用神)".to_string(),
            classical_grounding: "《궁통보감(窮通寶鑑)》: '수는 지혜의 근원이며 만물을 유통시키고 과열된 화(火)기를 조율하는 필수 브레이크다.'".to_string(),
            modern_translation: "전략적 유연성, 데이터 기반 분석, 침착한 관망, 시장 유통과 글로벌 네트워크 레버리지.".to_string(),
            shadow_and_caution: "과열된 조급증, 즉흥적인 투자, 브레이크 없는 돌진으로 인한 소진.".to_string(),
            tactical_remedy: "결정 전 24시간 숙고 루틴 도입, 데이터 검증 필수화, 수분 섭취 및 수변 환경 산책, 흑색·네이비 계열 활용.".to_string(),
        })
    } else if elem_upper.contains("WOOD") || elem_upper.contains("목") {
        Some(SymbolInterpretation {
            engine: "Saju (Four Pillars / BaZi)".to_string(),
            symbol: "Primary Yongshin: Wood (목 / 木用神)".to_string(),
            classical_grounding: "《적천수》: '목은 생명의 발동이며 뻗어나가는 기상이니, 새로운 판을 벌리고 비전을 선포하는 생기다.'".to_string(),
            modern_translation: "신규 비즈니스 런칭, 기획 및 아이디어 제안, 교육 및 인재 육성, 파트너십 구축.".to_string(),
            shadow_and_caution: "방향성 없는 무리한 확장, 감정적 고립, 비전 결핍으로 인한 무기력증.".to_string(),
            tactical_remedy: "아침 시간 적극 활용, 녹색 식물 배치, 새로운 학습 프로젝트 시작, 산책 및 스트레칭.".to_string(),
        })
    } else if elem_upper.contains("FIRE") || elem_upper.contains("화") {
        Some(SymbolInterpretation {
            engine: "Saju (Four Pillars / BaZi)".to_string(),
            symbol: "Primary Yongshin: Fire (화 / 火用神)".to_string(),
            classical_grounding: "《궁통보감》: '화는 밝음과 온기이니, 어둠을 밝혀 가치를 세상에 드러내고 열정을 전파하는 빛이다.'".to_string(),
            modern_translation: "브랜딩, 대중 마케팅, PR 및 스피치, 가치 시각화, 열정적 모멘텀 창출.".to_string(),
            shadow_and_caution: "우울감, 지나친 내향성, 가치를 알리지 않고 혼자 삭이는 폐쇄성.".to_string(),
            tactical_remedy: "햇볕 쬐기, 온열 요법, 자신을 외부에 적극 노출하는 소셜 미디어 활동, 붉은색·와인색 포인트 활용.".to_string(),
        })
    } else if elem_upper.contains("EARTH") || elem_upper.contains("토") {
        Some(SymbolInterpretation {
            engine: "Saju (Four Pillars / BaZi)".to_string(),
            symbol: "Primary Yongshin: Earth (토 / 土用神)".to_string(),
            classical_grounding: "《적천수》: '토는 만물의 터전이자 중재자이니, 사계절의 급격한 변화를 흡수하여 견고한 안정을 준다.'".to_string(),
            modern_translation: "부동산 및 실물 자산 확보, 플랫폼 및 인프라 구축, 신뢰와 중재력, 계약 체계화.".to_string(),
            shadow_and_caution: "감정의 롤러코스터, 급격한 변덕, 기반 없이 붕 떠 있는 상태.".to_string(),
            tactical_remedy: "규칙적인 식습관, 맨땅 걷기(Earthing), 안정적 부동산 자산 포트폴리오 관리, 황토색·베이지 계열 활용.".to_string(),
        })
    } else if elem_upper.contains("METAL") || elem_upper.contains("금") {
        Some(SymbolInterpretation {
            engine: "Saju (Four Pillars / BaZi)".to_string(),
            symbol: "Primary Yongshin: Metal (금 / 金用神)".to_string(),
            classical_grounding: "《궁통보감》: '금은 결실과 엄격한 결단이니, 불필요한 곁가지를 쳐내고 알곡만을 거두는 가을의 서리다.'".to_string(),
            modern_translation: "구조조정, 핵심 역량 집중, 결단력 있는 계약 해지 및 정예화, 금융 및 정밀 공학.".to_string(),
            shadow_and_caution: "미련과 우유부단함, 쓸모없는 인간관계 정리 실패, 분산된 리소스 낭비.".to_string(),
            tactical_remedy: "불필요한 관계 및 짐 과감히 버리기(미니멀리즘), 엄격한 수치 검증, 금속 액세서리 착용, 백색 계열 활용.".to_string(),
        })
    } else {
        None
    }
}
