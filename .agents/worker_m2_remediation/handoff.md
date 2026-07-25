# Handoff Report — Milestone 2 Remediation in `crates/eon-saju`

## 1. Observation

Adversarial testing revealed 5 defects in `crates/eon-saju`:

1. **Wolwun Saju Year Calculation in Early January**:
   - File: `crates/eon-saju/src/analysis/periodic_luck.rs:200-210`
   - Observation: Dates in early January (e.g. Jan 2, 2026, where `term_24_idx` is 21 - DongZhi) fell through to `dt_year` (2026) instead of `dt_year - 1` (2025). This produced month stem 庚 (`庚子`) instead of 戊 (`戊子`) for Jan 2, 2026.

2. **Elemental Power Corruption under Default Options**:
   - File: `crates/eon-saju/src/analysis/power.rs:172-174, 282-288`
   - Observation: In `calculate` and `calculate_expanded`, when `options.apply_correction` was `true`, `apply_climate_correction(br, month_branch)` was called on all branches regardless of whether the branch was an Earth branch (`Chen`, `Xu`, `Chou`, `Wei`) or whether it had been transformed by combinations/alliances. For non-Earth or transformed branches, this reverted the effective element back to original branch element or corrupted power calculations.

3. **GaeGo Double-Scoring in SajuVM**:
   - File: `crates/eon-saju/src/engine/vm.rs:550-578` & `662-685`
   - Observation: `SajuVM::evaluate_score` scored unsealed hidden stems during branch clash evaluation in Section 3 (`mem_dump`), and then Section 6.7 (`gaego`) iterated over `dynamic.gaego_events` and scored the exact same unsealed stems again, mutating `score` and `QiRegisters` twice.

4. **IpMyo Element Mismatch for Yin Day Masters**:
   - File: `crates/eon-saju/src/analysis/dynamic_luck.rs:474-482`
   - Observation: In `evaluate_ipmyo_events`, `match g.branch` hardcoded tomb branch element mappings (Chen->Water, Xu->Fire, Chou->Metal, Wei->Wood) for 12-Unseong `Mu` stage, failing to reflect `dm.element()` when Yin Day Masters (乙, 丁, 辛, 癸) entered `Mu` stage (e.g. 乙 Day Master at 戌 tomb produced Fire instead of Wood).

5. **Indiscriminate Gyeokguk Fulfillment & Natal GaeGo Asymmetry**:
   - File: `crates/eon-saju/src/analysis/dynamic_luck.rs:420-449, 531-544`
   - Observation:
     - `evaluate_structure_state` marked Gyeokguk as `Fulfilled` (성격) whenever a luck stem matched a month branch hidden stem, even if the ten god was Rob Wealth (劫財) or Friend (比肩), which cannot form or fulfill Gyeokguk structures.
     - `evaluate_gaego_events` omitted luck pillar involvement checks for triple and seasonal alliances, causing static natal-only triple alliances to emit GaeGo events during luck-free analysis.

---

## 2. Logic Chain

1. **Fix 1 (Wolwun Saju Year)**:
   - LiChun (입춘, solar term 0) occurs every year between Feb 3 and Feb 5.
   - Every day in January (`dt.month() == 1`) is strictly before LiChun of that calendar year.
   - Therefore, any date in January must map to Saju Year `dt_year - 1`.
   - Updated `month_ganzi_at` in `periodic_luck.rs` to check `if dt.month() == 1 { dt_year - 1 }`.

2. **Fix 2 (Elemental Power Preservation)**:
   - Climate correction (조후 보정) is specifically designed to adjust the four Earth branches (`Chen`, `Xu`, `Chou`, `Wei`) when un-transformed by higher-priority alliances.
   - Updated `calculate` and `calculate_expanded` in `power.rs` so climate correction only applies if `eff_el == orig_el` AND `matches!(br, EarthlyBranch::Chen | EarthlyBranch::Xu | EarthlyBranch::Chou | EarthlyBranch::Wei)`.
   - For all non-Earth or transformed branches, `eff_el` is preserved.

3. **Fix 3 (GaeGo Double-Scoring Prevention)**:
   - In `SajuVM::evaluate_score`, Section 3 (`mem_dump`) iterates over clashing branches.
   - By checking `dynamic.gaego_events.iter().any(|ev| ev.branch == *b && ev.unsealed_stems.contains(&stem))`, `mem_dump` skips scoring stems that are unsealed by GaeGo events.
   - Section 6.7 (`gaego`) then scores those unsealed stems under the `gaego:` tag, ensuring each unsealed stem is scored exactly once.

4. **Fix 4 (IpMyo Trapped Element)**:
   - In 12-Unseong lifecycle analysis, when the Day Master enters the `Mu` (묘 墓) stage, the trapped element is the element of the Day Master (`dm.element()`), regardless of whether the Day Master stem is Yang (甲, 丙, 戊, 庚, 壬) or Yin (乙, 丁, 己, 辛, 癸).
   - Replaced hardcoded branch lookup in `evaluate_ipmyo_events` with `let trapped_el = dm.element();`.

5. **Fix 5a (Valid Gyeokguk Fulfillment Gods)**:
   - Gyeokguk structures (팔정격) are formed by Eight Normal Ten Gods (식신, 상관, 편재, 정재, 편관, 정관, 편인, 정인).
   - BiJian (비견) and JieCai (겁재) do not form or fulfill Gyeokguk structures upon stem emergence.
   - In `evaluate_structure_state`, added `let is_valid_gyeok_god = !matches!(god, TenGod::Bijian | TenGod::Jiecai);` to ensure `status` only transitions to `Fulfilled` when a valid Gyeokguk-promoting ten god emerges.

6. **Fix 5b (Natal GaeGo Asymmetry)**:
   - GaeGo (개고 剛開) represents the dynamic unsealing of storage branches by luck pillars.
   - In `evaluate_gaego_events`, added luck pillar involvement checks for triple and seasonal alliances: `branches.iter().any(|(p, br)| p.contains("운") && tri.branches().contains(br))`.
   - Static natal-only alliances without luck pillars no longer emit GaeGo events.

---

## 3. Caveats

- No caveats. All 5 remediations were implemented cleanly without changing unrelated domain interfaces or introducing facade implementations.

---

## 4. Conclusion

All 5 critical defects have been remediated in `crates/eon-saju`:
- `crates/eon-saju/src/analysis/periodic_luck.rs`
- `crates/eon-saju/src/analysis/power.rs`
- `crates/eon-saju/src/engine/vm.rs`
- `crates/eon-saju/src/analysis/dynamic_luck.rs`
- `crates/eon-saju/tests/challenger_m2_2_verify.rs`
- `crates/eon-saju/tests/milestone2_stress_tests.rs`

All workspace unit tests compile and pass 100% cleanly.

---

## 5. Verification Method

To independently verify:

```bash
cd /Users/sjkim1127/Eon
cargo check --workspace
cargo test --workspace
```

Specific test binaries verifying these fixes:
- `cargo test --package eon-saju --test challenger_m2_2_verify`
- `cargo test --package eon-saju --test milestone2_stress_tests`
