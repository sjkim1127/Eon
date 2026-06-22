# Scope: Implementation Track for Vedic Astrology Enhancements

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
| M2 | R1 Ashtakoota Guna Milan | Enhance engine to return 8 components details; visual progress/gauge component in Compatibility tab with KO/EN/ZH/RU translations. | None | DONE | bbf398d6-2705-4e72-8fb3-32991a24a296, 6d5786f5-dc5b-4f25-b0df-2d6fed4fbf2d, 77fd3370-078c-4b8e-bac4-225afa02c24b, b659b762-047e-4824-9e33-bc34ea55ca48 |
| M3 | R2 Shadbala 6대 강도 | Refactor engine to compute 6 standard Shadbala factors; grid scorecard in Strength tab, Rupa benchmark comparison, KO/EN/ZH/RU translations. | M2 | IN_PROGRESS | - |
| M4 | R3 KP System Lords & Significators | Refactor KP engine to include sign/star/sub lords and significator table; render table in KP subtab, integrate translation keys for KO/EN/ZH/RU. | M3 | PLANNED | - |
| M5 | R4 Hierarchical Dasha Timeline | Implement hierarchical Maha -> Antar -> Pratyantar accordion UI in Dasha tab; dynamic current-period highlighting, KO/EN/ZH/RU translations. | M4 | PLANNED | - |

## Interface Contracts
### `eon-vedic` ↔ `eon-service` ↔ `eon-ui`
- **Ashtakoota Compatibility**:
  - `CompatibilityReport` will continue to be returned by `MatchingEngine::calculate_compatibility`.
  - Translations will be handled at the UI layer by mapping names to static `TK` enum values.
- **Shadbala 6 Factors**:
  - Add fields to `eon_vedic::analysis::strength::PlanetStrength`:
    - `sthana_bala: f64`, `dig_bala: f64`, `kala_bala: f64`, `chesta_bala: f64`, `naisargika_bala: f64`, `drik_bala: f64`
  - Ensure these are populated in `StrengthEngine::calculate` and serialized/deserialized seamlessly.
- **KP Cusps & Lords**:
  - `KpPoint` struct in `eon_vedic::analysis::kp` already returns `sign_lord`, `star_lord`, `sub_lord` as `VedicPlanet` enum values.
  - The service exposes this via `KpAnalysis` in `VedicAnalysisOutput`.
- **Dasha Timeline recursion**:
  - The UI will request up to 3 levels from `VimshottariDasha::calculate_timeline` via the service.
