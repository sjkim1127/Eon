//! 궁간 자화(自化), 3중 사화 충국(Triple Sihua Collision), 차성기궁(Borrowed Stars) 분석 연산기

use crate::transformations::get_sihua_stars;
use crate::types::{
    BorrowedStarsInfo, DaXian, LiuNian, PalaceData, SiHuaType, TripleSihuaCollision, ZiHuaInfo,
    ZwdsStar,
};
use eon_saju::core::stem::HeavenlyStem;

/// 각 궁의 천간(宮幹)에 의해 자궁 내 주성에 발생하는 자화(自化)를 검출합니다.
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
                        "{}에서 궁간({}) 자화({}) 발생 — {} 별이 자화됨",
                        p.name.korean(),
                        p.heavenly_stem,
                        s_type.korean(),
                        s_star.korean()
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

/// 주성이 없는 공궁(空宮)일 때 대궁(對宮)의 주성을 빌려오는 차성기궁(借星騎宮)을 검출합니다.
pub fn detect_borrowed_stars(palaces: &[PalaceData; 12]) -> Vec<BorrowedStarsInfo> {
    let mut results = Vec::new();

    for p in palaces {
        // 14 주성 존재 여부 체크
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

/// 생년 사화 + 대한 사화 + 유년 사화 간의 3중 충국 및 상충 패턴을 감지합니다.
pub fn detect_triple_sihua_collisions(
    palaces: &[PalaceData; 12],
    daxian_list: &[DaXian],
    current_liunian: Option<&LiuNian>,
) -> Vec<TripleSihuaCollision> {
    let mut collisions = Vec::new();

    if daxian_list.is_empty() {
        return collisions;
    }

    // 대표 대한 사화 추출 (첫번째 대한 기준 또는 지정 대한)
    let active_daxian = &daxian_list[0];
    let daxian_stem = parse_stem(&active_daxian.stem_hanja).ok();

    for p in palaces {
        let mut huaji_count = 0;
        let mut hualu_count = 0;
        let mut stars_involved = Vec::new();

        // 1. 생년 사화 체크
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

        // 2. 대한 사화 체크
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

        // 3. 유년 사화 체크
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
                collision_type: "삼화기 (三化忌 — 3중 화기 중첩)".to_string(),
                severity: "Crisis".to_string(),
                description: format!(
                    "{}에 3중 화기(化忌)가 집결하여 극심한 변화 및 과제가 집중되는 연도/시기입니다.",
                    p.name.korean()
                ),
                stars_involved: stars_involved.clone(),
            });
        } else if huaji_count >= 2 {
            collisions.push(TripleSihuaCollision {
                palace_idx: p.index,
                palace_name: p.name,
                collision_type: "쌍화기 (雙化忌 — 2중 화기 중첩)".to_string(),
                severity: "Caution".to_string(),
                description: format!(
                    "{}에 2중 화기(化忌)가 중첩하여 주의와 세심한 관리가 요구되는 시기입니다.",
                    p.name.korean()
                ),
                stars_involved: stars_involved.clone(),
            });
        }

        if hualu_count >= 2 {
            collisions.push(TripleSihuaCollision {
                palace_idx: p.index,
                palace_name: p.name,
                collision_type: "쌍록 (雙祿 / 祿轉成福 — 2중 화록 중첩)".to_string(),
                severity: "Opportunity".to_string(),
                description: format!(
                    "{}에 2중 화록(化祿)이 중첩하여 대단한 재물과 기회가 함께 도래하는 길시입니다.",
                    p.name.korean()
                ),
                stars_involved: stars_involved.clone(),
            });
        } else if hualu_count >= 1 && huaji_count >= 1 {
            collisions.push(TripleSihuaCollision {
                palace_idx: p.index,
                palace_name: p.name,
                collision_type: "록기상충 (祿忌相沖 — 기회와 위험 교차)".to_string(),
                severity: "Caution".to_string(),
                description: format!(
                    "{}에 화록과 화기가 공존하여 큰 성과 뒤에 리스크 관리가 동시에 요구됩니다.",
                    p.name.korean()
                ),
                stars_involved,
            });
        }
    }

    collisions
}

fn parse_stem(s: &str) -> Result<HeavenlyStem, ()> {
    match s {
        "甲" | "갑" => Ok(HeavenlyStem::Jia),
        "乙" | "을" => Ok(HeavenlyStem::Yi),
        "丙" | "병" => Ok(HeavenlyStem::Bing),
        "丁" | "정" => Ok(HeavenlyStem::Ding),
        "戊" | "무" => Ok(HeavenlyStem::Wu),
        "己" | "기" => Ok(HeavenlyStem::Ji),
        "庚" | "경" => Ok(HeavenlyStem::Geng),
        "辛" | "신" => Ok(HeavenlyStem::Xin),
        "壬" | "임" => Ok(HeavenlyStem::Ren),
        "癸" | "계" => Ok(HeavenlyStem::Gui),
        _ => Err(()),
    }
}
