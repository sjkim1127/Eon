## 2026-07-24T18:27:50Z

<USER_REQUEST>
You are the Remediation Worker for Milestone 1 (R1) in `crates/eon-saju`.

Read `AGENTS.md` at `/Users/sjkim1127/Eon/AGENTS.md` first.
Read the Challenger reports at:
- `/Users/sjkim1127/Eon/.agents/teamwork_preview_challenger_m1_1/handoff.md`
- `/Users/sjkim1127/Eon/.agents/teamwork_preview_challenger_m1_2/handoff.md`

Fix the following 4 specific bugs:
1. **Fix `DeukSe` count calculation in `crates/eon-saju/src/analysis/strength.rs` (lines 416–431)**:
   In `DeukSe::check_with_options`, check `if percentage > 0.0` before incrementing `shishang_count`, `caisheng_count`, `guanxing_count`, `bijie_count`, and `yinxing_count`.
2. **Fix Follower Pattern Selection in `crates/eon-saju/src/analysis/structure.rs` (lines 340–344)**:
   Compare actual TenGod energy percentages (`shishang_power`, `cai_power`, `guan_power`) or corrected `DeukSe` counts so `JongJae` / `GaJongJae` (재성종격/가종재격) and `JongSal` / `GaJongSal` (관살종격/가종살격) are reachable when Caisheng or Guanxing is dominant.
3. **Fix False-Positive Void Dissolution in `crates/eon-saju/src/analysis/void.rs` (lines 164–175)**:
   In `check_void_dissolution`, check that `target_branch` is actually contained in the detected Triple or Seasonal combination before marking `is_dissolved = true`.
4. **Fix Spirit Marker Position String Mismatch in `crates/eon-saju/src/analysis/spirit_markers.rs` (lines 719–730)**:
   Map `m.position` to branch position strings (`"년지"`, `"월지"`, `"일지"`, `"시지"`) matching `rel_analysis` branch tuple position strings, enabling `is_clashed` / `is_combined` detection and restoring voided noble spirits (`(공망해충/해합 구원)`).

MANDATORY INTEGRITY WARNING:
DO NOT CHEAT. All implementations must be genuine. DO NOT hardcode test results, create dummy/facade implementations, or circumvent the intended task. A Forensic Auditor will independently verify your work. Integrity violations WILL be detected and your work WILL be rejected.

After completing code modifications:
- Run `cargo check --workspace` to ensure zero compilation errors and warnings.
- Run `cargo test -p eon-saju` to verify all tests (including `milestone1_stress_tests` and `milestone1_part2_stress_tests`) pass 100%.
- Write your handoff report to `/Users/sjkim1127/Eon/.agents/worker_remediation_m1/handoff.md`.
- Send completion message back to parent orchestrator.
</USER_REQUEST>
