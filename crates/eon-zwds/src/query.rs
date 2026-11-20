//! Public query API over a built [`ZwdsChart`]: 飛星四化 (flying transformations), 三方四正, 大限四化,
//! 小限 and 斗君.
//!
//! The natal [`ZwdsChart::flying_sihua`] list is a flat dump of all 48 flights; the methods here
//! answer targeted questions ("where does the 夫妻宮 stem send 化忌?", "which palaces fly into the
//! 命宮?", "what is in the 三方四正 of the 財帛宮?") and work for any stem, so the same machinery
//! serves 大限四化, 小限四化 and the 合盤 cross-chart lookups ([`crate::compat`]).

use crate::limits::{dou_jun_palace, xiaoxian_palace, DouJun, XiaoXian};
use crate::palace::{
    fix_index, get_palace_name, get_palace_stem, palace_index_of_name, zwds_idx_to_std_idx,
};
use crate::transformations::get_sihua_stars;
use crate::types::{
    DaXian, PalaceData, PalaceIndex, PalaceName, SiHuaType, StarInPalace, ZwdsChart, ZwdsStar,
};
use eon_saju::core::branch::EarthlyBranch;
use eon_saju::core::stem::HeavenlyStem;
use serde::{Deserialize, Serialize};

/// Where one transformation of a stem lands in a chart.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SiHuaLanding {
    pub sihua_type: SiHuaType,
    pub star: ZwdsStar,
    /// Palace (ZWDS index) holding the transformed star.
    pub palace_idx: PalaceIndex,
    /// Natal name of that palace.
    pub palace_name: PalaceName,
}

/// The four transformations flown out of one palace's stem.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PalaceSiHua {
    /// Source palace (ZWDS index).
    pub from_palace_idx: PalaceIndex,
    /// Natal name of the source palace.
    pub from_palace: PalaceName,
    /// Heavenly stem of the source palace.
    pub stem_hanja: String,
    /// 化祿, 化權, 化科, 化忌 landings (in that order).
    pub landings: Vec<SiHuaLanding>,
}

/// 三方四正 of a palace: itself, its opposite (對宮) and the two trine palaces (三合).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SanFangSiZheng {
    pub palace_idx: PalaceIndex,
    pub palace: PalaceName,
    /// Opposite palace (對宮, +6).
    pub opposite_idx: PalaceIndex,
    pub opposite: PalaceName,
    /// The two trine palaces (三合, ±4), in ascending palace-index order.
    pub trine_idx: [PalaceIndex; 2],
    pub trine: [PalaceName; 2],
}

impl SanFangSiZheng {
    /// The four palace indices: self, opposite, then the two trines.
    pub fn palace_indices(&self) -> [PalaceIndex; 4] {
        [
            self.palace_idx,
            self.opposite_idx,
            self.trine_idx[0],
            self.trine_idx[1],
        ]
    }
}

/// Stars of a palace inside a 三方四正 group.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SanFangStars {
    pub palace_idx: PalaceIndex,
    pub palace: PalaceName,
    pub stars: Vec<StarInPalace>,
}

impl ZwdsChart {
    /// Natal year stem (from the lunar birth year the chart was built from).
    pub fn year_stem(&self) -> HeavenlyStem {
        HeavenlyStem::from_index(self.birth_lunar.year_stem_idx as i32)
    }

    /// Heavenly stem of a palace (五虎遁 from the natal year stem).
    pub fn palace_stem(&self, palace_idx: PalaceIndex) -> HeavenlyStem {
        get_palace_stem(self.year_stem(), palace_idx % 12)
    }

    /// ZWDS index of the palace carrying `name`.
    pub fn palace_index_of(&self, name: PalaceName) -> PalaceIndex {
        palace_index_of_name(self.soul_idx, name)
    }

    /// The palace carrying `name`.
    pub fn palace(&self, name: PalaceName) -> &PalaceData {
        &self.palaces[self.palace_index_of(name)]
    }

    /// Natal name of the palace at `palace_idx`.
    pub fn palace_name_at(&self, palace_idx: PalaceIndex) -> PalaceName {
        get_palace_name(self.soul_idx, palace_idx % 12)
    }

    /// Palace holding `star`, if it is placed in this chart.
    pub fn star_palace(&self, star: ZwdsStar) -> Option<PalaceIndex> {
        self.palaces
            .iter()
            .find(|p| p.stars.iter().any(|s| s.star == star))
            .map(|p| p.index)
    }

