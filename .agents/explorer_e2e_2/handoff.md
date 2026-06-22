# Handoff Report - KP System and Dasha APIs & Test Structure Exploration

This report details the investigation of the public APIs in `eon-vedic` and `eon-service` for the KP (Krishnamurti Padhdhati) system and the Vimshottari/Yogini dasha timelines, as well as the structure and configuration of existing integration tests.

---

## 1. Observation

We directly observed the following APIs, types, signatures, and test configurations in the codebase:

### A. KP System APIs
* **Module**: `eon_vedic::analysis::kp` (re-exported as `eon_vedic::kp` in `crates/eon-vedic/src/lib.rs:33`)
* **Core Structs**:
  1. `KpPoint` (`crates/eon-vedic/src/analysis/kp.rs:9-18`):
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
     ```
  2. `KpAnalysis` (`crates/eon-vedic/src/analysis/kp.rs:20-25`):
     ```rust
     #[derive(Debug, Clone, Serialize, Deserialize)]
     #[serde(rename_all = "camelCase")]
     pub struct KpAnalysis {
         pub cusps: Vec<KpPoint>,
         pub planets: Vec<KpPoint>,
     }
     ```
* **Core Signatures**:
  * `KpAnalysis::calculate` (`crates/eon-vedic/src/analysis/kp.rs:27-35`):
    ```rust
    impl KpAnalysis {
        pub fn calculate(
            time: DateTime<Utc>,
            latitude: f64,
            longitude: f64,
            ayanamsa: f64,
            natal_planets: &[VedicPosition],
            engine: &AstroEngine,
        ) -> Result<Self, String>
    }
    ```
* **Facade Integration**:
  * `eon-service` facade calls `KpAnalysis::calculate` inside `analyze` service (`crates/eon-service/src/services/vedic.rs:64-71`):
    ```rust
    let kp_analysis = Some(eon_vedic::analysis::kp::KpAnalysis::calculate(
        dt,
        input.base.lat,
        input.base.lon,
        chart.ayanamsa,
        &chart.planets,
        calculator.engine(),
    ).map_err(|e| ServiceError::Vedic(e))?);
    ```

### B. Dasha Timeline APIs
We observed two distinct dasha modules: one designed for **astrological analysis reports** and another for **advanced predictions** (supporting customizable year lengths).

#### 1. Analysis Dashas (`eon_vedic::analysis::dasha`)
* **Module**: `eon_vedic::analysis::dasha` (re-exported as `eon_vedic::prediction::dasha` in `crates/eon-vedic/src/lib.rs:31`)
* **Core Structs**:
  1. `DashaPeriod` (`crates/eon-vedic/src/analysis/dasha.rs:5-13`):
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
* **Core Signatures**:
  1. `VimshottariDasha::calculate_timeline` (`crates/eon-vedic/src/analysis/dasha.rs:33-37`):
     ```rust
     impl VimshottariDasha {
         pub fn calculate_timeline(
             birth_time: DateTime<Utc>,
             moon_long: f64,
             levels: u8,
         ) -> Vec<DashaPeriod>
     }
     ```
  2. `YoginiDasha::calculate_timeline` (`crates/eon-vedic/src/analysis/dasha.rs:253-256`):
     ```rust
     impl YoginiDasha {
         pub fn calculate_timeline(
             birth_time: DateTime<Utc>,
             moon_long: f64,
         ) -> Vec<DashaPeriod>
     }
     ```

#### 2. Prediction Dashas (`eon_vedic::prediction::dasha`)
* **Module**: `eon_vedic::prediction::dasha`
* **Core Structs**:
  1. `DashaPeriod` (`crates/eon-vedic/src/prediction/dasha.rs:7-16`):
     ```rust
     #[derive(Debug, Clone, Serialize, Deserialize)]
     pub struct DashaPeriod {
         pub planet: VedicPlanet,
         pub start_date: DateTime<Utc>,
         pub end_date: DateTime<Utc>,
         pub duration_years: f64,
         pub level: u8,
         pub name: Option<String>,
         pub sub_periods: Vec<DashaPeriod>,
     }
     ```
* **Core Signatures**:
  1. `Vimshottari::calculate` (`crates/eon-vedic/src/prediction/dasha.rs:46-51`):
     ```rust
     impl Vimshottari {
         pub fn calculate(
             moon_longitude: f64,
             birth_date: DateTime<Utc>,
             max_level: u8,
             year_type: VedicYearType,
         ) -> Vec<DashaPeriod>
     }
     ```
  2. `Yogini::calculate` (`crates/eon-vedic/src/prediction/dasha.rs:248-253`):
     ```rust
     impl Yogini {
         pub fn calculate(
             moon_longitude: f64,
             birth_date: DateTime<Utc>,
             max_level: u8,
             year_type: VedicYearType,
         ) -> Vec<DashaPeriod>
     }
     ```
  * Note: `VedicYearType` supports `Savana` (360 days), `Gregorian` (365.2425 days), and `Sidereal` (365.256363 days) configurations (`crates/eon-vedic/src/core/config.rs:24-28`).

---

## 2. Logic Chain

From our observations, we trace the logic of the Vedic system and existing integration test setup as follows:

1. **Zodiac & Ayanamsa configuration**:
   * Charts are calculated sidereally. To convert a tropical longitude to sidereal, an ayanamsa offset is subtracted (`sidereal = (tropical - ayanamsa + 360.0) % 360.0`).
   * Tests configure this via the `VedicConfig` struct (containing `ayanamsa: AyanamsaSystem` and `node_calc: NodeCalculation`), which is passed to `VedicChartCalculator::with_config`.
   * Standard tests (e.g. `crates/eon-vedic/tests/basic.rs`) construct the calculator using the default configuration (`VedicChartCalculator::new()`) and calculate coordinates using a `chrono::DateTime<Utc>`, `latitude: f64`, and `longitude: f64`.

2. **Test Setup Pattern**:
   * Integrations tests in `crates/eon-vedic/tests/` use helper functions from `common/mod.rs` to abstract chart generation:
     * `common::create_test_chart(year, month, day, hour, lat, lon)` utilizes default config.
     * Custom regression tests loop over `FIXTURES` (historical profiles defined in `fixtures/mod.rs` with exact dates, locations, and ayanamsas) to verify various components (Jaimini, Varga, Avastha, Dasha, Tajika) against pre-calculated baselines.
   * Standard testing strategy for other modules (e.g., `position_oracle_verify.rs`, `vimshopaka_oracle_verify.rs`) involves loading JSON files containing inputs and expected outputs, recalculating the chart sidereal degrees using Swiss Ephemeris FFI, and asserting results within a minor tolerance epsilon (typically `0.1` or `1e-4`).

3. **Compilation Conflict**:
   * Running `cargo test --package eon-vedic` failed because the integration test `crates/eon-vedic/tests/compatibility_shadbala_kp_dasha.rs` tries to reference deprecated or incorrect fields on `PlanetStrength` (e.g. `sthana_bala`, `chesta_bala`, `naisargika_bala`, `dig_bala`, `kala_bala`, `drik_bala`). 
   * However, `cargo check --workspace` passes because tests are not verified by default, and `cargo check --tests` fails at the same location. The rest of the library and facade code compiled successfully.

---

## 3. Caveats

* Placidus houses, which the KP System relies on (`engine.get_houses(..., b'P' as i32)`), fail or exhibit high mathematical instability near polar regions (latitudes > 66.5° N/S). The current KP implementation will pass this error up as a `String` from `get_houses` or propagate it through `ServiceError::Vedic` in the service facade.
* We have not fully investigated how the system handles the Krishnamurti ayanamsa specifically in the core calculator compared to the Raman or Lahiri ayanamsas. Since Krishnamurti is one of the variants in `AyanamsaSystem`, the calculator supports it, but standard facades default to `Lahiri`.
* We did not trace other dasha implementations such as Jaimini Chara Dasha and Kalachakra Dasha, as the user's primary concern is Vimshottari and Yogini.

---

## 4. Conclusion

The module structures and signatures for KP and Dashas are fully exposed, clean, and easily invocable. Although the library's implementation for KP and Dashas is covered in `compatibility_shadbala_kp_dasha.rs`, the compilation error in the `PlanetStrength` assertion blocks overall test suite completion.

To clean up, stabilize, and verify these features, we propose the following 4-tier testing matrix:

### Tier 1: Feature Coverage (Core API Sanity)
* **TC-KP-01: Cusp & House Lord Calculation**
  * *Objective*: Verify Placidus unequal house cusps and corresponding sign, star, and sub lords are computed.
  * *Setup*: UTC 2026-06-20 12:00:00, Lat 13.0, Lon 80.0, Lahiri ayanamsa.
  * *Assertions*: Verify `kp.cusps.len() == 12` and all points have valid, non-fallback lords.
* **TC-KP-02: Natal Planet Mapping**
  * *Objective*: Verify mapping of natal planets (e.g. Sun, Moon) to KP points.
  * *Setup*: standard chart with known planet longitudes.
  * *Assertions*: Verify planet longitudes and computed lords (sign, star, sub) match expected astronomical boundaries.
* **TC-DSH-01: Vimshottari Dasha Sequence**
  * *Objective*: Verify sequence of Mahadashas starting from Moon's nakshatra.
  * *Setup*: Moon at 0.0° (ruled by Ketu).
  * *Assertions*: Verify the first Mahadasha is Ketu, followed by Venus, Sun, Moon, Mars, Rahu, Jupiter, Saturn, and Mercury.
* **TC-DSH-02: Yogini Dasha Sequence**
  * *Objective*: Verify sequence of Yogini periods starting from Moon's nakshatra.
  * *Setup*: Moon at 0.0° (Mangala).
  * *Assertions*: Verify the sequence Mangala (Moon), Pingala (Sun), Dhanya (Jupiter), Bhramari (Mars), Bhadrika (Mercury), Ulka (Saturn), Siddha (Venus), and Sankata (Rahu).

### Tier 2: Boundary & Corner Cases
* **TC-KP-03: Zodiac Sign Boundary Transitions**
  * *Objective*: Verify sign lord transitions exactly on boundaries (29.999° vs 30.001°).
  * *Setup*: Planet at 29.999° (Aries, ruled by Mars) vs 30.001° (Taurus, ruled by Venus).
  * *Assertions*: `kp.planets[0].sign_lord == Mars` and `kp.planets[1].sign_lord == Venus`.
* **TC-KP-04: Nakshatra Star Lord Transitions**
  * *Objective*: Verify star lord transition on boundaries (13.333° vs 13.334°).
  * *Setup*: Planet at 13.33° (Ashwini, Ketu) vs 13.34° (Bharani, Venus).
  * *Assertions*: `kp.planets[0].star_lord == Ketu` and `kp.planets[1].star_lord == Venus`.
* **TC-KP-05: Polar Coordinate Handling**
  * *Objective*: Verify behavior under extreme high/low latitude where Placidus cusps fail.
  * *Setup*: Lat 80.0° N (Svalbard).
  * *Assertions*: Ensure the engine returns a controlled error or gracefully handles it, rather than panicking.
* **TC-DSH-03: Extreme Moon Longitude Boundaries**
  * *Objective*: Verify dasha calculation when Moon is exactly at 0.0° and 359.999° (Aries 0° vs Pisces 30°).
  * *Setup*: Moon at 359.999° (Revati end, ruled by Mercury).
  * *Assertions*: First Mahadasha should be Mercury with virtually 0 remaining duration.

### Tier 3: Cross-Feature Integration
* **TC-INT-01: Dasha Lord KP Significators**
  * *Objective*: Verify active Vimshottari Mahadasha lord's astronomical properties match its KP points.
  * *Setup*: Standard birth chart. Compute dasha timeline, extract current lord, calculate KP.
  * *Assertions*: Assert that the dasha lord's KP sign, star, and sub-lords match the calculated KP planet values.
* **TC-INT-02: Gochara Transit KP Cusp Mapping**
  * *Objective*: Verify transit planets can be mapped onto KP natal house cusps.
  * *Setup*: Natal chart + transit date/time.
  * *Assertions*: Map transit planets to KP cusps to verify their temporary house sign, star, and sub significations.

### Tier 4: Real-World Workloads & Facade E2E
* **TC-E2E-01: Vedic Analysis E2E Service Call**
  * *Objective*: Execute full `eon_service::services::vedic::analyze` facade pipeline.
  * *Setup*: `VedicAnalysisInput` for standard birth time (e.g. Seoul 1990-05-24 08:30).
  * *Assertions*: Validate that `VedicAnalysisOutput` has populated `kp_analysis` (12 cusps, 8 planets) and reports contain chronological Vimshottari & Yogini timelines with no temporal gaps.
* **TC-E2E-02: Career and Wealth Cusp Significators Audit**
  * *Objective*: End-to-end audit of career (10th) and wealth (2nd, 11th) KP cusps and active dashas.
  * *Setup*: Retrieve the sign/star/sub lords of the 2nd, 10th, and 11th house cusps from KP, and verify if their active periods correspond to high-earning life phases.

---

## 5. Verification Method

To verify the observations and conclusions independently:

1. **Inspect APIs and Signatures**:
   * View `crates/eon-vedic/src/analysis/kp.rs` using `view_file` to confirm `KpAnalysis::calculate` signature.
   * View `crates/eon-vedic/src/prediction/dasha.rs` to inspect prediction-level Vimshottari/Yogini dasha calculation and year type support.
   * View `crates/eon-vedic/src/analysis/dasha.rs` to inspect report-level dasha calculations.

2. **Verify Test Failure**:
   * Execute the following command in the terminal to observe the compilation errors of `PlanetStrength` in `compatibility_shadbala_kp_dasha.rs`:
     ```bash
     cargo check --tests --package eon-vedic
     ```
   * Execute this command to check standard tests (which pass successfully):
     ```bash
     cargo test --test basic
     ```
