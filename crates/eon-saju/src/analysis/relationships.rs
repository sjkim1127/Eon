//! Stem and Branch Interactions Analysis (Combinations, Clashes, Punishments, Harms, Destructions)
//!
//! Analyzes special dynamic relationships between Heavenly Stems and Earthly Branches.
//!
//! ## Heavenly Stem Relationships
//! - **Stem Combinations**: Jia-Ji, Yi-Geng, Bing-Xin, Ding-Ren, Wu-Gui
//! - **Stem Clashes**: Jia-Geng, Yi-Xin, Bing-Ren, Ding-Gui
//!
//! ## Earthly Branch Relationships
//! - **Triple Alliances**: Yin-Wu-Xu (Fire), Shen-Zi-Chen (Water), Si-You-Chou (Metal), Hai-Mao-Wei (Wood)
//! - **Semi-Alliances**: Two branches of a triple alliance
//! - **Seasonal Alliances**: Yin-Mao-Chen (East/Spring), Si-Wu-Wei (South/Summer), Shen-You-Xu (West/Autumn), Hai-Zi-Chou (North/Winter)
//! - **Six Combinations**: Zi-Chou, Yin-Hai, Mao-Xu, Chen-You, Si-Shen, Wu-Wei
//! - **Branch Clashes**: Zi-Wu, Chou-Wei, Yin-Shen, Mao-You, Chen-Xu, Si-Hai
//! - **Branch Punishments**: Triple (Yin-Si-Shen), Mutual (Chou-Xu-Wei), Self (Chen, Wu, You, Hai)
//! - **Branch Harms**: Zi-Wei, Chou-Wu, Yin-Si, Mao-Chen, Shen-Hai, You-Xu
//! - **Branch Destructions**: Zi-You, Chou-Chen, Yin-Hai, Mao-Wu, Si-Shen, Wu-Wei

use crate::analysis::supplementary_pillars::InterpretationLevel;
use crate::core::branch::EarthlyBranch;
use crate::core::element::Element;
use crate::core::pillars::FourPillars;
use crate::core::stem::HeavenlyStem;
use serde::{Deserialize, Serialize};

// ============================================
// Heavenly Stem Combinations
// ============================================

/// Heavenly Stem Combination types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StemCombination {
    /// 甲己合 → 土
    JiaJi,
    /// 乙庚合 → 金
    YiGeng,
    /// 丙辛合 → 水
    BingXin,
    /// 丁壬合 → 木
    DingRen,
    /// 戊癸合 → 火
    WuGui,
}

impl StemCombination {
    /// Check if two heavenly stems form a combination
    pub fn check(stem1: HeavenlyStem, stem2: HeavenlyStem) -> Option<Self> {
        use HeavenlyStem::*;
        match (stem1, stem2) {
            (Jia, Ji) | (Ji, Jia) => Some(Self::JiaJi),
            (Yi, Geng) | (Geng, Yi) => Some(Self::YiGeng),
            (Bing, Xin) | (Xin, Bing) => Some(Self::BingXin),
            (Ding, Ren) | (Ren, Ding) => Some(Self::DingRen),
            (Wu, Gui) | (Gui, Wu) => Some(Self::WuGui),
            _ => None,
        }
    }

    /// Transformed element resulting from combination
    pub const fn transformed_element(&self) -> Element {
        match self {
            Self::JiaJi => Element::Earth,
            Self::YiGeng => Element::Metal,
            Self::BingXin => Element::Water,
            Self::DingRen => Element::Wood,
            Self::WuGui => Element::Fire,
        }
    }

    pub const fn hangul(&self) -> &'static str {
        match self {
            Self::JiaJi => "Jia-Ji Combination",
            Self::YiGeng => "Yi-Geng Combination",
            Self::BingXin => "Bing-Xin Combination",
            Self::DingRen => "Ding-Ren Combination",
            Self::WuGui => "Wu-Gui Combination",
        }
    }

    /// Return component stems
    pub const fn stems(&self) -> (HeavenlyStem, HeavenlyStem) {
        use HeavenlyStem::*;
        match self {
            Self::JiaJi => (Jia, Ji),
            Self::YiGeng => (Yi, Geng),
            Self::BingXin => (Bing, Xin),
            Self::DingRen => (Ding, Ren),
            Self::WuGui => (Wu, Gui),
        }
    }

    pub const fn hanja(&self) -> &'static str {
        match self {
            Self::JiaJi => "甲己合",
            Self::YiGeng => "乙庚合",
            Self::BingXin => "丙辛合",
            Self::DingRen => "丁壬合",
            Self::WuGui => "戊癸合",
        }
    }
}

// ============================================
// Heavenly Stem Clashes
// ============================================

/// Heavenly Stem Clash types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StemClash {
    /// 甲庚沖
    JiaGeng,
    /// 乙辛沖
    YiXin,
    /// 丙壬沖
    BingRen,
    /// 丁癸沖
    DingGui,
}

impl StemClash {
    /// Check if two heavenly stems clash
    pub fn check(stem1: HeavenlyStem, stem2: HeavenlyStem) -> Option<Self> {
        use HeavenlyStem::*;
        match (stem1, stem2) {
            (Jia, Geng) | (Geng, Jia) => Some(Self::JiaGeng),
            (Yi, Xin) | (Xin, Yi) => Some(Self::YiXin),
            (Bing, Ren) | (Ren, Bing) => Some(Self::BingRen),
            (Ding, Gui) | (Gui, Ding) => Some(Self::DingGui),
            _ => None,
        }
    }

    pub const fn hangul(&self) -> &'static str {
        match self {
            Self::JiaGeng => "Jia-Geng Clash",
            Self::YiXin => "Yi-Xin Clash",
            Self::BingRen => "Bing-Ren Clash",
            Self::DingGui => "Ding-Gui Clash",
        }
    }

    pub const fn hanja(&self) -> &'static str {
        match self {
            Self::JiaGeng => "甲庚沖",
            Self::YiXin => "乙辛沖",
            Self::BingRen => "丙壬沖",
            Self::DingGui => "丁癸沖",
        }
    }
}

// ============================================
// Earthly Branch Triple Alliances
// ============================================

/// Triple Alliance types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TripleCombination {
    /// 寅午戌 → 火局
    YinWuXu,
    /// 申子辰 → 水局
    ShenZiChen,
    /// 巳酉丑 → 金局
    SiYouChou,
    /// 亥卯未 → 木局
    HaiMaoWei,
}

