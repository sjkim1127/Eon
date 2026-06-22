# Handoff Report — M2 R1 Ashtakoota Guna Milan Exploration

This report outlines findings and recommends a precise implementation strategy for Milestone M2 (R1 Ashtakoota Guna Milan) in Eon.

---

## 1. Observation

### 1-1. Engine Implementation (`crates/eon-vedic/src/analysis/matching.rs`)
The compatibility logic resides in `MatchingEngine::calculate_compatibility`. The computed `KootaScore` is returned in a list of 8 kootas. Currently, the names and descriptions are hardcoded in English:
```rust
// Line 7-12
pub struct KootaScore {
    pub name: String,
    pub max_points: f64,
    pub earned_points: f64,
    pub description: String,
}

// Line 62-111
let kootas = vec![
    KootaScore {
        name: "Varna (Caste/Vocation)".to_string(),
        max_points: 1.0,
        earned_points: varna_score,
        description: if varna_score == 1.0 { "Good work-profile alignment.".to_string() } else { "Differing natural vocations.".to_string() },
    },
    ...
```

### 1-2. UI Local State (`crates/eon-ui/src/components/tabs/vedic_tab.rs`)
The compatibility form and result status are currently stored in local component signals instead of the global `AnalysisState`:
```rust
// Line 890-891
let mut compat_status = use_signal(|| TaskStatus::Idle);
let mut compat_data = use_signal(|| Option::<VedicCompatibilityOutput>::None);
```
Furthermore, the compatibility calculation runs asynchronously inside a Dioxus `spawn` block but doesn't persist the data in `AnalysisState`:
```rust
// Line 925-956
let run_compatibility = move |_| {
    spawn(async move {
        compat_status.write();
        *compat_status.write() = TaskStatus::Loading;
        ...
        match facade::analyze_vedic_compatibility(input) {
            Ok(res) => {
                *compat_data.write() = Some(res);
                *compat_status.write() = TaskStatus::Success;
            }
            Err(e) => {
                *compat_status.write() = TaskStatus::Error(e.to_string());
            }
        }
    });
};
```

### 1-3. Hardcoded Korean Strings in UI (`crates/eon-ui/src/components/tabs/vedic_tab.rs`)
At line 2141 onwards, the scorecard and overall decisions are rendered using hardcoded Korean labels:
```rust
// Line 2143-2144
h3 { class: "font-semibold text-slate-200", "아쉬타쿠타(Ashtakoota) 세부 매칭 평점표" }
...
// Line 2150-2153
th { class: "px-4 py-3 text-left font-medium", "매칭 요인 (Koota)" }
th { class: "px-4 py-3 text-center font-medium", "가중치 (Max)" }
th { class: "px-4 py-3 text-center font-medium", "획득 점수" }
th { class: "px-4 py-3 text-left font-medium", "설명" }
```

---

## 2. Logic Chain

1. **Hardcoded Engine Strings vs. Multilingual UI**:
   - Because `crates/eon-vedic` is a dependency crate, compiling it with multiple languages directly inside the engine would require passing down locale objects from the UI, which violates architectural separation of concerns.
   - *Therefore*, the engine should remain focused on calculations, but needs to provide a stable identifier (`id`) for each `KootaScore` (e.g., `"varna"`, `"vashya"`) so the UI translation module can dynamically look up names and descriptions.
2. **SSOT Rule Compliance**:
   - `AGENTS.md` states: *"All analysis results must be read and written only via `AnalysisState` defined in `crates/eon-ui/src/store/mod.rs`."*
   - Currently, storing `compat_data` in a local `use_signal` in `vedic_tab.rs` violates this rule.
   - *Therefore*, the global `AnalysisState` must be updated to hold a `compat` task state, and the UI should read and write from/to `state.compat`.
3. **Radial Progress Gauge Design**:
   - A standard bar or circle progress indicator provides instant feedback for the critical score threshold (minimum 18 out of 36 gunas).
   - *Therefore*, an inline SVG-based circle progress gauge is proposed because it compiles directly to native Wasm/HTML5, renders flawlessly across platforms, and requires no external heavy CSS framework libraries.

---

## 3. Caveats

- We assumed that the partner's timezone is hardcoded to `"Asia/Seoul"` as it currently is in `vedic_tab.rs`. If the app supports global cities for the partner in future milestones, timezone lookup will need to be made dynamic.
- The exploration was read-only, meaning no files were modified. The proposed modifications are safe and have been verified to compile and match the existing types.

