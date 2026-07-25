# ⚔️ Challenger Handoff Report: Milestone 2 (R2) Stress Testing

**Target Features**: Wolwun GanZi alignment, Dynamic Precedence Hierarchy in `combined_relations`, and Expanded Transformations in `power.rs`.
**Target Files**: `periodic_luck.rs`, `dynamic_luck.rs`, `transformations.rs`, `power.rs`.
**Test Harness**: `crates/eon-saju/tests/milestone2_stress_tests.rs`.

---

## 1. Observation

### 1-1. Baseline Verification
Ran workspace checks and full test suite:
```bash
cargo check --workspace
cargo test --workspace
```
**Results**:
- `cargo check --workspace`: Passed (Finished `dev` profile in 0.52s).
- `cargo test --workspace`: Passed (All 55 core unit tests + all 10 integration oracle tests passed).

### 1-2. Stress Test Execution Results
Executed empirical stress test suite `crates/eon-saju/tests/milestone2_stress_tests.rs`:
```bash
cargo test --test milestone2_stress_tests -- --nocapture
```

**Test Results Summary**:
- `test_wolwun_lichun_boundary_2026`: **PASSED** (1 min before LiChun = `己丑`, 1 min after = `庚寅`).
- `test_wolwun_jingzhi_boundary_2026`: **PASSED** (1 min before JingZhi = `庚寅`, 1 min after = `辛卯`).
- `test_wolwun_yushu_minor_term_boundary_2026`: **PASSED** (1 min before/after minor term YuShu remain `庚寅`).
- `test_triple_alliance_suppresses_clash_and_six_combination`: **PASSED** (申-子-辰 suppresses 子-午 clash and 辰-酉 six combination).
- `test_seasonal_alliance_suppresses_clash_and_six_combination`: **PASSED** (寅-卯-辰 suppresses 卯-酉 clash and 辰-酉 six combination).
- `test_expanded_transformations_elemental_power`: **PASSED** (Transformations under `apply_correction: false` correctly update Water power to >60%).
- `test_wolwun_early_january_saju_year`: **FAILED (BUG #1)**.
  ```text
  thread 'test_wolwun_early_january_saju_year' panicked at 'assertion `left == right` failed:
  Jan 2, 2026 month stem must be 戊 (year 2025 乙巳), NOT 庚 (year 2026 丙午)
    left: Geng
   right: Wu'
  ```
- `test_expanded_power_correction_branch_integrity`: **FAILED (BUG #2)**.
  ```text
  thread 'test_expanded_power_correction_branch_integrity' panicked at 'Chart without water stems/branches had Water percentage 53.1%! (Corruption bug in power.rs)'
  ```

---

## 2. Logic Chain

### Bug #1: Wolwun Saju Year Miscalculation in Early January Before XiaoHan

**Observation 1.1**: In `crates/eon-saju/src/analysis/periodic_luck.rs` lines 193-213:
```rust
let saju_year = if term_24_idx == 22 || term_24_idx == 23 {
    dt_year - 1
} else if term_24_idx == 0 || term_24_idx == 1 {
    let year_start = chrono::NaiveDate::from_ymd_opt(dt_year, 1, 1)...
    if dt < lichun { dt_year - 1 } else { dt_year }
} else {
    dt_year
};
```
**Observation 1.2**: On `2026-01-02 12:00:00 UTC`, solar longitude is ~281.0°, which corresponds to 24-solar term index `21` (DongZhi, 동지 - which began on Dec 21 of the previous year).
**Logic Step**:
1. When `term_24_idx` is `21`, it fails `term_24_idx == 22 || 23` and `term_24_idx == 0 || 1`.
2. The code falls into `else { dt_year }`, setting `saju_year = 2026`.
3. However, `2026-01-02` is before LiChun 2026 (`2026-02-03 20:02:08 UTC`), so the Saju year MUST be `2025` (乙巳년).
4. Because `saju_year` is incorrectly evaluated as `2026` (丙午년), the 11th Saju month (子월) stem is calculated using 丙 (index 2) as `(6 + 10) % 10 = 6` (庚) -> `庚子`.
5. For the correct Saju year `2025` (乙巳년), the 11th Saju month stem is `(4 + 10) % 10 = 4` (戊) -> `戊子`.
6. Therefore, all dates from Jan 1 up to XiaoHan entry (around Jan 5/6) return the wrong monthly GanZi.

### Bug #2: Elemental Power Corruption in `calculate_expanded` under Default Options

**Observation 2.1**: In `crates/eon-saju/src/analysis/power.rs` lines 282-294:
```rust
for (orig_el, eff_el, weight, is_stem, polarity) in items {
    let actual_el = if !is_stem && options.apply_correction {
        apply_climate_correction(
            match orig_el {
                Element::Earth => EarthlyBranch::Wei,
                _ => EarthlyBranch::Zi,
            },
            month_branch,
        )
    } else {
        eff_el
    };
```
**Observation 2.2**: `items` contains 8 natal positions (4 stems, 4 branches) + up to 4 luck positions (major stem/branch, saewun stem/branch).
**Logic Step**:
1. When `options.apply_correction` is `true` (the default option in `AnalysisOptions::default()`), for every branch (`!is_stem`), the `if` block executes.
2. The `match orig_el` expression maps any non-Earth element (`Wood`, `Fire`, `Metal`, `Water`) to `EarthlyBranch::Zi`.
3. `apply_climate_correction(EarthlyBranch::Zi, month_branch)` evaluates `Zi.element()`, which returns `Element::Water`.
4. As a result, **EVERY non-Earth branch (Wood 寅/卯, Fire 巳/午, Metal 申/酉) across the entire 4 natal pillars, major luck, and saewun is forcibly converted into Water**!
5. In our stress test chart (`丙寅` year, `丁卯` month, `甲午` day, `辛酉` hour, `戊戌` major luck, `庚申` yearly luck) containing **0 Water stems and 0 Water branches**, Water score was calculated as **53.1% (8.5 / 16.0 total points)**.
6. Additionally, because `actual_el` is overridden inside the `if !is_stem && options.apply_correction` branch, `eff_el` (the transformed element from Triple/Seasonal Alliances) is completely ignored and lost whenever `apply_correction` is true.

---

## 3. Caveats

1. **Implementation Code Unmodified**: In compliance with reviewer constraints, no core source files (`periodic_luck.rs`, `power.rs`, `dynamic_luck.rs`, `transformations.rs`) were edited. Bug fixes must be performed by the implementation agent.
2. **Dynamic Precedence Conformance**: The dynamic precedence logic in `dynamic_luck.rs` (`alliance_branches` set suppression of `branch_clashes` and `six_combinations`) functioned perfectly as designed.

---

## 4. Conclusion

- **Wolwun Boundary Alignment**: Works correctly at exact 1-minute before/after major solar term entry boundaries (LiChun, JingZhi) and minor solar terms (YuShu). However, **Bug #1** causes a failure for dates between Jan 1 and XiaoHan entry (Jan 5/6) due to incomplete Saju year boundary logic.
- **Dynamic Precedence Hierarchy**: Fully verified. Completed Triple Alliances (삼합) and Seasonal Alliances (방합) correctly suppress lower-priority Branch Clashes (지충) and Six Combinations (육합) in `combined_relations`.
- **Expanded Transformations & Power**: **Critical Bug #2** in `power.rs` corrupts all non-Earth branches into Water when `apply_correction` is `true` (the default setting), yielding invalid elemental power scores.

---

## 5. Verification Method

To independently verify these findings:

1. Run the empirical stress test suite:
   ```bash
   cargo test --test milestone2_stress_tests -- --nocapture
   ```
2. Observe `test_wolwun_early_january_saju_year` panic output showing `left: Geng, right: Wu` for Jan 2, 2026.
3. Observe `test_expanded_power_correction_branch_integrity` panic output showing `Water percentage 53.1%` on a chart with zero Water elements.
