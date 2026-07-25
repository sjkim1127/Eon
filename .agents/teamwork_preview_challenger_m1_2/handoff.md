# Empirical Adversarial Challenge Report — Milestone 1 (R1) Part 2

## Challenge Summary

**Overall risk assessment**: MEDIUM

Empirical stress testing of Milestone 1 (R1) implementations for **12-Unseong**, **Samjae**, **Gongmang**, and **Noble Spirit Markers** (`twelve_stages.rs`, `void.rs`, `shinsal.rs`, `spirit_markers.rs`) was conducted.

- **Verified & Passed**: 
  - Samjae calculation logic across all 12 birth year branches and 12 transit branches.
  - 12-Unseong Yin-stem option (`yin_stem_reverse: true` vs `false`).
  - Gongmang dissolution for direct 6-clash (충) and 6-combination (육합).
  - Unclashed/uncombined Noble Spirit Marker annulment (`(귀인공망)`).
- **Critical Failure Modes / Bugs Identified**:
  1. **False Positive Void Dissolution (`void.rs:164–175`)**: `check_void_dissolution` checks if any Triple or Seasonal Combination exists in the 4 pillars, but fails to check if the voided branch is a member of that combination. Unrelated void branches are incorrectly marked dissolved.
  2. **Position String Mismatch in Spirit Marker Relations (`spirit_markers.rs:719–730`)**: `m.position.hangul()` returns `"년주"`, `"월주"`, `"일주"`, `"시주"`, whereas `rel_analysis` returns `"년지"`, `"월지"`, `"일지"`, `"시주"`. Because `"년주" != "년지"`, `is_clashed` and `is_combined` always evaluate to `false`. This completely breaks Noble Spirit restoration (`(공망해충/해합 구원)`) and clash/combination modifiers across all spirit markers.

---

## 1. Observation

### Observation 1: Samjae Calculation (`shinsal.rs:413–471`)
- `calculate_samjae(year_branch, transit_year_branch)` correctly maps the 4 Samhap groups:
  - `Shen | Zi | Chen`: Entrance (`Yin`), Dwelling (`Mao`), Exit (`Chen`).
  - `Yin | Wu | Xu`: Entrance (`Shen`), Dwelling (`You`), Exit (`Xu`).
  - `Si | You | Chou`: Entrance (`Hai`), Dwelling (`Zi`), Exit (`Chou`).
  - `Hai | Mao | Wei`: Entrance (`Si`), Dwelling (`Wu`), Exit (`Wei`).
- Verifiable via `cargo test -p eon-saju --test milestone1_part2_stress_tests`.
- **Omission Note**: `YearlyLuck::calculate` in `periodic_luck.rs:56` calls `ShinsalAnalysis::calculate_for_luck`, but `calculate_for_luck` does not invoke `calculate_samjae`. Consequently, Samjae indicators are not included in annual luck summaries.

### Observation 2: Unrelated Combination Void Dissolution Bug (`void.rs:164–175`)
```rust
164:    if !crate::analysis::relationships::TripleCombination::check(&all_branches).is_empty() {
165:        return Some((
166:            true,
167:            "원국 삼합(三合) 성국으로 공망 해소 (공망해합)".to_string(),
168:        ));
169:    }
170:    if !crate::analysis::relationships::SeasonalCombination::check(&all_branches).is_empty() {
171:        return Some((
172:            true,
173:            "원국 방합(方合) 성국으로 공망 해소 (공망해합)".to_string(),
174:        ));
175:    }
```
- In `check_void_dissolution`, `TripleCombination::check(&all_branches)` and `SeasonalCombination::check(&all_branches)` check the full set of 4 branches. If true, line 165 or 171 returns `is_dissolved = true` for `target_branch` without verifying if `target_branch` is part of the combination.
- Empirical test case: Chart `Year=戊寅`, `Month=乙卯`, `Day=丙辰`, `Hour=丁丑`.
  - Void branches for `丙辰` day are `子` and `丑`. Hour branch `丑` is voided.
  - `Year(寅)`, `Month(卯)`, `Day(辰)` form `寅卯辰` Seasonal Combination (방합).
  - `check_void_dissolution` returns `is_dissolved: true` with reason `"원국 방합(方合) 성국으로 공망 해소 (공망해합)"` for `丑`, even though `丑` is Earth (Winter) and NOT part of `寅卯辰` Wood (Spring).

