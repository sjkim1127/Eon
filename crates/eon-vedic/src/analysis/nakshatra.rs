use crate::planets::VedicPlanet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TarabalaCategory {
    Janma = 1,    // Danger to self
    Sampat = 2,   // Wealth and Prosperity
    Vipat = 3,    // Obstacles and Losses
    Kshema = 4,   // Well-being and Security
    Pratyak = 5,  // Opposition and Enemies
    Sadhana = 6,  // Achievement and Success
    Naidhana = 7, // Danger and Endings
    Mitra = 8,    // Friendship and Cooperation
    AtiMitra = 9, // Great Friendship
}

impl TarabalaCategory {
    pub fn description(&self) -> &'static str {
        match self {
            Self::Janma => "Janma (Birth) - Danger to self/health",
            Self::Sampat => "Sampat (Wealth) - Prosperity and gain",
            Self::Vipat => "Vipat (Danger) - Obstacles and accidents",
            Self::Kshema => "Kshema (Safety) - Security and comfort",
            Self::Pratyak => "Pratyak (Obstacle) - Resistance and conflicts",
            Self::Sadhana => "Sadhana (Effort) - Success through hard work",
            Self::Naidhana => "Naidhana (Ending) - Critical danger or finality",
            Self::Mitra => "Mitra (Friend) - Support and help",
            Self::AtiMitra => "Ati-Mitra (Great Friend) - Exceptional gains",
        }
    }

    pub fn is_auspicious(&self) -> bool {
        matches!(
            self,
            Self::Sampat | Self::Kshema | Self::Sadhana | Self::Mitra | Self::AtiMitra
        )
    }
}

pub struct NakshatraEngine;

impl NakshatraEngine {
    pub const NAKSHATRA_NAMES: [&'static str; 27] = [
        "Ashwini",
        "Bharani",
        "Krittika",
        "Rohini",
        "Mrigashira",
        "Ardra",
        "Punarvasu",
        "Pushya",
        "Ashlesha",
        "Magha",
        "Purva Phalguni",
        "Uttara Phalguni",
        "Hasta",
        "Chitra",
        "Swati",
        "Vishakha",
        "Anuradha",
        "Jyeshtha",
        "Mula",
        "Purva Ashadha",
        "Uttara Ashadha",
        "Shravana",
        "Dhanishta",
        "Shatabhisha",
        "Purva Bhadrapada",
        "Uttara Bhadrapada",
        "Revati",
    ];

