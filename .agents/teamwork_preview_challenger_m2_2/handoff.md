# Handoff Report

## 1. Observation
- `crates/eon-vedic/src/analysis/matching.rs`:
  - Line 34: `let male_moon = male.planets.iter().find(|p| p.planet == VedicPlanet::Moon).cloned().unwrap();`
  - Line 35: `let female_moon = female.planets.iter().find(|p| p.planet == VedicPlanet::Moon).cloned().unwrap();`
  - Line 383: `let moon = chart.planets.iter().find(|p| p.planet == VedicPlanet::Moon).unwrap();`
  These lines use `.unwrap()` on searches for the Moon planet, panicking if it is not present in the `planets` array.
- `crates/eon-vedic/src/analysis/matching.rs` lines 205-206:
  ```rust
  let dist_f_to_m = ((male_nak as i16 - female_nak as i16 + 27) % 9) + 1;
  let dist_m_to_f = ((female_nak as i16 - male_nak as i16 + 27) % 9) + 1;
  ```
  With `male_nak = 100` and `female_nak = 200`, `dist_f_to_m` evaluates to `0`. Since `0` is not in `[3, 5, 7]`, it returns `3.0` points for Tara compatibility.
- Polar coordinates in `KpAnalysis::calculate` (lines 37-42):
  ```rust
  let (cusps, _) = engine
      .get_houses(time, latitude, longitude, b'P' as i32)
      .or_else(|_| engine.get_houses(time, latitude, longitude, b'K' as i32))
      ...
  ```
  This cascading fallback mechanism successfully computes houses for latitude 90.0 and -90.0, avoiding crashes.
- A mathematical minimum score check was added to `crates/eon-vedic/tests/compatibility_shadbala_kp_dasha.rs` using Male (Capricorn/Nak 24) and Female (Leo/Nak 13).
  Command: `cargo test --test compatibility_shadbala_kp_dasha`
  Result: `test result: ok. 55 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s`

## 2. Logic Chain
- **Step 1**: The presence of raw `.unwrap()` calls without validation or conditional mapping in `MatchingEngine::calculate_compatibility` and `check_mangal_dosha` means any chart constructed without `VedicPlanet::Moon` will trigger a Rust thread panic.
- **Step 2**: The absence of bounds checks or error propagation on `nakshatra` in `calculate_tara` means that when `dist_f_to_m` is computed using i16 mod arithmetic, invalid inputs (such as 100 and 200) can evaluate to values outside of the range 1..9, including 0. Since 0 is not in the hardcoded `inauspicious` array `[3, 5, 7]`, the method returns `3.0` (auspicious) for invalid nakshatras.
- **Step 3**: By testing polar coordinates (90.0 / -90.0), we observed that the KP house cusp calculation resolves successfully due to cascading fallback logic from Placidus to Koch, Porphyry, and Equal houses.
- **Step 4**: By configuring Male Moon to Capricorn (caste 2, lord Saturn, Nak 24/Rakshasa/Adi) and Female Moon to Leo (caste 3, lord Sun, Nak 13/Deva/Adi), we successfully targeted the minimum possible score of `2.5` points. The test case confirms that the Ashtakoota compatibility implementation resolves this worst-case configuration without errors.

## 3. Caveats
- We did not verify the correctness of the raw Swiss Ephemeris FFI bindings (`eon-astro`), as it was out of scope.
- We assumed the user verification/main agent will decide whether they want to fix the `unwrap()` panics in `src/` (since our constraints specifically forbid us from making any changes to implementation files).

## 4. Conclusion
- The Milestone M2 Ashtakoota Compatibility engine is robust against crash-triggering polar coordinates, but contains critical vulnerability pathways (hard panics on missing Moon inputs) and logical flaws (out-of-bounds inputs yielding highly auspicious scores). The mathematical minimum score of 2.5 points was empirically confirmed.

## 5. Verification Method
- Run `cargo test --test compatibility_shadbala_kp_dasha` to execute the full suite including polar tests, missing Moon panic tests, and the minimum score (2.5) test case.
