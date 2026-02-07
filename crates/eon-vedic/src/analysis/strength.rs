use crate::chart::{VedicChart, VedicPosition};
use crate::constants::*;
use crate::planets::VedicPlanet;
use chrono::Timelike;
use serde::{Deserialize, Serialize}; // Time calculation needed

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetStrength {
    pub planet: VedicPlanet,
    pub exaltation_score: f64,   // 0.0 ~ 60.0 (Uchcha Bala)
    pub directional_score: f64,  // 0.0 ~ 60.0 (Dig Bala)
    pub chesta_score: f64,       // 0.0 ~ 60.0 (Chesta Bala - Motion)
    pub naisargika_score: f64,   // 0.0 ~ 60.0 (Natural strength)
    pub kala_score: f64,         // 0.0 ~ 60.0 (Time strength - Day/Night)
    pub drik_score: f64,         // Aspect strength (can be negative)
    pub paksha_score: f64,       // Moon Phase strength
    pub ayana_score: f64,        // Declination strength
    pub saptavargaja_score: f64, // 0.0 ~ 60.0 (Strength across 7 Vargas)
    // Additional Sthana Bala components (BPHS)
    pub kendra_bala: f64,           // 0.0 ~ 60.0 (Kendra house strength)
    pub drekkana_bala: f64,         // 0.0 ~ 60.0 (Drekkana strength)
    pub ojayugmarasyamsa_bala: f64, // 0.0 ~ 15.0 (Odd/Even sign strength)
    pub yuddha_bala: f64,           // Planetary war adjustment (can be positive or negative)
    pub ishta_phala: f64,           // Auspiciousness (0-60)
    pub kashta_phala: f64,          // Inauspiciousness (0-60)
    pub total_score: f64,           // Aggregate for MVP
    pub status: String,             // "Exalted", "Debilitated", "Strong", "Weak", "Neutral"
}

pub struct StrengthEngine;

impl StrengthEngine {
    /// Calculate basic strength metrics (Shadbala with BPHS Sthana Bala)
    pub fn calculate(pos: &VedicPosition, chart: &crate::chart::VedicChart) -> PlanetStrength {
        let ex_score = Self::calculate_uchcha_bala(pos.planet, pos.sidereal_deg);
        let dig_score = Self::calculate_dig_bala(pos.planet, pos.house_index);
        let chesta_score = Self::calculate_chesta_bala(pos, chart);
        let naisargika_score = Self::calculate_naisargika_bala(pos.planet);
        let kala_score = Self::calculate_kala_bala(pos.planet, chart);
        let ayana_score = Self::calculate_ayana_bala(pos.planet, pos.sidereal_deg);

        let drik_score = Self::calculate_drik_bala(pos, chart);
        let paksha_score = Self::calculate_paksha_bala(pos.planet, chart);

        let sapta_score = Self::calculate_saptavargaja_bala(pos);

        // Additional Sthana Bala components (BPHS)
        let kendra_bala = Self::calculate_kendra_bala(pos.house_index);
        let drekkana_bala = Self::calculate_drekkana_bala(pos);
        let ojayugmarasyamsa_bala = Self::calculate_ojayugmarasyamsa_bala(pos);

        // Ishta & Kashta Phala based on Exaltation (Uchcha) and Motion (Chesta)
        let (ishta_phala, kashta_phala) = Self::calculate_ishta_kashta(ex_score, chesta_score);

        // Yuddha Bala (Planetary War) - calculate once for the chart
        // We'll compute this per-planet context within calculate for now
        // Note: In production, this should be cached at chart level to avoid recalculation
        let yuddha_scores = Self::calculate_yuddha_bala(chart);
        let yuddha_bala = *yuddha_scores.get(&pos.planet).unwrap_or(&0.0);

        let total = ex_score
            + dig_score
            + chesta_score
            + naisargika_score
            + kala_score
            + drik_score
            + paksha_score
            + ayana_score
            + sapta_score
            + kendra_bala
            + drekkana_bala
            + ojayugmarasyamsa_bala
            + yuddha_bala; // Add planetary war adjustment

        // Simple status determination
        let status = if ex_score >= 50.0 {
            "Exalted".to_string()
        } else if ex_score <= 10.0 {
            "Debilitated".to_string()
        } else if total > 240.0 {
            "Strong".to_string()
        } else if total < 120.0 {
            "Weak".to_string()
        } else {
            "Neutral".to_string()
        };

        PlanetStrength {
            planet: pos.planet,
            exaltation_score: ex_score,
            directional_score: dig_score,
            chesta_score, // Store the final value (Paksha for Moon)
            naisargika_score,
            kala_score,
            drik_score,
            paksha_score,
            ayana_score,
            saptavargaja_score: sapta_score,
            kendra_bala,
            drekkana_bala,
            ojayugmarasyamsa_bala,
            yuddha_bala,
            ishta_phala,
            kashta_phala,
            total_score: total,
            status,
        }
    }

