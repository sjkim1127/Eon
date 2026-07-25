# Project: Saju Core Engine Enhancement (`crates/eon-saju`)

## Architecture
- `crates/eon-saju`: Saju analysis engine (VM, Yongsin, Gyeokguk, 12 Unseong, Sinsal, Dynamic Luck/Daewun/Saewun/Wolwun simulation, Edge-case verification).
- `crates/eon-service`: Unified façade and DTOs exposed to Dioxus frontend UI.
- `crates/eon-core`: Core primitives (Stems, Branches, Elements, Five Elements, etc.).

## Milestones
| # | Name | Scope | Dependencies | Status |
|---|------|-------|-------------|--------|
| 1 | Core Analysis Precision & Pattern Completeness | 억부/조후/통관/병약 용신 정밀 산출, 격국/12운성/신살 예외 보완 및 고급 특수 격국 패턴 완비 (R1) | none | DONE |
| 2 | Dynamic Luck & Temporal Simulation Engine | 대운/세운/월운 시기별 동적 운세 및 지장간/충합 변형 시뮬레이션 엔진 보강 (R2) | M1 | DONE |
| 3 | Codebase Architecture & Edge-Case Verification Suite | `crates/eon-saju` 모듈 구조/성능 최적화 및 다양한 명조 엣지케이스 단위/통합 테스트 suite 확충 (R3) | M1, M2 | IN_PROGRESS |


## Interface Contracts
- `crates/eon-saju`: internal module APIs and public engine exports.
- `crates/eon-service`: DTO constructors and facade functions (`analyze_saju`, etc.).

## Code Layout
- `crates/eon-saju/src/`: Core engine code.
- `crates/eon-saju/tests/`: Integration tests for Saju natal charts & dynamic simulation.
- `crates/eon-service/src/`: Façade layer connecting UI/Wasm to `eon-saju`.
