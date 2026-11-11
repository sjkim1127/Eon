# Calculation Specifications (Calculation Details)

This document describes the temporal calibration, true solar time adjustments, and astronomical rules implemented to derive exact Four Pillars GanZi.

## 1. True Solar Time (Local Meridian) Correction

Adjusts for the longitudinal deviation between standard time zones (e.g. Korea Standard Time UTC+9, 135.0°E) and the subject's actual birthplace longitude.

- **Formula**: `Standard Time - (Standard Meridian - Birth Longitude) * 4 minutes`
- **Example (Ansan, 126.8°E)**:
  - `(135.0 - 126.8) * 4 = 32.8 minutes` (~33 min difference)
  - Solar noon occurs 33 minutes later than standard meridian noon; hence **33 minutes are subtracted** from local clock time before pillar derivation.

## 2. Year Pillar Solar Transition on Li Chun (立春)

The year boundary is determined astronomically by Li Chun (solar ecliptic longitude exactly 315°), rather than Gregorian January 1 or Lunar New Year.

- Birth before exact Li Chun timestamp: preceding year GanZi.
- Birth at or after exact Li Chun timestamp: current year GanZi.

## 3. Rat Hour (子時) Handling

The engine supports the standard Early Rat Hour system (and configurable Night Rat Hour):

- **Early Rat Window**: 23:00 to 01:00 (after true solar time adjustment).
- **Day Pillar**: At 23:00, the Day Pillar advances to the next day.
- **Hour Pillar**: The corresponding Zi hour GanZi (Jia-Zi, Bing-Zi, etc.) is sequentially applied.

## 4. Month Pillar Solar Term Boundaries

Each seasonal month begins at the entry of the corresponding major solar term (e.g. JingZhi for Mao month, QingMing for Chen month).

- The engine calculates exact minute-level solar term crossings via `swisseph-rs` astronomical algorithms to assign the Month branch.
