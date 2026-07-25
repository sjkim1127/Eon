# Deep Investigation Report: Dynamic Luck Flows & Test Coverage in `crates/eon-saju`

## 1. Observation

### Summary of Source Files Inspected
- `crates/eon-saju/src/analysis/major_luck.rs` (514 lines)
- `crates/eon-saju/src/analysis/periodic_luck.rs` (594 lines)
- `crates/eon-saju/src/analysis/dynamic_luck.rs` (542 lines)
- `crates/eon-saju/src/analysis/relationships.rs` (600+ lines)
- `crates/eon-saju/src/analysis/structure.rs` (375 lines)
- `crates/eon-saju/src/core/branch.rs` & `core/branch_days.rs`
- `crates/eon-saju/src/core/twelve_stages.rs`
- `crates/eon-saju/src/engine/vm.rs` (1158 lines)
- `crates/eon-saju/tests/edge_cases.rs` (775 lines)

### Verification Execution Results
- `cargo test --package eon-saju`:
  - `unittests src/lib.rs`: **74 passed**, 0 failed
  - `tests/edge_cases.rs`: **22 passed**, 0 failed
  - Total test execution time: **< 0.15 seconds**

---

### Detailed Key Code Observations

#### 1.1 Daewun (대운, Major Luck Cycles)
- **Direction Determination (`LuckDirection`)**:
  - `major_luck.rs:45-54`: Evaluated via `LuckDirection::from_year_and_gender(year_stem, gender)`.
  - Yang male (`Yang + Male`) / Yin female (`Yin + Female`) $\rightarrow$ `Forward` (순행: next month pillar).
  - Yin male (`Yin + Male`) / Yang female (`Yang + Female`) $\rightarrow$ `Reverse` (역행: prev month pillar).
- **Start Age & Precise Date Calculation (`MajorLuckAnalysis`)**:
  - `major_luck.rs:149-218`: `calculate_astro` integrates `AstroEngine` (Swiss Ephemeris bindings) to measure duration between birth UTC time and closest Jieqi (절기, 짝수 인덱스 0, 2, 4...).
  - Time conversion rule: 3 days = 1 year, 6 hours = 1 month, 12 minutes = 1 day (`calculate_precise_start_with_times`).
  - Ultra-precise start date: `offset_seconds = diff_seconds * (365.2425 / 3.0)` added to birth UTC time.
  - Cycle generation: 10 cycles, each 10 years (`start_age + i*10`). Cycle start date offset uses `i * 10 * 365.2425 * 86400` seconds.

#### 1.2 Saewun (세운) & Wolwun (월운) Alignment
- **Saewun (`YearlyLuck`)**:
  - `periodic_luck.rs:78-83`: 60-cycle indexed from 1984 (JiaZi, 甲子). `GanZi::from_index((year - 1984).rem_euclid(60))`.
  - Evaluates 10-Gods, 12-Unseong, Shinsal, and `천전지충(天戦地沖)` (stem clash AND branch clash with day pillar).
- **Wolwun (`MonthlyLuck`)**:
  - `periodic_luck.rs:157-160`: `month_ganzi(year, month)` delegates to `calculate_month_ganzi(year, saju_month)` where `saju_month = if month == 1 { 12 } else { month - 1 }`.
  - **Identified Deficiency**: `MonthlyLuck` uses calendar month integer (`1..12`) directly instead of checking exact astronomical solar term entry dates (절입일시). For instance, Feb 2 is before Lichun (입춘), so it belongs to Chou (丑) month of previous saju year, but `MonthlyLuck::calculate(2026, 2)` maps Feb to Yin (寅) month unconditionally.

#### 1.3 Jijanggan (지장간) Hidden Stems & Stems-Branches Interaction
- **Hidden Stems (`EarthlyBranch::jijanggan`)**:
  - `branch.rs:167-183`: Returns `[yeogi, junggi, jeonggi]` (Remnant, Middle, Main stems).
  - `branch_days.rs:58-166`: `SaryeongAnalysis` calculates Won-ryeong-bun-geum (월령분금) based on days elapsed since solar term entry (7/7/16, 10/20, 9/3/18, etc.).
- **Stems-Branches Interaction**:
  - Full matrix implemented in `relationships.rs` & `dynamic_luck.rs`: Stem Combination/Clash, Branch Triple Combination (삼합), Seasonal Combination (방합), Six Combination (육합), Semi Combination (반합), Branch Clash (충), Punishment (형), Harm (해), Destruction (파), Amhap (암합), Myung-Amhap (명암합).
