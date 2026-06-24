//! 자미두수 격국(Destiny Patterns) 감지 모듈
//!
//! 명반의 삼방사정(본궁, 대궁, 재백궁, 관록궁)을 연산하고, 
//! 주요 6대 전통 길흉 격국을 감지하여 반환합니다.

use std::collections::HashMap;
use crate::types::{DestinyPattern, PalaceData, ZwdsStar, SiHuaType};

/// 명궁의 삼방사정을 분석하여 길격/흉격 격국 리스트를 반환합니다.
pub fn analyze_destiny_patterns(
    soul_idx: usize,
    star_positions: &HashMap<ZwdsStar, usize>,
    palaces: &[PalaceData; 12],
) -> Vec<DestinyPattern> {
    let mut patterns = Vec::new();

    // 삼방사정 (본궁, 대궁/천이궁, 재백궁, 관록궁) 구하기
    // ZWDS 좌표계: 0=寅, 1=卯, ..., 11=丑 (반시계 방향)
    // Ming = 0 (soul_idx)
    // Caibo = 4 (재백)
    // Qianyi = 6 (천이)
    // Guanlu = 8 (관록)
    let ming_idx = soul_idx;
    let qianyi_idx = (soul_idx + 6) % 12;
    let caibo_idx = (soul_idx + 4) % 12;
    let guanlu_idx = (soul_idx + 8) % 12;

    let three_four_directions = [ming_idx, qianyi_idx, caibo_idx, guanlu_idx];

    // Helper to check if a star is in the three-party/four-direction
    let has_star_in_three_four = |star: ZwdsStar| -> bool {
        if let Some(&pos) = star_positions.get(&star) {
            three_four_directions.contains(&pos)
        } else {
            false
        }
    };

    // Helper to check if a star is in a specific palace index
    let has_star_in_palace = |star: ZwdsStar, palace_idx: usize| -> bool {
        star_positions.get(&star) == Some(&palace_idx)
    };

    // 1. 자부조원격 (紫府朝垣格)
    // 조건: 자미(ZiWei)와 천부(TianFu)가 삼방사정(명궁, 재백궁, 관록궁, 천이궁)에 모두 있으면서, 둘 다 명궁(Ming)에 있지 않은 경우.
    if has_star_in_three_four(ZwdsStar::ZiWei) && has_star_in_three_four(ZwdsStar::TianFu) {
        let both_in_ming = has_star_in_palace(ZwdsStar::ZiWei, ming_idx) && has_star_in_palace(ZwdsStar::TianFu, ming_idx);
        if !both_in_ming {
            patterns.push(DestinyPattern {
                name_hanja: "紫府朝垣".to_string(),
                name_korean: "자부조원격".to_string(),
                is_auspicious: true,
                description_korean: "명궁의 삼방사정에서 자미성과 천부성을 모두 비추어, 평생 물질적 풍요와 높은 명예를 누리는 대표적인 길격입니다.".to_string(),
                description_english: "The Emperor (Ziwei) and Treasury (Tianfu) both shine on the Life Palace from the three-party and four-direction, indicating lifelong wealth and high social status.".to_string(),
            });
        }
    }

    // 2. 삼기조명격 (三奇加會格 / 三奇朝命格)
    // 조건: 명궁의 삼방사정에서 화록(HuaLu), 화권(HuaQuan), 화과(HuaKe)를 모두 보는 경우.
    let mut has_lu = false;
    let mut has_quan = false;
    let mut has_ke = false;
    for &p_idx in &three_four_directions {
        for star_in_p in &palaces[p_idx].stars {
            if let Some(sihua) = star_in_p.si_hua {
                match sihua {
                    SiHuaType::HuaLu => has_lu = true,
                    SiHuaType::HuaQuan => has_quan = true,
                    SiHuaType::HuaKe => has_ke = true,
                    _ => {}
                }
            }
        }
    }
    if has_lu && has_quan && has_ke {
        patterns.push(DestinyPattern {
            name_hanja: "三奇加會".to_string(),
            name_korean: "삼기조명격".to_string(),
            is_auspicious: true,
            description_korean: "명궁의 삼방사정에서 화록, 화권, 화과를 모두 만나 학문적 성취, 권력, 재물을 크게 이루며 사회적으로 대성하는 길격입니다.".to_string(),
            description_english: "The three transformations Hua Lu, Hua Quan, and Hua Ke meet in the three-party and four-direction, leading to outstanding achievements, authority, and prosperity.".to_string(),
        });
    }

    // 3. 양량창록격 (陽梁昌祿格)
    // 조건: 태양(TaiYang), 천량(TianLiang), 문창(WenChang), 록존(LuCun) 또는 화록(HuaLu)이 삼방사정에서 모두 만나는 경우.
    if has_star_in_three_four(ZwdsStar::TaiYang)
        && has_star_in_three_four(ZwdsStar::TianLiang)
        && has_star_in_three_four(ZwdsStar::WenChang)
    {
        let has_lu_or_lucun = has_star_in_three_four(ZwdsStar::LuCun) || has_lu;
        if has_lu_or_lucun {
            patterns.push(DestinyPattern {
                name_hanja: "陽梁昌祿".to_string(),
                name_korean: "양량창록격".to_string(),
                is_auspicious: true,
                description_korean: "태양, 천량, 문창, 록존(혹은 화록)이 삼방사정에서 결합하여 시험에서 우수한 성적을 거두거나 학술적, 공직 분야에서 크게 이름을 떨치는 길격입니다.".to_string(),
                description_english: "The Sun (Taiyang), Blessing (Tianliang), Intellect (Wenchang), and wealth (Lucun/Hua Lu) combine in the three-party and four-direction, indicating exceptional academic success and official honors.".to_string(),
            });
        }
    }

    // 4. 극향리명격 (極向離明格)
    // 조건: 자미(ZiWei)가 午宮(index 4)에 단독으로 배치되고, 이 午宮이 명궁(Ming Palace)인 경우.
    if ming_idx == 4 && has_star_in_palace(ZwdsStar::ZiWei, 4) {
        let mut main_stars_in_wu = 0;
        for star_in_p in &palaces[4].stars {
            if star_in_p.star.is_main_star() {
                main_stars_in_wu += 1;
            }
        }
        if main_stars_in_wu == 1 {
            patterns.push(DestinyPattern {
                name_hanja: "極向離明".to_string(),
                name_korean: "극향리명격".to_string(),
                is_auspicious: true,
                description_korean: "자미성이 남쪽인 오궁(午宮)에서 제왕의 자리에 앉아 명궁이 되는 격국으로, 뛰어난 통솔력과 고귀한 지위를 얻게 되는 길격입니다.".to_string(),
                description_english: "The Emperor (Ziwei) sits alone in the South (Wu Palace) as the Life Palace, symbolizing a king in his high court, representing strong leadership and noble status.".to_string(),
            });
        }
    }

    // 5. 석중은옥격 (石中隱玉格)
    // 조건: 거문(JuMen)이 子宮(index 10) 또는 午宮(index 4)에 있으며 명궁인 경우.
    if (ming_idx == 10 || ming_idx == 4) && has_star_in_palace(ZwdsStar::JuMen, ming_idx) {
        patterns.push(DestinyPattern {
            name_hanja: "石中隱玉".to_string(),
            name_korean: "석중은옥격".to_string(),
            is_auspicious: true,
            description_korean: "거문성이 자궁이나 오궁의 명궁에 임하여 돌 속에 옥이 감추어져 있는 것처럼, 처음에는 고생을 겪으나 결국 재능을 발현하여 크게 성공하는 길격입니다.".to_string(),
            description_english: "The Gate (Jumen) resides in the Child (Zi) or Sun (Wu) Palace as the Life Palace. Like a jade hidden inside a stone, talents are revealed over time, leading to great success.".to_string(),
        });
    }

    // 6. 살공겁조격 (殺拱劫조格)
    // 조건: 명궁 삼방사정에 칠살(QiSha)이 있고, 지공(DiKong)이나 지겁(DiJie) 중 하나 이상이 명궁 혹은 삼방사정에 있는 경우.
    if has_star_in_three_four(ZwdsStar::QiSha)
        && (has_star_in_three_four(ZwdsStar::DiKong) || has_star_in_three_four(ZwdsStar::DiJie))
    {
        patterns.push(DestinyPattern {
            name_hanja: "殺拱劫照".to_string(),
            name_korean: "살공겁조격".to_string(),
            is_auspicious: false,
            description_korean: "칠살의 파괴력과 지공·지겁의 공허함이 결합하여, 재물의 기복이 크고 삶에 풍파와 급격한 하락을 겪기 쉬운 대표적인 흉격입니다.".to_string(),
            description_english: "The unstable General (Qisha) meets Void (Dikong) or Robbery (Dijie) in the three-party and four-direction, indicating volatile fortune, sudden drops, and life challenges.".to_string(),
        });
    }

    patterns
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::StarInPalace;

    #[test]
    fn test_ji_xiang_li_ming() {
        // 극향리명격: 자미가 오궁(index 4)에 있고 오궁이 명궁인 경우.
        let soul_idx = 4;
        let mut star_positions = HashMap::new();
        star_positions.insert(ZwdsStar::ZiWei, 4);

        let mut palaces = vec![];
        for i in 0..12 {
            let mut stars = vec![];
            if i == 4 {
                stars.push(StarInPalace {
                    star: ZwdsStar::ZiWei,
                    si_hua: None,
                    brightness: None,
                });
            }
            palaces.push(PalaceData {
                index: i,
                name: crate::types::PalaceName::Ming,
                heavenly_stem: "".to_string(),
                earthly_branch: "".to_string(),
                stars,
                daxian_range: None,
                is_current_liu_nian: false,
            });
        }
        let palaces_arr: [PalaceData; 12] = palaces.try_into().unwrap();

        let patterns = analyze_destiny_patterns(soul_idx, &star_positions, &palaces_arr);
        assert!(patterns.iter().any(|p| p.name_hanja == "極向離明"));
    }

    #[test]
    fn test_sal_gong_geob_jo() {
        // 살공겁조격: 삼방사정에 칠살과 지공/지겁이 있는 경우.
        // 명궁=0(寅), 삼방사정 = 0, 6(대궁), 4(재백), 8(관록)
        let soul_idx = 0;
        let mut star_positions = HashMap::new();
        star_positions.insert(ZwdsStar::QiSha, 6);  // 대궁에 칠살
        star_positions.insert(ZwdsStar::DiKong, 4); // 재백에 지공

        let mut palaces = vec![];
        for i in 0..12 {
            palaces.push(PalaceData {
                index: i,
                name: crate::types::PalaceName::Ming,
                heavenly_stem: "".to_string(),
                earthly_branch: "".to_string(),
                stars: vec![],
                daxian_range: None,
                is_current_liu_nian: false,
            });
        }
        let palaces_arr: [PalaceData; 12] = palaces.try_into().unwrap();

        let patterns = analyze_destiny_patterns(soul_idx, &star_positions, &palaces_arr);
        assert!(patterns.iter().any(|p| p.name_hanja == "殺拱劫照"));
    }
}
