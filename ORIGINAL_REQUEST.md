# Vedic & Saju Core Engine Refinement Brief

Enhance and visualize key areas of Vedic Astrology (Ashtakoota Guna Milan 36-point compatibility, Shadbala 6-fold strength visualization, KP System House/Planetary Significators, and hierarchical Dasha timelines) across the Rust engine (`crates/eon-vedic`, `crates/eon-service`) and CLI/formatters.

## Implementation Tasks

### R1. Ashtakoota Guna Milan Compatibility Refinement
- Compute detailed scores (up to 36 points) across the 8 Ashtakoota factors (Varna, Vashya, Tara, Yoni, Maitri, Gana, Bhakoot, Nadi) with matching results in `crates/eon-vedic`.
- Provide clean formatted output and reports detailing individual factor points and interpretive text.

### R2. Shadbala & Bhava Bala 6-Fold Strength Visualization
- Extract the 6 distinct factors of Shadbala (Sthana Bala, Dig Bala, Kala Bala, Cheshta Bala, Naisargika Bala, Drik Bala) in `crates/eon-vedic`.
- Provide formatted scorecards detailing each factor's contribution relative to required Rupa benchmarks.

### R3. KP System House Cusps & Significators
- Implement KP System 12-house cusps and planetary significators (Sign, Star, Sub Lord linkage analysis).
- Support locale keys and structured tables for high-density CLI/agent prompt injection.

### R4. Hierarchical Multi-Level Dasha Timelines
- Compute nested Maha Dasha, Antar Dasha, and Pratyantar Dasha timelines with current time highlighting.

### Saju Engine Precision & Edge Cases
- Complete Saju analysis core engine (`crates/eon-saju`) precision, special structures, dynamic luck simulation, and comprehensive edge-case test suites.
- Fully verify Eokbu, JoHu, TongGwan, and ByeongYak useful god recommendations.
