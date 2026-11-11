# Eon Saju Analysis Engine Documentation

The Eon engine is a high-performance destiny intelligence engine combining classical metaphysical calculations with modern computational modeling. This document outlines the core architecture, layers, and specifications implemented across the engine.

## Module Architecture Overview

The engine is structured into 3 primary conceptual layers: **Calculation**, **Analysis**, and **Interpretation**.

### 1. Calculation Layer (`eon-core`, `eon-astro`, `eon-saju/pillars`)

- **Astronomical Precision**: Swiss Ephemeris (NASA JPL ephemeris algorithms) for exact solar ecliptic longitude and true solar time correction.
- **GanZi Computation**: Exact 24 solar terms determination (15° solar longitude intervals) for celestial pillar assignment.
- **Precision Major Luck**: Minute-resolution solar term ingress tracking to compute exact major luck transition milestones.
- **Zi-Hour Options**: Flexible early rat hour / night rat hour handling.

### 2. Analysis Layer (`eon-saju/power`, `eon-saju/transformations`)

- **110-Point Weighting Method**: Empirical quantitative scoring of Five Elements and Ten Gods.
- **Transformations (合化)**: Tracking actual elemental shifts resulting from Heavenly Stem and Earthly Branch combinations.
- **Correction Modes**: Position weighting and seasonal climate adjustments (e.g. wet vs dry Earth across seasons).

### 3. Interpretation Layer (`eon-saju/structure`, `eon-saju/yongshin`, `eon-saju/spirit_markers`)

- **Structure (格局)**: Month branch hidden stems manifestation for standard and exotic special structures.
- **Useful God (用神)**: Synthesis of Eokbu (balance), JoHu (climate), TongGwan (mediation), and ByeongYak (pathology/remedy).
- **Spirit Markers & Interactions**: Comprehensive analysis of 20+ spiritual stars, clashes, combinations, punishments, and harms.

## Documentation Index

1. [Calculation Logic Specifications (CALCULATION.md)](./CALCULATION.md) - Astronomical time correction and GanZi determination.
2. [Analysis & Scoring Specifications (ANALYSIS.md)](./ANALYSIS.md) - 110-point method and calibration modes.
3. [Structure & Useful God Specifications (INTERPRETATION.md)](./INTERPRETATION.md) - Metaphysical synthesis and recommendation algorithms.
