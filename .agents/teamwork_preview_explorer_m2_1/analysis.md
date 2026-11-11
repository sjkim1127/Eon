# Milestone M2 — Ashtakoota Guna Milan Detailed Refinement   (Analysis Report)

## 1.    (Overview & Summary)
  Eon   M2(R1: Ashtakoota    )     UI/i18n  . 
     Compatibility   8 (Ashtakoota)           , 4 (, , , )   i18n   UI     .

---

## 2.     DTO  (Vedic Matching Engine & DTOs)

### 2-1. 8 (Ashtakoota)   (`crates/eon-vedic/src/analysis/matching.rs`)
 Compatibility(Guna Milan) (Moon) (Rasi)  (Nakshatra)   36 points total 8  .

1. **Varna (  / ) [ 1.0]**:
   - ** **:  (Rasi)   (Brahmin=4, Kshatriya=3, Vaishya=2, Shudra=1) ,       1.0,   0.0 .
2. **Vashya (  / ) [ 2.0]**:
   - ** **: Rasi  (Chatushpada, Manushya, Jalachara, Vanachara, Keeta)   .   2.0,    1.0  0.5,   0.0.
3. **Tara (  /  ) [ 3.0]**:
   - ** **:      (mod 9) + 1  (3, 5, 7 - Vipat, Pratyak, Naidhana)  .     3.0,   1.5,    0.0(Tara Dosha).
4. **Yoni (/ ) [ 4.0]**:
   - ** **: 27  14   .  (:  ,   )  0.0,    4.0,   3.0,   2.0.
5. **Graha Maitri (  /  ) [ 5.0]**:
   - ** **:    (Lord)   (, , ) .   5.0,  0.0,      4.0, 3.0, 2.0, 1.0 .
6. **Gana ( ) [ 6.0]**:
   - ** **:   Deva(), Manushya(), Rakshasa(/)  .   6.0, Deva-Manushya 5.0, Manushya-Rakshasa 3.0, Deva-Rakshasa 1.0. (Rakshasa-Deva/Manushya   Gana Dosha ).
7. **Bhakoot (  /  ) [ 7.0]**:
   - ** **:       (1, 7, 3, 11, 4, 10)  7.0,  (2/12, 5/9, 6/8)  0.0(Bhakoot Dosha).
8. **Nadi (  /  ) [ 8.0]**:
   - ** **:   Adi(), Madhya(), Antya() .    8.0,  0.0(Nadi Dosha -          ).

### 2-2. (Mangal Dosha)   
-     Lagna  Moon  1, 2, 4, 7, 8, 12    (Manglik) .
- **Dosha Samya**:       (`mangal_dosha_cancelled = true`)    .
- ** **:  18.0 ,  (Nadi Dosha = 0, Bhakoot Dosha = 0)     `is_compatible = true` .

### 2-3. DTO   
-  : `crates/eon-service/src/services/vedic.rs` -> `analyze_compatibility` .
-  DTO: `VedicCompatibilityInput` (  `AnalysisInput` ).
-  DTO: `VedicCompatibilityOutput`
  ```rust
  pub struct VedicCompatibilityOutput {
      pub meta: AnalysisMeta,
      pub report: eon_vedic::analysis::matching::CompatibilityReport,
  }
  ```
  `CompatibilityReport`  `total_score` (f64), `is_compatible` (bool), `kootas` (Vec<KootaScore>), `male_mangal_dosha` (bool), `female_mangal_dosha` (bool), `mangal_dosha_cancelled` (bool), `explanation` (String)  .

---

## 3. UI   (UI Design Proposal)

### 3-1.   
    Compatibility   .
1. **   (Overall Status Card)**:
   -     (`{total_score} / 36 Gunas`) ,    (`✓  `  `⚠️  `)   .
   -    (Explanation) .
2. **   (Mangal Dosha Cards)**:
   -        .
   -     (`🔥  `),    (`✓  `)  .
   -  (Dosha Samya)        .
3. **8     (8-Koota Detailed Grid)**:
   - `grid grid-cols-1 md:grid-cols-2 gap-4`   8    .
   -   **[ ]**, **[ / ]**, **[  (Progress Bar)]**, **[  ]** .
   - ** (Progress Bar)**  :
     - (Ratio == 1.0): Emerald  (`bg-emerald-500`)
     -  (Ratio > 0.0): Purple  (`bg-purple-500`)
     - 0(Ratio == 0.0): Red  (`bg-red-500`)

---

## 4.  (i18n)  (Localization Strategy)

   `explanation`  Koota `description`     .     4 (Ko, En, Zh, Ru)    UI             .

### 4-1.    (Translation Keys in `TK` enum)
`crates/eon-ui/src/i18n/mod.rs`    .

