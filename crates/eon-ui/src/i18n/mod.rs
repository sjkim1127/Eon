// crates/eon-ui/src/i18n/mod.rs
// Zero-dependency, compile-time static i18n for Eon UI.
// Locale persisted in global Signal<Locale> + localStorage.

pub mod en;
pub mod ko;
pub mod ru;
pub mod zh;

use serde::{Deserialize, Serialize};

/// Supported UI locales.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Locale {
    #[default]
    Ko,
    En,
    Zh,
    Ru,
}

impl Locale {
    pub fn label(self) -> &'static str {
        match self {
            Locale::Ko => "🇰🇷 한국어",
            Locale::En => "🇺🇸 English",
            Locale::Zh => "🇨🇳 中文",
            Locale::Ru => "🇷🇺 Русский",
        }
    }

    pub fn flag(self) -> &'static str {
        match self {
            Locale::Ko => "🇰🇷",
            Locale::En => "🇺🇸",
            Locale::Zh => "🇨🇳",
            Locale::Ru => "🇷🇺",
        }
    }

    pub fn from_code(code: &str) -> Self {
        match code {
            "en" => Locale::En,
            "zh" => Locale::Zh,
            "ru" => Locale::Ru,
            _ => Locale::Ko,
        }
    }

    pub fn code(self) -> &'static str {
        match self {
            Locale::Ko => "ko",
            Locale::En => "en",
            Locale::Zh => "zh",
            Locale::Ru => "ru",
        }
    }

    pub fn all() -> &'static [Locale] {
        &[Locale::Ko, Locale::En, Locale::Zh, Locale::Ru]
    }
}

/// All translatable UI string keys.
/// Domain-specific proper nouns (천간, 지지, Nakshatra names, etc.)
/// are NOT included — those stay in their original language.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TK {
    // ── Navigation ─────────────────────────────────────────────────────
    NavSaju,
    NavVedic,
    NavStrength,
    NavTransit,
    NavSimulation,
    NavTier,
    NavAi,

    // ── Form ───────────────────────────────────────────────────────────
    FormYear,
    FormMonth,
    FormDay,
    FormHour,
    FormMinute,
    FormBirthplace,
    FormBirthplacePlaceholder,
    FormCityPlaceholder,
    FormLunar,
    FormMale,
    FormSavedProfiles,
    FormLoadProfile,
    FormSaveProfile,
    FormProfileNamePlaceholder,
    FormSaveBtn,
    FormAnalyzeBtn,

    // ── Status ─────────────────────────────────────────────────────────
    StatusIdle,
    StatusIdleHint,
    StatusLoading,
    StatusLoadingSaju,
    StatusLoadingVedic,
    StatusLoadingTransit,
    StatusLoadingTier,
    StatusLoadingStrength,
    StatusError,
    StatusNoData,

    // ── Geo / Search ───────────────────────────────────────────────────
    GeoSearching,
    GeoNoResult,
    GeoParseError,
    GeoNetworkError,
    GeoSearchComplete,

    // ── Section Headings ───────────────────────────────────────────────
    SectionSajuChart,
    SectionTenGods,
    SectionFortune,
    SectionSpirit,
    SectionComplexity,
    SectionVedicChart,
    SectionStrength,
    SectionTransit,
    SectionSimulation,
    SectionTier,
    SectionAi,
    SectionDasha,
    SectionYoga,
    SectionAshtakavarga,
    SectionPanchanga,
    SectionCompatibility,

    // ── Common Buttons / Labels ────────────────────────────────────────
    BtnClose,
    BtnCopy,
    BtnReset,
    BtnAnalyze,
    BtnCalculate,
    BtnViewMore,
    BtnStyleSouth,
    BtnStyleNorth,
    LabelMale,
    LabelFemale,
    LabelYear,
    LabelMonth,
    LabelDay,
    LabelHour,
    LabelMinute,
    LabelAscendant,
    LabelPlanet,
    LabelDegree,
    LabelHouse,
    LabelNakshatra,
    LabelLord,
    LabelScore,
    LabelTotal,
    LabelStrong,
    LabelWeak,
    LabelNeutral,
    LabelRetrograde,
    LabelCombust,
    LabelExalted,
    LabelDebilitated,
    LabelPada,
    LabelDeity,
    LabelPurpose,

    // ── Saju-specific ─────────────────────────────────────────────────
    SajuHourPillar,
    SajuDayPillar,
    SajuMonthPillar,
    SajuYearPillar,
    SajuDayMaster,
    SajuAnalyzeBtn,

    // ── Vedic-specific ────────────────────────────────────────────────
    VedicD1Chart,
    VedicVargaChart,
    VedicPartnerYear,
    VedicPartnerMonth,
    VedicPartnerDay,
    VedicPartnerHour,
    VedicPartnerMinute,
    VedicCompatCheck,
    VedicStyleToggle,
    VedicSubtabD1,
    VedicSubtabAshtakavarga,
    VedicSubtabDasha,
    VedicSubtabGochara,
    VedicSubtabYoga,
    VedicSubtabJaimini,
    VedicSubtabVarga,
    VedicLagnaRasi,
    VedicHouseScore,

    // ── Tier ──────────────────────────────────────────────────────────
    TierTitle,
    TierRank,
    TierScore,
    TierGrade,

    // ── Tooltip strings ───────────────────────────────────────────────
    TooltipRetrograde,
    TooltipCombust,
    TooltipExalted,
    TooltipDebilitated,
    TooltipDegree,
    TooltipNakshatra,
    TooltipLord,
    TooltipHouseNum,
    TooltipPlanetsIn,
    TooltipHouseLord,
    TooltipHouseScore,
    TooltipClickDetail,
}

/// Main translation dispatch function.
/// Usage: `t(locale, TK::NavSaju)`
pub fn t(locale: Locale, key: TK) -> &'static str {
    match locale {
        Locale::Ko => ko::translate(key),
        Locale::En => en::translate(key),
        Locale::Zh => zh::translate(key),
        Locale::Ru => ru::translate(key),
    }
}
