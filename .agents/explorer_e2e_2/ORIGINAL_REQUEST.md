## 2026-06-20T05:59:54Z
You are an Explorer agent. Your working directory is /Users/sjkim1127/Eon/.agents/explorer_e2e_2.
Your task is to explore the existing codebase of `eon-vedic` and `eon-service` to understand the public APIs for:
- KP system cusps, lords, and significators (e.g., `KpPoint`, `VedicAnalysisOutput`, etc.)
- Vimshottari and Yogini dasha timelines (e.g., `VimshottariDasha::calculate_timeline`)

Examine how integration tests are currently structured in `crates/eon-vedic/tests/` (e.g., vimshopaka_oracle_verify.rs, basic.rs) to see how charts are set up, what helper functions are used, and how dates/times/locations are constructed.
Provide a detailed report outlining:
1. The exact module structure, public types, and function signatures that we need to call for these features.
2. How existing tests initialize charts or input data (e.g., date, time, location, ayanamsa).
3. A proposal for the specific test cases we should design for Tier 1, Tier 2, Tier 3, and Tier 4 regarding KP System and Dashas.

Write your report to `/Users/sjkim1127/Eon/.agents/explorer_e2e_2/handoff.md`.