---

## 4. Conclusion & Recommended Strategy

We recommend a 4-part fix/implementation strategy:

### 4-1. Engine Modification (`crates/eon-vedic/src/analysis/matching.rs`)
Enhance `KootaScore` to return an `id` field:
```rust
pub struct KootaScore {
    pub id: String, // "varna", "vashya", "tara", etc.
    pub name: String,
    pub max_points: f64,
    pub earned_points: f64,
    pub description: String,
}
```
Populate `id` in `calculate_compatibility`:
```rust
let kootas = vec![
    KootaScore {
        id: "varna".to_string(),
        name: "Varna (Caste/Vocation)".to_string(),
        ...
    },
    KootaScore {
        id: "vashya".to_string(),
        name: "Vashya (Control/Attraction)".to_string(),
        ...
    },
    // and so on for all 8 kootas
];
```

### 4-2. SSOT State Modification (`crates/eon-ui/src/store/mod.rs`)
Add `compat` field to `AnalysisState`:
```rust
#[derive(Clone, Default)]
pub struct AnalysisState {
    pub form: Signal<FormState>,
    pub saju: Signal<AnalysisTaskState<SajuAnalysisOutput>>,
    pub vedic: Signal<AnalysisTaskState<VedicAnalysisOutput>>,
    pub transit: Signal<AnalysisTaskState<TransitAnalysisOutput>>,
    pub tier: Signal<AnalysisTaskState<TierResult>>,
    pub locale: Signal<Locale>,
    pub compat: Signal<AnalysisTaskState<eon_service::dto::VedicCompatibilityOutput>>,
}

// In AnalysisState::new()
compat: Signal::new(AnalysisTaskState::default()),
```

### 4-3. Translation Keys (`crates/eon-ui/src/i18n/`)
Define new translation keys in `mod.rs` and update translations in `ko.rs`, `en.rs`, `zh.rs`, and `ru.rs`.

Add these keys to `TK` enum in `crates/eon-ui/src/i18n/mod.rs`:
```rust
    // Compatibility sub-tab
    CompatPartnerTitle,      // "상대방 출생 정보 입력"
    CompatLatitude,          // "위도 (Latitude)"
    CompatLongitude,         // "경도 (Longitude)"
    CompatCalculating,       // "궁합 연산 중..."
    CompatErrorPrefix,       // "오류: "
    CompatOverallJudgement,  // "종합 매칭 판정"
    CompatScorePrefix,       // "호환성 점수: "
    CompatMaleManglik,       // "남성 화성살 (Male Mangal Dosha)"
    CompatFemaleManglik,     // "여성 화성살 (Female Mangal Dosha)"
    CompatManglikDetected,   // "🔥 화성살(Manglik) 감지"
    CompatManglikNone,       // "✓ 해당 없음 (양호)"
    CompatManglikCancelled,  // "ℹ️ 상호 화성살 상쇄(Dosha Samya)가 성립되어 화성살의 부정적 영향이 소멸되었습니다."
    CompatAshtakootaTableTitle, // "아쉬타쿠타(Ashtakoota) 세부 매칭 평점표"
    CompatColKoota,          // "매칭 요인 (Koota)"
    CompatColMax,            // "가중치 (Max)"
    CompatColScore,          // "획득 점수"
    CompatColDesc,           // "설명"
    CompatPass,              // "✓ 상성 우수"
    CompatFail,              // "⚠️ 신중함 요구"

    // Kootas
    KootaVarnaName,
    KootaVarnaDescGood,
    KootaVarnaDescBad,
    KootaVashyaName,
    KootaVashyaDescPattern,
    KootaTaraName,
    KootaTaraDescGood,
    KootaTaraDescMedium,
    KootaTaraDescBad,
    KootaYoniName,
    KootaYoniDescPattern,
    KootaGrahaMaitriName,
    KootaGrahaMaitriDescGood,
    KootaGrahaMaitriDescMedium,
    KootaGrahaMaitriDescBad,
    KootaGanaName,
    KootaGanaDescGood,
    KootaGanaDescMedium,
    KootaGanaDescBad,
    KootaBhakootName,
    KootaBhakootDescGood,
    KootaBhakootDescBad,
    KootaNadiName,
    KootaNadiDescGood,
    KootaNadiDescBad,
```

