//! 합충형해(合沖刑害) 분석
//!
//! 천간과 지지 간의 특수 관계를 분석합니다.
//!
//! ## 천간 관계
//! - **천간합(天干合)**: 甲己, 乙庚, 丙辛, 丁壬, 戊癸
//! - **천간충(天干沖)**: 甲庚, 乙辛, 丙壬, 丁癸
//!
//! ## 지지 관계
//! - **삼합(三合)**: 寅午戌(火), 申子辰(水), 巳酉丑(金), 亥卯未(木)
//! - **반합(半合)**: 삼합 중 두 개
//! - **방합(方合)**: 寅卯辰(東), 巳午未(南), 申酉戌(西), 亥子丑(北)
//! - **육합(六合)**: 子丑, 寅亥, 卯戌, 辰酉, 巳申, 午未
//! - **충(沖)**: 子午, 丑未, 寅申, 卯酉, 辰戌, 巳亥
//! - **형(刑)**: 삼형(寅巳申), 상형(丑戌未), 자형(辰午酉亥)
//! - **해(害)**: 子未, 丑午, 寅巳, 卯辰, 申亥, 酉戌
//! - **파(破)**: 子酉, 丑辰, 寅亥, 卯午, 巳申, 午未

use crate::analysis::supplementary_pillars::InterpretationLevel;
use crate::core::branch::EarthlyBranch;
use crate::core::element::Element;
use crate::core::pillars::FourPillars;
use crate::core::stem::HeavenlyStem;
use serde::{Deserialize, Serialize};

// ============================================
// 천간 합 (天干合)
// ============================================

/// 천간합의 종류
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
    /// 두 천간이 합인지 확인
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

    /// 합화 오행 (합이 되면 변하는 오행)
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
            Self::JiaJi => "갑기합",
            Self::YiGeng => "을경합",
            Self::BingXin => "병신합",
            Self::DingRen => "정임합",
            Self::WuGui => "무계합",
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
// 천간 충 (天干沖)
// ============================================

/// 천간충의 종류
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
    /// 두 천간이 충인지 확인
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
            Self::JiaGeng => "갑경충",
            Self::YiXin => "을신충",
            Self::BingRen => "병임충",
            Self::DingGui => "정계충",
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
// 지지 삼합 (三合)
// ============================================

/// 삼합의 종류
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
    /// 세 지지 목록에서 발견된 모든 삼합 확인
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

    /// 합화 오행
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
            Self::YinWuXu => "인오술 화국",
            Self::ShenZiChen => "신자진 수국",
            Self::SiYouChou => "사유축 금국",
            Self::HaiMaoWei => "해묘미 목국",
        }
    }

    /// 구성 지지 반환
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
// 지지 반합 (半合)
// ============================================

/// 반합의 종류 (삼합 중 2개)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SemiCombination {
    /// 寅午 (火局 반합)
    YinWu,
    /// 午戌 (火局 반합)
    WuXu,
    /// 寅戌 (火局 반합 - 중심 없음)
    YinXu,
    /// 申子 (水局 반합)
    ShenZi,
    /// 子辰 (水局 반합)
    ZiChen,
    /// 申辰 (水局 반합 - 중심 없음)
    ShenChen,
    /// 巳酉 (金局 반합)
    SiYou,
    /// 酉丑 (金局 반합)
    YouChou,
    /// 巳丑 (金局 반합 - 중심 없음)
    SiChou,
    /// 亥卯 (木局 반합)
    HaiMao,
    /// 卯未 (木局 반합)
    MaoWei,
    /// 亥未 (木局 반합 - 중심 없음)
    HaiWei,
}

impl SemiCombination {
    /// 두 지지가 반합인지 확인
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
            Self::YinWu => "인오반합",
            Self::WuXu => "오술반합",
            Self::YinXu => "인술반합",
            Self::ShenZi => "신자반합",
            Self::ZiChen => "자진반합",
            Self::ShenChen => "신진반합",
            Self::SiYou => "사유반합",
            Self::YouChou => "유축반합",
            Self::SiChou => "사축반합",
            Self::HaiMao => "해묘반합",
            Self::MaoWei => "묘미반합",
            Self::HaiWei => "해미반합",
        }
    }

    /// 왕지(子午卯酉)를 포함한 유력한 반합인지 확인
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
// 지지 방합 (方合)
// ============================================

