# Proposed Code Changes for Ashtakoota Guna Milan (Milestone M2 R1)

This document contains the exact proposed changes to enhance the Vedic compatibility engine, UI localization, and render a new progress/gauge component.

---

## 1. Engine Modifications

### Target File: `crates/eon-vedic/src/analysis/matching.rs`

Modify the `KootaScore` struct to include a machine-readable `id: String` field, and populate it when constructing the `kootas` list in `MatchingEngine::calculate_compatibility`.

#### Structure change:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KootaScore {
    pub id: String, // New field: machine-readable identifier
    pub name: String,
    pub max_points: f64,
    pub earned_points: f64,
    pub description: String,
}
```

#### Vector assembly change:
```rust
        let kootas = vec![
            KootaScore {
                id: "varna".to_string(),
                name: "Varna (Caste/Vocation)".to_string(),
                max_points: 1.0,
                earned_points: varna_score,
                description: if varna_score == 1.0 { "Good work-profile alignment.".to_string() } else { "Differing natural vocations.".to_string() },
            },
            KootaScore {
                id: "vashya".to_string(),
                name: "Vashya (Control/Attraction)".to_string(),
                max_points: 2.0,
                earned_points: vashya_score,
                description: format!("Mutual attraction rating: {}/2.", vashya_score),
            },
            KootaScore {
                id: "tara".to_string(),
                name: "Tara (Destiny/Health)".to_string(),
                max_points: 3.0,
                earned_points: tara_score,
                description: if tara_score == 3.0 { "Excellent destiny and longevity alignment.".to_string() } else if tara_score == 1.5 { "Moderate health compatibility.".to_string() } else { "Challenging health compatibility (Tara Dosha).".to_string() },
            },
            KootaScore {
                id: "yoni".to_string(),
                name: "Yoni (Sensory/Sexual)".to_string(),
                max_points: 4.0,
                earned_points: yoni_score,
                description: format!("Biological compatibility: {}/4.", yoni_score),
            },
            KootaScore {
                id: "graha_maitri".to_string(),
                name: "Graha Maitri (Friendship)".to_string(),
                max_points: 5.0,
                earned_points: graha_maitri_score,
                description: if graha_maitri_score >= 4.0 { "High mental harmony and friendship.".to_string() } else if graha_maitri_score >= 2.0 { "Average mental harmony.".to_string() } else { "Potential communication gaps.".to_string() },
            },
            KootaScore {
                id: "gana".to_string(),
                name: "Gana (Temperament)".to_string(),
                max_points: 6.0,
                earned_points: gana_score,
                description: if gana_score >= 5.0 { "Compatible temperaments.".to_string() } else if gana_score >= 3.0 { "Moderate temperament gaps.".to_string() } else { "High temperamental friction (Gana Dosha).".to_string() },
            },
            KootaScore {
                id: "bhakoot".to_string(),
                name: "Bhakoot (Emotional Node)".to_string(),
                max_points: 7.0,
                earned_points: bhakoot_score,
                description: if bhakoot_score == 7.0 { "Stable emotional bond.".to_string() } else { "Challenging emotional/financial cycles (Bhakoot Dosha).".to_string() },
            },
            KootaScore {
                id: "nadi".to_string(),
                name: "Nadi (Health/Genetics)".to_string(),
                max_points: 8.0,
                earned_points: nadi_score,
                description: if nadi_score == 8.0 { "Good genetic health & lineage compatibility.".to_string() } else { "Excessive similar energy (Nadi Dosha). Possible genetic mismatches.".to_string() },
            },
        ];
