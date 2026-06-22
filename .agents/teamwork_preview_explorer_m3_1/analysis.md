# Analysis Report — Milestone M3 (Shadbala & Robustness)

This report details the findings and designs for the Milestone M3 requirements, addressing the M2 Challenger robustness issues, reviewing Shadbala engine calculations, designing the Strength scorecard grid UI, and defining the localization strings.

---

## 1. M2 Challenger Robustness & Vulnerability Fixes

The M2 Challengers identified 5 vulnerabilities in `crates/eon-vedic/src/analysis/matching.rs` and related tests. Below are the precise root-cause analyses and proposed code modifications to mitigate them.

### Challenge 1: Unsafe Unwrap on Missing Moon Planet
* **Root Cause**: `MatchingEngine::calculate_compatibility` queries the input `VedicChart` for `VedicPlanet::Moon` using `.find()` and calls `.unwrap()` (lines 34–35). If Moon is missing (e.g. from an incomplete/mock chart), this immediately panics and halts the thread.
* **Proposed Mitigation**: Refactor `calculate_compatibility` to return a `Result<CompatibilityReport, String>` and use the `?` operator to safely propagate the error.

### Challenge 2: Unsafe Unwrap in Mangal Dosha Check
* **Root Cause**: In `check_mangal_dosha` (lines 376–393), if Mars is found, it automatically performs `chart.planets.iter().find(|p| p.planet == VedicPlanet::Moon).unwrap()`. If Mars is present but Moon is missing, it crashes on `.unwrap()`.
* **Proposed Mitigation**: Safely query the Moon in `check_mangal_dosha` using `if let Some(moon) = ...` and fallback to verifying only the Lagna house index if Moon is missing.

### Challenge 3 & 4: Nakshatra = 0 Underflow & Negative Modulo Tara
* **Root Causes**:
  * **Underflow**: If `nakshatra` is 0 (due to mock data), equations subtracting 1 from it (e.g., in `get_nakshatra_lord` or `calculate_tara`) can trigger an arithmetic underflow. In debug mode, this panics; in release mode, it wraps around to 255 and yields silent incorrect results.
  * **Negative Modulo**: In `calculate_tara` (lines 205–206), if `male_nak = 0` and `female_nak = 28`, `male_nak as i16 - female_nak as i16 + 27` equals `-1`. In Rust, `-1 % 9` evaluates to `-1`, resulting in `dist = 0` after `+ 1`. Since 0 is not in the inauspicious list `[3, 5, 7]`, the engine incorrectly returns a perfect `3.0` score.
* **Proposed Mitigation**: Add strict input validation at the entry of `calculate_compatibility` to check that Moon's `nakshatra` resides in `1..=27` and `rasi` in `1..=12` for both charts.

### Challenge 5: Weak Assertions for Extreme Coordinate Fallback Checks
* **Root Cause**: `test_kp_extreme_coordinates_cusps` asserts `assert!(kp.is_ok() || kp.is_err())` (line 539) which is a tautology and always passes, failing to verify whether the system successfully fell back to a valid house system at polar latitudes.
* **Proposed Mitigation**: Assert `assert!(kp.is_ok())` since the system is designed to successfully fall back to Koch, Porphyry, or Equal houses if Placidus fails. Similarly, add assertions in `test_stress_kp_extreme_coordinates_crash` for North and South Pole calculations.

---

### Proposed Code Changes for `matching.rs`
The following changes show the rewritten `MatchingEngine::calculate_compatibility` and `check_mangal_dosha` functions:

```rust
// Proposed rewrite of matching.rs (calculate_compatibility & check_mangal_dosha)

impl MatchingEngine {
    pub fn calculate_compatibility(
        male: &VedicChart,
        female: &VedicChart,
    ) -> Result<CompatibilityReport, String> {
        let male_moon = male.planets.iter().find(|p| p.planet == VedicPlanet::Moon)
            .ok_or_else(|| "Male Moon planet is missing from chart".to_string())?;
        let female_moon = female.planets.iter().find(|p| p.planet == VedicPlanet::Moon)
            .ok_or_else(|| "Female Moon planet is missing from chart".to_string())?;

        // Validate Nakshatra indices
        if male_moon.nakshatra == 0 || male_moon.nakshatra > 27 {
            return Err(format!("Invalid male Moon Nakshatra index: {} (must be 1-27)", male_moon.nakshatra));
        }
        if female_moon.nakshatra == 0 || female_moon.nakshatra > 27 {
            return Err(format!("Invalid female Moon Nakshatra index: {} (must be 1-27)", female_moon.nakshatra));
        }

        // Validate Rasi indices
        if male_moon.rasi == 0 || male_moon.rasi > 12 {
            return Err(format!("Invalid male Moon Rasi index: {} (must be 1-12)", male_moon.rasi));
        }
        if female_moon.rasi == 0 || female_moon.rasi > 12 {
            return Err(format!("Invalid female Moon Rasi index: {} (must be 1-12)", female_moon.rasi));
        }

        // 1. Varna (1 Guna)
        let varna_score = calculate_varna(male_moon.rasi, female_moon.rasi);
        
        // 2. Vashya (2 Gunas)
        let vashya_score = calculate_vashya(male_moon.rasi, female_moon.rasi);

        // 3. Tara (3 Gunas)
        let tara_score = calculate_tara(male_moon.nakshatra, female_moon.nakshatra);

        // 4. Yoni (4 Gunas)
        let yoni_score = calculate_yoni(male_moon.nakshatra, female_moon.nakshatra);

        // 5. Graha Maitri (5 Gunas)
        let graha_maitri_score = calculate_graha_maitri(male_moon.rasi, female_moon.rasi);

        // 6. Gana (6 Gunas)
        let gana_score = calculate_gana(male_moon.nakshatra, female_moon.nakshatra);

        // 7. Bhakoot (7 Gunas)
        let bhakoot_score = calculate_bhakoot(male_moon.rasi, female_moon.rasi);

        // 8. Nadi (8 Gunas)
        let nadi_score = calculate_nadi(male_moon.nakshatra, female_moon.nakshatra);

        let total = varna_score + vashya_score + tara_score + yoni_score + graha_maitri_score + gana_score + bhakoot_score + nadi_score;

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

        // Mangal Dosha
        let male_mangal = check_mangal_dosha(male);
        let female_mangal = check_mangal_dosha(female);
        let mangal_dosha_cancelled = male_mangal && female_mangal; // Dosha Samya: Both having it cancels the negative effect

        let has_critical_dosha = (nadi_score == 0.0) || (bhakoot_score == 0.0);
        let is_compatible = total >= 18.0 && (!has_critical_dosha || mangal_dosha_cancelled);

        let explanation = format!(
            "총 {}점 획득 (36점 만점). {}",
            total,
            if is_compatible {
                "전반적으로 조화로운 매칭입니다. 추천합니다."
            } else if total >= 18.0 {
                "점수는 높으나 주요 살(Nadi/Bhakoot Dosha)의 영향으로 신중한 주의가 필요합니다."
            } else {
                "성향적 차이가 커 상호 조율과 깊은 이해가 요구되는 상성입니다."
            }
        );

        Ok(CompatibilityReport {
            total_score: total,
            is_compatible,
            kootas,
            male_mangal_dosha: male_mangal,
            female_mangal_dosha: female_mangal,
            mangal_dosha_cancelled,
            explanation,
        })
    }
}

fn check_mangal_dosha(chart: &VedicChart) -> bool {
    let mars = chart.planets.iter().find(|p| p.planet == VedicPlanet::Mars);
    if let Some(m) = mars {
        // From Lagna (house_index)
        let is_mangal_lagna = [1, 2, 4, 7, 8, 12].contains(&m.house_index);
        
        // From Moon
        if let Some(moon) = chart.planets.iter().find(|p| p.planet == VedicPlanet::Moon) {
            let mut diff = m.rasi as i16 - moon.rasi as i16;
            if diff < 0 { diff += 12; }
            let house_from_moon = (diff + 1) as u8;
            let is_mangal_moon = [1, 2, 4, 7, 8, 12].contains(&house_from_moon);
            is_mangal_lagna || is_mangal_moon
        } else {
            is_mangal_lagna // Safe default if Moon is missing
        }
    } else {
        false
    }
}
```

