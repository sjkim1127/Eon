# Handoff Report: E2E Testing Strategy for Vedic Astrology & Service Integration

This report outlines the comprehensive E2E integration testing plan and strategy for the `eon-vedic` and `eon-service` crates, covering 49 total test cases across 4 tiers as defined by the project requirements.

---

## 1. Observation

During the exploration of the codebase, the following file paths, structures, and behaviors were observed:

### A. Service DTO and Facade Structures
In `crates/eon-service/src/dto.rs` (lines 81-114, 159-185), the input and output structures for Vedic analysis and compatibility are defined:
```rust
pub struct VedicAnalysisInput {
    #[serde(flatten)]
    pub base: AnalysisInput,
    pub precision: BirthTimePrecision,
    pub current: CurrentContext,
    pub target_year: Option<i32>,
}

pub struct VedicAnalysisOutput {
    pub meta: AnalysisMeta,
    pub report: eon_vedic::analysis::report::VedicAnalysisReport,
    pub tajika_report: Option<eon_vedic::analysis::report::TajikaReport>,
    pub chart: eon_vedic::core::chart::VedicChart,
    pub annual_chart: Option<eon_vedic::core::chart::VedicChart>,
    pub gochara: eon_vedic::analysis::gochara::GocharaSummary,
    pub varga_nakshatra_reports: eon_vedic::analysis::varga_nakshatra_report::VargaNakshatraReports,
    pub kp_analysis: Option<eon_vedic::analysis::kp::KpAnalysis>,
}

pub struct VedicCompatibilityInput {
    pub male: AnalysisInput,
    pub female: AnalysisInput,
}

pub struct VedicCompatibilityOutput {
    pub meta: AnalysisMeta,
    pub report: eon_vedic::analysis::matching::CompatibilityReport,
}
```

In `crates/eon-service/src/facade.rs` (lines 8-14), the core integration entry points are defined:
```rust
pub fn analyze_vedic(input: VedicAnalysisInput) -> Result<VedicAnalysisOutput, ServiceError> {
    crate::services::vedic::analyze(input)
}

pub fn analyze_vedic_compatibility(input: VedicCompatibilityInput) -> Result<VedicCompatibilityOutput, ServiceError> {
    crate::services::vedic::analyze_compatibility(input)
}
```

### B. Core Vedic Calculations
In `crates/eon-vedic/src/analysis/strength.rs` (lines 7-29), `PlanetStrength` encapsulates the Shadbala factors:
```rust
pub struct PlanetStrength {
    pub planet: VedicPlanet,
    pub exaltation_score: f64,   // Uchcha Bala
    pub directional_score: f64,  // Dig Bala
    pub chesta_score: f64,       // Chesta Bala
    pub naisargika_score: f64,   // Natural strength
    pub kala_score: f64,         // Time strength
    pub drik_score: f64,         // Aspect strength
    pub paksha_score: f64,       // Moon Phase strength
    pub ayana_score: f64,        // Declination strength
    pub saptavargaja_score: f64, // 7 Vargas strength
    pub kendra_bala: f64,        // Positional Kendra
    pub drekkana_bala: f64,
    pub ojayugmarasyamsa_bala: f64,
    pub yuddha_bala: f64,        // Planetary war
    pub ishta_phala: f64,
    pub kashta_phala: f64,
    pub total_score: f64,
    pub status: String,
}
```

In `crates/eon-vedic/src/analysis/kp.rs` (lines 7-25), the KP System utilizes Placidus unequal house systems through `AstroEngine::get_houses`:
```rust
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

pub struct KpAnalysis {
    pub cusps: Vec<KpPoint>,
    pub planets: Vec<KpPoint>,
}
```
And in `crates/eon-astro/src/lib.rs` (lines 339-372), the Placidus system is invoked via Swiss Ephemeris `swe_houses` using `'P' as i32`:
```rust
let ret = swiss_eph::swe_houses(
    julian_day,
    latitude,
    longitude,
    house_system, // 'P' (80)
    cusps.as_mut_ptr(),
    ascmc.as_mut_ptr(),
);
```