### Observation 3: Spirit Marker Position String Mismatch (`spirit_markers.rs:718–730`)
```rust
718:    if !m.is_stem {
719:        let pos_str = m.position.hangul();
720:        for (_, p1, p2) in &rel_analysis.branch_clashes {
721:            if p1 == pos_str || p2 == pos_str { is_clashed = true; }
722:        }
```
- `m.position.hangul()` (line 258 of `spirit_markers.rs`) returns `"년주"`, `"월주"`, `"일주"`, `"시주"`.
- `rel_analysis` (lines 777-780 of `relationships.rs`) stores branch position tuples with `"년지"`, `"월지"`, `"일지"`, `"시지"`.
- `p1 == pos_str` compares `"년지"` with `"년주"`, which is always `false`.
- Empirical test case: Chart `Year=癸未` (Void Tianyi), `Month=己丑` (丑未沖), `Day=庚寅`, `Hour=戊寅`.
  - Expectation: `tc.is_clashed == true`, `tc.summary` contains `"(공망해충/해합 구원)"`.
  - Actual result: `tc.is_clashed` is `false`, `tc.summary` is `"천을귀인 (귀인공망)"`.

### Observation 4: 12-Unseong Yin-Stem Config (`twelve_stages.rs:237–263`)
```rust
253:    let distance = if is_yang || !config.yin_stem_reverse {
254:        // 양간 또는 음포태 동행설(yin_stem_reverse == false): 순행 (시계방향)
255:        (branch_idx - changsheng_idx).rem_euclid(12)
256:    } else {
257:        // 음간 기본: 역행 (반시계방향)
258:        (changsheng_idx - branch_idx).rem_euclid(12)
259:    };
```
- `calculate_twelve_stage_with_config` correctly applies counter-clockwise distance when `yin_stem_reverse: true` and clockwise distance when `yin_stem_reverse: false`.
- Verifiable via `test_twelve_stages_yin_stem_reverse_config`.

---

## 2. Logic Chain

1. **Samjae Logic**:
   - `calculate_samjae` evaluates `(year_branch, transit_year_branch)` by matching the four 3-branch Samhap groups (`申子辰`, `寅午戌`, `巳酉丑`, `亥卯未`).
   - For each group, it correctly matches the 3 opposing branches for Entrance, Dwelling, and Exit.
   - Stress test over all 144 combinations verified 36 valid Samjae states and 108 None states with 100% precision.

2. **Gongmang Dissolution Logic**:
   - Gongmang dissolution requires the voided branch itself to be involved in a clash (충) or combination (합).
   - Single branch clashes (충) and six-combinations (육합) check `(target_branch, b)`, which inherently enforces membership.
   - However, for Triple and Seasonal combinations, `void.rs` passes `&all_branches` to `TripleCombination::check` and `SeasonalCombination::check`. It returns `Some(...)` if ANY combination exists in `all_branches`, missing the check `combination.branches().contains(&target_branch)`.

3. **Spirit Marker Modifier & Restoration Logic**:
   - `spirit_markers.rs` attempts to integrate `rel_analysis` (clashes and combinations) to adjust spirit marker interpretations and restore voided noble spirits.
   - Because `pos_str` uses `PillarPosition::hangul()` (`"년주"`), while `rel_analysis` uses branch string names (`"년지"`), no clash or combination match ever succeeds.
   - Consequently, `is_clashed` and `is_combined` remain `false` for all branch spirit markers in `spirit_markers.rs`.

4. **12-Unseong Config Logic**:
   - `AnalysisConfig::default().yin_stem_reverse` defaults to `true`.
   - `calculate_twelve_stage_with_config` branches based on `is_yang || !config.yin_stem_reverse`.
   - Yang stems are unaffected; Yin stems reverse direction when `true` and advance forward when `false`.

---

## 3. Caveats

- **Scope Limit**: Code modification was prohibited per role constraints ("Review-only — do NOT modify implementation code"). The bugs found were empirically captured and verified via `crates/eon-saju/tests/milestone1_part2_stress_tests.rs` without modifying production source code under `src/`.
- **Downstream Impact**: Fixing Bug 2 (`"년주"` vs `"년지"` mismatch) in `spirit_markers.rs` will activate clash/combination modifiers for all branch spirit markers across the system, which may alter spirit marker detail summaries in UI reports.

---

## 4. Conclusion

- **Samjae**: Implemented correctly for all 12 birth year branches. Recommendation: Wire `calculate_samjae` into `YearlyLuck::calculate` in `periodic_luck.rs`.
- **Gongmang Dissolution**: Functional for 6-clash and 6-combination, but contains a false-positive bug for Triple/Seasonal combinations. Recommendation: In `void.rs`, verify `comb.branches().contains(&target_branch)`.
- **Noble Spirit Marker Annulment & Restoration**: Annulment works; restoration is broken due to a string mismatch (`"년주"` vs `"년지"`). Recommendation: In `spirit_markers.rs:719`, map `pos_str` to `"년지"`, `"월지"`, `"일지"`, `"시지"` or replace string comparisons with enum comparisons.
- **12-Unseong Yin-Stem Config**: Configurable and 100% correct.

---

## 5. Verification Method

To independently verify all claims and bugs:

```bash
cargo check --workspace
cargo test -p eon-saju --test milestone1_part2_stress_tests
```

All 5 test cases in `milestone1_part2_stress_tests.rs` run in under 0.01s and document these findings empirically.
