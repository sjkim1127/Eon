# Milestone M3: Strength Visualization and Robustness Analysis

This document provides the requirements analysis, engine vulnerability audit, design specification, and translation definitions for Milestone M3 (R2: Shadbala & Bhava Bala 6대 강도 세부 수치 시각화).

---

## 1. Engine Vulnerability Mitigations (`crates/eon-vedic/src/analysis/matching.rs`)

The M2 Challengers identified 5 critical robustness/edge-case issues in the Vedic compatibility and calculation engine. Here is the analysis and the precise proposed code fixes.

### Vulnerability 1: Unsafe Unwrap on Missing Moon
* **Problem**: In `MatchingEngine::calculate_compatibility`, the code directly retrieves and unwraps the Moon from both the male and female charts:
  ```rust
  let male_moon = male.planets.iter().find(|p| p.planet == VedicPlanet::Moon).cloned().unwrap();
  let female_moon = female.planets.iter().find(|p| p.planet == VedicPlanet::Moon).cloned().unwrap();
  ```
  If either chart does not contain a Moon position, the application panics.
* **Proposed Mitigation**: Safely search for the Moon and return a clean error report with zeroed scores if either Moon is missing:
  ```rust
  let male_moon_opt = male.planets.iter().find(|p| p.planet == VedicPlanet::Moon);
  let female_moon_opt = female.planets.iter().find(|p| p.planet == VedicPlanet::Moon);

  if male_moon_opt.is_none() || female_moon_opt.is_none() {
      return CompatibilityReport {
          total_score: 0.0,
          is_compatible: false,
          kootas: vec![],
          male_mangal_dosha: false,
          female_mangal_dosha: false,
          mangal_dosha_cancelled: false,
          explanation: "Error: Moon position is missing from one or both charts. Compatibility cannot be calculated.".to_string(),
      };
  }
  let male_moon = male_moon_opt.unwrap().clone();
  let female_moon = female_moon_opt.unwrap().clone();
  ```

### Vulnerability 2: Unsafe Unwrap in Mangal Dosha on Missing Moon
* **Problem**: In `check_mangal_dosha` (lines 383–384), if Mars is present, the function proceeds to unwrap the Moon without validation:
  ```rust
  let moon = chart.planets.iter().find(|p| p.planet == VedicPlanet::Moon).unwrap();
  ```
  If Moon is missing from the chart (even if Mars is present), the application panics.
* **Proposed Mitigation**: Safely lookup both Mars and Moon. If Moon is missing but Mars is present, calculate Mangal Dosha using the Ascendant (Lagna) only, which is a standard astrological fallback, instead of crashing:
  ```rust
  fn check_mangal_dosha(chart: &VedicChart) -> bool {
      let mars = chart.planets.iter().find(|p| p.planet == VedicPlanet::Mars);
      let moon = chart.planets.iter().find(|p| p.planet == VedicPlanet::Moon);
      
      match (mars, moon) {
          (Some(m), Some(mon)) => {
              // From Lagna (house_index)
              let is_mangal_lagna = [1, 2, 4, 7, 8, 12].contains(&m.house_index);
              
              // From Moon
              let mut diff = m.rasi as i16 - mon.rasi as i16;
              if diff < 0 { diff += 12; }
              let house_from_moon = (diff + 1) as u8;
              let is_mangal_moon = [1, 2, 4, 7, 8, 12].contains(&house_from_moon);

              is_mangal_lagna || is_mangal_moon
          }
          (Some(m), None) => {
              // Fallback: Check only Lagna-based Mangal Dosha if Moon is missing
              [1, 2, 4, 7, 8, 12].contains(&m.house_index)
          }
          _ => false, // Mars is missing
      }
  }
  ```

### Vulnerability 3: Underflow Risk on Nakshatra=0
* **Problem**: If `female_nak` or `male_nak` is calculated as `0` (invalid range, Nakshatras must be 1..=27), it can trigger underflow/overflow panics or out-of-bounds indexing in subsequent operations.
* **Proposed Mitigation**: Validate Nakshatra values at the entry of helper functions and in `calculate_compatibility`. If they are out of bounds, return a clean error report:
  ```rust
  if !(1..=27).contains(&male_moon.nakshatra) || !(1..=27).contains(&female_moon.nakshatra) {
      return CompatibilityReport {
          total_score: 0.0,
          is_compatible: false,
          kootas: vec![],
          male_mangal_dosha: false,
          female_mangal_dosha: false,
          mangal_dosha_cancelled: false,
          explanation: "Error: Invalid Nakshatra value (must be 1..=27).".to_string(),
      };
  }
  ```

