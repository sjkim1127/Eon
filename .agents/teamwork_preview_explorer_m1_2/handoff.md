# Milestone 1 (R1) Engine Investigation & Algorithmic Strategy Handoff Report

**Target Modules**: `crates/eon-saju/src/` (Yongsin, Gyeokguk, 12-Unseong, Sinsal, Power/Strength)  
**Investigator Agent**: `teamwork_preview_explorer_m1_2`  
**Date**: 2026-07-25  

---

## 1. Observation

Direct code examination of `crates/eon-saju/src/` revealed the following exact implementation details, line numbers, and architectural patterns:

### 1-1. 억부(抑扶) 용신 및 신강/신약 평가 (`analysis/strength.rs`, `analysis/power.rs`, `analysis/yongshin.rs`)
* **Day Master Strength Evaluation (`strength.rs:468-547`)**:
  * Evaluates 4 criteria: `DeukRyeong` (월지 득령), `DeukJi` (지통근 득지), `DeukSi` (시지 득시), `DeukSe` (세력 득세).
  * `StrengthType`: Determined strictly by count of satisfied criteria (`acquired_count`): `2..=4` -> `Strong`, `0..=1` -> `Weak`.
  * `strength_score`: Calculated via `score_ryeong (25.0) + score_ji (25.0) + score_si (25.0) + support_ratio * 0.25`.
* **Element Weighting vs Raw Counting Mismatch (`yongshin.rs:91-103`)**:
  ```rust
  let yinxing = strength.deuk_se.yinxing_count as f32;
  let bijie = strength.deuk_se.bijie_count as f32;

  if yinxing > bijie * 1.5 {
      day_master_el.generates() // 재성 (용재파인)
  } else if bijie > yinxing * 1.5 {
      day_master_el.controlled_by() // 관성 (관살제겁)
  } else {
      day_master_el.generates() // 식상 (설기생재)
  }
  ```
  * `strength.deuk_se.yinxing_count` and `bijie_count` are raw integer counts (`u8`) of pillars containing TenGods (1..=8), **ignoring position weights** (e.g., Month Branch weight 3.5 vs Year Stem weight 1.0 in `config.weights`).
* **Rigid Weak Day Master Rule (`yongshin.rs:83-88`)**:
  ```rust
  StrengthType::Weak => {
      // 신약(身弱)은 항상 인성(印星)이 억부용신
      day_master_el.generated_by() // 인성
  }
  ```
  * For any weak day master, Yinxing (인성) is unconditionally selected as Eokbu Yongshin, even for charts with heavy Caisheng (재다신약) where Bijie (비겁) is classically required, or where Yinxing is completely absent/clashed in the chart.

### 1-2. 조후(調候) 용신 (`analysis/yongshin.rs:235-474`)
* **Thermal & Humidity Indices (`calculate_thermal_index`, `calculate_humidity_index`)**:
  * Thermal index score clamped to `[-100, 100]`. Base month weights: `Hai/Zi/Chou` (-40), `Si/Wu/Wei` (+40), `Yin/Mao/Chen` (+10), `Shen/You/Xu` (-10), multiplied by Saryeong weight (1.2x/1.0x/0.8x). Additive stem and branch adjustments.
  * 2D Johu Matrix (`get_johu_analysis`):
    * Cold (<= -30) & Wet (<= -30) -> Fire (火)
    * Hot (>= 30) & Dry (>= 30) -> Water (水)
    * Hot & Wet -> Metal (金)
    * Cold & Dry -> Wood (木)
* **Missing Stem-Level Specificity**:
  * Outputs generic element recommendations (`Element::Fire`, `Element::Water`, etc.) rather than specific Heavenly Stems required by classical Johu theory (*Qiong Tong Bao Giam* / 궁통보감), such as `Bing` (丙火 - solar heat) vs `Ding` (丁火 - artificial warmth), or `Gui` (癸水 - rain/dew) vs `Ren` (壬水 - river/ocean).

### 1-3. 통관(通關) 용신 (`analysis/yongshin.rs:480-561`)
* **Power-Based Mediation Logic**:
  ```rust
  let min_threshold = 15.0; // 최소 세력
  let max_diff = 20.0; // 세력 차이 허용 범위
  if score1 >= min_threshold && score2 >= min_threshold {
      let diff = (score1 - score2).abs();
      if diff <= max_diff { return Some(...) }
  }
  ```
  * Checks 5 pairwise clashes: Metal-Wood (via Water), Water-Fire (via Wood), Wood-Earth (via Fire), Fire-Metal (via Earth), Earth-Water (via Metal).