/// 방합의 종류 (계절의 결합)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SeasonalCombination {
    /// 寅卯辰 → 木局 (봄)
    YinMaoChen,
    /// 巳午未 → 火局 (여름)
    SiWuWei,
    /// 申酉戌 → 金局 (가을)
    ShenYouXu,
    /// 亥子丑 → 水局 (겨울)
    HaiZiChou,
}

impl SeasonalCombination {
    /// 세 지지 목록에서 발견된 모든 방합 확인
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

    /// 방합에 의한 오행
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
            Self::YinMaoChen => "인묘진 목방",
            Self::SiWuWei => "사오미 화방",
            Self::ShenYouXu => "신유술 금방",
            Self::HaiZiChou => "해자축 수방",
        }
    }

    /// 구성 지지 반환
    pub fn branches(&self) -> [EarthlyBranch; 3] {
        use EarthlyBranch::*;
        match self {
            Self::YinMaoChen => [Yin, Mao, Chen],
            Self::SiWuWei => [Si, Wu, Wei],
            Self::ShenYouXu => [Shen, You, Xu],
            Self::HaiZiChou => [Hai, Zi, Chou],
        }
    }
}

// ============================================
// 지지 육합 (六合)
// ============================================

/// 육합의 종류
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
    /// 두 지지가 육합인지 확인
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

    /// 합화 오행 (육합으로 변하는 오행)
    pub const fn transformed_element(&self) -> Option<Element> {
        match self {
            Self::ZiChou => Some(Element::Earth),  // 자축합토
            Self::YinHai => Some(Element::Wood),   // 인해합목
            Self::MaoXu => Some(Element::Fire),    // 묘술합화
            Self::ChenYou => Some(Element::Metal), // 진유합금
            Self::SiShen => Some(Element::Water),  // 사신합수
            Self::WuWei => Some(Element::Fire),    // 오미합화 (불의 세력)
        }
    }

    pub const fn hangul(&self) -> &'static str {
        match self {
            Self::ZiChou => "자축합",
            Self::YinHai => "인해합",
            Self::MaoXu => "묘술합",
            Self::ChenYou => "진유합",
            Self::SiShen => "사신합",
            Self::WuWei => "오미합",
        }
    }
}

// ============================================
// 지지 충 (沖)
// ============================================

/// 충(沖)의 종류에 따른 분류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ClashType {
    /// 왕지충(旺地沖) - 子午, 卯酉
    /// 순수 오행 충돌로 뿌리 손상이 가장 큼
    Cardinal,
    /// 생지충(生地沖) - 寅申, 巳亥
    /// 복합 지장간으로 일부만 손상
    Growth,
    /// 고지충(庫地沖/붕충) - 丑未, 辰戌
    /// 토(土)끼리 충, 붕토 효과로 손상 적음
    Storage,
}

impl ClashType {
    /// 한글 이름
    pub const fn hangul(&self) -> &'static str {
        match self {
            Self::Cardinal => "왕지충",
            Self::Growth => "생지충",
            Self::Storage => "고지충",
        }
    }

    /// 뿌리 손상율 (0.0 ~ 1.0)
    /// 이 비율만큼 통근 가중치가 감산됨
    pub const fn damage_ratio(&self) -> f32 {
        match self {
            Self::Cardinal => 0.70, // 왕지충: 70% 손상
            Self::Growth => 0.50,   // 생지충: 50% 손상
            Self::Storage => 0.20,  // 고지충: 20% 손상 (붕토 효과)
        }
    }
}

/// 지지충의 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BranchClash {
    /// 子午沖
    ZiWu,
    /// 丑未沖
    ChouWei,
    /// 寅申沖
    YinShen,
    /// 卯酉沖
    MaoYou,
    /// 辰戌沖
    ChenXu,
    /// 巳亥沖
    SiHai,
}

impl BranchClash {
    /// 두 지지가 충인지 확인
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

    /// 충의 종류 (왕지충, 생지충, 고지충)
    pub const fn clash_type(&self) -> ClashType {
        match self {
            // 왕지충: 子午(수화), 卯酉(목금) - 순수 오행 정충
            Self::ZiWu | Self::MaoYou => ClashType::Cardinal,
            // 생지충: 寅申(목금), 巳亥(화수) - 복합 지장간
            Self::YinShen | Self::SiHai => ClashType::Growth,
            // 고지충: 丑未, 辰戌 - 토끼리 붕충
            Self::ChouWei | Self::ChenXu => ClashType::Storage,
        }
    }