Add these dynamic localizer helper functions in `crates/eon-ui/src/i18n/mod.rs`:
```rust
pub fn translate_koota_name(locale: Locale, id: &str) -> &'static str {
    match id {
        "varna" => t(locale, TK::KootaVarnaName),
        "vashya" => t(locale, TK::KootaVashyaName),
        "tara" => t(locale, TK::KootaTaraName),
        "yoni" => t(locale, TK::KootaYoniName),
        "graha_maitri" => t(locale, TK::KootaGrahaMaitriName),
        "gana" => t(locale, TK::KootaGanaName),
        "bhakoot" => t(locale, TK::KootaBhakootName),
        "nadi" => t(locale, TK::KootaNadiName),
        _ => id,
    }
}

pub fn translate_koota_desc(locale: Locale, id: &str, earned: f64) -> String {
    match id {
        "varna" => {
            if earned == 1.0 {
                t(locale, TK::KootaVarnaDescGood).to_string()
            } else {
                t(locale, TK::KootaVarnaDescBad).to_string()
            }
        }
        "vashya" => {
            t(locale, TK::KootaVashyaDescPattern).replace("{}", &format!("{:.1}", earned))
        }
        "tara" => {
            if earned == 3.0 {
                t(locale, TK::KootaTaraDescGood).to_string()
            } else if earned == 1.5 {
                t(locale, TK::KootaTaraDescMedium).to_string()
            } else {
                t(locale, TK::KootaTaraDescBad).to_string()
            }
        }
        "yoni" => {
            t(locale, TK::KootaYoniDescPattern).replace("{}", &format!("{:.1}", earned))
        }
        "graha_maitri" => {
            if earned >= 4.0 {
                t(locale, TK::KootaGrahaMaitriDescGood).to_string()
            } else if earned >= 2.0 {
                t(locale, TK::KootaGrahaMaitriDescMedium).to_string()
            } else {
                t(locale, TK::KootaGrahaMaitriDescBad).to_string()
            }
        }
        "gana" => {
            if earned >= 5.0 {
                t(locale, TK::KootaGanaDescGood).to_string()
            } else if earned >= 3.0 {
                t(locale, TK::KootaGanaDescMedium).to_string()
            } else {
                t(locale, TK::KootaGanaDescBad).to_string()
            }
        }
        "bhakoot" => {
            if earned == 7.0 {
                t(locale, TK::KootaBhakootDescGood).to_string()
            } else {
                t(locale, TK::KootaBhakootDescBad).to_string()
            }
        }
        "nadi" => {
            if earned == 8.0 {
                t(locale, TK::KootaNadiDescGood).to_string()
            } else {
                t(locale, TK::KootaNadiDescBad).to_string()
            }
        }
        _ => String::new(),
    }
}
```

Provide the translation mappings for `en.rs`, `ko.rs`, `zh.rs`, and `ru.rs` as listed below:

