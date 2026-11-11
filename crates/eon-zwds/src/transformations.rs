//! Zi Wei Dou Shu Four Transformations (Si Hua) module.
//!
//! Resolves Si Hua star assignments based on natal year Heavenly Stem.

use crate::types::{SiHuaType, ZwdsStar};
use eon_saju::core::stem::HeavenlyStem;

/// Query target stars for [HuaLu, HuaQuan, HuaKe, HuaJi] based on birth year stem.
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

/// Check if a star holds a transformation tag.
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
