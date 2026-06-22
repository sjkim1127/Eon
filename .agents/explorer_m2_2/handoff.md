# Handoff Report — Ashtakoota Guna Milan Investigation

This handoff report summarizes the read-only investigation of Milestone M2 (R1 Ashtakoota Guna Milan) in Eon.

---

## 1. Observation

### 1-1. Ashtakoota Calculation Engine (`crates/eon-vedic/src/analysis/matching.rs`)
- In `matching.rs`, compatibility results are calculated using the `MatchingEngine::calculate_compatibility` method (lines 29–32).
- The `KootaScore` struct is defined on lines 5–12:
  ```rust
  pub struct KootaScore {
      pub name: String,
      pub max_points: f64,
      pub earned_points: f64,
      pub description: String,
  }
  ```
- The names and descriptions are hardcoded in English inside `MatchingEngine::calculate_compatibility` (lines 62–111), for example:
  ```rust
  KootaScore {
      name: "Varna (Caste/Vocation)".to_string(),
      max_points: 1.0,
      earned_points: varna_score,
      description: if varna_score == 1.0 { "Good work-profile alignment.".to_string() } else { "Differing natural vocations.".to_string() },
  }
  ```
- The overall explanation string is formatted in Korean on lines 121–131:
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

### 1-2. Compatibility Tab View (`crates/eon-ui/src/components/tabs/vedic_tab.rs`)
- Around line 2141, the scorecard table is rendered with hardcoded Korean text in headers and fields, e.g.:
  - Line 2144: `h3 { class: "font-semibold text-slate-200", "아쉬타쿠타(Ashtakoota) 세부 매칭 평점표" }`
  - Line 2150: `th { class: "px-4 py-3 text-left font-medium", "매칭 요인 (Koota)" }`
  - Line 2159: `td { class: "px-4 py-3 font-semibold text-slate-300", "{k.name}" }`
  - Line 2169: `td { class: "px-4 py-3 text-xs text-slate-400", "{k.description}" }`
- Similarly, the Mangal Dosha section contains hardcoded text:
  - Line 2113: `"남성 화성살 (Male Mangal Dosha)"`
  - Line 2120: `if compat.report.male_mangal_dosha { "🔥 화성살(Manglik) 감지" } else { "✓ 해당 없음 (양호)" }`
  - Line 2137: `"ℹ️ 상호 화성살 상쇄(Dosha Samya)가 성립되어 화성살의 부정적 영향이 소멸되었습니다."`

### 1-3. Localization Mappings (`crates/eon-ui/src/i18n/`)
- Mappings use compile-time zero-dependency static translation matching via the `TK` enum in `mod.rs` and the `translate(key: TK) -> &'static str` function inside language-specific files (`en.rs`, `ko.rs`, `zh.rs`, `ru.rs`).
- Currently, no keys represent the 8 kootas' names, descriptions, or the compatibility scorecard labels.

---

## 2. Logic Chain

1. **Calculations and Serialization**:
   - The engine computes the raw compatibility scores (`earned_points` out of `max_points`) correctly for each of the 8 factors (Observation 1-1).
   - Because the names and descriptions are returned as hardcoded English string fields (`name` and `description` in `KootaScore`), they cannot be easily or cleanly localized in the UI without parsing string prefixes or modifying the returned fields.
2. **Structuring the Fix**:
   - Adding a machine-readable field (like `koota_id: KootaId`) to the engine's `KootaScore` struct will allow the UI to map each score directly to the corresponding localized translation key (TK) (Observation 1-1).
3. **Decoupled Localization**:
   - By declaring translation keys (`TK::KootaVarnaName`, `TK::KootaVarnaDescGood`, etc.) in `crates/eon-ui/src/i18n/mod.rs` and providing language-specific values in `en.rs`, `ko.rs`, `zh.rs`, `ru.rs`, we can translate everything based on the current locale (Observation 1-3).
4. **Interactive Visual Progress component**:
   - Implementing a circular SVG progress gauge component (`CircularScoreGauge`) will visually show the compatibility total score in the Compatibility tab (Observation 1-2).
   - In addition, adding horizontal progress bar bars inside the table cells for each of the 8 kootas will represent how close they are to the maximum points (Observation 1-2).

---

## 3. Caveats

- **Timezone/DST Parameters**: Compatibility calculations prepare birth contexts with a hardcoded `"Asia/Seoul"` timezone for the partner (in `vedic_tab.rs` line 943). If partner profiles are loaded from other regions, this hardcoded timezone might introduce slight inaccuracies.
- **Engine Modification Parity**: Adding `koota_id` to `KootaScore` requires a cargo update and schema parity for any serialized payload, though it does not break compilation as `KootaScore` is only instantiated in `crates/eon-vedic` and used locally.

---

## 4. Conclusion

Milestone M2 (R1 Ashtakoota Guna Milan) requires:
1. Enhancing the engine `KootaScore` to include `koota_id: KootaId`.
2. Integrating translation keys in `crates/eon-ui/src/i18n/` for the 8 kootas' names/descriptions and overall matching text (supporting KO, EN, ZH, RU).
3. Rendering a circular SVG progress gauge (`CircularScoreGauge`) for the overall compatibility score.
4. Rendering micro horizontal progress bar gauges for individual kootas inside the scorecard table.

A complete codebase unified patch file has been successfully written to `/Users/sjkim1127/Eon/.agents/explorer_m2_2/ashtakoota_guna_milan.patch` for implementation.

---

## 5. Verification Method

To verify the proposed implementation:
1. Apply the unified patch:
   ```bash
   git apply /Users/sjkim1127/Eon/.agents/explorer_m2_2/ashtakoota_guna_milan.patch
   ```
2. Build and check the frontend to verify there are no compilation errors:
   ```bash
   cd crates/eon-ui
   cargo check
   dx build
   ```
3. Run the engine test workspace:
   ```bash
   cargo test --workspace
   ```
4. Verify the compatibility tab UI visually by entering birth information, running the compatibility analysis, and switching between languages (KO, EN, ZH, RU) to check that names, descriptions, and overall status text update accordingly.
