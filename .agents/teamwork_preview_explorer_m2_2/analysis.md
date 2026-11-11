# Milestone M2 Requirements Analysis: Ashtakoota Guna Milan Detail Enhancement (R1)

## Executive Summary
This report analyzes the requirements for **Milestone M2: Ashtakoota Guna Milan Detailed Refinement (R1)**. The objective is to design UI/UX changes to display the 8 Ashtakoota compatibility factors using localized progress bars/gauges, showing earned vs. max points and their interpretations across Korean, English, Chinese, and Russian languages. Because the backend (`crates/eon-vedic/src/analysis/matching.rs`) returns compatibility results with hardcoded English names and description strings, we have designed a clean **frontend-only dynamic localization architecture** that maps the backend output to i18n translation keys on the fly. This enables full internationalization and a premium UI presentation without modifying any backend code.

---

## 1. Engine Analysis: `matching.rs`
The Vedic compatibility calculations are performed in `crates/eon-vedic/src/analysis/matching.rs` within `MatchingEngine::calculate_compatibility`.

### 8 Koota Factors & Points Allocation
The total score is calculated out of **36 Gunas (points)** distributed across 8 factors:

| Order | Koota Factor | Max Points | Calculation Basis | Critical Dosha Condition |
|---|---|---|---|---|
| 1 | **Varna** (Caste/Vocation) | 1.0 | Rasi-based cast ranking (Brahmin=4, Kshatriya=3, Vaishya=2, Shudra=1). 1.0 if male >= female, else 0.0. | None |
| 2 | **Vashya** (Control/Attraction) | 2.0 | Rasi group matching (Chatushpada, Manushya, Jalachara, Vanachara, Keeta). | None |
| 3 | **Tara** (Destiny/Health) | 3.0 | Relative distance between Nakshatras. 3.0 if both auspicious, 1.5 if one is, else 0.0. | None |
| 4 | **Yoni** (Sensory/Sexual) | 4.0 | Nakshatra animal affinity (14 categories). 0.0 if enemy pairs, 4.0 if same animal, 3.0 if friendly, 2.0 if neutral. | None |
| 5 | **Graha Maitri** (Friendship) | 5.0 | Natural relationship between Rasi ruling lords. | None |
| 6 | **Gana** (Temperament) | 6.0 | Nakshatra Gana groups (Deva, Manushya, Rakshasa). | None |
| 7 | **Bhakoot** (Emotional Node) | 7.0 | Rasi relative position. 7.0 if auspicious (1, 7, 3, 11, 4, 10), else 0.0. | **Bhakoot Dosha** (0.0 points) |
| 8 | **Nadi** (Health/Genetics) | 8.0 | Nakshatra Nadi groups (Adi, Madhya, Antya). 8.0 if different, else 0.0. | **Nadi Dosha** (0.0 points) |

### Key Engine Structs
- **`KootaScore`**:
  ```rust
  pub struct KootaScore {
      pub name: String,
      pub max_points: f64,
      pub earned_points: f64,
      pub description: String,
  }
  ```
- **`CompatibilityReport`**:
  ```rust
  pub struct CompatibilityReport {
      pub total_score: f64,
      pub is_compatible: bool, // total >= 18 and no critical Nadi/Bhakoot dosha
      pub kootas: Vec<KootaScore>,
      pub male_mangal_dosha: bool,
      pub female_mangal_dosha: bool,
      pub mangal_dosha_cancelled: bool,
      pub explanation: String,
  }
  ```

---

## 2. Service & DTO Routing Analysis
- **Service Layer (`crates/eon-service/src/services/vedic.rs`)**:
  - `analyze_compatibility` prepares birth contexts for male and female inputs, calculates the Vedic charts using `VedicChartCalculator`, calls `MatchingEngine::calculate_compatibility(&male_chart, &female_chart)`, and returns `VedicCompatibilityOutput`.
- **DTOs (`crates/eon-service/src/dto.rs`)**:
  - `VedicCompatibilityInput` bundles male and female `AnalysisInput`.
  - `VedicCompatibilityOutput` contains `meta: AnalysisMeta` and `report: CompatibilityReport`.

---