In `crates/eon-vedic/src/analysis/dasha.rs` (lines 5-13, 33-37), the hierarchical dasha timeline represents the Vimshottari Dasha structure:
```rust
pub struct DashaPeriod {
    pub lord: VedicPlanet,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub sub_dashas: Vec<DashaPeriod>,
    pub name: Option<String>,
}

impl VimshottariDasha {
    pub fn calculate_timeline(
        birth_time: DateTime<Utc>,
        moon_long: f64,
        levels: u8,
    ) -> Vec<DashaPeriod> { ... }
}
```

### C. Timezone & Boundary Errors
In `crates/eon-core/src/birth.rs` (lines 188-205), invalid dates or non-existent local times (such as hours skipped due to DST transitions) return a `CoreError`:
```rust
pub fn to_utc(&self) -> Result<DateTime<Utc>, crate::error::CoreError> {
    let naive = self.local_datetime()
        .ok_or(crate::error::CoreError::InvalidDateTime)?;
    ...
    match tz.from_local_datetime(&naive) {
        chrono::LocalResult::Single(dt) => Ok(dt.with_timezone(&Utc)),
        chrono::LocalResult::Ambiguous(dt1, _dt2) => Ok(dt1.with_timezone(&Utc)),
        chrono::LocalResult::None => Err(crate::error::CoreError::NonExistentLocalTime),
    }
}
```
These errors are bubbled up to the `eon-service` layer as `ServiceError::BirthInfo` (in `crates/eon-service/src/birth.rs` line 12).

---

## 2. Logic Chain

1. **Calculations Coverage**: To verify that the core astrology implementations are correct, we must inspect the outputs of `analyze_vedic` and `analyze_vedic_compatibility`. 
2. **Feature Independence**: The test suite must verify the outputs of the four primary features (Ashtakoota Compatibility, Shadbala/Bhava Bala, KP Cusps/Lords, Vimshottari/Yogini Dashas) using an opaque-box method.
3. **Placidus Vulnerability**: In `eon-astro`, Placidus house division (`'P'`) is used for KP calculations. Swiss Ephemeris's Placidus algorithm fails at latitudes near or inside the polar circles ($> 66.5^\circ$ N/S) because the horizon and ecliptic do not intersect cleanly. Therefore, polar coordinates (e.g. latitude $= 70^\circ$) represent a critical boundary case that will throw `AstroError::HouseCalculationError`.
4. **DST Skips**: The time-zone conversions in `eon-core` will reject non-existent wall-clock times (e.g. 02:30 during Spring-forward transitions) with `CoreError::NonExistentLocalTime`. E2E tests must verify that such inputs return a structured error instead of panicking.
5. **Coupling/Interaction**: Under `crates/eon-vedic/src/analysis/yogas.rs` (lines 168-171, 189-192), Yoga manifestation quality is gated by Shadbala planetary strengths. This shows cross-feature coupling that must be covered under Tier 3.

---

## 3. Caveats

- **Ephemeris Precision**: The integration tests assume that the compiled-in or local cached Swiss Ephemeris files are available. If ephemeris files are completely missing, the astro engine falls back to analytical formulas, which may cause minor deviation in floating-point results. Tolerance thresholds (`epsilon` of `0.0001` degrees) should be used during assertions.
- **Polar Fallbacks**: The current implementation of KP houses does not feature an automatic mathematical fallback (like switching to Koch or Whole Sign) if Placidus fails. The test suite must assume that calling `analyze_vedic` at polar latitudes will return a `ServiceError`.

---

## 4. Conclusion

A comprehensive 4-tiered, 49-case integration test suite is required to validate both `eon-vedic` and `eon-service` under all valid, boundary, combined, and real-world conditions. The suite must be executed using cargo integration tests and run using the `eon-service` facade to ensure API stability.

---

## 5. Proposed `TEST_INFRA.md` Structure

The file `TEST_INFRA.md` at the project root should be structured as follows:

```markdown
# TEST_INFRA — E2E Testing Infrastructure & Philosophy

## 1. Test Philosophy
- **Opaque-Box Verification**: Tests invoke the highest public facades (`eon_service::facade`) and verify JSON-serializable outputs against expected contracts.
- **Requirement-Driven**: Tests are designed directly from domain specifications (Ashtakoota rules, BPHS Shadbala formulas, Placidus house divisions, Vimshottari calculations).
- **Zero-Tolerance for Panics**: Boundary conditions must produce structured errors (`ServiceError`), never panics.

## 2. Test Architecture
- **Integration Test Path**: `crates/eon-vedic/tests/compatibility_shadbala_kp_dasha.rs`
- **Execution Command**: `cargo test --package eon-vedic --test compatibility_shadbala_kp_dasha`
- **Oracles**: Snapshots or code-defined expected structures mapped to specific test IDs.

## 3. Feature Inventory & Coverage Matrix
- Feature 1: Ashtakoota Compatibility (8 Kootas, 36 Gunas, Mangal Dosha)
- Feature 2: Shadbala (6 Strengths) & Bhava Bala (12 House Strengths)
- Feature 3: KP System (Placidus Unequal House Cusps, Sign/Star/Sub-Lords)
- Feature 4: Vimshottari & Yogini Dasha (120-year and 36-year timelines, remaining balances)

## 4. Execution Tiers
- **Tier 1**: Feature Coverage (T1_COMP_01 to T1_DASH_05)
- **Tier 2**: Boundary & Corners (T2_COMP_01 to T2_DASH_05)
- **Tier 3**: Cross-Feature Combinations (T3_COMB_01 to T3_COMB_04)
- **Tier 4**: Real-World Application Workloads (T4_WORK_01 to T4_WORK_05)
```

---

## 6. Concrete List of 49 Test Cases

### Tier 1: Feature Coverage (20 Cases)

#### Ashtakoota Compatibility (5 Cases)
*   **T1_COMP_01: Golden Match**
    *   *Inputs*: Male: 1990-05-15 08:30:00 (Moon in Rohini), Female: 1990-05-15 08:30:00 (Moon in Rohini), Seoul, "Asia/Seoul".
    *   *Assertion*: `total_score >= 30.0`, `is_compatible = true`, Nadi score = 8.0, Bhakoot score = 7.0.
*   **T1_COMP_02: Nadi Dosha Clash**
    *   *Inputs*: Male: Moon in Ardra (Nadi = Adi), Female: Moon in Swati (Nadi = Adi).
    *   *Assertion*: `is_compatible = false`, Nadi points = 0.0, explanation contains "Nadi Dosha".
*   **T1_COMP_03: Bhakoot Dosha Clash**
    *   *Inputs*: Male: Moon in Aries (Rasi 1), Female: Moon in Virgo (Rasi 6) (6-8 Relationship).
    *   *Assertion*: `is_compatible = false`, Bhakoot points = 0.0, explanation contains "Bhakoot Dosha".
*   **T1_COMP_04: Mutual Mangal Dosha Cancellation**
    *   *Inputs*: Male with Mars in 7th house, Female with Mars in 7th house.
    *   *Assertion*: `male_mangal_dosha = true`, `female_mangal_dosha = true`, `mangal_dosha_cancelled = true`.
*   **T1_COMP_05: Active Mangal Dosha (Uncancelled)**
    *   *Inputs*: Male with Mars in 8th house, Female with Mars in 3rd house.
    *   *Assertion*: `male_mangal_dosha = true`, `female_mangal_dosha = false`, `mangal_dosha_cancelled = false`.

#### Shadbala & Bhava Bala (5 Cases)
*   **T1_SHAD_01: Exalted Sun Strength**
    *   *Inputs*: 2026-04-14 12:00:00 (Sun in Aries at ~0°), Seoul.
    *   *Assertion*: Sun `exaltation_score >= 50.0`, `status = "Exalted"`, `total_score > 200.0`.
*   **T1_SHAD_02: Debilitated Moon Strength**
    *   *Inputs*: 2026-11-15 12:00:00 (Moon in Scorpio at ~3°), Seoul.
    *   *Assertion*: Moon `exaltation_score <= 10.0`, `status = "Debilitated"`.
*   **T1_SHAD_03: Planetary War (Yuddha Bala)**
    *   *Inputs*: Conjunction of Mars and Venus within 1 degree (e.g. 2026-02-25 15:00:00).
    *   *Assertion*: One planet has positive `yuddha_bala`, the other has negative `yuddha_bala` adjustment.
*   **T1_SHAD_04: Dig Bala (Directional Strength)**
    *   *Inputs*: Jupiter in 1st house (rising at birth time).
    *   *Assertion*: Jupiter `directional_score >= 55.0` (max 60.0).
