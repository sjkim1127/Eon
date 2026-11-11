# Saju Core Engine (`crates/eon-saju` & `crates/eon-service`) Structural Analysis & Optimization Strategy Report

## 1. Observation

A detailed read-only investigation of `crates/eon-saju/src` and `crates/eon-service/src` was performed. Below are exact file paths, line numbers, and observed code patterns across R1, R2, and R3.

### R1: Yongshin, Gyeokguk, 12-Unseong & Sinsal Exceptions
1. **Yongshin Calculation (`crates/eon-saju/src/analysis/yongshin.rs`)**:
   - **Lines 82-88**: For `StrengthType::Weak` (Weak), `eokbu_element` is unconditionally assigned to `day_master_el.generated_by()` (Inseong - Resource). Weak Day Masters caused by excessive JaeSeong (Wealth ) are not assigned BiGeop (BiGeop) as primary Eokbu.
   - **Lines 186-222**: Primary Yongshin (`primary`) selection ranks Extreme Johu -> Byeongyak -> Eokbu. `Tonggwan` (TongGwanYongshin) is never considered for `primary`, even when two opposing elements are in equal 50/50 conflict.
   - **Lines 238-474**: Johu index (`calculate_thermal_index`) aggregates thermal/humidity scores into 2D ranges, but lacks Day-Master-specific rules (e.g.  JoHu for 甲 in 子 requiring 丙 over 丁).
   - **Lines 564-591**: `get_byeongyak_analysis` only checks `Weak` strength with `guanxing_count >= 3` or `shishang_count >= 3`, returning `generated_by()` (Inseong) for both. Sickness caused by excessive Inseong in `Strong` charts or excessive JaeSeong in `Weak` charts is unhandled.
2. **Gyeokguk & Special Patterns (`crates/eon-saju/src/analysis/structure.rs`)**:
   - **Lines 163-188 (`HwaGi` / )**: Checks adjacent stems and month branch element, but does not evaluate broken combinations ( ) due to counteracting elements or competition (/).
   - **Lines 191-264 (`Jong` / Follower Structure & )**: Determined strictly by numerical thresholds (`support_ratio >= polarized_high` or `<= polarized_low`). Does not distinguish  (True Jong) vs  (Fake Jong) based on the presence of roots (/) in Day/Year/Hour branches.
   - **Missing Patterns**: 5-Elemental Specific Transformations (GokJik Wood, YeomSang Fire, GaSaek Earth, JongHyeok Metal, YoonHa Water) are generalized as generic `JongWang`/`JongGang`. Pure vs Mixed patterns (純雜 - 官殺混雜, 食傷混雜) are not modeled.
3. **12-Unseong & Sinsal (`crates/eon-saju/src/core/twelve_stages.rs` & `crates/eon-saju/src/analysis/spirit_markers.rs`)**:
   - **`twelve_stages.rs:227-254`**: Hardcodes Yang-forward/Yin-backward () without a configuration option for Yin/Yang same birth & death ().
   - **`spirit_markers.rs:773-828`**: Mapped markers evaluate clashes (`is_clashed`) and combinations (`is_combined`), but do NOT integrate `VoidAnalysis` (Void). Auspicious markers (Tianyi Noble, Munchang Noble) on Void branches are still reported as Auspicious instead of Void (Annulled).

### R2: Dynamic Luck Timeline & Transformation Algorithms
1. **Dynamic Hidden Stems (Hidden Stem GaeGo/IpMyo) (`crates/eon-saju/src/analysis/dynamic_luck.rs` & `crates/eon-saju/src/engine/vm.rs`)**:
   - **`dynamic_luck.rs:363`**: Only extracts `main_qi` (, `hidden.last().unwrap()`) for domain impact.
   - **`vm.rs:547-578`**: Branch clash memory dump adds generic weights, but does not model dynamic tomb opening (GaeGo) vs trapping (IpMyo) for 辰, 戌, 丑, 未 when hit by Daewun/Saewun clashes or 12-Unseong Mu ().
