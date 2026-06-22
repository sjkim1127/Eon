# E2E Exploration Handoff Report: Compatibility & Shadbala

This report synthesizes the exploration of the `eon-vedic` and `eon-service` codebases to understand public APIs, test architectures, and design integration tests for Ashtakoota compatibility and Shadbala strength features.

---

## 1. Observation

Direct observations from code files in the workspace:

### 1-1. Ashtakoota Compatibility Public API
- **Location**: `crates/eon-vedic/src/analysis/matching.rs`
- **Method Signature**:
  ```rust
  pub struct MatchingEngine;

  impl MatchingEngine {
      pub fn calculate_compatibility(
          male: &VedicChart,
          female: &VedicChart,
      ) -> CompatibilityReport { ... }
  }
  ```
- **Public Structs**:
  ```rust
  #[derive(Debug, Clone, Serialize, Deserialize)]
  #[serde(rename_all = "camelCase")]
  pub struct CompatibilityReport {
      pub total_score: f64,              // out of 36
      pub is_compatible: bool,           // total >= 18 and no critical Nadi/Bhakoot dosha
      pub kootas: Vec<KootaScore>,
      pub male_mangal_dosha: bool,
      pub female_mangal_dosha: bool,
      pub mangal_dosha_cancelled: bool, // true if both have Mangal Dosha (Dosha Samya)
      pub explanation: String,
  }

  #[derive(Debug, Clone, Serialize, Deserialize)]
  #[serde(rename_all = "camelCase")]
  pub struct KootaScore {
      pub name: String,
      pub max_points: f64,
      pub earned_points: f64,
      pub description: String,
  }
  ```
- **Service Façade API**: Located in `crates/eon-service/src/services/vedic.rs:92`:
  ```rust
  pub fn analyze_compatibility(
      input: crate::dto::VedicCompatibilityInput,
  ) -> Result<crate::dto::VedicCompatibilityOutput, ServiceError> { ... }
  ```
  Where:
  - `VedicCompatibilityInput` (in `crates/eon-service/src/dto.rs:175`) contains `male: AnalysisInput` and `female: AnalysisInput`.
  - `AnalysisInput` contains year, month, day, hour, minute, latitude (`lat`), longitude (`lon`), and `timezone: String`.

---

### 1-2. Shadbala Strength Public API
- **Location**: `crates/eon-vedic/src/analysis/strength.rs`
- **Method Signature**:
  ```rust
  pub struct StrengthEngine;

  impl StrengthEngine {
      pub fn calculate(pos: &VedicPosition, chart: &crate::chart::VedicChart) -> PlanetStrength { ... }
  }
  ```
- **Public Structs**:
  ```rust
  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct PlanetStrength {
      pub planet: VedicPlanet,
      pub exaltation_score: f64,   // Uchcha Bala (0.0 ~ 60.0)
      pub directional_score: f64,  // Dig Bala (0.0 ~ 60.0)
      pub chesta_score: f64,       // Chesta Bala (0.0 ~ 60.0)
      pub naisargika_score: f64,   // Naisargika Bala (0.0 ~ 60.0)
      pub kala_score: f64,         // Kala Bala (0.0 ~ 60.0)
      pub drik_score: f64,         // Drik Bala (aspect strength)
      pub paksha_score: f64,       // Paksha Bala (Moon phase strength)
      pub ayana_score: f64,        // Ayana Bala (declination strength)
      pub saptavargaja_score: f64, // Saptavargaja Bala (0.0 ~ 60.0)
      
      // Additional Sthana Bala components (BPHS)
      pub kendra_bala: f64,
      pub drekkana_bala: f64,
      pub ojayugmarasyamsa_bala: f64,
      
      pub yuddha_bala: f64,        // Planetary war adjustment
      pub ishta_phala: f64,        // Auspiciousness (0.0 ~ 60.0)
      pub kashta_phala: f64,       // Inauspiciousness (0.0 ~ 60.0)
      pub total_score: f64,
      pub status: String,          // "Exalted", "Debilitated", "Strong", "Weak", "Neutral"
  }
  ```

---

### 1-3. Test Chart & Input Data Initialization
Existing integration tests under `crates/eon-vedic/tests/` initialize charts using the following patterns:
1. **Direct Calculation (e.g., `basic.rs` & `common/mod.rs`)**:
   ```rust
   let calc = VedicChartCalculator::new(); // or default()
   let dt = Utc.with_ymd_and_hms(year, month, day, hour, minute, 0).unwrap();
   let chart = calc.calculate(dt, lat, lon).unwrap();
   ```
