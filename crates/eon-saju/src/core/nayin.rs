//! Nayin Five Elements (納音五行)
//!
//! Calculates the Nayin Five Element corresponding to each of the 60 pillars.
//! Nayin represents unique energetic qualities formed by stem-branch synthesis.
//!
//! ## Reference
//! - bazica (Go BaZi Calculator) - get_ganzhi.go

use crate::core::element::Element;
use crate::core::ganzi::GanZi;
use serde::{Deserialize, Serialize};

/// Nayin Element Type (60 pillars grouped into 30 pairs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NayinType {
    /// Sea Metal (海中金)
    SeaGold,
    /// Furnace Fire (爐中火)
    FurnaceFire,
    /// Forest Wood (大林木)
    ForestWood,
    /// Road Earth (路傍土)
    RoadEarth,
    /// Sword Metal (劍鋒金)
    SwordGold,
    /// Volcano Fire (山頭火)
    VolcanoFire,
    /// Cave Water (澗下水)
    CaveWater,
    /// Fortress Earth (城頭土)
    FortressEarth,
    /// Wax Metal (白蠟金)
    WaxGold,
    /// Willow Wood (楊柳木)
    WillowWood,
    /// Stream Water (泉中水)
    StreamWater,
    /// Roof Earth (屋上土)
    RoofEarth,
    /// Lightning Fire (霹靂火)
    LightningFire,
    /// Pine Wood (松柏木)
    PineWood,
    /// River Water (長流水)
    RiverWater,
    /// Sand Metal (砂中金)
    SandGold,
    /// Forest Fire (山下火)
    ForestFire,
    /// Meadow Wood (平地木)
    MeadowWood,
    /// Adobe Earth (壁上土)
    AdobeEarth,
    /// Precious Metal (金箔金)
    PreciousGold,
    /// Lamp Fire (覆燈火)
    LampFire,
    /// Sky Water (天河水)
    SkyWater,
    /// Highway Earth (大驛土)
    HighwayEarth,
    /// Jewelry Metal (釵釧金)
    JewelryGold,
    /// Mulberry Wood (桑柘木)
    MulberryWood,
    /// Rapids Water (大溪水)
    RapidsWater,
    /// Desert Earth (沙中土)
    DesertEarth,
    /// Sun Fire (天上火)
    SunFire,
    /// Pomegranate Wood (石榴木)
    PomegranateWood,
    /// Ocean Water (大海水)
    OceanWater,
}

impl NayinType {
    /// Return the primary element of the Nayin type
    pub fn element(&self) -> Element {
        match self {
            Self::SeaGold
            | Self::SwordGold
            | Self::WaxGold
            | Self::SandGold
            | Self::PreciousGold
            | Self::JewelryGold => Element::Metal,

            Self::FurnaceFire
            | Self::VolcanoFire
            | Self::LightningFire
            | Self::ForestFire
            | Self::LampFire
            | Self::SunFire => Element::Fire,

            Self::ForestWood
            | Self::WillowWood
            | Self::PineWood
            | Self::MeadowWood
            | Self::MulberryWood
            | Self::PomegranateWood => Element::Wood,

            Self::RoadEarth
            | Self::FortressEarth
            | Self::RoofEarth
            | Self::AdobeEarth
            | Self::HighwayEarth
            | Self::DesertEarth => Element::Earth,

            Self::CaveWater
            | Self::StreamWater
            | Self::RiverWater
            | Self::SkyWater
            | Self::RapidsWater
            | Self::OceanWater => Element::Water,
        }
    }

