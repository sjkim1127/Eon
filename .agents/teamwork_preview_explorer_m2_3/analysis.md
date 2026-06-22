# Milestone M2 Analysis Report: Ashtakoota Guna Milan 상세 고도화 (R1)

## Executive Summary
This report analyzes the requirements for Milestone M2 (R1: Ashtakoota Guna Milan detailed enhancement). It reviews the compatibility engine code, routing layers, DTOs, and Dioxus UI, and designs a localized card-based interface with visual progress bars/gauges to replace the current basic table. The proposed design features full internationalization support for all 8 factors (Varna, Vashya, Tara, Yoni, Maitri, Gana, Bhakoot, Nadi), their meanings, detailed descriptions, and overall compatibility results across Korean, English, Chinese, and Russian.

---

## 1. Engine Analysis: `crates/eon-vedic/src/analysis/matching.rs`
The `MatchingEngine` computes 8 compatibility factors (Kootas) totaling 36 points (Gunas). Each factor is evaluated using the natal Moon position (Rasi and Nakshatra) of both partners.

| # | Koota Factor | Max Points | Base Calculation Logic |
|---|--------------|------------|------------------------|
| 1 | **Varna** (Caste/Vocation) | 1.0 | Rasi-based classification into 4 spiritual classes (Brahmin, Kshatriya, Vaishya, Shudra). Scores 1.0 if male's class $\ge$ female's class, else 0.0. |
| 2 | **Vashya** (Attraction/Control) | 2.0 | Rasi-based classification (Chatushpada, Manushya, Jalachara, Vanachara, Keeta). Scores 2.0 for same class, 1.0 or 0.5 for friendly classes, and 0.0 for incompatible ones. |
| 3 | **Tara** (Destiny/Health) | 3.0 | Calculates relative Nakshatra distance (modulo 9 + 1) in both directions. Inauspicious cycles are 3 (Vipat), 5 (Pratyak), 7 (Naidhana). Scores 3.0 if both directions are auspicious, 1.5 if one is, else 0.0 (Tara Dosha). |
| 4 | **Yoni** (Sensory/Sexual) | 4.0 | Nakshatra-based classification into 14 animal archetypes. Hostile pairs (e.g., Cow-Tiger, Cat-Rat) score 0.0. Same animal scores 4.0, friendly scores 3.0, neutral scores 2.0. |
| 5 | **Graha Maitri** (Friendship) | 5.0 | Compares natural friendship levels between Moon sign lords. Scores 5.0 (Friend-Friend), 4.0 (Friend-Neutral), 3.0 (Neutral-Neutral), 2.0 (Friend-Enemy), 1.0 (Neutral-Enemy), 0.0 (Enemy-Enemy). |
| 6 | **Gana** (Temperament) | 6.0 | Classifies Nakshatras into Deva (Divine), Manushya (Human), or Rakshasa (Demonic). Scores 6.0 for identical Gana, 5.0 for Deva-Manushya, 3.0 for Manushya-Rakshasa, and 1.0 for Deva-Rakshasa. |
| 7 | **Bhakoot** (Emotional Node) | 7.0 | Measures relative sign distance between Moon placements. Auspicious configurations (1/1, 7/7, 3/11, 4/10) score 7.0. Inauspicious configurations (2/12, 5/9, 6/8) score 0.0 (Bhakoot Dosha). |
| 8 | **Nadi** (Health/Genetics) | 8.0 | Classifies Nakshatras into Adi (Vata), Madhya (Pitta), or Antya (Kapha). Scores 8.0 if Nadis are different, else 0.0 (Nadi Dosha). |

### Mangal Dosha (Martial Affliction)
- Checks Mars house placement relative to Lagna and Moon (houses 1, 2, 4, 7, 8, 12).
- If both partners have Mangal Dosha, **Dosha Samya** occurs, cancelling the negative affliction (`mangal_dosha_cancelled = true`).

---

## 2. Service & DTO Routing: `crates/eon-service`
The compatibility request is routed through `crates/eon-service/src/services/vedic.rs` and utilizes structures from `crates/eon-service/src/dto.rs`.

- **Input DTO** (`VedicCompatibilityInput`):
  ```rust
  #[derive(Debug, Clone, Serialize, Deserialize)]
  #[serde(rename_all = "camelCase")]
  pub struct VedicCompatibilityInput {
      pub male: AnalysisInput,
      pub female: AnalysisInput,
  }
  ```