*   **T1_SHAD_05: Bhava Bala Calculation**
    *   *Inputs*: Standard birth profile (1990-05-15 08:30:00, Seoul).
    *   *Assertion*: `bhava_strengths` contains exactly 12 houses with non-zero total scores and descriptive reasons.

#### KP System Lords & Significators (5 Cases)
*   **T1_KP_01: Middle Latitude Placidus Cusps**
    *   *Inputs*: 1990-05-15 08:30:00, Seoul (37.5665° N, 126.9780° E).
    *   *Assertion*: `kp_analysis` contains exactly 12 house cusps, all sorted sequentially in longitude.
*   **T1_KP_02: planet KP Lords mapping**
    *   *Inputs*: 1990-05-15 08:30:00, Seoul.
    *   *Assertion*: Sun `sign_lord`, `star_lord`, and `sub_lord` are populated and match Lahiri-corrected longitudes.
*   **T1_KP_03: House Cusp KP Lords mapping**
    *   *Inputs*: 1990-05-15 08:30:00, Seoul.
    *   *Assertion*: House 1 cusp (Lagna) `sign_lord`, `star_lord`, and `sub_lord` are populated and mathematically correct.
*   **T1_KP_04: Equator Cusp Calculation**
    *   *Inputs*: 1990-05-15 08:30:00, Latitude = 0.0, Longitude = 100.0, "UTC".
    *   *Assertion*: Placidus house cusps are calculated successfully without mathematical division errors.
*   **T1_KP_05: Prime Meridian Cusp Calculation**
    *   *Inputs*: 1990-05-15 08:30:00, Greenwich (51.4779° N, 0.0° W).
    *   *Assertion*: Cusps computed successfully, longitudes verified against prime meridian offset.

#### Hierarchical Dasha Timeline (5 Cases)
*   **T1_DASH_01: Vimshottari Mahadasha Sequence**
    *   *Inputs*: Moon in Ashwini Nakshatra (Ketu lord).
    *   *Assertion*: Sequence of Mahadashas begins with Ketu, followed by Venus, Sun, Moon, Mars, Rahu, Jupiter, Saturn, Mercury.
*   **T1_DASH_02: Vimshottari Level 2 (Antardasha)**
    *   *Inputs*: Standard birth profile, `levels = 2`.
    *   *Assertion*: Each `DashaPeriod` in the timeline contains a non-empty list of `sub_dashas` representing the 9 nested Antardashas.
*   **T1_DASH_03: First Dasha Balance**
    *   *Inputs*: Moon at 6°40' Aries (exactly 50% through Ashwini Nakshatra, Ketu lord).
    *   *Assertion*: The first dasha period (Ketu) has a remaining duration of exactly 3.5 years (50% of 7 years).
*   **T1_DASH_04: Vimshottari Cumulative Duration**
    *   *Inputs*: Any valid birth chart.
    *   *Assertion*: Sum of all Mahadasha durations equals exactly 120 years.
*   **T1_DASH_05: Yogini Dasha Sequence**
    *   *Inputs*: Moon Nakshatra mapped to Yogini starting lord.
    *   *Assertion*: Cycle of 8 Yogini Dashas (Mangala, Pingala, Dhanya, Bhramari, Bhadrika, Ulka, Siddha, Sankata) totaling 36 years.

---

### Tier 2: Boundary and Corner Cases (20 Cases)

#### Ashtakoota Compatibility (5 Cases)
*   **T2_COMP_01: Identical Nakshatra and Pada**
    *   *Inputs*: Male and Female with exact same birth time and place (identical charts).
    *   *Assertion*: Compatibility calculates, Nadi score = 8.0, Bhakoot score = 7.0 (no Nadi/Bhakoot dosha when Nakshatra and Pada are identical).
*   **T2_COMP_02: Rasi Boundary Transition**
    *   *Inputs*: Male Moon at 29°59' Aries, Female Moon at 0°01' Taurus.
    *   *Assertion*: Handled without precision errors, signs are treated as distinct (Aries vs. Taurus), Graha Maitri calculates for Mars vs. Venus.
*   **T2_COMP_03: Invalid Timezone Input**
    *   *Inputs*: Timezone = "Invalid/Timezone_Name".
    *   *Assertion*: Returns `Err(ServiceError::BirthInfo(..))`. No panics.
