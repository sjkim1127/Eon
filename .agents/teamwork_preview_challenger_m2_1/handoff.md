# Handoff Report — Milestone M2 (Ashtakoota Compatibility Verification)

This report details the empirical findings from stress-testing the Ashtakoota compatibility implementation.

## 1. Observation

- **Observed File Path**: `crates/eon-vedic/src/analysis/matching.rs`
- **Line 34-35**:
  ```rust
  let male_moon = male.planets.iter().find(|p| p.planet == VedicPlanet::Moon).cloned().unwrap();
  let female_moon = female.planets.iter().find(|p| p.planet == VedicPlanet::Moon).cloned().unwrap();
  ```
- **Line 383**:
  ```rust
  let moon = chart.planets.iter().find(|p| p.planet == VedicPlanet::Moon).unwrap();
  ```
- **Observed File Path**: `crates/eon-vedic/src/analysis/kp.rs`
- **Line 127**:
  ```rust
  lords[((nakshatra - 1) % 9) as usize]
  ```
- **Test execution command**:
  `cargo test --package eon-vedic --test compatibility_shadbala_kp_dasha -- --nocapture`
- **Test results (panic behavior)**:
  ```
  thread 'test_stress_ashtakoota_missing_moon_mangal_dosha_panic' (3086732) panicked at crates/eon-vedic/src/analysis/matching.rs:35:98:
  called `Option::unwrap()` on a `None` value
  
  thread 'test_stress_ashtakoota_missing_moon_panic' (3086733) panicked at crates/eon-vedic/src/analysis/matching.rs:34:94:
  called `Option::unwrap()` on a `None` value
  ```
- **Test results (math wraparound)**:
  `Earned Tara points: 3` (returned for out-of-bounds `male_nak = 0`, `female_nak = 28` inputs).

---

## 2. Logic Chain

1. **Panic Vector 1**: `calculate_compatibility` queries the input `VedicChart` for `VedicPlanet::Moon` using `.find()` and immediately calls `.unwrap()` (Observation 1). If the Moon is missing from the list of planets in the chart (e.g. due to filtered inputs or custom testing), this `.unwrap()` will panic, crashing the thread (Observation 6).
2. **Panic Vector 2**: The Mangal Dosha check (`check_mangal_dosha`) queries for `VedicPlanet::Moon` and calls `.unwrap()` if Mars is present in the chart (Observation 2). If Mars is present but the Moon is missing, this results in an immediate panic (Observation 5).
3. **Underflow Risk**: `get_nakshatra_lord` and `get_sub_lord` perform `(nakshatra - 1)` (Observation 3). If `nakshatra == 0`, this underflows in Rust. In debug mode, this panics instantly due to overflow checks. In release mode, it wraps around to `255`, causing invalid array indexing and silent erroneous calculations.
4. **Mathematical Wraparound**: In `calculate_tara`, the distance modulo 9 calculation can return `0` when out-of-bounds inputs like `male_nak = 0, female_nak = 28` are supplied. Because `0` is not in the `[3, 5, 7]` inauspicious list, it incorrectly flags the combination as auspicious and grants a perfect score of `3.0` (Observation 7).

---

## 3. Caveats

- We assumed that `eon_astro` (Swiss Ephemeris wrapper) is correct and out of scope, so we did not review the FFI bindings.
- We did not implement fixes ourselves as we are in a review-only/challenger capacity and must not modify implementation code.

---

## 4. Conclusion

The Milestone M2 implementation of Ashtakoota compatibility contains multiple robustness risks: two panic vectors, an underflow risk, a mathematical wraparound anomaly, and weak test suite assertions. All have been empirically verified and catalogued in `challenge.md`.

---

## 5. Verification Method

To verify these findings:
1. View the report at `.agents/teamwork_preview_challenger_m2_1/challenge.md`.
2. Run the test suite:
   ```bash
   cargo test --package eon-vedic --test compatibility_shadbala_kp_dasha
   ```
3. Inspect `crates/eon-vedic/tests/compatibility_shadbala_kp_dasha.rs` to review the stress tests added (`test_stress_*`).