    /// Where the four transformations of `stem` land in this chart.
    ///
    /// This is the primitive behind every 飛星 query: pass the stem of a palace of *this* chart
    /// (飛星), of another person's chart (合盤) or of a luck period (大限/小限/流年).
    pub fn si_hua_landings_for_stem(&self, stem: HeavenlyStem) -> Vec<SiHuaLanding> {
        let stars = get_sihua_stars(stem);
        let types = [
            SiHuaType::HuaLu,
            SiHuaType::HuaQuan,
            SiHuaType::HuaKe,
            SiHuaType::HuaJi,
        ];
        types
            .into_iter()
            .zip(stars)
            .filter_map(|(sihua_type, star)| {
                self.star_palace(star).map(|palace_idx| SiHuaLanding {
                    sihua_type,
                    star,
                    palace_idx,
                    palace_name: self.palace_name_at(palace_idx),
                })
            })
            .collect()
    }

    /// 飛星: the four transformations flown from the stem of the palace at `from_idx`.
    pub fn flying_from_index(&self, from_idx: PalaceIndex) -> PalaceSiHua {
        let from_idx = from_idx % 12;
        let stem = self.palace_stem(from_idx);
        PalaceSiHua {
            from_palace_idx: from_idx,
            from_palace: self.palace_name_at(from_idx),
            stem_hanja: stem.hanja().to_string(),
            landings: self.si_hua_landings_for_stem(stem),
        }
    }

    /// 飛星: the four transformations flown from palace `from` (by natal name).
    pub fn flying_from(&self, from: PalaceName) -> PalaceSiHua {
        self.flying_from_index(self.palace_index_of(from))
    }

    /// 飛星 from palace `from` that land in palace `to` (empty when none do).
    pub fn flying_between(&self, from: PalaceName, to: PalaceName) -> Vec<SiHuaLanding> {
        self.flying_from(from)
            .landings
            .into_iter()
            .filter(|l| l.palace_name == to)
            .collect()
    }

    /// Every palace that flies a transformation into `to`: `(source palace, landing)`.
    pub fn flying_into(&self, to: PalaceName) -> Vec<(PalaceName, SiHuaLanding)> {
        PalaceName::ALL
            .into_iter()
            .flat_map(|from| {
                self.flying_between(from, to)
                    .into_iter()
                    .map(move |l| (from, l))
            })
            .collect()
    }

    /// 三方四正 of palace `name`.
    pub fn san_fang_si_zheng(&self, name: PalaceName) -> SanFangSiZheng {
        self.san_fang_si_zheng_at(self.palace_index_of(name))
    }

    /// 三方四正 of the palace at `palace_idx`.
    pub fn san_fang_si_zheng_at(&self, palace_idx: PalaceIndex) -> SanFangSiZheng {
        let p = (palace_idx % 12) as i32;
        let opposite_idx = fix_index(p + 6);
        let mut trine_idx = [fix_index(p + 4), fix_index(p + 8)];
        trine_idx.sort_unstable();
        SanFangSiZheng {
            palace_idx: p as usize,
            palace: self.palace_name_at(p as usize),
            opposite_idx,
            opposite: self.palace_name_at(opposite_idx),
            trine_idx,
            trine: [
                self.palace_name_at(trine_idx[0]),
                self.palace_name_at(trine_idx[1]),
            ],
        }
    }

    /// The stars standing in the 三方四正 of palace `name` (self, opposite, trines, in that order).
    pub fn san_fang_si_zheng_stars(&self, name: PalaceName) -> Vec<SanFangStars> {
        let group = self.san_fang_si_zheng(name);
        group
            .palace_indices()
            .into_iter()
            .map(|idx| SanFangStars {
                palace_idx: idx,
                palace: self.palace_name_at(idx),
                stars: self.palaces[idx].stars.clone(),
            })
            .collect()
    }

    /// 大限四化: the transformations flown from the stem of a 大限 palace.
    pub fn daxian_si_hua(&self, daxian: &DaXian) -> PalaceSiHua {
        self.flying_from_index(daxian.palace_idx)
    }

    /// The 大限 covering a nominal age (虛歲), if any.
    pub fn daxian_for_age(&self, age: u32) -> Option<&DaXian> {
        self.daxian
            .iter()
            .find(|d| age >= d.age_start && age <= d.age_end)
    }

    /// Nominal age (虛歲) in `target_year`, counted from the lunar birth year.
    pub fn nominal_age(&self, target_year: i32) -> u32 {
        (target_year - self.birth_lunar.year + 1).max(1) as u32
    }

