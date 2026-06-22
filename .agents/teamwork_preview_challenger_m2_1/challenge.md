# Adversarial Review Challenge Report — Milestone M2 (Ashtakoota Compatibility)

## Challenge Summary

**Overall risk assessment**: MEDIUM

Empirical stress testing of the Milestone M2 implementation (`MatchingEngine::calculate_compatibility` and associated Vedic engines) has identified multiple safety and mathematical robustness issues. Specifically:
- Two critical panic vectors exist due to unsafe `.unwrap()` calls on missing Moon inputs.
- An underflow vulnerability exists when custom/mock Nakshatras have a value of 0, which triggers panic in debug mode and wraps around in release mode.
- A mathematical wraparound anomaly in Tara Koota calculation yields an incorrect perfect score for out-of-bounds inputs instead of an error or zero.
- The test suite has weak assertions for extreme coordinate fallback checks.

---

## Challenges

### [High] Challenge 1: Unsafe Unwrap on Missing Moon Planet

- **Assumption challenged**: The compatibility engine assumes both male and female charts always contain the `VedicPlanet::Moon` planet.
- **Attack scenario**: If a caller passes a chart that has filtered planets or is a mock chart without the Moon:
  ```rust
  let male_moon = male.planets.iter().find(|p| p.planet == VedicPlanet::Moon).cloned().unwrap();
  ```
  This immediately panics on `.unwrap()`.
- **Blast radius**: Crashes the thread. In a Dioxus WASM client context, this can halt the entire web worker or UI thread, freezing the frontend.
- **Mitigation**: Return a `Result<CompatibilityReport, String>` or use `std::option::Option` to safely check for the Moon, or validate inputs at the API gateway layer before invoking `MatchingEngine`.

### [High] Challenge 2: Unsafe Unwrap in Mangal Dosha Check

- **Assumption challenged**: The Mangal Dosha check assumes that if Mars is present, the Moon must also be present in the chart.
- **Attack scenario**: In `check_mangal_dosha`:
  ```rust
  let mars = chart.planets.iter().find(|p| p.planet == VedicPlanet::Mars);
  if let Some(m) = mars {
      ...
      let moon = chart.planets.iter().find(|p| p.planet == VedicPlanet::Moon).unwrap();
  ```
  If Mars is present but Moon is missing, it will panic on `.unwrap()`.
- **Blast radius**: Thread crash / panic.
- **Mitigation**: Safely handle the Moon lookup:
  ```rust
  if let Some(moon) = chart.planets.iter().find(|p| p.planet == VedicPlanet::Moon) {
      // calculation
  } else {
      false
  }
  ```

### [Medium] Challenge 3: Underflow Vulnerability on Nakshatra Index

- **Assumption challenged**: Nakshatra index is always a valid index (1..=27).
- **Attack scenario**: If `nakshatra` is 0 (due to mock data or calculation edge case), `get_nakshatra_lord` and `get_sub_lord` do:
  ```rust
  lords[((nakshatra - 1) % 9) as usize]
  ```
  In Rust, `0 - 1` on `u8` underflows.
  - In **Debug mode**: Triggers an arithmetic overflow panic.
  - In **Release mode**: Wraps to 255. `255 % 9` is 3, indexing `lords[3]`, returning a silent incorrect value.
- **Blast radius**: Debug build crashes; Release builds produce incorrect/silent astrological output.
- **Mitigation**: Validate/clamp `nakshatra` to `1..=27` or wrap safely:
  ```rust
  if nakshatra == 0 || nakshatra > 27 { return Err(...); }
  ```

### [Medium] Challenge 4: Mathematical Wraparound Anomaly in Tara Koota

- **Assumption challenged**: Modulo arithmetic on difference of Nakshatras produces a distance between 1 and 9.
- **Attack scenario**: If custom/out-of-bounds nakshatras are present (`male_nak = 0`, `female_nak = 28`), the formula:
  ```rust
  let dist_f_to_m = ((male_nak as i16 - female_nak as i16 + 27) % 9) + 1;
  ```
  yields `0` instead of a number in `1..=9` due to negative modulo in Rust. Since `0` is not in the inauspicious list `[3, 5, 7]`, it defaults to being marked as auspicious (`m_ok = true`), yielding a perfect `3.0` Tara score for invalid input.
- **Blast radius**: Silent mathematical error producing perfect compatibility scores for out-of-bounds data.
- **Mitigation**: Clamp or normalize Nakshatra inputs to `1..=27` using `((nak - 1) % 27) + 1` before performing Koota math.

### [Low] Challenge 5: Weak Assertions for extreme coordinate fallback checks

- **Assumption challenged**: The test suite validates Placidus failure handling at poles.
- **Attack scenario**: The test `test_kp_extreme_coordinates_cusps` asserts:
  ```rust
  assert!(kp.is_ok() || kp.is_err());
  ```
  This is a tautology (always passes) and does not verify whether the system successfully fell back to a valid house system (Koch/Porphyry/Equal) or returned the expected error.
- **Blast radius**: Weak test validation hiding potential fallback failures.
- **Mitigation**: Assert concrete outcomes: expect `kp.is_ok()` (successful fallback) or check that a clean error structure is returned.

---

## Stress Test Results

| Scenario | Expected Behavior | Actual Behavior | Pass/Fail |
|---|---|---|---|
| Chart without Moon passed to `calculate_compatibility` | Controlled error or safe default | Panics: `called Option::unwrap() on a None value` | **FAIL** |
| Chart with Mars but without Moon passed to `calculate_compatibility` | Controlled error or safe default | Panics: `called Option::unwrap() on a None value` in `check_mangal_dosha` | **FAIL** |
| Out-of-bounds Nakshatras (`0`, `28`) passed to `calculate_compatibility` | Zero points / Error | Runs, but incorrectly calculates a perfect `3.0` Tara score | **FAIL** |
| Out-of-bounds Rasis (`0`, `13`) passed to `calculate_compatibility` | Graceful fallback or zero points | Gracefully handles via `_` matches but yields arbitrary points | **PASS** (Graceful, though points are arbitrary) |
| Extreme coordinates (90.0, -90.0 latitude) passed to `VedicChartCalculator` | Graceful calculation (WholeSign) | Returns `Ok` | **PASS** |

---

## Unchallenged Areas

- **Swiss Ephemeris bindings (`eon-astro`)** — out of scope for the Vedic module's compatibility engine.
- **Shadbala strength factors (exaltation, dig bala, kala bala)** — briefly verified, but detailed astronomical oracles were out of scope.