    /// 뿌리 손상율 (충 종류에 따른 차등 감산)
    pub const fn damage_ratio(&self) -> f32 {
        self.clash_type().damage_ratio()
    }

    pub const fn hangul(&self) -> &'static str {
        match self {
            Self::ZiWu => "자오충",
            Self::ChouWei => "축미충",
            Self::YinShen => "인신충",
            Self::MaoYou => "묘유충",
            Self::ChenXu => "진술충",
            Self::SiHai => "사해충",
        }
    }
}

// ============================================
// 지지 형 (刑)
// ============================================

/// 지지형의 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BranchPunishment {
    /// 삼형 (寅巳申) - 무은지형
    TriplePunishment,
    /// 상형 (丑戌未) - 무례지형
    MutualPunishment,
    /// 자형 (辰午酉亥) - 각자 자신
    SelfPunishment(EarthlyBranch),
}

impl BranchPunishment {
    /// 자형인지 확인
    pub fn is_self_punishment(branch: EarthlyBranch) -> bool {
        use EarthlyBranch::*;
        matches!(branch, Chen | Wu | You | Hai)
    }

    /// 두 지지가 같고 자형인지 확인
    pub fn check_self(b1: EarthlyBranch, b2: EarthlyBranch) -> Option<Self> {
        if b1 == b2 && Self::is_self_punishment(b1) {
            Some(Self::SelfPunishment(b1))
        } else {
            None
        }
    }

    /// 삼형살(寅巳申) 체크
    pub fn check_triple(branches: &[EarthlyBranch]) -> bool {
        use EarthlyBranch::*;
        branches.contains(&Yin) && branches.contains(&Si) && branches.contains(&Shen)
    }

    /// 상형살(丑戌未) 체크
    pub fn check_mutual(branches: &[EarthlyBranch]) -> bool {
        use EarthlyBranch::*;
        branches.contains(&Chou) && branches.contains(&Xu) && branches.contains(&Wei)
    }

    pub fn hangul(&self) -> String {
        match self {
            Self::TriplePunishment => "삼형(인사신)".to_string(),
            Self::MutualPunishment => "상형(축술미)".to_string(),
            Self::SelfPunishment(b) => format!("{}자형", b.hangul()),
        }
    }
}

// ============================================
// 지지 파 (破)
// ============================================

/// 지지파의 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BranchDestruction {
    /// 子酉破
    ZiYou,
    /// 卯午破
    MaoWu,
    /// 辰丑破
    ChenChou,
    /// 未戌破
    WeiXu,
    /// 寅亥破 (육합과 겹침)
    YinHai,
    /// 巳申破 (육합과 겹침)
    SiShen,
}

impl BranchDestruction {
    /// 두 지지가 파인지 확인
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
            Self::ZiYou => "자유파",
            Self::MaoWu => "묘오파",
            Self::ChenChou => "진축파",
            Self::WeiXu => "미술파",
            Self::YinHai => "인해파",
            Self::SiShen => "사신파",
        }
    }
}

// ============================================
// 지지 해 (害)
// ============================================

/// 지지해의 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BranchHarm {
    /// 子未害
    ZiWei,
    /// 丑午害
    ChouWu,
    /// 寅巳害
    YinSi,
    /// 卯辰害
    MaoChen,
    /// 申亥害
    ShenHai,
    /// 酉戌害
    YouXu,
}

impl BranchHarm {
    /// 두 지지가 해인지 확인
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
            Self::ZiWei => "자미해",
            Self::ChouWu => "축오해",
            Self::YinSi => "인사해",
            Self::MaoChen => "묘진해",
            Self::ShenHai => "신해해",
            Self::YouXu => "유술해",
        }
    }
}

// ============================================
// 암합 (暗合) 및 명암합 (明暗合)
// ============================================

/// 암합 (暗合, 지장간끼리의 합)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Amhap {
    pub branches: (EarthlyBranch, EarthlyBranch),
    pub combination: StemCombination,
}

/// 명암합 (明暗合, 천간과 지지 속 지장간의 합)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MyungAmHap {
    pub stem: HeavenlyStem,
    pub branch: EarthlyBranch,
    pub combination: StemCombination,
}