---

## 2. Review of Engine Calculations (`strength.rs`)

The Shadbala of each planet is calculated by summing its sub-strengths in `StrengthEngine::calculate`:

1. **Sthana Bala (Positional Strength)**:
   $$\text{Sthana Bala} = \text{Uchcha} + \text{Saptavargaja} + \text{Kendra} + \text{Drekkana} + \text{Ojayugma}$$
   * **Uchcha Bala**: Max 60 units. Measures distance from the deep debilitation point.
   * **Saptavargaja Bala**: Max 60 units. Evaluates planetary friendship/ownership across 7 divisional charts (D1, D2, D3, D7, D9, D12, D30).
   * **Kendra Bala**: 60 points for planets in Kendra (1,4,7,10), 30 for Panaphara (2,5,8,11), 15 for Apoklima (3,6,9,12).
   * **Drekkana Bala**: 15 points if masculine/feminine planet nature matches masculine/feminine Drekkana sign (D3); neuter planets always receive 7.5.
   * **Ojayugmarasyamsa Bala**: Max 15 points. Inherent masculine/feminine sign matching across D1 and D9.

2. **Dig Bala (Directional Strength)**:
   $$\text{Dig Bala} = \text{Shortest House Distance} \times 10$$
   * Mercury/Jupiter peak in 1st house. Sun/Mars in 10th. Saturn in 7th. Moon/Venus in 4th. Max 60 units.

3. **Kala Bala (Temporal Strength)**:
   $$\text{Kala Bala} = \text{Nathonnata} + \text{Vara} + \text{Hora} + \text{Tribhaga}$$
   * **Nathonnata Bala**: Diva-Ratri strength. 60 points depending on day/night birth.
   * **Vara Bala**: 45 points for the lord of the birth day.
   * **Hora Bala**: 60 points for the lord of the birth hour.
   * **Tribhaga Bala**: 60 points for the lord of the current 1/3 division of the day/night.

4. **Cheshta Bala (Motion Strength)**:
   * Max 60 units. Based on planetary velocity state (Vakra, Sama, Chara, etc.).
   * Sun uses declination/direction (Uttarayana bonus), Moon uses Paksha (waxing/waning).

5. **Naisargika Bala (Natural Strength)**:
   * Fixed natural strength constants: Sun (60.0), Moon (51.43), Venus (42.86), Jupiter (34.29), Mercury (25.71), Mars (17.14), Saturn (8.57).

6. **Drik Bala (Aspect Strength)**:
   * Sum of aspect values (Drishti) from all other planets. Benefics add positive points, malefics subtract points. Clamped to $[-60.0, 60.0]$.

---

## 3. UI Design for Strength Tab (`strength_tab.rs`)

We design a responsive grid UI where each planet is displayed as a scorecard. The total score is compared to the planet's Rupa minimum benchmark to assign a grade (A+, A, B, C, D) and a progress indicator. Six horizontal progress bars show the detailed contributions of the Shadbala factors.

### Proposed Dioxus Component Code for `strength_tab.rs` (Vedic Section)

