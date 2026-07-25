# Challenger Handoff Report — Milestone 2 (R2) SajuVM & Dynamic Luck

## 1. Observation

Direct observations from source inspection and test execution (`cargo test --test challenger_m2_2_verify -- --nocapture` and `cargo test --test milestone2_stress_tests`):

1. **GaeGo Double-Scoring in `SajuVM` (`crates/eon-saju/src/engine/vm.rs`)**:
   - In `vm.rs` lines 552–578 (Section 3: `지지 합충 분석`), every branch clash evaluates hidden stems of participating branches and calls `registers.update(el, bonus)` and `score += bonus` (logged as `mem_dump:<branch>(<stem>)`).
   - In `vm.rs` lines 661–685 (Section 6.7: `고지 개고`), `evaluate_score` iterates over `dynamic.gaego_events` and calls `registers.update(el, bonus)` and `score += bonus` (logged as `gaego:<branch>(<stem>)`) for the exact same unsealed stems.
   - Verbatim ESIL trace output from empirical run:
     `clash:월지-세운지지,impact:10.0; mem_dump:술(신),bonus:1.5; mem_dump:술(무),bonus:-2.4; mem_dump:진(계),bonus:3.0; mem_dump:진(무),bonus:-2.4; ... gaego:술(신),bonus:1.5; gaego:술(정),bonus:0.0; gaego:술(무),bonus:-2.4; gaego:진(을),bonus:0.0; gaego:진(계),bonus:3.0; gaego:진(무),bonus:-2.4;`
     (Total `mem_dump` count: 12, `gaego` count: 12 in a single `vm.step`).

2. **IpMyo Element Mismatch for Yin Day Masters (`crates/eon-saju/src/analysis/dynamic_luck.rs`)**:
   - `evaluate_ipmyo_events` (lines 464–492) checks `calculate_twelve_stage(dm, g.branch) == TwelveStage::Mu`.
   - `g.branch` is matched against hardcoded element tombs (lines 474–482):
     ```rust
     let trapped_el = match g.branch {
         EarthlyBranch::Chen => Element::Water,
         EarthlyBranch::Xu => Element::Fire,
         EarthlyBranch::Chou => Element::Metal,
         EarthlyBranch::Wei => Element::Wood,
         _ => dm.element(),
     };
     ```
   - For Yin Day Masters (乙 Wood, 丁 Fire, 辛 Metal, 癸 Water), 12-Unseong runs in reverse:
     - 乙 (Yin Wood) DM reaches `Mu` (묘) at 戌. `g.branch` is 戌, so `trapped_el` evaluates to `Element::Fire`!
     - Empirical output: `DM: Eul(Wood), Tomb Branch: 술, Trapped Element: Fire, Trigger: 대운 12운성 묘(墓)지 입묘`.

3. **Indiscriminate Gyeokguk Fulfillment (`crates/eon-saju/src/analysis/dynamic_luck.rs`)**:
   - `evaluate_structure_state` (lines 531–545) checks `natal.month.branch.hidden_stems().contains(&s)` for luck stems `s`.
   - When luck stem is 乙 (Rob Wealth / 劫財) and month branch is 辰 (`[乙, 癸, 戊]`), `status` transitions to `GyeokStatus::Fulfilled`.
   - Empirical output: `Status: Fulfilled, Description: "대운 천간 편관 투출로 격국 성격"` / `"대운 천간 겁재 투출로 격국 성격"`.

4. **Asymmetry in GaeGo Trigger Condition (`crates/eon-saju/src/analysis/dynamic_luck.rs`)**:
   - `evaluate_gaego_events` (lines 405–462) requires `(p1.contains("운") || p2.contains("운"))` for clashes and six-combinations, but omits this check for triple (`triple_combinations`) and seasonal (`seasonal_combinations`) alliances.
   - Result: Static natal-only triple alliances generate GaeGo events during luck-free analysis (`DynamicLuckAnalysis::analyze(natal, None, None, None, None, None)`).

