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

impl Palace {
    /// Trigram (卦) name of the palace; the centre has none.
    pub fn trigram(&self) -> &'static str {
        match self {
            Palace::Kan1 => "坎",
            Palace::Kun2 => "坤",
            Palace::Zhen3 => "震",
            Palace::Xun4 => "巽",
            Palace::Center5 => "中",
            Palace::Qian6 => "乾",
            Palace::Dui7 => "兌",
            Palace::Gen8 => "艮",
            Palace::Li9 => "離",
        }
    }

    pub fn korean(&self) -> &'static str {
        match self {
            Palace::Kan1 => "감궁(북)",
            Palace::Kun2 => "곤궁(남서)",
            Palace::Zhen3 => "진궁(동)",
            Palace::Xun4 => "손궁(남동)",
            Palace::Center5 => "중궁",
            Palace::Qian6 => "건궁(북서)",
            Palace::Dui7 => "태궁(서)",
            Palace::Gen8 => "간궁(북동)",
            Palace::Li9 => "리궁(남)",
        }
    }
}

impl Door {
    pub fn hanja(&self) -> &'static str {
        match self {
            Door::Xiu => "休門",
            Door::Sheng => "生門",
            Door::Shang => "傷門",
            Door::Du => "杜門",
            Door::Jing => "景門",
            Door::Si => "死門",
            Door::Jing2 => "驚門",
            Door::Kai => "開門",
        }
    }

    pub fn english(&self) -> &'static str {
        match self {
            Door::Xiu => "Rest",
            Door::Sheng => "Life",
            Door::Shang => "Harm",
            Door::Du => "Delusion",
            Door::Jing => "Scenery",
            Door::Si => "Death",
            Door::Jing2 => "Fear",
            Door::Kai => "Open",
        }
    }

    pub fn korean(&self) -> &'static str {
        match self {
            Door::Xiu => "휴문",
            Door::Sheng => "생문",
            Door::Shang => "상문",
            Door::Du => "두문",
            Door::Jing => "경문(景)",
            Door::Si => "사문",
            Door::Jing2 => "경문(驚)",
            Door::Kai => "개문",
        }
    }
}

impl Star {
    pub fn hanja(&self) -> &'static str {
        match self {
            Star::Peng => "天蓬",
            Star::Ren => "天任",
            Star::Chong => "天沖",
            Star::Fu => "天輔",
            Star::Ying => "天英",
            Star::Rui => "天芮",
            Star::Zhu => "天柱",
            Star::Xin => "天心",
            Star::Qin => "天禽",
        }
    }

    pub fn english(&self) -> &'static str {
        match self {
            Star::Peng => "Tian Peng",
            Star::Ren => "Tian Ren",
            Star::Chong => "Tian Chong",
            Star::Fu => "Tian Fu",
            Star::Ying => "Tian Ying",
            Star::Rui => "Tian Rui",
            Star::Zhu => "Tian Zhu",
            Star::Xin => "Tian Xin",
            Star::Qin => "Tian Qin",
        }
    }
}

impl Deity {
    pub fn hanja(&self) -> &'static str {
        match self {
            Deity::ZhiFu => "值符",
            Deity::TengShe => "螣蛇",
            Deity::TaiYin => "太陰",
            Deity::LiuHe => "六合",
            Deity::BaiHu => "白虎",
            Deity::XuanWu => "玄武",
            Deity::JiuDi => "九地",
            Deity::JiuTian => "九天",
        }
    }

    pub fn english(&self) -> &'static str {
        match self {
            Deity::ZhiFu => "Zhi Fu",
            Deity::TengShe => "Teng She",
            Deity::TaiYin => "Tai Yin",
            Deity::LiuHe => "Liu He",
            Deity::BaiHu => "Bai Hu",
            Deity::XuanWu => "Xuan Wu",
            Deity::JiuDi => "Jiu Di",
            Deity::JiuTian => "Jiu Tian",
        }
    }
}
