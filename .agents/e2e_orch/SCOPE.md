# Scope: E2E Testing for Vedic Astrology Enhancements

## Architecture
- The tests are written as integration tests under `crates/eon-vedic/tests/`.
- They will invoke the public API of `eon-vedic` (including `MatchingEngine`, `StrengthEngine`, `KpPoint`/`KpAnalysis`, and `VimshottariDasha`).
- The test harness is opaque-box, meaning it does not rely on internals, only public interfaces and structures.

## Milestones
| # | Name | Scope | Dependencies | Status | Conv ID |
|---|---|---|---|---|---|
| 1 | Test Infrastructure Design | Create `TEST_INFRA.md` at project root specifying the 4-tier plan, feature list, and scenarios. | None | DONE | af99a077-d5d9-49dc-ab10-9199dd8fdc6f |
| 2 | Test Case Development | Write the integration tests in `crates/eon-vedic/tests/compatibility_shadbala_kp_dasha.rs` covering Tiers 1-4. | M1 | DONE | a6d60a44-0c72-462e-95b4-45645bbfbb03 |
| 3 | Test Verification & Release | Verify tests compile and pass using cargo, fix issues, publish `TEST_READY.md`. | M2 | DONE | 9f2fb9d0-f517-4fc1-ae90-fb8a8c4adefc |