```rust
// Proposed scorecard render block inside StrengthTab in crates/eon-ui/src/components/tabs/strength_tab.rs

// Standard Rupa Minimum Benchmarks (in Virupas)
fn get_rupa_benchmark(p: VedicPlanet) -> f64 {
    match p {
        VedicPlanet::Sun => 390.0,
        VedicPlanet::Moon => 360.0,
        VedicPlanet::Mars => 300.0,
        VedicPlanet::Mercury => 420.0,
        VedicPlanet::Jupiter => 390.0,
        VedicPlanet::Venus => 330.0,
        VedicPlanet::Saturn => 300.0,
        _ => 240.0,
    }
}

// Factor Standard Minimum Benchmarks
fn get_factor_benchmark(factor: &str, p: VedicPlanet) -> f64 {
    match factor {
        "sthana" => match p {
            VedicPlanet::Sun | VedicPlanet::Mercury | VedicPlanet::Jupiter => 165.0,
            VedicPlanet::Moon | VedicPlanet::Venus => 133.0,
            VedicPlanet::Mars | VedicPlanet::Saturn => 96.0,
            _ => 100.0,
        },
        "dig" => 35.0,
        "kala" => 112.0,
        "chesta" => 50.0,
        "naisargika" => match p {
            VedicPlanet::Sun => 60.0,
            VedicPlanet::Moon => 51.43,
            VedicPlanet::Venus => 42.86,
            VedicPlanet::Jupiter => 34.29,
            VedicPlanet::Mercury => 25.71,
            VedicPlanet::Mars => 17.14,
            VedicPlanet::Saturn => 8.57,
            _ => 0.0,
        },
        _ => 30.0, // drik aspect reference
    }
}

// Render code within has_vedic conditional block:
if has_vedic {
    if let Some(vedic) = &state.vedic.read().data {
        let strengths: Vec<_> = vedic.chart.planets.iter()
            .filter(|p| PLANETS.contains(&p.planet))
            .map(|p| (p.planet, StrengthEngine::calculate(p, &vedic.chart)))
            .collect();

        rsx! {
            div { class: "mt-8 space-y-6",
                h3 { class: "text-xl font-bold text-slate-200 border-b border-slate-800 pb-2 flex items-center gap-2",
                    span { "☸️" }
                    "{t(locale, TK::SectionStrength)} (Shadbala)"
                }
                
                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                    {strengths.iter().map(|(planet, s)| {
                        let rupa_min = get_rupa_benchmark(*planet);
                        let ratio = s.total_score / rupa_min;
                        let pct = (ratio * 100.0) as u32;
                        
                        let (grade, grade_color) = if ratio >= 1.20 {
                            ("A+", "bg-amber-500/20 text-amber-300 border-amber-500/50 shadow-[0_0_8px_rgba(245,158,11,0.2)]")
                        } else if ratio >= 1.00 {
                            ("A", "bg-emerald-500/20 text-emerald-300 border-emerald-500/50")
                        } else if ratio >= 0.80 {
                            ("B", "bg-blue-500/20 text-blue-300 border-blue-500/50")
                        } else if ratio >= 0.60 {
                            ("C", "bg-yellow-500/20 text-yellow-300 border-yellow-500/50")
                        } else {
                            ("D", "bg-red-500/20 text-red-300 border-red-500/50")
                        };

                        let status_color = match s.status.as_str() {
                            "Exalted" => "text-yellow-400 font-bold",
                            "Strong" => "text-emerald-400 font-semibold",
                            "Debilitated" | "Weak" => "text-red-400 font-semibold",
                            _ => "text-slate-400",
                        };

                        let bar_color = planet_bar_color(*planet);

                        rsx! {
                            div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5 hover:border-slate-700 transition-all duration-300 flex flex-col gap-4",
                                // Card Header
                                div { class: "flex justify-between items-start",
                                    div { class: "flex items-center gap-3",
                                        div { class: "w-10 h-10 rounded-xl flex items-center justify-center {bar_color}/20 text-xl font-bold shrink-0",
                                            "{translate_planet(locale, *planet).chars().last().unwrap_or('★')}"
                                        }
                                        div {
                                            h4 { class: "font-bold text-slate-100", "{translate_planet(locale, *planet)}" }
                                            span { class: "text-xs {status_color}", "{s.status.as_str()}" }
                                        }
                                    }
                                    span { class: "px-3 py-1 rounded-full border text-xs font-black {grade_color}",
                                        "{grade}"
                                    }
                                }

                                // Total Score & Rupa ratio
                                div { class: "bg-slate-950 rounded-xl p-3 border border-slate-800/60 flex justify-between items-center",
                                    div {
                                        span { class: "text-xs text-slate-500 block uppercase font-semibold tracking-wider", "{t(locale, TK::LabelTotal)}" }
                                        span { class: "font-mono text-lg font-bold text-slate-200", "{s.total_score:.1}" }
                                        span { class: "text-xs text-slate-500 ml-1", "/ {rupa_min:.0} Virupas" }
                                    }
                                    span { class: "text-xl font-black font-mono text-emerald-400", "{pct}%" }
                                }

                                // 6 Factors Progress list
                                div { class: "space-y-3.5 flex-1",
                                    // Sthana Bala
                                    FactorProgressBar {
                                        label: t(locale, TK::ShadbalaSthana),
                                        val: s.sthana_bala,
                                        benchmark: get_factor_benchmark("sthana", *planet),
                                        max_val: 250.0,
                                        color: "bg-emerald-500"
                                    }
                                    // Dig Bala
                                    FactorProgressBar {
                                        label: t(locale, TK::ShadbalaDig),
                                        val: s.dig_bala,
                                        benchmark: get_factor_benchmark("dig", *planet),
                                        max_val: 60.0,
                                        color: "bg-cyan-500"
                                    }
                                    // Kala Bala
                                    FactorProgressBar {
                                        label: t(locale, TK::ShadbalaKala),
                                        val: s.kala_bala,
                                        benchmark: get_factor_benchmark("kala", *planet),
                                        max_val: 200.0,
                                        color: "bg-yellow-500"
                                    }
                                    // Cheshta Bala
                                    FactorProgressBar {
                                        label: t(locale, TK::ShadbalaChesta),
                                        val: s.chesta_bala,
                                        benchmark: get_factor_benchmark("chesta", *planet),
                                        max_val: 60.0,
                                        color: "bg-indigo-500"
                                    }
                                    // Naisargika Bala
                                    FactorProgressBar {
                                        label: t(locale, TK::ShadbalaNaisargika),
                                        val: s.naisargika_bala,
                                        benchmark: get_factor_benchmark("naisargika", *planet),
                                        max_val: 60.0,
                                        color: "bg-purple-500"
                                    }
                                    // Drik Bala (ranges from -60 to 60)
                                    DrikBalaBar {
                                        label: t(locale, TK::ShadbalaDrik),
                                        val: s.drik_bala
                                    }
                                }
                            }
                        }
                    })}
                }
            }
        }
    }
}

// Helper components for progress bar drawing:
#[component]
fn FactorProgressBar(label: &'static str, val: f64, benchmark: f64, max_val: f64, color: &'static str) -> Element {
    let pct_width = ((val / max_val * 100.0).min(100.0).max(0.0)) as u32;
    let benchmark_tick = (benchmark / max_val * 100.0) as u32;
    let is_met = val >= benchmark;
    let status_color = if is_met { "text-emerald-400" } else { "text-slate-400" };

    rsx! {
        div { class: "space-y-1",
            div { class: "flex justify-between text-xs font-medium",
                span { class: "text-slate-400", "{label}" }
                span { class: "font-mono {status_color}", "{val:.1} / {benchmark:.0}" }
            }
            div { class: "relative h-2 bg-slate-950 rounded-full overflow-hidden border border-slate-800",
                // Tick indicator for minimum benchmark
                div { 
                    class: "absolute top-0 bottom-0 w-0.5 bg-slate-700 z-10", 
                    style: "left: {benchmark_tick}%" 
                }
                // Actual value progress filled bar
                div { 
                    class: "h-full rounded-full transition-all duration-1000 {color} {if is_met { 'opacity-100' } else { 'opacity-60' }}", 
                    style: "width: {pct_width}%" 
                }
            }
        }
    }
}

#[component]
fn DrikBalaBar(label: &'static str, val: f64) -> Element {
    // Drik Bala is centered around 0 (neutral aspect influence) from -60 to +60.
    let display_val = val.clamp(-60.0, 60.0);
    
    // Scale width: 50% is the center (0.0 points).
    // If negative, fill left from 50%. If positive, fill right from 50%.
    let (left, width, color) = if display_val >= 0.0 {
        let w = (display_val / 120.0 * 100.0) as u32;
        (50, w, "bg-emerald-500")
    } else {
        let w = (display_val.abs() / 120.0 * 100.0) as u32;
        (50 - w, w, "bg-rose-500")
    };

    rsx! {
        div { class: "space-y-1",
            div { class: "flex justify-between text-xs font-medium",
                span { class: "text-slate-400", "{label}" }
                span { class: "font-mono {if display_val >= 0.0 { 'text-emerald-400' } else { 'text-rose-400' }}", 
                    "{if display_val > 0.0 { '+' } else { '' }}{display_val:.1}" 
                }
            }
            div { class: "relative h-2 bg-slate-950 rounded-full overflow-hidden border border-slate-800",
                // Midline marker at 0.0
                div { class: "absolute top-0 bottom-0 left-1/2 w-0.5 bg-slate-700 z-10" }
                // Aspect strength fill
                div { 
                    class: "absolute top-0 bottom-0 rounded-full transition-all duration-1000 {color}", 
                    style: "left: {left}%; width: {width}%" 
                }
            }
        }
    }
}
```

