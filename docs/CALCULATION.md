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

## 5. Daylight Saving Time

True solar time is derived from the *standard* (non-DST) clock. The engine removes the DST shift in
effect at the local birth time (`BirthInfo::dst_offset_minutes`) before applying the longitude
correction, so e.g. Seoul 1988-07-01 12:00 (UTC+10) is 11:00 standard time and about 10:28 true solar
time. Year and Month pillars use the absolute UTC instant and are unaffected. Zi Wei Dou Shu uses the
standard clock without the longitude shift. Local times that do not exist (the spring-forward gap) are
moved forward by one hour.

## 6. Unknown Birth Time

When only a date is supplied (`AnalysisInput.unknown_time`, set by the CLI/MCP for date-only input):

- Saju runs on three pillars (the Hour pillar, 得時, 命宮 and 身宮 are omitted; the strength score is
  rescaled to the remaining criteria). The Day pillar is computed from a noon placeholder and never
  rolls over at 23:00. Major Luck start age assumes a noon birth and may move by a few months.
- Vedic, Western, Zi Wei Dou Shu, Human Design, QiMen and I Ching (HeLuo) need the ascendant, houses
  or an hour-based chart and return `BirthTimeRequired`; the omni analysis lists them in
  `skippedEngines`.
- Tier 7.0 renormalises over the evaluated engines (see `docs/TIER.md`).

## 7. QiMen DunJia Conventions (轉盤 時家奇門, 拆補)

Verified by hand-derived reference pans in `crates/eon-qimen/src/builder/tests.rs`:

- Ju table: the standard 24-term table (陽遁 冬至–芒種, 陰遁 夏至–大雪), upper/middle/lower yuan from
  the 15-day 符頭 cycle (甲子, 己卯, 甲午, 己酉).
- Earth plate: 戊己庚辛壬癸丁丙乙 laid forward (陽遁) or backward (陰遁) from the Ju palace in 洛書 order.
- 值符/值使 from the 旬首 stem's palace; the star plate and door plate rotate on the ring
  坎1–艮8–震3–巽4–離9–坤2–兌7–乾6; deities start at the 值符 star's palace (clockwise for 陽遁,
  counter-clockwise for 陰遁).

School-dependent choices the engine currently makes (not verified against a single authority):

- The solar term is taken at the birth instant, not at the 符頭 day (a strict 拆補 reading may differ
  during the days between a term and the next 符頭).
- The centre palace 5 is entrusted to 坤2 (天禽 stays in the centre; the 值使 count starts from 2
  when the 旬首 is in the centre).
- The hour pillar comes from true solar time, whereas many QiMen tools use the standard clock.

## 8. Other School-Dependent Choices

- Zi Wei Dou Shu: leap-month births use the lunar month number only; the 15-day rule (some schools move
  days 16+ to the next month) and a late-Zi (晚子時) day shift are not applied.
- Vedic D6 (Shashtamsa): odd signs start from the sign itself, even signs from the 6th sign from it.
