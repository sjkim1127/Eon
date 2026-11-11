//! Zi Wei Dou Shu Chart construction module.
//!
//! Builds `ZwdsChart` from `BirthInfo`: lunar conversion, Ming/Body palace calculation, 12-palace assembly, decadal periods, and destiny rulers.

use eon_core::birth::BirthInfo;
use eon_core::birth::Gender;
use eon_saju::core::branch::EarthlyBranch;
use eon_saju::core::stem::HeavenlyStem;

use crate::calendar::solar_to_lunar_birth;
use crate::decadal::calculate_daxian;
use crate::error::ZwdsError;
use crate::palace::{
    get_five_elements, get_palace_name, get_palace_stem, get_soul_and_body_index,
    zwds_idx_to_std_idx,
};
use crate::stars::place_all_stars;
use crate::transformations::get_sihua_stars;
use crate::types::{LunarBirthInfo, PalaceData, StarInPalace, ZwdsChart, ZwdsStar};

/// Build `ZwdsChart` from `BirthInfo`.
pub fn build_chart(birth: &BirthInfo) -> Result<ZwdsChart, ZwdsError> {
    // 1. Solar to lunar calendar conversion
    let lunar = solar_to_lunar_birth(birth.year, birth.month, birth.day, birth.hour)?;

    build_chart_from_lunar(&lunar, birth)
}

