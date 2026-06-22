# Proposed Changes for Milestone M2 (Ashtakoota Guna Milan Localization & Gauge)

This document contains a structured breakdown of the recommended changes to Eon to support the Ashtakoota compatibility details and localization in Korean (KO), English (EN), Chinese (ZH), and Russian (RU).

---

## 1. Engine Enhancements (`crates/eon-vedic/src/analysis/matching.rs`)

### Summary of Changes
- Introduce `KootaId` enum representing the 8 components.
- Add `koota_id` field to `KootaScore` struct to allow the UI to identify components programmatically without resorting to fragile string parsing.

### Code Diff
```rust
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum KootaId {
    Varna,
    Vashya,
    Tara,
    Yoni,
    GrahaMaitri,
    Gana,
    Bhakoot,
    Nadi,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KootaScore {
    pub koota_id: KootaId,
    pub name: String,
    pub max_points: f64,
    pub earned_points: f64,
    pub description: String,
}
```

---

## 2. Localization Keys (`crates/eon-ui/src/i18n/mod.rs`)

### Summary of Changes
- Add translation keys to the `TK` enum in `crates/eon-ui/src/i18n/mod.rs` to support the names and descriptions of the 8 kootas as well as the scorecard UI labels.

### Translation Keys Added
- UI Labels:
  - `VedicCompatAshtakootaTableTitle`: Title for scorecard table.
  - `VedicCompatKootaFactor`: Label for factors.
  - `VedicCompatMaxWeight`: Label for maximum points.
  - `VedicCompatEarnedPoints`: Label for earned points.
  - `VedicCompatDescription`: Label for descriptions.
  - `VedicCompatOverallJudgement`: Title for compatibility decision.
  - `VedicCompatScoreLabel`: Overall score prefix.
  - `VedicCompatStatusExcellent`: Text for high compatibility status.
  - `VedicCompatStatusCaution`: Text for warning status.
  - `VedicCompatMaleMangal`, `VedicCompatFemaleMangal`: Mangal Dosha titles.
  - `VedicCompatMangalDetected`: Mangal Dosha warning.
  - `VedicCompatMangalNone`: Mangal Dosha none.
  - `VedicCompatMangalCancelled`: Mangal Dosha cancellation message.
- Koota Names:
  - `KootaVarnaName`, `KootaVashyaName`, `KootaTaraName`, `KootaYoniName`, `KootaGrahaMaitriName`, `KootaGanaName`, `KootaBhakootName`, `KootaNadiName`
- Koota Descriptions (Conditional / Parameterized):
  - `KootaVarnaDescGood`, `KootaVarnaDescBad`
  - `KootaVashyaDesc` (contains `{}` for dynamic rating substitution)
  - `KootaTaraDescGood`, `KootaTaraDescWarning`, `KootaTaraDescBad`
  - `KootaYoniDesc` (contains `{}` for dynamic rating substitution)
  - `KootaGrahaMaitriDescGood`, `KootaGrahaMaitriDescNeutral`, `KootaGrahaMaitriDescBad`
  - `KootaGanaDescGood`, `KootaGanaDescWarning`, `KootaGanaDescBad`
  - `KootaBhakootDescGood`, `KootaBhakootDescBad`
  - `KootaNadiDescGood`, `KootaNadiDescBad`
- Overall Explanation Strings:
  - `VedicCompatExplanationPrefix`
  - `VedicCompatExplanationCompatible`
  - `VedicCompatExplanationDosha`
  - `VedicCompatExplanationLowScore`

---

## 3. UI Component Enhancements (`crates/eon-ui/src/components/tabs/vedic_tab.rs`)

### Summary of Changes
- Define `CircularScoreGauge` component which renders a radial progress gauge using SVG and Tailwind CSS classes.
- Integrate overall compatibility description localization dynamically.
- Update table mapping to substitute English labels with localized names and descriptions based on the computed `koota_id`.
- Add mini horizontal progress bar gauges inside the scorecard table cells.

---

## 4. Unified Patch File
The machine-applicable unified patch containing all these changes can be found in `/Users/sjkim1127/Eon/.agents/explorer_m2_2/ashtakoota_guna_milan.patch`.