    /// English name with Hanja
    pub fn hangul(&self) -> &'static str {
        match self {
            Self::SeaGold => "Sea Metal (海中金)",
            Self::FurnaceFire => "Furnace Fire (爐中火)",
            Self::ForestWood => "Forest Wood (大林木)",
            Self::RoadEarth => "Road Earth (路傍土)",
            Self::SwordGold => "Sword Metal (劍鋒金)",
            Self::VolcanoFire => "Volcano Fire (山頭火)",
            Self::CaveWater => "Cave Water (澗下水)",
            Self::FortressEarth => "Fortress Earth (城頭土)",
            Self::WaxGold => "Wax Metal (白蠟金)",
            Self::WillowWood => "Willow Wood (楊柳木)",
            Self::StreamWater => "Stream Water (泉中水)",
            Self::RoofEarth => "Roof Earth (屋上土)",
            Self::LightningFire => "Lightning Fire (霹靂火)",
            Self::PineWood => "Pine Wood (松柏木)",
            Self::RiverWater => "River Water (長流水)",
            Self::SandGold => "Sand Metal (砂中金)",
            Self::ForestFire => "Forest Fire (山下火)",
            Self::MeadowWood => "Meadow Wood (平地木)",
            Self::AdobeEarth => "Adobe Earth (壁上土)",
            Self::PreciousGold => "Precious Metal (金箔金)",
            Self::LampFire => "Lamp Fire (覆燈火)",
            Self::SkyWater => "Sky Water (天河水)",
            Self::HighwayEarth => "Highway Earth (大驛土)",
            Self::JewelryGold => "Jewelry Metal (釵釧金)",
            Self::MulberryWood => "Mulberry Wood (桑柘木)",
            Self::RapidsWater => "Rapids Water (大溪水)",
            Self::DesertEarth => "Desert Earth (沙中土)",
            Self::SunFire => "Sun Fire (天上火)",
            Self::PomegranateWood => "Pomegranate Wood (石榴木)",
            Self::OceanWater => "Ocean Water (大海水)",
        }
    }

    /// English name
    pub fn english(&self) -> &'static str {
        match self {
            Self::SeaGold => "Sea Metal",
            Self::FurnaceFire => "Furnace Fire",
            Self::ForestWood => "Forest Wood",
            Self::RoadEarth => "Road Earth",
            Self::SwordGold => "Sword Metal",
            Self::VolcanoFire => "Volcano Fire",
            Self::CaveWater => "Cave Water",
            Self::FortressEarth => "Fortress Earth",
            Self::WaxGold => "Wax Metal",
            Self::WillowWood => "Willow Wood",
            Self::StreamWater => "Stream Water",
            Self::RoofEarth => "Roof Earth",
            Self::LightningFire => "Lightning Fire",
            Self::PineWood => "Pine Wood",
            Self::RiverWater => "River Water",
            Self::SandGold => "Sand Metal",
            Self::ForestFire => "Forest Fire",
            Self::MeadowWood => "Meadow Wood",
            Self::AdobeEarth => "Adobe Earth",
            Self::PreciousGold => "Precious Metal",
            Self::LampFire => "Lamp Fire",
            Self::SkyWater => "Sky Water",
            Self::HighwayEarth => "Highway Earth",
            Self::JewelryGold => "Jewelry Metal",
            Self::MulberryWood => "Mulberry Wood",
            Self::RapidsWater => "Rapids Water",
            Self::DesertEarth => "Desert Earth",
            Self::SunFire => "Sun Fire",
            Self::PomegranateWood => "Pomegranate Wood",
            Self::OceanWater => "Ocean Water",
        }
    }
}