    /// Ishta & Kashta Phala (BPHS Standard)
    /// Based on Uchcha Bala and Chesta Bala.
    /// Formula (BPHS):
    /// Ishta = sqrt(Uchcha * Chesta) (Geometric Mean)
    /// Kashta = 60 - Ishta
    fn calculate_ishta_kashta(uchcha: f64, chesta: f64) -> (f64, f64) {
        // BPHS: Use geometric mean instead of arithmetic mean
        let ishta = (uchcha * chesta).sqrt();
        let kashta = (60.0 - ishta).max(0.0);
        (ishta, kashta)
    }

    /// Ayana Bala (Equinoctial/Declination Strength) - BPHS Standard
    /// Based on planet's declination and nature.
    /// BPHS Formula: (Ecliptic Obliquity ± Declination) / (2 * Ecliptic Obliquity) * 60
    /// Ecliptic Obliquity = 23°27' = 23.45°
    /// Sun, Mars, Jupiter, Venus, Mercury: Strong in North (+).
    /// Moon, Saturn: Strong in South (-).
    fn calculate_ayana_bala(planet: VedicPlanet, declination: f64) -> f64 {
        use crate::core::constants::ECLIPTIC_OBLIQUITY;

        let direction_factor = match planet {
            VedicPlanet::Sun
            | VedicPlanet::Mars
            | VedicPlanet::Jupiter
            | VedicPlanet::Venus
            | VedicPlanet::Mercury => 1.0, // BPHS: Mercury is North-strong, not neutral
            VedicPlanet::Moon | VedicPlanet::Saturn => -1.0,
            _ => return 30.0,
        };

        // BPHS Formula: (23.45 + direction * dec) / (2 * 23.45) * 60
        // For north-strong planets with max north declination (+23.45): (23.45 + 23.45) / 46.9 * 60 = 60
        // For north-strong planets with max south declination (-23.45): (23.45 - 23.45) / 46.9 * 60 = 0
        let score = (ECLIPTIC_OBLIQUITY + direction_factor * declination)
            / (2.0 * ECLIPTIC_OBLIQUITY)
            * 60.0;

        score.max(0.0).min(60.0)
    }

    /// Paksha Bala (Moon Phase Strength)
    /// Benefics (Jup, Ven, Mon, Mer) gain in Waxing (Shukla).
    /// Malefics (Sun, Mar, Sat) gain in Waning (Krishna).
    fn calculate_paksha_bala(planet: VedicPlanet, chart: &crate::chart::VedicChart) -> f64 {
        let sun = chart.planets.iter().find(|p| p.planet == VedicPlanet::Sun);
        let moon = chart.planets.iter().find(|p| p.planet == VedicPlanet::Moon);

        if let (Some(s), Some(m)) = (sun, moon) {
            let mut angle = m.sidereal_deg - s.sidereal_deg;
            if angle < 0.0 {
                angle += 360.0;
            }

            // Paksha Point (0 to 60)
            // 0 (New Moon) -> 180 (Full Moon) -> 360 (New Moon)
            // Waxing: 0 to 180. Point increases.
            // Waning: 180 to 360. Point decreases.

            let moon_strength_base = if angle <= 180.0 {
                // Waxing: 0 -> 60 (at 180 deg)
                angle / 3.0
            } else {
                // Waning: 60 -> 0
                (360.0 - angle) / 3.0
            };

            let is_benefic = match planet {
                VedicPlanet::Jupiter
                | VedicPlanet::Venus
                | VedicPlanet::Moon
                | VedicPlanet::Mercury => true,
                _ => false,
            };

            if is_benefic {
                moon_strength_base
            } else {
                60.0 - moon_strength_base
            }
        } else {
            30.0
        }
    }