### Vulnerability 4: Tara Koota Modulo Wrap Anomaly
* **Problem**: In `calculate_tara`, the distance is calculated as:
  ```rust
  let dist_f_to_m = ((male_nak as i16 - female_nak as i16 + 27) % 9) + 1;
  ```
  In Rust, `%` is the remainder operator, which yields negative values for negative operands. If `male_nak - female_nak + 27` is negative, the remainder is negative. E.g. `-1 % 9` is `-1`, leading to invalid distances (0 instead of 9).
* **Proposed Mitigation**: Use `rem_euclid` (Euclidean modulo) which guarantees a positive remainder:
  ```rust
  fn calculate_tara(male_nak: u8, female_nak: u8) -> f64 {
      if !(1..=27).contains(&male_nak) || !(1..=27).contains(&female_nak) {
          return 0.0;
      }
      let dist_f_to_m = (male_nak as i16 - female_nak as i16).rem_euclid(9) + 1;
      let dist_m_to_f = (female_nak as i16 - male_nak as i16).rem_euclid(9) + 1;

      let inauspicious = [3, 5, 7]; // Vipat, Pratyak, Naidhana
      let m_ok = !inauspicious.contains(&dist_f_to_m);
      let f_ok = !inauspicious.contains(&dist_m_to_f);

      if m_ok && f_ok {
          3.0
      } else if m_ok || f_ok {
          1.5
      } else {
          0.0
      }
  }
  ```

### Vulnerability 5: Polar Coordinate Fallback Weak Assertion
* **Problem**: In `crates/eon-vedic/tests/compatibility_shadbala_kp_dasha.rs`, the test `test_kp_extreme_coordinates_cusps` asserts:
  ```rust
  assert!(kp.is_ok() || kp.is_err());
  ```
  This assertion is a tautology that is always true, failing to verify that the polar fallback mechanism actually worked.
* **Proposed Mitigation**: Assert that the calculation succeeds (is `Ok`) because the KP house calculations fall back to the Equal house system under polar latitudes (which always succeeds), and verify that exactly 12 house cusps are produced:
  ```rust
  #[test]
  fn test_kp_extreme_coordinates_cusps() {
      let calc = VedicChartCalculator::new();
      let time = Utc.with_ymd_and_hms(2026, 6, 20, 12, 0, 0).unwrap();
      // High latitude (Tromsø, Norway at 69.6° N) where Placidus fails
      let kp = KpAnalysis::calculate(time, 69.6, 18.9, 24.0, &[], calc.engine());
      assert!(kp.is_ok(), "Polar fallback failed to yield a valid KP house system");
      let kp = kp.unwrap();
      assert_eq!(kp.cusps.len(), 12, "Polar fallback did not yield exactly 12 house cusps");
  }
  ```

---

## 2. Review of Shadbala calculations in `crates/eon-vedic/src/analysis/strength.rs`

Shadbala (sixfold planetary strength) calculates six distinct factors (measured in Virupas, where 60 Virupas = 1 Rupa).

### The 6 Standard Factors and Calculation Flow
1. **Sthana Bala (Positional Strength)**:
   * **Formula**: `ex_score + sapta_score + kendra_bala + drekkana_bala + ojayugmarasyamsa_bala`
   * *Uchcha Bala (Exaltation)*: Distance from deep debilitation point divided by 3 (max 60).
   * *Saptavargaja Bala (7 Vargas)*: Evaluation of friendship relationships (Panchadha Maitri) across 7 divisional charts (D1, D2, D3, D7, D9, D12, D30).
   * *Kendra Bala*: 60 points for angles (1,4,7,10), 30 for succedent (2,5,8,11), 15 for cadent (3,6,9,12).
   * *Drekkana Bala*: 15 points if masculine/feminine nature matches masculine/even D3 sign. Neuter planets get 7.5.
   * *Ojayugmarasyamsa Bala*: 7.5 points for matching odd/even signs in D1 and D9.
2. **Dig Bala (Directional Strength)**:
   * **Formula**: Calculated based on the planet's house placement.
   * Sun/Mars gain max strength in 10th house; Saturn in 7th; Moon/Venus in 4th; Mercury/Jupiter in 1st. Points decrease by 10 per house away from their strongest house.