*   **T2_COMP_04: Unknown Birth Time (Noon Proxy)**
    *   *Inputs*: `unknown_time = true`.
    *   *Assertion*: Birth hour is forced to 12:00:00 (Noon proxy). Moon position and compatibility calculated using proxy values.
*   **T2_COMP_05: Southern vs Northern Hemisphere Coordinates**
    *   *Inputs*: Male: Melbourne (-37.8136, 144.9631), Female: London (51.5074, -0.1278).
    *   *Assertion*: Latitude signs (negative vs positive) are preserved, and chart compatibility resolves correctly.

#### Shadbala & Bhava Bala (5 Cases)
*   **T2_SHAD_01: Exact Exaltation Longitude**
    *   *Inputs*: Sun at exactly 10.0° Aries (10.0° in Sign 1).
    *   *Assertion*: Sun `exaltation_score` is exactly 60.0.
*   **T2_SHAD_02: Exact Debilitation Longitude**
    *   *Inputs*: Sun at exactly 10.0° Libra (10.0° in Sign 7).
    *   *Assertion*: Sun `exaltation_score` is exactly 0.0.
*   **T2_SHAD_03: Planetary War Exact Conjunction**
    *   *Inputs*: Two planets at the exact same longitude (difference = 0.0°).
    *   *Assertion*: War winner is determined deterministically without floating-point division by zero.
*   **T2_SHAD_04: Polar Latitudes Shadbala**
    *   *Inputs*: Latitude = 70.0° N, Longitude = 20.0° E.
    *   *Assertion*: Planet positions and strengths calculate successfully (Shadbala calculation is insulated from Placidus failures).
*   **T2_SHAD_05: Kala Bala Midday/Midnight boundaries**
    *   *Inputs*: Birth at exactly 12:00:00 local time vs 00:00:00 local time.
    *   *Assertion*: Correctly classified as day/night births; no boundary edge issues.

#### KP System Lords & Significators (5 Cases)
*   **T2_KP_01: Polar Circle Cusp Failure**
    *   *Inputs*: Latitude = 75.0° N, Longitude = 15.0° E (inside polar circle).
    *   *Assertion*: Returns `Err(ServiceError::Vedic(..))` or similar indicating house calculation failed (graceful error handling, no panics).
*   **T2_KP_02: Cusp Longitude Boundary Crossing**
    *   *Inputs*: Cusp longitude at exactly 29.999° of a sign.
    *   *Assertion*: Sign, star, and sub lord map to correct boundaries without rounding up to the next sign prematurely.
*   **T2_KP_03: North Pole Extreme (Latitude = 90.0°)**
    *   *Inputs*: Latitude = 90.0° N.
    *   *Assertion*: Graceful rejection with `ServiceError`.
*   **T2_KP_04: Fractional Hour Timezone**
    *   *Inputs*: Birth in Kathmandu, Nepal (timezone offset +05:45).
    *   *Assertion*: Standard local time converted to UTC correctly, and house cusps computed without shifts.
*   **T2_KP_05: DST Skip Hour Input**
    *   *Inputs*: Local time inputted during Spring DST skip hour (e.g. 2026-03-08 02:30:00 EST).
    *   *Assertion*: Returns `Err(ServiceError::BirthInfo("NonExistentLocalTime"))` or similar.

#### Hierarchical Dasha Timeline (5 Cases)
*   **T2_DASH_01: Moon at End of Nakshatra**
    *   *Inputs*: Moon longitude at 13°19'59" within a Nakshatra (99.99% elapsed).
    *   *Assertion*: First dasha duration is nearly 0 seconds; transitions to next lord almost instantly.
*   **T2_DASH_02: Moon at Start of Nakshatra**
    *   *Inputs*: Moon longitude at 0°00'01" within a Nakshatra (0.01% elapsed).
    *   *Assertion*: First dasha duration is nearly 100% of its full cycle.
*   **T2_DASH_03: Leap Year Birth Date**
    *   *Inputs*: Birth on February 29, 2024.
    *   *Assertion*: Dasha timeline calculates successfully and maintains 365.2425 days/year accuracy.
*   **T2_DASH_04: Julian/Gregorian Switch Date**
    *   *Inputs*: Birth on October 15, 1582.
    *   *Assertion*: Core astro conversion holds and dasha timeline calculates without date discontinuity.