- **Output DTO** (`VedicCompatibilityOutput`):
  ```rust
  #[derive(Debug, Clone, Serialize, Deserialize)]
  #[serde(rename_all = "camelCase")]
  pub struct VedicCompatibilityOutput {
      pub meta: AnalysisMeta,
      pub report: eon_vedic::analysis::matching::CompatibilityReport,
  }
  ```
- **Routing logic** (`crates/eon-service/src/services/vedic.rs:92`):
  Calculates charts for both individuals using the `VedicChartCalculator` and calls `MatchingEngine::calculate_compatibility` to generate the compatibility report.

---

## 3. Current UI Review: `crates/eon-ui/src/components/tabs/vedic_tab.rs` (Lines 1997-2250)
The compatibility results are rendered conditionally inside the tab selection `3`.
- **Form submission**: Initiated by a button click triggering `run_compatibility`, which spawns a task executing `facade::analyze_vedic_compatibility(input)`.
- **Current Rendering**:
  - Line 2092: Overall compatibility header displaying `compat.report.total_score` and `compat.report.explanation`.
  - Line 2110: Mangal Dosha grid cards for male and female.
  - Line 2142: Ashtakoota Scorecard Table rendering a basic HTML `table` element with columns: factor name (`k.name`), maximum points (`k.max_points`), earned points (`k.earned_points`), and description (`k.description`).

---

## 4. UI Enhancement Design: Localized Card-based Progress Indicators
We propose replacing the plain table with a highly polished grid of cards. Each card represents one of the 8 factors, showing:
1. **Visual Progress Bar**: Coloured according to points (Emerald for max points, Purple for partial, Empty/Red highlight for 0 points indicating a Dosha).
2. **Cosmic Meaning**: A short subtitle describing what the factor measures.
3. **Earned vs Max Score Badge**: In the format `earned / max`.
4. **Localized Explanation**: Real-time evaluation of the result.

