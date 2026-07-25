## 2026-07-24T18:48:44Z
You are assigned to remediate 5 critical defects discovered during adversarial testing of Milestone 2 (R2: Dynamic Luck & Temporal Simulation Engine) in `crates/eon-saju`.

Working Directory: /Users/sjkim1127/Eon/.agents/worker_m2_remediation
Project Root: /Users/sjkim1127/Eon

MANDATORY INTEGRITY WARNING:
DO NOT CHEAT. All implementations must be genuine. DO NOT hardcode test results, create dummy/facade implementations, or circumvent the intended task. A Forensic Auditor will independently verify your work. Integrity violations WILL be detected and your work WILL be rejected.

Please read `AGENTS.md` in the project root first before making any changes.

Here are the 5 specific defects to remediate in `crates/eon-saju`:

1. **Wolwun Saju Year Miscalculation in Early January Before XiaoHan** (`crates/eon-saju/src/analysis/periodic_luck.rs`):
   - Problem: In `month_ganzi_at`, when `term_24_idx` is `21` (DongZhi, e.g. Jan 1-5), the date is before LiChun 2026, but the code falls through to `dt_year` instead of `dt_year - 1`. Jan 2, 2026 must be in Saju Year 2025 (乙巳), returning month stem 戊 (`戊子`), NOT 庚 (`庚子`).
   - Fix: Correct Saju year determination so any date in January before LiChun maps to `dt_year - 1`.

2. **Elemental Power Corruption in `calculate_expanded` under Default Options** (`crates/eon-saju/src/analysis/power.rs`):
   - Problem: In `calculate_expanded` (lines 282-294), when `options.apply_correction` is `true` (default), for any branch (`!is_stem`), `match orig_el` maps all non-Earth elements (`Wood`, `Fire`, `Metal`, `Water`) to `EarthlyBranch::Zi`, which `apply_climate_correction` converts to Water. This corrupts every non-Earth branch across natal and luck pillars into Water.
   - Fix: Ensure `apply_climate_correction` correctly passes the actual branch / effective element (`eff_el`) or proper branch representation rather than hardcoding `Zi` for non-Earth elements.

3. **GaeGo Double-Scoring in `SajuVM`** (`crates/eon-saju/src/engine/vm.rs`):
   - Problem: In `SajuVM::evaluate_score`, Section 3 (`mem_dump`) scores hidden stems on branch clashes, and Section 6.7 (`gaego`) iterates over `dynamic.gaego_events` and scores the exact same unsealed hidden stems again, mutating scores and registers twice.
   - Fix: Prevent double scoring of unsealed stems between clash evaluation (`mem_dump`) and GaeGo event scoring (`gaego`).

4. **IpMyo Element Mismatch for Yin Day Masters** (`crates/eon-saju/src/analysis/dynamic_luck.rs`):
   - Problem: In `evaluate_ipmyo_events` (lines 474-482), `match g.branch` hardcodes tomb branch lookup (Chen->Water, Xu->Fire, Chou->Metal, Wei->Wood) without respecting the Day Master's element when Yin stems (乙, 丁, 辛, 癸) enter 12-Unseong `Mu` stage.
   - Fix: Determine the trapped element based on the Day Master's element (`dm.element()`) or proper 12-Unseong element trapping rules.

5. **Indiscriminate Gyeokguk Fulfillment & Natal GaeGo Asymmetry** (`crates/eon-saju/src/analysis/dynamic_luck.rs`):
   - Problem:
     a) `evaluate_structure_state` marks Gyeokguk as `Fulfilled` (성격) for any matching hidden stem including Rob Wealth (劫財) and Friend (比肩), which do not form or fulfill Gyeokguk structures. Ensure only valid Gyeokguk-promoting ten gods trigger `Fulfilled`.
     b) `evaluate_gaego_events` checks luck pillar involvement for clashes but omits it for triple/seasonal alliances, causing static natal-only triple alliances to emit GaeGo events during luck-free analysis. Require luck pillar involvement for triple/seasonal GaeGo triggers.