* **Selection Priority Deficit (`yongshin.rs:186-222`)**:
  * Primary Yongshin selection hierarchy checks extreme Johu first, then Byeongyak, then Eokbu. **Tonggwan Yongshin is omitted from primary selection**, even when a severe two-element clash creates a destructive deadlock in the chart.

### 1-4. 병약(病藥) 용신 (`analysis/yongshin.rs:564-591`)
* **Limited Trigger Range**:
  ```rust
  if strength.strength_type == StrengthType::Weak {
      if strength.deuk_se.guanxing_count >= 3 { ... }
      if strength.deuk_se.shishang_count >= 3 { ... }
  }
  ```
  * Only triggers when Weak Day Master has Guanxing >= 3 or Shishang >= 3, always designating Yinxing as the medicine.
  * Fails to detect:
    * Excessive Caisheng in Weak DM (재다신약 - Disease: Caisheng, Medicine: Bijie).
    * Excessive Yinxing in Strong DM (인다수침/모왕자고 - Disease: Yinxing, Medicine: Caisheng).
    * Excessive Bijie in Strong DM (비겁태과 - Disease: Bijie, Medicine: Guanxing/Shishang).
    * Yongshin Damage by Clash (용신상해 - Disease: Clash agent, Medicine: Combination or Controlling agent).

### 1-5. Special Gyeokguk (종격, 화격, 건록/양인, 외격) (`analysis/structure.rs`, `analysis/transformations.rs`)
* **Polarized / Jong-Gyeong Logic (`structure.rs:191-264`)**:
  * Evaluates `deuk_se.support_ratio`: `>= 80.0%` -> `JongWang` / `JongGang`; `<= 20.0%` -> `JongAh` / `JongJae` / `JongSal`.
  * **Missing Jin-Jong vs Ga-Jong (眞從 vs 假從) Differentiation**: Does not verify if roots of Day Master or Yinxing remain in hidden stems (which breaks pure Jin-Jong격 into Ga-Jong격 or normal weak chart).
* **HwaGi Gyeokguk (`structure.rs:164-188`)**:
  * Checks if DM combines with month/hour stem and transformed element == month branch element.
  * **Missing Break-Star (破星) Check**: Fails to verify if an opposing stem/branch exists in the chart that destroys the transformation (e.g. 乙庚化金 broken by 丙火 in year stem).
* **Missing Outer Patterns (외격/별격)**:
  * Missing Samhap Jeonwang patterns (곡직격, 염상격, 가색격, 종혁격, 윤하격) as explicit Gyeokguk sub-types, as well as classic outer patterns (비천록마, 임기용배, 정란차, 자요사/축요사).

### 1-6. 12-Unseong & Sinsal Edge Cases (`core/twelve_stages.rs`, `analysis/shinsal.rs`, `analysis/spirit_markers.rs`, `analysis/void.rs`)
* **12-Unseong (포태법) (`twelve_stages.rs:207-254`)**:
  * Correctly differentiates Yang stems forward (순행) and Yin stems backward (역행).
  * Assigns root weights: A-grade (1.0), B-grade (0.5), C-grade (0.0).
  * *Constraint*: Lacks configurable flag for "Yin Stem Same Direction" (음포태 동행설 vs 음건역행설), a key option required by various Saju academic schools.
* **Sinsal & Spirit Markers (`shinsal.rs`, `spirit_markers.rs`)**:
  * Implements 12-Sinsal, Cheoneul Gwiin, Wenchang, Taiji, Yuede, Tiande, Zhenglu, Jinyu, Anlu, Xuetang, Kuigang, Baihu, Yangin, Tianluo, Jimang, Wonjin, Gwimun, etc.
  * **Missing Samjae (삼재) Calculation**: No function or struct exists in `shinsal.rs` or `spirit_markers.rs` for computing 3-Year Calamity cycle (입삼재, 눌삼재, 날삼재) based on birth year branch vs transit year branch (e.g. 申子辰 -> 寅卯辰).
  * **Gongmang Dissolution (공망해충/해합) (`void.rs`)**: `VoidAnalysis` checks static branch containment without evaluating if branch clash (충) or combination (합) dissolves or alters the void effect.

---

## 2. Logic Chain

