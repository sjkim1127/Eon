# Original User Request

## Initial Request — 2026-06-20T10:09:43+09:00

Vedic Astrology(Vedic Astrology) 4 / (Ashtakoota Guna Milan Compatibility 36  , Shadbala 6-Fold Strength  , KP System / Significator ,  Dasha ) (rust) (Dioxus UI)      .

Working directory: /Users/sjkim1127/Eon
Integrity mode: benchmark

## Requirements

### R1. Ashtakoota Guna Milan Compatibility  
- `crates/eon-vedic`  Compatibility  Ashtakoota 8 Elements(Varna, Vashya, Tara, Yoni, Maitri, Gana, Bhakoot, Nadi)   ( 36)      .
- Dioxus  Compatibility(Compatibility)  8 Elements         ,         .

### R2. Shadbala & Bhava Bala 6-Fold Strength   
- `crates/eon-vedic`   Shadbala 6 (Sthana Bala, Dig Bala, Kala Bala, Cheshta Bala, Naisargika Bala, Drik Bala)    .
- Dioxus UI  Shadbala     ,     HTML/CSS       (Rupa)       .

### R3. KP System    Significator 
- KP System 12 Cusps    Significator   (Sign/Star/Sub Lord  ) UI  .
- Significator         locale(KO, EN, ZH, RU)    .

### R4.   Dasha  UI 
- Dasha  Major Luck(Maha Dasha)     , Major Luck   (Antar Dasha)  Yearly Luck(Pratyantar Dasha)     UI .
-          " "        .

## Acceptance Criteria

###     (Automated Verification)
- [ ] `cargo check --workspace`          .
- [ ] `cargo test --workspace`       ,       1    .
- [ ] `crates/eon-ui`  `dx build`        .

### UI     (Manual/Visual Verification)
- [ ] Dioxus     Shadbala     Dasha  UI        .
- [ ]           (`crates/eon-ui/src/i18n/`)  ///   .

## Follow-up — 2026-07-25T03:18:07+09:00

Eon  Saju(Saju)   (`crates/eon-saju`)   ,   Gyeokguk     ,       .

Working directory: /Users/sjkim1127/Eon
Integrity mode: demo

## Requirements

### R1. Core Analysis Precision & Pattern Completeness
Eokbu(抑扶), JoHu(調候), TongGwan(通關), ByeongYak(病藥) Yongshin   Gyeokguk/12/      ,   Gyeokguk    .

### R2. Dynamic Luck & Temporal Simulation
Major Luck(大運), Yearly Luck(年運), Monthly Luck(月運)        Hidden Stem/      .

### R3. Codebase Architecture & Edge-Case Verification
`crates/eon-saju`       ,     /  suite  .

## Acceptance Criteria

### Verification & Quality Bar
- [ ] `cargo check --workspace`          .
- [ ] `cargo test -p eon-saju`         100%  .
- [ ]  Saju  ( Gyeokguk  Strong/Weak )  Yongshin, Gyeokguk,         .