```rust
    // Vedic Compatibility
    VedicCompatPartnerInputHeader,     // "Enter Partner Birth Information"
    VedicCompatPartnerYearLabel,       // ""
    VedicCompatPartnerMonthLabel,      // ""
    VedicCompatPartnerDayLabel,        // ""
    VedicCompatPartnerHourLabel,       // " ()"
    VedicCompatPartnerMinLabel,        // " ()"
    VedicCompatPartnerLatLabel,        // " (Latitude)"
    VedicCompatPartnerLonLabel,        // " (Longitude)"
    VedicCompatRunButton,              // "💞 Compatibility  "
    VedicCompatLoadingText,            // "Compatibility  ..."
    VedicCompatOverallJudgementHeader, // "  "
    VedicCompatScoreLabelText,         // " "
    VedicCompatStatusExcellentText,    // "✓  "
    VedicCompatStatusCautionText,      // "⚠️  "
    VedicCompatMaleMangalTitle,        // "  (Male Mangal Dosha)"
    VedicCompatFemaleMangalTitle,      // "  (Female Mangal Dosha)"
    VedicCompatMangalDetected,         // "🔥 (Manglik) "
    VedicCompatMangalNotDetected,      // "✓   ()"
    VedicCompatMangalCancelledText,    // "ℹ️   (Dosha Samya)..."
    VedicCompatAshtakootaTableHeader,  // "Ashtakoota   "
```

---

## 5.    (Proposed Code Changes)

### 5-1. `crates/eon-ui/src/i18n/mod.rs`  
`TK`    .

```rust
<<<< BEFORE
    VedicLagnaRasi,
    VedicHouseScore,

    // ── Tier ──────────────────────────────────────────────────────────
==== AFTER
    VedicLagnaRasi,
    VedicHouseScore,

    // ── Vedic Compatibility ──
    VedicCompatPartnerInputHeader,
    VedicCompatPartnerYearLabel,
    VedicCompatPartnerMonthLabel,
    VedicCompatPartnerDayLabel,
    VedicCompatPartnerHourLabel,
    VedicCompatPartnerMinLabel,
    VedicCompatPartnerLatLabel,
    VedicCompatPartnerLonLabel,
    VedicCompatRunButton,
    VedicCompatLoadingText,
    VedicCompatOverallJudgementHeader,
    VedicCompatScoreLabelText,
    VedicCompatStatusExcellentText,
    VedicCompatStatusCautionText,
    VedicCompatMaleMangalTitle,
    VedicCompatFemaleMangalTitle,
    VedicCompatMangalDetected,
    VedicCompatMangalNotDetected,
    VedicCompatMangalCancelledText,
    VedicCompatAshtakootaTableHeader,

    // ── Tier ──────────────────────────────────────────────────────────
>>>>
```

### 5-2. `ko.rs`, `en.rs`, `zh.rs`, `ru.rs`    

#### `crates/eon-ui/src/i18n/ko.rs`
```rust
        // Vedic-specific (Vedic Compatibility)
        TK::VedicCompatPartnerInputHeader => "Enter Partner Birth Information",
        TK::VedicCompatPartnerYearLabel => "",
        TK::VedicCompatPartnerMonthLabel => "",
        TK::VedicCompatPartnerDayLabel => "",
        TK::VedicCompatPartnerHourLabel => " ()",
        TK::VedicCompatPartnerMinLabel => " ()",
        TK::VedicCompatPartnerLatLabel => " (Latitude)",
        TK::VedicCompatPartnerLonLabel => " (Longitude)",
        TK::VedicCompatRunButton => "💞 Compatibility  ",
        TK::VedicCompatLoadingText => "Compatibility  ...",
        TK::VedicCompatOverallJudgementHeader => "  ",
        TK::VedicCompatScoreLabelText => " ",
        TK::VedicCompatStatusExcellentText => "✓  ",
        TK::VedicCompatStatusCautionText => "⚠️  ",
        TK::VedicCompatMaleMangalTitle => "  (Male Mangal Dosha)",
        TK::VedicCompatFemaleMangalTitle => "  (Female Mangal Dosha)",
        TK::VedicCompatMangalDetected => "🔥 (Manglik) ",
        TK::VedicCompatMangalNotDetected => "✓   ()",
        TK::VedicCompatMangalCancelledText => "ℹ️   (Dosha Samya)     .",
        TK::VedicCompatAshtakootaTableHeader => "Ashtakoota   ",
```

#### `crates/eon-ui/src/i18n/en.rs`
```rust
        // Vedic-specific (Vedic Compatibility)
        TK::VedicCompatPartnerInputHeader => "Partner's Birth Information",
        TK::VedicCompatPartnerYearLabel => "Year",
        TK::VedicCompatPartnerMonthLabel => "Month",
        TK::VedicCompatPartnerDayLabel => "Day",
        TK::VedicCompatPartnerHourLabel => "Hour",
        TK::VedicCompatPartnerMinLabel => "Minute",
        TK::VedicCompatPartnerLatLabel => "Latitude",
        TK::VedicCompatPartnerLonLabel => "Longitude",
        TK::VedicCompatRunButton => "💞 Run Compatibility Analysis",
        TK::VedicCompatLoadingText => "Analyzing compatibility...",
        TK::VedicCompatOverallJudgementHeader => "Overall Compatibility Judgement",
        TK::VedicCompatScoreLabelText => "Compatibility Score",
        TK::VedicCompatStatusExcellentText => "✓ Excellent Compatibility",
        TK::VedicCompatStatusCautionText => "⚠️ Caution Required",
        TK::VedicCompatMaleMangalTitle => "Male Mangal Dosha",
        TK::VedicCompatFemaleMangalTitle => "Female Mangal Dosha",
        TK::VedicCompatMangalDetected => "🔥 Manglik Detected",
        TK::VedicCompatMangalNotDetected => "✓ N/A (Clear)",
        TK::VedicCompatMangalCancelledText => "ℹ️ Dosha Samya established: Both partners have Manglik status, cancelling the negative effects.",
        TK::VedicCompatAshtakootaTableHeader => "Ashtakoota Guna Milan Detailed Scorecard",
```