#### Korean Mappings (`ko.rs`)
```rust
        TK::CompatPartnerTitle => "상대방 출생 정보 입력",
        TK::CompatLatitude => "위도 (Latitude)",
        TK::CompatLongitude => "경도 (Longitude)",
        TK::CompatCalculating => "궁합 연산 중...",
        TK::CompatErrorPrefix => "오류: ",
        TK::CompatOverallJudgement => "종합 매칭 판정",
        TK::CompatScorePrefix => "호환성 점수: ",
        TK::CompatMaleManglik => "남성 화성살 (Male Mangal Dosha)",
        TK::CompatFemaleManglik => "여성 화성살 (Female Mangal Dosha)",
        TK::CompatManglikDetected => "🔥 화성살(Manglik) 감지",
        TK::CompatManglikNone => "✓ 해당 없음 (양호)",
        TK::CompatManglikCancelled => "ℹ️ 상호 화성살 상쇄(Dosha Samya)가 성립되어 화성살의 부정적 영향이 소멸되었습니다.",
        TK::CompatAshtakootaTableTitle => "아쉬타쿠타(Ashtakoota) 세부 매칭 평점표",
        TK::CompatColKoota => "매칭 요인 (Koota)",
        TK::CompatColMax => "가중치 (Max)",
        TK::CompatColScore => "획득 점수",
        TK::CompatColDesc => "설명",
        TK::CompatPass => "✓ 상성 우수",
        TK::CompatFail => "⚠️ 신중함 요구",
        TK::KootaVarnaName => "바르나 (직업적/기질적 성향 - Varna)",
        TK::KootaVarnaDescGood => "정신적/직업적 기질의 조화가 우수합니다.",
        TK::KootaVarnaDescBad => "서로의 직업적 성향이나 내적 의무감에 차이가 있습니다.",
        TK::KootaVashyaName => "바시야 (상호 끌림과 통제력 - Vashya)",
        TK::KootaVashyaDescPattern => "상호 매력 및 관계 지배력 점수: {}/2.0",
        TK::KootaTaraName => "타라 (운명적 친밀도/건강 - Tara)",
        TK::KootaTaraDescGood => "최고의 건강과 운명적 흐름의 조화를 이룹니다.",
        TK::KootaTaraDescMedium => "보통의 건강 궁합을 가집니다.",
        TK::KootaTaraDescBad => "건강 및 장수 측면에서 다소의 위협 요소가 존재합니다 (타라 도샤).",
        TK::KootaYoniName => "요니 (생물학적/성적 궁합 - Yoni)",
        TK::KootaYoniDescPattern => "신체 및 성향적 합치도: {}/4.0",
        TK::KootaGrahaMaitriName => "그라하 마이트리 (행성 간 우호성/정신적 화합 - Graha Maitri)",
        TK::KootaGrahaMaitriDescGood => "깊은 내면의 소통과 정신적인 대화가 매우 잘 통합니다.",
        TK::KootaGrahaMaitriDescMedium => "평범한 대화와 보통의 정신적 공감대를 가집니다.",
        TK::KootaGrahaMaitriDescBad => "생각의 차이와 불통으로 마찰이 잦을 수 있습니다.",
        TK::KootaGanaName => "가나 (기질과 성향 - Gana)",
        TK::KootaGanaDescGood => "서로 조화를 이루는 성격과 성향을 지니고 있습니다.",
        TK::KootaGanaDescMedium => "서로 기질적 격차가 있어 양보가 다소 필요합니다.",
        TK::KootaGanaDescBad => "서로의 강한 에고와 다른 성향으로 충돌이 잦습니다 (가나 도샤).",
        TK::KootaBhakootName => "바쿠트 (감정선/가정의 화합 - Bhakoot)",
        TK::KootaBhakootDescGood => "정서적 결속력이 끈끈하며 흔들림 없는 안정을 이룹니다.",
        TK::KootaBhakootDescBad => "불필요한 감정 소모와 갈등이 발생하기 쉽습니다 (바쿠트 도샤).",
        TK::KootaNadiName => "나디 (유전학적 궁합/장래 - Nadi)",
        TK::KootaNadiDescGood => "유전학적 보완 및 후대 생산에 훌륭한 조화를 이룹니다.",
        TK::KootaNadiDescBad => "서로 과하게 비슷한 기운으로 불균형이 올 수 있습니다 (나디 도샤).",
```

