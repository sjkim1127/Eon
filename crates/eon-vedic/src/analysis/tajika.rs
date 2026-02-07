use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TajikaAspectType {
    Mitra(bool),  // Friend (true = Very Friendly, false = Friendly)
    Shatru(bool), // Enemy (true = Very Hostile, false = Hostile)
    Sama,         // Neutral
}

pub struct TajikaEngine;

impl TajikaEngine {
    /// Calculate the Muntha (Annual Progressed Point)
    /// Rule: Muntha starts at Lagna at birth and progresses 1 sign per year.
    /// Muntha = (Birth Lagna Sign + Years Elapsed) % 12
    pub fn calculate_muntha(birth_lagna_rasi: u8, age_years: u32) -> u8 {
        let rasi = (birth_lagna_rasi as u32 + age_years - 1) % 12 + 1;
        rasi as u8
    }

    /// Tajika Aspects (Drishti)
    /// Friendly: 3, 5, 9, 11 houses apart
    /// Hostile: 1, 4, 7, 10 houses apart
    /// Neutral: 2, 6, 8, 12 houses apart
    pub fn get_aspect_type(house_diff_1_indexed: u8) -> TajikaAspectType {
        match house_diff_1_indexed {
            3 | 11 => TajikaAspectType::Mitra(false),  // Friendly
            5 | 9 => TajikaAspectType::Mitra(true),    // Very Friendly
            1 | 7 => TajikaAspectType::Shatru(true),   // Very Hostile
            4 | 10 => TajikaAspectType::Shatru(false), // Hostile
            _ => TajikaAspectType::Sama,               // Neutral
        }
    }

    /// Ithasala Yoga (Applying Aspect)
    /// Rule: Faster planet behind slower planet within orb.
    /// This is a simplified version of the logic.
    pub fn check_ithasala(
        p1_long: f64,
        p1_speed: f64,
        p2_long: f64,
        p2_speed: f64,
        orb: f64,
    ) -> bool {
        // Find who is faster
        let (faster_long, _faster_speed, slower_long, _slower_speed) = if p1_speed > p2_speed {
            (p1_long, p1_speed, p2_long, p2_speed)
        } else {
            (p2_long, p2_speed, p1_long, p1_speed)
        };

        let diff = (slower_long - faster_long + 360.0) % 360.0;

        // Applying if within orb and faster is "behind" slower (diff < orb)
        diff > 0.0 && diff <= orb
    }
}
