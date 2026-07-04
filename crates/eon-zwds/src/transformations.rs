//! 자미두수 사화(四化: 化祿·化權·化科·化忌) 연산 모듈
//!
//! 출생년 천간에 따른 사화 배열을 조회하고, 별들에 사화를 부여하는 역할을 합니다.

use crate::types::{SiHuaType, ZwdsStar};
use eon_saju::core::stem::HeavenlyStem;

/// 출생년 천간에 따른 [化祿, 化權, 化科, 化忌] 대상 별들을 조회합니다.
pub fn get_sihua_stars(year_stem: HeavenlyStem) -> [ZwdsStar; 4] {
    match year_stem {
        HeavenlyStem::Jia => [
            ZwdsStar::LianZhen,
            ZwdsStar::PoJun,
            ZwdsStar::WuQu,
            ZwdsStar::TaiYang,
        ],
        HeavenlyStem::Yi => [
            ZwdsStar::TianJi,
            ZwdsStar::TianLiang,
            ZwdsStar::ZiWei,
            ZwdsStar::TaiYin,
        ],
        HeavenlyStem::Bing => [
            ZwdsStar::TianTong,
            ZwdsStar::TianJi,
            ZwdsStar::WenChang,
            ZwdsStar::LianZhen,
        ],
        HeavenlyStem::Ding => [
            ZwdsStar::TaiYin,
            ZwdsStar::TianTong,
            ZwdsStar::TianJi,
            ZwdsStar::JuMen,
        ],
        HeavenlyStem::Wu => [
            ZwdsStar::TanLang,
            ZwdsStar::TaiYin,
            ZwdsStar::YouBi,
            ZwdsStar::TianJi,
        ],
        HeavenlyStem::Ji => [
            ZwdsStar::WuQu,
            ZwdsStar::TanLang,
            ZwdsStar::TianLiang,
            ZwdsStar::WenQu,
        ],
        HeavenlyStem::Geng => [
            ZwdsStar::TaiYang,
            ZwdsStar::WuQu,
            ZwdsStar::TaiYin,
            ZwdsStar::TianTong,
        ],
        HeavenlyStem::Xin => [
            ZwdsStar::JuMen,
            ZwdsStar::TaiYang,
            ZwdsStar::WenQu,
            ZwdsStar::WenChang,
        ],
        HeavenlyStem::Ren => [
            ZwdsStar::TianLiang,
            ZwdsStar::ZiWei,
            ZwdsStar::ZuoFu,
            ZwdsStar::WuQu,
        ],
        HeavenlyStem::Gui => [
            ZwdsStar::PoJun,
            ZwdsStar::JuMen,
            ZwdsStar::TaiYin,
            ZwdsStar::TanLang,
        ],
    }
}

/// 특정 별에 해당하는 사화 타입이 있는지 검사하여 반환합니다.
pub fn get_sihua_for_star(star: ZwdsStar, sihua_stars: &[ZwdsStar; 4]) -> Option<SiHuaType> {
    if star == sihua_stars[0] {
        Some(SiHuaType::HuaLu)
    } else if star == sihua_stars[1] {
        Some(SiHuaType::HuaQuan)
    } else if star == sihua_stars[2] {
        Some(SiHuaType::HuaKe)
    } else if star == sihua_stars[3] {
        Some(SiHuaType::HuaJi)
    } else {
        None
    }
}