/// 발견된 관계 정보 (레거시)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoundRelation {
    pub description: String,
    pub positions: (String, String),
}

/// 합충형해 상세 정보 (Explainable Relationship)
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

/// 합충형해 분석 결과
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipAnalysis {
    /// 발견된 모든 합충형해 상세 정보 (고도의 설명 가능성 포함)
    pub mapped_relationships: Vec<RelationshipDetail>,
    /// 천간합
    pub stem_combinations: Vec<(StemCombination, String, String)>,
    /// 천간충
    pub stem_clashes: Vec<(StemClash, String, String)>,
    /// 삼합
    pub triple_combinations: Vec<TripleCombination>,
    /// 방합
    pub seasonal_combinations: Vec<SeasonalCombination>,
    /// 유력한 반합 (왕지 포함, 眞合)
    pub dominant_semi_combinations: Vec<(SemiCombination, String, String)>,
    /// 무력한 반합 (왕지 미포함, 假合)
    pub weak_semi_combinations: Vec<(SemiCombination, String, String)>,
    /// 육합
    pub six_combinations: Vec<(SixCombination, String, String)>,
    /// 지지충
    pub branch_clashes: Vec<(BranchClash, String, String)>,
    /// 지지형
    pub branch_punishments: Vec<(BranchPunishment, String, String)>,
    /// 지지해
    pub branch_harms: Vec<(BranchHarm, String, String)>,
    /// 지지파
    pub branch_destructions: Vec<(BranchDestruction, String, String)>,
    /// 암합 (지장간 사이의 합)
    pub am_combinations: Vec<(Amhap, String, String)>,
    /// 명암합 (드러난 천간과 지장간 사이의 합)
    pub myung_am_combinations: Vec<(MyungAmHap, String, String)>,
}