impl TripleCombination {
    /// Check all triple alliances present in branch list
    pub fn check(branches: &[EarthlyBranch]) -> Vec<Self> {
        use EarthlyBranch::*;
        let mut results = Vec::new();

        let has_all =
            |b1, b2, b3| branches.contains(&b1) && branches.contains(&b2) && branches.contains(&b3);

        if has_all(Yin, Wu, Xu) {
            results.push(Self::YinWuXu);
        }
        if has_all(Shen, Zi, Chen) {
            results.push(Self::ShenZiChen);
        }
        if has_all(Si, You, Chou) {
            results.push(Self::SiYouChou);
        }
        if has_all(Hai, Mao, Wei) {
            results.push(Self::HaiMaoWei);
        }
        results
    }

    /// Transformed element
    pub const fn element(&self) -> Element {
        match self {
            Self::YinWuXu => Element::Fire,
            Self::ShenZiChen => Element::Water,
            Self::SiYouChou => Element::Metal,
            Self::HaiMaoWei => Element::Wood,
        }
    }

    pub const fn hangul(&self) -> &'static str {
        match self {
            Self::YinWuXu => "Yin-Wu-Xu Fire Alliance",
            Self::ShenZiChen => "Shen-Zi-Chen Water Alliance",
            Self::SiYouChou => "Si-You-Chou Metal Alliance",
            Self::HaiMaoWei => "Hai-Mao-Wei Wood Alliance",
        }
    }

    /// Return component branches
    pub fn branches(&self) -> [EarthlyBranch; 3] {
        use EarthlyBranch::*;
        match self {
            Self::YinWuXu => [Yin, Wu, Xu],
            Self::ShenZiChen => [Shen, Zi, Chen],
            Self::SiYouChou => [Si, You, Chou],
            Self::HaiMaoWei => [Hai, Mao, Wei],
        }
    }
}

// ============================================
// Earthly Branch Semi-Alliances
// ============================================

/// Semi-Alliance types (2 of 3 triple alliance branches)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SemiCombination {
    /// Yin-Wu (Fire Semi-Alliance)
    YinWu,
    /// Wu-Xu (Fire Semi-Alliance)
    WuXu,
    /// Yin-Xu (Fire Semi-Alliance without cardinal)
    YinXu,
    /// Shen-Zi (Water Semi-Alliance)
    ShenZi,
    /// Zi-Chen (Water Semi-Alliance)
    ZiChen,
    /// Shen-Chen (Water Semi-Alliance without cardinal)
    ShenChen,
    /// Si-You (Metal Semi-Alliance)
    SiYou,
    /// You-Chou (Metal Semi-Alliance)
    YouChou,
    /// Si-Chou (Metal Semi-Alliance without cardinal)
    SiChou,
    /// Hai-Mao (Wood Semi-Alliance)
    HaiMao,
    /// Mao-Wei (Wood Semi-Alliance)
    MaoWei,
    /// Hai-Wei (Wood Semi-Alliance without cardinal)
    HaiWei,
}

impl SemiCombination {
    /// Check if two branches form a semi-alliance
    pub fn check(b1: EarthlyBranch, b2: EarthlyBranch) -> Option<Self> {
        use EarthlyBranch::*;
        match (b1, b2) {
            (Yin, Wu) | (Wu, Yin) => Some(Self::YinWu),
            (Wu, Xu) | (Xu, Wu) => Some(Self::WuXu),
            (Yin, Xu) | (Xu, Yin) => Some(Self::YinXu),
            (Shen, Zi) | (Zi, Shen) => Some(Self::ShenZi),
            (Zi, Chen) | (Chen, Zi) => Some(Self::ZiChen),
            (Shen, Chen) | (Chen, Shen) => Some(Self::ShenChen),
            (Si, You) | (You, Si) => Some(Self::SiYou),
            (You, Chou) | (Chou, You) => Some(Self::YouChou),
            (Si, Chou) | (Chou, Si) => Some(Self::SiChou),
            (Hai, Mao) | (Mao, Hai) => Some(Self::HaiMao),
            (Mao, Wei) | (Wei, Mao) => Some(Self::MaoWei),
            (Hai, Wei) | (Wei, Hai) => Some(Self::HaiWei),
            _ => None,
        }
    }

    pub fn hangul(&self) -> &'static str {
        match self {
            Self::YinWu => "Yin-Wu Semi-Alliance",
            Self::WuXu => "Wu-Xu Semi-Alliance",
            Self::YinXu => "Yin-Xu Semi-Alliance",
            Self::ShenZi => "Shen-Zi Semi-Alliance",
            Self::ZiChen => "Zi-Chen Semi-Alliance",
            Self::ShenChen => "Shen-Chen Semi-Alliance",
            Self::SiYou => "Si-You Semi-Alliance",
            Self::YouChou => "You-Chou Semi-Alliance",
            Self::SiChou => "Si-Chou Semi-Alliance",
            Self::HaiMao => "Hai-Mao Semi-Alliance",
            Self::MaoWei => "Mao-Wei Semi-Alliance",
            Self::HaiWei => "Hai-Wei Semi-Alliance",
        }
    }

    /// Check if semi-alliance includes cardinal branch (Zi, Wu, Mao, You)
    pub fn is_dominant(&self) -> bool {
        matches!(
            self,
            Self::YinWu
                | Self::WuXu
                | Self::ShenZi
                | Self::ZiChen
                | Self::SiYou
                | Self::YouChou
                | Self::HaiMao
                | Self::MaoWei
        )
    }
}

// ============================================
// Earthly Branch Seasonal Alliances
// ============================================

/// Seasonal Alliance types (directional combinations)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SeasonalCombination {
    /// Yin-Mao-Chen -> Wood Directional (Spring)
    YinMaoChen,
    /// Si-Wu-Wei -> Fire Directional (Summer)
    SiWuWei,
    /// Shen-You-Xu -> Metal Directional (Autumn)
    ShenYouXu,
    /// Hai-Zi-Chou -> Water Directional (Winter)
    HaiZiChou,
}