*   **T2_DASH_05: Nested Antardasha UTC Day Boundary**
    *   *Inputs*: Antardasha boundary transitions exactly at 00:00:00 UTC.
    *   *Assertion*: Start and end dates are contiguous (end time of period N equals start time of period N+1).

---

### Tier 3: Cross-Feature Combinations (4 Cases)

*   **T3_COMB_01: Compatibility & Active Dasha Correlation**
    *   *Inputs*: Male and Female compatibility requested.
    *   *Assertion*: Verification that the active Vimshottari Mahadasha lords of both partners are extracted and compared against the Graha Maitri (Friendship) score.
*   **T3_COMB_02: Shadbala & Yoga Quality Gating**
    *   *Inputs*: Saturn in Libra (exalted, high Shadbala) vs Saturn in Aries (debilitated, low Shadbala) in the 1st house.
    *   *Assertion*: Sasa Mahapurusha Yoga is active with "VeryHigh" quality for exalted Saturn, but is weakened or deactivated for debilitated Saturn (Shadbala score < 100.0).
*   **T3_COMB_03: KP Cusps & Gochara (Transit) Integration**
    *   *Inputs*: Gochara transit calculated using natal KP cusp longitudes.
    *   *Assertion*: Verify that transit events trigger when transiting planets conjunct the exact unequal longitude of the KP cusps.
*   **T3_COMB_04: Annual Chart muntha Rasi & Vimshottari Dasha**
    *   *Inputs*: Target year set to calculate Tajika report.
    *   *Assertion*: Extract Muntha Rasi from Tajika report and verify if its lord matches the active Vimshottari Mahadasha lord.

---

### Tier 4: Real-World Workloads (5 Cases)

*   **T4_WORK_01: Standard Natal Reading Profile**
    *   *Inputs*: 1990-05-15 08:30:00, Seoul.
    *   *Assertion*: Full E2E analysis run. Verification that `meta`, `report`, `tajika_report`, `chart`, `gochara`, `varga_nakshatra_reports`, and `kp_analysis` fields are non-null and fully populated.
*   **T4_WORK_02: Relationship Compatibility Audit**
    *   *Inputs*: Male: 1988-06-12 15:45:00, Seoul. Female: 1990-09-20 10:30:00, Busan.
    *   *Assertion*: Compatibility output validated. Total points, Nadi/Bhakoot doshas, and Mangal Dosha values verified against golden oracle results.
*   **T4_WORK_03: Career & Wealth Audit**
    *   *Inputs*: 1975-10-30 04:20:00, New York.
    *   *Assertion*: Extracted KP significators for career houses (2, 6, 10, 11), checked Shadbala of the 10th house lord, and verified active Dasha periods.
*   **T4_WORK_04: 120-Year Timeline Reconstruction**
    *   *Inputs*: 1920-01-01 12:00:00, London.
    *   *Assertion*: Full Vimshottari timeline generated. Verifies that the nested Antardashas sum exactly to 120 years.
*   **T4_WORK_05: Extreme Location Profile (Iceland)**
    *   *Inputs*: Reykjavik, Iceland (64.1466° N, 21.9426° W).
    *   *Assertion*: Houses calculate successfully under high latitude (but below polar circles); KP analysis maps sign/star/sub lords without error.

---

## 7. Implementation Strategy for the Integration Test File

The integration test file will be created at:
`crates/eon-vedic/tests/compatibility_shadbala_kp_dasha.rs`

### A. Execution and Verification Method (Opaque-Box)
- The test runner will load input profiles and expected output schemas.
- It will execute the tests using the public API of the `eon-service` façade (such as `analyze_vedic` and `analyze_vedic_compatibility`).
- It will perform requirement-driven assertions on the output structures without depending on internal implementation details (e.g. verifying that Nadi Dosha sets points to 0, that polar latitudes return a specific error enum, etc.).

