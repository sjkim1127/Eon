# Handoff Report — Vedic Astrology Enhancements Exploration (M2-M5)

## 1. Observation

We explored the codebase and analyzed the relevant files. The following locations and details were observed:

### A. Planet Strength Definitions
In `crates/eon-vedic/src/analysis/strength.rs` (lines 7-34):
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetStrength {
    pub planet: VedicPlanet,
    pub exaltation_score: f64,   // 0.0 ~ 60.0 (Uchcha Bala)
    pub directional_score: f64,  // 0.0 ~ 60.0 (Dig Bala)
    pub chesta_score: f64,       // 0.0 ~ 60.0 (Chesta Bala - Motion)
    pub naisargika_score: f64,   // 0.0 ~ 60.0 (Natural strength)
    pub kala_score: f64,         // 0.0 ~ 60.0 (Time strength - Day/Night)
    pub drik_score: f64,         // Aspect strength (can be negative)
    pub paksha_score: f64,       // Moon Phase strength
    pub ayana_score: f64,        // Declination strength
    pub saptavargaja_score: f64, // 0.0 ~ 60.0 (Strength across 7 Vargas)
    // Additional Sthana Bala components (BPHS)
    pub kendra_bala: f64,           // 0.0 ~ 60.0 (Kendra house strength)
    pub drekkana_bala: f64,         // 0.0 ~ 60.0 (Drekkana strength)
    pub ojayugmarasyamsa_bala: f64, // 0.0 ~ 15.0 (Odd/Even sign strength)
    pub yuddha_bala: f64,           // Planetary war adjustment (can be positive or negative)
    pub ishta_phala: f64,           // Auspiciousness (0-60)
    pub kashta_phala: f64,          // Inauspiciousness (0-60)
    pub total_score: f64,           // Aggregate for MVP
    pub status: String,             // "Exalted", "Debilitated", "Strong", "Weak", "Neutral"
    pub sthana_bala: f64,
    pub dig_bala: f64,
    pub kala_bala: f64,
    pub chesta_bala: f64,
    pub naisargika_bala: f64,
    pub drik_bala: f64,
}
```
All six factors of Shadbala are populated: `sthana_bala`, `dig_bala`, `kala_bala`, `chesta_bala`, `naisargika_bala`, and `drik_bala` (clamped to `[-60.0, 60.0]`).

### B. Compatibility Engine
In `crates/eon-vedic/src/analysis/matching.rs` (lines 5-25):
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KootaScore {
    pub id: String,
    pub name: String,
    pub max_points: f64,
    pub earned_points: f64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompatibilityReport {
    pub total_score: f64, // out of 36
    pub is_compatible: bool, // total >= 18 and no critical Nadi/Bhakoot dosha
    pub kootas: Vec<KootaScore>,
    pub male_mangal_dosha: bool,
    ...
}
```
The Ashtakoota 8 factors are fully calculated and stored inside `kootas`.

