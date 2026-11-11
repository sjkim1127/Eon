//! Zi Wei Dou Shu Destiny Patterns detection module.
//!
//! Evaluates the three-party and four-direction alignments (Life, Travel, Wealth, Career)
//! and detects traditional auspicious and challenging destiny formations.

use crate::types::{DestinyPattern, PalaceData, SiHuaType, ZwdsStar};
use std::collections::HashMap;

/// Analyze three-party and four-direction to return detected destiny patterns.
pub fn analyze_destiny_patterns(
    soul_idx: usize,
    star_positions: &HashMap<ZwdsStar, usize>,
    palaces: &[PalaceData; 12],
) -> Vec<DestinyPattern> {
    let mut patterns = Vec::new();

    // Identify three-party and four-direction (Life, Travel, Wealth, Career)
    // ZWDS coordinate system: 0=Yin, 1=Mao, ..., 11=Chou (counter-clockwise)
    // Ming = soul_idx
    // Caibo = (soul_idx + 4) % 12 (Wealth)
    // Qianyi = (soul_idx + 6) % 12 (Travel / Opposite)
    // Guanlu = (soul_idx + 8) % 12 (Career)
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

    // 1.  (紫府朝垣格)
    // : (ZiWei) (TianFu) (, , , )  ,   (Ming)   .
    if has_star_in_three_four(ZwdsStar::ZiWei) && has_star_in_three_four(ZwdsStar::TianFu) {
        let both_in_ming = has_star_in_palace(ZwdsStar::ZiWei, ming_idx)
            && has_star_in_palace(ZwdsStar::TianFu, ming_idx);
        if !both_in_ming {
            patterns.push(DestinyPattern {
                name_hanja: "紫府朝垣".to_string(),
                name_korean: "Zi Fu Chao Yuan (Emperor & Treasury Alignment)".to_string(),
                name_english: "Zi Fu Chao Yuan (Emperor & Treasury Alignment)".to_string(),
                is_auspicious: true,
                description_korean: "The Emperor (Ziwei) and Treasury (Tianfu) both shine on the Life Palace from the three-party and four-direction, indicating lifelong wealth and high social status.".to_string(),
                description_english: "The Emperor (Ziwei) and Treasury (Tianfu) both shine on the Life Palace from the three-party and four-direction, indicating lifelong wealth and high social status.".to_string(),
            });
        }
    }

    // 2.  (三奇加會格 / 三奇朝命格)
    // :   (HuaLu), (HuaQuan), (HuaKe)   .
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
            name_korean: "San Qi Jia Hui (Three Wonders Conjunction)".to_string(),
            name_english: "San Qi Jia Hui (Three Wonders Conjunction)".to_string(),
            is_auspicious: true,
            description_korean: "The three transformations Hua Lu, Hua Quan, and Hua Ke meet in the three-party and four-direction, leading to outstanding achievements, authority, and prosperity.".to_string(),
            description_english: "The three transformations Hua Lu, Hua Quan, and Hua Ke meet in the three-party and four-direction, leading to outstanding achievements, authority, and prosperity.".to_string(),
        });
    }

    // 3.  (陽梁昌祿格)
    // : (TaiYang), (TianLiang), (WenChang), (LuCun)  (HuaLu)    .
    if has_star_in_three_four(ZwdsStar::TaiYang)
        && has_star_in_three_four(ZwdsStar::TianLiang)
        && has_star_in_three_four(ZwdsStar::WenChang)
    {
        let has_lu_or_lucun = has_star_in_three_four(ZwdsStar::LuCun) || has_lu;
        if has_lu_or_lucun {
            patterns.push(DestinyPattern {
                name_hanja: "陽梁昌祿".to_string(),
                name_korean: "Yang Liang Chang Lu (Academic Excellence & Civil Honor)".to_string(),
                name_english: "Yang Liang Chang Lu (Academic Excellence & Civil Honor)".to_string(),
                is_auspicious: true,
                description_korean: "The Sun (Taiyang), Blessing (Tianliang), Intellect (Wenchang), and wealth (Lucun/Hua Lu) combine in the three-party and four-direction, indicating exceptional academic success and official honors.".to_string(),
                description_english: "The Sun (Taiyang), Blessing (Tianliang), Intellect (Wenchang), and wealth (Lucun/Hua Lu) combine in the three-party and four-direction, indicating exceptional academic success and official honors.".to_string(),
            });
        }
    }

    // 4.  (極向離明格)
    // : (ZiWei) 午宮(index 4)  ,  午宮 (Ming Palace) .
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
                name_korean: "Ji Xiang Li Ming (Imperial Radiance Facing South)".to_string(),
                name_english: "Ji Xiang Li Ming (Imperial Radiance Facing South)".to_string(),
                is_auspicious: true,
                description_korean: "The Emperor (Ziwei) sits alone in the South (Wu Palace) as the Life Palace, symbolizing a king in his high court, representing strong leadership and noble status.".to_string(),
                description_english: "The Emperor (Ziwei) sits alone in the South (Wu Palace) as the Life Palace, symbolizing a king in his high court, representing strong leadership and noble status.".to_string(),
            });
        }
    }

    // 5.  (石中隱玉格)
    // : (JuMen) 子宮(index 10)  午宮(index 4)   .
    if (ming_idx == 10 || ming_idx == 4) && has_star_in_palace(ZwdsStar::JuMen, ming_idx) {
        patterns.push(DestinyPattern {
            name_hanja: "石中隱玉".to_string(),
            name_korean: "Shi Zhong Yin Yu (Jade Hidden in Stone)".to_string(),
            name_english: "Shi Zhong Yin Yu (Jade Hidden in Stone)".to_string(),
            is_auspicious: true,
            description_korean: "The Gate (Jumen) resides in the Child (Zi) or Sun (Wu) Palace as the Life Palace. Like a jade hidden inside a stone, talents are revealed over time, leading to great success.".to_string(),
            description_english: "The Gate (Jumen) resides in the Child (Zi) or Sun (Wu) Palace as the Life Palace. Like a jade hidden inside a stone, talents are revealed over time, leading to great success.".to_string(),
        });
    }

    // 6.  (殺拱劫照格)
    // :   (QiSha) , (DiKong) (DiJie)        .
    if has_star_in_three_four(ZwdsStar::QiSha)
        && (has_star_in_three_four(ZwdsStar::DiKong) || has_star_in_three_four(ZwdsStar::DiJie))
    {
        patterns.push(DestinyPattern {
            name_hanja: "殺拱劫照".to_string(),
            name_korean: "Sha Gong Jie Zhao (Destruction & Calamity Clash)".to_string(),
            name_english: "Sha Gong Jie Zhao (Destruction & Calamity Clash)".to_string(),
            is_auspicious: false,
            description_korean: "The unstable General (Qisha) meets Void (Dikong) or Robbery (Dijie) in the three-party and four-direction, indicating volatile fortune, sudden drops, and life challenges.".to_string(),
            description_english: "The unstable General (Qisha) meets Void (Dikong) or Robbery (Dijie) in the three-party and four-direction, indicating volatile fortune, sudden drops, and life challenges.".to_string(),
        });
    }

    // 7.  (機月同梁格)
    // :  ///   ,       .
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
            name_korean: "Ji Yue Tong Liang (Civil Bureaucrat & Strategic Assembly)".to_string(),
            name_english: "Ji Yue Tong Liang (Civil Bureaucrat & Strategic Assembly)".to_string(),
            is_auspicious: true,
            description_korean: "Plan (Tianji), Moon (Taiyin), Harmony (Tiantong), and Blessing (Tianliang) gather in the three-party, representing a highly successful administrative or advisory career with stable growth.".to_string(),
            description_english: "Plan (Tianji), Moon (Taiyin), Harmony (Tiantong), and Blessing (Tianliang) gather in the three-party, representing a highly successful administrative or advisory career with stable growth.".to_string(),
        });
    }

    // 8.  (紫府廉武相格)
    // :  ////   ,       .
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
            name_korean: "Zi Fu Lian Wu Xiang (Leadership Sovereign Coalition)".to_string(),
            name_english: "Zi Fu Lian Wu Xiang (Leadership Sovereign Coalition)".to_string(),
            is_auspicious: true,
            description_korean: "The Emperor, Treasury, Judge, Soldier, and Minister align in the three-party, generating high-class status, leadership potential, and outstanding material wealth.".to_string(),
            description_english: "The Emperor, Treasury, Judge, Soldier, and Minister align in the three-party, generating high-class status, leadership potential, and outstanding material wealth.".to_string(),
        });
    }

    // 9.  (巨日同宮格)
    // :   寅(0) 申(6)    .
    if (ming_idx == 0 || ming_idx == 6)
        && has_star_in_palace(ZwdsStar::TaiYang, ming_idx)
        && has_star_in_palace(ZwdsStar::JuMen, ming_idx)
    {
        patterns.push(DestinyPattern {
            name_hanja: "巨日同宮".to_string(),
            name_korean: "Ju Ri Tong Gong (Sun & Gate Conjunction)".to_string(),
            name_english: "Ju Ri Tong Gong (Sun & Gate Conjunction)".to_string(),
            is_auspicious: true,
            description_korean: "The Sun and Gate occupy the same Palace (Yin/Shen) as the Life Palace, scattering dark clouds with bright light, symbolizing great success in communication, diplomacy, or academic fields.".to_string(),
            description_english: "The Sun and Gate occupy the same Palace (Yin/Shen) as the Life Palace, scattering dark clouds with bright light, symbolizing great success in communication, diplomacy, or academic fields.".to_string(),
        });
    }

    // 10.  (日月同宮格)
    // :   丑(11) 未(5)    .
    if (ming_idx == 5 || ming_idx == 11)
        && has_star_in_palace(ZwdsStar::TaiYang, ming_idx)
        && has_star_in_palace(ZwdsStar::TaiYin, ming_idx)
    {
        patterns.push(DestinyPattern {
            name_hanja: "日月同宮".to_string(),
            name_korean: "Ri Yue Tong Gong (Sun & Moon Conjunction)".to_string(),
            name_english: "Ri Yue Tong Gong (Sun & Moon Conjunction)".to_string(),
            is_auspicious: true,
            description_korean: "The Sun and Moon occupy the same Palace (Chou/Wei) as the Life Palace, granting deep wisdom, versatile talents, and steady success in official or scholarly careers.".to_string(),
            description_english: "The Sun and Moon occupy the same Palace (Chou/Wei) as the Life Palace, granting deep wisdom, versatile talents, and steady success in official or scholarly careers.".to_string(),
        });
    }

    // 11.  (日月夾命格)
    // :           .
    let adj1 = (ming_idx + 1) % 12;
    let adj2 = (ming_idx + 11) % 12;
    let has_sun_yin_adj = (has_star_in_palace(ZwdsStar::TaiYang, adj1)
        && has_star_in_palace(ZwdsStar::TaiYin, adj2))
        || (has_star_in_palace(ZwdsStar::TaiYang, adj2)
            && has_star_in_palace(ZwdsStar::TaiYin, adj1));
    if has_sun_yin_adj {
        patterns.push(DestinyPattern {
            name_hanja: "日月夾命".to_string(),
            name_korean: "Ri Yue Jia Ming (Sun & Moon Flanking Life Palace)".to_string(),
            name_english: "Ri Yue Jia Ming (Sun & Moon Flanking Life Palace)".to_string(),
            is_auspicious: true,
            description_korean: "The Sun and Moon sandwich the Life Palace, bestowing light and support, resulting in strong helpers, high prestige, and an exceptionally smooth career path.".to_string(),
            description_english: "The Sun and Moon sandwich the Life Palace, bestowing light and support, resulting in strong helpers, high prestige, and an exceptionally smooth career path.".to_string(),
        });
    }

    // 12.  (雙祿朝垣格)
    // :     ,   (Ming)   .
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
            name_korean: "Shuang Lu Chao Yuan (Dual Wealth Stars Alignment)".to_string(),
            name_english: "Shuang Lu Chao Yuan (Dual Wealth Stars Alignment)".to_string(),
            is_auspicious: true,
            description_korean: "Lucun and Hua Lu both shine on the Life Palace from the three-party (excluding the Life Palace itself), bringing abundant financial fortunes and successful self-made wealth.".to_string(),
            description_english: "Lucun and Hua Lu both shine on the Life Palace from the three-party (excluding the Life Palace itself), bringing abundant financial fortunes and successful self-made wealth.".to_string(),
        });
    }

    // 13.  (百官朝拱格)
    // :   , /////  4     .
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
                name_korean: "Bai Guan Chao Gong (Hundred Officials Homage)".to_string(),
                name_english: "Bai Guan Chao Gong (Hundred Officials Homage)".to_string(),
                is_auspicious: true,
                description_korean: "The Emperor (Ziwei) sits in the Life Palace surrounded by at least 4 major assistant stars (ministers), signifying great authority, social status, and supreme support.".to_string(),
                description_english: "The Emperor (Ziwei) sits in the Life Palace surrounded by at least 4 major assistant stars (ministers), signifying great authority, social status, and supreme support.".to_string(),
            });
        }
    }

    // 14.  (鈴昌陀武格)
    // : (LingXing), (WenChang), (TuoLuo), (WuQu)    .
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
            name_korean: "Ling Chang Tuo Wu (Catastrophic Collision Formation)".to_string(),
            name_english: "Ling Chang Tuo Wu (Catastrophic Collision Formation)".to_string(),
            is_auspicious: false,
            description_korean: "Wuqu, Wenchang, Tuoluo, and Lingxing gather in the three-party, representing a risky astrological configuration that warns against sudden downfalls, legal issues, or financial collapses.".to_string(),
            description_english: "Wuqu, Wenchang, Tuoluo, and Lingxing gather in the three-party, representing a risky astrological configuration that warns against sudden downfalls, legal issues, or financial collapses.".to_string(),
        });
    }

    // 15.  (刑忌夾印格)
    // : (TianXiang, 印) ,     (  ) ( /)   .
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
                name_korean: "Xing Ji Jia Yin (Seal Trapped by Punishment & Clash)".to_string(),
                name_english: "Xing Ji Jia Yin (Seal Trapped by Punishment & Clash)".to_string(),
                is_auspicious: false,
                description_korean: "The Minister (Tianxiang) in the Life Palace is sandwiched by punishment (Xing/Qingyang) and jealousy (Ji), warning of legal disputes, document traps, or unexpected betrayals.".to_string(),
                description_english: "The Minister (Tianxiang) in the Life Palace is sandwiched by punishment (Xing/Qingyang) and jealousy (Ji), warning of legal disputes, document traps, or unexpected betrayals.".to_string(),
            });
        }
    }

    // 16.  (馬頭帶劍格)
    // :  午宮(4) ,       .
    if ming_idx == 4
        && has_star_in_palace(ZwdsStar::QingYang, 4)
        && (has_star_in_palace(ZwdsStar::TianTong, 4) || has_star_in_palace(ZwdsStar::TaiYin, 4))
    {
        patterns.push(DestinyPattern {
            name_hanja: "馬頭帶劍".to_string(),
            name_korean: "Ma Tou Dai Jian (Horse Head Carrying Sword)".to_string(),
            name_english: "Ma Tou Dai Jian (Horse Head Carrying Sword)".to_string(),
            is_auspicious: false, // Challenging pattern, formidable hero in turbulent times
            description_korean: "Qingyang resides in the Wu Palace (South) as the Life Palace with Tiantong or Taiyin, representing a high-risk but high-reward destiny of military authority or major breakthroughs through hardships.".to_string(),
            description_english: "Qingyang resides in the Wu Palace (South) as the Life Palace with Tiantong or Taiyin, representing a high-risk but high-reward destiny of military authority or major breakthroughs through hardships.".to_string(),
        });
    }

    // 17.  (泛水桃花格)
    // : (TanLang) 子宮(10)   .
    if ming_idx == 10 && has_star_in_palace(ZwdsStar::TanLang, 10) {
        patterns.push(DestinyPattern {
            name_hanja: "泛水桃花".to_string(),
            name_korean: "Fan Shui Tao Hua (Peach Blossom Floating on Water)".to_string(),
            name_english: "Fan Shui Tao Hua (Peach Blossom Floating on Water)".to_string(),
            is_auspicious: false,
            description_korean: "The Wolf (Tanlang) resides in the Child (Zi Palace - Water) as the Life Palace, amplifying natural charm and emotional entanglements, which requires self-discipline to prevent personal scanadals.".to_string(),
            description_english: "The Wolf (Tanlang) resides in the Child (Zi Palace - Water) as the Life Palace, amplifying natural charm and emotional entanglements, which requires self-discipline to prevent personal scanadals.".to_string(),
        });
    }

    // 18.  (羊陀夾忌格)
    // : (HuaJi)      (left/right) (QingYang) (TuoLuo)  .
    let left_of_ming = (ming_idx + 11) % 12;
    let right_of_ming = (ming_idx + 1) % 12;
    let ming_has_huaji = palaces[ming_idx]
        .stars
        .iter()
        .any(|st| st.si_hua == Some(SiHuaType::HuaJi));
    if ming_has_huaji
        && ((has_star_in_palace(ZwdsStar::QingYang, left_of_ming)
            && has_star_in_palace(ZwdsStar::TuoLuo, right_of_ming))
            || (has_star_in_palace(ZwdsStar::TuoLuo, left_of_ming)
                && has_star_in_palace(ZwdsStar::QingYang, right_of_ming)))
    {
        patterns.push(DestinyPattern {
            name_hanja: "羊陀夾忌".to_string(),
            name_korean: "Yang Tuo Jia Ji (Blade & Armor Trapping Clash)".to_string(),
            name_english: "Yang Tuo Jia Ji (Blade & Armor Trapping Clash)".to_string(),
            is_auspicious: false,
            description_korean: "The Life Palace with HuaJi is sandwiched between Qingyang and Tuoluo, creating an oppressive environment that requires defensive strategy and inner resilience.".to_string(),
            description_english: "The Life Palace with HuaJi is sandwiched between Qingyang and Tuoluo, creating an oppressive environment that requires defensive strategy and inner resilience.".to_string(),
        });
    }

    // 19.  (火鈴夾命格)
    if (has_star_in_palace(ZwdsStar::HuoXing, left_of_ming)
        && has_star_in_palace(ZwdsStar::LingXing, right_of_ming))
        || (has_star_in_palace(ZwdsStar::LingXing, left_of_ming)
            && has_star_in_palace(ZwdsStar::HuoXing, right_of_ming))
    {
        patterns.push(DestinyPattern {
            name_hanja: "火鈴夾命".to_string(),
            name_korean: "Huo Ling Jia Ming (Fiery Stars Flanking Life Palace)".to_string(),
            name_english: "Huo Ling Jia Ming (Fiery Stars Flanking Life Palace)".to_string(),
            is_auspicious: false,
            description_korean: "The Life Palace is sandwiched by Huoxing and Lingxing, indicating environmental friction and abrupt shifts requiring emotional control.".to_string(),
            description_english: "The Life Palace is sandwiched by Huoxing and Lingxing, indicating environmental friction and abrupt shifts requiring emotional control.".to_string(),
        });
    }

    // 20.  (空劫夾命格)
    if (has_star_in_palace(ZwdsStar::DiKong, left_of_ming)
        && has_star_in_palace(ZwdsStar::DiJie, right_of_ming))
        || (has_star_in_palace(ZwdsStar::DiJie, left_of_ming)
            && has_star_in_palace(ZwdsStar::DiKong, right_of_ming))
    {
        patterns.push(DestinyPattern {
            name_hanja: "空劫夾命".to_string(),
            name_korean: "Kong Jie Jia Ming (Void Stars Flanking Life Palace)".to_string(),
            name_english: "Kong Jie Jia Ming (Void Stars Flanking Life Palace)".to_string(),
            is_auspicious: false,
            description_korean: "The Life Palace is flanked by Dikong and Dijie, fostering spiritual depth and artistic or technical genius despite worldly fluctuations.".to_string(),
            description_english: "The Life Palace is flanked by Dikong and Dijie, fostering spiritual depth and artistic or technical genius despite worldly fluctuations.".to_string(),
        });
    }

    // 21.  (雙祿守命格)
    let ming_has_lu = palaces[ming_idx]
        .stars
        .iter()
        .any(|st| st.si_hua == Some(SiHuaType::HuaLu) || st.star == ZwdsStar::LuCun);
    let qianyi_has_lu = palaces[qianyi_idx]
        .stars
        .iter()
        .any(|st| st.si_hua == Some(SiHuaType::HuaLu) || st.star == ZwdsStar::LuCun);
    if ming_has_lu && qianyi_has_lu {
        patterns.push(DestinyPattern {
            name_hanja: "雙祿守命".to_string(),
            name_korean: "Shuang Lu Shou Ming (Dual Wealth Anchoring Life Palace)".to_string(),
            name_english: "Shuang Lu Shou Ming (Dual Wealth Anchoring Life Palace)".to_string(),
            is_auspicious: true,
            description_korean: "HuaLu and LuCun align across the Life and Travel Palaces, indicating exceptional wealth luck and financial harmony.".to_string(),
            description_english: "HuaLu and LuCun align across the Life and Travel Palaces, indicating exceptional wealth luck and financial harmony.".to_string(),
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
        // :  (index 4)    .
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
        // :   /  .
        // =0(寅),  = 0, 6(), 4(), 8()
        let soul_idx = 0;
        let mut star_positions = HashMap::new();
        star_positions.insert(ZwdsStar::QiSha, 6); // QiSha in opposite palace
        star_positions.insert(ZwdsStar::DiKong, 4); // DiKong in wealth palace

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
        let soul_idx = 0;
        let mut star_positions = HashMap::new();
        star_positions.insert(ZwdsStar::TianJi, 0); // TianJi in Ming
        star_positions.insert(ZwdsStar::TaiYin, 6); // TaiYin in opposite
        star_positions.insert(ZwdsStar::TianTong, 4); // TianTong in wealth
        star_positions.insert(ZwdsStar::TianLiang, 8); // TianLiang in career

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
        let soul_idx = 0;
        let mut star_positions = HashMap::new();
        star_positions.insert(ZwdsStar::ZiWei, 0); // ZiWei in Ming
        star_positions.insert(ZwdsStar::TianFu, 6); // TianFu in opposite
        star_positions.insert(ZwdsStar::LianZhen, 4); // LianZhen in wealth
        star_positions.insert(ZwdsStar::WuQu, 8); // WuQu in career
        star_positions.insert(ZwdsStar::TianXiang, 0); // TianXiang in Ming

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
        // :  寅(0)     .
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
        // :  子(10)    .
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
