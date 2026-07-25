## 2026-07-24T18:50:46Z

You are the Remediation Worker for Milestone 2 (R2) of the Saju Core Engine (`crates/eon-saju`) Enhancement Project.

Working Directory: /Users/sjkim1127/Eon/.agents/worker_remediation_m2
Project Root: /Users/sjkim1127/Eon

MANDATORY INTEGRITY WARNING:
DO NOT CHEAT. All implementations must be genuine. DO NOT hardcode test results, create dummy/facade implementations, or circumvent the intended task. A Forensic Auditor will independently verify your work. Integrity violations WILL be detected and your work WILL be rejected.

Your Task:
Remediate the 6 specific logic bugs identified in Milestone 2 in `crates/eon-saju`:

1. Wolwun Pre-XiaoHan Year Shift (`crates/eon-saju/src/analysis/periodic_luck.rs`):
   In `month_ganzi_at`, when `dt` is between Jan 1 and XiaoHan entry (24-solar term index 21, DongZhi), `saju_year` must be evaluated as `dt_year - 1` (since it is before LiChun). Currently, solar term index 21 falls through to `dt_year`.

2. Non-Earth Branch Mapping in `calculate_expanded` (`crates/eon-saju/src/analysis/power.rs`):
   In `calculate_expanded` (lines ~282-294), `match orig_el { Element::Earth => EarthlyBranch::Wei, _ => EarthlyBranch::Zi }` maps all non-Earth branches to `Zi` (Water) when `options.apply_correction` is true, corrupting branch element powers into Water.
   Fix: Store `Option<EarthlyBranch>` in the items tuple so `apply_climate_correction` receives the actual `EarthlyBranch` (and respects transformed element `eff_el`).

3. Winter Earth Climate Correction (`crates/eon-saju/src/analysis/power.rs`):
   By passing the actual `EarthlyBranch` to `apply_climate_correction`, ensure `EarthlyBranch::Chou` (丑) in winter months (`Hai`/`Zi`) is evaluated as `Chou` so it properly absorbs Water energy, rather than being treated as `Wei`.

4. GaeGo Double-Scoring (`crates/eon-saju/src/engine/vm.rs`):
   In `SajuVM::evaluate_score`, hidden stems were being scored twice: once in Section 3 (`mem_dump`) and again in Section 6.7 (`gaego`). Eliminate duplicate register scoring during clash memory dumps when GaeGo events execute.

5. IpMyo Element Matching for Yin Day Masters (`crates/eon-saju/src/analysis/dynamic_luck.rs`):
   In `evaluate_ipmyo_events`, `match g.branch` hardcoded tomb branches to fixed trapped elements (e.g., 戌 -> Fire). For Yin Day Masters (乙 Wood, 丁 Fire, 辛 Metal, 癸 Water), 12-Unseong `Mu` (墓) occurs at different tomb branches. Set `trapped_el` to the Day Master's element `dm.element()` rather than a static branch tomb lookup.

6. Gyeokguk Fulfillment Filter (`crates/eon-saju/src/analysis/dynamic_luck.rs`):
   In `evaluate_structure_state`, checking `hidden_stems().contains(&s)` was triggering `GyeokStatus::Fulfilled` for non-structural ten gods like 劫財 (Rob Wealth) or 比肩 (Friend). Restrict `Fulfilled` transitions to valid structural gods (excluding 劫財/比肩). Also ensure `evaluate_gaego_events` requires dynamic luck interactions for GaeGo triggers.

Verification:
- Run `cargo check --workspace`
- Run `cargo test --workspace` (must pass 100%, including `milestone2_stress_tests` and `challenger_m2_2_verify`).
- Write handoff report with exact test command outputs to `/Users/sjkim1127/Eon/.agents/worker_remediation_m2/handoff.md`.
- Notify parent orchestrator when complete.