1. **Observation**: `strength.rs` computes weighted power (`support_ratio`) via `power.rs` (accounting for month branch 3.5x weight, stem root multipliers, climate corrections).  
   **Deduction**: `yongshin.rs:91-103` uses unweighted raw counts (`yinxing_count` vs `bijie_count`) to decide between `용재파인`, `관살제겁`, and `설기생재`.  
   **Inference**: A chart with 2 weak Year/Hour Bijian stems (weight 1.0 x 2 = 2.0) and 1 powerful Month-Branch Yinxing (weight 3.5 x 1 = 3.5) will incorrectly evaluate `bijie_count (2) > yinxing_count (1)`, selecting `관살제겁` instead of `용재파인`. **Algorithm must be refactored to use weighted score percentages from `power.rs`.**

2. **Observation**: `yongshin.rs:83-87` forces `StrengthType::Weak` DM to always select Yinxing (`day_master_el.generated_by()`).  
   **Deduction**: In 재다신약 (Wealth Overwhelming Weak DM) charts where Yinxing is absent or destroyed by Caisheng, Bijie is the primary savior. Forcing Yinxing generates inaccurate advice.  
   **Inference**: Eokbu Yongshin for Weak DM must evaluate the primary cause of weakness (Caisheng vs Guanxing vs Shishang) and select Bijie when Caisheng dominates or Yinxing is missing.

3. **Observation**: `yongshin.rs:186-222` selects primary Yongshin using `Extreme Johu -> Byeongyak -> Eokbu`.  
   **Deduction**: Tonggwan Yongshin is never selected as `primary`, even when a chart suffers from a severe 40% vs 40% Metal-Wood clash that destroys health/harmony.  
   **Inference**: Primary selection logic needs a unified priority scoring matrix where Severe Deadlock Clashes (Tonggwan) can take precedence over minor Eokbu imbalances.

4. **Observation**: `structure.rs` relies solely on `support_ratio <= 20.0%` or `>= 80.0%` for 종격 (Jong-Gyeong).  
   **Deduction**: Classical Saju distinguishes 眞從 (Pure Follower, zero opposing roots) from 假從 (Fake Follower, weak opposing root present) and Normal Weak.  
   **Inference**: Gyeokguk classification requires checking hidden stem root weights (`root_score`) of Day Master/Yinxing before certifying a true `Jong-Gyeong`.

5. **Observation**: `spirit_markers.rs` contains over 30 Sinsal types, but contains no code for Samjae (삼재).  
   **Deduction**: Samjae is one of the most requested user-facing Sinsal features in Korean fortune-telling software.  
   **Inference**: Samjae calculation based on Tri-Harmony year groups (申子辰 -> 寅卯辰, 寅午戌 -> 申酉戌, 巳酉丑 -> 亥子丑, 亥卯未 -> 巳午未) must be added for Milestone 1 (R1).

---

## 3. Caveats

* **Scope of Investigation**: Investigation was strictly read-only within `crates/eon-saju/src/`. No production code in `crates/` was altered during this phase.
* **Wasm & Performance Impact**: Any proposed algorithmic enhancements to power scoring, Gyeokguk evaluation, or Sinsal mapping must remain zero-allocation / lightweight stack-friendly to run efficiently inside browser WebAssembly runtime (`crates/eon-ui`).
* **Academic Variations**: 12-Unseong Yin-Stem progression (음포태 역행 vs 동행) and Jin-Jong/Ga-Jong criteria have minor variations across traditional schools (자평진전, 적천수, 궁통보감). The proposed R1 strategy makes these configurable via `AnalysisConfig`.

---

## 4. Conclusion & Milestone 1 (R1) Algorithmic Improvement Strategies

To elevate `eon-saju` engine precision to professional-grade classical and modern standard, the following concrete algorithmic strategies are formulated for Milestone 1 (R1):

### Strategy 1: Weighted Power-Based 억부(抑扶) & 병약(病藥) Engine Refactoring
1. **Replace Raw Counts with Weighted Power Scores**:
   * Refactor `yongshin.rs:91-103` to compare `integrated.ten_god_scores` percentages rather than `yinxing_count` / `bijie_count`.
2. **Context-Aware Weak Day Master Yongshin**:
   * If Weak due to Caisheng (재다신약, Caisheng > 35%): Select **Bijie (비겁)** as Primary Eokbu Yongshin if Yinxing is weak/absent.
   * If Weak due to Guanxing (관살태과): Select **Yinxing (인성)** (관인상생).
   * If Weak due to Shishang (식상다설): Select **Yinxing (인성)** (인극식).
3. **Comprehensive Byeongyak (병약) Structural Diagnostic**:
   * Implement explicit `Byeong` (Disease Element) and `Yak` (Medicine Element) tracking structs.
   * Identify Disease when any single non-DM element exceeds 40% total power, or when a Yongshin is directly clashed by a strong opposing element. Medicine is chosen as the exact element that controls or neutralizes the Disease.