/// 60 Cycle Nayin lookup table (Index 0-59)
const NAYIN_TABLE: [NayinType; 60] = [
    // JiaZi (0), YiChou (1)
    NayinType::SeaGold,
    NayinType::SeaGold,
    // BingYin (2), DingMao (3)
    NayinType::FurnaceFire,
    NayinType::FurnaceFire,
    // WuChen (4), JiSi (5)
    NayinType::ForestWood,
    NayinType::ForestWood,
    // GengWu (6), XinWei (7)
    NayinType::RoadEarth,
    NayinType::RoadEarth,
    // RenShen (8), GuiYou (9)
    NayinType::SwordGold,
    NayinType::SwordGold,
    // JiaXu (10), YiHai (11)
    NayinType::VolcanoFire,
    NayinType::VolcanoFire,
    // BingZi (12), DingChou (13)
    NayinType::CaveWater,
    NayinType::CaveWater,
    // WuYin (14), JiMao (15)
    NayinType::FortressEarth,
    NayinType::FortressEarth,
    // GengChen (16), XinSi (17)
    NayinType::WaxGold,
    NayinType::WaxGold,
    // RenWu (18), GuiWei (19)
    NayinType::WillowWood,
    NayinType::WillowWood,
    // JiaShen (20), YiYou (21)
    NayinType::StreamWater,
    NayinType::StreamWater,
    // BingXu (22), DingHai (23)
    NayinType::RoofEarth,
    NayinType::RoofEarth,
    // WuZi (24), JiChou (25)
    NayinType::LightningFire,
    NayinType::LightningFire,
    // GengYin (26), XinMao (27)
    NayinType::PineWood,
    NayinType::PineWood,
    // RenChen (28), GuiSi (29)
    NayinType::RiverWater,
    NayinType::RiverWater,
    // JiaWu (30), YiWei (31)
    NayinType::SandGold,
    NayinType::SandGold,
    // BingShen (32), DingYou (33)
    NayinType::ForestFire,
    NayinType::ForestFire,
    // WuXu (34), JiHai (35)
    NayinType::MeadowWood,
    NayinType::MeadowWood,
    // GengZi (36), XinChou (37)
    NayinType::AdobeEarth,
    NayinType::AdobeEarth,
    // RenYin (38), GuiMao (39)
    NayinType::PreciousGold,
    NayinType::PreciousGold,
    // JiaChen (40), YiSi (41)
    NayinType::LampFire,
    NayinType::LampFire,
    // BingWu (42), DingWei (43)
    NayinType::SkyWater,
    NayinType::SkyWater,
    // WuShen (44), JiYou (45)
    NayinType::HighwayEarth,
    NayinType::HighwayEarth,
    // GengXu (46), XinHai (47)
    NayinType::JewelryGold,
    NayinType::JewelryGold,
    // RenZi (48), GuiChou (49)
    NayinType::MulberryWood,
    NayinType::MulberryWood,
    // JiaYin (50), YiMao (51)
    NayinType::RapidsWater,
    NayinType::RapidsWater,
    // BingChen (52), DingSi (53)
    NayinType::DesertEarth,
    NayinType::DesertEarth,
    // WuWu (54), JiWei (55)
    NayinType::SunFire,
    NayinType::SunFire,
    // GengShen (56), XinYou (57)
    NayinType::PomegranateWood,
    NayinType::PomegranateWood,
    // RenXu (58), GuiHai (59)
    NayinType::OceanWater,
    NayinType::OceanWater,
];

/// Calculate Nayin from GanZi
pub fn get_nayin(ganzi: &GanZi) -> NayinType {
    let idx = ganzi.index() as usize;
    NAYIN_TABLE[idx]
}

impl GanZi {
    /// Return Nayin Five Element for this GanZi
    pub fn nayin(&self) -> NayinType {
        get_nayin(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::branch::EarthlyBranch;
    use crate::core::stem::HeavenlyStem;

    #[test]
    fn test_nayin_basic() {
        let jiazi = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Zi);
        assert_eq!(jiazi.nayin(), NayinType::SeaGold);
        assert_eq!(jiazi.nayin().element(), Element::Metal);

        let bingyin = GanZi::new(HeavenlyStem::Bing, EarthlyBranch::Yin);
        assert_eq!(bingyin.nayin(), NayinType::FurnaceFire);
        assert_eq!(bingyin.nayin().element(), Element::Fire);

        let gengxu = GanZi::new(HeavenlyStem::Geng, EarthlyBranch::Xu);
        assert_eq!(gengxu.nayin(), NayinType::JewelryGold);
        assert_eq!(gengxu.nayin().element(), Element::Metal);
    }

    #[test]
    fn test_nayin_pairs() {
        let jiazi = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Zi);
        let yichou = GanZi::new(HeavenlyStem::Yi, EarthlyBranch::Chou);
        assert_eq!(jiazi.nayin(), yichou.nayin());
    }

    #[test]
    fn test_nayin_hangul() {
        let jiazi = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Zi);
        assert_eq!(jiazi.nayin().hangul(), "Sea Metal (海中金)");
    }
}
