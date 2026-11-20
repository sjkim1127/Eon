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

## 6a. Major Luck (大運) in the Yearly Timeline

Major luck cycles begin on an exact instant (`MajorLuck.start_date`: 3 days = 1 year from birth to
the next/previous solar term, then every 10 × 365.2425 days). That instant usually falls in the
middle of a calendar year, so the yearly timeline (`LifePathEmulator`) and the high-resolution
fuzz audit assign each saju year the cycle that governs **the majority of that year**: the cycle in
force at ~Aug 4, the midpoint between two Ipchun (`MajorLuckAnalysis::cycle_for_year`).

Example: a cycle starting 2027-12-27 first governs 2028; 2027 still belongs to the previous cycle.
Years before the first cycle use the natal month pillar. The transition-period (交運期) flag and the
volatility term still use the integer start age of the cycle.

The batch simulators (`BatchSimulator`) use the same year-based lookup. `SajuVM::simulate_life`
(used by the transit analysis) works on completed (international) age, like `MajorLuck::start_age`
and the service's `current_age`: frame age N spans birthday N to N+1, and both the major luck
(`MajorLuckAnalysis::cycle_at`) and the saju year (Ipchun taken as ~Feb 4) are looked up at the
midpoint of that span. Example: a Nov 27 birth in 2004 has age 21 = Nov 2025..Nov 2026, which is
the year 2026 (earlier versions used 2024 for it).

Not changed: `at_age` (integer-age lookup) remains for callers that really have an age, such as
`LuckAnalysis::current_major_luck`; `LuckAnalysis::major_luck_for_year` is the year-based
counterpart.

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

## 9. Two-Person Comparisons

- **Western synastry harmony (`SynastryResult::harmony_score`)**: 0-100 balance between supportive
  (trine, sextile, conjunction) and tense (square, opposition, conjunction involving Mars, Saturn or
  Pluto) inter-chart aspects. Each aspect is weighted by orb tightness and by the importance of the
  two bodies (Sun/Moon 3, Mercury/Venus/Mars/Ascendant 2, Jupiter/Saturn 1.5, others 1); the score is
  `50 + 50 * (supportive - tense) / (supportive + tense)`, or 50 with no aspects. It is a heuristic
  balance, not a verdict. (Earlier versions added a flat +4/-3 per aspect and saturated at 0 or 100.)
- **Vedic Ashtakoota**: scored as groom (male) versus bride (female). `compat` uses the stated
  genders when they differ, otherwise the first person is the groom.
- **Saju compatibility**: `CompatibilityAuditor` compares stems and branches of the two charts
  (combinations, clashes, resource contention); the sync score is a heuristic.
- **House systems**: names are matched exactly (Placidus, Koch, WholeSign, Equal, Porphyry,
  Regiomontanus, Campanus, Alcabitius, Morinus, Topocentric, or the Swiss Ephemeris letter). Unknown
  names are rejected rather than mapped to another system.

## 10. Western Timing Techniques and Natal Extras

All techniques are tropical and use the Swiss Ephemeris (Moshier) positions of `eon-astro`.

- **Transits**: the sky at the target instant over the *natal* chart (houses stay natal). Orbs
  are school-dependent: Moon 1°, Sun/Mercury/Venus/Mars 2°, Jupiter to Pluto 3°, nodes/Lilith/Chiron
  1.5°, sextiles 25% tighter. Only the five major aspects are used; the targets are the natal planets
  (without the derived South Node), the Ascendant and the Midheaven.
- **Solar / lunar return**: the instant the Sun (Moon) is back on its natal longitude, found by Newton
  iteration. The solar return belongs to the calendar year given (the one that contains the birthday).
  The lunar return is the first one on or after the date. The return chart can be cast for a
  different place (relocation); the aspects to the natal chart use 5° orbs (4° sextile) because the
  two charts are compared as static pictures.
- **Solar arc**: arc = secondary-progressed Sun minus natal Sun, added to every natal point and to
  the Ascendant and Midheaven. Aspects between directed and natal points use a 1° orb.
- **Annual profection**: the Ascendant advances one sign (and one house) per completed year of life
  (age = elapsed time / 365.2425 days). The lord of the year is the *traditional* ruler of the
  profected sign (Mars for Scorpio, Saturn for Aquarius, Jupiter for Pisces).
- **Antiscia**: mirror across the Cancer/Capricorn axis (180° - longitude); contra-antiscia across the
  Aries/Libra axis (360° - longitude). Contacts within 1°, each pair listed once.
- **Mutual reception**: two of the seven classical planets in each other's domicile signs
  (traditional rulerships). Exaltation-based receptions are not reported.
- **Midpoints**: the shorter-arc midpoint of every pair among Sun to Pluto, True Node, Ascendant and
  Midheaven; a third body on it on the 90° dial (which merges the 0/45/90/135/180° contacts) within
  1° is reported, tightest first, at most 40.

Not implemented: harmonics, fixed stars, Davison chart, triplicity/terms/faces, eclipses, more
Arabic lots, firdaria.