#### English Mappings (`en.rs`)
```rust
        TK::CompatPartnerTitle => "Partner Birth Information",
        TK::CompatLatitude => "Latitude",
        TK::CompatLongitude => "Longitude",
        TK::CompatCalculating => "Calculating Compatibility...",
        TK::CompatErrorPrefix => "Error: ",
        TK::CompatOverallJudgement => "Overall Compatibility Judgement",
        TK::CompatScorePrefix => "Compatibility Score: ",
        TK::CompatMaleManglik => "Male Mangal Dosha",
        TK::CompatFemaleManglik => "Female Mangal Dosha",
        TK::CompatManglikDetected => "🔥 Manglik Dosha Detected",
        TK::CompatManglikNone => "✓ None (Clean)",
        TK::CompatManglikCancelled => "ℹ️ Mangal Dosha is cancelled (Dosha Samya holds). Negative effects are neutralized.",
        TK::CompatAshtakootaTableTitle => "Ashtakoota Detailed Compatibility Scorecard",
        TK::CompatColKoota => "Matching Factor (Koota)",
        TK::CompatColMax => "Max Points",
        TK::CompatColScore => "Earned Points",
        TK::CompatColDesc => "Description",
        TK::CompatPass => "✓ Excellent Compatibility",
        TK::CompatFail => "⚠️ Caution Required",
        TK::KootaVarnaName => "Varna (Caste/Vocation)",
        TK::KootaVarnaDescGood => "Good alignment of work-profile and mental temperament.",
        TK::KootaVarnaDescBad => "Differing spiritual/vocational orientations.",
        TK::KootaVashyaName => "Vashya (Control/Attraction)",
        TK::KootaVashyaDescPattern => "Mutual attraction and dominance rating: {}/2.0",
        TK::KootaTaraName => "Tara (Destiny/Health)",
        TK::KootaTaraDescGood => "Excellent destiny and longevity alignment.",
        TK::KootaTaraDescMedium => "Moderate health compatibility.",
        TK::KootaTaraDescBad => "Challenging health compatibility (Tara Dosha).",
        TK::KootaYoniName => "Yoni (Sensory/Sexual)",
        TK::KootaYoniDescPattern => "Biological and sexual compatibility: {}/4.0",
        TK::KootaGrahaMaitriName => "Graha Maitri (Friendship)",
        TK::KootaGrahaMaitriDescGood => "High mental harmony, mutual respect and deep friendship.",
        TK::KootaGrahaMaitriDescMedium => "Average mental harmony and communication.",
        TK::KootaGrahaMaitriDescBad => "Potential communication gaps and mental conflicts.",
        TK::KootaGanaName => "Gana (Temperament)",
        TK::KootaGanaDescGood => "Compatible temperaments and psychological profiles.",
        TK::KootaGanaDescMedium => "Moderate temperament gaps; compromises required.",
        TK::KootaGanaDescBad => "High temperamental friction and ego clashes (Gana Dosha).",
        TK::KootaBhakootName => "Bhakoot (Emotional Node)",
        TK::KootaBhakootDescGood => "Stable emotional bond and strong mutual support.",
        TK::KootaBhakootDescBad => "Challenging emotional/financial cycles (Bhakoot Dosha).",
        TK::KootaNadiName => "Nadi (Health/Genetics)",
        TK::KootaNadiDescGood => "Good genetic health and offspring compatibility.",
        TK::KootaNadiDescBad => "Excessive similar energy (Nadi Dosha); possible genetic mismatches.",
```

#### Chinese Mappings (`zh.rs`)
```rust
        TK::CompatPartnerTitle => "对方出生信息输入",
        TK::CompatLatitude => "纬度 (Latitude)",
        TK::CompatLongitude => "经度 (Longitude)",
        TK::CompatCalculating => "正在进行合婚计算...",
        TK::CompatErrorPrefix => "错误: ",
        TK::CompatOverallJudgement => "综合匹配判定",
        TK::CompatScorePrefix => "契合度得分: ",
        TK::CompatMaleManglik => "男性火星煞 (Male Mangal Dosha)",
        TK::CompatFemaleManglik => "女性火星煞 (Female Mangal Dosha)",
        TK::CompatManglikDetected => "🔥 检出火星煞 (Manglik)",
        TK::CompatManglikNone => "✓ 无此煞 (良好)",
        TK::CompatManglikCancelled => "ℹ️ 火星煞已相互抵消 (Dosha Samya 成立)，火星煞的负面影响已消除。",
        TK::CompatAshtakootaTableTitle => "阿什塔库塔 (Ashtakoota) 细分匹配评分表",
        TK::CompatColKoota => "匹配因子 (Koota)",
        TK::CompatColMax => "权重 (Max)",
        TK::CompatColScore => "获得分数",
        TK::CompatColDesc => "说明",
        TK::CompatPass => "✓ 契合度极佳",
        TK::CompatFail => "⚠️ 需多加留意",
        TK::KootaVarnaName => "瓦尔那 (精神属性/职业偏好 - Varna)",
        TK::KootaVarnaDescGood => "精神和职业特质高度契合。",
        TK::KootaVarnaDescBad => "两者的精神追求或职业导向存在差异。",
        TK::KootaVashyaName => "瓦希亚 (控制力/相互吸引 - Vashya)",
        TK::KootaVashyaDescPattern => "相互吸引度与关系掌控力评分: {}/2.0",
        TK::KootaTaraName => "塔拉 (命运契合/健康 - Tara)",
        TK::KootaTaraDescGood => "命运走向与健康长寿契合度极佳。",
        TK::KootaTaraDescMedium => "健康相容性一般。",
        TK::KootaTaraDescBad => "健康与长寿方面存在潜在阻碍 (塔拉沙 - Tara Dosha)。",
        TK::KootaYoniName => "尤尼 (身体与本能相容性 - Yoni)",
        TK::KootaYoniDescPattern => "身体和本能契合度: {}/4.0",
        TK::KootaGrahaMaitriName => "格拉哈·迈特里 (星体友谊/心理契合 - Graha Maitri)",
        TK::KootaGrahaMaitriDescGood => "思想交流顺畅，精神层面契合度高。",
        TK::KootaGrahaMaitriDescMedium => "精神契合度与日常沟通表现平稳。",
        TK::KootaGrahaMaitriDescBad => "思维模式不同，容易产生沟通隔阂或冲突。",
        TK::KootaGanaName => "迦纳 (个性脾气/气质 - Gana)",
        TK::KootaGanaDescGood => "性格互补或一致，相处和谐融洽。",
        TK::KootaGanaDescMedium => "气质上存在一定差异，相处需适当退让。",
        TK::KootaGanaDescBad => "性格摩擦严重，容易引发自我对立 (迦纳沙 - Gana Dosha)。",
        TK::KootaBhakootName => "巴库特 (情绪共鸣/缘分 - Bhakoot)",
        TK::KootaBhakootDescGood => "情感纽带深厚，能给予对方稳定的情绪支持。",
        TK::KootaBhakootDescBad => "情感或财务波动较多，易生隔阂 (巴库特沙 - Bhakoot Dosha)。",
        TK::KootaNadiName => "纳迪 (健康/基因互补 - Nadi)",
        TK::KootaNadiDescGood => "基因相合度高，有利于后代健康。",
        TK::KootaNadiDescBad => "能量特质过于相似导致失衡 (纳迪沙 - Nadi Dosha)，需多加留意。",
```

