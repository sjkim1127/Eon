//! 납음(納音) 오행 - Nayin Five Elements
//!
//! 60갑자 각각에 해당하는 납음 오행을 계산합니다.
//! 납음은 천간과 지지의 조합에 따른 고유한 오행 특성을 나타냅니다.
//!
//! ## 참조
//! - bazica (Go BaZi Calculator) - get_ganzhi.go

use crate::core::element::Element;
use crate::core::ganzi::GanZi;
use serde::{Deserialize, Serialize};

/// 납음 오행 유형 (60갑자를 30쌍으로 그룹화)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NayinType {
    /// 해중금 (海中金) - 바다 속의 금
    SeaGold,
    /// 노중화 (爐中火) - 화로 속의 불
    FurnaceFire,
    /// 대림목 (大林木) - 큰 숲의 나무
    ForestWood,
    /// 노방토 (路傍土) - 길가의 흙
    RoadEarth,
    /// 검봉금 (劍鋒金) - 칼날의 금
    SwordGold,
    /// 산두화 (山頭火) - 산 정상의 불
    VolcanoFire,
    /// 간하수 (澗下水) - 골짜기 물
    CaveWater,
    /// 성두토 (城頭土) - 성벽의 흙
    FortressEarth,
    /// 백랍금 (白蠟金) - 백랍(밀랍) 금
    WaxGold,
    /// 양류목 (楊柳木) - 버드나무
    WillowWood,
    /// 천하수 (泉中水) - 샘물
    StreamWater,
    /// 옥상토 (屋上土) - 지붕 위의 흙
    RoofEarth,
    /// 벽력화 (霹靂火) - 번개불
    LightningFire,
    /// 송백목 (松柏木) - 소나무/측백나무
    PineWood,
    /// 장류수 (長流水) - 큰 강물
    RiverWater,
    /// 사중금 (砂中金) - 모래 속의 금
    SandGold,
    /// 산하화 (山下火) - 산 아래의 불
    ForestFire,
    /// 평지목 (平地木) - 평지의 나무
    MeadowWood,
    /// 벽상토 (壁上土) - 담장의 흙
    AdobeEarth,
    /// 금박금 (金箔金) - 금박 금
    PreciousGold,
    /// 복등화 (覆燈火) - 등불
    LampFire,
    /// 천하수 (天河水) - 하늘의 강물
    SkyWater,
    /// 대역토 (大驛土) - 큰 역참의 흙
    HighwayEarth,
    /// 차천금 (釵釧金) - 비녀/팔찌 금
    JewelryGold,
    /// 상자목 (桑柘木) - 뽕나무
    MulberryWood,
    /// 대계수 (大溪水) - 큰 계곡의 물
    RapidsWater,
    /// 사중토 (沙中土) - 사막의 흙
    DesertEarth,
    /// 천상화 (天上火) - 하늘의 불 (태양)
    SunFire,
    /// 석류목 (石榴木) - 석류나무
    PomegranateWood,
    /// 대해수 (大海水) - 큰 바다의 물
    OceanWater,
}

impl NayinType {
    /// 납음 오행의 기본 오행을 반환
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

    /// 한국어 이름
    pub fn hangul(&self) -> &'static str {
        match self {
            Self::SeaGold => "해중금(海中金)",
            Self::FurnaceFire => "노중화(爐中火)",
            Self::ForestWood => "대림목(大林木)",
            Self::RoadEarth => "노방토(路傍土)",
            Self::SwordGold => "검봉금(劍鋒金)",
            Self::VolcanoFire => "산두화(山頭火)",
            Self::CaveWater => "간하수(澗下水)",
            Self::FortressEarth => "성두토(城頭土)",
            Self::WaxGold => "백랍금(白蠟金)",
            Self::WillowWood => "양류목(楊柳木)",
            Self::StreamWater => "천중수(泉中水)",
            Self::RoofEarth => "옥상토(屋上土)",
            Self::LightningFire => "벽력화(霹靂火)",
            Self::PineWood => "송백목(松柏木)",
            Self::RiverWater => "장류수(長流水)",
            Self::SandGold => "사중금(砂中金)",
            Self::ForestFire => "산하화(山下火)",
            Self::MeadowWood => "평지목(平地木)",
            Self::AdobeEarth => "벽상토(壁上土)",
            Self::PreciousGold => "금박금(金箔金)",
            Self::LampFire => "복등화(覆燈火)",
            Self::SkyWater => "천하수(天河水)",
            Self::HighwayEarth => "대역토(大驛土)",
            Self::JewelryGold => "차천금(釵釧金)",
            Self::MulberryWood => "상자목(桑拓木)",
            Self::RapidsWater => "대계수(大溪水)",
            Self::DesertEarth => "사중토(沙中土)",
            Self::SunFire => "천상화(天上火)",
            Self::PomegranateWood => "석류목(石榴木)",
            Self::OceanWater => "대해수(大海水)",
        }
    }

    /// 영어 이름
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