2. **Calculation with Custom Configuration (e.g., `varga_nakshatra_report.rs`)**:
   ```rust
   let config = VedicConfig {
       ayanamsa: AyanamsaSystem::Lahiri,
       node_calc: NodeCalculation::MeanNode,
       house_system: HouseSystem::WholeSign,
       year_type: VedicYearType::Gregorian,
   };
   let calculator = VedicChartCalculator::with_config(config);
   let chart = calculator.calculate(dt, lat, lon).unwrap();
   ```
3. **Fixture-based JSON Snapshot Verification (e.g., `shadbala_oracle_verify.rs`)**:
   - Reads `tests/fixtures/shadbala_oracle.json` or `position_oracle.json`.
   - Parses the JSON fields (year, month, day, hour, lat, lon) and runs them through `common::create_test_chart(...)`.
   - Asserts actual vs. expected properties (like `uchcha_bala`, `dig_bala`) using a custom tolerance comparison helper:
     ```rust
     pub fn assert_approx_eq(actual: f64, expected: f64, epsilon: f64, msg: &str) { ... }
     ```

---

## 2. Logic Chain

1. **API Invocations**:
   - Compatibility requires both a male and a female `VedicChart` generated by `VedicChartCalculator`.
   - Shadbala is computed on a per-planet basis by fetching the planet's `VedicPosition` from a computed `VedicChart` (e.g., `chart.planets.iter().find(|p| p.planet == planet_enum)`) and supplying it alongside the overall `VedicChart` (for relationship and aspect context).
   
2. **Missing Coverage**:
   - Currently, there are **no integration tests** written specifically for `MatchingEngine::calculate_compatibility` in `crates/eon-vedic/tests/`.
   - Shadbala is tested against standard fixtures in `shadbala_oracle_verify.rs`, but does not cover extreme boundaries (such as polar births or exact station speeds).

3. **Test Infrastructure Alignment**:
   - Based on `TEST_INFRA.md` at the project root, a 4-tier E2E testing framework is mandated. Integration test development should cover these features across Tiers 1-4.

---

## 3. Caveats

- **Timezone conversions**: The low-level `VedicChartCalculator` expects standard UTC time (`DateTime<Utc>`). The high-level facade converts local time to UTC using `prepare_birth_context`.
- **Rahu/Ketu Shadbala**: Classical Shadbala is only defined for the 7 standard planets (Sun to Saturn). For Rahu and Ketu, the engine uses default fallback values (30.0 for some factors, 0.0 for others) because they are shadow nodes rather than physical bodies.

---

## 4. Conclusion

We propose the following integration test design to be implemented in `crates/eon-vedic/tests/compatibility_shadbala_kp_dasha.rs`:

### 4-1. Ashtakoota Compatibility Test Cases
#### Tier 1: Feature Coverage (>= 5 cases)
- **C-T1-1 (Auspicious Compatibility)**: Pair with high compatibility (>28 points, e.g., Male Moon in Pushya/Cancer, Female Moon in Rohini/Taurus). Verify `total_score` and `is_compatible` is `true`.
- **C-T1-2 (Nadi Dosha)**: Couple with Moon in Nakshatras belonging to the same Nadi (e.g., Ashwini and Ardra - both Adi Nadi). Verify `nadi_score` is `0.0` and `is_compatible` is `false`.
- **C-T1-3 (Bhakoot Dosha)**: Moon signs in inauspicious relative positions (e.g., 2/12, 5/9, 6/8). Verify `bhakoot_score` is `0.0`.
- **C-T1-4 (Mangal Dosha Cancellation)**: Both charts have Mars in doshic houses (1, 2, 4, 7, 8, 12). Verify `mangal_dosha_cancelled` is `true`.
- **C-T1-5 (No Mangal Dosha)**: Both charts have Mars in non-doshic houses (e.g., 3rd/9th). Verify both `mangal_dosha` values are `false`.

#### Tier 2: Boundary & Corner Cases (>= 5 cases)
- **C-T2-1 (Identical Birth Chart Compatibility)**: Male and Female charts calculated with identical birth details. Verify that:
  - `nadi_score` = `0.0` (Nadi Dosha triggers for identical nakshatras)
  - `bhakoot_score` = `7.0` (same Rasi)
  - `varna_score` = `1.0`, `vashya_score` = `2.0`, `tara_score` = `3.0`, `yoni_score` = `4.0`, `graha_maitri_score` = `5.0`, `gana_score` = `6.0`
