use crate::dto::SymbolInterpretation;

pub fn interpret_major_star(star_name: &str) -> Option<SymbolInterpretation> {
    let name_lower = star_name.to_lowercase();
    if name_lower.contains("pojun") || name_lower.contains("파군") {
        Some(SymbolInterpretation {
            engine: "Zi Wei Dou Shu (Purple Star Astrology)".to_string(),
            symbol: "Major Star: PoJun (파군성 / 破軍星)".to_string(),
            classical_grounding: "《자미두수 전서(紫微斗數全書)》: '파군은 북두 제7성으로 소모와 개척의 신(耗星)이니, 옛 것을 파괴하고 새로운 판을 짠다.'".to_string(),
            modern_translation: "파괴적 혁신가, 판을 흔드는 체인저(Game Changer), 정체된 시장을 허물고 0에서 1을 만들어내는 개척자.".to_string(),
            shadow_and_caution: "안정된 것을 오래 유지하지 못하고 스스로 뒤엎어버리는 탕진과 반골 기질, 인내심 부족.".to_string(),
            tactical_remedy: "파괴의 에너지를 자기 인생이나 관계가 아닌 '외부 시장의 모순과 기술적 난제'를 깨부수는 데 집중할 것.".to_string(),
        })
    } else if name_lower.contains("qisha") || name_lower.contains("칠살") {
        Some(SymbolInterpretation {
            engine: "Zi Wei Dou Shu (Purple Star Astrology)".to_string(),
            symbol: "Major Star: QiSha (칠살성 / 七殺星)".to_string(),
            classical_grounding: "《자미두수 전서》: '칠살은 남두 제6성으로 장수의 권위이니 독단적으로 결단을 내리고 일사분란하게 진두지휘한다.'".to_string(),
            modern_translation: "강력한 총괄 지휘관, 결단력 있는 최고 경영자(CEO), 홀로 책임을 지고 전선을 돌파하는 야전사령관.".to_string(),
            shadow_and_caution: "독재와 불통, 실패를 용납하지 못하는 냉혹함, 주변 사람들을 지치게 만드는 과도한 긴장감.".to_string(),
            tactical_remedy: "부드러운 소통과 정서적 공감 능력을 의식적으로 훈련하고, 참모진의 직언을 수용하는 창구를 열어둘 것.".to_string(),
        })
    } else if name_lower.contains("ziwei") || name_lower.contains("자미") {
        Some(SymbolInterpretation {
            engine: "Zi Wei Dou Shu (Purple Star Astrology)".to_string(),
            symbol: "Major Star: ZiWei (자미성 / 紫微星)".to_string(),
            classical_grounding: "《자미두수 전서》: '자미는 북두의 제왕이니 만성을 통솔하고 존귀함과 리더십의 극치를 상징한다.'".to_string(),
            modern_translation: "최고 권위자, 품격 있는 브랜드 리더, 거대한 조직을 품고 방향성을 제시하는 통합형 군주.".to_string(),
            shadow_and_caution: "귀가 얇아 간신의 아첨에 휘둘릴 위험, 체면에 집착하여 실리를 잃는 허세, 실무 디테일 경시.".to_string(),
            tactical_remedy: "좌보(左輔)·우필(右弼)과 같은 유능한 실무 참모진을 반드시 양옆에 두고 직언에 귀를 기울일 것.".to_string(),
        })
    } else if name_lower.contains("wuqu") || name_lower.contains("무곡") {
        Some(SymbolInterpretation {
            engine: "Zi Wei Dou Shu (Purple Star Astrology)".to_string(),
            symbol: "Major Star: WuQu (무곡성 / 武曲星)".to_string(),
            classical_grounding: "《자미두수 전서》: '무곡은 재백(財帛)의 주인이니 과단성 있고 실속이 차있으며 차가운 쇠의 정밀함이다.'".to_string(),
            modern_translation: "금융 및 자산 운용의 프로페셔널, 치밀한 수치 기반 경영자, 결단력 있는 실속파 자산가.".to_string(),
            shadow_and_caution: "인정머리 없음, 지나치게 계산적인 인간관계로 인한 고독감, 유연성 부족.".to_string(),
            tactical_remedy: "수치적 계산 외에 인간적인 온기와 정서적 유대를 나누는 여백을 일상에 들일 것.".to_string(),
        })
    } else if name_lower.contains("taiyang") || name_lower.contains("태양") {
        Some(SymbolInterpretation {
            engine: "Zi Wei Dou Shu (Purple Star Astrology)".to_string(),
            symbol: "Major Star: TaiYang (태양성 / 太陽星)".to_string(),
            classical_grounding: "《자미두수 전서》: '태양은 중천의 주인이니 밝음과 공명정대함을 관장하고 만인에게 빛을 베푼다.'".to_string(),
            modern_translation: "공익적 대의를 품은 글로벌 리더, 대중적 스피커 및 인플루언서, 해외 비즈니스 및 공공 사업의 구심점.".to_string(),
            shadow_and_caution: "남 좋은 일만 시켜주고 실속을 챙기지 못하는 번아웃, 과시욕, 남의 일에 오지랖을 부리다 지치는 성향.".to_string(),
            tactical_remedy: "빛을 비추되 자신의 그릇과 실속을 먼저 채우고, 계약과 보상 체계를 명확히 한 후 공익에 기여할 것.".to_string(),
        })
    } else if name_lower.contains("taiyin") || name_lower.contains("태음") {
        Some(SymbolInterpretation {
            engine: "Zi Wei Dou Shu (Purple Star Astrology)".to_string(),
            symbol: "Major Star: TaiYin (태음성 / 太陰星)".to_string(),
            classical_grounding: "《자미두수 전서》: '태음은 은은한 달빛이니 내적인 부와 우아함, 부동산과 축적된 자산을 상징한다.'".to_string(),
            modern_translation: "치밀한 기획가, 부동산 및 실물 자산가, 심미안과 감수성을 갖춘 소프트파워 기획자.".to_string(),
            shadow_and_caution: "우울감, 상처받기 쉬운 지나친 예민함, 직접 맞서지 못하고 속으로 삭이다가 관계를 차단하는 회피성.".to_string(),
            tactical_remedy: "햇볕을 자주 쬐고 정서적 불안을 안정적인 고정 자산 매입과 창작 활동으로 승화시킬 것.".to_string(),
        })
    } else if name_lower.contains("tianfu") || name_lower.contains("천부") {
        Some(SymbolInterpretation {
            engine: "Zi Wei Dou Shu (Purple Star Astrology)".to_string(),
            symbol: "Major Star: TianFu (천부성 / 天府星)".to_string(),
            classical_grounding: "《자미두수 전서》: '천부는 남두의 주인이자 천상의 곳간이니, 보수적이고 안정적이며 부를 지키는 힘이다.'".to_string(),
            modern_translation: "대기업 총괄 수석, 국고와 곳간을 지키는 CFO, 위기 속에서도 자산을 절대 잃지 않는 방어의 귀재.".to_string(),
            shadow_and_caution: "변화에 대한 두려움, 모험 회피, 지나치게 안전한 길만을 고집하여 시대의 대전환을 놓치는 보수성.".to_string(),
            tactical_remedy: "곳간의 일부를 혁신적인 미래 벤처에 투자하여 방어력과 공격력의 균형을 맞출 것.".to_string(),
        })
    } else if name_lower.contains("tanlang") || name_lower.contains("탐랑") {
        Some(SymbolInterpretation {
            engine: "Zi Wei Dou Shu (Purple Star Astrology)".to_string(),
            symbol: "Major Star: TanLang (탐랑성 / 貪狼星)".to_string(),
            classical_grounding: "《자미두수 전서》: '탐랑은 욕망과 도화의 별이니 다재다능하고 사교성이 뛰어나며 기회를 포착하는 감각이 탁월하다.'".to_string(),
            modern_translation: "매력 자본의 소유자, 엔터테인먼트 및 라이프스타일 비즈니스의 승부사, 트렌드를 선점하는 네트워커.".to_string(),
            shadow_and_caution: "욕망의 분산으로 인한 집중력 결여, 유흥과 쾌락으로 인한 재정 누수, 피상적 관계 과다.".to_string(),
            tactical_remedy: "매력과 사교력을 명확한 단 하나의 핵심 비즈니스 모델로 집중시키고 명상이나 영적 수련으로 절제할 것.".to_string(),
        })
    } else {
        None
    }
}