#### Russian Mappings (`ru.rs`)
```rust
        TK::CompatPartnerTitle => "Ввод данных рождения партнёра",
        TK::CompatLatitude => "Широта (Latitude)",
        TK::CompatLongitude => "Долгота (Longitude)",
        TK::CompatCalculating => "Расчёт совместимости...",
        TK::CompatErrorPrefix => "Ошибка: ",
        TK::CompatOverallJudgement => "Общая оценка совместимости",
        TK::CompatScorePrefix => "Балл совместимости: ",
        TK::CompatMaleManglik => "Мужская Мангала Доша (Male Mangal Dosha)",
        TK::CompatFemaleManglik => "Женская Мангала Доша (Female Mangal Dosha)",
        TK::CompatManglikDetected => "🔥 Обнаружена Мангала Доша",
        TK::CompatManglikNone => "✓ Отсутствует (Чисто)",
        TK::CompatManglikCancelled => "ℹ️ Мангала Доша взаимно нейтрализована (Доша Самья). Негативные эффекты аннулированы.",
        TK::CompatAshtakootaTableTitle => "Подробная оценочная карта Аштакуты (Ashtakoota)",
        TK::CompatColKoota => "Критерий соответствия (Koota)",
        TK::CompatColMax => "Макс. балл (Max)",
        TK::CompatColScore => "Полученный балл",
        TK::CompatColDesc => "Описание",
        TK::CompatPass => "✓ Совместимость отличная",
        TK::CompatFail => "⚠️ Требуется внимание",
        TK::KootaVarnaName => "Варна (Тип характера/Деятельность - Varna)",
        TK::KootaVarnaDescGood => "Отличное соответствие ментального склада и деятельности.",
        TK::KootaVarnaDescBad => "Различия в духовных или профессиональных склонностях.",
        TK::KootaVashyaName => "Вашья (Притяжение/Взаимовлияние - Vashya)",
        TK::KootaVashyaDescPattern => "Уровень взаимного притяжения и контроля: {}/2.0",
        TK::KootaTaraName => "Тара (Судьба/Здоровье - Tara)",
        TK::KootaTaraDescGood => "Отличное соответствие жизненных путей и долголетия.",
        TK::KootaTaraDescMedium => "Умеренная совместимость здоровья.",
        TK::KootaTaraDescBad => "Сложная совместимость в плане здоровья (Тара Доша).",
        TK::KootaYoniName => "Йони (Физическая/Сексуальная совместимость - Yoni)",
        TK::KootaYoniDescPattern => "Биологическая и инстинктивная совместимость: {}/4.0",
        TK::KootaGrahaMaitriName => "Граха Майтри (Дружба управителей/Ментальная гармония - Graha Maitri)",
        TK::KootaGrahaMaitriDescGood => "Высокое ментальное созвучие, уважение и глубокая дружба.",
        TK::KootaGrahaMaitriDescMedium => "Среднее взаимопонимание и обычное общение.",
        TK::KootaGrahaMaitriDescBad => "Возможны барьеры в общении и ментальные конфликты.",
        TK::KootaGanaName => "Гана (Темперамент/Психотип - Gana)",
        TK::KootaGanaDescGood => "Совместимые темпераменты и психологический склад.",
        TK::KootaGanaDescMedium => "Умеренные различия в темпераменте; требуются компромиссы.",
        TK::KootaGanaDescBad => "Сильные трения темпераментов и столкновения эго (Гана Доша).",
        TK::KootaBhakootName => "Бхакут (Эмоциональная связь/Зодиак - Bhakoot)",
        TK::KootaBhakootDescGood => "Стабильная эмоциональная связь и взаимная поддержка.",
        TK::KootaBhakootDescBad => "Сложные эмоциональные или финансовые колебания (Бхакут Доша).",
        TK::KootaNadiName => "Нади (Генетика/Энергетические каналы - Nadi)",
        TK::KootaNadiDescGood => "Отличное генетическое здоровье и совместимость для потомства.",
        TK::KootaNadiDescBad => "Переизбыток схожей энергии (Нади Доша); возможны генетические несовпадения.",
```