#### `crates/eon-ui/src/i18n/zh.rs`
```rust
        // Vedic-specific (Vedic Compatibility)
        TK::VedicCompatPartnerInputHeader => "输入对方出生信息",
        TK::VedicCompatPartnerYearLabel => "年",
        TK::VedicCompatPartnerMonthLabel => "月",
        TK::VedicCompatPartnerDayLabel => "日",
        TK::VedicCompatPartnerHourLabel => "时",
        TK::VedicCompatPartnerMinLabel => "分",
        TK::VedicCompatPartnerLatLabel => "纬度 (Latitude)",
        TK::VedicCompatPartnerLonLabel => "经度 (Longitude)",
        TK::VedicCompatRunButton => "💞 运行合婚分析",
        TK::VedicCompatLoadingText => "正在进行合婚计算...",
        TK::VedicCompatOverallJudgementHeader => "综合配对判定",
        TK::VedicCompatScoreLabelText => "配对分值",
        TK::VedicCompatStatusExcellentText => "✓ 极佳配对",
        TK::VedicCompatStatusCautionText => "⚠️ 需谨慎处理",
        TK::VedicCompatMaleMangalTitle => "男方火星煞 (Male Mangal Dosha)",
        TK::VedicCompatFemaleMangalTitle => "女方火星煞 (Female Mangal Dosha)",
        TK::VedicCompatMangalDetected => "🔥 检测到火星煞 (Manglik)",
        TK::VedicCompatMangalNotDetected => "✓ 无 (良好)",
        TK::VedicCompatMangalCancelledText => "ℹ️ 双方火星煞相互抵消 (Dosha Samya) 成立，消除了负面影响。",
        TK::VedicCompatAshtakootaTableHeader => "阿什塔库塔 (Ashtakoota) 详细评分表",
```

#### `crates/eon-ui/src/i18n/ru.rs`
```rust
        // Vedic-specific (Vedic Compatibility)
        TK::VedicCompatPartnerInputHeader => "Ввод данных партнёра",
        TK::VedicCompatPartnerYearLabel => "Год",
        TK::VedicCompatPartnerMonthLabel => "Месяц",
        TK::VedicCompatPartnerDayLabel => "День",
        TK::VedicCompatPartnerHourLabel => "Час",
        TK::VedicCompatPartnerMinLabel => "Минута",
        TK::VedicCompatPartnerLatLabel => "Широта",
        TK::VedicCompatPartnerLonLabel => "Долгота",
        TK::VedicCompatRunButton => "💞 Запустить анализ совместимости",
        TK::VedicCompatLoadingText => "Анализ совместимости...",
        TK::VedicCompatOverallJudgementHeader => "Общая оценка совместимости",
        TK::VedicCompatScoreLabelText => "Балл совместимости",
        TK::VedicCompatStatusExcellentText => "✓ Отличная совместимость",
        TK::VedicCompatStatusCautionText => "⚠️ Требуется осторожность",
        TK::VedicCompatMaleMangalTitle => "Марсианское влияние у мужчины (Mangal Dosha)",
        TK::VedicCompatFemaleMangalTitle => "Марсианское влияние у женщины (Mangal Dosha)",
        TK::VedicCompatMangalDetected => "🔥 Обнаружена Манглик Доша",
        TK::VedicCompatMangalNotDetected => "✓ Нет (В норме)",
        TK::VedicCompatMangalCancelledText => "ℹ️ Установлена Доша Самья: Оба партнёра имеют Манглик Доша, нейтрализуя её негативные эффекты.",
        TK::VedicCompatAshtakootaTableHeader => "Детальная карта оценки Аштакута",
```

---

### 5-3. `crates/eon-ui/src/components/tabs/vedic_tab.rs`  

