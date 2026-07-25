# Challenger Handoff Report: Milestone 1 (R1) Yongsin & Gyeokguk Empirical Stress Testing

**Agent**: `teamwork_preview_challenger_m1_1` (Critic / Specialist)
**Target Module**: `crates/eon-saju/src/analysis/yongshin.rs`, `structure.rs`, `strength.rs`
**Date**: 2026-07-25

---

## 1. Observation

Direct code and test execution evidence gathered during adversarial review:

1. **`strength.rs:416-431` (DeukSe count hardcoding)**:
   ```rust
   for (ten_god, percentage, _) in integrated.ten_god_scores {
       match ten_god {
           TenGod::Bijian | TenGod::Jiecai => { bijie_count += 1; support_ratio += percentage; }
           TenGod::Zhengyin | TenGod::Pianyin => { yinxing_count += 1; support_ratio += percentage; }
           TenGod::Shishen | TenGod::Shangguan => shishang_count += 1,
           TenGod::Zhengcai | TenGod::Piancai => caisheng_count += 1,
           TenGod::Zhengguan | TenGod::Pianguan => guanxing_count += 1,
       }
   }
   ```
   `integrated.ten_god_scores` contains all 10 `TenGod` enum variants (populated from `TenGod::ALL` in `power.rs:200-204`). The loop in `DeukSe::check_with_options` increments `shishang_count`, `caisheng_count`, `guanxing_count`, `bijie_count`, `yinxing_count` by 1 for *every* variant regardless of `percentage > 0.0`. Thus, `shishang_count = 2`, `caisheng_count = 2`, `guanxing_count = 2` for **every single natal chart**.

2. **`structure.rs:340-344` (Follower pattern selection unreachable branches)**:
   ```rust
   let shishang = strength.deuk_se.shishang_count;
   let cai = strength.deuk_se.caisheng_count;
   let guan = strength.deuk_se.guanxing_count;

   let (structure, name, desc) = if shishang >= cai && shishang >= guan { ... }
   ```
   Because `shishang == 2`, `cai == 2`, and `guan == 2` for all charts, `shishang >= cai && shishang >= guan` evaluates to `2 >= 2 && 2 >= 2` (always `true`).

3. **`yongshin.rs:129-137` (Weak DM heavy Caisheng arbitrary threshold)**:
   ```rust
   if cai_power > 35.0 && yin_power < 15.0 {
       day_master_el // 비겁
   } else if guan_power > 35.0 { ... }
   ```
   When `cai_power > 35.0` and `yin_power >= 15.0` (e.g. `yin_power = 15.5%`), the `cai_power > 35.0 && yin_power < 15.0` condition fails, causing the engine to fall through to `else` and select `day_master_el.generated_by()` (Inseong).

4. **`structure.rs:275-320` (Samhap Jeonwang missing breaker checks)**:
   ```rust
   Element::Wood if matches!(month_branch, Yin | Mao | Chen | Hai) => (StructureType::GokJik, ...)
   ```
   Jeonwang patterns (곡직격, 염상격, 가색격, 종혁격, 윤하격) are assigned purely based on `support_ratio >= polarized_high` and `month_branch` season. No check is made for Samhap/Banghap frame existence or breaker stars (파성/관살).

5. **`yongshin.rs:544-546` (Tonggwan option mismatch)**:
   `get_tonggwan_analysis` calls `IntegratedAnalysis::calculate` with `apply_correction: false`, whereas `YongshinAnalysis::from_pillars_with_config` calls it with `apply_correction: true`.

6. **Empirical Test Suite Execution (`crates/eon-saju/tests/milestone1_stress_tests.rs`)**:
   `cargo test -p eon-saju --test milestone1_stress_tests` ran 7 tests.
   - `test_m1_bug_deuk_se_count_prevents_jong_jae` confirmed that a 100% Earth (Wealth) chart (`戊戌 己未 甲辰 己丑`) evaluates to `GaJongAh` (식상종격) instead of `JongJae` (재성종격).

---

## 2. Logic Chain

1. **Step 1 (Follower Pattern Bug)**:
   - *Observation 1*: `DeukSe` loops through all 10 TenGod enum variants in `integrated.ten_god_scores` without checking `percentage > 0.0`.
   - *Reasoning*: Because every TenGod enum is present in `integrated.ten_god_scores`, `shishang_count`, `caisheng_count`, `guanxing_count` are hardcoded to `2`.
   - *Observation 2*: `structure.rs:342` evaluates `if shishang >= cai && shishang >= guan`.
   - *Reasoning*: Since `2 >= 2 && 2 >= 2` is always `true`, the condition for `JongAh` / `GaJongAh` matches on every follower chart. The `else if cai >= shishang && cai >= guan` (`JongJae` / `GaJongJae`) and `else` (`JongSal` / `GaJongSal`) branches are completely unreachable.
   - *Conclusion*: `StructureType::JongJae`, `GaJongJae`, `JongSal`, `GaJongSal` can never be assigned in the current implementation.

2. **Step 2 (재다신약 Boundary Brittle Risk)**:
   - *Observation 3*: `yongshin.rs:129` checks `cai_power > 35.0 && yin_power < 15.0`.
   - *Reasoning*: In Saju theory (재다신약 財多身弱), heavy Caisheng (財) attacks Inseong (印 - 財剋印). Recommending Inseong when Caisheng is high leads to a direct clash. Having an arbitrary hard cutoff at `yin_power < 15.0%` means a chart with `cai_power = 45%` and `yin_power = 15.1%` will recommend Inseong instead of BiGeop.

