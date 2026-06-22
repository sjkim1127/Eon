# E2E Test Infra: Vedic Astrology Enhancements

## Test Philosophy
- Opaque-box, requirement-driven. No dependency on implementation design.
- Methodology: Category-Partition + BVA + Pairwise + Workload Testing.

## Feature Inventory
| # | Feature | Source (requirement) | Tier 1 | Tier 2 | Tier 3 |
|---|---------|---------------------|:------:|:------:|:------:|
| 1 | Ashtakoota Compatibility | ORIGINAL_REQUEST §R1 | 5 | 5 | ✓ |
| 2 | Shadbala & Bhava Bala 6 Factors | ORIGINAL_REQUEST §R2 | 5 | 5 | ✓ |
| 3 | KP System Lords & Significators | ORIGINAL_REQUEST §R3 | 5 | 5 | ✓ |
| 4 | Hierarchical Dasha Timeline | ORIGINAL_REQUEST §R4 | 5 | 5 | ✓ |

## Test Architecture
- Test runner: cargo test --package eon-vedic --test compatibility_shadbala_kp_dasha
- Test case format: Rust integration tests in `crates/eon-vedic/tests/compatibility_shadbala_kp_dasha.rs`
- Directory layout:
  - `crates/eon-vedic/tests/compatibility_shadbala_kp_dasha.rs`

## Real-World Application Scenarios (Tier 4)
| # | Scenario | Features Exercised | Complexity |
|---|----------|--------------------|------------|
| 1 | Standard Natal Reading | Shadbala, KP cusps, Dasha timeline | Medium |
| 2 | Relationship Compatibility | Ashtakoota factors, current active Dashas | High |
| 3 | Career & Wealth Audit | KP significators, Shadbala strengths, active Dashas | High |
| 4 | Extreme Location Analysis | KP cusps, Shadbala under polar latitudes | Medium |
| 5 | Historical Timeline Reconstruction | 120-year Vimshottari timeline, boundary transitions | High |

## Coverage Thresholds
- Tier 1 (Feature Coverage): >= 5 test cases per feature (Total >= 20)
- Tier 2 (Boundary & Corner Cases): >= 5 test cases per feature (Total >= 20)
- Tier 3 (Cross-Feature Combinations): Pairwise coverage of major feature interactions (Total >= 4)
- Tier 4 (Real-World Application Scenarios): Realistic workload test cases (Total >= 5)
- Total minimum test cases: 49