- **Jijanggan Memory Dump in VM (`SajuVM`)**:
  - `engine/vm.rs:547-578`: When `BranchClash` occurs in `SajuVM`, hidden stems are unsealed/dumped (`mem_dump`), adding elemental weights (`10.0 * memory_dump_weight * priority`) to registers and scoring engine.

#### 1.4 Dynamic 12-Unseong & Gyeokguk State
- **12-Unseong (12운성)**:
  - Calculated via `twelve_stages::calculate_twelve_stage(day_master, branch)` for active luck branches in `YearlyLuck`, `MonthlyLuck`, `DailyLuck`, `HourlyLuck`.
  - Integrated into `SajuVM` (`vm.rs:404-434`): Wang-Sang-Hyu-Su-Sa energy scores applied (+10.0 for Changsheng/Guandai/Jianlu/Diwang, -5.0 for Shuai/Bing/Si/Mu, -2.0 for others).
- **Gyeokguk (格局 Structure)**:
  - `structure.rs:144-375`: `StructureAnalysis::from_pillars` evaluates natal structure statically based on month branch hidden stems and top stem projections, or special structures (Jong-gyeok, Hwa-gi-gyeok).
  - **Identified Deficiency**: Gyeokguk is statically calculated from four pillars. The engine does NOT currently track dynamic Gyeokguk state transitions (e.g. 변격 Transformation of Structure, 파격 Breaking of Structure, 성격 Fulfillment of Structure) when active luck introduces new stems or triple combinations.

#### 1.5 Test Suite Audit (`crates/eon-saju/tests/edge_cases.rs`)
- **Covered Edge Cases**:
  - Lichun boundary before/after (`test_case_1_lichun_boundary_before`, `test_case_1_lichun_boundary_after`).
  - Night Rat Hour vs Early Rat Hour (`test_case_2_night_rat_hour_comparison`).
  - 1988 Summer Time (DST) & Longitude correction (`test_case_3_1988_summer_time`, `test_case_4_longitude_comparison`).
  - Solar term matching for leap month (`test_case_5_leap_month`).
  - Complete user chart verification (Kim Sung-ju natal chart: `test_user_saju_complete`).
  - VM parallel 100-year simulation (`test_case_10_vm_parallel_simulation`).
  - VM void escaping (`test_case_12_vm_void_and_talgong`).
  - VM 10-God patterns (상관견관: `test_case_13_vm_shinsal_and_patterns`).
- **Gaps / Uncovered Edge Cases**:
  - `test_case_5_lunar_input` is currently an unfulfilled stub ("현재는 음력→양력 자동 변환이 없으므로 이 테스트는 인터페이스만 확인").
  - Lack of precise solar term transition test cases for `MonthlyLuck` (Wolwun).
  - Lack of dynamic Gyeokguk transition assertions across luck cycles.
  - Lack of property-based randomized fuzzing across wide date ranges (1900-2100).

---

## 2. Logic Chain

1. **Daewun Precision**:
   - *Observation*: `MajorLuckAnalysis::calculate_astro` computes birth time UTC and exact Jieqi entry via `AstroEngine`.
   - *Deduction*: Daewun start age and direction calculations are mathematically sound and precise. However, individual cycle start dates assume fixed 365.2425-day yearly increments rather than querying astronomical term boundaries for each 10-year block.

2. **Wolwun Boundary Misalignment**:
   - *Observation*: `MonthlyLuck::month_ganzi(year, month)` uses `month - 1` mapping.
   - *Deduction*: In authentic Saju theory, Saju months shift on exact solar term entry (절입 시각). Using calendar months creates incorrect monthly GanZi predictions for births or queries occurring in the first 4-8 days of a calendar month before the solar term entry date.

3. **Jijanggan Weight Activation Scope**:
   - *Observation*: Jijanggan memory dump (`mem_dump`) is only activated during branch clash (`BranchClash`) inside `SajuVM`.
   - *Deduction*: Storage branches (辰戌丑未 - 고지) can also be unsealed by specific stem projections (투출) or semi-combinations (반합). Restricting hidden stem activation to branch clashes underestimates Jijanggan influence in non-clashing charts.