5. **Existing Workspace Test Failures (`crates/eon-saju/tests/milestone2_stress_tests.rs`)**:
   - `test_expanded_power_correction_branch_integrity` failed: `Chart without water stems/branches had Water percentage 53.1%! (Corruption bug in power.rs)`.
   - `test_wolwun_early_january_saju_year` failed: `assertion failed: Jan 2, 2026 month stem must be 戊 (year 2025 乙巳), NOT 庚 (year 2026 丙午)`.

## 2. Logic Chain

1. **GaeGo Double Scoring**:
   - Observation 1 shows `SajuVM::evaluate_score` executes two separate passes over storage branch stems when a clash occurs: first in section 3 (`mem_dump`), second in section 6.7 (`gaego`).
   - Because both passes mutate `score` by adding `bonus` and call `registers.update(el, bonus)`, storage branch hidden stems are scored twice, inflating energy register values and distortion of VM scores during luck clash periods.

2. **IpMyo Yin Stem Element Distortion**:
   - Observation 2 shows `evaluate_ipmyo_events` triggers when `dm` (Day Master) reaches `TwelveStage::Mu`.
   - For 乙 (Yin Wood), 12-Unseong `Mu` occurs at 戌. But `match g.branch` maps 戌 to `Element::Fire` (the tomb of Fire).
   - Thus, an 乙 Day Master entering 戌 tomb produces an `IpMyoEvent` claiming Fire is trapped, contradicting the Day Master's element (Wood) and incorrectly penalizing Fire registers.

3. **Gyeokguk False Fulfillment**:
   - Observation 3 shows `evaluate_structure_state` checks only `hidden_stems().contains(&s)`.
   - In Saju theory, Rob Wealth (劫財) and Friend (比肩) do not constitute valid Gyeokguk structures, nor does their emergence from hidden stems constitute structure fulfillment ("성격").
   - Treating any matching hidden stem (including non-primary or destructive ten gods) as `Fulfilled` generates false positive structure transitions during luck cycles.

4. **GaeGo Trigger Asymmetry**:
   - Observation 4 shows `evaluate_gaego_events` enforces luck pillar checks for clashes (`p1.contains("운")`) but not for triple combinations (`tri.branches().contains(&b)`).
   - Consequently, natal storage branches in static triple alliances are reported as dynamic GaeGo events even when no luck cycle interacts with them.

## 3. Caveats

- **No implementation code modified**: As per role constraints (review-only / critic), zero source files in `crates/` were modified. Only test harness `crates/eon-saju/tests/challenger_m2_2_verify.rs` was added for empirical reproduction.
- **Workspace Test Failures**: `milestone2_stress_tests.rs` contains two failing tests (`test_expanded_power_correction_branch_integrity` and `test_wolwun_early_january_saju_year`) created by other test suites. These were noted as existing findings and not modified.

## 4. Conclusion

Empirical adversarial stress testing of Milestone 2 (R2) Jijanggan GaeGo/IpMyo and Dynamic Gyeokguk in `SajuVM` revealed **4 critical logic defects**:
1. **Double-scoring of hidden stems** in `SajuVM::evaluate_score` during clash-induced GaeGo.
2. **Element mismatch in `IpMyoEvent`** for Yin Day Masters (乙, 丁, 辛, 癸) due to fixed branch tomb lookup.
3. **False positive `Fulfilled` (성격) transitions** for invalid ten gods (e.g. 劫財) in `evaluate_structure_state`.
4. **Natal-only GaeGo false positives** due to missing luck pillar filters on triple/seasonal combinations.

## 5. Verification Method

To independently reproduce and verify all findings:

1. Run the empirical challenger test suite:
   ```bash
   cargo test --test challenger_m2_2_verify -- --nocapture
   ```
2. Run workspace stress tests:
   ```bash
   cargo test --test milestone2_stress_tests
   ```
3. Inspect `crates/eon-saju/src/engine/vm.rs` at lines 552-578 (`mem_dump`) vs lines 661-685 (`gaego`).
4. Inspect `crates/eon-saju/src/analysis/dynamic_luck.rs` at lines 474-482 (`evaluate_ipmyo_events`) and lines 531-545 (`evaluate_structure_state`).
