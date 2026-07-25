# Handoff Report — Milestone 2 (R2) Remediation Empirical Challenge 2

## 1. Observation

### Verification Suite Results
- Executed `cargo test --test challenger_m2_2_verify -- --nocapture`: Passed (3/3 tests passed).
- Executed `cargo test --test challenger_m2_remediation_stress -- --nocapture`: Passed (11/11 tests passed).
- Executed `cargo test --workspace`: Passed (all workspace targets and unit/integration tests passed).

### Component Audit Observations
1. **GaeGo Unsealing and Hidden Stem Scoring in `SajuVM` (`crates/eon-saju/src/engine/vm.rs`)**:
   - `vm.rs` lines 556–563: `mem_dump` checks `is_gaego_unsealed = dynamic.gaego_events.iter().any(|ev| ev.branch == *b && ev.unsealed_stems.contains(&stem))`. If true, `mem_dump` skips scoring the stem to prevent double counting.
   - Section 6.7 (lines 671–694): `gaego_events` iterates unsealed stems and applies `bonus = 10.0 * self.config.vm.memory_dump_weight * weight`, updating `registers` and writing `gaego:` tags.
   - Single-counting invariant verified: no unsealed stem is double-counted in both `mem_dump:` and `gaego:`.

2. **IpMyo Trapping for Yin Day Masters (乙, 丁, 辛, 癸) (`crates/eon-saju/src/analysis/dynamic_luck.rs`)**:
   - `dynamic_luck.rs` lines 476–494: `evaluate_ipmyo_events` checks `stage == TwelveStage::Mu` against Day Master `dm`.
   - `let trapped_el = dm.element()` guarantees that trapped elements for Yin Day Masters (乙 -> Wood, 丁 -> Fire, 辛 -> Metal, 癸 -> Water) match the Day Master's own element rather than the tomb branch's Earth element. Verified across all 10 Heavenly Stems in `test_stress_ipmyo_trapped_element_all_10_stems`.

3. **Gyeokguk State Transitions (`Fulfilled`, `Transformed`, `Broken`) (`crates/eon-saju/src/analysis/dynamic_luck.rs`)**:
   - `dynamic_luck.rs` lines 501–563: `evaluate_structure_state` prioritizes:
     - `Transformed` (변격) when `triple_combinations` or `seasonal_combinations` complete.
     - `Fulfilled` (성격) when a luck stem matches a month branch hidden stem and is not `Bijian` (比肩) or `Jiecai` (劫財).
     - `Broken` (파격) when `combined.branch_clashes` contains a clash on the month branch (`"월지"`).
   - Verified exclusions of `Bijian`/`Jiecai` stems in `test_stress_gyeokguk_fulfillment_exclusion_of_bijian_jiecai`.
   - Verified state machine transitions in `test_stress_gyeokguk_state_machine_coverage`.

4. **New Defect Uncovered — Alliance Suppression Position Isolation in `dynamic_luck.rs`**:
   - Code location: `crates/eon-saju/src/analysis/dynamic_luck.rs` lines 314–335.
   - Verbatim code snippet:
     ```rust
     let mut alliance_branches = std::collections::HashSet::new();
     for tc in &analysis.triple_combinations {
         for b in tc.branches() {
             alliance_branches.insert(b);
         }
     }
     for sc in &analysis.seasonal_combinations {
         for b in sc.branches() {
             alliance_branches.insert(b);
         }
     }

     if !alliance_branches.is_empty() {
         analysis.six_combinations.retain(|(six, _, _)| {
             let (b1, b2) = six.branches();
             !alliance_branches.contains(&b1) && !alliance_branches.contains(&b2)
         });
         analysis.branch_clashes.retain(|(clash, _, _)| {
             let (b1, b2) = clash.branches();
             !alliance_branches.contains(&b1) && !alliance_branches.contains(&b2)
         });
     }
     ```
   - Observed behavior: `alliance_branches` stores `HashSet<EarthlyBranch>` without pillar position tracking. When a chart has duplicate instances of an `EarthlyBranch` (e.g. Day 辰 in 申子辰 Triple Alliance, and Hour 辰 not in the alliance), `alliance_branches.contains(&EarthlyBranch::Chen)` returns `true` for the non-alliance Hour 辰. This erroneously suppresses valid clashes involving Hour 辰 (e.g., Major Luck 戌 vs Hour 辰).