    pub fn get_name(index_1: u8) -> &'static str {
        if index_1 > 0 && index_1 <= 27 {
            Self::NAKSHATRA_NAMES[index_1 as usize - 1]
        } else {
            "Unknown"
        }
    }

    pub fn get_lord(index_1: u8) -> VedicPlanet {
        let idx_0 = (index_1 as usize - 1) % 9;
        match idx_0 {
            0 => VedicPlanet::Ketu,
            1 => VedicPlanet::Venus,
            2 => VedicPlanet::Sun,
            3 => VedicPlanet::Moon,
            4 => VedicPlanet::Mars,
            5 => VedicPlanet::Rahu,
            6 => VedicPlanet::Jupiter,
            7 => VedicPlanet::Saturn,
            8 => VedicPlanet::Mercury,
            _ => VedicPlanet::Sun,
        }
    }

    /// Deity (ruler) for each nakshatra (1..27)
    pub const NAKSHATRA_DEITIES: [&'static str; 27] = [
        "Ashwini Kumara",
        "Yama",
        "Agni",
        "Brahma",
        "Chandra",
        "Rudra",
        "Aditi",
        "Brihaspati",
        "Nagas",
        "Pitrs",
        "Bhaga",
        "Aryaman",
        "Savitr",
        "Tvastr",
        "Vayu",
        "Indra-Agni",
        "Mitra",
        "Indra",
        "Nritti",
        "Apas",
        "Vishvedevas",
        "Vishnu",
        "Vasus",
        "Varuna",
        "Ajaikapada",
        "Ahirbudhnya",
        "Pushan",
    ];

    /// Purushartha (Purpose) for each nakshatra: Dharma, Artha, Kama, Moksha
    pub const NAKSHATRA_PURPOSES: [&'static str; 27] = [
        "Dharma", "Artha", "Kama", "Moksha", "Moksha", "Kama", "Artha", "Dharma", "Dharma",
        "Artha", "Kama", "Moksha", "Moksha", "Kama", "Artha", "Dharma", "Dharma", "Artha", "Kama",
        "Moksha", "Moksha", "Kama", "Dharma", "Dharma", "Artha", "Artha", "Kama",
    ];

    pub fn get_deity(index_1: u8) -> &'static str {
        if index_1 > 0 && index_1 <= 27 {
            Self::NAKSHATRA_DEITIES[index_1 as usize - 1]
        } else {
            "Unknown"
        }
    }

    pub fn get_purpose(index_1: u8) -> &'static str {
        if index_1 > 0 && index_1 <= 27 {
            Self::NAKSHATRA_PURPOSES[index_1 as usize - 1]
        } else {
            "Unknown"
        }
    }

    /// Pada lord (ruler of the quarter) from navamsa sign of pada
    pub fn get_pada_lord(nakshatra: u8, pada: u8) -> VedicPlanet {
        let navamsa_sign_idx = ((nakshatra as usize - 1) * 4 + (pada as usize - 1).min(3)) % 12;
        let sign_1based = navamsa_sign_idx + 1;
        // Sign lords: Aries=Mars, Taurus=Venus, Gemini=Mercury, Cancer=Moon, Leo=Sun, Virgo=Mercury,
        // Libra=Venus, Scorpio=Mars, Sagittarius=Jupiter, Capricorn=Saturn, Aquarius=Saturn, Pisces=Jupiter
        match sign_1based {
            1 => VedicPlanet::Mars,
            2 => VedicPlanet::Venus,
            3 => VedicPlanet::Mercury,
            4 => VedicPlanet::Moon,
            5 => VedicPlanet::Sun,
            6 => VedicPlanet::Mercury,
            7 => VedicPlanet::Venus,
            8 => VedicPlanet::Mars,
            9 => VedicPlanet::Jupiter,
            10 => VedicPlanet::Saturn,
            11 => VedicPlanet::Saturn,
            12 => VedicPlanet::Jupiter,
            _ => VedicPlanet::Sun,
        }
    }

    /// Calculate Tarabala from natal Moon Nakshatra to target Nakshatra
    pub fn calculate_tarabala(natal_moon_nak: u8, target_nak: u8) -> TarabalaCategory {
        let count = if target_nak >= natal_moon_nak {
            target_nak - natal_moon_nak + 1
        } else {
            (27 - natal_moon_nak) + target_nak + 1
        };

        let cat = (count - 1) % 9;
        match cat {
            0 => TarabalaCategory::Janma,
            1 => TarabalaCategory::Sampat,
            2 => TarabalaCategory::Vipat,
            3 => TarabalaCategory::Kshema,
            4 => TarabalaCategory::Pratyak,
            5 => TarabalaCategory::Sadhana,
            6 => TarabalaCategory::Naidhana,
            7 => TarabalaCategory::Mitra,
            8 | _ => TarabalaCategory::AtiMitra, // (count % 9 == 0)
        }
    }

    /// Calculate nakshatra (1..27) and pada (1..4) from sidereal longitude (0..360)
    pub fn nakshatra_and_pada(longitude: f64) -> (u8, u8) {
        let deg = (longitude % 360.0 + 360.0) % 360.0;
        let nak_pos = deg / (360.0 / 27.0);
        let nakshatra = (nak_pos.floor() as u8).saturating_add(1).min(27);
        let pada_pos = (deg % (360.0 / 27.0)) / (360.0 / 108.0);
        let pada = (pada_pos.floor() as u8).saturating_add(1).min(4);
        (nakshatra, pada)
    }

    /// Detailed Nakshatra info for a planet
    pub fn get_pada_description(pada: u8) -> &'static str {
        match pada {
            1 => "Dharma Pada (Fire/Aries quality)",
            2 => "Artha Pada (Earth/Taurus quality)",
            3 => "Kama Pada (Air/Gemini quality)",
            4 => "Moksha Pada (Water/Cancer quality)",
            _ => "Unknown",
        }
    }
}