    /// 小限 for `target_year`.
    pub fn xiaoxian(&self, target_year: i32) -> XiaoXian {
        let age = self.nominal_age(target_year);
        let palace_idx = xiaoxian_palace(self.birth_lunar.year_branch_idx, self.is_male, age);
        let stem = self.palace_stem(palace_idx);
        XiaoXian {
            year: target_year,
            age,
            palace_idx,
            palace_name: self.palace_name_at(palace_idx),
            stem_hanja: stem.hanja().to_string(),
            branch_hanja: branch_hanja(palace_idx),
            si_hua: get_sihua_stars(stem),
        }
    }

    /// 斗君 for `target_year`.
    pub fn dou_jun(&self, target_year: i32) -> DouJun {
        let target_branch_std = (target_year - 4).rem_euclid(12) as usize;
        let palace_idx = dou_jun_palace(
            target_branch_std,
            self.birth_lunar.month,
            self.birth_lunar.time_branch_idx,
        );
        let stem = self.palace_stem(palace_idx);
        DouJun {
            year: target_year,
            palace_idx,
            palace_name: self.palace_name_at(palace_idx),
            stem_hanja: stem.hanja().to_string(),
            branch_hanja: branch_hanja(palace_idx),
        }
    }
}

fn branch_hanja(palace_idx: PalaceIndex) -> String {
    EarthlyBranch::from_index(zwds_idx_to_std_idx(palace_idx % 12) as i32)
        .hanja()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendar::lunar_to_birth_info;
    use crate::chart::build_chart_from_lunar;
    use eon_core::birth::{BirthInfo, Gender};

    /// 庚午 year (1990), lunar 4/21, 巳 hour, male: Ming in 子, Fire6, 祿存 in 申.
    fn chart_1990() -> ZwdsChart {
        let lunar = lunar_to_birth_info(1990, 4, 21, false, 10);
        let birth = BirthInfo::solar(1990, 5, 15, 10, 30).with_gender(Gender::Male);
        build_chart_from_lunar(&lunar, &birth).unwrap()
    }

    #[test]
    fn natal_reference_chart_layout() {
        let c = chart_1990();
        assert_eq!(c.soul_idx, 10); // 子
        assert_eq!(c.body_idx, 8); // 戌
                                   // Palace names run counter-clockwise from 命宮 (子): 兄弟 亥, 夫妻 戌, 財帛 申, 遷移 午, 官祿 辰.
        assert_eq!(c.palace_index_of(PalaceName::Xiongdi), 9);
        assert_eq!(c.palace_index_of(PalaceName::Fuqi), 8);
        assert_eq!(c.palace_index_of(PalaceName::Caibo), 6);
        assert_eq!(c.palace_index_of(PalaceName::Qianyi), 4);
        assert_eq!(c.palace_index_of(PalaceName::Guanlu), 2);
        assert_eq!(c.palace_index_of(PalaceName::Fumu), 11);
        // The 身宮 (戌) falls in the 夫妻宮, one of the six palaces it may legally occupy.
        assert_eq!(c.palaces[c.body_idx].name, PalaceName::Fuqi);
        // 12 gods are attached to every palace.
        assert!(c.palaces.iter().all(|p| p.twelve_gods.is_some()));
        // Fire6 → 長生 in 寅, forward (陽男): 子 is 胎 and 亥 is 絕.
        let cs = |i: usize| c.palaces[i].twelve_gods.unwrap().chang_sheng;
        assert_eq!(cs(0), crate::twelve_gods::ChangSheng::ChangSheng);
        assert_eq!(cs(10), crate::twelve_gods::ChangSheng::Tai);
        assert_eq!(cs(9), crate::twelve_gods::ChangSheng::Jue);
        // 博士 starts at 祿存 (申).
        assert_eq!(
            c.palaces[c.star_palace(ZwdsStar::LuCun).unwrap()]
                .twelve_gods
                .unwrap()
                .bo_shi,
            crate::twelve_gods::BoShi::BoShi
        );
    }

    #[test]
    fn san_fang_si_zheng_of_ming() {
        let c = chart_1990();
        let g = c.san_fang_si_zheng(PalaceName::Ming);
        // 命宮 子 (10): 對宮 午 (4) = 遷移; the trine of 子 is 申·辰, i.e. 辰 (2) = 官祿 and
        // 申 (6) = 財帛.
        assert_eq!(g.opposite_idx, 4);
        assert_eq!(g.opposite, PalaceName::Qianyi);
        assert_eq!(g.trine_idx, [2, 6]);
        assert_eq!(g.trine, [PalaceName::Guanlu, PalaceName::Caibo]);
        let stars = c.san_fang_si_zheng_stars(PalaceName::Ming);
        assert_eq!(stars.len(), 4);
        assert_eq!(stars[0].palace, PalaceName::Ming);
    }

    #[test]
    fn flying_queries_match_natal_list() {
        let c = chart_1990();
        // 命宮 stem is 戊 (子 palace): 戊 → 貪狼祿 太陰權 右弼科 天機忌.
        let ming = c.flying_from(PalaceName::Ming);
        assert_eq!(ming.stem_hanja, "戊");
        let star_of = |t: SiHuaType| ming.landings.iter().find(|l| l.sihua_type == t).unwrap();
        assert_eq!(star_of(SiHuaType::HuaLu).star, ZwdsStar::TanLang);
        assert_eq!(star_of(SiHuaType::HuaJi).star, ZwdsStar::TianJi);
        // The flat list and the query agree for every palace and every transformation.
        for from in PalaceName::ALL {
            let q = c.flying_from(from);
            for l in &q.landings {
                assert!(c.flying_sihua.iter().any(|f| f.from_palace == from
                    && f.to_palace == l.palace_name
                    && f.sihua_type == l.sihua_type
                    && f.star == l.star));
            }
            assert_eq!(q.landings.len(), 4);
        }
        // flying_into is the transpose of flying_between.
        let into = c.flying_into(PalaceName::Fuqi);
        for (from, l) in &into {
            assert!(c.flying_between(*from, PalaceName::Fuqi).contains(l));
        }
    }

    #[test]
    fn daxian_si_hua_uses_the_decade_palace_stem() {
        let c = chart_1990();
        // Fire6 → first 大限 age 6–15 in 命宮 (子, stem 戊); second (陽男 forward) in 丑 (stem 己).
        let d0 = &c.daxian[0];
        assert_eq!(d0.palace_idx, 10);
        assert_eq!(c.daxian_si_hua(d0).stem_hanja, "戊");
        let d1 = c.daxian_for_age(20).unwrap();
        assert_eq!(d1.palace_idx, 11);
        let sh = c.daxian_si_hua(d1);
        assert_eq!(sh.stem_hanja, "己");
        // 己 → 武曲祿 貪狼權 天梁科 文曲忌.
        assert_eq!(sh.landings[0].star, ZwdsStar::WuQu);
        assert_eq!(sh.landings[3].star, ZwdsStar::WenQu);
    }

    #[test]
    fn xiaoxian_and_dou_jun_for_reference_chart() {
        let c = chart_1990();
        // 2026: nominal age 37; 午 year male → 小限 age 1 in 辰, 37 − 1 = 36 ≡ 0 → back in 辰 (2).
        let x = c.xiaoxian(2026);
        assert_eq!(x.age, 37);
        assert_eq!(x.palace_idx, 2);
        assert_eq!(x.branch_hanja, "辰");
        assert_eq!(x.stem_hanja, "庚"); // 庚午 year: 寅 = 戊, so 辰 = 庚
        assert_eq!(x.palace_name, PalaceName::Guanlu);
        // 2026 丙午, lunar month 4, 巳 hour → 斗君 in 申 (6): 午 (4) − 3 + 5.
        let d = c.dou_jun(2026);
        assert_eq!(d.palace_idx, 6);
        assert_eq!(d.branch_hanja, "申");
        assert_eq!(d.palace_name, PalaceName::Caibo);
    }

    #[test]
    fn female_chart_runs_xiaoxian_backwards() {
        // 甲子 (1984) lunar 6/1, 戌 hour (std idx 10 → hour 20), female.
        let lunar = lunar_to_birth_info(1984, 6, 1, false, 20);
        let birth = BirthInfo::solar(1984, 7, 1, 20, 0).with_gender(Gender::Female);
        let c = build_chart_from_lunar(&lunar, &birth).unwrap();
        assert!(!c.is_male);
        // 金四局 with Ming in 酉 (hand-derived in the twelve_gods tests): 陽女 → 逆行.
        assert_eq!(c.soul_idx, 7);
        assert_eq!(c.five_elements, crate::types::FiveElementsClass::Metal4);
        // 申子辰 → 小限 age 1 in 戌 (8); female age 43 (2026) → 辰 (2).
        assert_eq!(c.xiaoxian(2026).age, 43);
        assert_eq!(c.xiaoxian(2026).palace_idx, 2);
        // Direction of 大限: 陽女 逆行 → second decade in 申 (idx 6 = 7 − 1).
        assert_eq!(c.daxian[1].palace_idx, 6);
    }
}