impl SeasonalCombination {
    /// Check all seasonal alliances present in branch list
    pub fn check(branches: &[EarthlyBranch]) -> Vec<Self> {
        use EarthlyBranch::*;
        let mut results = Vec::new();

        let has_all =
            |b1, b2, b3| branches.contains(&b1) && branches.contains(&b2) && branches.contains(&b3);

        if has_all(Yin, Mao, Chen) {
            results.push(Self::YinMaoChen);
        }
        if has_all(Si, Wu, Wei) {
            results.push(Self::SiWuWei);
        }
        if has_all(Shen, You, Xu) {
            results.push(Self::ShenYouXu);
        }
        if has_all(Hai, Zi, Chou) {
            results.push(Self::HaiZiChou);
        }
        results
    }

    /// Return component branches
    pub fn branches(&self) -> [EarthlyBranch; 3] {
        use EarthlyBranch::*;
        match self {
            Self::YinMaoChen => [Yin, Mao, Chen],
            Self::SiWuWei => [Si, Wu, Wei],
            Self::ShenYouXu => [Shen, You, Xu],
            Self::HaiZiChou => [Hai, Zi, Chou],
        }
    }

    /// Transformed element from seasonal alliance
    pub const fn element(&self) -> Element {
        match self {
            Self::YinMaoChen => Element::Wood,
            Self::SiWuWei => Element::Fire,
            Self::ShenYouXu => Element::Metal,
            Self::HaiZiChou => Element::Water,
        }
    }

    pub const fn hangul(&self) -> &'static str {
        match self {
            Self::YinMaoChen => "Yin-Mao-Chen Wood Directional",
            Self::SiWuWei => "Si-Wu-Wei Fire Directional",
            Self::ShenYouXu => "Shen-You-Xu Metal Directional",
            Self::HaiZiChou => "Hai-Zi-Chou Water Directional",
        }
    }
}

// ============================================
// Earthly Branch Six Combinations
// ============================================

/// Six Combination types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SixCombination {
    /// 子丑合 → 土
    ZiChou,
    /// 寅亥合 → 木
    YinHai,
    /// 卯戌合 → 火
    MaoXu,
    /// 辰酉合 → 金
    ChenYou,
    /// 巳申合 → 水
    SiShen,
    /// 午未合 → 火/土
    WuWei,
}

impl SixCombination {
    /// Check if two branches form a six-combination
    pub fn check(b1: EarthlyBranch, b2: EarthlyBranch) -> Option<Self> {
        use EarthlyBranch::*;
        match (b1, b2) {
            (Zi, Chou) | (Chou, Zi) => Some(Self::ZiChou),
            (Yin, Hai) | (Hai, Yin) => Some(Self::YinHai),
            (Mao, Xu) | (Xu, Mao) => Some(Self::MaoXu),
            (Chen, You) | (You, Chen) => Some(Self::ChenYou),
            (Si, Shen) | (Shen, Si) => Some(Self::SiShen),
            _ => None,
        }
    }

    /// Transformed element resulting from six combination
    pub const fn transformed_element(&self) -> Option<Element> {
        match self {
            Self::ZiChou => Some(Element::Earth), // Zi-Chou transforms to Earth
            Self::YinHai => Some(Element::Wood),  // Yin-Hai transforms to Wood
            Self::MaoXu => Some(Element::Fire),   // Mao-Xu transforms to Fire
            Self::ChenYou => Some(Element::Metal), // Chen-You transforms to Metal
            Self::SiShen => Some(Element::Water), // Si-Shen transforms to Water
            Self::WuWei => Some(Element::Fire),   // Wu-Wei transforms to Fire
        }
    }

    pub const fn hangul(&self) -> &'static str {
        match self {
            Self::ZiChou => "Zi-Chou Six Combination",
            Self::YinHai => "Yin-Hai Six Combination",
            Self::MaoXu => "Mao-Xu Six Combination",
            Self::ChenYou => "Chen-You Six Combination",
            Self::SiShen => "Si-Shen Six Combination",
            Self::WuWei => "Wu-Wei Six Combination",
        }
    }

    /// Return component branches
    pub const fn branches(&self) -> (EarthlyBranch, EarthlyBranch) {
        use EarthlyBranch::*;
        match self {
            Self::ZiChou => (Zi, Chou),
            Self::YinHai => (Yin, Hai),
            Self::MaoXu => (Mao, Xu),
            Self::ChenYou => (Chen, You),
            Self::SiShen => (Si, Shen),
            Self::WuWei => (Wu, Wei),
        }
    }
}

// ============================================
// Earthly Branch Clashes
// ============================================

/// Branch clash classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ClashType {
    /// Cardinal Clash - Zi-Wu, Mao-You
    /// Pure elemental clash with highest root disruption
    Cardinal,
    /// Growth Clash - Yin-Shen, Si-Hai
    /// Complex hidden stems with partial root disruption
    Growth,
    /// Storage/Tomb Clash - Chou-Wei, Chen-Xu
    /// Earth clash with low damage and unsealing effect
    Storage,
}

impl ClashType {
    /// Formatted English name
    pub const fn hangul(&self) -> &'static str {
        match self {
            Self::Cardinal => "CardinalClash",
            Self::Growth => "GrowthClash",
            Self::Storage => "StorageClash",
        }
    }

    /// Root damage ratio (0.0 ~ 1.0)
    /// Root weighting deduction factor
    pub const fn damage_ratio(&self) -> f32 {
        match self {
            Self::Cardinal => 0.70, // Cardinal clash: 70% damage
            Self::Growth => 0.50,   // Growth clash: 50% damage
            Self::Storage => 0.20,  // Storage clash: 20% damage (unsealing effect)
        }
    }
}

/// Earthly Branch Clash types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BranchClash {
    /// Zi-Wu Clash
    ZiWu,
    /// Chou-Wei Clash
    ChouWei,
    /// Yin-Shen Clash
    YinShen,
    /// Mao-You Clash
    MaoYou,
    /// Chen-Xu Clash
    ChenXu,
    /// Si-Hai Clash
    SiHai,
}

impl BranchClash {
    /// Check if two branches clash
    pub fn check(b1: EarthlyBranch, b2: EarthlyBranch) -> Option<Self> {
        use EarthlyBranch::*;
        match (b1, b2) {
            (Zi, Wu) | (Wu, Zi) => Some(Self::ZiWu),
            (Chou, Wei) | (Wei, Chou) => Some(Self::ChouWei),
            (Yin, Shen) | (Shen, Yin) => Some(Self::YinShen),
            (Mao, You) | (You, Mao) => Some(Self::MaoYou),
            (Chen, Xu) | (Xu, Chen) => Some(Self::ChenXu),
            (Si, Hai) | (Hai, Si) => Some(Self::SiHai),
            _ => None,
        }
    }

