# Project: Eon Vedic Astrology Enhancements

## Architecture
Eon is a single-language Rust application running Dioxus Web on the frontend and executing core engines directly via WASM in the browser.
- **`crates/eon-vedic`**: Core engine that calculates Vedic astrology features.
  - `analysis/matching.rs`: Compatibility engine (Ashtakoota).
  - `analysis/strength.rs`: Planetary strength (Shadbala).
  - `analysis/kp.rs`: KP System (cusps, lords, significators).
  - `analysis/dasha.rs`: Dasha timelines (Vimshottari, Yogini).
- **`crates/eon-service`**: Façade & DTO definitions. Translates domain types to presentation-friendly outputs.
- **`crates/eon-ui`**: Dioxus-based SPA frontend. Computations are triggered from components asynchronously and update `AnalysisState`.
- **`crates/eon-ui/src/i18n`**: Zero-dependency compile-time localization system.

## Milestones
| # | Name | Scope | Dependencies | Status | Conv ID |
|---|---|---|---|---|---|
| M1 | E2E Testing Framework & Setup | Set up initial integration tests and E2E verification files (`TEST_READY.md`, `TEST_INFRA.md`). Enumerate 4-tier test case structures. | None | DONE | 9fe611dd-1402-4089-888a-62fce40a2d5b |
| M2 | R1 Ashtakoota Guna Milan | Enhance engine to return 8 components details; visual progress/gauge component in Compatibility tab with KO/EN/ZH/RU translations. | M1 | DONE | 125d66a6-538c-4ffc-a542-9cafb9511739 |
| M3 | R2 Shadbala 6대 강도 | Refactor engine to compute 6 standard Shadbala factors; grid scorecard in Strength tab, Rupa benchmark comparison, KO/EN/ZH/RU translations. | M2 | IN_PROGRESS | 2edcdc61-005a-4a04-ba79-d86d8deaedde |
| M4 | R3 KP System Lords & Significators | Refactor KP engine to include sign/star/sub lords and significator table; render table in KP subtab, integrate translation keys for KO/EN/ZH/RU. | M3 | PLANNED | 2edcdc61-005a-4a04-ba79-d86d8deaedde |
| M5 | R4 Hierarchical Dasha Timeline | Implement hierarchical Maha -> Antar -> Pratyantar accordion UI in Dasha tab; dynamic current-period highlighting, KO/EN/ZH/RU translations. | M4 | PLANNED | 2edcdc61-005a-4a04-ba79-d86d8deaedde |
| M6 | Final Verification & Hardening | Run all E2E tests, execute Phase 2 (Adversarial Coverage Hardening), and Forensic Audit check. | M1-M5 | PLANNED | - |

## Interface Contracts
### `eon-vedic` ↔ `eon-service` ↔ `eon-ui`
- **Ashtakoota Compatibility**:
  - `CompatibilityReport` will continue to be returned by `MatchingEngine::calculate_compatibility`.
  - Translations will be handled at the UI layer by mapping names to static `TK` enum values.
- **Shadbala 6 Factors**:
  - Add fields to `eon_vedic::analysis::strength::PlanetStrength`:
    - `sthana_bala: f64`
    - `dig_bala: f64`
    - `kala_bala: f64`
    - `chesta_bala: f64`
    - `naisargika_bala: f64`
    - `drik_bala: f64`
  - Ensure these are populated in `StrengthEngine::calculate` and serialized/deserialized seamlessly.
- **KP Cusps & Lords**:
  - `KpPoint` struct in `eon_vedic::analysis::kp` already returns `sign_lord`, `star_lord`, `sub_lord` as `VedicPlanet` enum values.
  - The service exposes this via `KpAnalysis` in `VedicAnalysisOutput`.
- **Dasha Timeline recursion**:
  - The UI will request up to 3 levels from `VimshottariDasha::calculate_timeline` via the service (already supported in engine if levels=3).

## Code Layout
- `crates/eon-vedic/src/analysis/matching.rs` - Compatibility algorithm
- `crates/eon-vedic/src/analysis/strength.rs` - Shadbala calculation
- `crates/eon-vedic/src/analysis/kp.rs` - KP cusps & lords calculations
- `crates/eon-vedic/src/analysis/dasha.rs` - Vimshottari / Yogini dasha calculations
- `crates/eon-service/src/services/vedic.rs` - Main Vedic service facade
- `crates/eon-service/src/dto.rs` - Data transfer objects
- `crates/eon-ui/src/components/tabs/vedic_tab.rs` - UI component for Vedic tab
- `crates/eon-ui/src/i18n/` - i18n modules (`mod.rs`, `ko.rs`, `en.rs`, `zh.rs`, `ru.rs`)
