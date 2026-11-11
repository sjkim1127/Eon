//! Zi Wei Dou Shu (ZWDS) domain type definitions.
//!
//! Coordinate system: 12 Palaces start with Yin (寅) = 0, increasing counter-clockwise.
//! [0=寅, 1=卯, 2=辰, 3=巳, 4=午, 5=未, 6=申, 7=酉, 8=戌, 9=亥, 10=子, 11=丑]

use serde::{Deserialize, Serialize};

// ============================================================
// 12 Palaces index
// ============================================================

/// ZWDS coordinate system: Yin=0, counter-clockwise
/// `fix_index(i)` = `((i % 12) + 12) % 12`
pub type PalaceIndex = usize;

/// 12 Palace names
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PalaceName {
    /// Life Palace (Ming) - Self, personality, foundational destiny
    Ming,
    /// Siblings Palace (Xiongdi)
    Xiongdi,
    /// Spouse Palace (Fuqi)
    Fuqi,
    /// Children Palace (Zinv)
    Zinv,
    /// Wealth Palace (Caibo)
    Caibo,
    /// Health Palace (Jie)
    Jie,
    /// Travel Palace (Qianyi)
    Qianyi,
    /// Friends / Servants Palace (Nupao)
    Nupao,
    /// Career Palace (Guanlu)
    Guanlu,
    /// Property Palace (Tianzhai)
    Tianzhai,
    /// Karma / Happiness Palace (Fude)
    Fude,
    /// Parents Palace (Fumu)
    Fumu,
}

impl PalaceName {
    pub const ALL: [PalaceName; 12] = [
        Self::Ming,
        Self::Xiongdi,
        Self::Fuqi,
        Self::Zinv,
        Self::Caibo,
        Self::Jie,
        Self::Qianyi,
        Self::Nupao,
        Self::Guanlu,
        Self::Tianzhai,
        Self::Fude,
        Self::Fumu,
    ];

    pub fn hanja(&self) -> &'static str {
        match self {
            Self::Ming => "命宮",
            Self::Xiongdi => "兄弟宮",
            Self::Fuqi => "夫妻宮",
            Self::Zinv => "子女宮",
            Self::Caibo => "財帛宮",
            Self::Jie => "疾厄宮",
            Self::Qianyi => "遷移宮",
            Self::Nupao => "奴僕宮",
            Self::Guanlu => "官祿宮",
            Self::Tianzhai => "田宅宮",
            Self::Fude => "福德宮",
            Self::Fumu => "父母宮",
        }
    }

    pub fn korean(&self) -> &'static str {
        self.english()
    }

    pub fn english(&self) -> &'static str {
        match self {
            Self::Ming => "Life Palace",
            Self::Xiongdi => "Siblings Palace",
            Self::Fuqi => "Spouse Palace",
            Self::Zinv => "Children Palace",
            Self::Caibo => "Wealth Palace",
            Self::Jie => "Health Palace",
            Self::Qianyi => "Travel Palace",
            Self::Nupao => "Friends Palace",
            Self::Guanlu => "Career Palace",
            Self::Tianzhai => "Property Palace",
            Self::Fude => "Karma Palace",
            Self::Fumu => "Parents Palace",
        }
    }
}

// ============================================================
// Stars enumeration
// ============================================================

/// Complete ZWDS star system
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ZwdsStar {
    // ── 14 主星 ──────────────────────────────────────────
    /// 紫微
    ZiWei,
    /// 天機
    TianJi,
    /// 太陽
    TaiYang,
    /// 武曲
    WuQu,
    /// 天同
    TianTong,
    /// 廉貞
    LianZhen,
    /// 天府
    TianFu,
    /// 太陰
    TaiYin,
    /// 貪狼
    TanLang,
    /// 巨門
    JuMen,
    /// 天相
    TianXiang,
    /// 天梁
    TianLiang,
    /// 七殺
    QiSha,
    /// 破軍
    PoJun,

    // ── 6 輔星 A ─────────────────────────────────────────
    /// 文昌
    WenChang,
    /// 文曲
    WenQu,
    /// 左輔
    ZuoFu,
    /// 右弼
    YouBi,
    /// 天魁
    TianKui,
    /// 天鉞
    TianYue,

    // ── 祿馬三方 ──────────────────────────────────────────
    /// 禄存
    LuCun,
    /// 擎羊
    QingYang,
    /// 陀羅
    TuoLuo,
    /// 天馬
    TianMa,

    // ── 火鈴 ─────────────────────────────────────────────
    /// 火星
    HuoXing,
    /// 鈴星
    LingXing,

    // ── 空劫 ─────────────────────────────────────────────
    /// 地劫
    DiJie,
    /// 地空
    DiKong,

    // ── Minor and Auxiliary Stars (~24 stars) ───────────────────
    /// 紅鸞
    HongLuan,
    /// 天喜
    TianXi,
    /// 天刑
    TianXing,
    /// 天姚
    TianYao,
    /// 解神
    JieShen,
    /// 天巫
    TianWu,
    /// 天月 (TianYueStar, distinct from TianYue)
    TianYueStar,
    /// 陰煞
    YinSha,
    /// 台輔
    TaiFu,
    /// 封誥
    FengGao,
    /// 三台
    SanTai,
    /// 八座
    BaZuo,
    /// 恩光
    EnGuang,
    /// 天貴
    TianGui,
    /// 天才
    TianCai,
    /// 天壽
    TianShou,
    /// 龍池
    LongChi,
    /// 鳳閣
    FengGe,
    /// 天哭
    TianKu,
    /// 天虛
    TianXu,
    /// 華蓋
    HuaGai,
    /// 咸池
    XianChi,
    /// 孤辰
    GuChen,
    /// 寡宿
    GuaSu,
    /// 天空
    TianKong,
    /// 劫殺
    JieSha,
    /// 天廚
    TianChu,
    /// 天官
    TianGuan,
    /// 天福
    TianFu2, // Avoid name collision with TianFu
}