### Proposed UI Layout and Dioxus `rsx!` Code
```rust
// Replace lines 2141-2175 with a modern grid and localized card rendering
div { class: "space-y-4",
    div { class: "border-b border-slate-800 pb-2",
        h3 { class: "text-lg font-bold text-slate-200", 
            match locale {
                Locale::Ko => "아쉬타쿠타(Ashtakoota) 8대 세부 매칭",
                Locale::En => "Ashtakoota 8 Compatibility Factors",
                Locale::Zh => "阿什塔库塔 (Ashtakoota) 八大维度匹配",
                Locale::Ru => "8 факторов совместимости Аштакута",
            }
        }
    }
    
    div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
        {compat.report.kootas.iter().map(|k| {
            let koota_type = if k.name.starts_with("Varna") {
                "Varna"
            } else if k.name.starts_with("Vashya") {
                "Vashya"
            } else if k.name.starts_with("Tara") {
                "Tara"
            } else if k.name.starts_with("Yoni") {
                "Yoni"
            } else if k.name.starts_with("Graha Maitri") || k.name.starts_with("Maitri") {
                "Maitri"
            } else if k.name.starts_with("Gana") {
                "Gana"
            } else if k.name.starts_with("Bhakoot") {
                "Bhakoot"
            } else if k.name.starts_with("Nadi") {
                "Nadi"
            } else {
                "Unknown"
            };

            let (koota_title, koota_meaning, koota_result_desc) = match koota_type {
                "Varna" => {
                    let name = match locale {
                        Locale::Ko => "바르나 (Varna - 정신적 역량/직업적 성향)",
                        Locale::En => "Varna (Caste/Vocation)",
                        Locale::Zh => "瓦纳 (Varna - 精神素质/职业倾向)",
                        Locale::Ru => "Варна (Varna - Склонность/Класс)",
                    };
                    let meaning = match locale {
                        Locale::Ko => "상대방의 자아 상태와 정신적 깊이, 삶에 대처하는 본성적 기질 및 직업적 조화를 측정합니다.",
                        Locale::En => "Measures mental capacity, ego alignment, and natural vocational inclination.",
                        Locale::Zh => "衡量双方的精神觉悟、自我状态、以及应对生活时的天生气质与职业默契度。",
                        Locale::Ru => "Измеряет эго, духовное развитие и совместимость в сфере деятельности и призвании.",
                    };
                    let desc = if k.earned_points == 1.0 {
                        match locale {
                            Locale::Ko => "정신적 성향과 직업적 가치관이 조화롭게 잘 맞습니다.",
                            Locale::En => "Good alignment in mental inclination and work profile.",
                            Locale::Zh => "精神倾向与职业价值观非常契合。",
                            Locale::Ru => "Отличное соответствие профессиональных и духовных склонностей.",
                        }
                    } else {
                        match locale {
                            Locale::Ko => "서로 다른 내적 본성과 사회활동적 성향을 지니고 있습니다.",
                            Locale::En => "Differing natural vocations and inner nature.",
                            Locale::Zh => "双方拥有不同的天生本性与职业追求。",
                            Locale::Ru => "Различные врожденные склонности и жизненные ориентиры.",
                        }
                    };
                    (name, meaning, desc.to_string())
                },
                "Vashya" => {
                    let name = match locale {
                        Locale::Ko => "바시야 (Vashya - 지배력/상호 매력)",
                        Locale::En => "Vashya (Control/Attraction)",
                        Locale::Zh => "瓦夏 (Vashya - 相互吸引/控制力)",
                        Locale::Ru => "Вашья (Vashya - Влияние/Притяжение)",
                    };
                    let meaning = match locale {
                        Locale::Ko => "부부 사이의 상호 끌림, 통제력, 그리고 누가 누구에게 더 주도적인 영향을 미치는지를 분석합니다.",
                        Locale::En => "Measures mutual attraction, control dynamics, and dominant influence between partners.",
                        Locale::Zh => "分析夫妻之间的相互吸引力、控制权关系，以及谁对谁占有主导支配地位。",
                        Locale::Ru => "Измеряет взаимное притяжение, баланс контроля и доминирование партнеров друг над другом.",
                    };
                    let desc = match locale {
                        Locale::Ko => format!("상호 이끌림 점수: {}/2.0", k.earned_points),
                        Locale::En => format!("Mutual attraction rating: {}/2.0.", k.earned_points),
                        Locale::Zh => format!("相互吸引指数: {}/2.0。", k.earned_points),
                        Locale::Ru => format!("Показатель взаимного притяжения: {}/2.0.", k.earned_points),
                    };
                    (name, meaning, desc)
                },
                "Tara" => {
                    let name = match locale {
                        Locale::Ko => "타라 (Tara - 건강/운명적 상성)",
                        Locale::En => "Tara (Destiny/Health)",
                        Locale::Zh => "塔拉 (Tara - 健康/命运吉凶)",
                        Locale::Ru => "Тара (Tara - Судьба/Здоровье)",
                    };
                    let meaning = match locale {
                        Locale::Ko => "두 사람의 나크샤트라(달의 성좌) 간의 거리를 계산하여 건강, 장수, 그리고 상호 운명적인 길흉을 분석합니다.",
                        Locale::En => "Measures relationship longevity, health, and mutual destiny alignment based on Nakshatras.",
                        Locale::Zh => "通过双方星宿(Nakshatra)之间的距离，评估健康运、寿命以及彼此命运의 가치관을 분별합니다.",
                        Locale::Ru => "Рассчитывает совместимость судеб, здоровья и долголетия на основе взаимного расположения Накшатр.",
                    };
                    let desc = if k.earned_points == 3.0 {
                        match locale {
                            Locale::Ko => "탁월한 건강 관리와 운명적 장수의 조화를 보입니다.",
                            Locale::En => "Excellent destiny and longevity alignment.",
                            Locale::Zh => "极佳的寿命与命运吉凶调和。",
                            Locale::Ru => "Превосходная совместимость в плане здоровья и долголетия.",
                        }
                    } else if k.earned_points == 1.5 {
                        match locale {
                            Locale::Ko => "대체로 양호하지만 피로 누적 시 건강상의 주의가 요구됩니다.",
                            Locale::En => "Moderate health compatibility.",
                            Locale::Zh => "健康兼容性一般，需要注意作息与调理。",
                            Locale::Ru => "Умеренная совместимость здоровья.",
                        }
                    } else {
                        match locale {
                            Locale::Ko => "건강 또는 신체 에너지 조율이 필요합니다 (타라 도샤 경고).",
                            Locale::En => "Challenging health compatibility (Tara Dosha).",
                            Locale::Zh => "健康与气场产生冲突 (存在塔拉煞气 / Tara Dosha)。",
                            Locale::Ru => "Проблемы со здоровьем или энергетическим балансом (Тара Доша).",
                        }
                    };
                    (name, meaning, desc.to_string())
                },
                "Yoni" => {
                    let name = match locale {
                        Locale::Ko => "요니 (Yoni - 육체적/생물학적 조화)",
                        Locale::En => "Yoni (Sensory/Sexual)",
                        Locale::Zh => "约尼 (Yoni - 肉体/生理和谐)",
                        Locale::Ru => "Йони (Yoni - Физиология/Секс)",
                    };
                    let meaning = match locale {
                        Locale::Ko => "생물학적 동물 유형 매칭을 통해 육체적 교감, 감각적 이끌림 및 성적 호환성을 측정합니다.",
                        Locale::En => "Measures physical intimacy, sensory attraction, and biological/sexual compatibility using Nakshatra animal symbols.",
                        Locale::Zh => "通过星宿所属的动物原型，评测双方的身体默契、感官吸引以及性生活的和谐度。",
                        Locale::Ru => "Измеряет физическую близость, чувственное влечение и биологическую совместимость на основе тотемов животных Накшатр.",
                    };
                    let desc = match locale {
                        Locale::Ko => format!("생물학적 및 감각적 조화도: {}/4.0", k.earned_points),
                        Locale::En => format!("Biological compatibility: {}/4.0.", k.earned_points),
                        Locale::Zh => format!("生理与感官和谐度: {}/4.0。", k.earned_points),
                        Locale::Ru => format!("Биологическая совместимость: {}/4.0.", k.earned_points),
                    };
                    (name, meaning, desc)
                },
                "Maitri" => {
                    let name = match locale {
                        Locale::Ko => "그라하 마이트리 (Graha Maitri - 정신적 우정)",
                        Locale::En => "Graha Maitri (Friendship)",
                        Locale::Zh => "格拉哈迈特里 (Graha Maitri - 心理友谊)",
                        Locale::Ru => "Граха Майтри (Graha Maitri - Дружба)",
                    };
                    let meaning = match locale {
                        Locale::Ko => "출생 시 달이 속한 별자리의 지배성 간의 관계를 분석하여 정서적 대화, 가치관 및 친밀도를 측정합니다.",
                        Locale::En => "Measures intellectual harmony, shared values, and friendship quality based on the relationship between Moon sign lords.",
                        Locale::Zh => "分析双方月亮星座守护星之间的星曜关系，考量彼此在思想沟通、价值观以及友谊上的契合度。",
                        Locale::Ru => "Определяет интеллектуальную гармонию, схожесть взглядов и теплоту дружеских отношений по планетам-управителям знаков Луны.",
                    };
                    let desc = if k.earned_points >= 4.0 {
                        match locale {
                            Locale::Ko => "서로 깊이 이해하며 친구 같은 강한 정신적 유대감과 우정을 공유합니다.",
                            Locale::En => "High mental harmony and friendship.",
                            Locale::Zh => "心意相通，共享深厚的思想默契与朋友般的友谊。",
                            Locale::Ru => "Высокая ментальная гармония и дружба.",
                        }
                    } else if k.earned_points >= 2.0 {
                        match locale {
                            Locale::Ko => "보편적인 수준의 소통이 가능하며, 상호 양보를 통해 조화가 가능합니다.",
                            Locale::En => "Average mental harmony.",
                            Locale::Zh => "沟通正常，彼此可以通过包容来达成共识。",
                            Locale::Ru => "Средняя ментальная гармония.",
                        }
                    } else {
                        match locale {
                            Locale::Ko => "의사소통 방식이나 기본적인 사고관에 차이가 있어 갈등 우려가 있습니다.",
                            Locale::En => "Potential communication gaps.",
                            Locale::Zh => "在思维方式或核心观念上存在分歧，可能导致沟通障碍。",
                            Locale::Ru => "Возможные барьеры в общении и недопонимание.",
                        }
                    };
                    (name, meaning, desc.to_string())
                },
                "Gana" => {
                    let name = match locale {
                        Locale::Ko => "가나 (Gana - 성품/기질 유형)",
                        Locale::En => "Gana (Temperament)",
                        Locale::Zh => "加纳 (Gana - 气数/脾气类型)",
                        Locale::Ru => "Гана (Gana - Темперамент)",
                    };
                    let meaning = match locale {
                        Locale::Ko => "인간의 영혼 유형(Deva-신성, Manushya-인간성, Rakshasa-야생성)을 구분하여 성격적 조화와 마찰 정도를 측정합니다.",
                        Locale::En => "Measures character temperament and compatibility by categorizing Nakshatras into Deva (Divine), Manushya (Human), or Rakshasa (Demonic/Wild).",
                        Locale::Zh => "通过划分灵性本性（Deva天性、Manushya人性、Rakshasa狂野性）来衡量双方性格的协调性与冲突概率。",
                        Locale::Ru => "Сравнивает природу характера партнеров (Дева - Божественная, Манушья - Человеческая, Ракшаса - Демоническая).",
                    };
                    let desc = if k.earned_points >= 5.0 {
                        match locale {
                            Locale::Ko => "성품과 성격 유형이 조화를 이루어 마찰이 적고 편안합니다.",
                            Locale::En => "Compatible temperaments.",
                            Locale::Zh => "脾气气性相投，性格温和，冲突较少。",
                            Locale::Ru => "Совместимые темпераменты.",
                        }
                    } else if k.earned_points >= 3.0 {
                        match locale {
                            Locale::Ko => "기질 차이가 약간 존재하나, 의식적인 조율을 통해 극복할 수 있습니다.",
                            Locale::En => "Moderate temperament gaps.",
                            Locale::Zh => "气场存在些许差异，但可以通过意识层面的沟通来化解。",
                            Locale::Ru => "Умеренные различия в характерах.",
                        }
                    } else {
                        match locale {
                            Locale::Ko => "기질적 성향 차이가 큽니다. 갈등 시 조율이 절실히 요구됩니다 (가나 도샤 경고).",
                            Locale::En => "High temperamental friction (Gana Dosha).",
                            Locale::Zh => "性格本性差异巨大，容易产生强烈的脾气对立 (存在加纳煞气 / Gana Dosha)。",
                            Locale::Ru => "Высокое трение характеров (Гана Доша).",
                        }
                    };
                    (name, meaning, desc.to_string())
                },
                "Bhakoot" => {
                    let name = match locale {
                        Locale::Ko => "바쿠트 (Bhakoot - 감정적 결속/행복)",
                        Locale::En => "Bhakoot (Emotional Node)",
                        Locale::Zh => "巴库特 (Bhakoot - 情绪纽带/家庭兴旺)",
                        Locale::Ru => "Бакут (Bhakoot - Эмоции/Процветание)",
                    };
                    let meaning = match locale {
                        Locale::Ko => "서로의 달 별자리가 맺는 각도를 통해 감정적인 안정감, 부부의 애정 및 생활의 안락함을 측정합니다.",
                        Locale::En => "Measures emotional bonding, conjugal happiness, and long-term domestic prosperity based on Moon sign relationships.",
                        Locale::Zh => "通过双方月亮星座所呈的角度，测试情感的稳固性、婚姻幸福感以及长期的家庭繁荣。",
                        Locale::Ru => "Определяет эмоциональную близость, семейное благополучие и материальный достаток на основе углового расстояния между знаками Луны.",
                    };
                    let desc = if k.earned_points == 7.0 {
                        match locale {
                            Locale::Ko => "안정적이고 강한 정서적 결속력을 가집니다.",
                            Locale::En => "Stable emotional bond.",
                            Locale::Zh => "情感纽带稳固而深厚。",
                            Locale::Ru => "Стабильная эмоциональная связь.",
                        }
                    } else {
                        match locale {
                            Locale::Ko => "생활 중 감정 기복이 있거나 재정적 변동 주기가 올 수 있습니다 (바쿠트 도샤 경고).",
                            Locale::En => "Challenging emotional/financial cycles (Bhakoot Dosha).",
                            Locale::Zh => "情绪容易产生隔阂，或面临财务、家庭起伏的周期性考验 (存在巴库特煞气 / Bhakoot Dosha)。",
                            Locale::Ru => "Сложные эмоциональные или финансовые спады (Бхакут Доша).",
                        }
                    };
                    (name, meaning, desc.to_string())
                },
                "Nadi" => {
                    let name = match locale {
                        Locale::Ko => "나디 (Nadi - 건강/유전적 조합)",
                        Locale::En => "Nadi (Health/Genetics)",
                        Locale::Zh => "纳迪 (Nadi - 遗传健康/血脉)",
                        Locale::Ru => "Нади (Nadi - Генетика/Здоровье)",
                    };
                    let meaning = match locale {
                        Locale::Ko => "체질적 에너지(바타, 피타, 카파)의 대립을 확인하여 건강, 자녀 출산 운 및 유전적 결합의 조화를 측정합니다.",
                        Locale::En => "Measures genetic compatibility, lineage vitality, and physiological health alignment.",
                        Locale::Zh => "检查双方的体质能量类型（风、胆、痰），从而测评遗传基因的契合度及下一代的健康健康度。",
                        Locale::Ru => "Оценивает генетическое здоровье, крепость потомства и совместимость физиологических типов энергии (аюрведических конституций).",
                    };
                    let desc = if k.earned_points == 8.0 {
                        match locale {
                            Locale::Ko => "유전적 호환성이 뛰어나며 자녀 출산 및 유전적 건강에 적합합니다.",
                            Locale::En => "Good genetic health & lineage compatibility.",
                            Locale::Zh => "遗传基因极具互补性，适宜繁衍且健康运佳。",
                            Locale::Ru => "Прекрасное генетическое здоровье и совместимость рода.",
                        }
                    } else {
                        match locale {
                            Locale::Ko => "체질적 성향이 너무 일치하여 기 흐름의 불균형이 우려됩니다 (나디 도샤 경고).",
                            Locale::En => "Excessive similar energy (Nadi Dosha). Possible genetic mismatches.",
                            Locale::Zh => "体质能量过于相同，可能导致气场排斥或后代基因重叠 (存在纳迪煞气 / Nadi Dosha)。",
                            Locale::Ru => "Избыток схожей энергии (Нади Доша). Возможные генетические несоответствия.",
                        }
                    };
                    (name, meaning, desc.to_string())
                },
                _ => (k.name.as_str(), "", k.description.clone())
            };

            let progress_pct = (k.earned_points / k.max_points * 100.0).min(100.0);

            let (score_badge_class, progress_bar_color_class, result_text_color_class) = if k.earned_points == k.max_points {
                (
                    "bg-emerald-950/80 border border-emerald-800 text-emerald-400",
                    "bg-gradient-to-r from-emerald-500 to-teal-500",
                    "text-emerald-400"
                )
            } else if k.earned_points == 0.0 {
                (
                    "bg-rose-950/80 border border-rose-900/50 text-rose-400",
                    "bg-slate-800",
                    "text-rose-400 font-bold"
                )
            } else {
                (
                    "bg-purple-950/80 border border-purple-800 text-purple-400",
                    "bg-gradient-to-r from-purple-500 to-indigo-500",
                    "text-slate-300"
                )
            };

            rsx! {
                div { 
                    key: "{k.name}",
                    class: "p-5 bg-slate-900 border border-slate-800/80 rounded-2xl space-y-3.5 hover:border-slate-700/80 transition-all duration-200 shadow-lg relative overflow-hidden group",
                    
                    div { class: "absolute inset-0 bg-gradient-to-tr from-purple-500/0 via-indigo-500/0 to-indigo-500/5 opacity-0 group-hover:opacity-100 transition-opacity duration-300 pointer-events-none" }
                    
                    div { class: "flex justify-between items-start gap-2 z-10 relative",
                        div { class: "space-y-1",
                            h4 { class: "text-sm font-bold text-slate-200", "{koota_title}" }
                            p { class: "text-xs text-slate-500 leading-relaxed max-w-[85%]", "{koota_meaning}" }
                        }
                        span { class: "px-2.5 py-1 text-xs font-mono font-bold rounded-lg shrink-0 {score_badge_class}",
                            "{k.earned_points:.1} / {k.max_points:.1}"
                        }
                    }
                    
                    div { class: "w-full h-2.5 bg-slate-950/80 border border-slate-800/40 rounded-full overflow-hidden relative",
                        if k.earned_points > 0.0 {
                            div { 
                                class: "h-full rounded-full transition-all duration-500 {progress_bar_color_class}",
                                style: "width: {progress_pct}%"
                            }
                        }
                    }

                    div { class: "flex items-center gap-1.5 pt-1 z-10 relative",
                        if k.earned_points == 0.0 {
                            span { class: "text-rose-400 shrink-0", "⚠️" }
                        } else if k.earned_points == k.max_points {
                            span { class: "text-emerald-400 shrink-0", "✓" }
                        } else {
                            span { class: "text-purple-400 shrink-0", "•" }
                        }
                        span { class: "text-xs font-medium {result_text_color_class}",
                            "{koota_result_desc}"
                        }
                    }
                }
            }
        })}
    }
}
```