```

---

## 2. Localization Additions

### Target File: `crates/eon-ui/src/i18n/mod.rs`

Add the new translation keys in `pub enum TK`:

```rust
    // ... Existing keys ...

    // ── Vedic Compatibility ──────────────────────────────
    CompatTitleInput,
    CompatBtnRun,
    CompatStatusLoading,
    CompatHeaderOverall,
    CompatScoreLabel,
    CompatIsCompatibleGood,
    CompatIsCompatibleCaution,
    CompatMaleMangalDosha,
    CompatFemaleMangalDosha,
    CompatMangalDetected,
    CompatMangalNotDetected,
    CompatMangalCancelled,
    CompatAshtakootaTableTitle,
    CompatTableColKoota,
    CompatTableColMax,
    CompatTableColEarned,
    CompatTableColDesc,
    CompatExplanationGood,
    CompatExplanationWarning,
    CompatExplanationBad,
    CompatExplanationSummary,

    // 8 Kootas Names
    KootaVarnaName,
    KootaVashyaName,
    KootaTaraName,
    KootaYoniName,
    KootaGrahaMaitriName,
    KootaGanaName,
    KootaBhakootName,
    KootaNadiName,

    // 8 Kootas Descriptions
    KootaVarnaDescGood,
    KootaVarnaDescBad,
    KootaVashyaDescPattern,
    KootaTaraDescGood,
    KootaTaraDescOk,
    KootaTaraDescBad,
    KootaYoniDescPattern,
    KootaGrahaMaitriDescGood,
    KootaGrahaMaitriDescOk,
    KootaGrahaMaitriDescBad,
    KootaGanaDescGood,
    KootaGanaDescOk,
    KootaGanaDescBad,
    KootaBhakootDescGood,
    KootaBhakootDescBad,
    KootaNadiDescGood,
    KootaNadiDescBad,
```

---

### Target Files: `crates/eon-ui/src/i18n/{ko.rs, en.rs, zh.rs, ru.rs}`

Implement the translations for the newly added keys in each locale's `translate(key: TK)` function.

#### 1) Korean (`ko.rs`)
```rust
        TK::CompatTitleInput => "상대방 출생 정보 입력",
        TK::CompatBtnRun => "💞 궁합 분석 실행",
        TK::CompatStatusLoading => "궁합 연산 중...",
        TK::CompatHeaderOverall => "종합 매칭 판정",
        TK::CompatScoreLabel => "호환성 점수",
        TK::CompatIsCompatibleGood => "✓ 상성 우수",
        TK::CompatIsCompatibleCaution => "⚠️ 신중함 요구",
        TK::CompatMaleMangalDosha => "남성 화성살 (Male Mangal Dosha)",
        TK::CompatFemaleMangalDosha => "여성 화성살 (Female Mangal Dosha)",
        TK::CompatMangalDetected => "🔥 화성살(Manglik) 감지",
        TK::CompatMangalNotDetected => "✓ 해당 없음 (양호)",
        TK::CompatMangalCancelled => "ℹ️ 상호 화성살 상쇄(Dosha Samya)가 성립되어 화성살의 부정적 영향이 소멸되었습니다.",
        TK::CompatAshtakootaTableTitle => "아쉬타쿠타(Ashtakoota) 세부 매칭 평점표",
        TK::CompatTableColKoota => "매칭 요인 (Koota)",
        TK::CompatTableColMax => "가중치 (Max)",
        TK::CompatTableColEarned => "획득 점수",
        TK::CompatTableColDesc => "설명",
        TK::CompatExplanationGood => "전반적으로 조화로운 매칭입니다. 추천합니다.",
        TK::CompatExplanationWarning => "점수는 높으나 주요 살(Nadi/Bhakoot Dosha)의 영향으로 신중한 주의가 필요합니다.",
        TK::CompatExplanationBad => "성향적 차이가 커 상호 조율과 깊은 이해가 요구되는 상성입니다.",
        TK::CompatExplanationSummary => "총 {}점 획득 (36점 만점).",

        // Koota Names
        TK::KootaVarnaName => "바르나 (Varna - 직업/사회활동)",
        TK::KootaVashyaName => "바시야 (Vashya - 성적 매력/통제)",
        TK::KootaTaraName => "타라 (Tara - 건강/운명적 수명)",
        TK::KootaYoniName => "요니 (Yoni - 본능/생리적 궁합)",
        TK::KootaGrahaMaitriName => "그라하 마이트리 (Graha Maitri - 우정/정신적 조화)",
        TK::KootaGanaName => "가나 (Gana - 기질/성격 유형)",
        TK::KootaBhakootName => "바쿠트 (Bhakoot - 감정/정서적 유대)",
        TK::KootaNadiName => "나디 (Nadi - 체질/유전적 조화)",

        // Koota Descriptions
        TK::KootaVarnaDescGood => "직업 및 사회 활동 성향이 잘 조화됩니다.",
        TK::KootaVarnaDescBad => "사회 활동에서의 지향성과 성향 차이가 있습니다.",
        TK::KootaVashyaDescPattern => "상호 호감 및 끌림 강도: {}/2.",
        TK::KootaTaraDescGood => "운명적 수명과 건강운이 매우 조화롭습니다.",
        TK::KootaTaraDescOk => "건강 및 컨디션 관리에 무난한 상성입니다.",
        TK::KootaTaraDescBad => "건강운 상의 마찰이 있을 수 있습니다 (타라 도샤).",
        TK::KootaYoniDescPattern => "생체 활성 및 본능적 화합도: {}/4.",
        TK::KootaGrahaMaitriDescGood => "정신적인 화합과 깊은 우정이 성립됩니다.",
        TK::KootaGrahaMaitriDescOk => "무난하고 평범한 의사소통 수준입니다.",
        TK::KootaGrahaMaitriDescBad => "의사소통에서의 오해나 가치관 차이가 있을 수 있습니다.",
        TK::KootaGanaDescGood => "비슷하고 조화로운 내면의 기질을 가지고 있습니다.",
        TK::KootaGanaDescOk => "기질적 차이가 있으나 조율 가능한 수준입니다.",
        TK::KootaGanaDescBad => "강한 성격적 충돌이나 가치관 대립이 우려됩니다 (가나 도샤).",
        TK::KootaBhakootDescGood => "안정적이고 견고한 정서적 유대가 형성됩니다.",
        TK::KootaBhakootDescBad => "기복 있는 감정 흐름이나 재정적 불안정을 초래할 수 있습니다 (바쿠트 도샤).",
        TK::KootaNadiDescGood => "체질적 궁합이 우수하고 자손 대의 건강한 흐름이 기대됩니다.",
        TK::KootaNadiDescBad => "체질적 에너지가 한쪽으로 치우쳐 마찰이 발생할 수 있습니다 (나디 도샤).",