    /// Clash classification (Cardinal, Growth, Storage)
    pub const fn clash_type(&self) -> ClashType {
        match self {
            // Cardinal clash: Zi-Wu, Mao-You
            Self::ZiWu | Self::MaoYou => ClashType::Cardinal,
            // Growth clash: Yin-Shen, Si-Hai
            Self::YinShen | Self::SiHai => ClashType::Growth,
            // Storage clash: Chou-Wei, Chen-Xu
            Self::ChouWei | Self::ChenXu => ClashType::Storage,
        }
    }

    /// Root damage ratio
    pub const fn damage_ratio(&self) -> f32 {
        self.clash_type().damage_ratio()
    }

    pub const fn hangul(&self) -> &'static str {
        match self {
            Self::ZiWu => "Zi-Wu Clash",
            Self::ChouWei => "Chou-Wei Clash",
            Self::YinShen => "Yin-Shen Clash",
            Self::MaoYou => "Mao-You Clash",
            Self::ChenXu => "Chen-Xu Clash",
            Self::SiHai => "Si-Hai Clash",
        }
    }

    /// Return component branches
    pub const fn branches(&self) -> (EarthlyBranch, EarthlyBranch) {
        use EarthlyBranch::*;
        match self {
            Self::ZiWu => (Zi, Wu),
            Self::ChouWei => (Chou, Wei),
            Self::YinShen => (Yin, Shen),
            Self::MaoYou => (Mao, You),
            Self::ChenXu => (Chen, Xu),
            Self::SiHai => (Si, Hai),
        }
    }
}

// ============================================
// Earthly Branch Punishments
// ============================================

/// Branch Punishment types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BranchPunishment {
    /// Triple Punishment (Yin-Si-Shen)
    TriplePunishment,
    /// Mutual Punishment (Chou-Xu-Wei)
    MutualPunishment,
    /// Self Punishment (Chen, Wu, You, Hai)
    SelfPunishment(EarthlyBranch),
}

impl BranchPunishment {
    /// Check if branch is self-punishment
    pub fn is_self_punishment(branch: EarthlyBranch) -> bool {
        use EarthlyBranch::*;
        matches!(branch, Chen | Wu | You | Hai)
    }

    /// Check if two branches form self-punishment
    pub fn check_self(b1: EarthlyBranch, b2: EarthlyBranch) -> Option<Self> {
        if b1 == b2 && Self::is_self_punishment(b1) {
            Some(Self::SelfPunishment(b1))
        } else {
            None
        }
    }

    /// Check Triple Punishment (Yin-Si-Shen)
    pub fn check_triple(branches: &[EarthlyBranch]) -> bool {
        use EarthlyBranch::*;
        branches.contains(&Yin) && branches.contains(&Si) && branches.contains(&Shen)
    }

    /// Check Mutual Punishment (Chou-Xu-Wei)
    pub fn check_mutual(branches: &[EarthlyBranch]) -> bool {
        use EarthlyBranch::*;
        branches.contains(&Chou) && branches.contains(&Xu) && branches.contains(&Wei)
    }

    pub fn hangul(&self) -> String {
        match self {
            Self::TriplePunishment => "Triple Punishment (Yin-Si-Shen)".to_string(),
            Self::MutualPunishment => "Mutual Punishment (Chou-Xu-Wei)".to_string(),
            Self::SelfPunishment(b) => format!("Self Punishment ({})", b.hangul()),
        }
    }
}

// ============================================
// Earthly Branch Destructions
// ============================================

/// Branch Destruction types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BranchDestruction {
    /// Zi-You Destruction
    ZiYou,
    /// Mao-Wu Destruction
    MaoWu,
    /// Chen-Chou Destruction
    ChenChou,
    /// Wei-Xu Destruction
    WeiXu,
    /// Yin-Hai Destruction
    YinHai,
    /// Si-Shen Destruction
    SiShen,
}

impl BranchDestruction {
    /// Check if two branches form destruction
    pub fn check(b1: EarthlyBranch, b2: EarthlyBranch) -> Option<Self> {
        use EarthlyBranch::*;
        match (b1, b2) {
            (Zi, You) | (You, Zi) => Some(Self::ZiYou),
            (Mao, Wu) | (Wu, Mao) => Some(Self::MaoWu),
            (Chen, Chou) | (Chou, Chen) => Some(Self::ChenChou),
            (Wei, Xu) | (Xu, Wei) => Some(Self::WeiXu),
            (Yin, Hai) | (Hai, Yin) => Some(Self::YinHai),
            (Si, Shen) | (Shen, Si) => Some(Self::SiShen),
            _ => None,
        }
    }

    pub const fn hangul(&self) -> &'static str {
        match self {
            Self::ZiYou => "Zi-You Destruction",
            Self::MaoWu => "Mao-Wu Destruction",
            Self::ChenChou => "Chen-Chou Destruction",
            Self::WeiXu => "Wei-Xu Destruction",
            Self::YinHai => "Yin-Hai Destruction",
            Self::SiShen => "Si-Shen Destruction",
        }
    }
}

// ============================================
// Earthly Branch Harms
// ============================================

/// Branch Harm types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BranchHarm {
    /// Zi-Wei Harm
    ZiWei,
    /// Chou-Wu Harm
    ChouWu,
    /// Yin-Si Harm
    YinSi,
    /// Mao-Chen Harm
    MaoChen,
    /// Shen-Hai Harm
    ShenHai,
    /// You-Xu Harm
    YouXu,
}

impl BranchHarm {
    /// Check if two branches form harm
    pub fn check(b1: EarthlyBranch, b2: EarthlyBranch) -> Option<Self> {
        use EarthlyBranch::*;
        match (b1, b2) {
            (Zi, Wei) | (Wei, Zi) => Some(Self::ZiWei),
            (Chou, Wu) | (Wu, Chou) => Some(Self::ChouWu),
            (Yin, Si) | (Si, Yin) => Some(Self::YinSi),
            (Mao, Chen) | (Chen, Mao) => Some(Self::MaoChen),
            (Shen, Hai) | (Hai, Shen) => Some(Self::ShenHai),
            (You, Xu) | (Xu, You) => Some(Self::YouXu),
            _ => None,
        }
    }