impl ZwdsStar {
    /// Hanja / Chinese character representation
    pub fn hanja(&self) -> &'static str {
        match self {
            Self::ZiWei => "紫微",
            Self::TianJi => "天機",
            Self::TaiYang => "太陽",
            Self::WuQu => "武曲",
            Self::TianTong => "天同",
            Self::LianZhen => "廉貞",
            Self::TianFu => "天府",
            Self::TaiYin => "太陰",
            Self::TanLang => "貪狼",
            Self::JuMen => "巨門",
            Self::TianXiang => "天相",
            Self::TianLiang => "天梁",
            Self::QiSha => "七殺",
            Self::PoJun => "破軍",
            Self::WenChang => "文昌",
            Self::WenQu => "文曲",
            Self::ZuoFu => "左輔",
            Self::YouBi => "右弼",
            Self::TianKui => "天魁",
            Self::TianYue => "天鉞",
            Self::LuCun => "禄存",
            Self::QingYang => "擎羊",
            Self::TuoLuo => "陀羅",
            Self::TianMa => "天馬",
            Self::HuoXing => "火星",
            Self::LingXing => "鈴星",
            Self::DiJie => "地劫",
            Self::DiKong => "地空",
            Self::HongLuan => "紅鸞",
            Self::TianXi => "天喜",
            Self::TianXing => "天刑",
            Self::TianYao => "天姚",
            Self::JieShen => "解神",
            Self::TianWu => "天巫",
            Self::TianYueStar => "天月",
            Self::YinSha => "陰煞",
            Self::TaiFu => "台輔",
            Self::FengGao => "封誥",
            Self::SanTai => "三台",
            Self::BaZuo => "八座",
            Self::EnGuang => "恩光",
            Self::TianGui => "天貴",
            Self::TianCai => "天才",
            Self::TianShou => "天壽",
            Self::LongChi => "龍池",
            Self::FengGe => "鳳閣",
            Self::TianKu => "天哭",
            Self::TianXu => "天虛",
            Self::HuaGai => "華蓋",
            Self::XianChi => "咸池",
            Self::GuChen => "孤辰",
            Self::GuaSu => "寡宿",
            Self::TianKong => "天空",
            Self::JieSha => "劫殺",
            Self::TianChu => "天廚",
            Self::TianGuan => "天官",
            Self::TianFu2 => "天福",
        }
    }

    /// English / Pinyin representation
    pub fn korean(&self) -> &'static str {
        match self {
            Self::ZiWei => "ZiWei",
            Self::TianJi => "TianJi",
            Self::TaiYang => "TaiYang",
            Self::WuQu => "WuQu",
            Self::TianTong => "TianTong",
            Self::LianZhen => "LianZhen",
            Self::TianFu => "TianFu",
            Self::TaiYin => "TaiYin",
            Self::TanLang => "TanLang",
            Self::JuMen => "JuMen",
            Self::TianXiang => "TianXiang",
            Self::TianLiang => "TianLiang",
            Self::QiSha => "QiSha",
            Self::PoJun => "PoJun",
            Self::WenChang => "WenChang",
            Self::WenQu => "WenQu",
            Self::ZuoFu => "ZuoFu",
            Self::YouBi => "YouBi",
            Self::TianKui => "TianKui",
            Self::TianYue => "TianYue",
            Self::LuCun => "LuCun",
            Self::QingYang => "QingYang",
            Self::TuoLuo => "TuoLuo",
            Self::TianMa => "TianMa",
            Self::HuoXing => "HuoXing",
            Self::LingXing => "LingXing",
            Self::DiJie => "DiJie",
            Self::DiKong => "DiKong",
            Self::HongLuan => "HongLuan",
            Self::TianXi => "TianXi",
            Self::TianXing => "TianXing",
            Self::TianYao => "TianYao",
            Self::JieShen => "JieShen",
            Self::TianWu => "TianWu",
            Self::TianYueStar => "TianYueStar",
            Self::YinSha => "YinSha",
            Self::TaiFu => "TaiFu",
            Self::FengGao => "FengGao",
            Self::SanTai => "SanTai",
            Self::BaZuo => "BaZuo",
            Self::EnGuang => "EnGuang",
            Self::TianGui => "TianGui",
            Self::TianCai => "TianCai",
            Self::TianShou => "TianShou",
            Self::LongChi => "LongChi",
            Self::FengGe => "FengGe",
            Self::TianKu => "TianKu",
            Self::TianXu => "TianXu",
            Self::HuaGai => "HuaGai",
            Self::XianChi => "XianChi",
            Self::GuChen => "GuChen",
            Self::GuaSu => "GuaSu",
            Self::TianKong => "TianKong",
            Self::JieSha => "JieSha",
            Self::TianChu => "TianChu",
            Self::TianGuan => "TianGuan",
            Self::TianFu2 => "TianFu2",
        }
    }

    /// Whether this is a major star
    pub fn is_main_star(&self) -> bool {
        matches!(
            self,
            Self::ZiWei
                | Self::TianJi
                | Self::TaiYang
                | Self::WuQu
                | Self::TianTong
                | Self::LianZhen
                | Self::TianFu
                | Self::TaiYin
                | Self::TanLang
                | Self::JuMen
                | Self::TianXiang
                | Self::TianLiang
                | Self::QiSha
                | Self::PoJun
        )
    }

    pub fn is_major(&self) -> bool {
        self.is_main_star()
    }
}