```

#### 2) English (`en.rs`)
```rust
        TK::CompatTitleInput => "Partner Birth Information",
        TK::CompatBtnRun => "💞 Run Compatibility Analysis",
        TK::CompatStatusLoading => "Calculating compatibility...",
        TK::CompatHeaderOverall => "Overall Match Verdict",
        TK::CompatScoreLabel => "Compatibility Score",
        TK::CompatIsCompatibleGood => "✓ Excellent Match",
        TK::CompatIsCompatibleCaution => "⚠️ Caution Advised",
        TK::CompatMaleMangalDosha => "Male Mangal Dosha",
        TK::CompatFemaleMangalDosha => "Female Mangal Dosha",
        TK::CompatMangalDetected => "🔥 Mangal Dosha (Manglik) Detected",
        TK::CompatMangalNotDetected => "✓ Clear (Auspicious)",
        TK::CompatMangalCancelled => "ℹ️ Mutual cancellation (Dosha Samya) has occurred, neutralizing the negative effects.",
        TK::CompatAshtakootaTableTitle => "Ashtakoota Detailed Compatibility Scorecard",
        TK::CompatTableColKoota => "Factor (Koota)",
        TK::CompatTableColMax => "Max Gunas",
        TK::CompatTableColEarned => "Earned Score",
        TK::CompatTableColDesc => "Description",
        TK::CompatExplanationGood => "Overall a highly harmonious and compatible match. Recommended.",
        TK::CompatExplanationWarning => "Score is high, but caution is advised due to critical Dosha (Nadi/Bhakoot).",
        TK::CompatExplanationBad => "Significant temperamental differences. Mutual adjustment and deep understanding are required.",
        TK::CompatExplanationSummary => "Total Score: {}/36 Gunas.",

        // Koota Names
        TK::KootaVarnaName => "Varna (Caste/Vocation)",
        TK::KootaVashyaName => "Vashya (Control/Attraction)",
        TK::KootaTaraName => "Tara (Destiny/Health)",
        TK::KootaYoniName => "Yoni (Sensory/Sexual)",
        TK::KootaGrahaMaitriName => "Graha Maitri (Friendship/Mental)",
        TK::KootaGanaName => "Gana (Temperament/Behavior)",
        TK::KootaBhakootName => "Bhakoot (Emotional Bond)",
        TK::KootaNadiName => "Nadi (Health/Genetics)",

        // Koota Descriptions
        TK::KootaVarnaDescGood => "Good work-profile and vocational alignment.",
        TK::KootaVarnaDescBad => "Differing natural vocations and societal outlook.",
        TK::KootaVashyaDescPattern => "Mutual attraction rating: {}/2.",
        TK::KootaTaraDescGood => "Excellent destiny and longevity alignment.",
        TK::KootaTaraDescOk => "Moderate health compatibility.",
        TK::KootaTaraDescBad => "Challenging health compatibility (Tara Dosha).",
        TK::KootaYoniDescPattern => "Biological compatibility: {}/4.",
        TK::KootaGrahaMaitriDescGood => "High mental harmony and friendship.",
        TK::KootaGrahaMaitriDescOk => "Average mental harmony and rapport.",
        TK::KootaGrahaMaitriDescBad => "Potential communication and perspective gaps.",
        TK::KootaGanaDescGood => "Compatible temperaments and behaviors.",
        TK::KootaGanaDescOk => "Moderate temperament gaps, manageable.",
        TK::KootaGanaDescBad => "High temperamental friction (Gana Dosha).",
        TK::KootaBhakootDescGood => "Stable and supportive emotional bond.",
        TK::KootaBhakootDescBad => "Challenging emotional/financial cycles (Bhakoot Dosha).",
        TK::KootaNadiDescGood => "Good genetic health & lineage compatibility.",
        TK::KootaNadiDescBad => "Excessive similar energy (Nadi Dosha). Possible genetic mismatches.",