- **C-T2-2 (Extreme coordinates / Polar region compatibility)**: Latitudes above Arctic Circle (e.g., 75.0° N). Verify the chart computes and compatibility returns safely.
- **C-T2-3 (Historical Boundary date compatibility)**: Birth dates set to the year 1500. Verify the planetary ephemeris calculation handles boundary years.
- **C-T2-4 (Nakshatra boundary transition)**: Moon longitude placed at exactly 13°20' (boundary between Nakshatras 1 and 2). Verify stable categorization.
- **C-T2-5 (Zero coordinates)**: Latitude 0.0, Longitude 0.0 (Null Island). Verify stable execution.

---

### 4-2. Shadbala Strength Test Cases
#### Tier 1: Feature Coverage (>= 5 cases)
- **S-T1-1 (Uchcha Bala / Exaltation)**: Sun near 10° Aries. Verify `exaltation_score` is close to 60.0.
- **S-T1-2 (Neecha Bala / Debilitation)**: Sun near 10° Libra. Verify `exaltation_score` is close to 0.0.
- **S-T1-3 (Dig Bala / Directional)**: Jupiter in the 1st house (Ascendant). Verify `directional_score` is 60.0.
- **S-T1-4 (Yuddha Bala / Planetary War)**: Mars and Venus within 1° longitude. Verify `yuddha_bala` is non-zero (one planet gains, the other loses).
- **S-T1-5 (Kala Bala / Time Strength)**: Day-strong planets (Sun, Jupiter, Venus) in a day birth chart vs. night birth chart. Verify higher Kala scores for day births.

#### Tier 2: Boundary & Corner Cases (>= 5 cases)
- **S-T2-1 (Extreme Polar latitude Dig Bala)**: Birth at 80° N where house cusps are highly compressed. Verify that `house_index` calculations do not crash and Dig Bala computes.
- **S-T2-2 (Stationary planet speed)**: Speed is exactly 0.0. Verify `is_retrograde` is false and motion state score is resolved correctly in `calculate_chesta_bala`.
- **S-T2-3 (Cap on Ayana Bala)**: Planet at maximum declination (+23.45°). Verify `ayana_score` is capped at exactly 60.0.
- **S-T2-4 (Exact Combustion edge)**: Venus at exactly 10.0° difference from the Sun. Verify combustion boolean boundary.
- **S-T2-5 (0.0° Aries starting longitude)**: Sun at exactly 0.0° longitude. Verify boundary stability.

---

### 4-3. Tier 3: Cross-Feature Combination Cases (>= 4 cases)
- **X-T3-1 (Moon Nakshatra dual mapping)**: Verify that the Moon's Nakshatra determines both the starting Vimshottari Dasha lord and the Ashtakoota compatibility factors correctly.
- **X-T3-2 (House placement sync)**: Verify that planet house indices (which determine Dig Bala and Kendra Bala in Shadbala) are fully synchronized with the calculated KP house cusps.
- **X-T3-3 (Mangal Dosha and house system)**: Verify how choosing Sripati vs. Whole Sign house systems changes the house placement of Mars, altering the Mangal Dosha result and consequently modifying the compatibility outcome.
- **X-T3-4 (Dasha lord Shadbala check)**: Confirm that the active Dasha lord's Shadbala status (e.g., "Strong", "Weak") aligns with the corresponding planet strength in the natal reading.

---

### 4-4. Tier 4: Real-World Workload Cases (>= 5 cases)
- **W-T4-1 (Historical Profile Reading)**: Amitabh Bachchan (1942-10-11). Validate all 7 planet Shadbala scores against historical standards.
- **W-T4-2 (End-to-End Compatibility)**: Run compatibility analysis between two realistic profiles and verify overall performance under 100ms.
- **W-T4-3 (Polar Region Reading)**: Birth at Tromsø, Norway (69.649° N). Verify full natal report generation.
- **W-T4-4 (Double War Scenario)**: A chart containing two separate planetary wars (e.g., Mars-Venus and Mercury-Saturn). Verify Yuddha Bala allocates correctly to all four.
- **W-T4-5 (120-Year Lifecycle Profile)**: Compute chart, Shadbala scores, and full 120-year Vimshottari dasha hierarchy to test system-wide memory stability.

---

## 5. Verification Method

To verify these observations and proposed cases:
1. **Source Code Inspection**:
   - Check `crates/eon-vedic/src/analysis/matching.rs` to inspect the Ashtakoota score logic.
   - Check `crates/eon-vedic/src/analysis/strength.rs` to inspect the 6-factor Shadbala calculations.
2. **Execute Existing Tests**:
   Run the standard test command to ensure the current suite passes without issues:
   ```bash
   cargo test --package eon-vedic
   ```
3. **Asserting Proposed Cases**:
   Proposed cases will be implemented in the new test suite `crates/eon-vedic/tests/compatibility_shadbala_kp_dasha.rs` during the implementation phase.