---

## 5. Precise Translation Mapping
To support a fully localized experience, the following translations will be utilized.

### 5-1. Overall Compatibility & Judgments
| Language | Overall Judgment Title | Highly Compatible | Caution Advised | Compatible Explanation | High Score w/ Dosha Explanation | Low Score Explanation | Score Template |
|---|---|---|---|---|---|---|---|
| **Ko** | 종합 매칭 판정 | ✓ 상성 우수 | ⚠️ 신중함 요구 | 전반적으로 조화로운 매칭입니다. 추천합니다. | 점수는 높으나 주요 살(Nadi/Bhakoot Dosha)의 영향으로 신중한 주의가 필요합니다. | 성향적 차이가 커 상호 조율과 깊은 이해가 요구되는 상성입니다. | 호환성 점수: {} / 36 Gunas |
| **En** | Overall Compatibility Judgment | ✓ Highly Compatible | ⚠️ Caution Advised | Overall harmonious matchup. Highly recommended. | Score is high, but key afflictions (Nadi/Bhakoot Dosha) indicate cautious attention is required. | Significant temperamental differences. Mutual adjustment and deep understanding are needed. | Compatibility Score: {} / 36 Gunas |
| **Zh** | 综合匹配判定 | ✓ 相性极佳 | ⚠️ 建议慎重 | 总体配对非常和谐。强烈推荐。 | 分数虽高，但受主要煞气 (Nadi/Bhakoot Dosha) 影响，仍需多加注意与克制。 | 性情差异较大，需要双方在婚后有更多的包容与深层理解。 | 相性评分: {} / 36 Gunas |
| **Ru** | Комплексная оценка совместимости | ✓ Высокая совместимость | ⚠️ Требуется осторожность | В целом очень гармоничный союз. Рекомендуется. | Балл высокий, но из-за критических поражений (Нади/Бхакут Доша) требуется особое внимание. | Значительные различия в характерах. Потребуются взаимные уступки и глубокое терпение. | Очки совместимости: {} из 36 Гун |