    /// Drik Bala (Aspect Strength) - BPHS Standard
    /// Calculates the sum of aspect values (Drishti) from all other planets.
    /// Benefic aspects add positive strength, malefic aspects subtract.
    /// No arbitrary scaling - direct aspect values are used.
    fn calculate_drik_bala(pos: &VedicPosition, chart: &crate::chart::VedicChart) -> f64 {
        let mut total_drik = 0.0;

        for aspector in &chart.planets {
            if aspector.planet == pos.planet {
                continue;
            }

            let diff = (pos.sidereal_deg - aspector.sidereal_deg + 360.0) % 360.0;
            let val = Self::get_aspect_value(aspector.planet, diff);

            // Influence of aspecting planet nature (BPHS standard)
            let is_malefic = matches!(
                aspector.planet,
                VedicPlanet::Sun
                    | VedicPlanet::Mars
                    | VedicPlanet::Saturn
                    | VedicPlanet::Rahu
                    | VedicPlanet::Ketu
            );

            // BPHS: Direct aspect values (no division by 4)
            if is_malefic {
                total_drik -= val;
            } else {
                total_drik += val;
            }
        }

        total_drik
    }

    fn get_aspect_value(planet: VedicPlanet, diff: f64) -> f64 {
        // BPHS standard aspect strength in Virupas (1 Virupa = 1/60 of full strength)
        // Fixed formula based on BPHS Chapter 27, Verse 23-25
        let mut val = if diff > 30.0 && diff <= 60.0 {
            // 30° to 60°: (D - 30) / 2 → 0 to 15 Virupas
            (diff - 30.0) / 2.0
        } else if diff > 60.0 && diff <= 90.0 {
            // 60° to 90°: (D - 60) + 15 → 15 to 45 Virupas (increasing)
            (diff - 60.0) + 15.0
        } else if diff > 90.0 && diff <= 120.0 {
            // 90° to 120°: 45 - (D - 90) / 2 → 45 to 30 Virupas (decreasing)
            45.0 - (diff - 90.0) / 2.0
        } else if diff > 120.0 && diff <= 150.0 {
            // 120° to 150°: 30 - (D - 120) → 30 to 0 Virupas (decreasing)
            30.0 - (diff - 120.0)
        } else if diff > 150.0 && diff <= 180.0 {
            // 150° to 180°: (D - 150) * 2 → 0 to 60 Virupas (increasing to maximum)
            (diff - 150.0) * 2.0
        } else if diff > 180.0 && diff <= 300.0 {
            // 180° to 300°: Mirror of 60° to 180° (decreasing from 60)
            let reverse_diff = 360.0 - diff; // Convert to equivalent forward angle
            if reverse_diff <= 30.0 {
                0.0
            } else if reverse_diff <= 60.0 {
                (reverse_diff - 30.0) / 2.0
            } else if reverse_diff <= 90.0 {
                (reverse_diff - 60.0) + 15.0
            } else if reverse_diff <= 120.0 {
                45.0 - (reverse_diff - 90.0) / 2.0
            } else if reverse_diff <= 150.0 {
                30.0 - (reverse_diff - 120.0)
            } else {
                (reverse_diff - 150.0) * 2.0
            }
        } else {
            0.0
        };

        // Special Aspects (these override the standard curve)
        match planet {
            VedicPlanet::Mars => {
                // Mars has full aspect (60) at 4th (90°) and 8th (210°)
                if (diff - 90.0).abs() < 15.0 {
                    val = 60.0;
                }
                if (diff - 210.0).abs() < 15.0 {
                    val = 60.0;
                }
            }
            VedicPlanet::Jupiter => {
                // Jupiter has full aspect (60) at 5th (120°) and 9th (240°)
                if (diff - 120.0).abs() < 15.0 {
                    val = 60.0;
                }
                if (diff - 240.0).abs() < 15.0 {
                    val = 60.0;
                }
            }
            VedicPlanet::Saturn => {
                // Saturn has full aspect (60) at 3rd (60°) and 10th (270°)
                if (diff - 60.0).abs() < 15.0 {
                    val = 60.0;
                }
                if (diff - 270.0).abs() < 15.0 {
                    val = 60.0;
                }
            }
            _ => {}
        }

        val.max(0.0).min(60.0)
    }

