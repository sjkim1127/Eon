//! The four "twelve-god" star groups of Zi Wei Dou Shu (十二神煞).
//!
//! Each group occupies all twelve palaces, one god per palace:
//!
//! | Group | Starts at | Direction |
//! |-------|-----------|-----------|
//! | 長生十二神 | by five-element bureau (水二/土五 → 申, 木三 → 亥, 金四 → 巳, 火六 → 寅) | 陽男陰女 順行, 陰男陽女 逆行 |
//! | 博士十二神 | the palace of 祿存 | 陽男陰女 順行, 陰男陽女 逆行 |
//! | 將前十二神 | 將星 by year branch (寅午戌 → 午, 申子辰 → 子, 巳酉丑 → 酉, 亥卯未 → 卯) | always 順行 |
//! | 歲前十二神 | 歲建 in the palace of the year branch | always 順行 |
//!
//! 順行 means following the branch order (寅 → 卯 → 辰 ...), i.e. increasing [`PalaceIndex`].
//!
//! Sources: 《紫微斗數全書》 安長生十二神訣, 安博士十二神訣, 安將前諸星訣, 安歲前諸星訣; cross-checked
//! against iztro (`src/star/decorativeStar.ts`, `getchangsheng12`, `getBoShi12`, `getYearly12`).
//! Unlike the 14 major stars these gods are kept apart from [`crate::PalaceData::stars`]: they are a
//! per-palace annotation ([`PalaceTwelveGods`]) and never take part in formation or 四化 logic.

use crate::palace::{fix_index, std_idx_to_zwds_idx};
use crate::types::{FiveElementsClass, PalaceIndex};
use serde::{Deserialize, Serialize};

macro_rules! twelve_gods_enum {
    (
        $(#[$meta:meta])*
        $name:ident { $( $variant:ident => ($hanja:literal, $english:literal) ),+ $(,)? }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub enum $name { $( $variant ),+ }

        impl $name {
            /// The twelve gods in their canonical order (the order they are laid out along the palaces).
            pub const ALL: [$name; 12] = [ $( $name::$variant ),+ ];

            /// Chinese name.
            pub fn hanja(&self) -> &'static str {
                match self { $( Self::$variant => $hanja ),+ }
            }

            /// Pinyin / English name.
            pub fn english(&self) -> &'static str {
                match self { $( Self::$variant => $english ),+ }
            }

            /// Same as [`Self::english`] (the project convention for the `korean()` accessors).
            pub fn korean(&self) -> &'static str {
                self.english()
            }

            /// Position of this god in the canonical order (0 = first god of the group).
            pub fn order(&self) -> usize {
                Self::ALL.iter().position(|g| g == self).unwrap_or(0)
            }
        }
    };
}

twelve_gods_enum! {
    /// 長生十二神: the twelve life stages.
    ChangSheng {
        ChangSheng => ("長生", "Chang Sheng (Birth)"),
        MuYu => ("沐浴", "Mu Yu (Bathing)"),
        GuanDai => ("冠帶", "Guan Dai (Capping)"),
        LinGuan => ("臨官", "Lin Guan (Office)"),
        DiWang => ("帝旺", "Di Wang (Peak)"),
        Shuai => ("衰", "Shuai (Decline)"),
        Bing => ("病", "Bing (Sickness)"),
        Si => ("死", "Si (Death)"),
        Mu => ("墓", "Mu (Tomb)"),
        Jue => ("絕", "Jue (Extinction)"),
        Tai => ("胎", "Tai (Conception)"),
        Yang => ("養", "Yang (Nurture)")
    }
}

twelve_gods_enum! {
    /// 博士十二神: the twelve "scholar" gods, laid out from 祿存.
    BoShi {
        BoShi => ("博士", "Bo Shi (Scholar)"),
        LiShi => ("力士", "Li Shi (Strongman)"),
        QingLong => ("青龍", "Qing Long (Azure Dragon)"),
        XiaoHao => ("小耗", "Xiao Hao (Minor Loss)"),
        JiangJun => ("將軍", "Jiang Jun (General)"),
        ZouShu => ("奏書", "Zou Shu (Memorial)"),
        FeiLian => ("飛廉", "Fei Lian (Flying Lian)"),
        XiShen => ("喜神", "Xi Shen (Joy God)"),
        BingFu => ("病符", "Bing Fu (Sickness Talisman)"),
        DaHao => ("大耗", "Da Hao (Major Loss)"),
        FuBing => ("伏兵", "Fu Bing (Ambush)"),
        GuanFu => ("官府", "Guan Fu (Officialdom)")
    }
}