3. **Kala Bala (Temporal Strength)**:
   * **Formula**: `nathonnata_bala + vara_bala + hora_bala + tribhaga_bala`
   * *Nathonnata*: Sun/Jup/Ven strong in day; Moon/Mars/Sat strong in night; Mer always 60.
   * *Vara Bala*: 45 points to the lord of the day.
   * *Hora Bala*: 60 points to the lord of the hour.
   * *Tribhaga Bala*: 60 points if the planet rules the specific 1/3 division of day/night.
4. **Cheshta Bala (Motional Strength)**:
   * **Formula**: Based on speed and retrograde state. Retrograde planets get 60. Others mapped to 7 speed-based motion states. Sun uses Uttarayana declination ratio; Moon uses Paksha Bala.
5. **Naisargika Bala (Natural Strength)**:
   * Fixed natural strength values: Sun (60.0), Moon (51.43), Venus (42.86), Jupiter (34.29), Mercury (25.71), Mars (17.14), Saturn (8.57).
6. **Drik Bala (Aspect Strength)**:
   * Aspect sum of aspects (Drishti) from other planets. Malefics subtract strength; benefics add. Clamped between -60.0 and 60.0.

---

## 3. Responsive Scorecard Grid UI Design (`crates/eon-ui/src/components/tabs/strength_tab.rs`)

### Design Concept
* **Responsive Grid**: Layout adapts dynamically using Tailwind CSS: `grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6`.
* **7 Scorecards**: One card for each of the 7 traditional planets (Sun, Moon, Mars, Mercury, Jupiter, Venus, Saturn). Rahu/Ketu are omitted as they do not possess standard Shadbala factors.
* **Overall Progress & Status**:
  * Displays total score vs. the planet's traditional minimum required benchmark.
  * Grades are calculated on the ratio of actual strength to benchmark:
    * **Grade A (Excellent)**: $\ge 120\%$ of benchmark.
    * **Grade B (Good)**: $100\% \text{ to } 120\%$.
    * **Grade C (Neutral)**: $80\% \text{ to } 100\%$.
    * **Grade D (Weak)**: $< 80\%$.
* **6 Factors Progress Bars**:
  * Displays each factor with a progress bar mapped against its traditional benchmark.
  * A **marker indicator** (vertical tick line) is drawn at the benchmark value.
  * Bar fill turns **green** if it meets/exceeds the benchmark, or **red/orange** if below, providing immediate visual feedback.

### Traditional Benchmarks (in Virupas)
| Planet | Sthana Bala | Dig Bala | Kala Bala | Cheshta Bala | Naisargika Bala | Drik Bala | Total Benchmark |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **Sun** | 165.0 | 35.0 | 80.0 | 50.0 | 60.0 | 0.0 | **390.0** |
| **Moon** | 133.0 | 50.0 | 100.0 | 50.0 | 51.43 | 0.0 | **360.0** |
| **Mars** | 96.0 | 50.0 | 70.0 | 50.0 | 17.14 | 0.0 | **300.0** |
| **Mercury** | 165.0 | 35.0 | 100.0 | 50.0 | 25.71 | 0.0 | **420.0** |
| **Jupiter** | 165.0 | 35.0 | 100.0 | 50.0 | 34.29 | 0.0 | **390.0** |
| **Venus** | 133.0 | 50.0 | 100.0 | 50.0 | 42.86 | 0.0 | **330.0** |
| **Saturn** | 96.0 | 30.0 | 80.0 | 50.0 | 8.57 | 0.0 | **300.0** |