## 3. UI Implementation Analysis
In `crates/eon-ui/src/components/tabs/vedic_tab.rs` (lines 1997-2250):
- **Input Forms**: Lines 2000-2069 collect partner's year, month, day, hour, minute, latitude, and longitude.
- **Trigger**: The `"💞 Compatibility  "` button triggers `run_compatibility`, which calls the async facade function `facade::analyze_vedic_compatibility`.
- **State**: The component reads the task state from `compat_status` (`TaskStatus`) and compatibility output from `compat_data`.
- **Current Table Rendering (Lines 2142-2175)**: Renders a traditional table with columns `  (Koota)`, ` (Max)`, ` `, and ``. All headers and overall summaries are currently hardcoded in Korean.

---

## 4. Proposed UI/UX Designs
Instead of a flat table, we propose a responsive **Grid Card Layout** with visual progress bars:
1. **Grid Container**: A 2-column responsive layout (`grid grid-cols-1 md:grid-cols-2 gap-4`).
2. **Visual Indicators**:
   - Each factor is displayed in a dedicated card.
   - **Progress Bar**: Displays the ratio of earned points to max points.
     - **Green/Emerald Gradient** (`from-emerald-500 to-teal-500`): Full or excellent points ($\ge 99\%$).
     - **Purple/Indigo Gradient** ($0\% < \text{earned} < 100\%$): Partial score.
     - **Red/Rose Gradient** ($\le 1\%$): Zero score.
   - **Critical Dosha Alerts**: A prominent warning badge (`⚠️ Critical Dosha`) is displayed if `Nadi` or `Bhakoot` score is `0.0`.
3. **Responsive Cards**: Cards leverage transition animations (`hover:scale-[1.01] transition-all duration-200`) and structured spacing.

---

## 5. i18n Translation Mapping Strategy

Because backend descriptions are hardcoded English strings, we translate them dynamically in the frontend by checking the name and content of each factor.

### Proposed i18n Keys (`crates/eon-ui/src/i18n/mod.rs`)
Add the following keys to the `TK` enum:
```rust
    // Vedic Compatibility
    VedicCompatPartnerInput,
    VedicCompatRun,
    VedicCompatCalculating,
    VedicCompatOverallHeader,
    VedicCompatScoreText,
    VedicCompatHarmonious,
    VedicCompatCautionDosha,
    VedicCompatFriction,
    VedicCompatExcellent,
    VedicCompatAttention,
    VedicCompatMaleMangal,
    VedicCompatFemaleMangal,
    VedicCompatMangalDetected,
    VedicCompatMangalNone,
    VedicCompatMangalCancelled,
    VedicCompatTableTitle,
    VedicCompatColFactor,
    VedicCompatColMax,
    VedicCompatColEarned,
    VedicCompatColDesc,
    
    // 8 Kootas Names
    VedicKootaVarna,
    VedicKootaVashya,
    VedicKootaTara,
    VedicKootaYoni,
    VedicKootaMaitri,
    VedicKootaGana,
    VedicKootaBhakoot,
    VedicKootaNadi,
    
    // Varna Descriptions
    VedicVarnaGood,
    VedicVarnaDiff,
    
    // Vashya Descriptions
    VedicVashyaRating,
    
    // Tara Descriptions
    VedicTaraExcellent,
    VedicTaraModerate,
    VedicTaraChallenging,
    
    // Yoni Descriptions
    VedicYoniRating,
    
    // Maitri Descriptions
    VedicMaitriHigh,
    VedicMaitriAverage,
    VedicMaitriGap,
    
    // Gana Descriptions
    VedicGanaCompatible,
    VedicGanaModerate,
    VedicGanaFriction,
    
    // Bhakoot Descriptions
    VedicBhakootStable,
    VedicBhakootChallenging,
    
    // Nadi Descriptions
    VedicNadiGood,
    VedicNadiDosha,
```

### Multilingual Translation Dictionary
The translations for `ko.rs`, `en.rs`, `zh.rs`, and `ru.rs` are mapped as follows:

