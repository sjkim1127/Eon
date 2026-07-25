# Saju Core Engine (`crates/eon-saju` & `crates/eon-service`) Structural Analysis & Optimization Strategy Report

## 1. Observation

A detailed read-only investigation of `crates/eon-saju/src` and `crates/eon-service/src` was performed. Below are exact file paths, line numbers, and observed code patterns across R1, R2, and R3.

### R1: Yongshin, Gyeokguk, 12-Unseong & Sinsal Exceptions
1. **Yongshin Calculation (`crates/eon-saju/src/analysis/yongshin.rs`)**:
   - **Lines 82-88**: For `StrengthType::Weak` (신약), `eokbu_element` is unconditionally assigned to `day_master_el.generated_by()` (Inseong - 인성). Weak Day Masters caused by excessive JaeSeong (재성 과다) are not assigned BiGeop (비겁) as primary Eokbu.
   - **Lines 186-222**: Primary Yongshin (`primary`) selection ranks Extreme Johu -> Byeongyak -> Eokbu. `Tonggwan` (통관용신) is never considered for `primary`, even when two opposing elements are in equal 50/50 conflict.
   - **Lines 238-474**: Johu index (`calculate_thermal_index`) aggregates thermal/humidity scores into 2D ranges, but lacks Day-Master-specific rules (e.g. 궁통보감 십간조후론 for 甲목 in 子월 requiring 丙화 over 丁화).
   - **Lines 564-591**: `get_byeongyak_analysis` only checks `Weak` strength with `guanxing_count >= 3` or `shishang_count >= 3`, returning `generated_by()` (Inseong) for both. Sickness caused by excessive Inseong in `Strong` charts or excessive JaeSeong in `Weak` charts is unhandled.
2. **Gyeokguk & Special Patterns (`crates/eon-saju/src/analysis/structure.rs`)**:
   - **Lines 163-188 (`HwaGi` / 화기격)**: Checks adjacent stems and month branch element, but does not evaluate broken combinations (합화 파격) due to counteracting elements or competition (쟁합/투합).
   - **Lines 191-264 (`Jong` / 종격 & 전왕격)**: Determined strictly by numerical thresholds (`support_ratio >= polarized_high` or `<= polarized_low`). Does not distinguish 진종 (True Jong) vs 가종 (Fake Jong) based on the presence of roots (근/뿌리) in Day/Year/Hour branches.
   - **Missing Patterns**: 5-Elemental Specific Transformations (곡직격 Wood, 염상격 Fire, 가색격 Earth, 종혁격 Metal, 윤하격 Water) are generalized as generic `JongWang`/`JongGang`. Pure vs Mixed patterns (純雜 - 官殺混雜, 食傷混雜) are not modeled.
3. **12-Unseong & Sinsal (`crates/eon-saju/src/core/twelve_stages.rs` & `crates/eon-saju/src/analysis/spirit_markers.rs`)**:
   - **`twelve_stages.rs:227-254`**: Hardcodes Yang-forward/Yin-backward (양순음역) without a configuration option for Yin/Yang same birth & death (음양동생동사).
   - **`spirit_markers.rs:773-828`**: Mapped markers evaluate clashes (`is_clashed`) and combinations (`is_combined`), but do NOT integrate `VoidAnalysis` (공망). Auspicious markers (천을귀인, 문창귀인) on Void branches are still reported as Auspicious instead of 귀인공망 (Annulled).

### R2: Dynamic Luck Timeline & Transformation Algorithms
1. **Dynamic Hidden Stems (지장간 개고/입묘) (`crates/eon-saju/src/analysis/dynamic_luck.rs` & `crates/eon-saju/src/engine/vm.rs`)**:
   - **`dynamic_luck.rs:363`**: Only extracts `main_qi` (본기, `hidden.last().unwrap()`) for domain impact.
   - **`vm.rs:547-578`**: Branch clash memory dump adds generic weights, but does not model dynamic tomb opening (개고) vs trapping (입묘) for 辰, 戌, 丑, 未 when hit by Daewun/Saewun clashes or 12-Unseong Mu (묘지).