twelve_gods_enum! {
    /// 將前十二神: the twelve "before the general" gods, laid out from 將星 by year branch.
    JiangQian {
        JiangXing => ("將星", "Jiang Xing (General Star)"),
        PanAn => ("攀鞍", "Pan An (Mounting the Saddle)"),
        SuiYi => ("歲驛", "Sui Yi (Annual Post Horse)"),
        XiShen => ("息神", "Xi Shen (Resting God)"),
        HuaGai => ("華蓋", "Hua Gai (Canopy)"),
        JieSha => ("劫煞", "Jie Sha (Robbery Sha)"),
        ZaiSha => ("災煞", "Zai Sha (Calamity Sha)"),
        TianSha => ("天煞", "Tian Sha (Heaven Sha)"),
        ZhiBei => ("指背", "Zhi Bei (Pointing Back)"),
        XianChi => ("咸池", "Xian Chi (Salty Pool)"),
        YueSha => ("月煞", "Yue Sha (Moon Sha)"),
        WangShen => ("亡神", "Wang Shen (Lost Spirit)")
    }
}

twelve_gods_enum! {
    /// 歲前十二神: the twelve "before the year" gods, laid out from 歲建 (太歲).
    SuiQian {
        SuiJian => ("歲建", "Sui Jian (Tai Sui)"),
        HuiQi => ("晦氣", "Hui Qi (Gloom)"),
        SangMen => ("喪門", "Sang Men (Mourning Gate)"),
        GuanSuo => ("貫索", "Guan Suo (Shackles)"),
        GuanFu => ("官符", "Guan Fu (Official Writ)"),
        XiaoHao => ("小耗", "Xiao Hao (Minor Loss)"),
        DaHao => ("大耗", "Da Hao (Major Loss)"),
        LongDe => ("龍德", "Long De (Dragon Virtue)"),
        BaiHu => ("白虎", "Bai Hu (White Tiger)"),
        TianDe => ("天德", "Tian De (Heaven Virtue)"),
        DiaoKe => ("弔客", "Diao Ke (Condolence Guest)"),
        BingFu => ("病符", "Bing Fu (Sickness Talisman)")
    }
}

/// The four twelve-god groups as they fall on one palace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PalaceTwelveGods {
    /// 長生十二神
    pub chang_sheng: ChangSheng,
    /// 博士十二神
    pub bo_shi: BoShi,
    /// 將前十二神
    pub jiang_qian: JiangQian,
    /// 歲前十二神
    pub sui_qian: SuiQian,
}

/// Palace (ZWDS index, 寅 = 0) where 長生 starts for a five-element bureau.
///
/// 水二局·土五局長生在申, 木三局長生在亥, 金四局長生在巳, 火六局長生在寅.
pub fn chang_sheng_start(bureau: FiveElementsClass) -> PalaceIndex {
    match bureau {
        FiveElementsClass::Water2 | FiveElementsClass::Earth5 => 6, // 申
        FiveElementsClass::Wood3 => 9,                              // 亥
        FiveElementsClass::Metal4 => 3,                             // 巳
        FiveElementsClass::Fire6 => 0,                              // 寅
    }
}

/// Palace (ZWDS index) where 將星 sits for a year branch (standard index, 子 = 0).
///
/// 寅午戌年將星午, 申子辰年將星子, 巳酉丑年將星酉, 亥卯未年將星卯.
pub fn jiang_xing_start(year_branch_std_idx: usize) -> PalaceIndex {
    let branch_std = match year_branch_std_idx % 12 {
        2 | 6 | 10 => 6, // 寅午戌 → 午
        8 | 0 | 4 => 0,  // 申子辰 → 子
        5 | 9 | 1 => 9,  // 巳酉丑 → 酉
        _ => 3,          // 亥卯未 → 卯
    };
    std_idx_to_zwds_idx(branch_std)
}

