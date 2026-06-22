## Challenge Summary

**Overall risk assessment**: MEDIUM

- The core logic executes correctly and handles extreme polar coordinates robustly via a cascading fallback mechanism in the house system calculation.
- However, the `MatchingEngine` has a critical vulnerability that results in a hard thread panic (`unwrap()` on a `None` value) when compatibility checks are run on charts with missing Moon positions.
- In addition, there is no bounds validation on Nakshatra or Rasi values in the compatibility engine, which allows out-of-bounds or invalid inputs to return mathematically valid but astrologically incorrect (and sometimes overly positive) scores.

---

## Challenges

### [High] Challenge 1: Hard Panic on Missing Moon in Vedic Charts (`unwrap()` vulnerability)

- **Assumption challenged**: It is assumed that any `VedicChart` passed to `MatchingEngine::calculate_compatibility` or `check_mangal_dosha` always has a populated Moon position in its `planets` array.
- **Attack scenario**: If a chart is custom-built (e.g., from partial/incomplete user data, mock charts, or filtered API inputs) and does not contain `VedicPlanet::Moon`, the application will crash with a hard panic:
  ```rust
  let male_moon = male.planets.iter().find(|p| p.planet == VedicPlanet::Moon).cloned().unwrap();
  ```
  And similarly in `check_mangal_dosha`:
  ```rust
  let moon = chart.planets.iter().find(|p| p.planet == VedicPlanet::Moon).unwrap();
  ```
- **Blast radius**: High. In a single-threaded WebAssembly frontend (Dioxus Web), a thread panic can crash or freeze the entire user interface.
- **Mitigation**: Modify `calculate_compatibility` to return a `Result<CompatibilityReport, VedicError>` (or a safe default/error structure) instead of unwrapping. Replace `.unwrap()` calls with error propagation or safe defaults.

### [Medium] Challenge 2: Out-of-Bounds Nakshatras and Rasis Return Garbage Scores

- **Assumption challenged**: It is assumed that Nakshatra (1..=27) and Rasi (1..=12) values are always within their respective valid bounds before scoring.
- **Attack scenario**: If invalid nakshatras (e.g. 100 and 200) are passed into `calculate_compatibility`:
  - `calculate_tara` computes the distances as:
    ```rust
    let dist_f_to_m = ((100 - 200 + 27) % 9) + 1; // Evaluates to 0
    ```
  - Since `dist_f_to_m` is `0`, which is not in the inauspicious set `[3, 5, 7]`, it is treated as auspicious.
  - As a result, invalid nakshatras 100 and 200 return an excellent Tara score of `3.0` points instead of flagging an error or receiving `0.0`.
- **Blast radius**: Medium. Returns incorrect/faulty astrological scores (silent failure / garbage-in, garbage-out) without notifying the system or the user.
- **Mitigation**: Add validation logic in `VedicPosition` or at the entry point of the `MatchingEngine` to ensure `rasi` is between 1 and 12, and `nakshatra` is between 1 and 27.

### [Low] Challenge 3: Unreachable 0.0 Tara Score due to Modulo Constraints

- **Assumption challenged**: It is assumed that the Tara score can range from 0.0 to 3.0 points depending on compatibility.
- **Attack scenario**: Mathematically, the forward and backward relative distances modulo 9 will always sum to 9 (or 0). Thus, it is impossible for both `dist_f_to_m` and `dist_m_to_f` to simultaneously fall within the inauspicious set `{3, 5, 7}`.
- **Blast radius**: Low. The Tara score can never be `0.0` for any inputs (valid or invalid); its minimum value is always `1.5`.
- **Mitigation**: Document this mathematical behavior, or adjust the scoring system if the astrological intention was to allow 0.0 points under certain mutual Tara Dosha conditions.

---

## Stress Test Results

- **Missing Moon in Male Chart** → Panic caught cleanly via `catch_unwind` → Panicked as expected (`Option::unwrap()` on `None` value) → **PASS** (vulnerability confirmed)
- **Missing Moon in Female Chart** → Panic caught cleanly via `catch_unwind` → Panicked as expected (`Option::unwrap()` on `None` value) → **PASS** (vulnerability confirmed)
- **Custom Nakshatras (0 or 28)** → System should not crash and should resolve gracefully → Returns total score of 28.0 without crashing → **PASS** (crash-resilient, but correctness compromised)
- **Out-of-bounds Nakshatras (100 and 200)** → System should flag as invalid or return 0.0 → Returned 3.0 points for Tara → **FAIL** (vulnerability confirmed; incorrect positive score)
- **Polar Coordinates (lat = 90.0, -90.0)** → House cusp calculation should use fallbacks and succeed → North and South Pole both calculate successfully → **PASS**
- **Mathematical Minimum Score (Capricorn/Nak 24 vs Leo/Nak 13)** → Should yield exactly 2.5 points (minimum possible score) → Yielded exactly 2.5 points → **PASS**

---

## Unchallenged Areas

- **Swiss Ephemeris FFI bindings (`eon-astro`)** — Out of scope; assumed correct.
- **Detailed Shadbala factor calculations** — Checked for crash resilience, but the exact physics/astronomy formulas for strength were not challenged.
