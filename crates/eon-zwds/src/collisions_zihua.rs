//! Palace Self-Transformation (ZiHua), Triple SiHua Collision, and Borrowed Stars analysis engine

use crate::transformations::get_sihua_stars;
use crate::types::{
    BorrowedStarsInfo, DaXian, LiuNian, PalaceData, SiHuaType, TripleSihuaCollision, ZiHuaInfo,
    ZwdsStar,
};
use eon_saju::core::stem::HeavenlyStem;

/// Detects Self-Transformation (ZiHua) triggered on resident stars by palace stems
pub fn detect_zi_hua(palaces: &[PalaceData; 12]) -> Vec<ZiHuaInfo> {
    let mut results = Vec::new();

    for p in palaces {
        if let Ok(stem) = parse_stem(&p.heavenly_stem) {
            let sihua_stars = get_sihua_stars(stem);
            let sihua_types = [
                SiHuaType::HuaLu,
                SiHuaType::HuaQuan,
                SiHuaType::HuaKe,
                SiHuaType::HuaJi,
            ];

            for (idx, &s_star) in sihua_stars.iter().enumerate() {
                if p.stars.iter().any(|st| st.star == s_star) {
                    let s_type = sihua_types[idx];
                    let desc = format!(
                        "Self-Transformation ({:?}) triggered in {} via Palace Stem {} — affects star {:?}",
                        s_type,
                        p.name.english(),
                        p.heavenly_stem,
                        s_star
                    );
                    results.push(ZiHuaInfo {
                        palace_idx: p.index,
                        palace_name: p.name,
                        star: s_star,
                        sihua_type: s_type,
                        description: desc,
                    });
                }
            }
        }
    }

    results
}

/// Detects Borrowed Stars when a palace is empty of major stars (borrowing from opposite palace)
pub fn detect_borrowed_stars(palaces: &[PalaceData; 12]) -> Vec<BorrowedStarsInfo> {
    let mut results = Vec::new();

    for p in palaces {
        // Check for 14 major stars
        let has_major_star = p.stars.iter().any(|st| st.star.is_major());
        if !has_major_star {
            let opposite_idx = (p.index + 6) % 12;
            let opp_palace = &palaces[opposite_idx];
            let borrowed: Vec<ZwdsStar> = opp_palace
                .stars
                .iter()
                .filter(|st| st.star.is_major())
                .map(|st| st.star)
                .collect();

            if !borrowed.is_empty() {
                results.push(BorrowedStarsInfo {
                    palace_idx: p.index,
                    palace_name: p.name,
                    opposite_palace_idx: opposite_idx,
                    borrowed_stars: borrowed,
                });
            }
        }
    }

    results
}

/// Detects Triple SiHua Collisions between natal, decadal, and annual mutagenic stars
pub fn detect_triple_sihua_collisions(
    palaces: &[PalaceData; 12],
    daxian_list: &[DaXian],
    current_liunian: Option<&LiuNian>,
) -> Vec<TripleSihuaCollision> {
    let mut collisions = Vec::new();

    if daxian_list.is_empty() {
        return collisions;
    }

    // Extract active decadal Si Hua
    let active_daxian = &daxian_list[0];
    let daxian_stem = parse_stem(&active_daxian.stem_hanja).ok();

    for p in palaces {
        let mut huaji_count = 0;
        let mut hualu_count = 0;
        let mut stars_involved = Vec::new();

        // 1. Check natal Si Hua
        for st in &p.stars {
            if let Some(s_type) = st.si_hua {
                match s_type {
                    SiHuaType::HuaJi => {
                        huaji_count += 1;
                        stars_involved.push(st.star);
                    }
                    SiHuaType::HuaLu => {
                        hualu_count += 1;
                        stars_involved.push(st.star);
                    }
                    _ => {}
                }
            }
        }

        // 2. Check decadal Si Hua
        if let Some(d_stem) = daxian_stem {
            let d_sihua = get_sihua_stars(d_stem);
            if p.stars.iter().any(|st| st.star == d_sihua[3]) {
                huaji_count += 1;
                stars_involved.push(d_sihua[3]);
            }
            if p.stars.iter().any(|st| st.star == d_sihua[0]) {
                hualu_count += 1;
                stars_involved.push(d_sihua[0]);
            }
        }

        // 3. Check annual Si Hua
        if let Some(ln) = current_liunian {
            if p.stars.iter().any(|st| st.star == ln.si_hua[3]) {
                huaji_count += 1;
                stars_involved.push(ln.si_hua[3]);
            }
            if p.stars.iter().any(|st| st.star == ln.si_hua[0]) {
                hualu_count += 1;
                stars_involved.push(ln.si_hua[0]);
            }
        }

        stars_involved.sort();
        stars_involved.dedup();

        if huaji_count >= 3 {
            collisions.push(TripleSihuaCollision {
                palace_idx: p.index,
                palace_name: p.name,
                collision_type: "Triple HuaJi (三化忌)".to_string(),
                severity: "Crisis".to_string(),
                description: format!(
                    "Triple HuaJi (化忌) overlap in {}, concentrating profound transformations and testing challenges.",
                    p.name.english()
                ),
                stars_involved: stars_involved.clone(),
            });
        } else if huaji_count >= 2 {
            collisions.push(TripleSihuaCollision {
                palace_idx: p.index,
                palace_name: p.name,
                collision_type: "Double HuaJi (雙化忌)".to_string(),
                severity: "Caution".to_string(),
                description: format!(
                    "Double HuaJi (化忌) overlap in {}, calling for heightened caution and measured risk mitigation.",
                    p.name.english()
                ),
                stars_involved: stars_involved.clone(),
            });
        }

        if hualu_count >= 2 {
            collisions.push(TripleSihuaCollision {
                palace_idx: p.index,
                palace_name: p.name,
                collision_type: "Double HuaLu (雙祿)".to_string(),
                severity: "Opportunity".to_string(),
                description: format!(
                    "Double HuaLu (化祿) overlap in {}, signaling significant financial expansion and auspicious fortune.",
                    p.name.english()
                ),
                stars_involved: stars_involved.clone(),
            });
        } else if hualu_count >= 1 && huaji_count >= 1 {
            collisions.push(TripleSihuaCollision {
                palace_idx: p.index,
                palace_name: p.name,
                collision_type: "Lu-Ji Clash (祿忌相沖)".to_string(),
                severity: "Caution".to_string(),
                description: format!(
                    "HuaLu and HuaJi coexist in {}, demanding proactive risk mitigation following major gains.",
                    p.name.english()
                ),
                stars_involved,
            });
        }
    }

    collisions
}

fn parse_stem(s: &str) -> Result<HeavenlyStem, ()> {
    match s {
        "甲" | "Jia" | "jia" => Ok(HeavenlyStem::Jia),
        "乙" | "Yi" | "yi" => Ok(HeavenlyStem::Yi),
        "丙" | "Bing" | "bing" => Ok(HeavenlyStem::Bing),
        "丁" | "Ding" | "ding" => Ok(HeavenlyStem::Ding),
        "戊" | "Wu" | "wu" => Ok(HeavenlyStem::Wu),
        "己" | "Ji" | "ji" => Ok(HeavenlyStem::Ji),
        "庚" | "Geng" | "geng" => Ok(HeavenlyStem::Geng),
        "辛" | "Xin" | "xin" => Ok(HeavenlyStem::Xin),
        "壬" | "Ren" | "ren" => Ok(HeavenlyStem::Ren),
        "癸" | "Gui" | "gui" => Ok(HeavenlyStem::Gui),
        _ => Err(()),
    }
}