---

## 4. Translations Definition (`crates/eon-ui/src/i18n/`)

We introduce new translation keys to support localized Shadbala labels.

### Translation Key Additions to `mod.rs`
```rust
// In crates/eon-ui/src/i18n/mod.rs (inside enum TK)
    ShadbalaSthana,
    ShadbalaDig,
    ShadbalaKala,
    ShadbalaChesta,
    ShadbalaNaisargika,
    ShadbalaDrik,
    ShadbalaBenchmark,
    ShadbalaGrade,
    ShadbalaStatus,
    ShadbalaRequiredMin,
```

### Localized Strings for Each Locale File

#### Korean (`ko.rs`)
```rust
        TK::ShadbalaSthana => "위치 강도 (Sthana)",
        TK::ShadbalaDig => "방향 강도 (Dig)",
        TK::ShadbalaKala => "시간 강도 (Kala)",
        TK::ShadbalaChesta => "운동 강도 (Cheshta)",
        TK::ShadbalaNaisargika => "자연 강도 (Naisargika)",
        TK::ShadbalaDrik => "성상 강도 (Drik)",
        TK::ShadbalaBenchmark => "최소 기준 (Rupa)",
        TK::ShadbalaGrade => "등급",
        TK::ShadbalaStatus => "상태",
        TK::ShadbalaRequiredMin => "필수 최소치",
```