// ============================================================
// Five Elements Bureau (Wu Xing Ju)
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FiveElementsClass {
    /// Water 2nd Bureau (starts at age 2)
    Water2,
    /// Wood 3rd Bureau (starts at age 3)
    Wood3,
    /// Metal 4th Bureau (starts at age 4)
    Metal4,
    /// Earth 5th Bureau (starts at age 5)
    Earth5,
    /// Fire 6th Bureau (starts at age 6)
    Fire6,
}

impl FiveElementsClass {
    /// Decadal starting age
    pub fn starting_age(&self) -> u8 {
        match self {
            Self::Water2 => 2,
            Self::Wood3 => 3,
            Self::Metal4 => 4,
            Self::Earth5 => 5,
            Self::Fire6 => 6,
        }
    }

    /// Bureau numeric value
    pub fn value(&self) -> u32 {
        match self {
            Self::Water2 => 2,
            Self::Wood3 => 3,
            Self::Metal4 => 4,
            Self::Earth5 => 5,
            Self::Fire6 => 6,
        }
    }

    pub fn korean(&self) -> &'static str {
        match self {
            Self::Water2 => "Water 2nd Bureau (Water2)",
            Self::Wood3 => "Wood 3rd Bureau (Wood3)",
            Self::Metal4 => "Metal 4th Bureau (Metal4)",
            Self::Earth5 => "Earth 5th Bureau (Earth5)",
            Self::Fire6 => "Fire 6th Bureau (Fire6)",
        }
    }
}

// ============================================================
// Four Transformations (Si Hua)
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SiHuaType {
    /// Hua Lu - Prosperity, fortune, wealth
    HuaLu,
    /// Hua Quan - Authority, power, leadership
    HuaQuan,
    /// Hua Ke - Fame, academic renown, reputation
    HuaKe,
    /// Hua Ji - Obstacle, challenge, catalyst
    HuaJi,
}

impl SiHuaType {
    pub fn korean(&self) -> &'static str {
        match self {
            Self::HuaLu => "Hua Lu (Prosperity)",
            Self::HuaQuan => "Hua Quan (Authority)",
            Self::HuaKe => "Hua Ke (Fame / Acclaim)",
            Self::HuaJi => "Hua Ji (Obstacle / Catalyst)",
        }
    }
    pub fn emoji(&self) -> &'static str {
        match self {
            Self::HuaLu => "祿",
            Self::HuaQuan => "權",
            Self::HuaKe => "科",
            Self::HuaJi => "忌",
        }
    }
}