/// Build ZWDS chart from converted lunar birth information.
pub fn build_chart_from_lunar(
    lunar: &LunarBirthInfo,
    birth: &BirthInfo,
) -> Result<ZwdsChart, ZwdsError> {
    let year_stem = HeavenlyStem::from_index(lunar.year_stem_idx as i32);

    // 2. Calculate Ming (soul) and Body palace indices
    let (soul_idx, body_idx) = get_soul_and_body_index(lunar.month, lunar.time_branch_idx);

    // 3. Determine Ming palace Heavenly Stem and Five Elements Bureau
    let soul_stem = get_palace_stem(year_stem, soul_idx);
    let five_elements = get_five_elements(soul_stem, soul_idx);

    // 4. Calculate star placements (HashMap<ZwdsStar, PalaceIndex>)
    let star_positions = place_all_stars(lunar, five_elements.value(), soul_idx, body_idx);

    // 5. Query natal Si Hua stars for birth year stem
    let sihua_stars = get_sihua_stars(year_stem);

    // 6. Generate decadal luck periods
    let daxian_list = calculate_daxian(
        birth.gender.unwrap_or(Gender::Male),
        year_stem,
        soul_idx,
        five_elements,
    );

    // 7. Assemble 12 palace data
    // PalaceIndex: 0=Yin .. 11=Chou
    let mut palaces_vec = Vec::new();
    for p_idx in 0..12 {
        let p_name = get_palace_name(soul_idx, p_idx);
        let p_stem = get_palace_stem(year_stem, p_idx);
        let branch_std = zwds_idx_to_std_idx(p_idx);
        let p_branch = EarthlyBranch::from_index(branch_std as i32);

        // Stars and Si Hua tags located in this palace
        let mut stars_in_palace = Vec::new();
        for (&star, &pos) in star_positions.iter() {
            if pos == p_idx {
                // Check Si Hua tag
                let si_hua = if star == sihua_stars[0] {
                    Some(crate::types::SiHuaType::HuaLu)
                } else if star == sihua_stars[1] {
                    Some(crate::types::SiHuaType::HuaQuan)
                } else if star == sihua_stars[2] {
                    Some(crate::types::SiHuaType::HuaKe)
                } else if star == sihua_stars[3] {
                    Some(crate::types::SiHuaType::HuaJi)
                } else {
                    None
                };

                stars_in_palace.push(StarInPalace {
                    star,
                    si_hua,
                    brightness: crate::brightness::get_star_brightness(star, p_idx),
                });
            }
        }

        // Sort stars (major stars first)
        stars_in_palace.sort_by_key(|s| !s.star.is_main_star());

        // Map decadal age range
        let daxian_range = daxian_list
            .iter()
            .find(|d| d.palace_idx == p_idx)
            .map(|d| (d.age_start as u8, d.age_end as u8));

        palaces_vec.push(PalaceData {
            index: p_idx,
            name: p_name,
            heavenly_stem: p_stem.hanja().to_string(),
            earthly_branch: p_branch.hanja().to_string(),
            stars: stars_in_palace,
            daxian_range,
            is_current_liu_nian: false, // Updated by service layer for specific target year
        });
    }

    let palaces: [PalaceData; 12] = palaces_vec
        .try_into()
        .map_err(|_| ZwdsError::Internal("Failed to generate 12-palace data".to_string()))?;

    // 8. Determine Ming Zhu (Life Master) and Shen Zhu (Body Master)
    // Ming Zhu determined by Earthly Branch of Ming palace
    let soul_branch_std_idx = zwds_idx_to_std_idx(soul_idx);
    let soul_master = match soul_branch_std_idx {
        0 => ZwdsStar::TanLang,  // 子
        1 => ZwdsStar::JuMen,    // 丑
        2 => ZwdsStar::LuCun,    // 寅
        3 => ZwdsStar::WenQu,    // 卯
        4 => ZwdsStar::LianZhen, // 辰
        5 => ZwdsStar::WuQu,     // 巳
        6 => ZwdsStar::PoJun,    // 午
        7 => ZwdsStar::WuQu,     // 未
        8 => ZwdsStar::LianZhen, // 申
        9 => ZwdsStar::WenQu,    // 酉
        10 => ZwdsStar::LuCun,   // 戌
        11 => ZwdsStar::JuMen,   // 亥
        _ => unreachable!(),
    };

    // Shen Zhu determined by Earthly Branch of birth year
    let body_master = match lunar.year_branch_idx {
        0 => ZwdsStar::HuoXing,   // 子
        1 => ZwdsStar::TianXiang, // 丑
        2 => ZwdsStar::TianLiang, // 寅
        3 => ZwdsStar::TianTong,  // 卯
        4 => ZwdsStar::WenChang,  // 辰
        5 => ZwdsStar::TianJi,    // 巳
        6 => ZwdsStar::HuoXing,   // 午
        7 => ZwdsStar::TianXiang, // 未
        8 => ZwdsStar::TianLiang, // 申
        9 => ZwdsStar::TianTong,  // 酉
        10 => ZwdsStar::WenChang, // 戌
        11 => ZwdsStar::TianJi,   // 亥
        _ => unreachable!(),
    };

    let destiny_patterns =
        crate::destiny_patterns::analyze_destiny_patterns(soul_idx, &star_positions, &palaces);

    // 9. Calculate Flying Si Hua between palaces
    let mut flying_sihua = Vec::new();
    for p_idx in 0..12 {
        let p_stem = get_palace_stem(year_stem, p_idx);
        let sihua_stars = get_sihua_stars(p_stem);
        let from_palace = get_palace_name(soul_idx, p_idx);

        let types = [
            (sihua_stars[0], crate::types::SiHuaType::HuaLu),
            (sihua_stars[1], crate::types::SiHuaType::HuaQuan),
            (sihua_stars[2], crate::types::SiHuaType::HuaKe),
            (sihua_stars[3], crate::types::SiHuaType::HuaJi),
        ];

        for (star, sihua_type) in types {
            if let Some(&to_p_idx) = star_positions.get(&star) {
                let to_palace = get_palace_name(soul_idx, to_p_idx);
                flying_sihua.push(crate::types::FlyingSiHua {
                    from_palace,
                    to_palace,
                    sihua_type,
                    star,
                });
            }
        }
    }

    let zi_hua = crate::collisions_zihua::detect_zi_hua(&palaces);
    let borrowed_stars = crate::collisions_zihua::detect_borrowed_stars(&palaces);
    let collisions =
        crate::collisions_zihua::detect_triple_sihua_collisions(&palaces, &daxian_list, None);

    Ok(ZwdsChart {
        palaces,
        soul_idx,
        body_idx,
        soul_master,
        body_master,
        five_elements,
        daxian: daxian_list,
        destiny_patterns,
        flying_sihua,
        zi_hua,
        collisions,
        borrowed_stars,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{PalaceName, SiHuaType};
    use eon_core::birth::Gender;

    #[test]
    fn test_build_chart() {
        let birth = BirthInfo::solar(2004, 11, 27, 22, 0)
            .with_gender(Gender::Male)
            .with_location(eon_core::Location::seoul())
            .with_korea_timezone();

        let chart = build_chart(&birth).unwrap();
        // 2004-11-27 22:00 -> Lunar 10-16, Hai hour.
        // Ming palace is Zi (10), Body palace is Xu (8).
        assert_eq!(chart.soul_idx, 10);
        assert_eq!(chart.body_idx, 8);

        // Ming Zhu: Ming branch Zi (0) -> TanLang
        assert_eq!(chart.soul_master, ZwdsStar::TanLang);

        // Shen Zhu: Year branch Shen (8) -> TianLiang
        assert_eq!(chart.body_master, ZwdsStar::TianLiang);

        // Verify brightness is populated for all major stars
        let mut main_star_count = 0;
        for palace in chart.palaces.iter() {
            for star_in_p in palace.stars.iter() {
                if star_in_p.star.is_main_star() {
                    assert!(
                        star_in_p.brightness.is_some(),
                        "Brightness missing for major star {:?}",
                        star_in_p.star
                    );
                    main_star_count += 1;
                }
            }
        }
        assert_eq!(main_star_count, 14, "All 14 major stars must be placed");
    }

    #[test]
    fn test_flying_sihua_calculation() {
        let birth = BirthInfo::solar(2004, 11, 27, 22, 0)
            .with_gender(Gender::Male)
            .with_location(eon_core::Location::seoul())
            .with_korea_timezone();

        let chart = build_chart(&birth).unwrap();

        // 1. Flying Si Hua list must not be empty
        assert!(
            !chart.flying_sihua.is_empty(),
            "Flying Si Hua list cannot be empty"
        );
        // 12 palaces * 4 transformations = 48 flying stars
        assert_eq!(
            chart.flying_sihua.len(),
            48,
            "Total of 48 Flying Si Hua transformations required"
        );

        // 2. Verify Flying Si Hua originating from Ming palace (Zi = index 10)
        // 2004 Jia-Shen year -> Yin palace is Bing-Yin -> Zi palace is Bing-Zi (Stem: Bing)
        // Bing stem Si Hua: Tong(Lu), Ji(Quan), Chang(Ke), Lian(Ji)
        let outbound_from_子 = chart
            .flying_sihua
            .iter()
            .filter(|fs| fs.from_palace == PalaceName::Ming) // Ming palace located at Zi (10)
            .collect::<Vec<_>>();
        assert_eq!(
            outbound_from_子.len(),
            4,
            "Ming palace must project 4 Flying Si Hua transformations"
        );

        let lu_star = outbound_from_子
            .iter()
            .find(|fs| fs.sihua_type == SiHuaType::HuaLu)
            .unwrap();
        assert_eq!(
            lu_star.star,
            ZwdsStar::TianTong,
            "Bing stem Hua Lu must be TianTong"
        );

        let quan_star = outbound_from_子
            .iter()
            .find(|fs| fs.sihua_type == SiHuaType::HuaQuan)
            .unwrap();
        assert_eq!(
            quan_star.star,
            ZwdsStar::TianJi,
            "Bing stem Hua Quan must be TianJi"
        );

        let ke_star = outbound_from_子
            .iter()
            .find(|fs| fs.sihua_type == SiHuaType::HuaKe)
            .unwrap();
        assert_eq!(
            ke_star.star,
            ZwdsStar::WenChang,
            "Bing stem Hua Ke must be WenChang"
        );

        let ji_star = outbound_from_子
            .iter()
            .find(|fs| fs.sihua_type == SiHuaType::HuaJi)
            .unwrap();
        assert_eq!(
            ji_star.star,
            ZwdsStar::LianZhen,
            "Bing stem Hua Ji must be LianZhen"
        );
    }
}
