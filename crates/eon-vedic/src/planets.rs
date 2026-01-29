use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VedicPlanet {
    Sun = 0,        // Surya
    Moon = 1,       // Chandra
    Mars = 4,       // Mangala
    Mercury = 2,    // Budha
    Jupiter = 5,    // Guru
    Venus = 3,      // Shukra
    Saturn = 6,     // Shani
    Rahu = 11,      // North Node (True)
    Ketu = 12,      // South Node (True = Rahu + 180) - Need check SE ID
    Ascendant = 100, // Lagna
}

impl VedicPlanet {
    pub fn se_id(&self) -> i32 {
        match self {
            Self::Sun => 0, // SE_SUN
            Self::Moon => 1, // SE_MOON
            Self::Mercury => 2, // SE_MERCURY
            Self::Venus => 3, // SE_VENUS
            Self::Mars => 4, // SE_MARS
            Self::Jupiter => 5, // SE_JUPITER
            Self::Saturn => 6, // SE_SATURN
            Self::Rahu => 11, // SE_TRUE_NODE
            Self::Ketu => 11, // Handled as Node + 180 usually
            Self::Ascendant => -1, // Not a body
        }
    }
}