    /// Naisargika Bala (Natural Strength)
    /// Fixed values from Sun (strongest) to Saturn (weakest)
    fn calculate_naisargika_bala(planet: VedicPlanet) -> f64 {
        match planet {
            VedicPlanet::Sun => 60.0,
            VedicPlanet::Moon => 51.43,
            VedicPlanet::Venus => 42.86,
            VedicPlanet::Jupiter => 34.29,
            VedicPlanet::Mercury => 25.71,
            VedicPlanet::Mars => 17.14,
            VedicPlanet::Saturn => 8.57,
            _ => 0.0,
        }
    }

    /// Kala Bala (Time Strength) - Updated with Tribhaga Logic
    fn calculate_kala_bala(planet: VedicPlanet, chart: &VedicChart) -> f64 {
        let mut score = 0.0;
        let p = &chart.panchanga;

        // 1. Nathonnata Bala (Diva-Ratri)
        let div_ratri = if planet == VedicPlanet::Mercury {
            60.0
        } else {
            let is_day_strong = matches!(
                planet,
                VedicPlanet::Sun | VedicPlanet::Jupiter | VedicPlanet::Venus
            );
            let is_night_strong = matches!(
                planet,
                VedicPlanet::Moon | VedicPlanet::Mars | VedicPlanet::Saturn
            );
            if p.is_day_birth && is_day_strong {
                60.0
            } else if !p.is_day_birth && is_night_strong {
                60.0
            } else {
                0.0
            }
        };
        score += div_ratri;

        // 2. Vara Bala (Day Lord)
        if planet == p.day_lord {
            score += 45.0;
        }

        // 3. Hora Bala (Hour Lord)
        if planet == p.hour_lord {
            score += 60.0;
        }

        // 4. Tribhaga Bala (Day/Night 3 Parts)
        // Jupiter always gets 60.
        if planet == VedicPlanet::Jupiter {
            score += 60.0;
        } else {
            // Determine the 3 lords based on Day or Night
            // Day Lords: Mercury, Sun, Saturn
            // Night Lords: Moon, Venus, Mars
            let lords = if p.is_day_birth {
                [VedicPlanet::Mercury, VedicPlanet::Sun, VedicPlanet::Saturn]
            } else {
                [VedicPlanet::Moon, VedicPlanet::Venus, VedicPlanet::Mars]
            };

            // Calculate duration in seconds to find which part
            let sr_secs = p.sunrise.num_seconds_from_midnight();
            let ss_secs = p.sunset.num_seconds_from_midnight();
            let bt_secs = p.current_time.time().num_seconds_from_midnight();

            let (start_secs, duration_secs) = if p.is_day_birth {
                (sr_secs, ss_secs.wrapping_sub(sr_secs)) // Simple subtraction for day
            } else {
                // Night duration = (24h - sunset) + sunrise
                let day_seconds = 24 * 3600;
                let night_len = (day_seconds - ss_secs) + sr_secs;
                (ss_secs, night_len)
            };

            // Elapsed time from start of period
            let elapsed = if bt_secs >= start_secs {
                bt_secs - start_secs
            } else {
                // Wrapped around midnight (e.g. Birth 2AM, Start 18PM)
                (bt_secs + 24 * 3600) - start_secs
            };

            let part_len = duration_secs / 3;
            if part_len > 0 {
                let idx = (elapsed / part_len).min(2) as usize; // Clamp to max 2
                if lords[idx] == planet {
                    score += 60.0;
                }
            }
        }

        score
    }