```

#### 3) Chinese (`zh.rs`)
```rust
        TK::CompatTitleInput => "输入对方出生信息",
        TK::CompatBtnRun => "💞 开始合婚分析",
        TK::CompatStatusLoading => "正在进行合婚计算...",
        TK::CompatHeaderOverall => "综合匹配判定",
        TK::CompatScoreLabel => "契合度分数",
        TK::CompatIsCompatibleGood => "✓ 契合度优秀",
        TK::CompatIsCompatibleCaution => "⚠️ 需谨慎对待",
        TK::CompatMaleMangalDosha => "男性火星煞 (Male Mangal Dosha)",
        TK::CompatFemaleMangalDosha => "女性火星煞 (Female Mangal Dosha)",
        TK::CompatMangalDetected => "🔥 检测到火星煞 (Manglik)",
        TK::CompatMangalNotDetected => "✓ 无煞（良好）",
        TK::CompatMangalCancelled => "ℹ️ 双方均有火星煞（煞照抵消），负面影响已消除。",
        TK::CompatAshtakootaTableTitle => "八大契合度（Ashtakoota）详细得分表",
        TK::CompatTableColKoota => "契合要素 (Koota)",
        TK::CompatTableColMax => "权重分 (Max)",
        TK::CompatTableColEarned => "获得分数",
        TK::CompatTableColDesc => "详细描述",
        TK::CompatExplanationGood => "整体非常和谐契合，值得推荐。",
        TK::CompatExplanationWarning => "虽然得分较高，但受到关键煞气（Nadi/Bhakoot Dosha）影响，建议保持谨慎。",
        TK::CompatExplanationBad => "性格和观念差异较大，需要双方深度的磨合与理解。",
        TK::CompatExplanationSummary => "共获得 {} 分（满分 36 分）。",

        // Koota Names
        TK::KootaVarnaName => "巴纳 (Varna - 社会角色/职业倾向)",
        TK::KootaVashyaName => "瓦夏 (Vashya - 控制力与相互吸引)",
        TK::KootaTaraName => "塔拉 (Tara - 星盘相性与健康)",
        TK::KootaYoniName => "尤尼 (Yoni - 生物本能与性契合)",
        TK::KootaGrahaMaitriName => "葛哈麦崔 (Graha Maitri - 心灵契合与友谊)",
        TK::KootaGanaName => "迦纳 (Gana - 内在气质与性格类型)",
        TK::KootaBhakootName => "巴库特 (Bhakoot - 情感纽带与运势)",
        TK::KootaNadiName => "纳迪 (Nadi - 遗传体质与健康平衡)",

        // Koota Descriptions
        TK::KootaVarnaDescGood => "社会活动与职业倾向契合度良好。",
        TK::KootaVarnaDescBad => "社会活动志向与价值观存在差异。",
        TK::KootaVashyaDescPattern => "吸引力与掌控力评估： {}/2。",
        TK::KootaTaraDescGood => "宿命与健康寿命的极佳匹配。",
        TK::KootaTaraDescOk => "健康契合度一般，需多注意保养。",
        TK::KootaTaraDescBad => "可能面临健康方面的相性冲突 (Tara Dosha)。",
        TK::KootaYoniDescPattern => "生理契合度及相适性： {}/4。",
        TK::KootaGrahaMaitriDescGood => "精神契合度高，友情深厚。",
        TK::KootaGrahaMaitriDescOk => "沟通无碍，属于普通水平。",
        TK::KootaGrahaMaitriDescBad => "存在沟通隔阂与价值观冲突的隐患。",
        TK::GanaDescGood => "内在气质和性格类型极度匹配。",
        TK::GanaDescOk => "存在一定性格差异，但尚可调和。",
        TK::GanaDescBad => "可能存在强烈的性格冲突与观念对立 (Gana Dosha)。",
        TK::BhakootDescGood => "能形成稳定且坚实的情感联结。",
        TK::BhakootDescBad => "可能会带来情感起伏或财务不稳的风险 (Bhakoot Dosha)。",
        TK::NadiDescGood => "体质契合度高，有利于子孙后代的健康。",
        TK::NadiDescBad => "体质能量单一失衡，易生冲突与疲劳 (Nadi Dosha)。",