#### 1) Koota     (      )
```rust
struct LocalizedKoota {
    name: &'static str,
    meaning: &'static str,
    interpretation: String,
}

fn localize_koota(locale: Locale, index: usize, earned: f64, max: f64) -> LocalizedKoota {
    match locale {
        Locale::Ko => match index {
            0 => LocalizedKoota {
                name: " (Varna) -  ",
                meaning: " ,       .",
                interpretation: if earned == 1.0 {
                    "      .".to_string()
                } else {
                    "        .".to_string()
                }
            },
            1 => LocalizedKoota {
                name: " (Vashya) -  ",
                meaning: "  ,     .",
                interpretation: format!("    : {}/2.", earned),
            },
            2 => LocalizedKoota {
                name: " (Tara) -  ",
                meaning: "         .",
                interpretation: if earned == 3.0 {
                    "    .".to_string()
                } else if earned == 1.5 {
                    "      .".to_string()
                } else {
                    "     ( ).".to_string()
                }
            },
            3 => LocalizedKoota {
                name: " (Yoni) -  ",
                meaning: ",         .",
                interpretation: format!("/ : {}/4.", earned),
            },
            4 => LocalizedKoota {
                name: "  (Graha Maitri) -  ",
                meaning: "  ()      .",
                interpretation: if earned >= 4.0 {
                    "     .".to_string()
                } else if earned >= 2.0 {
                    "       .".to_string()
                } else {
                    "      .".to_string()
                }
            },
            5 => LocalizedKoota {
                name: " (Gana) -  ",
                meaning: " (Deva-, Manushya-, Rakshasa-) .",
                interpretation: if earned >= 5.0 {
                    "     .".to_string()
                } else if earned >= 3.0 {
                    "  ,   .".to_string()
                } else {
                    "        ( ).".to_string()
                }
            },
            6 => LocalizedKoota {
                name: " (Bhakoot) -  ",
                meaning: "     ,   .",
                interpretation: if earned == 7.0 {
                    "      .".to_string()
                } else {
                    "       ( ).".to_string()
                }
            },
            7 => LocalizedKoota {
                name: " (Nadi) -  ",
                meaning: "  ,   ()   .",
                interpretation: if earned == 8.0 {
                    "       .".to_string()
                } else {
                    "        ( ).".to_string()
                }
            },
            _ => LocalizedKoota { name: "", meaning: "", interpretation: "".to_string() }
        },
        Locale::En => match index {
            0 => LocalizedKoota {
                name: "Varna - Spiritual Inclination",
                meaning: "Represents ego development, spiritual growth, and natural work inclination.",
                interpretation: if earned == 1.0 {
                    "Smooth collaboration and healthy division of roles.".to_string()
                } else {
                    "Possible friction due to different life perspectives and career attitudes.".to_string()
                }
            },
            1 => LocalizedKoota {
                name: "Vashya - Mutual Influence",
                meaning: "Analyzes the degree of mutual attraction, influence, and dominance in relationship.",
                interpretation: format!("Mutual attraction and influence rating: {}/2.", earned),
            },
            2 => LocalizedKoota {
                name: "Tara - Destiny and Health",
                meaning: "Evaluates health, longevity, and overall destiny based on birth stars.",
                interpretation: if earned == 3.0 {
                    "Excellent alignment of longevity and fortune.".to_string()
                } else if earned == 1.5 {
                    "Moderate health and energy compatibility.".to_string()
                } else {
                    "Challenging health and energy match (Tara Dosha).".to_string()
                }
            },
            3 => LocalizedKoota {
                name: "Yoni - Biological Affinity",
                meaning: "Represents physical, biological compatibility and instinctual intimacy through animal symbols.",
                interpretation: format!("Biological and physical compatibility: {}/4.", earned),
            },
            4 => LocalizedKoota {
                name: "Graha Maitri - Mental Bond",
                meaning: "Measures intellectual communication, friendship, and emotional resonance based on ruling planets.",
                interpretation: if earned >= 4.0 {
                    "Deep mental harmony and mutual friendship.".to_string()
                } else if earned >= 2.0 {
                    "Average intellectual connection and understanding.".to_string()
                } else {
                    "Potential gaps in communication and mental alignment.".to_string()
                }
            },
            5 => LocalizedKoota {
                name: "Gana - Temperament",
                meaning: "Compares fundamental temperaments (Deva-Divine, Manushya-Human, Rakshasa-Demon).",
                interpretation: if earned >= 5.0 {
                    "Compatible temperaments, low emotional friction.".to_string()
                } else if earned >= 3.0 {
                    "Moderate gaps in temperament; patience is required.".to_string()
                } else {
                    "High emotional friction and temperamental clash (Gana Dosha).".to_string()
                }
            },
            6 => LocalizedKoota {
                name: "Bhakoot - Emotional Node",
                meaning: "Analyzes emotional health, prosperity, and marital longevity based on Moon sign distances.",
                interpretation: if earned == 7.0 {
                    "Stable emotional bond and harmonious family life.".to_string()
                } else {
                    "Challenging emotional cycles or potential financial stress (Bhakoot Dosha).".to_string()
                }
            },
            7 => LocalizedKoota {
                name: "Nadi - Genetic Health",
                meaning: "The most critical factor; checks genetic health, nervous energy, and lineage compatibility.",
                interpretation: if earned == 8.0 {
                    "Balanced nervous energies and excellent genetic compatibility.".to_string()
                } else {
                    "Excessive similar nervous energy, risk of genetic mismatch (Nadi Dosha).".to_string()
                }
            },
            _ => LocalizedKoota { name: "", meaning: "", interpretation: "".to_string() }
        },
        Locale::Zh => match index {
            0 => LocalizedKoota {
                name: "Varna (瓦尔纳) - 精神倾向",
                meaning: "代表精神成长、自尊以及天然的工作倾斜度与社会角色分工。",
                interpretation: if earned == 1.0 {
                    "相互合作与角色分工非常顺利，关系积极。".to_string()
                } else {
                    "因价值观和对待工作的态度不同，可能会产生分歧。".to_string()
                }
            },
            1 => LocalizedKoota {
                name: "Vashya (瓦夏) - 相互吸引力",
                meaning: "分析夫妻之间的主导权、相互吸引力与控制力平衡。",
                interpretation: format!("相互吸引与合作度评分: {}/2。", earned),
            },
            2 => LocalizedKoota {
                name: "Tara (塔拉) - 命运与健康",
                meaning: "通过双方出生星宿的相对距离，来判断健康、寿命及整体运势。",
                interpretation: if earned == 3.0 {
                    "极佳的命运契合度与长寿运。".to_string()
                } else if earned == 1.5 {
                    "中等水平的健康与活动兼容性。".to_string()
                } else {
                    "健康与寿命需要注意的配置 (Tara 煞/Tara Dosha)。".to_string()
                }
            },
            3 => LocalizedKoota {
                name: "Yoni (约尼) - 生物学亲和力",
                meaning: "以象征性的动物类型来评估身体、性契合度及本能的亲密感。",
                interpretation: format!("生物学和生理上的契合度: {}/4。", earned),
            },
            4 => LocalizedKoota {
                name: "Graha Maitri (行星友谊) - 精神交流",
                meaning: "通过双方月亮守护星的关系，衡量智力沟通、友谊和共鸣程度。",
                interpretation: if earned >= 4.0 {
                    "精神上有深厚的纽带和温暖的友谊。".to_string()
                } else if earned >= 2.0 {
                    "平稳的智力交流，沟通顺畅。".to_string()
                } else {
                    "因沟通方式差异容易产生误解的组合。".to_string()
                }
            },
            5 => LocalizedKoota {
                name: "Gana (迦纳) - 气质与性情",
                meaning: "比较双方的基本气质类型（Deva-天神，Manushya-人类，Rakshasa-罗刹）。",
                interpretation: if earned >= 5.0 {
                    "气质契合，极少有情绪冲突。".to_string()
                } else if earned >= 3.0 {
                    "中等水平的气质和谐，需要相互妥协。".to_string()
                } else {
                    "性格差异大，可能发生频繁摩擦 (Gana 煞/Gana Dosha)。".to_string()
                }
            },
            6 => LocalizedKoota {
                name: "Bhakoot (巴库特) - 情感纽带",
                meaning: "基于相对月亮星座的距离，分析情感健康、繁荣和婚姻的稳定性。",
                interpretation: if earned == 7.0 {
                    "情感没有大起大落，维持稳定的情感纽带。".to_string()
                } else {
                    "容易感到情感疏离或面临家庭财务波动的风险 (Bhakoot 煞/Bhakoot Dosha)。".to_string()
                }
            },
            7 => LocalizedKoota {
                name: "Nadi (纳迪) - 遗传兼容性",
                meaning: "最关键的因素，检测双方神经系统能量、体质以及遗传兼容性。",
                interpretation: if earned == 8.0 {
                    "身体免疫系统契合，有利于子嗣健康。".to_string()
                } else {
                    "相同类型的过剩能量，有遗传不契合的风险 (Nadi 煞/Nadi Dosha)。".to_string()
                }
            },
            _ => LocalizedKoota { name: "", meaning: "", interpretation: "".to_string() }
        },
        Locale::Ru => match index {
            0 => LocalizedKoota {
                name: "Варна (Varna) - Ментальный склад",
                meaning: "Отражает духовное развитие, эго и естественные склонности в работе и социальной роли.",
                interpretation: if earned == 1.0 {
                    "Гармоничное сотрудничество и легкое разделение обязанностей.".to_string()
                } else {
                    "Возможны разногласия из-за несовпадающих взглядов на карьеру и жизнь.".to_string()
                }
            },
            1 => LocalizedKoota {
                name: "Вашья (Vashya) - Взаимное влияние",
                meaning: "Анализирует степень взаимного притяжения, влияния и баланса авторитета в паре.",
                interpretation: format!("Оценка взаимного притяжения и влияния: {}/2.", earned),
            },
            2 => LocalizedKoota {
                name: "Тара (Tara) - Судьба и здоровье",
                meaning: "Оценивает здоровье, долголетие и общую удачу на основе взаимного положения накшатр.",
                interpretation: if earned == 3.0 {
                    "Отличное соответствие жизненных циклов и долголетия.".to_string()
                } else if earned == 1.5 {
                    "Умеренная совместимость по уровню здоровья и энергии.".to_string()
                } else {
                    "Сложная совместимость, требующая внимания к здоровью (Тара-доша).".to_string()
                }
            },
            3 => LocalizedKoota {
                name: "Йони (Yoni) - Биологическая близость",
                meaning: "Определяет телесную и инстинктивную близость через символы животных типов накшатр.",
                interpretation: format!("Биологическая и инстинктивная совместимость: {}/4.", earned),
            },
            4 => LocalizedKoota {
                name: "Граха Майтри (Graha Maitri) - Ментальная связь",
                meaning: "Показывает интеллектуальное взаимопонимание, дружбу на основе отношений планет-управителей Луны.",
                interpretation: if earned >= 4.0 {
                    "Глубокая ментальная гармония и искренняя дружба в паре.".to_string()
                } else if earned >= 2.0 {
                    "Средний уровень понимания, спокойный диалог.".to_string()
                } else {
                    "Возможны частые недопонимания из-за разного стиля мышления.".to_string()
                }
            },
            5 => LocalizedKoota {
                name: "Гана (Gana) - Темперамент",
                meaning: "Сравнивает базовые психотипы (Дева - божественный, Манушья - человеческий, Ракшас - импульсивный).",
                interpretation: if earned >= 5.0 {
                    "Гармоничное сочетание характеров, минимум эмоциональных трений.".to_string()
                } else if earned >= 3.0 {
                    "Среднее совпадение, требуется взаимная уступчивость.".to_string()
                } else {
                    "Высокий риск эмоциональных конфликтов из-за несовместимости характеров (Гана-доша).".to_string()
                }
            },
            6 => LocalizedKoota {
                name: "Бхакут (Bhakoot) - Эмоциональная связь",
                meaning: "Оценивает эмоциональную стабильность, семейное процветание по взаимному положению знаков Луны.",
                interpretation: if earned == 7.0 {
                    "Стабильная эмоциональная привязанность без резких перепадов.".to_string()
                } else {
                    "Вероятны эмоциональное отдаление или финансовые трудности в семье (Бхакут-доша).".to_string()
                }
            },
            7 => LocalizedKoota {
                name: "Нади (Nadi) - Генетическая совместимость",
                meaning: "Важнейший фактор. Проверяет баланс нервной энергии и генетическое здоровье потомства.",
                interpretation: if earned == 8.0 {
                    "Сбалансированное здоровье и отличная совместимость на уровне потомства.".to_string()
                } else {
                    "Избыток однородной энергии, генетическая несовместимость (Нади-доша).".to_string()
                }
            },
            _ => LocalizedKoota { name: "", meaning: "", interpretation: "".to_string() }
        }
    }
}
```

