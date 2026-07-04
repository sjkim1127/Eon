//! 자미두수 격국(Destiny Patterns) 감지 모듈
//!
//! 명반의 삼방사정(본궁, 대궁, 재백궁, 관록궁)을 연산하고,
//! 주요 6대 전통 길흉 격국을 감지하여 반환합니다.

use crate::types::{DestinyPattern, PalaceData, SiHuaType, ZwdsStar};
use std::collections::HashMap;

/// 명궁의 삼방사정을 분석하여 길격/흉격 격국 리스트를 반환합니다.
pub fn analyze_destiny_patterns(
    soul_idx: usize,
    star_positions: &HashMap<ZwdsStar, usize>,
    palaces: &[PalaceData; 12],
) -> Vec<DestinyPattern> {
    let mut patterns = Vec::new();

    // 삼방사정 (본궁, 대궁/천이궁, 재백궁, 관록궁) 구하기
    // ZWDS 좌표계: 0=寅, 1=卯, ..., 11=丑 (반시계 방향)
    // Ming = soul_idx
    // Caibo = (soul_idx + 4) % 12 (재백)
    // Qianyi = (soul_idx + 6) % 12 (천이/대궁)
    // Guanlu = (soul_idx + 8) % 12 (관록)
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
        let both_in_ming = has_star_in_palace(ZwdsStar::ZiWei, ming_idx)
            && has_star_in_palace(ZwdsStar::TianFu, ming_idx);
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

    // 6. 살공겁조격 (殺拱劫照格)
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

    // 7. 기월동량격 (機月同梁格)
    // 조건: 명궁에 천기/태음/천동/천량 중 하나가 있고, 삼방사정에 이 네 별이 모두 모이는 격국.
    let is_ming_gi_wol_dong_ryang = [
        ZwdsStar::TianJi,
        ZwdsStar::TaiYin,
        ZwdsStar::TianTong,
        ZwdsStar::TianLiang,
    ]
    .iter()
    .any(|&s| has_star_in_palace(s, ming_idx));
    let has_all_four_gi_wol_dong_ryang = [
        ZwdsStar::TianJi,
        ZwdsStar::TaiYin,
        ZwdsStar::TianTong,
        ZwdsStar::TianLiang,
    ]
    .iter()
    .all(|&s| has_star_in_three_four(s));
    if is_ming_gi_wol_dong_ryang && has_all_four_gi_wol_dong_ryang {
        patterns.push(DestinyPattern {
            name_hanja: "機月同梁".to_string(),
            name_korean: "기월동량격".to_string(),
            is_auspicious: true,
            description_korean: "천기, 태음, 천동, 천량이 삼방사정에서 회조하여, 공직이나 학술, 대기업 등 안정적인 조직에서 기획력과 행정력으로 성공을 거두는 전형적인 샐러리맨/참모형 길격입니다.".to_string(),
            description_english: "Plan (Tianji), Moon (Taiyin), Harmony (Tiantong), and Blessing (Tianliang) gather in the three-party, representing a highly successful administrative or advisory career with stable growth.".to_string(),
        });
    }

    // 8. 자부염무상격 (紫府廉武相格)
    // 조건: 명궁에 자미/천부/염정/무곡/천상 중 하나가 있고, 삼방사정에 이 다섯 별이 모두 모이는 격국.
    let is_ming_zi_fu_lian_wu_xiang = [
        ZwdsStar::ZiWei,
        ZwdsStar::TianFu,
        ZwdsStar::LianZhen,
        ZwdsStar::WuQu,
        ZwdsStar::TianXiang,
    ]
    .iter()
    .any(|&s| has_star_in_palace(s, ming_idx));
    let has_all_five_zi_fu_lian_wu_xiang = [
        ZwdsStar::ZiWei,
        ZwdsStar::TianFu,
        ZwdsStar::LianZhen,
        ZwdsStar::WuQu,
        ZwdsStar::TianXiang,
    ]
    .iter()
    .all(|&s| has_star_in_three_four(s));
    if is_ming_zi_fu_lian_wu_xiang && has_all_five_zi_fu_lian_wu_xiang {
        patterns.push(DestinyPattern {
            name_hanja: "紫府廉武相".to_string(),
            name_korean: "자부염무상격".to_string(),
            is_auspicious: true,
            description_korean: "자미, 천부, 염정, 무곡, 천상이 삼방사정에서 강력한 세력을 이루어, 뛰어난 사회적 지위, 강력한 리더십, 풍부한 재정적 결실을 거두는 최상위 길격 중 하나입니다.".to_string(),
            description_english: "The Emperor, Treasury, Judge, Soldier, and Minister align in the three-party, generating high-class status, leadership potential, and outstanding material wealth.".to_string(),
        });
    }

    // 9. 거일동궁격 (巨日同宮格)
    // 조건: 태양과 거문이 寅궁(0)이나 申궁(6)에 동궁하며 명궁이 되는 격국.
    if (ming_idx == 0 || ming_idx == 6)
        && has_star_in_palace(ZwdsStar::TaiYang, ming_idx)
        && has_star_in_palace(ZwdsStar::JuMen, ming_idx)
    {
        patterns.push(DestinyPattern {
            name_hanja: "巨日同宮".to_string(),
            name_korean: "거일동궁격".to_string(),
            is_auspicious: true,
            description_korean: "태양의 양광이 거문의 어두움을 걷어내는 격국으로, 주로 언론, 학술, 교육, 외교 등 말과 재능을 펼치는 분야나 외국과의 교류에서 대성하는 길격입니다.".to_string(),
            description_english: "The Sun and Gate occupy the same Palace (Yin/Shen) as the Life Palace, scattering dark clouds with bright light, symbolizing great success in communication, diplomacy, or academic fields.".to_string(),
        });
    }

    // 10. 일월동궁격 (日月同宮格)
    // 조건: 태양과 태음이 丑궁(11)이나 未궁(5)에 동궁하며 명궁이 되는 격국.
    if (ming_idx == 5 || ming_idx == 11)
        && has_star_in_palace(ZwdsStar::TaiYang, ming_idx)
        && has_star_in_palace(ZwdsStar::TaiYin, ming_idx)
    {
        patterns.push(DestinyPattern {
            name_hanja: "日月同宮".to_string(),
            name_korean: "일월동궁격".to_string(),
            is_auspicious: true,
            description_korean: "태양(해)과 태음(달)이 동궁하여 명궁을 비추는 격국으로, 성격이 다재다능하고 지혜로우며 공직이나 연구 분야 등에서 점진적인 명예와 성취를 거둡니다.".to_string(),
            description_english: "The Sun and Moon occupy the same Palace (Chou/Wei) as the Life Palace, granting deep wisdom, versatile talents, and steady success in official or scholarly careers.".to_string(),
        });
    }

    // 11. 일월협명격 (日月夾命格)
    // 조건: 명궁의 양 옆 궁에 태양과 태음이 나란히 배치되어 명궁을 협하는 격국.
    let adj1 = (ming_idx + 1) % 12;
    let adj2 = (ming_idx + 11) % 12;
    let has_sun_yin_adj = (has_star_in_palace(ZwdsStar::TaiYang, adj1)
        && has_star_in_palace(ZwdsStar::TaiYin, adj2))
        || (has_star_in_palace(ZwdsStar::TaiYang, adj2)
            && has_star_in_palace(ZwdsStar::TaiYin, adj1));
    if has_sun_yin_adj {
        patterns.push(DestinyPattern {
            name_hanja: "日月夾命".to_string(),
            name_korean: "일월협명격".to_string(),
            is_auspicious: true,
            description_korean: "명궁이 태양과 태음 사이에 자리하여 양옆의 은혜로운 빛을 입는 격국으로, 귀인의 전폭적인 도움이나 상사의 후원을 받아 평생 귀한 신분으로 순조롭게 출세하는 길격입니다.".to_string(),
            description_english: "The Sun and Moon sandwich the Life Palace, bestowing light and support, resulting in strong helpers, high prestige, and an exceptionally smooth career path.".to_string(),
        });
    }

    // 12. 쌍록조원격 (雙祿朝垣格)
    // 조건: 록존과 화록이 명궁 삼방사정에 배치되나, 둘 다 명궁(Ming) 자체에는 없는 경우.
    let mut has_hua_lu_in_three_directions = false;
    for &p_idx in &[qianyi_idx, caibo_idx, guanlu_idx] {
        for star_in_p in &palaces[p_idx].stars {
            if star_in_p.si_hua == Some(SiHuaType::HuaLu) {
                has_hua_lu_in_three_directions = true;
            }
        }
    }
    let has_lucun_in_three_directions = [qianyi_idx, caibo_idx, guanlu_idx]
        .iter()
        .any(|&idx| has_star_in_palace(ZwdsStar::LuCun, idx));
    if has_hua_lu_in_three_directions && has_lucun_in_three_directions {
        patterns.push(DestinyPattern {
            name_hanja: "雙祿朝垣".to_string(),
            name_korean: "쌍록조원격".to_string(),
            is_auspicious: true,
            description_korean: "록존과 화록이 삼방사정에서 명궁을 든든하게 비추어 도와주는 격국으로, 일생 재무적 기회가 끊이지 않으며 자수성가하여 큰 부를 축적하는 대표적인 부유 격국입니다.".to_string(),
            description_english: "Lucun and Hua Lu both shine on the Life Palace from the three-party (excluding the Life Palace itself), bringing abundant financial fortunes and successful self-made wealth.".to_string(),
        });
    }

    // 13. 백관조공격 (百官朝拱格)
    // 조건: 명궁에 자미가 있고, 좌보/우필/천괴/천월/문창/문곡 중 4개 이상의 보좌성이 삼방사정에 있는 경우.
    if has_star_in_palace(ZwdsStar::ZiWei, ming_idx) {
        let assistants = [
            ZwdsStar::ZuoFu,
            ZwdsStar::YouBi,
            ZwdsStar::TianKui,
            ZwdsStar::TianYue,
            ZwdsStar::WenChang,
            ZwdsStar::WenQu,
        ];
        let assistant_count = assistants
            .iter()
            .filter(|&&s| has_star_in_three_four(s))
            .count();
        if assistant_count >= 4 {
            patterns.push(DestinyPattern {
                name_hanja: "百官朝拱".to_string(),
                name_korean: "백관조공격".to_string(),
                is_auspicious: true,
                description_korean: "제왕인 자미성이 명궁에 자리하고 여러 보좌길성(신하)들이 주위를 둘러싸고 옹위하는 형국으로, 막강한 권력, 강한 조력자, 높은 지위에 오르게 되는 제왕의 명조입니다.".to_string(),
                description_english: "The Emperor (Ziwei) sits in the Life Palace surrounded by at least 4 major assistant stars (ministers), signifying great authority, social status, and supreme support.".to_string(),
            });
        }
    }

    // 14. 령창타무격 (鈴昌陀武格)
    // 조건: 영성(LingXing), 문창(WenChang), 타라(TuoLuo), 무곡(WuQu)이 삼방사정에서 모두 모이는 흉격.
    let has_ling_chang_tuo_wu = [
        ZwdsStar::LingXing,
        ZwdsStar::WenChang,
        ZwdsStar::TuoLuo,
        ZwdsStar::WuQu,
    ]
    .iter()
    .all(|&s| has_star_in_three_four(s));
    if has_ling_chang_tuo_wu {
        patterns.push(DestinyPattern {
            name_hanja: "鈴昌陀武".to_string(),
            name_korean: "령창타무격".to_string(),
            is_auspicious: false,
            description_korean: "무곡의 금기운에 영성, 타라, 문창의 수살이 결합하여 갑작스러운 재난, 금융상의 대패, 혹은 중대한 실패를 암시하므로 일생 투자와 건강에 극도로 신중해야 하는 흉격입니다.".to_string(),
            description_english: "Wuqu, Wenchang, Tuoluo, and Lingxing gather in the three-party, representing a risky astrological configuration that warns against sudden downfalls, legal issues, or financial collapses.".to_string(),
        });
    }

    // 15. 형기협인격 (刑忌夾印格)
    // 조건: 천상(TianXiang, 印)이 명궁이고, 인접한 두 궁에 각각 화기(또는 거문화기 등)와 천형(또는 경양/타라)이 배치되어 협하는 흉격.
    if has_star_in_palace(ZwdsStar::TianXiang, ming_idx) {
        let adj_palaces = [(ming_idx + 1) % 12, (ming_idx + 11) % 12];
        let mut has_ji = false;
        let mut has_xing_or_yang = false;
        for &p_idx in &adj_palaces {
            for star_in_p in &palaces[p_idx].stars {
                if star_in_p.si_hua == Some(SiHuaType::HuaJi) {
                    has_ji = true;
                }
                if star_in_p.star == ZwdsStar::QingYang
                    || star_in_p.star == ZwdsStar::TuoLuo
                    || star_in_p.star == ZwdsStar::TianXing
                {
                    has_xing_or_yang = true;
                }
            }
        }
        if has_ji && has_xing_or_yang {
            patterns.push(DestinyPattern {
                name_hanja: "刑忌夾印".to_string(),
                name_korean: "형기협인격".to_string(),
                is_auspicious: false,
                description_korean: "관인(도장)인 천상성이 형성(경양/천형)과 기성(화기) 사이에 협여 억압받는 격국으로, 송사, 배신, 감금, 또는 예상치 못한 재난에 휘말리기 쉬우며 문서나 계약 시 각별한 주의가 필요합니다.".to_string(),
                description_english: "The Minister (Tianxiang) in the Life Palace is sandwiched by punishment (Xing/Qingyang) and jealousy (Ji), warning of legal disputes, document traps, or unexpected betrayals.".to_string(),
            });
        }
    }

    // 16. 마두대검격 (馬頭帶劍格)
    // 조건: 경양이 午宮(4)에 있고, 오궁이 명궁이면서 동시에 천동이나 태음과 동궁하는 격국.
    if ming_idx == 4
        && has_star_in_palace(ZwdsStar::QingYang, 4)
        && (has_star_in_palace(ZwdsStar::TianTong, 4) || has_star_in_palace(ZwdsStar::TaiYin, 4))
    {
        patterns.push(DestinyPattern {
            name_hanja: "馬頭帶劍".to_string(),
            name_korean: "마두대검격".to_string(),
            is_auspicious: false, // 흉격 성향이 강하나, 난세의 영웅격
            description_korean: "칼에 해당하는 경양이 남쪽 오궁에서 칼날을 치켜세우는 형국으로, 삶의 기복과 고난이 대단히 격렬하지만 군인, 경찰, 외과 의사 등 개척적인 특수 직업군에서 큰 권력을 장악하기도 합니다.".to_string(),
            description_english: "Qingyang resides in the Wu Palace (South) as the Life Palace with Tiantong or Taiyin, representing a high-risk but high-reward destiny of military authority or major breakthroughs through hardships.".to_string(),
        });
    }

    // 17. 범수도화격 (泛水桃花格)
    // 조건: 탐랑(TanLang)이 子宮(10)의 명궁에 있는 격국.
    if ming_idx == 10 && has_star_in_palace(ZwdsStar::TanLang, 10) {
        patterns.push(DestinyPattern {
            name_hanja: "泛水桃花".to_string(),
            name_korean: "범수도화격".to_string(),
            is_auspicious: false,
            description_korean: "도화의 별인 탐랑이 물(子궁)을 만나 도화 기운이 넘쳐흐르는 격국으로, 매력이 뛰어나 예체능이나 인기 업종에서 주목을 받을 수 있으나, 이성 관계나 주색으로 인한 낭패와 풍파를 경계해야 합니다.".to_string(),
            description_english: "The Wolf (Tanlang) resides in the Child (Zi Palace - Water) as the Life Palace, amplifying natural charm and emotional entanglements, which requires self-discipline to prevent personal scanadals.".to_string(),
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
        star_positions.insert(ZwdsStar::QiSha, 6); // 대궁에 칠살
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

    #[test]
    fn test_gi_wol_dong_ryang() {
        // 기월동량격: 명궁에 천기가 있고 삼방사정에 천기, 태음, 천동, 천량이 모두 있는 경우.
        let soul_idx = 0;
        let mut star_positions = HashMap::new();
        star_positions.insert(ZwdsStar::TianJi, 0); // 본궁에 천기
        star_positions.insert(ZwdsStar::TaiYin, 6); // 대궁에 태음
        star_positions.insert(ZwdsStar::TianTong, 4); // 재백에 천동
        star_positions.insert(ZwdsStar::TianLiang, 8); // 관록에 천량

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
        assert!(patterns.iter().any(|p| p.name_hanja == "機月同梁"));
    }

    #[test]
    fn test_zi_fu_lian_wu_xiang() {
        // 자부염무상격: 명궁에 자미가 있고 삼방사정에 자미, 천부, 염정, 무곡, 천상이 모두 있는 경우.
        let soul_idx = 0;
        let mut star_positions = HashMap::new();
        star_positions.insert(ZwdsStar::ZiWei, 0); // 본궁 자미
        star_positions.insert(ZwdsStar::TianFu, 6); // 대궁 천부
        star_positions.insert(ZwdsStar::LianZhen, 4); // 재백 염정
        star_positions.insert(ZwdsStar::WuQu, 8); // 관록 무곡
        star_positions.insert(ZwdsStar::TianXiang, 0); // 본궁 천상 (동궁 가능)

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
        assert!(patterns.iter().any(|p| p.name_hanja == "紫府廉武相"));
    }

    #[test]
    fn test_ge_ri_dong_gong() {
        // 거일동궁격: 명궁이 寅궁(0)이고 거문과 태양이 명궁에 동궁하는 경우.
        let soul_idx = 0;
        let mut star_positions = HashMap::new();
        star_positions.insert(ZwdsStar::JuMen, 0);
        star_positions.insert(ZwdsStar::TaiYang, 0);

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
        assert!(patterns.iter().any(|p| p.name_hanja == "巨日同宮"));
    }

    #[test]
    fn test_beom_su_dohwa() {
        // 범수도화격: 명궁이 子궁(10)이고 탐랑이 명궁에 있는 경우.
        let soul_idx = 10;
        let mut star_positions = HashMap::new();
        star_positions.insert(ZwdsStar::TanLang, 10);

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
        assert!(patterns.iter().any(|p| p.name_hanja == "泛水桃花"));
    }
}
