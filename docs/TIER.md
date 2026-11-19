# Destiny Tier 7.0 — Model Notes

This document records how the Tier 7.0 score is computed, and what it does and does not mean.
It is derived from `crates/eon-service/src/services/tier.rs`; if the code and this file disagree,
the code is authoritative and this file should be updated.

## Status: a designed heuristic, not a validated measurement

Every engine behind the tier (Saju, Vedic, Western, ZWDS, Human Design, QiMen, Numerology) is a
deterministic calculation whose *astronomical and rule-based outputs* are checked against
independent references (see the test suites). The **Tier score is different**: the components,
weights, bonuses and grade thresholds were chosen by the author. There is no external ground truth
(no outcome data) that could say a given score is "right". Treat the tier as a *consistent,
reproducible summary index* for comparing charts produced by the same model, not as a
prediction. Agents consuming it should say so.

## Pipeline

1. **Component scores** — up to 15 `DestinyComponent { key, score (0-100), weight }`.
2. **Weight renormalisation** — components of engines that were not run get weight `0`; the
   remaining weights are divided by their sum so the active weights add up to exactly `1.0`.
3. **Raw score** — `component_raw = Σ score × weight`.
4. **Synergies** — `quantum_synergies` add fixed deltas (can be negative) to the raw score.
5. **Clamp** — `destiny_raw_score = clamp(component_raw + Σ delta, 0, 100)`.
6. **Spread** — `destiny_tier_score = spread_normalize(destiny_raw_score)`.
7. **Grade** — threshold lookup on the rounded score.

### Spread curve

`spread_normalize` keeps 0, 50 and 100 fixed and pushes scores away from the middle:

- `s < 50`: `50 · (s/50)^1.4` (lower)
- `s ≥ 50`: `50 + 50 · ((s−50)/50)^0.7` (raised)

So a raw 70 becomes about 76 and a raw 30 becomes about 24. It is monotonic.

### Grades

| Score | Grade | | Score | Grade |
|---|---|---|---|---|
| ≥ 97 | S+ | | ≥ 51 | C+ |
| ≥ 90 | S | | ≥ 43 | C |
| ≥ 83 | A+ | | ≥ 35 | D+ |
| ≥ 75 | A | | ≥ 27 | D |
| ≥ 67 | B+ | | ≥ 18 | E |
| ≥ 59 | B | | < 18 | F |

## Components (nominal weights before renormalisation)

| # | Key | Weight | Engine | Score formula |
|---|---|---|---|---|
| 1 | `saju_strength` | 0.10 | Saju | `min(|strength_score|, 50) × 2` — saturates at 100 once the strength index reaches 50 |
| 2 | `element_flow` | 0.07 | Saju | five-element throughput × 100 |
| 3 | `structure` | 0.08 | Saju | 40 for a Follower structure, 85 for every other structure |
| 4 | `spirit_markers` | 0.05 | Saju | `min(20 × auspicious, 100) − 10 × inauspicious`, clamped to 0..100 |
| 5 | `vedic_houses` | 0.09 | Vedic | `overall_strength_score / 6` |
| 6 | `vedic_yogas` | 0.08 | Vedic | `min(30 × count(VeryHigh yogas), 100)` |
| 7 | `planet_status` | 0.05 | Vedic | share of the 7 planets in Bala/Yuva avastha × 100 |
| 8 | `western_astrology` | 0.09 | Western | `min(50 + 15 × aspect_patterns + min(2 × aspects, 50), 100)` |
| 9 | `zwds_harmony` | 0.09 | ZWDS | `clamp(50 + min(20 × formations, 60) − 10 × SiHua collisions, 20, 100)` |
| 10 | `human_design_dynamics` | 0.07 | Human Design | `clamp(45 + min(10 × channels, 40) + 10, 20, 100)` (15 for a Reflector) |
| 11 | `qimen_harmony` | 0.06 | QiMen | `clamp(45 + door + star, 20, 100)`; Kai/Xiu/Sheng envoy door +25, Jing/Du +10; Ren/Fu/Chong chief star +15, other +5 |
| 12 | `numerology_vibration` | 0.06 | Numerology | `clamp(30 + min(6 × life_path, 50) + 20 if master, 20, 100)` |
| 13 | `luck_cycle` | 0.05 | Transit | transit composite score (only when a transit analysis is supplied) |
| 14 | `stability` | 0.03 | Saju VM | `100 − min(2 × total_crashes, 100)` |
| 15 | `golden_time` | 0.03 | Saju | average score of the golden period (50 when none) |

Nominal weights sum to 1.00 (0.10+0.07+0.08+0.05+0.09+0.08+0.05+0.09+0.09+0.07+0.06+0.06+0.05+
0.03+0.03 = 1.00). Without a transit analysis the effective weights are renormalised over the
remaining components.

### Quantum synergies

Fixed deltas added to the raw score when a condition holds:

| Title | Condition | Delta |
|---|---|---|
| Noble Alignment | ≥ 2 auspicious Saju markers and ≥ 1 top Vedic yoga | +12 |
| Geometric Resonance | Western aspect patterns and ZWDS formations both present | +15 |
| Quantum Energy Alignment | ≥ 3 Human Design defined centers and a QiMen chart (its `ju_number > 0` test is always true) | +14 |
| Master Number Resonance | master life-path number and (Saju marker or Vedic yoga) | +16 |
| 8-Engine Universal Harmony | ≥ 1 auspicious Saju marker, ≥ 1 top Vedic yoga, and non-empty Western aspects, ZWDS formations, Human Design defined centers, a QiMen pan and a positive life path | +22 |
| Inflection Point Collision | Saju VM `total_crashes` > 20 | −10 |

Synergies are not weighted, so a chart on which every engine ran can gain much more than a chart
on which some engines were skipped. Compare tiers only between charts with the same set of
evaluated engines.

## Unknown birth time

When only the date is known (`unknown_time`), Vedic, Western, ZWDS, Human Design and QiMen are
not evaluated (see `skippedEngines` in the output). Their components get weight 0 and the
remaining ones are renormalised; `tier.vedicResult` is `null`. Saju runs on three pillars (the
得時 criterion is dropped and the strength score rescaled). Such a tier is **not comparable** with
a tier that used all engines.

## Known limitations of the model

These are properties of the *design*, listed so that nobody reads more into the number than it
carries.

- **Numerology is scored by magnitude.** Component 12 gives a higher score to a higher life-path
  number (`6 × life_path`, capped). Numerological meaning does not rank 9 above 1; this term is an
  arbitrary stand-in and the weakest component in the model.
- **Several components are coarse.** `structure` takes only two values; `saju_strength`
  saturates at a strength index of 50; Vedic yogas, Western patterns and ZWDS formations are
  counted, not weighed by quality.
- **Counting rewards chart size.** More aspects/patterns/channels raise the score regardless of
  what they are.
- **Different engines, different conventions.** Each engine follows one school. Open choices that
  affect results: ZWDS leap-month and late-Zi (23:00–24:00) handling, the QiMen 拆補 term basis
  and the 寄坤 counting rule (see `docs/CALCULATION.md`), and the Vedic D6 convention.
- **No calibration.** Nothing ties the 0–100 scale or the grade cut-offs to a distribution of
  real charts. The spread curve shapes the spread, not the meaning.

## How to change the model safely

- Keep the weights summing to 1.0 *before* renormalisation, or update the table above.
- Any change to a formula or threshold should update this file and the tests in
  `services/tier.rs` and `tests/destiny_tier_omni_tests.rs` in the same commit.
- Bump `tier_model_version` (currently `7.0.0`) when scores for the same input change.