#### 2) `vedic_tab.rs`       
(  1996~2185  Dioxus     )

```rust
                                3 => rsx! {
                                    // Compatibility Tab
                                    div { class: "space-y-6",
                                        // 1. Partner Input Form
                                        div { class: "p-5 bg-slate-900 border border-slate-800 rounded-2xl space-y-4",
                                            h3 { class: "text-lg font-semibold text-slate-200", "{t(locale, TK::VedicCompatPartnerInputHeader)}" }
                                            div { class: "grid grid-cols-2 md:grid-cols-5 gap-3",
                                                div { class: "flex flex-col gap-1",
                                                    label { class: "text-xs text-slate-500", "{t(locale, TK::VedicCompatPartnerYearLabel)}" }
                                                    input {
                                                        class: "bg-slate-800 border border-slate-700 rounded-xl px-3 py-2 text-slate-200",
                                                        r#type: "number",
                                                        value: "{partner_year}",
                                                        oninput: move |e| *partner_year.write() = e.value().parse().unwrap_or(1992)
                                                    }
                                                }
                                                div { class: "flex flex-col gap-1",
                                                    label { class: "text-xs text-slate-500", "{t(locale, TK::VedicCompatPartnerMonthLabel)}" }
                                                    input {
                                                        class: "bg-slate-800 border border-slate-700 rounded-xl px-3 py-2 text-slate-200",
                                                        r#type: "number",
                                                        value: "{partner_month}",
                                                        oninput: move |e| *partner_month.write() = e.value().parse().unwrap_or(8)
                                                    }
                                                }
                                                div { class: "flex flex-col gap-1",
                                                    label { class: "text-xs text-slate-500", "{t(locale, TK::VedicCompatPartnerDayLabel)}" }
                                                    input {
                                                        class: "bg-slate-800 border border-slate-700 rounded-xl px-3 py-2 text-slate-200",
                                                        r#type: "number",
                                                        value: "{partner_day}",
                                                        oninput: move |e| *partner_day.write() = e.value().parse().unwrap_or(24)
                                                    }
                                                }
                                                div { class: "flex flex-col gap-1",
                                                    label { class: "text-xs text-slate-500", "{t(locale, TK::VedicCompatPartnerHourLabel)}" }
                                                    input {
                                                        class: "bg-slate-800 border border-slate-700 rounded-xl px-3 py-2 text-slate-200",
                                                        r#type: "number",
                                                        value: "{partner_hour}",
                                                        oninput: move |e| *partner_hour.write() = e.value().parse().unwrap_or(14)
                                                    }
                                                }
                                                div { class: "flex flex-col gap-1",
                                                    label { class: "text-xs text-slate-500", "{t(locale, TK::VedicCompatPartnerMinLabel)}" }
                                                    input {
                                                        class: "bg-slate-800 border border-slate-700 rounded-xl px-3 py-2 text-slate-200",
                                                        r#type: "number",
                                                        value: "{partner_minute}",
                                                        oninput: move |e| *partner_minute.write() = e.value().parse().unwrap_or(30)
                                                    }
                                                }
                                            }
                                            div { class: "grid grid-cols-2 gap-3",
                                                div { class: "flex flex-col gap-1",
                                                    label { class: "text-xs text-slate-500", "{t(locale, TK::VedicCompatPartnerLatLabel)}" }
                                                    input {
                                                        class: "bg-slate-800 border border-slate-700 rounded-xl px-3 py-2 text-slate-200",
                                                        r#type: "number",
                                                        step: "any",
                                                        value: "{partner_lat}",
                                                        oninput: move |e| *partner_lat.write() = e.value().parse().unwrap_or(37.5665)
                                                    }
                                                }
                                                div { class: "flex flex-col gap-1",
                                                    label { class: "text-xs text-slate-500", "{t(locale, TK::VedicCompatPartnerLonLabel)}" }
                                                    input {
                                                        class: "bg-slate-800 border border-slate-700 rounded-xl px-3 py-2 text-slate-200",
                                                        r#type: "number",
                                                        step: "any",
                                                        value: "{partner_lon}",
                                                        oninput: move |e| *partner_lon.write() = e.value().parse().unwrap_or(126.9780)
                                                    }
                                                }
                                            }
                                            button {
                                                class: "w-full py-3 bg-gradient-to-r from-purple-700 to-indigo-700 hover:from-purple-600 hover:to-indigo-600 rounded-xl font-bold text-white shadow-lg",
                                                onclick: run_compatibility,
                                                "{t(locale, TK::VedicCompatRunButton)}"
                                            }
                                        }

                                        // 2. Compatibility Results
                                        match &*compat_status.read() {
                                            TaskStatus::Loading => rsx! {
                                                div { class: "flex flex-col items-center py-10 gap-2",
                                                    div { class: "w-10 h-10 rounded-full border-4 border-purple-500/30 border-t-purple-400 animate-spin" }
                                                    p { class: "text-purple-400 font-medium text-sm animate-pulse", "{t(locale, TK::VedicCompatLoadingText)}" }
                                                }
                                            },
                                            TaskStatus::Error(e) => rsx! {
                                                div { class: "p-4 rounded-xl bg-red-900/20 border border-red-800/50 text-red-400 text-sm", "{t(locale, TK::StatusError)}: {e}" }
                                            },
                                            TaskStatus::Success => {
                                                if let Some(compat) = &*compat_data.read() {
                                                    // Dynamically localize the overall matching explanation
                                                    let explanation_text = match locale {
                                                        Locale::Ko => format!(
                                                            "Total {} points (out of 36). {}",
                                                            compat.report.total_score,
                                                            if compat.report.is_compatible {
                                                                "  . ."
                                                            } else if compat.report.total_score >= 18.0 {
                                                                "   (Nadi/Bhakoot Dosha)    ."
                                                            } else {
                                                                "        ."
                                                            }
                                                        ),
                                                        Locale::En => format!(
                                                            "Earned {} points in total (out of 36). {}",
                                                            compat.report.total_score,
                                                            if compat.report.is_compatible {
                                                                "Overall a very harmonious match. Highly recommended."
                                                            } else if compat.report.total_score >= 18.0 {
                                                                "High score, but requires careful attention due to critical Nadi/Bhakoot Dosha."
                                                            } else {
                                                                "Significant temperamental differences; requires mutual adjustment and deep understanding."
                                                            }
                                                        ),
                                                        Locale::Zh => format!(
                                                            "共获得 {} 分（满分 36 分）。{}",
                                                            compat.report.total_score,
                                                            if compat.report.is_compatible {
                                                                "总体而言是非常和谐的配对，值得推荐。"
                                                            } else if compat.report.total_score >= 18.0 {
                                                                "虽然得分较高，但由于存在关键的纳迪/巴库特煞（Nadi/Bhakoot Dosha），需要谨慎对待。"
                                                            } else {
                                                                "性情差异较大，需要相互包容与深度理解。"
                                                            }
                                                        ),
                                                        Locale::Ru => format!(
                                                            "Всего набрано {} баллов (из 36). {}",
                                                            compat.report.total_score,
                                                            if compat.report.is_compatible {
                                                                "В целом гармоничное сочетание. Рекомендуется."
                                                            } else if compat.report.total_score >= 18.0 {
                                                                "Высокий балл, но требуется осторожность из-за критических Нади/Бхакут дош."
                                                            } else {
                                                                "Значительные различия в характерах; требуются взаимные уступки и глубокое понимание."
                                                            }
                                                        ),
                                                    };

                                                    rsx! {
                                                        div { class: "space-y-6 animate-in fade-in duration-500",
                                                            // Overall Judgement Card
                                                            div { class: "p-5 bg-slate-900 border border-slate-800 rounded-2xl flex flex-col md:flex-row md:items-center justify-between gap-4",
                                                                div {
                                                                    h3 { class: "text-xs text-slate-500 uppercase tracking-widest font-bold", "{t(locale, TK::VedicCompatOverallJudgementHeader)}" }
                                                                    p { class: "text-2xl font-bold text-slate-200 mt-1",
                                                                        "{t(locale, TK::VedicCompatScoreLabelText)}: "
                                                                        span { class: "text-purple-400", "{compat.report.total_score} / 36 Gunas" }
                                                                    }
                                                                    p { class: "text-sm text-slate-400 mt-1.5", "{explanation_text}" }
                                                                }
                                                                div { class: "flex gap-2",
                                                                    if compat.report.is_compatible {
                                                                        span { class: "px-4 py-2 rounded-xl bg-emerald-950/60 border border-emerald-800/60 text-emerald-400 text-sm font-bold", "{t(locale, TK::VedicCompatStatusExcellentText)}" }
                                                                    } else {
                                                                        span { class: "px-4 py-2 rounded-xl bg-amber-950/60 border border-amber-800/60 text-amber-400 text-sm font-bold", "{t(locale, TK::VedicCompatStatusCautionText)}" }
                                                                    }
                                                                }
                                                            }

                                                            // Mangal Dosha Cards
                                                            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                                                div { class: "p-4 rounded-2xl bg-slate-900 border border-slate-800 space-y-2",
                                                                    h4 { class: "text-xs text-slate-500 font-semibold uppercase tracking-wider", "{t(locale, TK::VedicCompatMaleMangalTitle)}" }
                                                                    p {
                                                                        class: if compat.report.male_mangal_dosha {
                                                                            "text-lg font-bold text-red-400"
                                                                        } else {
                                                                            "text-lg font-bold text-emerald-400"
                                                                        },
                                                                        if compat.report.male_mangal_dosha { "{t(locale, TK::VedicCompatMangalDetected)}" } else { "{t(locale, TK::VedicCompatMangalNotDetected)}" }
                                                                    }
                                                                }
                                                                div { class: "p-4 rounded-2xl bg-slate-900 border border-slate-800 space-y-2",
                                                                    h4 { class: "text-xs text-slate-500 font-semibold uppercase tracking-wider", "{t(locale, TK::VedicCompatFemaleMangalTitle)}" }
                                                                    p {
                                                                        class: if compat.report.female_mangal_dosha {
                                                                            "text-lg font-bold text-red-400"
                                                                        } else {
                                                                            "text-lg font-bold text-emerald-400"
                                                                        },
                                                                        if compat.report.female_mangal_dosha { "{t(locale, TK::VedicCompatMangalDetected)}" } else { "{t(locale, TK::VedicCompatMangalNotDetected)}" }
                                                                    }
                                                                }
                                                            }
                                                            if compat.report.mangal_dosha_cancelled {
                                                                div { class: "p-4 rounded-xl bg-blue-950/40 border border-blue-800/40 text-blue-300 text-xs font-semibold",
                                                                    "{t(locale, TK::VedicCompatMangalCancelledText)}"
                                                                }
                                                            }

                                                            // Ashtakoota Detailed Cards Grid with Progress Bars
                                                            div { class: "space-y-4",
                                                                h3 { class: "font-semibold text-slate-200 text-base", "{t(locale, TK::VedicCompatAshtakootaTableHeader)}" }
                                                                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                                                    {compat.report.kootas.iter().enumerate().map(|(idx, k)| {
                                                                        let local = localize_koota(locale, idx, k.earned_points, k.max_points);
                                                                        let pct = (k.earned_points / k.max_points * 100.0).min(100.0).max(0.0);
                                                                        
                                                                        // Determine colors based on performance
                                                                        let (bar_color, text_color, bg_light) = if k.earned_points == k.max_points {
                                                                            ("bg-emerald-500", "text-emerald-400", "bg-emerald-500/10")
                                                                        } else if k.earned_points > 0.0 {
                                                                            ("bg-purple-500", "text-purple-400", "bg-purple-500/10")
                                                                        } else {
                                                                            ("bg-red-500", "text-red-400", "bg-red-500/10")
                                                                        };

                                                                        rsx! {
                                                                            div { class: "p-5 bg-slate-900 border border-slate-800 rounded-2xl flex flex-col justify-between space-y-3.5 hover:border-slate-700 transition-all duration-200",
                                                                                div { class: "space-y-1.5",
                                                                                    div { class: "flex items-center justify-between",
                                                                                        span { class: "text-sm font-bold text-slate-200", "{idx + 1}. {local.name}" }
                                                                                        span { class: "px-2 py-0.5 rounded font-mono text-xs font-bold {text_color} {bg_light}", "{k.earned_points:.1} / {k.max_points:.1} {t(locale, TK::LabelScore)}" }
                                                                                    }
                                                                                    p { class: "text-xs text-slate-500 leading-relaxed", "{local.meaning}" }
                                                                                }
                                                                                
                                                                                // Progress Bar
                                                                                div { class: "space-y-2",
                                                                                    div { class: "w-full bg-slate-800 rounded-full h-2 overflow-hidden",
                                                                                        div { 
                                                                                            class: "h-full rounded-full {bar_color} transition-all duration-500", 
                                                                                            style: "width: {pct}%" 
                                                                                        }
                                                                                    }
                                                                                    p { class: "text-[11px] {text_color} font-medium leading-relaxed", "{local.interpretation}" }
                                                                                }
                                                                            }
                                                                        }
                                                                    })}
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    rsx! { div {} }
                                                }
                                            },
                                            _ => rsx! { div {} }
                                        }
                                    }
                                },
```
