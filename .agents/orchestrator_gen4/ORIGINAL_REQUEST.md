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
