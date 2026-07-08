use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ArrowDirection {
    Left,  // Tone 1, 2, 3
    Right, // Tone 4, 5, 6
}

impl ArrowDirection {
    pub fn from_tone(tone: u8) -> Self {
        if tone <= 3 {
            ArrowDirection::Left
        } else {
            ArrowDirection::Right
        }
    }
}

// 1. Digestion (Design Sun/Earth)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DigestionColor {
    Appetite, // 1
    Taste,    // 2
    Thirst,   // 3
    Touch,    // 4
    Sound,    // 5
    Light,    // 6
}

impl DigestionColor {
    pub fn from_color(color: u8) -> Self {
        match color {
            1 => Self::Appetite,
            2 => Self::Taste,
            3 => Self::Thirst,
            4 => Self::Touch,
            5 => Self::Sound,
            6 => Self::Light,
            _ => Self::Appetite, // fallback
        }
    }
}

// 2. Environment (Design Nodes)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EnvironmentColor {
    Caves,    // 1
    Markets,  // 2
    Kitchens, // 3
    Mountains,// 4
    Valleys,  // 5
    Shores,   // 6
}

impl EnvironmentColor {
    pub fn from_color(color: u8) -> Self {
        match color {
            1 => Self::Caves,
            2 => Self::Markets,
            3 => Self::Kitchens,
            4 => Self::Mountains,
            5 => Self::Valleys,
            6 => Self::Shores,
            _ => Self::Caves,
        }
    }
}

// 3. Motivation (Personality Sun/Earth)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MotivationColor {
    Fear,      // 1
    Hope,      // 2
    Desire,    // 3
    Need,      // 4
    Guilt,     // 5
    Innocence, // 6
}

impl MotivationColor {
    pub fn from_color(color: u8) -> Self {
        match color {
            1 => Self::Fear,
            2 => Self::Hope,
            3 => Self::Desire,
            4 => Self::Need,
            5 => Self::Guilt,
            6 => Self::Innocence,
            _ => Self::Fear,
        }
    }

    pub fn transference(&self) -> Self {
        match self {
            Self::Fear => Self::Need,
            Self::Hope => Self::Guilt,
            Self::Desire => Self::Innocence,
            Self::Need => Self::Fear,
            Self::Guilt => Self::Hope,
            Self::Innocence => Self::Desire,
        }
    }
}

// 4. Perspective/View (Personality Nodes)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PerspectiveColor {
    Survival,    // 1
    Possibility, // 2
    Power,       // 3
    Wanting,     // 4
    Probability, // 5
    Personal,    // 6
}

impl PerspectiveColor {
    pub fn from_color(color: u8) -> Self {
        match color {
            1 => Self::Survival,
            2 => Self::Possibility,
            3 => Self::Power,
            4 => Self::Wanting,
            5 => Self::Probability,
            6 => Self::Personal,
            _ => Self::Survival,
        }
    }

    pub fn transference(&self) -> Self {
        match self {
            Self::Survival => Self::Wanting,
            Self::Possibility => Self::Probability,
            Self::Power => Self::Personal,
            Self::Wanting => Self::Survival,
            Self::Probability => Self::Possibility,
            Self::Personal => Self::Power,
        }
    }
}

// Tones (Cognition)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ToneCognition {
    Smell,       // 1
    Taste,       // 2
    OuterVision, // 3
    InnerVision, // 4
    Feeling,     // 5
    Touch,       // 6
}

impl ToneCognition {
    pub fn from_tone(tone: u8) -> Self {
        match tone {
            1 => Self::Smell,
            2 => Self::Taste,
            3 => Self::OuterVision,
            4 => Self::InnerVision,
            5 => Self::Feeling,
            6 => Self::Touch,
            _ => Self::Smell,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BaseOrientation {
    Reactive,   // 1
    Active,     // 2
    Objective,  // 3
    Subjective, // 4
    Personal,   // 5
}

impl BaseOrientation {
    pub fn from_base(base: u8) -> Self {
        match base {
            1 => Self::Reactive,
            2 => Self::Active,
            3 => Self::Objective,
            4 => Self::Subjective,
            5 => Self::Personal,
            _ => Self::Reactive,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DigestionVariable {
    pub direction: ArrowDirection,
    pub color: DigestionColor,
    pub tone: ToneCognition,
    pub base: BaseOrientation,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnvironmentVariable {
    pub direction: ArrowDirection,
    pub color: EnvironmentColor,
    pub tone: ToneCognition,
    pub base: BaseOrientation,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MotivationVariable {
    pub direction: ArrowDirection,
    pub color: MotivationColor,
    pub transference: MotivationColor,
    pub tone: ToneCognition,
    pub base: BaseOrientation,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PerspectiveVariable {
    pub direction: ArrowDirection,
    pub color: PerspectiveColor,
    pub transference: PerspectiveColor,
    pub tone: ToneCognition,
    pub base: BaseOrientation,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PhsVariablesResult {
    pub digestion: DigestionVariable,
    pub environment: EnvironmentVariable,
    pub motivation: MotivationVariable,
    pub perspective: PerspectiveVariable,
}

pub fn calculate_phs(
    design_sun: &crate::HdPlanetData,
    design_node: &crate::HdPlanetData,
    pers_sun: &crate::HdPlanetData,
    pers_node: &crate::HdPlanetData,
) -> PhsVariablesResult {
    PhsVariablesResult {
        digestion: DigestionVariable {
            direction: ArrowDirection::from_tone(design_sun.tone),
            color: DigestionColor::from_color(design_sun.color),
            tone: ToneCognition::from_tone(design_sun.tone),
            base: BaseOrientation::from_base(design_sun.base),
        },
        environment: EnvironmentVariable {
            direction: ArrowDirection::from_tone(design_node.tone),
            color: EnvironmentColor::from_color(design_node.color),
            tone: ToneCognition::from_tone(design_node.tone),
            base: BaseOrientation::from_base(design_node.base),
        },
        motivation: MotivationVariable {
            direction: ArrowDirection::from_tone(pers_sun.tone),
            color: MotivationColor::from_color(pers_sun.color),
            transference: MotivationColor::from_color(pers_sun.color).transference(),
            tone: ToneCognition::from_tone(pers_sun.tone),
            base: BaseOrientation::from_base(pers_sun.base),
        },
        perspective: PerspectiveVariable {
            direction: ArrowDirection::from_tone(pers_node.tone),
            color: PerspectiveColor::from_color(pers_node.color),
            transference: PerspectiveColor::from_color(pers_node.color).transference(),
            tone: ToneCognition::from_tone(pers_node.tone),
            base: BaseOrientation::from_base(pers_node.base),
        },
    }
}