    pub const fn hangul(&self) -> &'static str {
        match self {
            Self::ZiWei => "Zi-Wei Harm",
            Self::ChouWu => "Chou-Wu Harm",
            Self::YinSi => "Yin-Si Harm",
            Self::MaoChen => "Mao-Chen Harm",
            Self::ShenHai => "Shen-Hai Harm",
            Self::YouXu => "You-Xu Harm",
        }
    }
}

// ============================================
// Hidden Combinations (Amhap) and Manifest-Hidden Combinations (MyungAmHap)
// ============================================

/// Hidden combination between hidden stems in earthly branches
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Amhap {
    pub branches: (EarthlyBranch, EarthlyBranch),
    pub combination: StemCombination,
}

/// Manifest-hidden combination between heavenly stem and branch hidden stem
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MyungAmHap {
    pub stem: HeavenlyStem,
    pub branch: EarthlyBranch,
    pub combination: StemCombination,
}

/// Found relation info (legacy)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoundRelation {
    pub description: String,
    pub positions: (String, String),
}

/// Detailed explainable relationship interaction
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipDetail {
    pub relation_type: String,
    pub name: String,
    pub positions: Vec<String>,
    pub level: InterpretationLevel,
    pub summary: String,
    pub description: String,
    pub reasons: Vec<String>,
    pub transformed_element: Option<Element>,
}

/// Relationship analysis outcome
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipAnalysis {
    /// All detailed explainable relationships
    pub mapped_relationships: Vec<RelationshipDetail>,
    /// Stem combinations
    pub stem_combinations: Vec<(StemCombination, String, String)>,
    /// Stem clashes
    pub stem_clashes: Vec<(StemClash, String, String)>,
    /// Triple alliances
    pub triple_combinations: Vec<TripleCombination>,
    /// Seasonal alliances
    pub seasonal_combinations: Vec<SeasonalCombination>,
    /// Dominant semi-alliances (with cardinal branch)
    pub dominant_semi_combinations: Vec<(SemiCombination, String, String)>,
    /// Weak semi-alliances (without cardinal branch)
    pub weak_semi_combinations: Vec<(SemiCombination, String, String)>,
    /// Six combinations
    pub six_combinations: Vec<(SixCombination, String, String)>,
    /// Branch clashes
    pub branch_clashes: Vec<(BranchClash, String, String)>,
    /// Branch punishments
    pub branch_punishments: Vec<(BranchPunishment, String, String)>,
    /// Branch harms
    pub branch_harms: Vec<(BranchHarm, String, String)>,
    /// Branch destructions
    pub branch_destructions: Vec<(BranchDestruction, String, String)>,
    /// Hidden combinations (Amhap)
    pub am_combinations: Vec<(Amhap, String, String)>,
    /// Manifest-hidden combinations (MyungAmHap)
    pub myung_am_combinations: Vec<(MyungAmHap, String, String)>,
}