### Strategy 2: 2D Johu (調候) Precision & Heavenly Stem Mapping
1. **Stem-Level Johu Specification (궁통보감 / Qiong Tong Bao Giam Index)**:
   * Expand `RecommendedYongshin` for Johu to specify precise preferred Heavenly Stems (e.g. 丙火 over 丁火 in winter; 癸水 over 壬水 in summer).
2. **Solar Term Gradient Weighting**:
   * Enhance `calculate_thermal_index` using exact solar longitude (절기시각) distance from solstice/equinox to dynamically scale base month thermal weights continuously (-40.0 to +40.0).

### Strategy 3: Unified Primary Yongshin Priority Scoring Algorithm
* Implement a multi-factorial Priority Matrix for `YongshinAnalysis::primary`:
  $$\text{PriorityScore}(E) = w_{\text{johu}} \cdot S_{\text{johu}}(E) + w_{\text{byeong}} \cdot S_{\text{byeong}}(E) + w_{\text{tonggwan}} \cdot S_{\text{tonggwan}}(E) + w_{\text{eokbu}} \cdot S_{\text{eokbu}}(E)$$
  This guarantees that severe deadlock clashes (통관) or urgent thermal extremes (조후) receive primary status when their urgency exceeds routine Eokbu adjustments.

### Strategy 4: Gyeokguk (격국) Refinement: 眞從 vs 假從, 破星, & Outer Patterns
1. **Jin-Jong (眞從) vs Ga-Jong (假從) Classification**:
   * Before assigning `JongAh`, `JongJae`, or `JongSal`, verify `deuk_ji.root_score` for DM / Yinxing. If root score == 0 -> `JinJong` (진종격); if root score > 0 but support ratio <= 20% -> `GaJong` (가종격).
2. **HwaGi (화기격) Break-Star Validation**:
   * Verify absence of controlling stems/branches (破星) before certifying `HwaGi`.
3. **Samhap Jeonwang & Special Outer Patterns**:
   * Formally classify `곡직격` (Wood), `염상격` (Fire), `가색격` (Earth), `종혁격` (Metal), `윤하격` (Water) under `SpecialTransformation` when seasonal combination (방합) or triple combination (삼합) dominates the month branch.

### Strategy 5: Sinsal (신살) Edge Cases & Samjae Engine Integration
1. **Add Samjae (삼재) Module**:
   ```rust
   pub enum SamjaeStage { Entrance (입삼재), Dwelling (눌삼재), Exit (날삼재) }
   pub fn calculate_samjae(year_branch: EarthlyBranch, transit_year_branch: EarthlyBranch) -> Option<SamjaeStage>
   ```
   * Map Year Branch Triads: `ShenZiChen` -> `Yin` (입), `Mao` (눌), `Chen` (날); `YinWuXu` -> `Shen` (입), `You` (눌), `Xu` (날); `SiYouChou` -> `Hai` (입), `Zi` (눌), `Chou` (날); `HaiMaoWei` -> `Si` (입), `Wu` (눌), `Wei` (날).
2. **Gongmang & Spirit Marker Dynamics under Clash/Harm**:
   * Implement Gongmang Dissolution (`공망해충` / `공망해합`) in `void.rs` when the voided branch undergoes 6-Combination or Branch Clash.
3. **Configurable 12-Unseong Yin Stem Option**:
   * Add `yin_stem_reverse: bool` (default: true) in `AnalysisConfig` to toggle between traditional counter-clockwise rotation and forward rotation.

---

## 5. Verification Method

To independently verify these findings and future R1 implementer additions:

1. **Workspace Syntax & Unit Test Verification**:
   ```bash
   cargo check --workspace
   cargo test --package eon-saju
   ```
2. **Target Test Executions**:
   * Yongshin & Johu: `cargo test --package eon-saju analysis::yongshin::tests`
   * Strength & Power: `cargo test --package eon-saju analysis::strength::tests`
   * 12-Unseong: `cargo test --package eon-saju core::twelve_stages::tests`
   * Sinsal & Spirit Markers: `cargo test --package eon-saju analysis::spirit_markers::tests`
3. **Invalidation Conditions**:
   * Any change that introduces `package.json` or npm commands violates project layout (`AGENTS.md`).
   * Any hardcoded Yinxing selection for weak DM without checking Caisheng power invalidates R1 Eokbu accuracy.
   * Any failure of existing workspace unit tests during refactoring.