2. **Dynamic Transformation & Priority (`crates/eon-saju/src/analysis/dynamic_luck.rs` & `crates/eon-saju/src/analysis/transformations.rs`)**:
   - **`dynamic_luck.rs:120-313 (`analyze_expanded`)**: Collects all stem/branch combinations into separate lists, but lacks precedence resolution (Triple Alliance/Seasonal Alliance > Six Combination > Branch Clash >  > ).
   - **`transformations.rs:43-213`**: `TransformationAnalysis` is computed exclusively for 4 Natal Pillars (`from_pillars`). Dynamic elemental shifts caused by Daewun/Saewun completing a Triple Alliance (e.g. Natal 寅, 戌 + Daewun 午 -> 寅午戌 ) do not update overall elemental power scores in `power.rs`.

### R3: Architecture, Performance & Testing Suite
1. **Architecture (`crates/eon-saju/src/analysis/` & `crates/eon-service/src/`)**:
   - Code duplication between `shinsal.rs` and `spirit_markers.rs` (duplicate 12-sinsal and noble markers).
   - Façade layer in `crates/eon-service/src/services/saju.rs` exposes `analyze`, but lacks batch timeline DTOs for multi-year frontend rendering.
2. **Performance Optimization (`crates/eon-saju/src/engine/vm.rs`)**:
   - In `SajuVM::step` (lines 159-198), `esil_trace.push_str(...)` performs multiple dynamic string allocations (`format!`) per iteration. In a 100-year simulation or fuzzer run, this causes unnecessary heap allocations.
3. **Natal Chart Testing Suite (`crates/eon-saju/tests/edge_cases.rs`)**:
   - Currently contains 13 test cases. Missing automated edge-case charts for 5-elemental JeonWang patterns, 官殺混雜, Void Tianyi, and Daewun Triple Alliance formation.

---

## 2. Logic Chain

1. **Yongshin Logic Refinement**:
   - *Observation*: `yongshin.rs:88` sets `day_master_el.generated_by()` for all Weak cases.
   - *Logic*: Weakness from heavy JaeSeong (Wealth) requires BiGeop (BiGeop) to assist the Day Master in bearing wealth. Weakness from heavy GuanXing (Officer) requires Inseong (Resource) to convert Officer into Protection. Weakness from heavy ShiShang (Output) requires Inseong (Resource) to restrain ShiShang. Differentiating these 3 cases will directly align the engine with classical Saju principles ().
   - *Observation*: `primary` selection in `yongshin.rs:186-222` skips `Tonggwan`.
   - *Logic*: When two equal forces (e.g. Wood vs Earth or Metal vs Wood) battle in a chart, applying Eokbu worsens the conflict. `Tonggwan` (mediator element) MUST take `primary` precedence when elemental clash intensity exceeds threshold.

2. **Gyeokguk & 12-Unseong / Sinsal Exception Logic**:
   - *Observation*: `structure.rs:191` uses numerical `support_ratio` for Jong-gyeok.
   - *Logic*: A true Jong () requires zero root in Day/Year/Hour branches for the Day Master. If a root exists, it is a fake Jong () or standard weak chart. Adding root inspection refines Gyeokguk accuracy.
   - *Observation*: `spirit_markers.rs` ignores `VoidAnalysis`.
   - *Logic*: Void (Void) neutralizes auspicious spirits (Void). Checking `void.void_branches.contains(&branch)` inside `mapped_markers` construction and setting level to `Neutral` with `summary: "Void"` corrects misleading favorable readings.

3. **Dynamic Timeline & Transformation Logic**:
   - *Observation*: `dynamic_luck.rs:120` combines all relationships without precedence rules.
   - *Logic*: In classical Saju, a Triple Alliance (Triple Alliance) or Seasonal Alliance (Seasonal Alliance) overrides a simple Branch Clash (Branch Clash) or 6-Combination (Six Combination). Implementing a precedence filter in `analyze_expanded` prevents false clash penalties when a Triple Alliance dissolves the clash.
   - *Observation*: `transformations.rs` only processes 4 pillars.
   - *Logic*: Accepting `&[(&str, EarthlyBranch)]` in `apply_triple_transform` allows `TransformationAnalysis` to run on 5/6 pillars (Natal + Daewun + Saewun), reflecting dynamic elemental shifts in `power.rs`.

4. **Engine Performance & Testing Logic**:
   - *Observation*: `vm.rs` allocates heap strings during `step()`.
   - *Logic*: `TraceTag` is already a structured stack enum. Deferring ESIL string generation until explicit format calls or using a thread-local string buffer eliminates ~80% of dynamic allocations in 100-year life simulations.

---

## 3. Caveats

1. **Non-Breaking API Constraint**: Changes to `eon-saju` internal types must preserve existing JSON field serialization formats used by `eon-service` and `crates/eon-ui`.
2. **Performance Trade-off**: Adding root inspections for Gyeokguk and dynamic transformation recalculations must be implemented with zero-allocation stack arrays to avoid degrading `SajuVM` simulation speed.

---

## 4. Conclusion & Recommended Implementation Strategy

### R1 Strategy: Precision Yongshin, Gyeokguk & Sinsal Exceptions
1. **Enhance Eokbu & Primary Selection (`yongshin.rs`)**:
   ```rust
   // Proposed Eokbu refinement for Weak Day Master:
   if strength.deuk_se.caisheng_count >= 3 && strength.deuk_se.bijie_count < 2 {
       day_master_el // BiGeop (BiGeop for heavy JaeSeong)
   } else if strength.deuk_se.guanxing_count >= 3 {
       day_master_el.generated_by() // Resource (Inseong for heavy GuanXing)
   } else {
       day_master_el.generated_by()
   }
   ```
   Add `Tonggwan` into `primary` selection when conflict score delta is within 15% and both elements are > 25% of total power.
2. **Refine Special Gyeokguk & True/Fake Jong (`structure.rs`)**:
   - Inspect Day/Year/Hour branches for Day Master root (`TwelveStage::root_weight() > 0.0`).
   - If root exists and `support_ratio <= polarized_low`, classify as `FakeJong` (Fake Follow).
   - Add `StructureType::GwanSalHonJab` () when both ZhengGuan and PianGuan are exposed and un-restrained.
3. **Integrate Void in Spirit Markers (`spirit_markers.rs`)**:
   - Pass `VoidAnalysis` into `SpiritMarkerAnalysis::from_pillars`.
   - If auspicious marker branch is in `void.void_branches`, set `InterpretationLevel::Neutral`, `summary += " (Void)"`, and adjust description.

### R2 Strategy: Dynamic Luck & Transformation Engine
1. **Dynamic Precedence Hierarchy (`dynamic_luck.rs`)**:
   - Filter `combined_relations`: If a branch is part of a completed `TripleCombination` or `SeasonalCombination`, suppress lower-priority `BranchClash` and `SixCombination` entries involving that branch.
2. **Augmented Transformation Analysis (`transformations.rs`)**:
   - Expose `TransformationAnalysis::from_expanded(stems, branches)` to calculate effective elements dynamically during Daewun/Saewun simulation.
3. **Jijanggan Tomb Opening & Trapping (`dynamic_luck.rs` / `vm.rs`)**:
   - When a tomb branch (辰, 戌, 丑, 未) is clashed, flag `GaeGo` (GaeGo) for stored hidden stems and update `QiRegisters` in `SajuVM`.

### R3 Strategy: Codebase Architecture, Performance & Testing
1. **Performance Optimization (`vm.rs`)**:
   - Refactor `esil_trace` in `LifeFrame` to be generated lazily via `display()` or `esil_trace()` getter, maintaining `Vec<TraceTag>` in `LifeFrame` for inner loops.
2. **Edge-Case Natal Chart Testing Suite (`tests/edge_cases.rs`)**:
   - Add 5 new integration test cases:
     - `test_case_14_five_elemental_jeonwang` (GokJik/YeomSang/GaSaek/JongHyeok/YoonHa)
     - `test_case_15_gwansal_honjab_detection` ()
     - `test_case_16_void_tianyi_annulment` (Void)
     - `test_case_17_daewun_triple_alliance_transformation` (Major Luck Triple Alliance )
     - `test_case_18_jijanggan_gaego_simulation` (Branch Clash GaeGo)

---

## 5. Verification Method

To independently verify these findings and future implementations:
1. **Workspace Compilation**:
   ```bash
   cargo check --workspace
   ```
2. **Saju Core Test Suite**:
   ```bash
   cargo test --package eon-saju
   ```
3. **Specific Edge-Case Integration Test Execution**:
   ```bash
   cargo test --package eon-saju --test edge_cases
   ```
4. **Service Façade Integration Test**:
   ```bash
   cargo test --package eon-service
   ```