### 5-2. Mangal Dosha Localizations
| Language | Male Label | Female Label | Detected | Not Detected | Cancelled Notice |
|---|---|---|---|---|---|
| **Ko** | 남성 화성살 (Male Mangal Dosha) | 여성 화성살 (Female Mangal Dosha) | 🔥 화성살(Manglik) 감지 | ✓ 해당 없음 (양호) | ℹ️ 상호 화성살 상쇄(Dosha Samya)가 성립되어 화성살의 부정적 영향이 소멸되었습니다. |
| **En** | Male Mangal Dosha (Manglik) | Female Mangal Dosha (Manglik) | 🔥 Mangal Dosha Detected | ✓ Not Applicable (Good) | ℹ️ Dosha Samya established: Mangal Dosha of both partners cancel each other's negative effects. |
| **Zh** | 男方火星煞 (Male Mangal Dosha) | 女方火星煞 (Female Mangal Dosha) | 🔥 检测到火星煞 (Manglik) | ✓ 无此煞 (良好) | ℹ️ 双方火星煞相互抵消 (Dosha Samya)，火星煞的负面影响已消除。 |
| **Ru** | Мангалик-доша у мужчины (Male Mangal) | Мангалик-доша у женщины (Female Mangal) | 🔥 Обнаружена Мангала Доша | ✓ Не применимо (Хорошо) | ℹ️ Установлена Доша Самья: Мангала Доша у обоих партнеров нейтрализует негативные эффекты. |

---

## 6. Implementation Steps for Next Developer
1. **Dioxus Router / state**: Ensure `Locale` is resolved as `locale` in the compatibility matching logic section.
2. **Replace UI Layout**: Insert the Dioxus card-grid `rsx!` block in `crates/eon-ui/src/components/tabs/vedic_tab.rs` (replacing the scorecard table section).
3. **Incorporate i18n Keys**: If desired, standard keys can be added to `crates/eon-ui/src/i18n/mod.rs` and the corresponding translation tables to avoid inline match blocks inside the UI file, though the structured local match approach proposed above minimizes codebase coupling.
4. **Wasm Verification**: Run `cargo check` and `dx build` within `crates/eon-ui` to verify compiler safety and correct styling.