impl RelationshipAnalysis {
    /// Analyze relationships from four pillars
    pub fn from_pillars(pillars: &FourPillars) -> Self {
        let mut stems = vec![
            ("year_stem", pillars.year.stem),
            ("month_stem", pillars.month.stem),
            ("day_stem", pillars.day.stem),
        ];
        let mut branches = vec![
            ("year_branch", pillars.year.branch),
            ("month_branch", pillars.month.branch),
            ("day_branch", pillars.day.branch),
        ];
        // The Hour pillar is absent when the birth time is unknown
        if let Some(hour) = pillars.hour {
            stems.push(("hour_stem", hour.stem));
            branches.push(("hour_branch", hour.branch));
        }

        let mut stem_combinations = Vec::new();
        let mut stem_clashes = Vec::new();

        let mut dominant_semi_combinations = Vec::new();
        let mut weak_semi_combinations = Vec::new();
        let mut six_combinations = Vec::new();
        let mut branch_clashes = Vec::new();
        let mut branch_punishments = Vec::new();
        let mut branch_harms = Vec::new();
        let mut branch_destructions = Vec::new();
        let mut am_combinations = Vec::new();
        let mut myung_am_combinations = Vec::new();

        // Analyze heavenly stems (all pairs)
        for i in 0..stems.len() {
            for j in (i + 1)..stems.len() {
                let (pos1, stem1) = stems[i];
                let (pos2, stem2) = stems[j];

                if let Some(comb) = StemCombination::check(stem1, stem2) {
                    stem_combinations.push((comb, pos1.to_string(), pos2.to_string()));
                }
                if let Some(clash) = StemClash::check(stem1, stem2) {
                    stem_clashes.push((clash, pos1.to_string(), pos2.to_string()));
                }
            }
        }

        // Analyze earthly branches (all pairs)
        for i in 0..branches.len() {
            for j in (i + 1)..branches.len() {
                let (pos1, branch1) = branches[i];
                let (pos2, branch2) = branches[j];

                if let Some(semi) = SemiCombination::check(branch1, branch2) {
                    if semi.is_dominant() {
                        dominant_semi_combinations.push((semi, pos1.to_string(), pos2.to_string()));
                    } else {
                        weak_semi_combinations.push((semi, pos1.to_string(), pos2.to_string()));
                    }
                }
                if let Some(six) = SixCombination::check(branch1, branch2) {
                    six_combinations.push((six, pos1.to_string(), pos2.to_string()));
                }
                if let Some(clash) = BranchClash::check(branch1, branch2) {
                    branch_clashes.push((clash, pos1.to_string(), pos2.to_string()));
                }
                if let Some(harm) = BranchHarm::check(branch1, branch2) {
                    branch_harms.push((harm, pos1.to_string(), pos2.to_string()));
                }
                if let Some(dest) = BranchDestruction::check(branch1, branch2) {
                    branch_destructions.push((dest, pos1.to_string(), pos2.to_string()));
                }
            }
        }

        // Analyze self-punishments
        for i in 0..branches.len() {
            for j in (i + 1)..branches.len() {
                let (pos1, branch1) = branches[i];
                let (pos2, branch2) = branches[j];
                if let Some(pun) = BranchPunishment::check_self(branch1, branch2) {
                    branch_punishments.push((pun, pos1.to_string(), pos2.to_string()));
                }
            }
        }

        // Analyze triple and mutual punishments
        let all_branches: Vec<_> = branches.iter().map(|(_, b)| *b).collect();
        if BranchPunishment::check_triple(&all_branches) {
            branch_punishments.push((
                BranchPunishment::TriplePunishment,
                "TriplePunishment".to_string(),
                "YinSiShen".to_string(),
            ));
        }
        if BranchPunishment::check_mutual(&all_branches) {
            branch_punishments.push((
                BranchPunishment::MutualPunishment,
                "MutualPunishment".to_string(),
                "ChouXuWei".to_string(),
            ));
        }

        // Analyze triple and seasonal alliances
        let all_branches: Vec<_> = branches.iter().map(|(_, b)| *b).collect();
        let triple_combinations = TripleCombination::check(&all_branches);
        let seasonal_combinations = SeasonalCombination::check(&all_branches);

        // Analyze hidden combinations (branch hidden stems)
        for i in 0..branches.len() {
            for j in (i + 1)..branches.len() {
                let (pos1, b1) = branches[i];
                let (pos2, b2) = branches[j];
                let combinations = Self::check_am_combinations(b1, b2);
                for comb in combinations {
                    am_combinations.push((
                        Amhap {
                            branches: (b1, b2),
                            combination: comb,
                        },
                        pos1.to_string(),
                        pos2.to_string(),
                    ));
                }
            }
        }

        // Analyze manifest-hidden combinations (stem with branch hidden stems)
        for (stem_pos, stem) in &stems {
            for (branch_pos, branch) in &branches {
                let combinations = Self::check_myung_am_combinations(*stem, *branch);
                for comb in combinations {
                    myung_am_combinations.push((
                        MyungAmHap {
                            stem: *stem,
                            branch: *branch,
                            combination: comb,
                        },
                        stem_pos.to_string(),
                        branch_pos.to_string(),
                    ));
                }
            }
        }

        // === Generate detailed explainable relationships ===
        let mut mapped_relationships = Vec::new();

        // 1. Branch Clash (BranchClash)
        for (clash, p1, p2) in &branch_clashes {
            let info = clash.clash_type();
            mapped_relationships.push(RelationshipDetail {
                relation_type: "BranchClash".to_string(),
                name: clash.hangul().to_string(),
                positions: vec![p1.clone(), p2.clone()],
                level: InterpretationLevel::Caution,
                summary: format!("Two forces collide directly via {}", clash.hangul()),
                description: format!(
                    "Clash corresponding to {}. Root vitality destabilized by approx {}%, driving strong structural changes.",
                    info.hangul(),
                    (info.damage_ratio() * 100.0) as u32
                ),
                reasons: vec![format!("{} ({})", info.hangul(), info.damage_ratio())],
                transformed_element: None,
            });
        }

        // 2. Stem Combination (StemCombination)
        for (comb, p1, p2) in &stem_combinations {
            let elem = comb.transformed_element();
            mapped_relationships.push(RelationshipDetail {
                relation_type: "StemCombination".to_string(),
                name: comb.hangul().to_string(),
                positions: vec![p1.clone(), p2.clone()],
                level: InterpretationLevel::Auspicious,
                summary: format!("New energy formed via {}", comb.hangul()),
                description: format!(
                    "Two heavenly stems combine to create a flow transforming toward {} nature.",
                    elem.hangul()
                ),
                reasons: vec![format!("{} transformation", elem.hangul())],
                transformed_element: Some(elem),
            });
        }

        // 3. Six Combination (SixCombination)
        for (six, p1, p2) in &six_combinations {
            let elem = six.transformed_element();
            mapped_relationships.push(RelationshipDetail {
                relation_type: "SixCombination".to_string(),
                name: six.hangul().to_string(),
                positions: vec![p1.clone(), p2.clone()],
                level: InterpretationLevel::Auspicious,
                summary: format!("Harmonious union of branches via {}", six.hangul()),
                description: "Distinct earthly branch energies bind harmoniously, providing stability and security.".to_string(),
                reasons: vec!["Six Combination".to_string()],
                transformed_element: elem,
            });
        }

        // 4. Triple Alliance (TripleCombination)
        for triple in &triple_combinations {
            let elem = triple.element();
            mapped_relationships.push(RelationshipDetail {
                relation_type: "TripleCombination".to_string(),
                name: triple.hangul().to_string(),
                positions: vec!["all".to_string()], // Triple alliance spans chart
                level: InterpretationLevel::Auspicious,
                summary: format!("{} energy amplified by formation of {}", elem.hangul(), triple.hangul()),
                description: "Three branches unite with purposeful alignment, forming a powerful elemental alliance bureau."
                    .to_string(),
                reasons: vec!["Triple Alliance Completed".to_string()],
                transformed_element: Some(elem),
            });
        }

        // 5. Seasonal Alliance (SeasonalCombination)
        for seasonal in &seasonal_combinations {
            let elem = seasonal.element();
            mapped_relationships.push(RelationshipDetail {
                relation_type: "SeasonalCombination".to_string(),
                name: seasonal.hangul().to_string(),
                positions: vec!["all".to_string()],
                level: InterpretationLevel::Auspicious,
                summary: format!("Seasonal forces coalesced via {}", seasonal.hangul()),
                description:
                    "Branches representing seasonal vitality assemble, making the corresponding elemental power extremely formidable."
                        .to_string(),
                reasons: vec!["Seasonal Combination".to_string()],
                transformed_element: Some(elem),
            });
        }

        // 6. Punishments, Harms, Destructions
        for (pun, p1, p2) in &branch_punishments {
            mapped_relationships.push(RelationshipDetail {
                relation_type: "BranchPunishment".to_string(),
                name: pun.hangul(),
                positions: vec![p1.clone(), p2.clone()],
                level: InterpretationLevel::Caution,
                summary: "Complex friction and interaction between earthly branches (Punishment)".to_string(),
                description: "Occurs when energies intertwine discordantly during coordination, potentially generating stress and adjustment friction.".to_string(),
                reasons: vec!["Branch Punishment".to_string()],
                transformed_element: None,
            });
        }
        for (harm, p1, p2) in &branch_harms {
            mapped_relationships.push(RelationshipDetail {
                relation_type: "BranchHarm".to_string(),
                name: harm.hangul().to_string(),
                positions: vec![p1.clone(), p2.clone()],
                level: InterpretationLevel::Caution,
                summary: "Mutual obstruction or impedance between branches (Harm)".to_string(),
                description: "Encountering energies that disrupt six-combinations, diminishing practical gains or inciting conflict."
                    .to_string(),
                reasons: vec!["Branch Harm".to_string()],
                transformed_element: None,
            });
        }

        // 7. Semi-Alliance (Dominant / Weak)
        for (semi, p1, p2) in &dominant_semi_combinations {
            let triple = match semi {
                SemiCombination::YinWu | SemiCombination::WuXu | SemiCombination::YinXu => {
                    TripleCombination::YinWuXu
                }
                SemiCombination::ShenZi | SemiCombination::ZiChen | SemiCombination::ShenChen => {
                    TripleCombination::ShenZiChen
                }
                SemiCombination::SiYou | SemiCombination::YouChou | SemiCombination::SiChou => {
                    TripleCombination::SiYouChou
                }
                SemiCombination::HaiMao | SemiCombination::MaoWei | SemiCombination::HaiWei => {
                    TripleCombination::HaiMaoWei
                }
            };
            let elem = triple.element();
            mapped_relationships.push(RelationshipDetail {
                relation_type: "SemiCombination".to_string(),
                name: format!("{}(眞)", semi.hangul()),
                positions: vec![p1.clone(), p2.clone()],
                level: InterpretationLevel::Auspicious,
                summary: format!("{} energy activated via {}", elem.hangul(), semi.hangul()),
                description: "Potent combination including a cardinal branch (Zi, Wu, Mao, You), inducing substantial elemental shifts akin to triple alliances.".to_string(),
                reasons: vec!["True Semi-Alliance".to_string(), format!("Contains {}", elem.hangul())],
                transformed_element: Some(elem),
            });
        }
        for (semi, p1, p2) in &weak_semi_combinations {
            let triple = match semi {
                SemiCombination::YinWu | SemiCombination::WuXu | SemiCombination::YinXu => {
                    TripleCombination::YinWuXu
                }
                SemiCombination::ShenZi | SemiCombination::ZiChen | SemiCombination::ShenChen => {
                    TripleCombination::ShenZiChen
                }
                SemiCombination::SiYou | SemiCombination::YouChou | SemiCombination::SiChou => {
                    TripleCombination::SiYouChou
                }
                SemiCombination::HaiMao | SemiCombination::MaoWei | SemiCombination::HaiWei => {
                    TripleCombination::HaiMaoWei
                }
            };
            let elem = triple.element();
            mapped_relationships.push(RelationshipDetail {
                relation_type: "SemiCombination".to_string(),
                name: format!("{}(Weak)", semi.hangul()),
                positions: vec![p1.clone(), p2.clone()],
                level: InterpretationLevel::Neutral,
                summary: format!("Formation of {} (subtle force)", semi.hangul()),
                description: "Combination lacking the central cardinal branch; while subtle in strength, it maintains an inclination toward that element.".to_string(),
                reasons: vec!["Weak Semi-Alliance".to_string(), "Missing cardinal branch".to_string()],
                transformed_element: Some(elem),
            });
        }

        // 8. Branch Destruction (BranchDestruction)
        for (dest, p1, p2) in &branch_destructions {
            mapped_relationships.push(RelationshipDetail {
                relation_type: "BranchDestruction".to_string(),
                name: dest.hangul().to_string(),
                positions: vec![p1.clone(), p2.clone()],
                level: InterpretationLevel::Caution,
                summary: "Disruption and destruction of energy (Destruction)".to_string(),
                description: "Action that breaks consolidated vitality or causes internal divergence, requiring meticulous calibration.".to_string(),
                reasons: vec!["Branch Destruction".to_string()],
                transformed_element: None,
            });
        }

        // 9. Hidden Combination / Manifest-Hidden Combination
        for (am, p1, p2) in &am_combinations {
            let elem = am.combination.transformed_element();
            mapped_relationships.push(RelationshipDetail {
                relation_type: "Amhap".to_string(),
                name: format!("Hidden Combo ({})", am.combination.hangul()),
                positions: vec![p1.clone(), p2.clone()],
                level: InterpretationLevel::Auspicious,
                summary: "Subtle bond between hidden earthly stems".to_string(),
                description: "Denotes covert rapport, affection, or unpublicized cooperation operating beneath the surface.".to_string(),
                reasons: vec!["Hidden Combination".to_string(), format!("{} transformation", elem.hangul())],
                transformed_element: Some(elem),
            });
        }
        for (ma, p1, p2) in &myung_am_combinations {
            let elem = ma.combination.transformed_element();
            mapped_relationships.push(RelationshipDetail {
                relation_type: "MyungAmHap".to_string(),
                name: format!("Manifest-Hidden Combo ({})", ma.combination.hangul()),
                positions: vec![p1.clone(), p2.clone()],
                level: InterpretationLevel::Auspicious,
                summary: "Bond between heavenly stem and branch hidden stem".to_string(),
                description: "Strong bond linking overt willpower with covert ambient factors, denoting an inseparable connection.".to_string(),
                reasons: vec!["Manifest-Hidden Combination".to_string(), format!("{} transformation", elem.hangul())],
                transformed_element: Some(elem),
            });
        }

        Self {
            mapped_relationships,
            stem_combinations,
            stem_clashes,
            triple_combinations,
            seasonal_combinations,
            dominant_semi_combinations,
            weak_semi_combinations,
            six_combinations,
            branch_clashes,
            branch_punishments,
            branch_harms,
            branch_destructions,
            am_combinations,
            myung_am_combinations,
        }
    }