4. **Static Gyeokguk Limitation**:
   - *Observation*: `StructureAnalysis` evaluates Gyeokguk once from `FourPillars`.
   - *Deduction*: Active luck periods (대운/세운) can complete triple combinations or project previously unrevealed stems, changing the ruling element. Without dynamic Gyeokguk tracking (`DynamicStructureState`), VM simulation misses critical life transitions (변격/성격/파격).

5. **Test Harness Completeness**:
   - *Observation*: 96 tests pass, but lunar conversion and dynamic luck transitions are only partially tested.
   - *Deduction*: A property-based fuzzer and comprehensive boundary regression harness are needed for Milestone 2 & 3 to guarantee zero edge-case regressions.

---

## 3. Caveats

1. **Read-Only Scope**: This report was produced purely through read-only static analysis and test execution. No production code in `crates/eon-saju/src` was modified.
2. **Network Access**: Operates under CODE_ONLY network mode; no external web searches or APIs were queried.
3. **Domain Scope**: Focus is strictly on `crates/eon-saju` domain engine, VM, and tests. `eon-ui` and `eon-vedic` were excluded from this specific scope.

---

## 4. Conclusion

The `crates/eon-saju` engine possesses an advanced, high-performance architecture featuring a virtual machine (`SajuVM`), ESIL execution tracing, 60-cycle ganzi math, and Swiss Ephemeris astronomical bindings.

To achieve production-grade maturity in Milestone 2 (R2) and Milestone 3 (R3), the following refactoring and testing strategies must be executed:

### Concrete Refactoring Strategy for Milestone 2 (R2)
1. **Astronomical Wolwun Alignment (`MonthlyLuckRefactor`)**:
   - Modify `MonthlyLuck::calculate` to accept `DateTime<Utc>` or query `AstroEngine::get_solar_term_index` to determine the true Saju month branch based on solar term boundaries.
2. **Dynamic Gyeokguk Tracking (`DynamicStructureState`)**:
   - Extend `DynamicLuckAnalysis` and `SajuVM` with a dynamic Gyeokguk evaluator:
     - `DynamicStructureState { base_structure: StructureType, active_structure: StructureType, status: GyeokStatus }` where `GyeokStatus` $\in$ `{ Stable, Transformed, Broken, Fulfilled }`.
3. **Enhanced Jijanggan Activation (`JijangganActivationEngine`)**:
   - Implement storage branch unsealing (고지개고 辰戌丑未) for stem projections and semi-combinations in addition to branch clashes.

### Concrete Testing Strategy for Milestone 3 (R3)
1. **Property-Based Fuzzer (`Proptest` Integration)**:
   - Integrate `proptest` or expand `engine/fuzzer.rs` to generate 100,000+ random birth dates between 1900 and 2100. Assert:
     - No panics or numeric overflows/underflows in `SajuVM::step`.
     - `QiRegisters` sum strictly equals 100.0% after normalization.
     - Asymptotic score bounds strictly remain in range `[15.0, 100.0]`.
2. **Solar Term Boundary Regression Test Suite**:
   - Create tests covering exact 1-minute before/after timestamps for all 24 solar term entry points in a target year.
3. **Dynamic Gyeokguk Transition Verification**:
   - Add integration tests asserting Gyeokguk state changes during specific Daewun/Saewun cycles (e.g. base 정관격 transformed to 삼합수국/상관격).

---

## 5. Verification Method

To independently verify the observations and findings in this report:

1. **Run Unit & Integration Test Suite**:
   ```bash
   cargo test --package eon-saju
   ```
   *Expected Output*: 74 unit tests in `src/lib.rs` and 22 integration tests in `tests/edge_cases.rs` pass cleanly.

2. **Inspect Core Code Implementation**:
   - Daewun calculation: `crates/eon-saju/src/analysis/major_luck.rs` (lines 45-250)
   - Periodic luck calculation: `crates/eon-saju/src/analysis/periodic_luck.rs` (lines 78-160)
   - Dynamic luck & interaction: `crates/eon-saju/src/analysis/dynamic_luck.rs` (lines 120-350)
   - Virtual machine score & memory dump: `crates/eon-saju/src/engine/vm.rs` (lines 168-700)
   - Edge case integration tests: `crates/eon-saju/tests/edge_cases.rs` (lines 1-775)

3. **Conditions for Invalidation**:
   - If `MonthlyLuck` is modified to use `AstroEngine` solar term boundaries, observation 1.2 is invalidated.
   - If `StructureAnalysis` is updated to support dynamic luck periods, observation 1.4 is invalidated.