3. **Step 3 (Jeonwang Breaker Star Omission)**:
   - *Observation 4*: `structure.rs:275-320` evaluates Jeonwang patterns solely based on `support_ratio` and `month_branch`.
   - *Reasoning*: Classical Jeonwang patterns (전왕 5격) require pure element strength without opposing GwanSal or breaker stars (파성 破星). A chart with 80% Wood and 20% Metal (e.g. 酉 in hour branch) will still be classified as pure `GokJik` (곡직격) because `support_ratio >= 80.0%` and `month_branch == Mao`, despite the presence of 酉 (Metal) which breaks (破格) the GokJik pattern.

---

## 3. Caveats

- **Scope Limit**: Review focused on `yongshin.rs`, `structure.rs`, `strength.rs`, and `power.rs` in `eon-saju`. Dioxus UI components in `eon-ui` and Vedic logic in `eon-vedic` were not evaluated in this stress test.
- **Code Modifications**: Per role guidelines ("Review-only — do NOT modify implementation code"), no production engine files were modified. All bug demonstrations were performed via empirical tests in `tests/milestone1_stress_tests.rs`.

---

## 4. Conclusion & Adversarial Review Summary

### Challenge Summary
**Overall Risk Assessment**: **CRITICAL**

### Challenges

#### [CRITICAL] Challenge 1: Hardcoded TenGod Counts in `DeukSe` render `JongJae` and `JongSal` Unreachable
- **Assumption challenged**: `DeukSe` counts (`shishang_count`, `caisheng_count`, `guanxing_count`) reflect actual natal chart element distribution.
- **Attack scenario**: Input a pure Wealth follower chart (e.g. DM 甲, Year 戊戌, Month 己未, Day 甲辰, Hour 己丑).
- **Blast radius**: 100% of follower charts that should be `JongJae` (진종재격/가종재격) or `JongSal` (진종살격/가종살격) are incorrectly output as `JongAh` (식상종격).
- **Mitigation**: Update `DeukSe::check_with_options` to either filter `percentage > 0.0` or compare TenGod energy percentages (`shishang_power`, `cai_power`, `guan_power`) directly in `structure.rs`.

#### [HIGH] Challenge 2: Arbitrary Inseong Threshold (`yin_power < 15.0%`) in 재다신약 Yongsin
- **Assumption challenged**: `yin_power < 15.0%` is a safe boundary for selecting BiGeop in 재다신약.
- **Attack scenario**: A weak DM chart with `cai_power = 45%` and `yin_power = 15.1%`.
- **Blast radius**: Recommends Inseong (which suffers 財剋印 from 45% Caisheng) instead of BiGeop.
- **Mitigation**: Compare relative power: if `cai_power > yin_power * 2.0` and `cai_power > 35.0`, select BiGeop regardless of absolute `yin_power`.

#### [MEDIUM] Challenge 3: Jeonwang 5-Type Classification Ignores Breaker Stars (파성)
- **Assumption challenged**: High `support_ratio` and birth season are sufficient to declare GokJik / YeomSang / GaSaek / JongHyeok / YoonHa.
- **Attack scenario**: High Wood chart (`support_ratio = 82%`) with a clashing 酉 (Metal) branch in hour pillar.
- **Blast radius**: Misclassifies a broken pattern (파격) as a pure Jeonwang outer pattern.
- **Mitigation**: Add a breaker star check: verify no opposing element (controlled_by / controls) exists in Heavenly Stems or Branch hidden stems.

---

## 5. Stress Test Results

| Scenario | Target Case | Expected Behavior | Actual Behavior | Pass/Fail |
|---|---|---|---|---|
| 1 | Weak DM Heavy Caisheng (재다신약) | Yongsin is BiGeop (Wood) | Yongsin is BiGeop (Wood) | **PASS** |
| 2 | Equal 50/50 Metal-Wood Clash | Tonggwan is Primary (Water) | Tonggwan is Primary (Water) | **PASS** |
| 3a | Extreme Winter Chart | Johu preferred stem = 丙火 | Johu preferred stem = 丙火 | **PASS** |
| 3b | Extreme Summer Chart | Johu preferred stem = 癸水 | Johu preferred stem = 癸水 | **PASS** |
| 4a | True vs Fake Jong (Root Check) | Differs `Jong` vs `GaJong` | Root check differentiates `Jong` vs `GaJong` | **PASS** |
| 4b | Heavy Wealth Follower (`JongJae`) | Classified as `JongJae` / `GaJongJae` | Misclassified as `GaJongAh` due to DeukSe count bug | **FAIL (BUG)** |
| 5 | Samhap Jeonwang (5 Types) | Identifies GokJik, YeomSang, GaSaek, JongHyeok, YoonHa | Identifies all 5 Jeonwang types | **PASS** |

---

## 6. Verification Method

To independently verify these empirical findings:
1. Run workspace compilation check:
   ```bash
   cargo check --workspace
   ```
2. Run `eon-saju` unit & integration tests:
   ```bash
   cargo test -p eon-saju
   ```
3. Run the empirical stress test suite:
   ```bash
   cargo test -p eon-saju --test milestone1_stress_tests -- --nocapture
   ```
4. Inspect `tests/milestone1_stress_tests.rs` for `test_m1_bug_deuk_se_count_prevents_jong_jae` output.