#### English (`en.rs`)
```rust
        TK::ShadbalaSthana => "Sthana Bala (Positional)",
        TK::ShadbalaDig => "Dig Bala (Directional)",
        TK::ShadbalaKala => "Kala Bala (Temporal)",
        TK::ShadbalaChesta => "Cheshta Bala (Motion)",
        TK::ShadbalaNaisargika => "Naisargika Bala (Natural)",
        TK::ShadbalaDrik => "Drik Bala (Aspect)",
        TK::ShadbalaBenchmark => "Min Benchmark (Rupa)",
        TK::ShadbalaGrade => "Grade",
        TK::ShadbalaStatus => "Status",
        TK::ShadbalaRequiredMin => "Required Min",
```

#### Chinese (`zh.rs`)
```rust
        TK::ShadbalaSthana => "位置力量 (Sthana Bala)",
        TK::ShadbalaDig => "方向力量 (Dig Bala)",
        TK::ShadbalaKala => "时间力量 (Kala Bala)",
        TK::ShadbalaChesta => "运动力量 (Cheshta Bala)",
        TK::ShadbalaNaisargika => "自然力量 (Naisargika Bala)",
        TK::ShadbalaDrik => "相位力量 (Drik Bala)",
        TK::ShadbalaBenchmark => "最低基准 (Rupa)",
        TK::ShadbalaGrade => "评级",
        TK::ShadbalaStatus => "状态",
        TK::ShadbalaRequiredMin => "最低要求",
```

#### Russian (`ru.rs`)
```rust
        TK::ShadbalaSthana => "Стхана Бала (Позиционная)",
        TK::ShadbalaDig => "Диг Бала (Направленная)",
        TK::ShadbalaKala => "Кала Бала (Временная)",
        TK::ShadbalaChesta => "Чешта Бала (Движения)",
        TK::ShadbalaNaisargika => "Найсаргика Бала (Естественная)",
        TK::ShadbalaDrik => "Дрик Бала (Аспектная)",
        TK::ShadbalaBenchmark => "Критерий (Рупа)",
        TK::ShadbalaGrade => "Класс",
        TK::ShadbalaStatus => "Статус",
        TK::ShadbalaRequiredMin => "Мин. Требуется",
```