    /// Check hidden combinations between earthly branches
    pub fn check_am_combinations(b1: EarthlyBranch, b2: EarthlyBranch) -> Vec<StemCombination> {
        let jj1 = b1.jijanggan();
        let jj2 = b2.jijanggan();
        let mut results = Vec::new();
        for s1 in &jj1 {
            for s2 in &jj2 {
                if let Some(comb) = StemCombination::check(*s1, *s2) {
                    results.push(comb);
                }
            }
        }
        results.sort_by_key(|c| c.hangul());
        results.dedup();
        results
    }

    /// Check manifest-hidden combinations between stem and branch hidden stems
    pub fn check_myung_am_combinations(
        stem: HeavenlyStem,
        branch: EarthlyBranch,
    ) -> Vec<StemCombination> {
        let jj = branch.jijanggan();
        let mut results = Vec::new();
        for s in &jj {
            if let Some(comb) = StemCombination::check(stem, *s) {
                results.push(comb);
            }
        }
        results.sort_by_key(|c| c.hangul());
        results.dedup();
        results
    }

    /// Check if any combinations exist
    pub fn has_combinations(&self) -> bool {
        !self.stem_combinations.is_empty()
            || !self.triple_combinations.is_empty()
            || !self.seasonal_combinations.is_empty()
            || !self.dominant_semi_combinations.is_empty()
            || !self.weak_semi_combinations.is_empty()
            || !self.six_combinations.is_empty()
    }