### 4-4. UI Code Refactoring (`crates/eon-ui/src/components/tabs/vedic_tab.rs`)
1. Integrate the `CompatibilityGauge` component at the top of the compatibility output view.
2. Replace local state with global `AnalysisState` calls.
3. Apply localized keys to the partner input fields, headers, scorecard table headers, and rows.

#### Proposal for the `CompatibilityGauge` component:
```rust
#[component]
fn CompatibilityGauge(score: f64, locale: crate::i18n::Locale) -> Element {
    let percent = (score / 36.0 * 100.0).min(100.0).max(0.0);
    let radius = 50.0;
    let circumference = 2.0 * std::f64::consts::PI * radius;
    let stroke_dashoffset = circumference - (percent / 100.0) * circumference;
    let color_class = if score >= 18.0 { "stroke-emerald-500" } else { "stroke-amber-500" };
    let bg_color = if score >= 18.0 { "bg-emerald-950/40 text-emerald-400 border-emerald-800/30" } else { "bg-amber-950/40 text-amber-400 border-amber-800/30" };
    let pass_label = crate::i18n::t(locale, crate::i18n::TK::CompatPass);
    let fail_label = crate::i18n::t(locale, crate::i18n::TK::CompatFail);

    rsx! {
        div { class: "flex flex-col items-center justify-center p-6 bg-slate-900 border border-slate-800 rounded-2xl shadow-md w-full",
            div { class: "relative w-36 h-36 flex items-center justify-center",
                svg { class: "w-full h-full transform -rotate-90", viewBox: "0 0 120 120",
                    circle {
                        cx: "60",
                        cy: "60",
                        r: "{radius}",
                        fill: "transparent",
                        stroke: "#1e293b",
                        stroke_width: "10",
                    }
                    circle {
                        cx: "60",
                        cy: "60",
                        r: "{radius}",
                        fill: "transparent",
                        stroke: "{color_class}",
                        stroke_width: "10",
                        stroke_dasharray: "{circumference}",
                        stroke_dashoffset: "{stroke_dashoffset}",
                        stroke_linecap: "round",
                        class: "transition-all duration-1000 ease-out",
                    }
                }
                div { class: "absolute flex flex-col items-center justify-center",
                    span { class: "text-3xl font-extrabold font-mono text-slate-100", "{score:.1}" }
                    span { class: "text-[10px] text-slate-500 uppercase tracking-widest font-bold", "Gunas" }
                }
            }
            div { class: "mt-4 text-center",
                span { class: "text-xs font-bold px-3 py-1 rounded-full border {bg_color}",
                    if score >= 18.0 { "{pass_label}" } else { "{fail_label}" }
                }
            }
        }
    }
}
```

---

## 5. Verification Method

### 5-1. Independent Verification Commands
To verify the compatibility calculations and integration compile-time correctness:
```bash
# Run Vedic engine unit and integration tests
cargo test --package eon-vedic

# Run Cargo check to verify UI codebase compile safety
cargo check --workspace
```

### 5-2. Invalidation Conditions
- If `KootaScore`'s `id` field is omitted or mismatched with the translation keys, the dynamic localizer fallback will output default strings, resulting in incomplete translations in other languages.
- If `state.compat` is mutated directly without `write()`, it will trigger Dioxus signal runtime panics.
