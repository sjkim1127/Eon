# Handoff Report — M2 R1 Ashtakoota Guna Milan Investigation

## 1. Observation

Direct observations made during the read-only codebase exploration:

### A. Engine Calculations & Structs
* **File Path**: `crates/eon-vedic/src/analysis/matching.rs`
* **Line 7-12**: `KootaScore` struct definition:
  ```rust
  pub struct KootaScore {
      pub name: String,
      pub max_points: f64,
      pub earned_points: f64,
      pub description: String,
  }
  ```
  Note: This struct lacks an `id` or tag field, forcing the UI to rely on parsing the English string `name` or `description` to identify which of the 8 Kootas is being evaluated.
* **Line 62-111**: The `MatchingEngine::calculate_compatibility` function constructs a vector of `KootaScore` containing hardcoded English names and description outputs, e.g.:
  ```rust
  KootaScore {
      name: "Varna (Caste/Vocation)".to_string(),
      max_points: 1.0,
      earned_points: varna_score,
      description: if varna_score == 1.0 { "Good work-profile alignment.".to_string() } else { "Differing natural vocations.".to_string() },
  }
  ```
* **Line 121-131**: The overall compatibility explanation is built as a static Korean string in the engine:
  ```rust
  let explanation = format!(
      "총 {}점 획득 (36점 만점). {}",
      total,
      if is_compatible {
          "전반적으로 조화로운 매칭입니다. 추천합니다."
      } else if total >= 18.0 {
          "점수는 높으나 주요 살(Nadi/Bhakoot Dosha)의 영향으로 신중한 주의가 필요합니다."
      } else {
          "성향적 차이가 커 상호 조율과 깊은 이해가 요구되는 상성입니다."
      }
  );
  ```

### B. UI Scorecard Rendering
* **File Path**: `crates/eon-ui/src/components/tabs/vedic_tab.rs`
* **Line 2141-2175**: The scorecard table is rendered inside a `Success` task pattern match block:
  ```rust
  // Ashtakoota Scorecard Table
  div { class: "bg-slate-900 border border-slate-800 rounded-2xl overflow-hidden",
      div { class: "bg-slate-800/50 border-b border-slate-800 px-5 py-3",
          h3 { class: "font-semibold text-slate-200", "아쉬타쿠타(Ashtakoota) 세부 매칭 평점표" }
      }
      ...
      tbody { class: "divide-y divide-slate-800",
          {compat.report.kootas.iter().map(|k| rsx! {
              tr { class: "hover:bg-slate-800/20 transition-colors",
                  td { class: "px-4 py-3 font-semibold text-slate-300", "{k.name}" }
                  ...
                  td { class: "px-4 py-3 text-xs text-slate-400", "{k.description}" }
              }
          })}
      }
  ```
* **Line 925-956**: The `run_compatibility` handler runs the backend facade call asynchronously inside a `spawn` block:
  ```rust
  let run_compatibility = move |_| {
      spawn(async move {
          *compat_status.write() = TaskStatus::Loading;
          let input = VedicCompatibilityInput { ... };
          match facade::analyze_vedic_compatibility(input) {
              Ok(res) => {
                  *compat_data.write() = Some(res);
                  *compat_status.write() = TaskStatus::Success;
              }
              ...
          }
      });
  };
  ```

### C. Localization Structure
* **File Path**: `crates/eon-ui/src/i18n/mod.rs`
  * Contains the `Locale` enum (supporting `Ko`, `En`, `Zh`, `Ru`) and the translation key enum `TK`.
* **Translation Files**: `ko.rs`, `en.rs`, `zh.rs`, `ru.rs` within `crates/eon-ui/src/i18n/` implement `translate(key: TK) -> &'static str`.

---

## 2. Logic Chain

1. **Hardcoded Strings Restriction**: In order to translate the 8 components' (Kootas) names and descriptions as well as overall compatibility text into English, Chinese, and Russian (in addition to Korean), we cannot use the raw strings returned by the backend engine (`matching.rs`), because those strings are hardcoded in English/Korean.
2. **Identification of Kootas**: Adding a machine-readable string field `id` (e.g. `"varna"`, `"vashya"`, etc.) to the `KootaScore` struct in the engine allows the UI code (`vedic_tab.rs`) to uniquely and safely identify each Koota component.
3. **UI-Driven Formatting**: Rather than formatting the description text on the backend engine (which doesn't know the user's active locale), the UI should use the `id` and the earned/max points to look up the appropriate translation keys (`TK`) and format the descriptions dynamically. This allows the backend engine to remain pure and locale-agnostic.
4. **Overall Explanation Translation**: The overall compatibility summary statement (`explanation` field) should also be re-assembled on the frontend using locale-specific translation keys to avoid returning Korean explanations to English, Chinese, or Russian users.
5. **Modern Visuals**: Replacing the static text block header with an SVG circular progress gauge representing the earned compatibility score out of 36 will satisfy the "visual progress/gauge component in the Compatibility tab" requirement cleanly within the Dioxus component tree.

---

## 3. Caveats

* **Ayanamsa and Calculation parameters**: The compatibility calculations are driven by the Moon's longitude and Nakshatra placements. The parameters for preparing birth contexts and calculating charts are assumed to be identical to the user's main chart parameters.
* **Compatibility structure compatibility**: Modifying `KootaScore` to add `id: String` requires modifying the struct in `crates/eon-vedic` and its references, which is safe because all parts of the application reside in the same single-language Rust workspace and Dioxus Web invokes `eon-service` directly.

---

## 4. Conclusion

Milestone M2 R1 (Ashtakoota Guna Milan localization and gauge enhancement) can be fully implemented with:
1. Enhancing `KootaScore` with a new `id: String` field in `crates/eon-vedic/src/analysis/matching.rs`.
2. Introducing a set of translation keys (`TK`) in `crates/eon-ui/src/i18n/mod.rs` and writing their translations in `ko.rs`, `en.rs`, `zh.rs`, and `ru.rs`.
3. Updating `crates/eon-ui/src/components/tabs/vedic_tab.rs` to format Kootas dynamically using the active locale, and adding a circular SVG progress gauge representing the compatibility rating.

No changes to the asynchronous execution structure are needed as `run_compatibility` is already conforming to the `spawn(async move { ... })` rule.

---

## 5. Verification Method

To independently verify the implementation after code changes are made:
1. **Compilation Check**:
   ```bash
   cd crates/eon-ui
   cargo check
   dx build
   ```
2. **Testing Compatibility Calculations**:
   Run unit tests in the Vedic engine:
   ```bash
   cargo test --package eon-vedic
   ```
3. **UI Verification**:
   Navigate to the Vedic Compatibility tab, input partner birth details, select different UI languages (KO, EN, ZH, RU), and verify that:
   * Overall rating header changes language correctly.
   * SVG circular gauge displays correctly with the percentage/score.
   * Ashtakoota scorecard table headers, factor names, and descriptions are translated correctly into the selected language.