### Proposed Dioxus Component Implementation (Code Proposal)
```rust
use dioxus::prelude::*;
use crate::store::{AnalysisState, TaskStatus};
use crate::i18n::{t, TK, Locale, translate_planet};
use eon_vedic::planets::VedicPlanet;
use eon_vedic::analysis::strength::{PlanetStrength, StrengthEngine};

#[derive(Debug, Clone)]
struct PlanetBenchmark {
    sthana: f64,
    dig: f64,
    kala: f64,
    chesta: f64,
    naisargika: f64,
    drik: f64,
    total: f64,
}

fn get_benchmark(planet: VedicPlanet) -> PlanetBenchmark {
    match planet {
        VedicPlanet::Sun => PlanetBenchmark { sthana: 165.0, dig: 35.0, kala: 80.0, chesta: 50.0, naisargika: 60.0, drik: 0.0, total: 390.0 },
        VedicPlanet::Moon => PlanetBenchmark { sthana: 133.0, dig: 50.0, kala: 100.0, chesta: 50.0, naisargika: 51.43, drik: 0.0, total: 360.0 },
        VedicPlanet::Mars => PlanetBenchmark { sthana: 96.0, dig: 50.0, kala: 70.0, chesta: 50.0, naisargika: 17.14, drik: 0.0, total: 300.0 },
        VedicPlanet::Mercury => PlanetBenchmark { sthana: 165.0, dig: 35.0, kala: 100.0, chesta: 50.0, naisargika: 25.71, drik: 0.0, total: 420.0 },
        VedicPlanet::Jupiter => PlanetBenchmark { sthana: 165.0, dig: 35.0, kala: 100.0, chesta: 50.0, naisargika: 34.29, drik: 0.0, total: 390.0 },
        VedicPlanet::Venus => PlanetBenchmark { sthana: 133.0, dig: 50.0, kala: 100.0, chesta: 50.0, naisargika: 42.86, drik: 0.0, total: 330.0 },
        VedicPlanet::Saturn => PlanetBenchmark { sthana: 96.0, dig: 30.0, kala: 80.0, chesta: 50.0, naisargika: 8.57, drik: 0.0, total: 300.0 },
        _ => PlanetBenchmark { sthana: 0.0, dig: 0.0, kala: 0.0, chesta: 0.0, naisargika: 0.0, drik: 0.0, total: 0.0 },
    }
}

fn get_grade(score: f64, benchmark: f64) -> (&'static str, &'static str) {
    let ratio = score / benchmark;
    if ratio >= 1.2 {
        ("Grade A", "text-emerald-400 bg-emerald-950/40 border-emerald-500/30")
    } else if ratio >= 1.0 {
        ("Grade B", "text-blue-400 bg-blue-950/40 border-blue-500/30")
    } else if ratio >= 0.8 {
        ("Grade C", "text-amber-400 bg-amber-950/40 border-amber-500/30")
    } else {
        ("Grade D", "text-red-400 bg-red-950/40 border-red-500/30")
    }
}

// Sub-component for individual factor progress bars
#[component]
fn FactorProgressBar(
    locale: Locale,
    label_key: TK,
    value: f64,
    benchmark: f64,
    max_val: f64,
) -> Element {
    let earned_pct = ((value / max_val) * 100.0).clamp(0.0, 100.0) as u32;
    let benchmark_pct = ((benchmark / max_val) * 100.0).clamp(0.0, 100.0) as u32;
    let is_satisfied = value >= benchmark;
    let bar_color = if is_satisfied { "bg-emerald-500" } else { "bg-rose-500" };

    rsx! {
        div { class: "space-y-1",
            div { class: "flex justify-between text-xs font-medium text-slate-400",
                span { "{t(locale, label_key)}" }
                span { class: "font-mono", "{value:.1} / {benchmark:.1}" }
            }
            div { class: "relative h-2.5 bg-slate-800 rounded-full overflow-visible",
                // Actual Score Bar
                div {
                    class: "h-full {bar_color} rounded-full transition-all duration-500",
                    style: "width: {earned_pct}%"
                }
                // Benchmark Tick Marker
                div {
                    class: "absolute top-[-2px] h-[14px] w-[2px] bg-slate-300 shadow-md z-10",
                    style: "left: {benchmark_pct}%"
                }
            }
        }
    }
}
```

---

## 4. Internationalization (i18n) Translation Additions

The translation dictionary in `crates/eon-ui/src/i18n/` must be expanded to cover Shadbala-specific proper nouns, overall status labels, and grading.

### Key Definitions (`crates/eon-ui/src/i18n/mod.rs`)
Add these enum variants to `pub enum TK`:
```rust
    // ── Shadbala Keys ──────────────────────────────────────────────────
    ShadbalaTitle,
    ShadbalaTotalScore,
    ShadbalaBenchmark,
    ShadbalaStatus,
    ShadbalaGrade,
    ShadbalaFactorSthana,
    ShadbalaFactorDig,
    ShadbalaFactorKala,
    ShadbalaFactorChesta,
    ShadbalaFactorNaisargika,
    ShadbalaFactorDrik,
    ShadbalaStatusExalted,
    ShadbalaStatusDebilitated,
    ShadbalaStatusStrong,
    ShadbalaStatusWeak,
    ShadbalaStatusNeutral,
```

