# Project: Saju Core Engine Enhancement (`crates/eon-saju`)

## Architecture
- `crates/eon-saju`: Saju analysis engine (VM, Yongsin, Gyeokguk, 12 Unseong, Sinsal, Dynamic Luck/Daewun/Saewun/Wolwun simulation, Edge-case verification).
- `crates/eon-service`: Unified façade and DTOs exposed to Dioxus frontend UI.
- `crates/eon-core`: Core primitives (Stems, Branches, Elements, Five Elements, etc.).

## Milestones
| # | Name | Scope | Dependencies | Status |
|---|------|-------|-------------|--------|
| 1 | Core Analysis Precision & Pattern Completeness | Eokbu/JoHu/TongGwan/ByeongYak Yongshin  , Gyeokguk/12/      Gyeokguk   (R1) | none | DONE |
| 2 | Dynamic Luck & Temporal Simulation Engine | Major Luck/Yearly Luck/Monthly Luck     Hidden Stem/     (R2) | M1 | DONE |
| 3 | Codebase Architecture & Edge-Case Verification Suite | `crates/eon-saju`  /      /  suite  (R3) | M1, M2 | IN_PROGRESS |


## Interface Contracts
- `crates/eon-saju`: internal module APIs and public engine exports.
- `crates/eon-service`: DTO constructors and facade functions (`analyze_saju`, etc.).

## Code Layout
- `crates/eon-saju/src/`: Core engine code.
- `crates/eon-saju/tests/`: Integration tests for Saju natal charts & dynamic simulation.
- `crates/eon-service/src/`: Façade layer connecting UI/Wasm to `eon-saju`.
