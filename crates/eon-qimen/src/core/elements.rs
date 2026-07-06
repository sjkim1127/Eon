use serde::{Deserialize, Serialize};

/// 구궁 (Nine Palaces)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum Palace {
    Kan1,    // 1궁 (감궁, 北)
    Kun2,    // 2궁 (곤궁, 西南)
    Zhen3,   // 3궁 (진궁, 東)
    Xun4,    // 4궁 (손궁, 東南)
    Center5, // 5궁 (중궁, 中央)
    Qian6,   // 6궁 (건궁, 西北)
    Dui7,    // 7궁 (태궁, 西)
    Gen8,    // 8궁 (간궁, 東北)
    Li9,     // 9궁 (이궁, 南)
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

/// 8문 (Eight Doors - Human Plate)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Door {
    Xiu,   // 휴문 (Xiu Men - Rest)
    Sheng, // 생문 (Sheng Men - Life)
    Shang, // 상문 (Shang Men - Harm)
    Du,    // 두문 (Du Men - Delusion)
    Jing,  // 경문 (Jing(景) Men - Scenery)
    Si,    // 사문 (Si Men - Death)
    Jing2, // 경문 (Jing(驚) Men - Fear/Shock)
    Kai,   // 개문 (Kai Men - Open)
}

/// 9성 (Nine Stars - Star Plate)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Star {
    Peng,  // 천봉성 (Tian Peng)
    Ren,   // 천임성 (Tian Ren)
    Chong, // 천충성 (Tian Chong)
    Fu,    // 천보성 (Tian Fu)
    Ying,  // 천영성 (Tian Ying)
    Rui,   // 천예성 (Tian Rui)
    Zhu,   // 천주성 (Tian Zhu)
    Xin,   // 천심성 (Tian Xin)
    Qin,   // 천금성 (Tian Qin) - 보통 5궁에 위치
}

/// 8신 (Eight Deities - Deity Plate)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Deity {
    ZhiFu,   // 직부 (Zhi Fu - Chief)
    TengShe, // 구사 (Teng She - Snake)
    TaiYin,  // 태음 (Tai Yin - Moon)
    LiuHe,   // 육합 (Liu He - Harmony)
    BaiHu,   // 백호 (Bai Hu - White Tiger) / 음둔일 때는 구진(Gou Chen)
    XuanWu,  // 현무 (Xuan Wu - Black Tortoise) / 음둔일 때는 주작(Zhu Que)
    JiuDi,   // 구지 (Jiu Di - Nine Earth)
    JiuTian, // 구천 (Jiu Tian - Nine Heaven)
}