/// Lay the four groups over the twelve palaces.
///
/// * `bureau` - five-element bureau (decides where 長生 starts)
/// * `forward` - `true` for 陽男·陰女 (see [`crate::decadal::is_forward_direction`])
/// * `lucun_idx` - palace of 祿存 (博士 starts there)
/// * `year_branch_std_idx` - birth-year branch, standard index (子 = 0)
///
/// The returned array is indexed by palace (ZWDS index, 寅 = 0).
pub fn place_twelve_gods(
    bureau: FiveElementsClass,
    forward: bool,
    lucun_idx: PalaceIndex,
    year_branch_std_idx: usize,
) -> [PalaceTwelveGods; 12] {
    let dir: i32 = if forward { 1 } else { -1 };
    let cs_start = chang_sheng_start(bureau) as i32;
    let bs_start = lucun_idx as i32;
    let jq_start = jiang_xing_start(year_branch_std_idx) as i32;
    let sq_start = std_idx_to_zwds_idx(year_branch_std_idx % 12) as i32;

    std::array::from_fn(|p| {
        let p = p as i32;
        // Offset (in palaces, following `dir`) from the starting palace of each directional group.
        let along = |start: i32| (dir * (p - start)).rem_euclid(12) as usize;
        // 將前 / 歲前 always follow the branch order.
        let fixed = |start: i32| fix_index(p - start);
        PalaceTwelveGods {
            chang_sheng: ChangSheng::ALL[along(cs_start)],
            bo_shi: BoShi::ALL[along(bs_start)],
            jiang_qian: JiangQian::ALL[fixed(jq_start)],
            sui_qian: SuiQian::ALL[fixed(sq_start)],
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Palace indices (ZWDS, 寅 = 0) for readable branch names.
    const YIN: usize = 0;
    const MAO: usize = 1;
    const CHEN: usize = 2;
    const SI: usize = 3;
    const WU: usize = 4;
    const WEI: usize = 5;
    const SHEN: usize = 6;
    const YOU: usize = 7;
    const XU: usize = 8;
    const HAI: usize = 9;
    const ZI: usize = 10;
    const CHOU: usize = 11;

    #[test]
    fn lexicon_is_complete_and_bilingual() {
        for g in ChangSheng::ALL {
            assert!(!g.hanja().is_empty() && !g.english().is_empty());
        }
        assert_eq!(ChangSheng::ALL.len(), 12);
        assert_eq!(BoShi::LiShi.hanja(), "力士");
        assert_eq!(JiangQian::JiangXing.hanja(), "將星");
        assert_eq!(SuiQian::SuiJian.hanja(), "歲建");
        assert_eq!(SuiQian::BingFu.order(), 11);
    }

    #[test]
    fn chang_sheng_start_by_bureau() {
        // 《紫微斗數全書》: 水二局土五局長生在申, 木三局長生在亥, 金四局長生在巳, 火六局長生在寅.
        assert_eq!(chang_sheng_start(FiveElementsClass::Water2), SHEN);
        assert_eq!(chang_sheng_start(FiveElementsClass::Earth5), SHEN);
        assert_eq!(chang_sheng_start(FiveElementsClass::Wood3), HAI);
        assert_eq!(chang_sheng_start(FiveElementsClass::Metal4), SI);
        assert_eq!(chang_sheng_start(FiveElementsClass::Fire6), YIN);
    }

    #[test]
    fn jiang_xing_start_by_year_branch() {
        // 寅午戌年將星午, 申子辰年將星子, 巳酉丑年將星酉, 亥卯未年將星卯.
        for std_idx in [2, 6, 10] {
            assert_eq!(jiang_xing_start(std_idx), WU);
        }
        for std_idx in [8, 0, 4] {
            assert_eq!(jiang_xing_start(std_idx), ZI);
        }
        for std_idx in [5, 9, 1] {
            assert_eq!(jiang_xing_start(std_idx), YOU);
        }
        for std_idx in [11, 3, 7] {
            assert_eq!(jiang_xing_start(std_idx), MAO);
        }
    }

    // ── Hand-checked reference charts ───────────────────────────────────────────
    // Every expected value below was derived by hand from the verse rules quoted in the module
    // docs (not by running the code), and the 將前 華蓋/咸池/劫煞 results were cross-checked against
    // the independent 華蓋/咸池/劫殺 placements in `stars::location::place_year_branch_stars`.

    /// Chart A: 庚午 year (陽), male → 順行, 火六局, 祿存 in 申.
    /// (E.g. 1990-05-15 10:30 in Seoul: Ming in 子.)
    #[test]
    fn reference_chart_yang_male_fire6() {
        let gods = place_twelve_gods(FiveElementsClass::Fire6, true, SHEN, 6 /* 午 */);

        // 長生: 寅 長生, 卯 沐浴, 辰 冠帶, 巳 臨官, 午 帝旺, 未 衰, 申 病, 酉 死, 戌 墓, 亥 絕, 子 胎, 丑 養
        let cs = [YIN, MAO, CHEN, SI, WU, WEI, SHEN, YOU, XU, HAI, ZI, CHOU];
        for (i, &palace) in cs.iter().enumerate() {
            assert_eq!(gods[palace].chang_sheng, ChangSheng::ALL[i], "長生 #{i}");
        }
        // 博士 from 祿存 (申), forward: 申 博士, 酉 力士, 戌 青龍, 亥 小耗, 子 將軍, 丑 奏書, 寅 飛廉,
        // 卯 喜神, 辰 病符, 巳 大耗, 午 伏兵, 未 官府
        assert_eq!(gods[SHEN].bo_shi, BoShi::BoShi);
        assert_eq!(gods[YOU].bo_shi, BoShi::LiShi);
        assert_eq!(gods[ZI].bo_shi, BoShi::JiangJun);
        assert_eq!(gods[YIN].bo_shi, BoShi::FeiLian);
        assert_eq!(gods[WEI].bo_shi, BoShi::GuanFu);
        // 將前 (午年 → 將星 午): 午 將星, 未 攀鞍, 申 歲驛, 酉 息神, 戌 華蓋, 亥 劫煞, 子 災煞,
        // 丑 天煞, 寅 指背, 卯 咸池, 辰 月煞, 巳 亡神
        assert_eq!(gods[WU].jiang_qian, JiangQian::JiangXing);
        assert_eq!(gods[XU].jiang_qian, JiangQian::HuaGai);
        assert_eq!(gods[HAI].jiang_qian, JiangQian::JieSha);
        assert_eq!(gods[MAO].jiang_qian, JiangQian::XianChi);
        assert_eq!(gods[SI].jiang_qian, JiangQian::WangShen);
        // 歲前 (午年 → 歲建 午): 午 歲建, 未 晦氣, 申 喪門, 酉 貫索, 戌 官符, 亥 小耗, 子 大耗,
        // 丑 龍德, 寅 白虎, 卯 天德, 辰 弔客, 巳 病符
        assert_eq!(gods[WU].sui_qian, SuiQian::SuiJian);
        assert_eq!(gods[HAI].sui_qian, SuiQian::XiaoHao);
        assert_eq!(gods[ZI].sui_qian, SuiQian::DaHao);
        assert_eq!(gods[YIN].sui_qian, SuiQian::BaiHu);
        assert_eq!(gods[SI].sui_qian, SuiQian::BingFu);
    }

    /// Chart B: 乙丑 year (陰), female → 順行, 土五局, 祿存 in 卯.
    #[test]
    fn reference_chart_yin_female_earth5() {
        let gods = place_twelve_gods(FiveElementsClass::Earth5, true, MAO, 1 /* 丑 */);

        // 長生 from 申: 申 長生, 酉 沐浴, 戌 冠帶, 亥 臨官, 子 帝旺, 丑 衰, 寅 病, 卯 死, 辰 墓,
        // 巳 絕, 午 胎, 未 養
        assert_eq!(gods[SHEN].chang_sheng, ChangSheng::ChangSheng);
        assert_eq!(gods[ZI].chang_sheng, ChangSheng::DiWang);
        assert_eq!(gods[MAO].chang_sheng, ChangSheng::Si);
        assert_eq!(gods[WEI].chang_sheng, ChangSheng::Yang);
        // 博士 from 祿存 (卯), forward: 卯 博士, 辰 力士, 巳 青龍, 午 小耗, 未 將軍, 申 奏書, 酉 飛廉,
        // 戌 喜神, 亥 病符, 子 大耗, 丑 伏兵, 寅 官府
        assert_eq!(gods[MAO].bo_shi, BoShi::BoShi);
        assert_eq!(gods[WU].bo_shi, BoShi::XiaoHao);
        assert_eq!(gods[HAI].bo_shi, BoShi::BingFu);
        assert_eq!(gods[YIN].bo_shi, BoShi::GuanFu);
        // 將前 (丑年, 巳酉丑 → 將星 酉): 酉 將星, 戌 攀鞍, 亥 歲驛, 子 息神, 丑 華蓋, 寅 劫煞,
        // 卯 災煞, 辰 天煞, 巳 指背, 午 咸池, 未 月煞, 申 亡神
        assert_eq!(gods[YOU].jiang_qian, JiangQian::JiangXing);
        assert_eq!(gods[CHOU].jiang_qian, JiangQian::HuaGai);
        assert_eq!(gods[YIN].jiang_qian, JiangQian::JieSha);
        assert_eq!(gods[WU].jiang_qian, JiangQian::XianChi);
        assert_eq!(gods[SHEN].jiang_qian, JiangQian::WangShen);
        // 歲前 (丑年 → 歲建 丑): 丑 歲建, 寅 晦氣, 卯 喪門, ... 子 病符
        assert_eq!(gods[CHOU].sui_qian, SuiQian::SuiJian);
        assert_eq!(gods[YIN].sui_qian, SuiQian::HuiQi);
        assert_eq!(gods[MAO].sui_qian, SuiQian::SangMen);
        assert_eq!(gods[ZI].sui_qian, SuiQian::BingFu);
    }

    /// Chart C: 甲子 year (陽), female → 逆行, 金四局, 祿存 in 寅.
    #[test]
    fn reference_chart_yang_female_metal4_reverse() {
        let gods = place_twelve_gods(FiveElementsClass::Metal4, false, YIN, 0 /* 子 */);

        // 長生 from 巳 running backwards: 巳 長生, 辰 沐浴, 卯 冠帶, 寅 臨官, 丑 帝旺, 子 衰, 亥 病,
        // 戌 死, 酉 墓, 申 絕, 未 胎, 午 養
        assert_eq!(gods[SI].chang_sheng, ChangSheng::ChangSheng);
        assert_eq!(gods[CHEN].chang_sheng, ChangSheng::MuYu);
        assert_eq!(gods[YIN].chang_sheng, ChangSheng::LinGuan);
        assert_eq!(gods[CHOU].chang_sheng, ChangSheng::DiWang);
        assert_eq!(gods[YOU].chang_sheng, ChangSheng::Mu);
        assert_eq!(gods[WU].chang_sheng, ChangSheng::Yang);
        // 博士 from 祿存 (寅) running backwards: 寅 博士, 丑 力士, 子 青龍, 亥 小耗, 戌 將軍, 酉 奏書,
        // 申 飛廉, 未 喜神, 午 病符, 巳 大耗, 辰 伏兵, 卯 官府
        assert_eq!(gods[YIN].bo_shi, BoShi::BoShi);
        assert_eq!(gods[CHOU].bo_shi, BoShi::LiShi);
        assert_eq!(gods[ZI].bo_shi, BoShi::QingLong);
        assert_eq!(gods[SHEN].bo_shi, BoShi::FeiLian);
        assert_eq!(gods[MAO].bo_shi, BoShi::GuanFu);
        // 將前 / 歲前 ignore the direction (子年, 申子辰 → 將星 子; 歲建 子):
        // 子 將星, 丑 攀鞍, 寅 歲驛, 卯 息神, 辰 華蓋, 巳 劫煞, ... 亥 亡神
        assert_eq!(gods[ZI].jiang_qian, JiangQian::JiangXing);
        assert_eq!(gods[CHOU].jiang_qian, JiangQian::PanAn);
        assert_eq!(gods[CHEN].jiang_qian, JiangQian::HuaGai);
        assert_eq!(gods[SI].jiang_qian, JiangQian::JieSha);
        assert_eq!(gods[YOU].jiang_qian, JiangQian::XianChi);
        assert_eq!(gods[HAI].jiang_qian, JiangQian::WangShen);
        assert_eq!(gods[ZI].sui_qian, SuiQian::SuiJian);
        assert_eq!(gods[CHOU].sui_qian, SuiQian::HuiQi);
        assert_eq!(gods[HAI].sui_qian, SuiQian::BingFu);
    }

    #[test]
    fn every_group_covers_all_twelve_palaces_once() {
        for forward in [true, false] {
            let gods = place_twelve_gods(FiveElementsClass::Wood3, forward, ZI, 5);
            for group in 0..4 {
                let mut seen = [false; 12];
                for g in &gods {
                    let order = match group {
                        0 => g.chang_sheng.order(),
                        1 => g.bo_shi.order(),
                        2 => g.jiang_qian.order(),
                        _ => g.sui_qian.order(),
                    };
                    assert!(!seen[order], "duplicate god in group {group}");
                    seen[order] = true;
                }
            }
        }
    }
}