    /// Chesta Bala (Motion Strength) - BPHS Standard
    /// Planets gain strength based on their motion state classification.
    /// BPHS defines 7-8 motion states with specific scores:
    /// - Vakra (Retrograde): 60
    /// - Anuvakra (Post-retrograde stationary): 30  
    /// - Vikala (Very Slow): 15
    /// - Mandatara (Slow): 7.5
    /// - Manda (Medium): 15
    /// - Sama (Average): 30
    /// - Chara (Fast): 45
    /// - Ati-chara (Very Fast): 60
    ///
    /// For Sun and Moon (which don't retrograde), BPHS uses Ayana-based calculation:
    /// - Sun: Based on Uttarayana (northward) vs Dakshinayana (southward) movement
    /// - Moon: Based on Paksha (waxing vs waning) already covered in Paksha Bala
    fn calculate_chesta_bala(pos: &VedicPosition, chart: &VedicChart) -> f64 {
        use crate::core::constants::*;

        // Sun: BPHS Ayana-based Chesta Bala
        // Maximum when moving North (increasing declination), minimum when moving South
        if pos.planet == VedicPlanet::Sun {
            // Sun's declination ranges from -23.45° to +23.45°
            // Chesta Bala = [(Declination + 23.45) / (2 * 23.45)] * 60
            // This gives 0 at minimum declination, 60 at maximum
            let dec = pos.declination;
            let chesta = ((dec + ECLIPTIC_OBLIQUITY) / (2.0 * ECLIPTIC_OBLIQUITY)) * 60.0;
            return chesta.max(0.0).min(60.0);
        }

        // Moon: Paksha-based strength
        if pos.planet == VedicPlanet::Moon {
            return Self::calculate_paksha_bala(VedicPlanet::Moon, chart);
        }

        // Get speed in degrees per day
        let speed = pos.speed;

        // Retrograde planets always get maximum Chesta Bala
        if pos.is_retrograde || speed < 0.0 {
            return 60.0;
        }

        // Select appropriate motion state thresholds based on planet
        let motion_states: &[(&str, f64, f64, f64)] = match pos.planet {
            VedicPlanet::Mars => &MARS_MOTION_STATES,
            VedicPlanet::Mercury => &MERCURY_MOTION_STATES,
            VedicPlanet::Jupiter => &JUPITER_MOTION_STATES,
            VedicPlanet::Venus => &VENUS_MOTION_STATES,
            VedicPlanet::Saturn => &SATURN_MOTION_STATES,
            _ => return 30.0, // Rahu/Ketu get neutral
        };

        // Find which motion state the planet is in based on its current speed
        for &(_name, min_speed, max_speed, score) in motion_states {
            if speed >= min_speed && speed < max_speed {
                return score;
            }
        }

        // Default to average if somehow no match (shouldn't happen with proper thresholds)
        30.0
    }

    /// Uchcha Bala (Exaltation Strength)
    /// Max 60 units at Deep Exaltation point, 0 units at Deep Debilitation point.
    fn calculate_uchcha_bala(planet: VedicPlanet, longitude: f64) -> f64 {
        let deep_exalt_deg = match planet {
            VedicPlanet::Sun => DEEP_EXALT_SUN,
            VedicPlanet::Moon => DEEP_EXALT_MOON,
            VedicPlanet::Mars => DEEP_EXALT_MARS,
            VedicPlanet::Mercury => DEEP_EXALT_MERCURY,
            VedicPlanet::Jupiter => DEEP_EXALT_JUPITER,
            VedicPlanet::Venus => DEEP_EXALT_VENUS,
            VedicPlanet::Saturn => DEEP_EXALT_SATURN,
            _ => return 30.0, // Nodes/ASC default
        };

        let deep_debilit_deg = (deep_exalt_deg + 180.0) % 360.0;

        // Arc distance from Deep Debilitation point
        let mut arc = (longitude - deep_debilit_deg).abs();
        if arc > 180.0 {
            arc = 360.0 - arc;
        }

        // Score = Distance / 3 (since 180 degrees = 60 units)
        arc / 3.0
    }

    /// Dig Bala (Directional Strength)
    /// Max 60 units at powerful house, 0 units at opposite (weakest) house.
    fn calculate_dig_bala(planet: VedicPlanet, house: u8) -> f64 {
        let power_house = match planet {
            VedicPlanet::Mercury | VedicPlanet::Jupiter => 1,
            VedicPlanet::Sun | VedicPlanet::Mars => 10,
            VedicPlanet::Saturn => 7,
            VedicPlanet::Moon | VedicPlanet::Venus => 4,
            _ => return 30.0,
        };

        let weak_house = match power_house {
            1 => 7,
            10 => 4,
            7 => 1,
            4 => 10,
            _ => 1,
        };

        // Shortest distance in houses (12 houses total)
        let diff = (house as i32 - weak_house as i32).abs();
        let dist_houses = if diff > 6 { 12 - diff } else { diff };

        // Score = (Houses Dist / 6) * 60 = Houses Dist * 10
        dist_houses as f64 * 10.0
    }

