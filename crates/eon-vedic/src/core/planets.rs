use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum VedicPlanet {
    Sun = 0,         // Surya
    Moon = 1,        // Chandra
    Mars = 4,        // Mangala
    Mercury = 2,     // Budha
    Jupiter = 5,     // Guru
    Venus = 3,       // Shukra
    Saturn = 6,      // Shani
    Rahu = 11,       // North Node (True)
    Ketu = 12,       // South Node (True = Rahu + 180) - Need check SE ID
    Ascendant = 100, // Lagna
}

impl VedicPlanet {
    pub fn se_id(&self) -> i32 {
        match self {
            Self::Sun => 0,        // SE_SUN
            Self::Moon => 1,       // SE_MOON
            Self::Mercury => 2,    // SE_MERCURY
            Self::Venus => 3,      // SE_VENUS
            Self::Mars => 4,       // SE_MARS
            Self::Jupiter => 5,    // SE_JUPITER
            Self::Saturn => 6,     // SE_SATURN
            Self::Rahu => 11,      // SE_TRUE_NODE
            Self::Ketu => 11,      // Handled as Node + 180 usually
            Self::Ascendant => -1, // Not a body
        }
    }

    /// Get the ruler of a given Rasi (1 = Aries, ..., 12 = Pisces)
    pub fn get_ruler_of(rasi: u8) -> Self {
        match rasi {
            1 | 8 => VedicPlanet::Mars,     // Aries, Scorpio
            2 | 7 => VedicPlanet::Venus,    // Taurus, Libra
            3 | 6 => VedicPlanet::Mercury,  // Gemini, Virgo
            4 => VedicPlanet::Moon,         // Cancer
            5 => VedicPlanet::Sun,          // Leo
            9 | 12 => VedicPlanet::Jupiter, // Sagittarius, Pisces
            10 | 11 => VedicPlanet::Saturn, // Capricorn, Aquarius
            _ => VedicPlanet::Sun,          // Fallback (should not happen for 1-12)
        }
    }

    pub fn exaltation_rasi(&self) -> u8 {
        match self {
            Self::Sun => 1,     // Aries
            Self::Moon => 2,    // Taurus
            Self::Mars => 10,   // Capricorn
            Self::Mercury => 6, // Virgo
            Self::Jupiter => 4, // Cancer
            Self::Venus => 12,  // Pisces
            Self::Saturn => 7,  // Libra
            Self::Rahu => 2,    // Taurus (common view) or 3
            Self::Ketu => 8,    // Scorpio (common view) or 9
            _ => 0,
        }
    }

    pub fn debilitation_rasi(&self) -> u8 {
        match self.exaltation_rasi() {
            0 => 0,
            n => (n + 5) % 12 + 1, // Opposite sign
        }
    }

    /// Natural Friendship (Naisargika Maitri)
    /// Returns: 1 (Friend), 0 (Neutral), -1 (Enemy)
    pub fn naisargika_relation(&self, other: Self) -> i8 {
        if *self == other {
            return 1;
        }
        match self {
            Self::Sun => match other {
                Self::Moon | Self::Mars | Self::Jupiter => 1,
                Self::Mercury => 0,
                Self::Venus | Self::Saturn => -1,
                _ => 0,
            },
            Self::Moon => match other {
                Self::Sun | Self::Mercury => 1,
                Self::Mars | Self::Jupiter | Self::Venus | Self::Saturn => 0,
                _ => 0,
            },
            Self::Mars => match other {
                Self::Sun | Self::Moon | Self::Jupiter => 1,
                Self::Venus | Self::Saturn => 0,
                Self::Mercury => -1,
                _ => 0,
            },
            Self::Mercury => match other {
                Self::Sun | Self::Venus => 1,
                Self::Mars | Self::Jupiter | Self::Saturn => 0,
                Self::Moon => -1,
                _ => 0,
            },
            Self::Jupiter => match other {
                Self::Sun | Self::Moon | Self::Mars => 1,
                Self::Saturn => 0,
                Self::Mercury | Self::Venus => -1,
                _ => 0,
            },
            Self::Venus => match other {
                Self::Mercury | Self::Saturn => 1,
                Self::Mars | Self::Jupiter => 0,
                Self::Sun | Self::Moon => -1,
                _ => 0,
            },
            Self::Saturn => match other {
                Self::Mercury | Self::Venus => 1,
                Self::Jupiter => 0,
                Self::Sun | Self::Moon | Self::Mars => -1,
                _ => 0,
            },
            _ => 0,
        }
    }

    /// Temporal Friendship (Tatkalika Maitri)
    /// Rule: Planets in 2, 3, 4, 10, 11, 12 houses from each other are friends.
    pub fn tatkalika_relation(b_house: u8, g_house: u8) -> i8 {
        let diff = (g_house as i16 - b_house as i16 + 12) % 12;
        match diff {
            1 | 2 | 3 | 9 | 10 | 11 => 1, // 2nd, 3rd, 4th, 10th, 11th, 12th houses
            _ => -1,
        }
    }

    /// Combined Friendship (Panchadha Maitri)
    /// Natural + Temporal
    /// 2: Great Friend (Adhi Mitra), 1: Friend (Mitra), 0: Neutral (Sama), -1: Enemy (Shatru), -2: Great Enemy (Adhi Shatru)
    pub fn panchadha_relation(&self, other: Self, b_house: u8, g_house: u8) -> i8 {
        self.naisargika_relation(other) + Self::tatkalika_relation(b_house, g_house)
    }
}