```

#### 4) Russian (`ru.rs`)
```rust
        TK::CompatTitleInput => "Ввод данных партнера",
        TK::CompatBtnRun => "💞 Начать анализ совместимости",
        TK::CompatStatusLoading => "Анализ совместимости...",
        TK::CompatHeaderOverall => "Общий вердикт совместимости",
        TK::CompatScoreLabel => "Оценка совместимости",
        TK::CompatIsCompatibleGood => "✓ Высокая совместимость",
        TK::CompatIsCompatibleCaution => "⚠️ Требуется осторожность",
        TK::CompatMaleMangalDosha => "Мангал Доша у мужчины",
        TK::CompatFemaleMangalDosha => "Мангал Доша у женщины",
        TK::CompatMangalDetected => "🔥 Обнаружена Мангал Доша (Манглик)",
        TK::CompatMangalNotDetected => "✓ Нет влияния (норма)",
        TK::CompatMangalCancelled => "ℹ️ Взаимная Мангал Доша нейтрализована (Доша Самья). Влияние отсутствует.",
        TK::CompatAshtakootaTableTitle => "Детальная таблица совместимости Аштакута",
        TK::CompatTableColKoota => "Фактор совместимости (Кута)",
        TK::CompatTableColMax => "Макс. вес (Max)",
        TK::CompatTableColEarned => "Набранный балл",
        TK::CompatTableColDesc => "Описание",
        TK::CompatExplanationGood => "В целом гармоничный союз с высокой совместимостью. Рекомендуется.",
        TK::CompatExplanationWarning => "Оценка высокая, однако из-за критической Доши (Нади/Бхакут) необходима осторожность.",
        TK::CompatExplanationBad => "Выявлены глубокие различия в характерах. Требуется взаимная уступчивость и понимание.",
        TK::CompatExplanationSummary => "Набрано баллов: {} из 36.",

        // Koota Names
        TK::KootaVarnaName => "Варна (Varna - Социальная роль)",
        TK::KootaVashyaName => "Вашья (Vashya - Взаимное притяжение)",
        TK::KootaTaraName => "Тара (Tara - Судьба и здоровье)",
        TK::KootaYoniName => "Йони (Yoni - Физиологическая гармония)",
        TK::KootaGrahaMaitriName => "Граха Майтри (Graha Maitri - Ментальная связь)",
        TK::KootaGanaName => "Гана (Gana - Темперамент)",
        TK::KootaBhakootName => "Бхакут (Bhakoot - Эмоциональная связь)",
        TK::KootaNadiName => "Нади (Nadi - Конституция тела)",

        // Koota Descriptions
        TK::KootaVarnaDescGood => "Благоприятное сочетание жизненных путей и призваний.",
        TK::KootaVarnaDescBad => "Различия в социальном позиционировании и целях.",
        TK::KootaVashyaDescPattern => "Уровень взаимного магнетизма: {} из 2.",
        TK::KootaTaraDescGood => "Превосходная энергетическая совместимость и долголетие.",
        TK::KootaTaraDescOk => "Умеренная совместимость здоровья.",
        TK::KootaTaraDescBad => "Возможно неблагоприятное влияние на здоровье (Тара Доша).",
        TK::KootaYoniDescPattern => "Физиологическое сродство и совместимость: {} из 4.",
        TK::KootaGrahaMaitriDescGood => "Высокий уровень взаимопонимания и дружбы.",
        TK::KootaGrahaMaitriDescOk => "Средний уровень понимания, нейтральный союз.",
        TK::KootaGrahaMaitriDescBad => "Трудности в общении и разница во взглядах.",
        TK::KootaGanaDescGood => "Созвучие темпераментов и схожий образ мышления.",
        TK::KootaGanaDescOk => "Небольшие расхождения в характерах, поддающиеся компромиссу.",
        TK::KootaGanaDescBad => "Высокая эмоциональная конфликтность и трения (Гана Доша).",
        TK::KootaBhakootDescGood => "Стабильная, надежная эмоциональная связь.",
        TK::KootaBhakootDescBad => "Возможны резкие спады настроения или финансовая нестабильность (Бхакут Доша).",
        TK::KootaNadiDescGood => "Прекрасная генетическая и конституциональная совместимость.",
        TK::KootaNadiDescBad => "Перекос энергий в одинаковую конституцию (Нади Доша). Возможны проблемы с потомством.",