impl RelationshipAnalysis {
    /// 사주에서 합충형해 분석
    pub fn from_pillars(pillars: &FourPillars) -> Self {
        let stems = [
            ("년간", pillars.year.stem),
            ("월간", pillars.month.stem),
            ("일간", pillars.day.stem),
            ("시간", pillars.hour.stem),
        ];

        let branches = [
            ("년지", pillars.year.branch),
            ("월지", pillars.month.branch),
            ("일지", pillars.day.branch),
            ("시지", pillars.hour.branch),
        ];

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

        // 천간 분석 (모든 쌍)
        for i in 0..4 {
            for j in (i + 1)..4 {
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

        // 지지 분석 (모든 쌍)
        for i in 0..4 {
            for j in (i + 1)..4 {
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

        // 자형 분석 (같은 지지가 2개 이상)
        for i in 0..4 {
            for j in (i + 1)..4 {
                let (pos1, branch1) = branches[i];
                let (pos2, branch2) = branches[j];
                if let Some(pun) = BranchPunishment::check_self(branch1, branch2) {
                    branch_punishments.push((pun, pos1.to_string(), pos2.to_string()));
                }
            }
        }

        // 삼형살 및 상형살 분석
        let all_branches: Vec<_> = branches.iter().map(|(_, b)| *b).collect();
        if BranchPunishment::check_triple(&all_branches) {
            branch_punishments.push((
                BranchPunishment::TriplePunishment,
                "삼형".to_string(),
                "인사신".to_string(),
            ));
        }
        if BranchPunishment::check_mutual(&all_branches) {
            branch_punishments.push((
                BranchPunishment::MutualPunishment,
                "상형".to_string(),
                "축술미".to_string(),
            ));
        }

        // 삼합 및 방합 분석
        let all_branches: Vec<_> = branches.iter().map(|(_, b)| *b).collect();
        let triple_combinations = TripleCombination::check(&all_branches);
        let seasonal_combinations = SeasonalCombination::check(&all_branches);

        // 암합 분석 (지지 간의 지장간 합)
        for i in 0..4 {
            for j in (i + 1)..4 {
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

        // 명암합 분석 (천간과 지지 지장간의 합)
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

        // === 상세 설명 모델(mapped_relationships) 생성 ===
        let mut mapped_relationships = Vec::new();

        // 1. 지지충 (BranchClash) - 파괴력 1순위
        for (clash, p1, p2) in &branch_clashes {
            let info = clash.clash_type();
            mapped_relationships.push(RelationshipDetail {
                relation_type: "BranchClash".to_string(),
                name: clash.hangul().to_string(),
                positions: vec![p1.clone(), p2.clone()],
                level: InterpretationLevel::Caution,
                summary: format!("{}으로 두 기운이 정면 충돌", clash.hangul()),
                description: format!(
                    "{}에 해당하는 충입니다. 뿌리 기운이 {}% 가량 흔들리며 강한 변화를 동반합니다.",
                    info.hangul(),
                    (info.damage_ratio() * 100.0) as u32
                ),
                reasons: vec![format!("{} ({})", info.hangul(), info.damage_ratio())],
                transformed_element: None,
            });
        }

        // 2. 천간합 (StemCombination)
        for (comb, p1, p2) in &stem_combinations {
            let elem = comb.transformed_element();
            mapped_relationships.push(RelationshipDetail {
                relation_type: "StemCombination".to_string(),
                name: comb.hangul().to_string(),
                positions: vec![p1.clone(), p2.clone()],
                level: InterpretationLevel::Auspicious,
                summary: format!("{}으로 새로운 기운 형성", comb.hangul()),
                description: format!(
                    "두 천간이 결합하여 {}의 성질로 변화하려는 흐름이 생깁니다.",
                    elem.hangul()
                ),
                reasons: vec![format!("{} 합화", elem.hangul())],
                transformed_element: Some(elem),
            });
        }

        // 3. 육합 (SixCombination)
        for (six, p1, p2) in &six_combinations {
            let elem = six.transformed_element();
            mapped_relationships.push(RelationshipDetail {
                relation_type: "SixCombination".to_string(),
                name: six.hangul().to_string(),
                positions: vec![p1.clone(), p2.clone()],
                level: InterpretationLevel::Auspicious,
                summary: format!("{}으로 지지의 화합", six.hangul()),
                description: "서로 다른 지지의 기운이 조화롭게 묶여 안정감을 줍니다.".to_string(),
                reasons: vec!["육합(六合)".to_string()],
                transformed_element: elem,
            });
        }

        // 4. 삼합 (TripleCombination)
        for triple in &triple_combinations {
            let elem = triple.element();
            mapped_relationships.push(RelationshipDetail {
                relation_type: "TripleCombination".to_string(),
                name: triple.hangul().to_string(),
                positions: vec!["전체".to_string()], // 삼합은 전체 분포
                level: InterpretationLevel::Auspicious,
                summary: format!("{} 성립으로 {} 기운 강화", triple.hangul(), elem.hangul()),
                description: "세 지지가 목적 의식을 가지고 하나의 강력한 오행 국(局)을 형성합니다."
                    .to_string(),
                reasons: vec!["삼합 완결".to_string()],
                transformed_element: Some(elem),
            });
        }

        // 5. 방합 (SeasonalCombination)
        for seasonal in &seasonal_combinations {
            let elem = seasonal.element();
            mapped_relationships.push(RelationshipDetail {
                relation_type: "SeasonalCombination".to_string(),
                name: seasonal.hangul().to_string(),
                positions: vec!["전체".to_string()],
                level: InterpretationLevel::Auspicious,
                summary: format!("{} 성립으로 계절적 세력 결집", seasonal.hangul()),
                description:
                    "계절의 힘을 상징하는 지지들이 모여 해당 오행의 세력이 매우 강력해집니다."
                        .to_string(),
                reasons: vec!["방합(方合)".to_string()],
                transformed_element: Some(elem),
            });
        }

        // 6. 형/해/파 (Punishments, Harms, Destructions)
        for (pun, p1, p2) in &branch_punishments {
            mapped_relationships.push(RelationshipDetail {
                relation_type: "BranchPunishment".to_string(),
                name: pun.hangul(),
                positions: vec![p1.clone(), p2.clone()],
                level: InterpretationLevel::Caution,
                summary: "지지 사이의 복잡한 간섭(刑)".to_string(),
                description: "기운이 매끄럽지 못하게 얽히거나 조정되는 과정에서 스트레스가 발생할 수 있습니다.".to_string(),
                reasons: vec!["형살(刑殺)".to_string()],
                transformed_element: None,
            });
        }
        for (harm, p1, p2) in &branch_harms {
            mapped_relationships.push(RelationshipDetail {
                relation_type: "BranchHarm".to_string(),
                name: harm.hangul().to_string(),
                positions: vec![p1.clone(), p2.clone()],
                level: InterpretationLevel::Caution,
                summary: "서로의 기운을 해치거나 방해함(害)".to_string(),
                description: "육합을 방해하는 기운들이 만나 실익이 줄어들거나 갈등이 생집니다."
                    .to_string(),
                reasons: vec!["해살(害殺)".to_string()],
                transformed_element: None,
            });
        }

        // 7. 반합 (眞/假)
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
                summary: format!("{} 성립으로 {} 기운 발동", semi.hangul(), elem.hangul()),
                description: "왕지(子午卯酉)를 포함한 강력한 결합입니다. 삼합에 준하는 실질적인 오행 변화가 일어납니다.".to_string(),
                reasons: vec!["진반합(眞半合)".to_string(), format!("{} 포함", elem.hangul())],
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
                name: format!("{}(假)", semi.hangul()),
                positions: vec![p1.clone(), p2.clone()],
                level: InterpretationLevel::Neutral,
                summary: format!("{} 형성 (세력 미약)", semi.hangul()),
                description: "중심 기운인 왕지가 빠진 결합입니다. 작용력은 약하지만 해당 오행으로 향하려는 경향성을 띱니다.".to_string(),
                reasons: vec!["가반합(假半合)".to_string(), "왕지 미포함".to_string()],
                transformed_element: Some(elem),
            });
        }

        // 8. 파 (BranchDestruction)
        for (dest, p1, p2) in &branch_destructions {
            mapped_relationships.push(RelationshipDetail {
                relation_type: "BranchDestruction".to_string(),
                name: dest.hangul().to_string(),
                positions: vec![p1.clone(), p2.clone()],
                level: InterpretationLevel::Caution,
                summary: "기운의 분열과 파괴(破)".to_string(),
                description: "완성된 기운을 깨뜨리거나 내부적인 분열을 야기할 수 있는 작용입니다. 세밀한 조정이 필요합니다.".to_string(),
                reasons: vec!["파살(破殺)".to_string()],
                transformed_element: None,
            });
        }

        // 9. 암합 / 명암합
        for (am, p1, p2) in &am_combinations {
            let elem = am.combination.transformed_element();
            mapped_relationships.push(RelationshipDetail {
                relation_type: "Amhap".to_string(),
                name: format!("암합({})", am.combination.hangul()),
                positions: vec![p1.clone(), p2.clone()],
                level: InterpretationLevel::Auspicious,
                summary: "지장간끼리의 비밀스러운 결합".to_string(),
                description: "겉으로 드러나지 않는 은밀한 유대감이나 애정, 혹은 보이지 않는 곳에서의 협력을 의미합니다.".to_string(),
                reasons: vec!["암합(暗合)".to_string(), format!("{} 합화", elem.hangul())],
                transformed_element: Some(elem),
            });
        }
        for (ma, p1, p2) in &myung_am_combinations {
            let elem = ma.combination.transformed_element();
            mapped_relationships.push(RelationshipDetail {
                relation_type: "MyungAmHap".to_string(),
                name: format!("명암합({})", ma.combination.hangul()),
                positions: vec![p1.clone(), p2.clone()],
                level: InterpretationLevel::Auspicious,
                summary: "천간과 지지 속 지장간의 결합".to_string(),
                description: "드러난 의지와 보이지 않는 환경 사이의 강한 유대입니다. 떼려야 뗄 수 없는 밀접한 관계를 뜻합니다.".to_string(),
                reasons: vec!["명암합(明暗合)".to_string(), format!("{} 합화", elem.hangul())],
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

    /// 지지 간의 암합(지장간 합) 체크
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

    /// 천간과 지지 지장간 사이의 명암합 체크
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

    /// 합이 있는지 확인
    pub fn has_combinations(&self) -> bool {
        !self.stem_combinations.is_empty()
            || !self.triple_combinations.is_empty()
            || !self.seasonal_combinations.is_empty()
            || !self.dominant_semi_combinations.is_empty()
            || !self.weak_semi_combinations.is_empty()
            || !self.six_combinations.is_empty()
    }

    /// 충이 있는지 확인
    pub fn has_clashes(&self) -> bool {
        !self.stem_clashes.is_empty() || !self.branch_clashes.is_empty()
    }
}

impl std::fmt::Display for RelationshipAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "【합충형해 분석】")?;
        writeln!(f, "─────────────────────────────────")?;

        if !self.stem_combinations.is_empty() {
            write!(f, "천간합: ")?;
            for (i, (comb, p1, p2)) in self.stem_combinations.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{} ({}-{})", comb.hangul(), p1, p2)?;
            }
            writeln!(f)?;
        }

        if !self.stem_clashes.is_empty() {
            write!(f, "천간충: ")?;
            for (i, (clash, p1, p2)) in self.stem_clashes.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{} ({}-{})", clash.hangul(), p1, p2)?;
            }
            writeln!(f)?;
        }

        if !self.triple_combinations.is_empty() {
            write!(f, "삼합: ")?;
            for (i, triple) in self.triple_combinations.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", triple.hangul())?;
            }
            writeln!(f)?;
        }

        if !self.seasonal_combinations.is_empty() {
            write!(f, "방합: ")?;
            for (i, seasonal) in self.seasonal_combinations.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", seasonal.hangul())?;
            }
            writeln!(f)?;
        }

        if !self.dominant_semi_combinations.is_empty() {
            write!(f, "반합(眞): ")?;
            for (i, (semi, p1, p2)) in self.dominant_semi_combinations.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{} ({}-{})", semi.hangul(), p1, p2)?;
            }
            writeln!(f)?;
        }

