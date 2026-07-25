# Handoff Report: Milestone 2 (R2) Code Quality & Architecture Review

## 1. Observation

### Verification Executions
- `cargo check --workspace`: Passed cleanly with zero errors and zero warnings.
- `cargo test --workspace`: Passed 100% (all workspace test suites including `eon-saju` unit & integration tests).
- **Integrity Violation Scan**: Verified no hardcoded test results, facade implementations, or verification shortcuts exist in `crates/eon-saju`.

### Key Observations in Source Code

1. **`IntegratedAnalysis::calculate_expanded` (`crates/eon-saju/src/analysis/power.rs:258-295`)**:
   - In `calculate_expanded`, branch elements are collected into `items` tuples storing `(orig_el, eff_el, weight, is_stem, polarity)`.
   - When applying climate correction (`options.apply_correction`), line 284-290 executes:
     ```rust
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
   - `apply_climate_correction` accepts `(branch: EarthlyBranch, month: EarthlyBranch)`.
   - For `EarthlyBranch::Chou` (丑), `orig_el` is `Element::Earth`. The match expression maps `Element::Earth` to `EarthlyBranch::Wei`.
   - In winter months (`EarthlyBranch::Hai` / `Zi`), `apply_climate_correction(EarthlyBranch::Wei, month)` evaluates `Wei` (which checks for summer months `Si`/`Wu`) and returns `Element::Earth`, failing to convert `Chou` (丑) to `Element::Water` as expected during winter.

2. **`TraceTag` Heap Allocation Footprint (`crates/eon-saju/src/engine/trace_tag.rs:61-67` & `crates/eon-saju/src/engine/vm.rs:680-725`)**:
   - `TraceTag::GaeGo`, `TraceTag::IpMyo`, and `TraceTag::DynamicGyeok` variants define `String` fields (e.g. `branch: String`, `unsealed_stem: String`, `tomb_branch: String`, `active_structure: String`, `status: String`).
   - During `SajuVM::step`, string representations are created via `.to_string()` on static string slices (`&'static str`), e.g., `event.branch.hangul().to_string()`.
   - In multi-year VM timeline simulations (e.g., 80-100 `LifeFrame` iterations), this causes unnecessary heap allocations, contradicting `trace_tag.rs` header comments regarding heap allocation minimization.

3. **VM Dynamic Mechanics (`SajuVM::step`)**:
   - `GaeGoEvent` unsealing correctly dumps hidden stems (`b.jijanggan()`), updating `QiRegisters` and adding weighted bonuses.
   - `IpMyoEvent` trapping correctly applies penalties (-8.0) and updates `QiRegisters`.
   - `DynamicStructureState` transitions (`Fulfilled`, `Transformed`, `Broken`, `Stable`) correctly influence VM life frame scores and tags.

---

## 2. Logic Chain

1. **Verification of Integrity & Workspace Build**:
   - *Observation*: `cargo check --workspace` and `cargo test --workspace` passed cleanly. Code inspection confirms standard Saju rules (Oh-Ho-Dun-Wol-Beop, 12-Unseong Mu storage trapping, GaeGo unsealing, Dynamic Gyeokguk state transitions) are implemented algorithmically without hardcoded test branches.
   - *Logic*: The implementation is authentic and functional, but code review must stress-test boundary conditions and logic correctness.

2. **Climate Correction Loss in `calculate_expanded`**:
   - *Observation*: `IntegratedAnalysis::calculate` (natal version) passes the actual `EarthlyBranch` to `apply_climate_correction(branch, month_branch)`.
   - *Observation*: `IntegratedAnalysis::calculate_expanded` replaces the `EarthlyBranch` with `orig_el: Element` inside `items`, then attempts to reconstruct `EarthlyBranch` via `match orig_el { Element::Earth => EarthlyBranch::Wei, _ => EarthlyBranch::Zi }`.
   - *Logic*: All Earth branches (`Wei`, `Chou`, `Chen`, `Xu`) have `orig_el == Element::Earth`. Mapping all Earth elements to `EarthlyBranch::Wei` loses the specific identity of `EarthlyBranch::Chou`. `Chou` (丑) in winter (`Hai`/`Zi`) is meant to absorb Water energy, but because it is evaluated as `Wei`, climate correction returns `Element::Earth`. This produces inaccurate elemental power calculations for expanded 5/6-pillar analysis.

3. **VM Stepping Memory Footprint**:
   - *Observation*: `SajuVM::step` is designed for high-frequency execution (e.g., simulating 100-year timelines or batch Monte Carlo analysis).
   - *Logic*: Allocating owned `String`s for static Hanja/Hangul labels (`"辰"`, `"甲"`, `"성격(成格)"`) inside `TraceTag` variants causes heap allocation churn per frame. Storing `&'static str`, `EarthlyBranch`, `HeavenlyStem`, `GyeokStatus`, or `Cow<'static, str>` avoids dynamic allocations entirely during VM execution.

---

## 3. Caveats

- **Scope Boundary**: Review was strictly analytical and non-destructive. No source code files were edited during this review turn.
- **Test Suite Coverage**: Current test cases in `edge_cases.rs` verify `calculate_expanded` for standard charts, but did not test a natal chart with `EarthlyBranch::Chou` in a winter month under expanded 5/6-pillar options.

---

## 4. Conclusion & Review Verdict

**Verdict**: **REQUEST_CHANGES**

### Findings Summary

#### [Major] Finding 1: Incorrect Climate Correction Branch Mapping in `IntegratedAnalysis::calculate_expanded`
- **Where**: `crates/eon-saju/src/analysis/power.rs:258-293`
- **Why**: `calculate_expanded` maps branch items using `orig_el: Element` and attempts to reconstruct `EarthlyBranch::Wei` for any `Element::Earth`. This erases `EarthlyBranch::Chou` (丑), causing winter climate correction (`Chou` in `Hai`/`Zi` month -> `Water`) to fail in expanded 5/6 pillar analysis.
- **Suggestion**: Include the actual `Option<EarthlyBranch>` in the `items` tuple inside `calculate_expanded`, and pass the real branch to `apply_climate_correction(branch, month_branch)`.

#### [Minor] Finding 2: Unnecessary String Heap Allocations in `TraceTag` and `SajuVM::step`
- **Where**: `crates/eon-saju/src/engine/trace_tag.rs:61-67` and `crates/eon-saju/src/engine/vm.rs:680-725`
- **Why**: `GaeGo`, `IpMyo`, and `DynamicGyeok` tag variants use `String` fields, forcing heap allocations for static domain strings (`event.branch.hangul().to_string()`) during every VM step.
- **Suggestion**: Use domain types (`EarthlyBranch`, `HeavenlyStem`, `Element`, `GyeokStatus`) or `&'static str` in `TraceTag` enum fields to eliminate heap allocations during VM execution.

---

## 5. Verification Method

To independently verify these findings:

1. **Verify Workspace Build & Tests**:
   ```bash
   cargo check --workspace
   cargo test --workspace
   ```

2. **Verify Climate Correction Flaw in `power.rs`**:
   - Inspect `crates/eon-saju/src/analysis/power.rs` lines 283-293.
   - Observe `match orig_el { Element::Earth => EarthlyBranch::Wei, _ => EarthlyBranch::Zi }`.
   - Confirm that `EarthlyBranch::Chou` is mapped to `Wei` and fails winter climate correction.
