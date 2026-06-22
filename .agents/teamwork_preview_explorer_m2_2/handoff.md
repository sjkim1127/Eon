# Handoff Report: Milestone M2 (R1: Ashtakoota Guna Milan 상세 고도화)

## 1. Observation
- **Vedic Matching Calculations**:
  In `crates/eon-vedic/src/analysis/matching.rs` (lines 62-111):
  ```rust
  let kootas = vec![
      KootaScore {
          name: "Varna (Caste/Vocation)".to_string(),
          max_points: 1.0,
          earned_points: varna_score,
          description: if varna_score == 1.0 { "Good work-profile alignment.".to_string() } else { "Differing natural vocations.".to_string() },
      },
      ...
  ```
  The name, max points, and description are hardcoded in English.
- **Vedic Compatibility Service & DTOs**:
  In `crates/eon-service/src/services/vedic.rs` (lines 92-122) and `crates/eon-service/src/dto.rs` (lines 180-185), compatibility results are routed via:
  ```rust
  pub struct VedicCompatibilityOutput {
      pub meta: AnalysisMeta,
      pub report: eon_vedic::analysis::matching::CompatibilityReport,
  }
  ```
- **Dioxus Tab Rendering**:
  In `crates/eon-ui/src/components/tabs/vedic_tab.rs` (lines 2142-2175), compatibility results are rendered as a plain table using raw engine strings:
  ```rust
  td { class: "px-4 py-3 font-semibold text-slate-300", "{k.name}" }
  td { class: "px-4 py-3 text-center font-mono text-slate-500", "{k.max_points:.1}" }
  ...
  td { class: "px-4 py-3 text-xs text-slate-400", "{k.description}" }
  ```
- **i18n Translation Files**:
  In `crates/eon-ui/src/i18n/`, `ko.rs`, `en.rs`, `zh.rs`, and `ru.rs` contain the compile-time translation lookup for UI elements, but none of the compatibility factors or description strings are currently included.

## 2. Logic Chain
1. To display 8 factors using progress bars showing earned points out of maximum points (Milestone M2 R1 Requirement 4), we must replace the `Ashtakoota Scorecard Table` (lines 2142-2175) with a card-based grid layout where progress percentage is calculated as `k.earned_points / k.max_points`.
2. To translate the factors and their dynamic descriptions across Korean, English, Chinese, and Russian (Milestone M2 R1 Requirement 5), we need translation keys (`TK`) in `i18n/mod.rs` and their translations in `ko.rs`, `en.rs`, `zh.rs`, and `ru.rs`.
3. Since the backend returns static English strings for `name` and description patterns, we can dynamically match `k.name` and the content of `k.description` on the frontend to map them to the correct `TK` translation keys.
4. This frontend translation mapping prevents having to refactor the engine serialization structures or adding locale parameters to the backend calculations.

## 3. Caveats
- **Description Matching**: If backend developers change the exact phrasing of the hardcoded English descriptions in `crates/eon-vedic/src/analysis/matching.rs`, the frontend string matching logic (e.g. `k.name.starts_with(...)` or `desc.contains(...)`) will fail back to rendering raw English strings.
- **Translation Completeness**: The translations provided are based on standardized Vedic astrological descriptions, but they should be reviewed by localizers for natural phrasing.

## 4. Conclusion
We have completed the investigation and designed the UI layout and i18n dictionary for Milestone M2 R1. The proposed changes are fully documented in `analysis.md` and are ready for implementation by the implementer agent.

## 5. Verification Method
- **Compilation Check**:
  Run `cargo check --workspace` to ensure no workspace-wide compilation errors.
- **Engine Tests**:
  Run `cargo test --package eon-vedic` to check that the compatibility calculation logic passes all tests.
- **UI File Check**:
  Inspect `crates/eon-ui/src/components/tabs/vedic_tab.rs` to verify that lines 2087-2185 match the target replacement block in the analysis report.