### C. KP System Analysis
In `crates/eon-vedic/src/analysis/kp.rs` (lines 7-25):
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KpPoint {
    pub name: String,
    pub longitude: f64,
    pub rasi: u8,
    pub nakshatra: u8,
    pub pada: u8,
    pub sign_lord: VedicPlanet,
    pub star_lord: VedicPlanet,
    pub sub_lord: VedicPlanet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KpAnalysis {
    pub cusps: Vec<KpPoint>,
    pub planets: Vec<KpPoint>,
}
```
`KpAnalysis` contains unequal Placidus house cusps (12 points) and mapped planets with their sign/star/sub lords. However, it does not currently compute the KP Significators.

### D. Dasha Engine
In `crates/eon-vedic/src/analysis/dasha.rs` (lines 5-13):
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DashaPeriod {
    pub lord: VedicPlanet,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub sub_dashas: Vec<DashaPeriod>,
    pub name: Option<String>,
}
```
It supports recursion based on the `levels` argument.
In `crates/eon-vedic/src/analysis/report.rs` (line 147):
```rust
            let timeline = crate::analysis::dasha::VimshottariDasha::calculate_timeline(
                birth_time,
                m.sidereal_deg,
                2, // Mahadasha + Antardasha
            );
```
Currently, `levels` is set to `2`.

### E. Dioxus UI & Translations
- `crates/eon-ui/src/components/tabs/vedic_tab.rs` contains the UI components for rendering the Vedic Astrology tab.
- `crates/eon-ui/src/components/tabs/strength_tab.rs` renders the Saju and Vedic planet strengths side-by-side.
- `crates/eon-ui/src/i18n/` contains translation dictionaries (`mod.rs`, `ko.rs`, `en.rs`, `zh.rs`, `ru.rs`) where compatibility matching names and descriptions are already fully translated.

---

## 2. Logic Chain

1. **R1 (Ashtakoota Compatibility Visuals)**: The matching engine already outputs detailed scores in `kootas`. The UI (`vedic_tab.rs`, line 2220) currently lists these scores in a table. To make it more interactive, we should render custom progress bars using Dioxus `rsx!` markup (e.g. mapping `earned_points / max_points` to width percentage) rather than changing the engine logic.
2. **R2 (Shadbala & Bhava Bala 6 Factors)**: The engine already computes `sthana_bala`, `dig_bala`, `kala_bala`, `chesta_bala`, `naisargika_bala`, and `drik_bala`. We will show a scorecard grid for all planets in the UI (`strength_tab.rs` or a sub-section in `vedic_tab.rs`). We can define the standard Rupa minimum thresholds:
   - Sun: 390, Moon: 360, Mars: 300, Mercury: 420, Jupiter: 390, Venus: 330, Saturn: 300, Rahu/Ketu: 240.
   We can map the ratio of `total_score / rupa_minimum` to percentage and letter grades:
   - A+ ($\ge 120\%$), A ($\ge 100\%$), B ($\ge 80\%$), C ($\ge 60\%$), D ($< 60\%$).
   For each planet card, we will render 6 horizontal progress bars representing the contribution of each of the 6 sub-strengths.
3. **R3 (KP System Cusps & Significators)**: The engine currently lacks a significator determination algorithm. We can add a function to `kp.rs` that calculates level A, B, C, D significators for all 12 houses:
   - **Level A**: Planets in Nakshatra of occupants of the house.
   - **Level B**: Planets occupying the house.
   - **Level C**: Planets in Nakshatra of the house lord.
   - **Level D**: Lord of the house.
   We will update `KpAnalysis` to include `significators: Vec<KpHouseSignificators>` and render this inside `vedic_tab.rs`. We must define localized labels in the translation files.
4. **R4 (Hierarchical Dasha accordion)**: Currently, only 2 levels of Dasha are computed. Changing the `levels` argument in `report.rs` to `3` will yield Mahadasha -> Antardasha -> Pratyantardasha. In `vedic_tab.rs`, we can introduce Dioxus signals to track which Mahadasha and Antardasha are expanded, and auto-expand the ones containing the current UTC time. We will highlight the active periods with glowing borders and badges.

---

## 3. Caveats

- We assume Placidus unequal house cusps are calculated correctly. In polar regions, the engine automatically falls back to Koch, Porphyry, or Equal, which is already verified by tests.
- Rahu and Ketu do not own signs in classical Parashara astrology, so their `lord_score` in Bhava Bala defaults to `0.0`. This is expected behaviour.

---

## 4. Conclusion

The implementation of Vedic Astrology enhancements for M2-M5 is highly feasible and requires:
1. Extending `kp.rs` with the 4-level significator algorithm and exposing it in `VedicAnalysisOutput`.
2. Increasing Vimshottari levels to `3` in `report.rs`.
3. Updating `vedic_tab.rs` (KP tables, Dasha accordion, Ashtakoota progress bars) and `strength_tab.rs` (Shadbala detailed scorecards).
4. Registering additional translation keys for KP significator stages in the 4 supported locales.

---

## 5. Verification Method

- Run `cargo test --package eon-vedic` to ensure all 54 unit tests pass successfully.
- Verify compiling of Dioxus UI using `dx build` under `crates/eon-ui`.
- Access the web interface and visually inspect:
  1. The new progress bars in the Ashtakoota table under Compatibility.
  2. The detailed scorecard grid under Strength.
  3. The 12-house significator table under KP System.
  4. The multi-level accordion timeline under Dashas.