    /// Saptavargaja Bala (Strength across 7 Vargas)
    /// D1, D2, D3, D7, D9, D12, D30
    /// Points: Great Friend (45), Friend (30), Neutral (15), Enemy (10), Great Enemy (5) -> Normalize to 0~60 (sum / 7)
    fn calculate_saptavargaja_bala(pos: &VedicPosition) -> f64 {
        let vargas = [
            pos.rasi,
            pos.hora_rasi,
            pos.drekkana_rasi,
            pos.saptamsa_rasi,
            pos.navamsa_rasi,
            pos.dwadasamsa_rasi,
            pos.trimsamsa_rasi,
        ];

        let mut total_v_points = 0.0;
        for rasi in vargas {
            let lord = VedicPlanet::get_ruler_of(rasi);
            // We use Natural Friendship (Naisargika) for Saptavargaja usually
            let relation = pos.planet.naisargika_relation(lord);

            let pts = if pos.planet == lord {
                45.0
            }
            // Own Sign
            else if relation == 1 {
                30.0
            }
            // Friend
            else if relation == 0 {
                15.0
            }
            // Neutral
            else {
                5.0
            }; // Enemy

            total_v_points += pts;
        }

        // Normalize sum (max 45*7 = 315) to 60 units
        (total_v_points / 315.0) * 60.0
    }

    /// Kendra Bala (BPHS Sthana Bala component)
    /// Planets in Kendra houses (1, 4, 7, 10) get full strength (60)
    /// Planets in Panaphara houses (2, 5, 8, 11) get half strength (30)
    /// Planets in Apoklima houses (3, 6, 9, 12) get quarter strength (15)
    fn calculate_kendra_bala(house: u8) -> f64 {
        match house {
            1 | 4 | 7 | 10 => 60.0, // Kendra (Angular)
            2 | 5 | 8 | 11 => 30.0, // Panaphara (Succedent)
            3 | 6 | 9 | 12 => 15.0, // Apoklima (Cadent)
            _ => 0.0,
        }
    }

    /// Drekkana Bala (BPHS Sthana Bala component)
    /// Based on masculine/feminine/neuter nature of planet and drekkana placement
    /// Masculine planets: Sun, Mars, Jupiter
    /// Feminine planets: Moon, Venus
    /// Neuter planets: Mercury, Saturn
    fn calculate_drekkana_bala(pos: &VedicPosition) -> f64 {
        // Determine planet nature
        let is_masculine = matches!(
            pos.planet,
            VedicPlanet::Sun | VedicPlanet::Mars | VedicPlanet::Jupiter
        );
        let is_feminine = matches!(pos.planet, VedicPlanet::Moon | VedicPlanet::Venus);

        // Get drekkana sign (D3)
        let drekk_rasi = pos.drekkana_rasi;

        // Odd signs (1,3,5,7,9,11) are masculine
        // Even signs (2,4,6,8,10,12) are feminine
        let drekk_is_masculine = drekk_rasi % 2 == 1;

        // BPHS: Matching nature gets 15 points
        if is_masculine && drekk_is_masculine {
            15.0
        } else if is_feminine && !drekk_is_masculine {
            15.0
        } else if !is_masculine && !is_feminine {
            // Neuter planets (Mercury, Saturn) get 7.5 always
            7.5
        } else {
            0.0
        }
    }

