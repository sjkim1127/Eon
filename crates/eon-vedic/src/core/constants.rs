// Basic Astronomical Constants
pub const SIDEREAL_YEAR_DAYS: f64 = 365.256363;
pub const GREGORIAN_YEAR_DAYS: f64 = 365.2425;
pub const SAVANA_YEAR_DAYS: f64 = 360.0;
pub const SECONDS_PER_DAY: f64 = 86400.0;

// Average Daily Motion (Degrees)
pub const AVG_SPEED_MARS: f64 = 0.524;
pub const AVG_SPEED_MERCURY: f64 = 1.4;
pub const AVG_SPEED_JUPITER: f64 = 0.083;
pub const AVG_SPEED_VENUS: f64 = 1.2;
pub const AVG_SPEED_SATURN: f64 = 0.033;
pub const AVG_SPEED_DEFAULT: f64 = 1.0;

// Deep Exaltation Points (Sidereal Longitude)
pub const DEEP_EXALT_SUN: f64 = 10.0; // Aries 10
pub const DEEP_EXALT_MOON: f64 = 33.0; // Taurus 3
pub const DEEP_EXALT_MARS: f64 = 298.0; // Capricorn 28
pub const DEEP_EXALT_MERCURY: f64 = 165.0; // Virgo 15
pub const DEEP_EXALT_JUPITER: f64 = 95.0; // Cancer 5
pub const DEEP_EXALT_VENUS: f64 = 357.0; // Pisces 27
pub const DEEP_EXALT_SATURN: f64 = 200.0; // Libra 20

// Vimshottari Dasha Years
pub const DASHA_YEARS_KETU: f64 = 7.0;
pub const DASHA_YEARS_VENUS: f64 = 20.0;
pub const DASHA_YEARS_SUN: f64 = 6.0;
pub const DASHA_YEARS_MOON: f64 = 10.0;
pub const DASHA_YEARS_MARS: f64 = 7.0;
pub const DASHA_YEARS_RAHU: f64 = 18.0;
pub const DASHA_YEARS_JUPITER: f64 = 16.0;
pub const DASHA_YEARS_SATURN: f64 = 19.0;
pub const DASHA_YEARS_MERCURY: f64 = 17.0;

// Shadbala Constants (BPHS Standard)
/// Ecliptic obliquity in degrees (23°27')
pub const ECLIPTIC_OBLIQUITY: f64 = 23.45;

// Chesta Bala Motion States and Scores
// Based on BPHS classification of planetary speeds
// Format: (name, min_speed (deg/day), max_speed (deg/day), score (0-60))

/// Mars motion thresholds and scores (name, min_speed, max_speed, score)
pub const MARS_MOTION_STATES: [(&str, f64, f64, f64); 7] = [
    ("Vakra (Retrograde)", -2.0, 0.0, 60.0),
    ("Vikala (Very Slow)", 0.0, 0.25, 15.0),
    ("Mandatara (Slow)", 0.25, 0.4, 7.5),
    ("Manda (Medium)", 0.4, 0.5, 15.0),
    ("Sama (Average)", 0.5, 0.6, 30.0),
    ("Chara (Fast)", 0.6, 0.8, 45.0),
    ("Ati-chara (Very Fast)", 0.8, 2.0, 60.0),
];

/// Mercury motion thresholds and scores (name, min_speed, max_speed, score)
pub const MERCURY_MOTION_STATES: [(&str, f64, f64, f64); 7] = [
    ("Vakra (Retrograde)", -2.0, 0.0, 60.0),
    ("Vikala (Very Slow)", 0.0, 1.0, 15.0),
    ("Mandatara (Slow)", 1.0, 1.2, 7.5),
    ("Manda (Medium)", 1.2, 1.4, 15.0),
    ("Sama (Average)", 1.4, 1.6, 30.0),
    ("Chara (Fast)", 1.6, 1.9, 45.0),
    ("Ati-chara (Very Fast)", 1.9, 3.0, 60.0),
];

/// Jupiter motion thresholds and scores (name, min_speed, max_speed, score)
pub const JUPITER_MOTION_STATES: [(&str, f64, f64, f64); 7] = [
    ("Vakra (Retrograde)", -2.0, 0.0, 60.0),
    ("Vikala (Very Slow)", 0.0, 0.05, 15.0),
    ("Mandatara (Slow)", 0.05, 0.07, 7.5),
    ("Manda (Medium)", 0.07, 0.083, 15.0),
    ("Sama (Average)", 0.083, 0.1, 30.0),
    ("Chara (Fast)", 0.1, 0.12, 45.0),
    ("Ati-chara (Very Fast)", 0.12, 0.25, 60.0),
];

/// Venus motion thresholds and scores (name, min_speed, max_speed, score)
pub const VENUS_MOTION_STATES: [(&str, f64, f64, f64); 7] = [
    ("Vakra (Retrograde)", -2.0, 0.0, 60.0),
    ("Vikala (Very Slow)", 0.0, 1.0, 15.0),
    ("Mandatara (Slow)", 1.0, 1.1, 7.5),
    ("Manda (Medium)", 1.1, 1.2, 15.0),
    ("Sama (Average)", 1.2, 1.3, 30.0),
    ("Chara (Fast)", 1.3, 1.5, 45.0),
    ("Ati-chara (Very Fast)", 1.5, 2.5, 60.0),
];

/// Saturn motion thresholds and scores (name, min_speed, max_speed, score)
pub const SATURN_MOTION_STATES: [(&str, f64, f64, f64); 7] = [
    ("Vakra (Retrograde)", -2.0, 0.0, 60.0),
    ("Vikala (Very Slow)", 0.0, 0.02, 15.0),
    ("Mandatara (Slow)", 0.02, 0.028, 7.5),
    ("Manda (Medium)", 0.028, 0.033, 15.0),
    ("Sama (Average)", 0.033, 0.038, 30.0),
    ("Chara (Fast)", 0.038, 0.045, 45.0),
    ("Ati-chara (Very Fast)", 0.045, 0.15, 60.0),
];