// ============================================================
// Palace data
// ============================================================

/// Star brightness (Miao, Wang, Li, Xian)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ZwdsBrightness {
    /// Miao (Temple) - Highest brightness, auspicious
    Miao,
    /// Wang (Bright) - Very bright, auspicious
    Wang,
    /// De (Gain) - Gained ground, positive
    De,
    /// Li (Benefit) - Beneficial, above average
    Li,
    /// Ping (Peace / Neutral) - Average
    Ping,
    /// Bu (Dim) - Below average
    Bu,
    /// Xian (Fallen) - Lowest brightness, challenged
    Xian,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StarInPalace {
    pub star: ZwdsStar,
    /// Si Hua transformation attached to this star
    pub si_hua: Option<SiHuaType>,
    /// Star brightness status
    pub brightness: Option<ZwdsBrightness>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PalaceData {
    /// Palace index (0=Yin)
    pub index: PalaceIndex,
    /// Palace name
    pub name: PalaceName,
    /// Palace Heavenly Stem
    pub heavenly_stem: String,
    /// Palace Earthly Branch
    pub earthly_branch: String,
    /// Stars situated in this palace
    pub stars: Vec<StarInPalace>,
    /// Decadal luck age range
    pub daxian_range: Option<(u8, u8)>,
    /// Whether active in current annual flow
    pub is_current_liu_nian: bool,
}

// ============================================================
// Lunar calendar data
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LunarBirthInfo {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub is_leap_month: bool,
    /// Year stem index (0=Jia..9=Gui)
    pub year_stem_idx: usize,
    /// Year branch index (0=Zi..11=Hai)
    pub year_branch_idx: usize,
    /// Hour branch index (0=Zi..11=Hai)
    pub time_branch_idx: usize,
}

// ============================================================
// Fortune analysis data
// ============================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DaXian {
    pub index: u32,
    pub age_start: u32,
    pub age_end: u32,
    pub palace_idx: PalaceIndex,
    pub stem_hanja: String,
    pub branch_hanja: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LiuNian {
    pub year: i32,
    pub palace_idx: PalaceIndex,
    pub stem_hanja: String,
    pub branch_hanja: String,
    pub si_hua: [ZwdsStar; 4],
    pub liu_lu: PalaceIndex,
    pub liu_yang: PalaceIndex,
    pub liu_tuo: PalaceIndex,
    pub liu_chang: PalaceIndex,
    pub liu_qu: PalaceIndex,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LiuYue {
    pub month: u32,
    pub palace_idx: PalaceIndex,
    pub stem_hanja: String,
    pub branch_hanja: String,
    pub si_hua: [ZwdsStar; 4],
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LiuRi {
    pub day: u32,
    pub palace_idx: PalaceIndex,
    pub stem_hanja: String,
    pub branch_hanja: String,
    pub si_hua: [ZwdsStar; 4],
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZiHuaInfo {
    pub palace_idx: PalaceIndex,
    pub palace_name: PalaceName,
    pub star: ZwdsStar,
    pub sihua_type: SiHuaType,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TripleSihuaCollision {
    pub palace_idx: PalaceIndex,
    pub palace_name: PalaceName,
    pub collision_type: String,
    pub severity: String,
    pub description: String,
    pub stars_involved: Vec<ZwdsStar>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BorrowedStarsInfo {
    pub palace_idx: PalaceIndex,
    pub palace_name: PalaceName,
    pub opposite_palace_idx: PalaceIndex,
    pub borrowed_stars: Vec<ZwdsStar>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyPattern {
    pub name_hanja: String,
    pub name_korean: String,
    pub name_english: String,
    pub is_auspicious: bool,
    pub description_korean: String,
    pub description_english: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlyingSiHua {
    pub from_palace: PalaceName,
    pub to_palace: PalaceName,
    pub sihua_type: SiHuaType,
    pub star: ZwdsStar,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZwdsChart {
    pub palaces: [PalaceData; 12],
    pub soul_idx: PalaceIndex,
    pub body_idx: PalaceIndex,
    pub soul_master: ZwdsStar,
    pub body_master: ZwdsStar,
    pub five_elements: FiveElementsClass,
    pub daxian: Vec<DaXian>,
    pub destiny_patterns: Vec<DestinyPattern>,
    pub flying_sihua: Vec<FlyingSiHua>,
    #[serde(default)]
    pub zi_hua: Vec<ZiHuaInfo>,
    #[serde(default)]
    pub collisions: Vec<TripleSihuaCollision>,
    #[serde(default)]
    pub borrowed_stars: Vec<BorrowedStarsInfo>,
}