    /// Ojayugmarasyamsa Bala (BPHS Sthana Bala component)
    /// Based on odd/even sign placement in D1 (Rasi) and D9 (Navamsa)
    /// Masculine planets get strength in odd signs
    /// Feminine planets get strength in even signs
    /// Mercury gets strength in both (neuter)
    fn calculate_ojayugmarasyamsa_bala(pos: &VedicPosition) -> f64 {
        let is_masculine = matches!(
            pos.planet,
            VedicPlanet::Sun | VedicPlanet::Mars | VedicPlanet::Jupiter
        );
        let is_feminine = matches!(pos.planet, VedicPlanet::Moon | VedicPlanet::Venus);
        let is_mercury = pos.planet == VedicPlanet::Mercury;

        let rasi_is_odd = pos.rasi % 2 == 1;
        let navamsa_is_odd = pos.navamsa_rasi % 2 == 1;

        let mut score = 0.0;

        // D1 (Rasi) strength: 7.5 points max
        if is_mercury {
            score += 7.5; // Mercury always gets points
        } else if is_masculine && rasi_is_odd {
            score += 7.5;
        } else if is_feminine && !rasi_is_odd {
            score += 7.5;
        }

        // D9 (Navamsa) strength: 7.5 points max
        if is_mercury {
            score += 7.5; // Mercury always gets points
        } else if is_masculine && navamsa_is_odd {
            score += 7.5;
        } else if is_feminine && !navamsa_is_odd {
            score += 7.5;
        }

        score
    }

    /// Get relative size/brightness for planetary war determination
    /// Larger/brighter planets win when declinations are close
    /// Order: Jupiter > Saturn > Mars > Venus > Mercury (approximate)
    fn get_planet_relative_size(planet: VedicPlanet) -> f64 {
        match planet {
            VedicPlanet::Jupiter => 5.0,
            VedicPlanet::Saturn => 4.0,
            VedicPlanet::Mars => 3.0,
            VedicPlanet::Venus => 2.0,
            VedicPlanet::Mercury => 1.0,
            _ => 0.0, // Should not happen (only war participants)
        }
    }

    /// Yuddha Bala (Planetary War) - BPHS Standard
    /// When two planets (excluding Sun, Moon, Rahu, Ketu) are within 1 degree,
    /// a planetary war occurs. The planet with higher latitude wins.
    /// Winner gains strength, loser loses strength.
    ///
    /// BPHS Chapter 27, Verse 31-33:
    /// - War occurs when planets are within 1° in longitude
    /// - Sun and Moon don't participate in wars
    /// - Rahu and Ketu don't participate in wars
    /// - Winner: Planet with higher latitude (farther from ecliptic)
    /// - Winner gains 60 Virupas, Loser loses 60 Virupas
    fn calculate_yuddha_bala(
        chart: &crate::chart::VedicChart,
    ) -> std::collections::HashMap<VedicPlanet, f64> {
        let mut yuddha_scores = std::collections::HashMap::new();

        // Only Mars, Mercury, Jupiter, Venus, and Saturn can engage in wars
        let war_participants = vec![
            VedicPlanet::Mars,
            VedicPlanet::Mercury,
            VedicPlanet::Jupiter,
            VedicPlanet::Venus,
            VedicPlanet::Saturn,
        ];

        // Find all planet positions
        let mut positions: Vec<(&VedicPosition, VedicPlanet)> = Vec::new();
        for planet in &war_participants {
            if let Some(pos) = chart.planets.iter().find(|p| p.planet == *planet) {
                positions.push((pos, *planet));
            }
        }

        // Check each pair of planets
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let (pos1, p1) = positions[i];
                let (pos2, p2) = positions[j];

                // Calculate longitudinal difference
                let mut long_diff = (pos1.sidereal_deg - pos2.sidereal_deg).abs();
                if long_diff > 180.0 {
                    long_diff = 360.0 - long_diff;
                }

                // War occurs if within 1 degree
                if long_diff <= 1.0 {
                    // Winner is the planet with higher latitude (farther from ecliptic)
                    let dec1 = pos1.declination.abs();
                    let dec2 = pos2.declination.abs();

                    // BPHS: Winner has higher declination (farther from ecliptic)
                    // If declinations are close, use inherent size
                    let size1 = Self::get_planet_relative_size(p1);
                    let size2 = Self::get_planet_relative_size(p2);

                    let (winner, loser) = if (dec1 - dec2).abs() > 0.1 {
                        if dec1 > dec2 {
                            (p1, p2)
                        } else {
                            (p2, p1)
                        }
                    } else {
                        if size1 > size2 {
                            (p1, p2)
                        } else {
                            (p2, p1)
                        }
                    };

                    // BPHS: Winner gains 60 Virupas, Loser loses 60 Virupas
                    *yuddha_scores.entry(winner).or_insert(0.0) += 60.0;
                    *yuddha_scores.entry(loser).or_insert(0.0) -= 60.0;
                }
            }
        }

        yuddha_scores
    }
}