### Korean Translations (`crates/eon-ui/src/i18n/ko.rs`)
```rust
        TK::ShadbalaTitle => "샤드발라 행성 세력 (Shadbala)",
        TK::ShadbalaTotalScore => "총점",
        TK::ShadbalaBenchmark => "최소 기준값 (Benchmark)",
        TK::ShadbalaStatus => "상태",
        TK::ShadbalaGrade => "등급",
        TK::ShadbalaFactorSthana => "체성 강도 (Sthana Bala - 위치)",
        TK::ShadbalaFactorDig => "방향 강도 (Dig Bala - 하우스)",
        TK::ShadbalaFactorKala => "시간 강도 (Kala Bala - 주야/시각)",
        TK::ShadbalaFactorChesta => "운동 강도 (Cheshta Bala - 속도/역행)",
        TK::ShadbalaFactorNaisargika => "자연 강도 (Naisargika Bala - 고유세기)",
        TK::ShadbalaFactorDrik => "시각 강도 (Drik Bala - 타행성 각도)",
        TK::ShadbalaStatusExalted => "고양 (Exalted)",
        TK::ShadbalaStatusDebilitated => "쇠퇴 (Debilitated)",
        TK::ShadbalaStatusStrong => "강함",
        TK::ShadbalaStatusWeak => "약함",
        TK::ShadbalaStatusNeutral => "중립",
```

### English Translations (`crates/eon-ui/src/i18n/en.rs`)
```rust
        TK::ShadbalaTitle => "Shadbala Planetary Strength",
        TK::ShadbalaTotalScore => "Total Score",
        TK::ShadbalaBenchmark => "Min Benchmark",
        TK::ShadbalaStatus => "Status",
        TK::ShadbalaGrade => "Grade",
        TK::ShadbalaFactorSthana => "Sthana Bala (Positional)",
        TK::ShadbalaFactorDig => "Dig Bala (Directional)",
        TK::ShadbalaFactorKala => "Kala Bala (Temporal)",
        TK::ShadbalaFactorChesta => "Cheshta Bala (Motional)",
        TK::ShadbalaFactorNaisargika => "Naisargika Bala (Natural)",
        TK::ShadbalaFactorDrik => "Drik Bala (Aspectual)",
        TK::ShadbalaStatusExalted => "Exalted",
        TK::ShadbalaStatusDebilitated => "Debilitated",
        TK::ShadbalaStatusStrong => "Strong",
        TK::ShadbalaStatusWeak => "Weak",
        TK::ShadbalaStatusNeutral => "Neutral",
```

### Chinese Translations (`crates/eon-ui/src/i18n/zh.rs`)
```rust
        TK::ShadbalaTitle => "沙德巴拉行星力量 (Shadbala)",
        TK::ShadbalaTotalScore => "总分",
        TK::ShadbalaBenchmark => "最低基准",
        TK::ShadbalaStatus => "状态",
        TK::ShadbalaGrade => "等级",
        TK::ShadbalaFactorSthana => "位置力量 (Sthana Bala)",
        TK::ShadbalaFactorDig => "方向力量 (Dig Bala)",
        TK::ShadbalaFactorKala => "时间力量 (Kala Bala)",
        TK::ShadbalaFactorChesta => "运动力量 (Cheshta Bala)",
        TK::ShadbalaFactorNaisargika => "自然力量 (Naisargika Bala)",
        TK::ShadbalaFactorDrik => "相位力量 (Drik Bala)",
        TK::ShadbalaStatusExalted => "庙旺",
        TK::ShadbalaStatusDebilitated => "落陷",
        TK::ShadbalaStatusStrong => "强旺",
        TK::ShadbalaStatusWeak => "虚弱",
        TK::ShadbalaStatusNeutral => "中性",
```

### Russian Translations (`crates/eon-ui/src/i18n/ru.rs`)
```rust
        TK::ShadbalaTitle => "Сила Планет (Шадбала)",
        TK::ShadbalaTotalScore => "Общий балл",
        TK::ShadbalaBenchmark => "Минимум",
        TK::ShadbalaStatus => "Статус",
        TK::ShadbalaGrade => "Оценка",
        TK::ShadbalaFactorSthana => "Стхана Bala (Позиционная)",
        TK::ShadbalaFactorDig => "Диг Bala (Направленная)",
        TK::ShadbalaFactorKala => "Кала Bala (Временная)",
        TK::ShadbalaFactorChesta => "Чешта Bala (Двигательная)",
        TK::ShadbalaFactorNaisargika => "Найсаргика Bala (Естественная)",
        TK::ShadbalaFactorDrik => "Дрик Bala (Аспектная)",
        TK::ShadbalaStatusExalted => "Экзальтация",
        TK::ShadbalaStatusDebilitated => "Падение",
        TK::ShadbalaStatusStrong => "Сильный",
        TK::ShadbalaStatusWeak => "Слабый",
        TK::ShadbalaStatusNeutral => "Нейтральный",
```