```

---

## 3. UI Modifications & Circular Gauge Component

### Target File: `crates/eon-ui/src/components/tabs/vedic_tab.rs`

Modify the compatibility rendering block inside `TaskStatus::Success` (around line 2088 onwards).

#### Proposed UI Block implementation:

```rust
TaskStatus::Success => {
    if let Some(compat) = &*compat_data.read() {
        // 1. Compute translated summary & explanation
        let total_score = compat.report.total_score;
        let translated_explanation = {
            let base_desc = if compat.report.is_compatible {
                t(locale, TK::CompatExplanationGood)
            } else if total_score >= 18.0 {
                t(locale, TK::CompatExplanationWarning)
            } else {
                t(locale, TK::CompatExplanationBad)
            };
            format!("{} {}", t(locale, TK::CompatExplanationSummary).replace("{}", &format!("{:.1}", total_score)), base_desc)
        };

        // 2. Prepare SVG Circular Gauge dimensions
        let score_pct = (total_score / 36.0 * 100.0).min(100.0).max(0.0);
        let radius = 40.0;
        let circumference = 2.0 * std::f64::consts::PI * radius; // ~251.327
        let stroke_offset = circumference - (score_pct / 100.0) * circumference;

        rsx! {
            div { class: "space-y-6 animate-in fade-in duration-500",
                
                // --- Visual Progress / Gauge Header Component ---
                div { class: "p-5 bg-slate-900 border border-slate-800 rounded-2xl flex flex-col sm:flex-row items-center gap-5",
                    // SVG Circular Progress Gauge
                    div { class: "relative w-24 h-24 flex items-center justify-center shrink-0",
                        svg { class: "w-full h-full transform -rotate-90", viewBox: "0 0 100 100",
                            // Background track
                            circle {
                                class: "text-slate-800",
                                stroke_width: "8",
                                stroke: "currentColor",
                                fill: "transparent",
                                r: "{radius}",
                                cx: "50",
                                cy: "50"
                            }
                            // Color-coded progress ring
                            circle {
                                class: if compat.report.is_compatible { "text-emerald-500" } else { "text-purple-500" },
                                stroke_width: "8",
                                stroke_dasharray: "{circumference}",
                                stroke_dashoffset: "{stroke_offset}",
                                stroke_linecap: "round",
                                stroke: "currentColor",
                                fill: "transparent",
                                r: "{radius}",
                                cx: "50",
                                cy: "50"
                            }
                        }
                        // Absolute score overlay text
                        div { class: "absolute flex flex-col items-center justify-center",
                            span { class: "text-xl font-bold text-slate-100", "{total_score:.1}" }
                            span { class: "text-[9px] text-slate-500 font-semibold tracking-wider uppercase", "/ 36" }
                        }
                    }

                    // Text descriptions
                    div { class: "flex-1 space-y-1 text-center sm:text-left",
                        h3 { class: "text-xs text-slate-500 uppercase tracking-widest font-bold", "{t(locale, TK::CompatHeaderOverall)}" }
                        p { class: "text-base font-bold text-slate-200", 
                            "{t(locale, TK::CompatScoreLabel)}: "
                            span { class: "text-purple-400 font-mono", "{total_score:.1} / 36.0 Gunas" }
                        }
                        p { class: "text-xs text-slate-400 leading-relaxed", "{translated_explanation}" }
                    }

                    // Verdict Badge
                    div { class: "shrink-0",
                        if compat.report.is_compatible {
                            span { class: "px-4 py-2 rounded-xl bg-emerald-950/60 border border-emerald-800/60 text-emerald-400 text-sm font-bold shadow-lg shadow-emerald-950/20", "{t(locale, TK::CompatIsCompatibleGood)}" }
                        } else {
                            span { class: "px-4 py-2 rounded-xl bg-amber-950/60 border border-amber-800/60 text-amber-400 text-sm font-bold shadow-lg shadow-amber-950/20", "{t(locale, TK::CompatIsCompatibleCaution)}" }
                        }
                    }
                }

                // --- Mangal Dosha Cards (Localized) ---
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                    div { class: "p-4 rounded-2xl bg-slate-900 border border-slate-800 space-y-2",
                        h4 { class: "text-xs text-slate-500 font-semibold uppercase tracking-wider", "{t(locale, TK::CompatMaleMangalDosha)}" }
                        p {
                            class: if compat.report.male_mangal_dosha { "text-lg font-bold text-red-400" } else { "text-lg font-bold text-emerald-400" },
                            if compat.report.male_mangal_dosha { t(locale, TK::CompatMangalDetected) } else { t(locale, TK::CompatMangalNotDetected) }
                        }
                    }
                    div { class: "p-4 rounded-2xl bg-slate-900 border border-slate-800 space-y-2",
                        h4 { class: "text-xs text-slate-500 font-semibold uppercase tracking-wider", "{t(locale, TK::CompatFemaleMangalDosha)}" }
                        p {
                            class: if compat.report.female_mangal_dosha { "text-lg font-bold text-red-400" } else { "text-lg font-bold text-emerald-400" },
                            if compat.report.female_mangal_dosha { t(locale, TK::CompatMangalDetected) } else { t(locale, TK::CompatMangalNotDetected) }
                        }
                    }
                }
                if compat.report.mangal_dosha_cancelled {
                    div { class: "p-4 rounded-xl bg-blue-950/40 border border-blue-800/40 text-blue-300 text-xs font-semibold",
                        "{t(locale, TK::CompatMangalCancelled)}"
                    }
                }

                // --- Ashtakoota Scorecard Table (Localized) ---
                div { class: "bg-slate-900 border border-slate-800 rounded-2xl overflow-hidden shadow-xl",
                    div { class: "bg-slate-800/50 border-b border-slate-800 px-5 py-3.5",
                        h3 { class: "font-semibold text-slate-200", "{t(locale, TK::CompatAshtakootaTableTitle)}" }
                    }
                    div { class: "overflow-x-auto",
                        table { class: "w-full text-sm",
                            thead {
                                tr { class: "bg-slate-800/30 text-xs text-slate-400 uppercase",
                                    th { class: "px-4 py-3 text-left font-medium", "{t(locale, TK::CompatTableColKoota)}" }
                                    th { class: "px-4 py-3 text-center font-medium", "{t(locale, TK::CompatTableColMax)}" }
                                    th { class: "px-4 py-3 text-center font-medium", "{t(locale, TK::CompatTableColEarned)}" }
                                    th { class: "px-4 py-3 text-left font-medium", "{t(locale, TK::CompatTableColDesc)}" }
                                }
                            }
                            tbody { class: "divide-y divide-slate-800",
                                {compat.report.kootas.iter().map(|k| {
                                    // 3. Map to Localized Koota Name and Description
                                    let (koota_name, koota_desc) = match k.id.as_str() {
                                        "varna" => (
                                            t(locale, TK::KootaVarnaName),
                                            if k.earned_points == 1.0 { t(locale, TK::KootaVarnaDescGood).to_string() } else { t(locale, TK::KootaVarnaDescBad).to_string() }
                                        ),
                                        "vashya" => (
                                            t(locale, TK::KootaVashyaName),
                                            t(locale, TK::KootaVashyaDescPattern).replace("{}", &format!("{:.1}", k.earned_points))
                                        ),
                                        "tara" => (
                                            t(locale, TK::KootaTaraName),
                                            if k.earned_points == 3.0 {
                                                t(locale, TK::KootaTaraDescGood).to_string()
                                            } else if k.earned_points == 1.5 {
                                                t(locale, TK::KootaTaraDescOk).to_string()
                                            } else {
                                                t(locale, TK::KootaTaraDescBad).to_string()
                                            }
                                        ),
                                        "yoni" => (
                                            t(locale, TK::KootaYoniName),
                                            t(locale, TK::KootaYoniDescPattern).replace("{}", &format!("{:.1}", k.earned_points))
                                        ),
                                        "graha_maitri" => (
                                            t(locale, TK::KootaGrahaMaitriName),
                                            if k.earned_points >= 4.0 {
                                                t(locale, TK::KootaGrahaMaitriDescGood).to_string()
                                            } else if k.earned_points >= 2.0 {
                                                t(locale, TK::KootaGrahaMaitriDescOk).to_string()
                                            } else {
                                                t(locale, TK::KootaGrahaMaitriDescBad).to_string()
                                            }
                                        ),
                                        "gana" => (
                                            t(locale, TK::KootaGanaName),
                                            if k.earned_points >= 5.0 {
                                                t(locale, TK::KootaGanaDescGood).to_string()
                                            } else if k.earned_points >= 3.0 {
                                                t(locale, TK::KootaGanaDescOk).to_string()
                                            } else {
                                                t(locale, TK::KootaGanaDescBad).to_string()
                                            }
                                        ),
                                        "bhakoot" => (
                                            t(locale, TK::KootaBhakootName),
                                            if k.earned_points == 7.0 {
                                                t(locale, TK::KootaBhakootDescGood).to_string()
                                            } else {
                                                t(locale, TK::KootaBhakootDescBad).to_string()
                                            }
                                        ),
                                        "nadi" => (
                                            t(locale, TK::KootaNadiName),
                                            if k.earned_points == 8.0 {
                                                t(locale, TK::KootaNadiDescGood).to_string()
                                            } else {
                                                t(locale, TK::KootaNadiDescBad).to_string()
                                            }
                                        ),
                                        _ => (k.name.as_str(), k.description.clone())
                                    };

                                    rsx! {
                                        tr { class: "hover:bg-slate-800/20 transition-colors",
                                            td { class: "px-4 py-3 font-semibold text-slate-300", "{koota_name}" }
                                            td { class: "px-4 py-3 text-center font-mono text-slate-500", "{k.max_points:.1}" }
                                            td {
                                                class: if k.earned_points > 0.0 {
                                                    "px-4 py-3 text-center font-bold font-mono text-purple-400"
                                                } else {
                                                    "px-4 py-3 text-center font-bold font-mono text-slate-600"
                                                },
                                                "{k.earned_points:.1}"
                                            }
                                            td { class: "px-4 py-3 text-xs text-slate-400", "{koota_desc}" }
                                        }
                                    }
                                })}
                            }
                        }
                    }
                }
            }
        }
    } else {
        rsx! { div {} }
    }
}
```
