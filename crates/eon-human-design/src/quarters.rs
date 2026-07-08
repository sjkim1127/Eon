use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ZodiacQuarter {
    Initiation,
    Civilization,
    Duality,
    Mutation,
}

impl ZodiacQuarter {
    /// Determines the Quarter based on the Personality Sun Gate.
    pub fn from_gate(gate: u8) -> Self {
        // Quarter of Initiation: Gates 13 to 24 (13, 49, 30, 55, 37, 63, 22, 36, 25, 17, 21, 51, 42, 3, 27, 24)
        const INITIATION: [u8; 16] = [
            13, 49, 30, 55, 37, 63, 22, 36, 25, 17, 21, 51, 42, 3, 27, 24,
        ];

        // Quarter of Civilization: Gates 2 to 33 (2, 23, 8, 20, 16, 35, 45, 12, 15, 52, 39, 53, 62, 56, 31, 33)
        const CIVILIZATION: [u8; 16] =
            [2, 23, 8, 20, 16, 35, 45, 12, 15, 52, 39, 53, 62, 56, 31, 33];

        // Quarter of Duality: Gates 7 to 44 (7, 4, 29, 59, 40, 64, 47, 6, 46, 18, 48, 57, 32, 50, 28, 44)
        const DUALITY: [u8; 16] = [7, 4, 29, 59, 40, 64, 47, 6, 46, 18, 48, 57, 32, 50, 28, 44];

        // Quarter of Mutation: Gates 1 to 19 (1, 43, 14, 34, 9, 5, 26, 11, 10, 58, 38, 54, 61, 60, 41, 19)
        const MUTATION: [u8; 16] = [1, 43, 14, 34, 9, 5, 26, 11, 10, 58, 38, 54, 61, 60, 41, 19];

        if INITIATION.contains(&gate) {
            Self::Initiation
        } else if CIVILIZATION.contains(&gate) {
            Self::Civilization
        } else if DUALITY.contains(&gate) {
            Self::Duality
        } else if MUTATION.contains(&gate) {
            Self::Mutation
        } else {
            // Fallback (should never happen if 1..64 is passed)
            Self::Initiation
        }
    }

    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Initiation => "Quarter of Initiation",
            Self::Civilization => "Quarter of Civilization",
            Self::Duality => "Quarter of Duality",
            Self::Mutation => "Quarter of Mutation",
        }
    }

    pub fn name_ko(&self) -> &'static str {
        match self {
            Self::Initiation => "시작의 분기 (Initiation)",
            Self::Civilization => "문명의 분기 (Civilization)",
            Self::Duality => "이원성의 분기 (Duality)",
            Self::Mutation => "돌연변이의 분기 (Mutation)",
        }
    }

    pub fn theme_en(&self) -> &'static str {
        match self {
            Self::Initiation => "Purpose fulfilled through Mind",
            Self::Civilization => "Purpose fulfilled through Form",
            Self::Duality => "Purpose fulfilled through Bonding",
            Self::Mutation => "Purpose fulfilled through Transformation",
        }
    }

    pub fn theme_ko(&self) -> &'static str {
        match self {
            Self::Initiation => "정신(Mind)을 통한 목적 달성",
            Self::Civilization => "형태(Form)를 통한 목적 달성",
            Self::Duality => "결속(Bonding)을 통한 목적 달성",
            Self::Mutation => "변형(Transformation)을 통한 목적 달성",
        }
    }
}