| Translation Key | Korean (`ko.rs`) | English (`en.rs`) | Chinese (`zh.rs`) | Russian (`ru.rs`) |
|---|---|---|---|---|
| `VedicCompatPartnerInput` | "Enter Partner Birth Information" | "Enter Partner Birth Info" | "输入对方出生信息" | "Введите данные рождения партнера" |
| `VedicCompatRun` | "💞 Compatibility  " | "💞 Run Compatibility Analysis" | "💞 运行合婚分析" | "💞 Запустить анализ совместимости" |
| `VedicCompatCalculating` | "Compatibility  ..." | "Computing compatibility..." | "正在计算合婚..." | "Вычисление совместимости..." |
| `VedicCompatOverallHeader` | "  " | "Overall Compatibility Judgment" | "综合匹配判定" | "Общая оценка совместимости" |
| `VedicCompatScoreText` | " : " | "Compatibility Score: " | "匹配分数: " | "Оценка совместимости: " |
| `VedicCompatHarmonious` | "  . ." | "Overall harmonious match. Highly recommended." | "整体和谐，推荐交往。" | "Гармоничный союз. Рекомендуется." |
| `VedicCompatCautionDosha` | "   (Nadi/Bhakoot Dosha)    ." | "High score, but caution required due to critical Doshas (Nadi/Bhakoot)." | "得分较高，但因主要煞神（Nadi/Bhakoot Dosha）需谨慎。" | "Высокий балл, но нужна осторожность из-за Дош (Нади/Бхакут)." |
| `VedicCompatFriction` | "        ." | "Significant differences in temperament; mutual understanding required." | "性格差异较大，需要相互包容和深度理解。" | "Значительные различия; требуется глубокое взаимопонимание." |
| `VedicCompatExcellent` | "✓  " | "✓ Excellent Match" | "✓ 极佳匹配" | "✓ Отличная совместимость" |
| `VedicCompatAttention` | "⚠️  " | "⚠️ Caution Required" | "⚠️ 需谨慎" | "⚠️ Требуется осторожность" |
| `VedicCompatMaleMangal` | "  (Male Mangal Dosha)" | "Male Mangal Dosha" | "男性火星煞 (Mangal Dosha)" | "Мужская Мангала Доша" |
| `VedicCompatFemaleMangal` | "  (Female Mangal Dosha)" | "Female Mangal Dosha" | "女性火星煞 (Mangal Dosha)" | "Женская Мангала Доша" |
| `VedicCompatMangalDetected` | "🔥 (Manglik) " | "🔥 Manglik Detected" | "🔥 检测到火星煞 (Manglik)" | "🔥 Обнаружена Манглик" |
| `VedicCompatMangalNone` | "✓   ()" | "✓ Not Applicable (Good)" | "✓ 无 (良好)" | "✓ Не применимо (Хорошо)" |
| `VedicCompatMangalCancelled` | "ℹ️   (Dosha Samya)     ." | "ℹ️ Mutual Mangal Dosha Cancellation (Dosha Samya) is active; negative effects are neutralized." | "ℹ️ 相互火星煞抵消 (Dosha Samya) 成立，负面影响已消除。" | "ℹ️ Взаимное устранение Мангала Доши (Доша Самья) активно; негативные эффекты сняты." |
| `VedicCompatTableTitle` | "Ashtakoota   " | "Ashtakoota Detailed Compatibility Table" | "八合 (Ashtakoota) 详细匹配评分表" | "Подробная таблица совместимости Аштакута" |
| `VedicCompatColFactor` | "  (Koota)" | "Factor (Koota)" | "匹配要素 (Koota)" | "Фактор (Кута)" |
| `VedicCompatColMax` | " (Max)" | "Max Points" | "权重上限" | "Макс. баллы" |
| `VedicCompatColEarned` | " " | "Earned Points" | "获得分数" | "Набранные баллы" |
| `VedicCompatColDesc` | "" | "Description" | "说明" | "Описание" |
| `VedicKootaVarna` | " (Varna - /)" | "Varna (Caste/Vocation)" | "瓦尔纳 (Varna - 种姓/职业)" | "Варна (Varna - Каста/Призвание)" |
| `VedicKootaVashya` | " (Vashya -  /)" | "Vashya (Control/Attraction)" | "瓦施亚 (Vashya - 相互吸引/支配)" | "Вашья (Vashya - Притяжение/Контроль)" |
| `VedicKootaTara` | " (Tara - /)" | "Tara (Destiny/Health)" | "塔拉 (Tara - 健康/命运)" | "Тара (Tara - Судьба/Здоровье)" |
| `VedicKootaYoni` | " (Yoni - / Compatibility)" | "Yoni (Sensory/Sexual)" | "约尼 (Yoni - 生物/性吸引力)" | "Йони (Yoni - Сексуальная совместимость)" |
| `VedicKootaMaitri` | "  (Graha Maitri -  /)" | "Graha Maitri (Friendship/Mental)" | "友谊 (Graha Maitri - 精神契合/友情)" | "Граха Майтри (Graha Maitri - Дружба/Разум)" |
| `VedicKootaGana` | " (Gana - /)" | "Gana (Temperament/Behavior)" | "迦纳 (Gana - 气质/行为方式)" | "Гана (Gana - Темперамент/Поведение)" |
| `VedicKootaBhakoot` | " (Bhakoot - /)" | "Bhakoot (Emotional Node/Relationship)" | "巴库特 (Bhakoot - 情感/家庭)" | "Бхакут (Bhakoot - Эмоции/Отношения)" |
| `VedicKootaNadi` | " (Nadi - / )" | "Nadi (Health/Genetics)" | "纳迪 (Nadi - 体质/遗传健康)" | "Нади (Nadi - Здоровье/Генетика)" |
| `VedicVarnaGood` | " /   ." | "Good work-profile alignment." | "自然的职业与社会倾向契合良好。" | "Хорошее соответствие профессиональных наклонностей." |
| `VedicVarnaDiff` | "     ." | "Differing natural vocations." | "天然的职业志向和倾向存在差异。" | "Различные естественные наклонности и призвания." |
| `VedicVashyaRating` | "    : {}/2." | "Mutual attraction rating: {}/2." | "相互吸引与支配协调度: {}/2。" | "Рейтинг взаимного притяжения: {}/2." |
| `VedicTaraExcellent` | "      ." | "Excellent destiny and longevity alignment." | "卓越的健康与命运协同效应。" | "Отличная совместимость судеб и долголетия." |
| `VedicTaraModerate` | "     ." | "Moderate health compatibility." | "中等水平的健康兼容性。" | "Умеренная совместимость здоровья." |
| `VedicTaraChallenging` | "   Compatibility ( )." | "Challenging health compatibility (Tara Dosha)." | "健康方面需要注意的匹配 (Tara Dosha)。" | "Проблемная совместимость здоровья (Тара Доша)." |
| `VedicYoniRating` | "   : {}/4." | "Biological compatibility: {}/4." | "生物学与肉体亲密度: {}/4。" | "Биологическая совместимость: {}/4." |
| `VedicMaitriHigh` | "      ." | "High mental harmony and friendship." | "极高沟通水平与深厚精神纽带。" | "Высокая ментальная гармония и дружба." |
| `VedicMaitriAverage` | "    Compatibility." | "Average mental harmony." | "平稳普通的精神契合度。" | "Средняя ментальная совместимость." |
| `VedicMaitriGap` | "       ." | "Potential communication gaps." | "容易出现沟通和相互理解的偏差。" | "Возможны проблемы в общении и взаимопонимании." |
| `VedicGanaCompatible` | "     ." | "Compatible temperaments." | "性情相似，冲突少且舒适。" | "Совместимые темпераменты, мало трения." |
| `VedicGanaModerate` | "       ." | "Moderate temperament gaps." | "存在中等性格差异，需要相互体谅。" | "Умеренные различия в характере, требуется понимание." |
| `VedicGanaFriction` | "       ( )." | "High temperamental friction (Gana Dosha)." | "因性格差异容易产生剧烈冲突 (Gana Dosha)。" | "Высокое трение из-за разницы темпераментов (Гана Доша)." |
| `VedicBhakootStable` | "    ." | "Stable emotional bond." | "情感紧密，夫妻生活稳定。" | "Стабильная эмоциональная связь." |
| `VedicBhakootChallenging` | "       ( )." | "Challenging emotional/financial cycles (Bhakoot Dosha)." | "易引发情绪波动与财务起伏 (Bhakoot Dosha)。" | "Возможны эмоциональные и финансовые спады (Бхакут Доша)." |
| `VedicNadiGood` | "      ." | "Good genetic health & lineage compatibility." | "极佳的生理平衡与健康子女运。" | "Хорошее генетическое здоровье и совместимость потомства." |
| `VedicNadiDosha` | "      ( )." | "Excessive similar energy (Nadi Dosha). Possible genetic mismatches." | "体质能量过度相似导致冲突 (Nadi Dosha)。" | "Избыток схожей конституциональной энергии (Нади Доша)." |