    /// Check if any clashes exist
    pub fn has_clashes(&self) -> bool {
        !self.stem_clashes.is_empty() || !self.branch_clashes.is_empty()
    }
}

impl std::fmt::Display for RelationshipAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[Branch & Stem Interactions]")?;
        writeln!(f, "---------------------------------")?;

        if !self.stem_combinations.is_empty() {
            write!(f, "Stem Combinations: ")?;
            for (i, (comb, p1, p2)) in self.stem_combinations.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{} ({}-{})", comb.hangul(), p1, p2)?;
            }
            writeln!(f)?;
        }

        if !self.stem_clashes.is_empty() {
            write!(f, "Stem Clashes: ")?;
            for (i, (clash, p1, p2)) in self.stem_clashes.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{} ({}-{})", clash.hangul(), p1, p2)?;
            }
            writeln!(f)?;
        }

        if !self.triple_combinations.is_empty() {
            write!(f, "Triple Alliances: ")?;
            for (i, triple) in self.triple_combinations.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", triple.hangul())?;
            }
            writeln!(f)?;
        }

        if !self.seasonal_combinations.is_empty() {
            write!(f, "Seasonal Alliances: ")?;
            for (i, seasonal) in self.seasonal_combinations.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", seasonal.hangul())?;
            }
            writeln!(f)?;
        }

        if !self.dominant_semi_combinations.is_empty() {
            write!(f, "Dominant Semi-Alliances: ")?;
            for (i, (semi, p1, p2)) in self.dominant_semi_combinations.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{} ({}-{})", semi.hangul(), p1, p2)?;
            }
            writeln!(f)?;
        }

        if !self.weak_semi_combinations.is_empty() {
            write!(f, "Weak Semi-Alliances: ")?;
            for (i, (semi, p1, p2)) in self.weak_semi_combinations.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{} ({}-{})", semi.hangul(), p1, p2)?;
            }
            writeln!(f)?;
        }

        if !self.six_combinations.is_empty() {
            write!(f, "Six Combinations: ")?;
            for (i, (six, p1, p2)) in self.six_combinations.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{} ({}-{})", six.hangul(), p1, p2)?;
            }
            writeln!(f)?;
        }

        if !self.branch_clashes.is_empty() {
            write!(f, "Branch Clashes: ")?;
            for (i, (clash, p1, p2)) in self.branch_clashes.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{} ({}-{})", clash.hangul(), p1, p2)?;
            }
            writeln!(f)?;
        }

        if !self.branch_punishments.is_empty() {
            write!(f, "Branch Punishments: ")?;
            for (i, (pun, p1, p2)) in self.branch_punishments.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{} ({}-{})", pun.hangul(), p1, p2)?;
            }
            writeln!(f)?;
        }

        if !self.branch_harms.is_empty() {
            write!(f, "Branch Harms: ")?;
            for (i, (harm, p1, p2)) in self.branch_harms.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{} ({}-{})", harm.hangul(), p1, p2)?;
            }
            writeln!(f)?;
        }

        if !self.branch_destructions.is_empty() {
            write!(f, "Branch Destructions: ")?;
            for (i, (dest, p1, p2)) in self.branch_destructions.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{} ({}-{})", dest.hangul(), p1, p2)?;
            }
            writeln!(f)?;
        }

        if !self.am_combinations.is_empty() {
            write!(f, "Hidden Combinations: ")?;
            for (i, (am, p1, p2)) in self.am_combinations.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{} ({}-{})", am.combination.hangul(), p1, p2)?;
            }
            writeln!(f)?;
        }

        if !self.myung_am_combinations.is_empty() {
            write!(f, "Manifest-Hidden Combinations: ")?;
            for (i, (ma, p1, p2)) in self.myung_am_combinations.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{} ({}-{})", ma.combination.hangul(), p1, p2)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

// ============================================
// FourPillars helper methods
// ============================================

impl FourPillars {
    /// Relationship analysis
    pub fn relationships(&self) -> RelationshipAnalysis {
        RelationshipAnalysis::from_pillars(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::pillars::SajuInput;

    #[test]
    fn test_user_relationships() {
        // Chart: Jia-Shen Year, Yi-Hai Month, Geng-Xu Day, Ding-Hai Hour
        let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
        let pillars = FourPillars::calculate(&input).unwrap();

        let analysis = pillars.relationships();

        println!("{}", analysis);

        // Verify Yi-Geng combination
        assert!(analysis
            .stem_combinations
            .iter()
            .any(|(c, _, _)| *c == StemCombination::YiGeng));

        // Verify Jia-Geng clash
        assert!(analysis
            .stem_clashes
            .iter()
            .any(|(c, _, _)| *c == StemClash::JiaGeng));

        // Verify Hai-Hai self-punishment
        assert!(analysis
            .branch_punishments
            .iter()
            .any(|(p, _, _)| matches!(p, BranchPunishment::SelfPunishment(EarthlyBranch::Hai))));

        // Verify Shen-Hai harm
        assert!(analysis
            .branch_harms
            .iter()
            .any(|(h, _, _)| *h == BranchHarm::ShenHai));
    }

    #[test]
    fn test_stem_combination() {
        assert!(StemCombination::check(HeavenlyStem::Yi, HeavenlyStem::Geng).is_some());
        assert!(StemCombination::check(HeavenlyStem::Jia, HeavenlyStem::Ji).is_some());
    }

    #[test]
    fn test_branch_clash() {
        assert!(BranchClash::check(EarthlyBranch::Zi, EarthlyBranch::Wu).is_some());
        assert!(BranchClash::check(EarthlyBranch::Yin, EarthlyBranch::Shen).is_some());
    }
}