---

## 2. Logic Chain

1. **GaeGo Single-Counting**:
   - Observation: In `vm.rs`, `mem_dump` checks `is_gaego_unsealed` against `dynamic.gaego_events`.
   - Deduction: Stems marked as unsealed by GaeGo skip `mem_dump` and are evaluated exclusively in Section 6.7 (`gaego`).
   - Conclusion: GaeGo double-counting has been successfully remediated.

2. **IpMyo Trapped Element Matching**:
   - Observation: `evaluate_ipmyo_events` assigns `trapped_el = dm.element()`.
   - Deduction: For Yin Day Masters (乙, 丁, 辛, 癸), `dm.element()` evaluates to Wood, Fire, Metal, and Water respectively.
   - Conclusion: IpMyo trapping for Yin Day Masters correctly matches the Day Master's element.

3. **Gyeokguk State Transitions**:
   - Observation: `evaluate_structure_state` explicitly checks `!matches!(god, TenGod::Bijian | TenGod::Jiecai)` before setting `status = Fulfilled`, and checks month branch clashes (`"월지"`) to apply `Broken`.
   - Deduction: Neither `Bijian` nor `Jiecai` can trigger Gyeokguk fulfillment, and month branch clashes correctly force `Broken` status.
   - Conclusion: Gyeokguk state transitions operate as designed.

4. **Alliance Branch Over-Suppression Flaw**:
   - Observation: In `dynamic_luck.rs:314`, `alliance_branches` is a `HashSet<EarthlyBranch>`.
   - Deduction: When a chart contains multiple pillars with the same `EarthlyBranch` (e.g. Day 辰 and Hour 辰), and only one pillar (Day 辰) participates in a Triple/Seasonal alliance, `alliance_branches` records `EarthlyBranch::Chen`. The filter `.retain(...)` checks `!alliance_branches.contains(&b1) && !alliance_branches.contains(&b2)`. Because `b1 = EarthlyBranch::Chen` for Hour 辰, the filter evaluates to `false` and removes the clash for Hour 辰.
   - Conclusion: Valid clashes on non-alliance duplicate branches are wrongly over-suppressed. To fix, `alliance_branches` must store position-qualified identifiers `(position_str, branch)` rather than bare `EarthlyBranch` variants.

---

## 3. Caveats

- **Scope Limit**: The over-suppression flaw occurs in edge case charts with duplicate branch characters where one branch belongs to an alliance and another does not. Standard charts with distinct branch characters are unaffected.
- **Implementation Code**: No implementation files in `crates/eon-saju/src` were modified during this challenge (adhering to review-only constraints).

---

## 4. Conclusion

- **Remediation Status**: The three target features (GaeGo unsealing single-counting, IpMyo Yin Day Master trapped element matching, and Gyeokguk state transitions) are **FULLY REMEDIATED** and verified by empirical test suites.
- **New Finding**: A secondary defect in `dynamic_luck.rs` precedence hierarchy was identified where `alliance_branches` uses unpositioned `HashSet<EarthlyBranch>`, leading to over-suppression of clashes on duplicate branches outside the alliance.

---

## 5. Verification Method

To independently verify all findings, run the following commands from the repository root (`/Users/sjkim1127/Eon`):

```bash
# 1. Run targeted challenger verification suite
cargo test --test challenger_m2_2_verify -- --nocapture

# 2. Run comprehensive remediation stress test suite (includes alliance position isolation & Gyeokguk state machine)
cargo test --test challenger_m2_remediation_stress -- --nocapture

# 3. Run full workspace test suite
cargo test --workspace
```