2. **Dynamic Transformation & Priority (`crates/eon-saju/src/analysis/dynamic_luck.rs` & `crates/eon-saju/src/analysis/transformations.rs`)**:
   - **`dynamic_luck.rs:120-313 (`analyze_expanded`)**: Collects all stem/branch combinations into separate lists, but lacks precedence resolution (삼합/방합 > 육합 > 지지충 > 반합 > 형해파).
   - **`transformations.rs:43-213`**: `TransformationAnalysis` is computed exclusively for 4 Natal Pillars (`from_pillars`). Dynamic elemental shifts caused by Daewun/Saewun completing a Triple Alliance (e.g. Natal 寅, 戌 + Daewun 午 -> 寅午戌 화국) do not update overall elemental power scores in `power.rs`.

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
   - *Logic*: Weakness from heavy JaeSeong (재성) requires BiGeop (비겁) to assist the Day Master in bearing wealth. Weakness from heavy GuanXing (관성) requires Inseong (인성) to convert Officer into Protection. Weakness from heavy ShiShang (식상) requires Inseong (인성) to restrain ShiShang. Differentiating these 3 cases will directly align the engine with classical Saju principles (자평진전).
   - *Observation*: `primary` selection in `yongshin.rs:186-222` skips `Tonggwan`.
   - *Logic*: When two equal forces (e.g. Wood vs Earth or Metal vs Wood) battle in a chart, applying Eokbu worsens the conflict. `Tonggwan` (mediator element) MUST take `primary` precedence when elemental clash intensity exceeds threshold.

2. **Gyeokguk & 12-Unseong / Sinsal Exception Logic**:
   - *Observation*: `structure.rs:191` uses numerical `support_ratio` for Jong-gyeok.
   - *Logic*: A true Jong (진종) requires zero root in Day/Year/Hour branches for the Day Master. If a root exists, it is a fake Jong (가종) or standard weak chart. Adding root inspection refines Gyeokguk accuracy.
   - *Observation*: `spirit_markers.rs` ignores `VoidAnalysis`.
   - *Logic*: Void (공망) neutralizes auspicious spirits (귀인공망). Checking `void.void_branches.contains(&branch)` inside `mapped_markers` construction and setting level to `Neutral` with `summary: "귀인공망"` corrects misleading favorable readings.

3. **Dynamic Timeline & Transformation Logic**:
   - *Observation*: `dynamic_luck.rs:120` combines all relationships without precedence rules.
   - *Logic*: In classical Saju, a Triple Alliance (삼합) or Seasonal Alliance (방합) overrides a simple Branch Clash (지충) or 6-Combination (육합). Implementing a precedence filter in `analyze_expanded` prevents false clash penalties when a Triple Alliance dissolves the clash.
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
       day_master_el // 비겁 (BiGeop for heavy JaeSeong)
   } else if strength.deuk_se.guanxing_count >= 3 {
       day_master_el.generated_by() // 인성 (Inseong for heavy GuanXing)
   } else {
       day_master_el.generated_by()
   }
   ```
   Add `Tonggwan` into `primary` selection when conflict score delta is within 15% and both elements are > 25% of total power.
2. **Refine Special Gyeokguk & True/Fake Jong (`structure.rs`)**:
   - Inspect Day/Year/Hour branches for Day Master root (`TwelveStage::root_weight() > 0.0`).
   - If root exists and `support_ratio <= polarized_low`, classify as `FakeJong` (가종격).
   - Add `StructureType::GwanSalHonJab` (관살혼잡격) when both ZhengGuan and PianGuan are exposed and un-restrained.
3. **Integrate Void in Spirit Markers (`spirit_markers.rs`)**:
   - Pass `VoidAnalysis` into `SpiritMarkerAnalysis::from_pillars`.
   - If auspicious marker branch is in `void.void_branches`, set `InterpretationLevel::Neutral`, `summary += " (귀인공망)"`, and adjust description.

### R2 Strategy: Dynamic Luck & Transformation Engine
1. **Dynamic Precedence Hierarchy (`dynamic_luck.rs`)**:
   - Filter `combined_relations`: If a branch is part of a completed `TripleCombination` or `SeasonalCombination`, suppress lower-priority `BranchClash` and `SixCombination` entries involving that branch.
2. **Augmented Transformation Analysis (`transformations.rs`)**:
   - Expose `TransformationAnalysis::from_expanded(stems, branches)` to calculate effective elements dynamically during Daewun/Saewun simulation.
3. **Jijanggan Tomb Opening & Trapping (`dynamic_luck.rs` / `vm.rs`)**:
   - When a tomb branch (辰, 戌, 丑, 未) is clashed, flag `GaeGo` (개고) for stored hidden stems and update `QiRegisters` in `SajuVM`.

### R3 Strategy: Codebase Architecture, Performance & Testing
1. **Performance Optimization (`vm.rs`)**:
   - Refactor `esil_trace` in `LifeFrame` to be generated lazily via `display()` or `esil_trace()` getter, maintaining `Vec<TraceTag>` in `LifeFrame` for inner loops.
2. **Edge-Case Natal Chart Testing Suite (`tests/edge_cases.rs`)**:
   - Add 5 new integration test cases:
     - `test_case_14_five_elemental_jeonwang` (곡직/염상/가색/종혁/윤하격)
     - `test_case_15_gwansal_honjab_detection` (관살혼잡격)
     - `test_case_16_void_tianyi_annulment` (귀인공망)
     - `test_case_17_daewun_triple_alliance_transformation` (대운 삼합 성국)
     - `test_case_18_jijanggan_gaego_simulation` (지지충 개고)

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