        if !self.weak_semi_combinations.is_empty() {
            write!(f, "반합(假): ")?;
            for (i, (semi, p1, p2)) in self.weak_semi_combinations.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{} ({}-{})", semi.hangul(), p1, p2)?;
            }
            writeln!(f)?;
        }

        if !self.six_combinations.is_empty() {
            write!(f, "육합: ")?;
            for (i, (six, p1, p2)) in self.six_combinations.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{} ({}-{})", six.hangul(), p1, p2)?;
            }
            writeln!(f)?;
        }

        if !self.branch_clashes.is_empty() {
            write!(f, "지지충: ")?;
            for (i, (clash, p1, p2)) in self.branch_clashes.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{} ({}-{})", clash.hangul(), p1, p2)?;
            }
            writeln!(f)?;
        }

        if !self.branch_punishments.is_empty() {
            write!(f, "지지형: ")?;
            for (i, (pun, p1, p2)) in self.branch_punishments.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{} ({}-{})", pun.hangul(), p1, p2)?;
            }
            writeln!(f)?;
        }

        if !self.branch_harms.is_empty() {
            write!(f, "지지해: ")?;
            for (i, (harm, p1, p2)) in self.branch_harms.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{} ({}-{})", harm.hangul(), p1, p2)?;
            }
            writeln!(f)?;
        }

        if !self.branch_destructions.is_empty() {
            write!(f, "지지파: ")?;
            for (i, (dest, p1, p2)) in self.branch_destructions.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{} ({}-{})", dest.hangul(), p1, p2)?;
            }
            writeln!(f)?;
        }

        if !self.am_combinations.is_empty() {
            write!(f, "암합: ")?;
            for (i, (am, p1, p2)) in self.am_combinations.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{} ({}-{})", am.combination.hangul(), p1, p2)?;
            }
            writeln!(f)?;
        }

        if !self.myung_am_combinations.is_empty() {
            write!(f, "명암합: ")?;
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
// FourPillars 편의 메서드
// ============================================

impl FourPillars {
    /// 합충형해 분석
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
        // 김성주님 사주: 甲申年 乙亥月 庚戌日 丁亥時
        let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
        let pillars = FourPillars::calculate(&input).unwrap();

        let analysis = pillars.relationships();

        println!("{}", analysis);

        // 을경합 확인 (乙-庚)
        assert!(analysis
            .stem_combinations
            .iter()
            .any(|(c, _, _)| *c == StemCombination::YiGeng));

        // 갑경충 확인 (甲-庚)
        assert!(analysis
            .stem_clashes
            .iter()
            .any(|(c, _, _)| *c == StemClash::JiaGeng));

        // 해해자형 확인 (亥-亥) - 월지와 시지가 모두 亥
        assert!(analysis
            .branch_punishments
            .iter()
            .any(|(p, _, _)| matches!(p, BranchPunishment::SelfPunishment(EarthlyBranch::Hai))));

        // 신해해 확인 (申-亥)
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