/// 60갑자 납음 테이블 (인덱스 0-59)
const NAYIN_TABLE: [NayinType; 60] = [
    // 갑자(0), 을축(1)
    NayinType::SeaGold,
    NayinType::SeaGold,
    // 병인(2), 정묘(3)
    NayinType::FurnaceFire,
    NayinType::FurnaceFire,
    // 무진(4), 기사(5)
    NayinType::ForestWood,
    NayinType::ForestWood,
    // 경오(6), 신미(7)
    NayinType::RoadEarth,
    NayinType::RoadEarth,
    // 임신(8), 계유(9)
    NayinType::SwordGold,
    NayinType::SwordGold,
    // 갑술(10), 을해(11)
    NayinType::VolcanoFire,
    NayinType::VolcanoFire,
    // 병자(12), 정축(13)
    NayinType::CaveWater,
    NayinType::CaveWater,
    // 무인(14), 기묘(15)
    NayinType::FortressEarth,
    NayinType::FortressEarth,
    // 경진(16), 신사(17)
    NayinType::WaxGold,
    NayinType::WaxGold,
    // 임오(18), 계미(19)
    NayinType::WillowWood,
    NayinType::WillowWood,
    // 갑신(20), 을유(21)
    NayinType::StreamWater,
    NayinType::StreamWater,
    // 병술(22), 정해(23)
    NayinType::RoofEarth,
    NayinType::RoofEarth,
    // 무자(24), 기축(25)
    NayinType::LightningFire,
    NayinType::LightningFire,
    // 경인(26), 신묘(27)
    NayinType::PineWood,
    NayinType::PineWood,
    // 임진(28), 계사(29)
    NayinType::RiverWater,
    NayinType::RiverWater,
    // 갑오(30), 을미(31)
    NayinType::SandGold,
    NayinType::SandGold,
    // 병신(32), 정유(33)
    NayinType::ForestFire,
    NayinType::ForestFire,
    // 무술(34), 기해(35)
    NayinType::MeadowWood,
    NayinType::MeadowWood,
    // 경자(36), 신축(37)
    NayinType::AdobeEarth,
    NayinType::AdobeEarth,
    // 임인(38), 계묘(39)
    NayinType::PreciousGold,
    NayinType::PreciousGold,
    // 갑진(40), 을사(41)
    NayinType::LampFire,
    NayinType::LampFire,
    // 병오(42), 정미(43)
    NayinType::SkyWater,
    NayinType::SkyWater,
    // 무신(44), 기유(45)
    NayinType::HighwayEarth,
    NayinType::HighwayEarth,
    // 경술(46), 신해(47)
    NayinType::JewelryGold,
    NayinType::JewelryGold,
    // 임자(48), 계축(49)
    NayinType::MulberryWood,
    NayinType::MulberryWood,
    // 갑인(50), 을묘(51)
    NayinType::RapidsWater,
    NayinType::RapidsWater,
    // 병진(52), 정사(53)
    NayinType::DesertEarth,
    NayinType::DesertEarth,
    // 무오(54), 기미(55)
    NayinType::SunFire,
    NayinType::SunFire,
    // 경신(56), 신유(57)
    NayinType::PomegranateWood,
    NayinType::PomegranateWood,
    // 임술(58), 계해(59)
    NayinType::OceanWater,
    NayinType::OceanWater,
];

/// GanZi에서 납음 오행을 계산
pub fn get_nayin(ganzi: &GanZi) -> NayinType {
    let idx = ganzi.index() as usize;
    NAYIN_TABLE[idx]
}

impl GanZi {
    /// 이 간지의 납음 오행을 반환
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
        // 갑자 = 해중금
        let jiazi = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Zi);
        assert_eq!(jiazi.nayin(), NayinType::SeaGold);
        assert_eq!(jiazi.nayin().element(), Element::Metal);

        // 병인 = 노중화
        let bingyin = GanZi::new(HeavenlyStem::Bing, EarthlyBranch::Yin);
        assert_eq!(bingyin.nayin(), NayinType::FurnaceFire);
        assert_eq!(bingyin.nayin().element(), Element::Fire);

        // 경술 = 차천금 (김성주 일주)
        let gengxu = GanZi::new(HeavenlyStem::Geng, EarthlyBranch::Xu);
        assert_eq!(gengxu.nayin(), NayinType::JewelryGold);
        assert_eq!(gengxu.nayin().element(), Element::Metal);
    }

    #[test]
    fn test_nayin_pairs() {
        // 연속된 두 간지는 같은 납음을 공유
        let jiazi = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Zi);
        let yichou = GanZi::new(HeavenlyStem::Yi, EarthlyBranch::Chou);
        assert_eq!(jiazi.nayin(), yichou.nayin());
    }

    #[test]
    fn test_nayin_hangul() {
        let jiazi = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Zi);
        assert_eq!(jiazi.nayin().hangul(), "해중금(海中金)");
    }
}