### B. Integration Test File Outline (Rust Template)
```rust
mod common;

use chrono::{TimeZone, Utc};
use eon_service::dto::{
    AnalysisInput, BirthTimePrecision, VedicAnalysisInput, VedicCompatibilityInput,
};
use eon_service::facade::{analyze_vedic, analyze_vedic_compatibility};
use eon_service::error::ServiceError;
use eon_vedic::planets::VedicPlanet;

// Helper to construct basic AnalysisInput
fn make_input(year: i32, month: u32, day: u32, hour: u32, minute: u32, lat: f64, lon: f64, tz: &str) -> AnalysisInput {
    AnalysisInput {
        year,
        month,
        day,
        hour,
        minute,
        is_lunar: false,
        is_leap_month: false,
        lat,
        lon,
        timezone: tz.to_string(),
    }
}

// Helper to construct VedicAnalysisInput
fn make_vedic_input(base: AnalysisInput, unknown_time: bool) -> VedicAnalysisInput {
    VedicAnalysisInput::new(base, Some(unknown_time), Some(Utc::now()))
}

// TIER 1: FEATURE COVERAGE TESTS
#[test]
fn test_tier1_feature_coverage() {
    // T1_COMP_01: Golden Match Compatibility
    let male = make_input(1990, 5, 15, 8, 30, 37.5665, 126.9780, "Asia/Seoul");
    let female = make_input(1990, 5, 15, 8, 30, 37.5665, 126.9780, "Asia/Seoul");
    let comp_input = VedicCompatibilityInput { male, female };
    
    let result = analyze_vedic_compatibility(comp_input).unwrap();
    assert!(result.report.total_score >= 30.0, "Golden match should have a high score");
    assert!(result.report.is_compatible, "Golden match must be compatible");

    // T1_SHAD_01: Exalted Sun Strength
    let base = make_input(2026, 4, 14, 12, 0, 37.5665, 126.9780, "Asia/Seoul");
    let input = make_vedic_input(base, false);
    let output = analyze_vedic(input).unwrap();
    let sun_pos = output.chart.planets.iter().find(|p| p.planet == VedicPlanet::Sun).unwrap();
    let strength = eon_vedic::analysis::strength::StrengthEngine::calculate(sun_pos, &output.chart);
    assert!(strength.exaltation_score >= 50.0, "Sun in Aries should have exalted score");
    assert_eq!(strength.status, "Exalted");

    // T1_KP_01: KP Cusp Count & Sorting
    assert_eq!(output.kp_analysis.as_ref().unwrap().cusps.len(), 12);
    let mut prev_long = -1.0;
    for cusp in &output.kp_analysis.as_ref().unwrap().cusps {
        assert!(cusp.longitude >= prev_long, "Cusps must be sorted by longitude");
        prev_long = cusp.longitude;
    }

    // T1_DASH_01: Vimshottari Sequence Length
    assert_eq!(output.report.dasha_timeline.len(), 9, "Should calculate 9 Mahadashas");
}

// TIER 2: BOUNDARY AND CORNER TESTS
#[test]
fn test_tier2_boundary_corners() {
    // T2_KP_01: Polar Latitudes Failure (Placidus undefined)
    let polar_base = make_input(1990, 5, 15, 8, 30, 75.0, 15.0, "UTC");
    let polar_input = make_vedic_input(polar_base, false);
    let polar_result = analyze_vedic(polar_input);
    
    assert!(
        polar_result.is_err(),
        "Placidus house calculation must return error at polar latitudes"
    );
    match polar_result.unwrap_err() {
        ServiceError::Vedic(msg) => {
            assert!(msg.contains("Failed to calculate houses") || msg.contains("HouseCalculationError"));
        }
        other => panic!("Expected ServiceError::Vedic, got {:?}", other),
    }

    // T2_COMP_03: Invalid Timezone Name
    let bad_base = make_input(1990, 5, 15, 8, 30, 37.5665, 126.9780, "Invalid/Zone_Name");
    let bad_input = make_vedic_input(bad_base, false);
    let bad_result = analyze_vedic(bad_input);
    assert!(bad_result.is_err());
    match bad_result.unwrap_err() {
        ServiceError::BirthInfo(msg) => {
            assert!(msg.contains("InvalidTimezone"));
        }
        other => panic!("Expected ServiceError::BirthInfo, got {:?}", other),
    }
}
```

### C. Asserting Floating-Point Values
For longitude and strength comparisons, exact comparisons (like `assert_eq!`) are prone to failing across different platforms or optimization levels. Tests should use approximate comparisons:
```rust
fn assert_approx(actual: f64, expected: f64, epsilon: f64, field_name: &str) {
    assert!(
        (actual - expected).abs() <= epsilon,
        "{} mismatch: expected {}, got {}",
        field_name, expected, actual
    );
}
```