---

## 6. Target Code Structure & Proposed Code Changes
Below are the exact code modifications to apply when implementing Milestone M2 (R1).

### Part 6-1: Localizing `crates/eon-ui/src/components/tabs/vedic_tab.rs`
Replace the compatibility rendering block (lines 2087 to 2185) with:

```rust
TaskStatus::Success => {
    if let Some(compat) = &*compat_data.read() {
        rsx! {
            div { class: "space-y-6 animate-in fade-in duration-500",
                // Overall Compatibility Header
                div { class: "p-5 bg-slate-900 border border-slate-800 rounded-2xl flex flex-col md:flex-row md:items-center justify-between gap-4",
                    div {
                        h3 { class: "text-xs text-slate-500 uppercase tracking-widest font-bold", "{t(locale, TK::VedicCompatOverallHeader)}" }
                        p { class: "text-2xl font-bold text-slate-200 mt-1",
                            "{t(locale, TK::VedicCompatScoreText).replace(\"{}\", &format!(\"{:.1}\", compat.report.total_score))} "
                            span { class: "text-purple-400", "/ 36 Gunas" }
                        }
                        p { class: "text-sm text-slate-400 mt-1.5", 
                            if compat.report.is_compatible {
                                t(locale, TK::VedicCompatHarmonious)
                            } else if compat.report.total_score >= 18.0 {
                                t(locale, TK::VedicCompatCautionDosha)
                            } else {
                                t(locale, TK::VedicCompatFriction)
                            }
                        }
                    }
                    div { class: "flex gap-2",
                        if compat.report.is_compatible {
                            span { class: "px-4 py-2 rounded-xl bg-emerald-950/60 border border-emerald-800/60 text-emerald-400 text-sm font-bold", "{t(locale, TK::VedicCompatExcellent)}" }
                        } else {
                            span { class: "px-4 py-2 rounded-xl bg-amber-950/60 border border-amber-800/60 text-amber-400 text-sm font-bold", "{t(locale, TK::VedicCompatAttention)}" }
                        }
                    }
                }

                // Mangal Dosha Card
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                    div { class: "p-4 rounded-2xl bg-slate-900 border border-slate-800 space-y-2",
                        h4 { class: "text-xs text-slate-500 font-semibold uppercase tracking-wider", "{t(locale, TK::VedicCompatMaleMangal)}" }
                        p {
                            class: if compat.report.male_mangal_dosha {
                                "text-lg font-bold text-red-400"
                            } else {
                                "text-lg font-bold text-emerald-400"
                            },
                            if compat.report.male_mangal_dosha { t(locale, TK::VedicCompatMangalDetected) } else { t(locale, TK::VedicCompatMangalNone) }
                        }
                    }
                    div { class: "p-4 rounded-2xl bg-slate-900 border border-slate-800 space-y-2",
                        h4 { class: "text-xs text-slate-500 font-semibold uppercase tracking-wider", "{t(locale, TK::VedicCompatFemaleMangal)}" }
                        p {
                            class: if compat.report.female_mangal_dosha {
                                "text-lg font-bold text-red-400"
                            } else {
                                "text-lg font-bold text-emerald-400"
                            },
                            if compat.report.female_mangal_dosha { t(locale, TK::VedicCompatMangalDetected) } else { t(locale, TK::VedicCompatMangalNone) }
                        }
                    }
                }
                if compat.report.mangal_dosha_cancelled {
                    div { class: "p-4 rounded-xl bg-blue-950/40 border border-blue-800/40 text-blue-300 text-xs font-semibold",
                        "{t(locale, TK::VedicCompatMangalCancelled)}"
                    }
                }

                // Ashtakoota Scorecard Grid with Progress Bars & Localized Descriptions
                div { class: "space-y-4",
                    h3 { class: "font-semibold text-slate-200 text-lg px-1", "{t(locale, TK::VedicCompatTableTitle)}" }
                    
                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                        {compat.report.kootas.iter().map(|k| {
                            // Translate Koota Name
                            let name_key = if k.name.starts_with("Varna") {
                                TK::VedicKootaVarna
                            } else if k.name.starts_with("Vashya") {
                                TK::VedicKootaVashya
                            } else if k.name.starts_with("Tara") {
                                TK::VedicKootaTara
                            } else if k.name.starts_with("Yoni") {
                                TK::VedicKootaYoni
                            } else if k.name.starts_with("Graha Maitri") {
                                TK::VedicKootaMaitri
                            } else if k.name.starts_with("Gana") {
                                TK::VedicKootaGana
                            } else if k.name.starts_with("Bhakoot") {
                                TK::VedicKootaBhakoot
                            } else if k.name.starts_with("Nadi") {
                                TK::VedicKootaNadi
                            } else {
                                TK::LabelNone
                            };
                            let localized_name = if name_key != TK::LabelNone {
                                t(locale, name_key)
                            } else {
                                &k.name
                            };

                            // Translate Koota Description
                            let localized_desc = if k.name.starts_with("Varna") {
                                if k.earned_points == 1.0 {
                                    t(locale, TK::VedicVarnaGood).to_string()
                                } else {
                                    t(locale, TK::VedicVarnaDiff).to_string()
                                }
                            } else if k.name.starts_with("Vashya") {
                                t(locale, TK::VedicVashyaRating).replace("{}", &format!("{:.1}", k.earned_points))
                            } else if k.name.starts_with("Tara") {
                                if k.earned_points == 3.0 {
                                    t(locale, TK::VedicTaraExcellent).to_string()
                                } else if k.earned_points == 1.5 {
                                    t(locale, TK::VedicTaraModerate).to_string()
                                } else {
                                    t(locale, TK::VedicTaraChallenging).to_string()
                                }
                            } else if k.name.starts_with("Yoni") {
                                t(locale, TK::VedicYoniRating).replace("{}", &format!("{:.0}", k.earned_points))
                            } else if k.name.starts_with("Graha Maitri") {
                                if k.earned_points >= 4.0 {
                                    t(locale, TK::VedicMaitriHigh).to_string()
                                } else if k.earned_points >= 2.0 {
                                    t(locale, TK::VedicMaitriAverage).to_string()
                                } else {
                                    t(locale, TK::VedicMaitriGap).to_string()
                                }
                            } else if k.name.starts_with("Gana") {
                                if k.earned_points >= 5.0 {
                                    t(locale, TK::VedicGanaCompatible).to_string()
                                } else if k.earned_points >= 3.0 {
                                    t(locale, TK::VedicGanaModerate).to_string()
                                } else {
                                    t(locale, TK::VedicGanaFriction).to_string()
                                }
                            } else if k.name.starts_with("Bhakoot") {
                                if k.earned_points == 7.0 {
                                    t(locale, TK::VedicBhakootStable).to_string()
                                } else {
                                    t(locale, TK::VedicBhakootChallenging).to_string()
                                }
                            } else if k.name.starts_with("Nadi") {
                                if k.earned_points == 8.0 {
                                    t(locale, TK::VedicNadiGood).to_string()
                                } else {
                                    t(locale, TK::VedicNadiDosha).to_string()
                                }
                            } else {
                                k.description.clone()
                            };

                            let pct = k.earned_points / k.max_points;
                            let (bar_color, text_color, card_bg) = if pct >= 0.99 {
                                ("bg-gradient-to-r from-emerald-500 to-teal-500", "text-emerald-400", "bg-slate-900 border-slate-800/80")
                            } else if pct <= 0.01 {
                                ("bg-gradient-to-r from-red-600 to-rose-500", "text-rose-400", "bg-rose-950/10 border-rose-900/30")
                            } else {
                                ("bg-gradient-to-r from-purple-500 to-indigo-500", "text-purple-400", "bg-slate-900 border-slate-800/80")
                            };

                            let is_critical_dosha = pct <= 0.01 && (k.name.starts_with("Nadi") || k.name.starts_with("Bhakoot"));

                            rsx! {
                                div { class: "p-5 border rounded-2xl flex flex-col justify-between gap-3 transition-all hover:scale-[1.01] duration-200 {card_bg}",
                                    div { class: "space-y-1",
                                        div { class: "flex items-center justify-between",
                                            span { class: "font-semibold text-slate-200 text-sm md:text-base", "{localized_name}" }
                                            span { class: "font-mono font-bold text-xs md:text-sm {text_color}", "{k.earned_points:.1} / {k.max_points:.1}" }
                                        }
                                        if is_critical_dosha {
                                            div { class: "inline-flex items-center px-2 py-0.5 rounded text-[10px] font-bold bg-rose-900/50 text-rose-300 border border-rose-800/30 mt-1 uppercase tracking-wider",
                                                "⚠️ Critical Dosha"
                                            }
                                        }
                                    }
                                    
                                    div { class: "w-full bg-slate-800/60 rounded-full h-2 mt-1.5 overflow-hidden",
                                        div {
                                            class: "h-full rounded-full transition-all duration-500 {bar_color}",
                                            style: format!("width: {}%", pct * 100.0)
                                        }
                                    }
                                    
                                    p { class: "text-xs text-slate-400 leading-relaxed mt-2", "{localized_desc}" }
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
}
```
