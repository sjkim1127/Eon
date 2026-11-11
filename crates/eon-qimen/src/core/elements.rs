use serde::{Deserialize, Serialize};

/// Nine Palaces (Jiu Gong)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum Palace {
    Kan1,    // Palace 1 (Kan, North)
    Kun2,    // Palace 2 (Kun, Southwest)
    Zhen3,   // Palace 3 (Zhen, East)
    Xun4,    // Palace 4 (Xun, Southeast)
    Center5, // Palace 5 (Center)
    Qian6,   // Palace 6 (Qian, Northwest)
    Dui7,    // Palace 7 (Dui, West)
    Gen8,    // Palace 8 (Gen, Northeast)
    Li9,     // Palace 9 (Li, South)
}

impl Palace {
    pub fn as_u8(&self) -> u8 {
        match self {
            Palace::Kan1 => 1,
            Palace::Kun2 => 2,
            Palace::Zhen3 => 3,
            Palace::Xun4 => 4,
            Palace::Center5 => 5,
            Palace::Qian6 => 6,
            Palace::Dui7 => 7,
            Palace::Gen8 => 8,
            Palace::Li9 => 9,
        }
    }

    pub fn from_u8(num: u8) -> Option<Self> {
        match num {
            1 => Some(Palace::Kan1),
            2 => Some(Palace::Kun2),
            3 => Some(Palace::Zhen3),
            4 => Some(Palace::Xun4),
            5 => Some(Palace::Center5),
            6 => Some(Palace::Qian6),
            7 => Some(Palace::Dui7),
            8 => Some(Palace::Gen8),
            9 => Some(Palace::Li9),
            _ => None,
        }
    }
}

/// Eight Doors (Ba Men - Human Plate)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Door {
    Xiu,   // Xiu Men (Rest)
    Sheng, // Sheng Men (Life)
    Shang, // Shang Men (Harm)
    Du,    // Du Men (Delusion / Concealment)
    Jing,  // Jing Men (Scenery)
    Si,    // Si Men (Death)
    Jing2, // Jing Men (Fear / Shock)
    Kai,   // Kai Men (Open)
}

/// Nine Stars (Jiu Xing - Star Plate)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Star {
    Peng,  // Tian Peng
    Ren,   // Tian Ren
    Chong, // Tian Chong
    Fu,    // Tian Fu
    Ying,  // Tian Ying
    Rui,   // Tian Rui
    Zhu,   // Tian Zhu
    Xin,   // Tian Xin
    Qin,   // Tian Qin (Center Palace 5)
}

/// Eight Deities (Ba Shen - Deity Plate)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Deity {
    ZhiFu,   // Zhi Fu (Chief)
    TengShe, // Teng She (Surging Snake)
    TaiYin,  // Tai Yin (Great Moon)
    LiuHe,   // Liu He (Six Harmonies)
    BaiHu,   // Bai Hu (White Tiger) / Gou Chen in Yin Dun
    XuanWu,  // Xuan Wu (Black Tortoise) / Zhu Que in Yin Dun
    JiuDi,   // Jiu Di (Nine Earth)
    JiuTian, // Jiu Tian (Nine Heaven)
}
