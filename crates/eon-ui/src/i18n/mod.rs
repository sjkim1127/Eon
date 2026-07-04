// crates/eon-ui/src/i18n/mod.rs
// Zero-dependency, compile-time static i18n for Eon UI.
// Locale persisted in global Signal<Locale> + localStorage.

pub mod en;
pub mod ko;
pub mod ru;
pub mod zh;
pub mod zwds_interpret;
pub mod iching_db;

use serde::{Deserialize, Serialize};
use eon_vedic::planets::VedicPlanet;
use eon_vedic::analysis::avasthas::LajjitadiAvastha;

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
    NavZwds,
    NavIChing,
    NavWestern,
    NavHumanDesign,

    // ── Western Astrology ──────────────────────────────────────────────
    WestTitle,
    WestPlanet,
    WestSign,
    WestDegree,
    WestHouse,
    WestSpeed,
    WestRetrograde,
    WestAspect,
    WestOrb,
    WestCusp,
    WestElement,
    WestModality,
    WestChartRuler,
    WestHouseSystem,
    WestPlacidus,
    WestKoch,
    WestWholeSign,
    WestEqual,
    WestDominantTitle,
    WestElementsTitle,
    WestModalitiesTitle,
    WestAspectsTitle,
    WestReportCopyBtn,
    WestReportCopiedBtn,

    // ── Human Design ───────────────────────────────────────────────────
    HdTitle,
    HdType,
    HdProfile,
    HdAuthority,
    HdActiveGates,
    HdActiveChannels,
    HdDefinedCenters,
    HdOpenCenters,
    HdDesignSide,
    HdPersonalitySide,
    HdCenterHead,
    HdCenterAjna,
    HdCenterThroat,
    HdCenterSelf,
    HdCenterHeart,
    HdCenterSacral,
    HdCenterRoot,
    HdCenterSpleen,
    HdCenterSolarPlexus,
    HdReportCopyBtn,
    HdReportCopiedBtn,

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
    FormUseNightRatHour,
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
    SectionDasha,
    SectionYoga,
    SectionAshtakavarga,
    SectionPanchanga,
    SectionCompatibility,

    // ── Common Buttons / Labels ────────────────────────────────────────
    BtnClose,
    BtnCopy,
    BtnCopySajuMarkdown,
    BtnCopyVedicMarkdown,
    BtnCopyCombinedMarkdown,
    MsgCopiedToClipboard,
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

    // ── Global / Common ──────────────────────────────────────────────────
    LabelAge,
    LabelSuffixYears,
    LabelNone,
    LabelYes,
    LabelNo,

    // ── Saju Tab ────────────────────────────────────────────────────────
    SajuDeukRyeong,
    SajuDeukJi,
    SajuDeukSi,
    SajuDeukSe,
    SajuSilRyeong,
    SajuSilJi,
    SajuSilSi,
    SajuSilSe,
    SajuYongShen,
    SajuPrimaryYongShen,
    SajuHeeShen,
    SajuYongShenDetail,
    SajuInfoInputTime,
    SajuInfoCorrectedTime,
    SajuInfoTimezone,
    SajuInfoDst,
    SajuAnalysisMetaTitle,
    SajuInfoDstApplied,
    SajuInfoDstNone,
    SajuPowerWeighted,
    SajuPowerDominant,
    SajuPowerTenGods,
    SajuLuckMajor,
    SajuLuckShiftAge,
    SajuLuckAgeFrom,
    SajuPillarsSupplementary,
    SajuPillarTaiYuan,
    SajuPillarMingGong,
    SajuPillarShenGong,
    SajuVoidTitle,
    SajuVoidXun,
    SajuVoidBranches,
    SajuVoidTenGods,
    SajuVoidSuffix,
    SajuLevelAuspicious,
    SajuLevelCaution,
    SajuFuzzerTitle,
    SajuFuzzerCrashes,
    SajuFuzzerDesc,
    SajuFuzzerEnergyLevel,
    SajuFuzzerVector,
    SajuFuzzerTags,
    SajuFuzzerNoCrash,
    SajuLoadTitle,
    SajuLoadDesc,
    SajuLoadNoEvent,
    SajuSpiritDetailTitle,
    SajuPillarStem,
    SajuPillarBranch,
    SajuSpiritRequirement,

    SajuStructTitle,
    SajuStructDesc,
    SajuStructDecision,
    SajuProjectedStem,
    SajuProjectionPath,
    SajuStructReasons,
    SajuHiddenHarmonyTitle,
    SajuHiddenHarmonyDesc,
    SajuAmHarmonyTitle,
    SajuMyungAmHarmonyTitle,
    SajuLabelHarmony,
    SajuLabelTransformedElement,

    // ── Simulation Tab ──────────────────────────────────────────────────
    SimVulnerability,
    SimVmCrashes,
    SimEntropy,
    SimQiEfficiency,
    SimBottleneck,
    SimComplexity,
    SimStability,
    SimEnergyFlowTitle,
    SimScoreOverall,
    SimScoreAverage,
    SimGoldenTimeTitle,
    SimGoldenTimeRange,
    SimGoldenTimeAvg,
    SimLifeFramesTitle,
    SimColMajorLuck,
    SimColAnnualLuck,
    SimColScore,
    SimColTrend,
    SimColAge,
    SimKarmaDiagnostics,
    SimStrategy,

    // ── Tier Tab ────────────────────────────────────────────────────────
    TierPotentialTitle,
    TierDestinyPowerScore,
    TierEasternSajuScore,
    TierVedicScore,
    TierStrengthsInherent,
    TierWeaknessesCaution,
    TierWeightsTitle,

    // ── Vedic Yoga Categories ────────────────────────────────────────────
    VedicYogaCategoryRaja,
    VedicYogaCategoryNabhasa,
    VedicYogaCategoryDhana,
    VedicYogaCategoryChandra,
    VedicYogaCategoryParivartana,
    VedicYogaCategoryArishta,
    VedicYogaCategoryNeecha,

    // ── Vedic Compatibility ──────────────────────────────
    CompatTitleInput,
    CompatBtnRun,
    CompatStatusLoading,
    CompatHeaderOverall,
    CompatScoreLabel,
    CompatIsCompatibleGood,
    CompatIsCompatibleCaution,
    CompatMaleMangalDosha,
    CompatFemaleMangalDosha,
    CompatMangalDetected,
    CompatMangalNotDetected,
    CompatMangalCancelled,
    CompatAshtakootaTableTitle,
    CompatTableColKoota,
    CompatTableColMax,
    CompatTableColEarned,
    CompatTableColDesc,
    CompatExplanationGood,
    CompatExplanationWarning,
    CompatExplanationBad,
    CompatExplanationSummary,

    // 8 Kootas Names
    KootaVarnaName,
    KootaVashyaName,
    KootaTaraName,
    KootaYoniName,
    KootaGrahaMaitriName,
    KootaGanaName,
    KootaBhakootName,
    KootaNadiName,

    // 8 Kootas Descriptions
    KootaVarnaDescGood,
    KootaVarnaDescBad,
    KootaVashyaDescPattern,
    KootaTaraDescGood,
    KootaTaraDescOk,
    KootaTaraDescBad,
    KootaYoniDescPattern,
    KootaGrahaMaitriDescGood,
    KootaGrahaMaitriDescOk,
    KootaGrahaMaitriDescBad,
    KootaGanaDescGood,
    KootaGanaDescOk,
    KootaGanaDescBad,
    KootaBhakootDescGood,
    KootaBhakootDescBad,
    KootaNadiDescGood,
    KootaNadiDescBad,

    // --- Shadbala ---
    ShadbalaSthanaBala,
    ShadbalaDigBala,
    ShadbalaKalaBala,
    ShadbalaChestaBala,
    ShadbalaNaisargikaBala,
    ShadbalaDrikBala,
    ShadbalaRequired,
    ShadbalaActual,
    ShadbalaSatisfied,
    ShadbalaUnsatisfied,
    ShadbalaRupa,

    // --- KP System ---
    KpLevel1,
    KpLevel2,
    KpLevel3,
    KpLevel4,
    KpSignificatorsTitle,
    KpHouseSignificatorsTitle,
    KpTableColSignificators,

    // --- Dasha ---
    DashaMahaDasha,
    DashaAntarDasha,
    DashaPratyantarDasha,

    // --- Saju Jijanggan ---
    SajuHiddenStemsTitle,
    SajuProjectionTitle,
    SajuProjectionDesc,
    SajuProjLevelMain,
    SajuProjLevelSub,
    SajuJijangganYeogi,
    SajuJijangganJunggi,
    SajuJijangganJeonggi,

    // --- ZWDS ---
    ZwdsAnalyzeBtn,
    ZwdsReportTitle,
    ZwdsMasterSoul,
    ZwdsMasterBody,
    ZwdsElementsBureau,
    ZwdsCurrentDaxian,
    ZwdsDaxianTitle,
    ZwdsDaxianCardTitle,
    ZwdsCurrentDaxianBadge,
    ZwdsCopyBtn,
    ZwdsCopySuccess,
    ZwdsIdleHint,
    ZwdsLoadingHint,
    ZwdsErrorHint,
    ZwdsLoadFailed,
    ZwdsCenterCardTitle,
    ZwdsLiuNianBadge,
    ZwdsPalaceSuffix,
    ZwdsDaxianSuffix,

    // I Ching / He Luo
    IChingTitle,
    IChingPreNatal,
    IChingPostNatal,
    IChingYuanDang,
    IChingTimelineTitle,
    IChingSelectYaoHint,
    IChingAgeSuffix,
    IChingYangYao,
    IChingYinYao,
    StatusLoadingIChing,

    // I Ching Advanced Keys
    IChingYuanQi,
    IChingHuaGong,
    IChingGetYuanQi,
    IChingLoseYuanQi,
    IChingGetHuaGong,
    IChingLoseHuaGong,
    IChingSeYao,
    IChingYingYao,
    IChingTi,
    IChingYong,
    IChingTiYong,
    IChingYearlyHex,
    IChingMonthlyHex,
    IChingNoble,
    IChingVoid,
    IChingRok,
    IChingHorse,
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

pub fn translate_koota_name(locale: Locale, id: &str) -> &'static str {
    match id {
        "varna" => t(locale, TK::KootaVarnaName),
        "vashya" => t(locale, TK::KootaVashyaName),
        "tara" => t(locale, TK::KootaTaraName),
        "yoni" => t(locale, TK::KootaYoniName),
        "graha_maitri" => t(locale, TK::KootaGrahaMaitriName),
        "gana" => t(locale, TK::KootaGanaName),
        "bhakoot" => t(locale, TK::KootaBhakootName),
        "nadi" => t(locale, TK::KootaNadiName),
        _ => "",
    }
}

pub fn translate_koota_desc(locale: Locale, id: &str, earned_points: f64) -> String {
    match id {
        "varna" => {
            if earned_points == 1.0 {
                t(locale, TK::KootaVarnaDescGood).to_string()
            } else {
                t(locale, TK::KootaVarnaDescBad).to_string()
            }
        }
        "vashya" => {
            t(locale, TK::KootaVashyaDescPattern).replace("{}", &format!("{:.1}", earned_points))
        }
        "tara" => {
            if earned_points == 3.0 {
                t(locale, TK::KootaTaraDescGood).to_string()
            } else if earned_points == 1.5 {
                t(locale, TK::KootaTaraDescOk).to_string()
            } else {
                t(locale, TK::KootaTaraDescBad).to_string()
            }
        }
        "yoni" => {
            t(locale, TK::KootaYoniDescPattern).replace("{}", &format!("{:.1}", earned_points))
        }
        "graha_maitri" => {
            if earned_points >= 4.0 {
                t(locale, TK::KootaGrahaMaitriDescGood).to_string()
            } else if earned_points >= 2.0 {
                t(locale, TK::KootaGrahaMaitriDescOk).to_string()
            } else {
                t(locale, TK::KootaGrahaMaitriDescBad).to_string()
            }
        }
        "gana" => {
            if earned_points >= 5.0 {
                t(locale, TK::KootaGanaDescGood).to_string()
            } else if earned_points >= 3.0 {
                t(locale, TK::KootaGanaDescOk).to_string()
            } else {
                t(locale, TK::KootaGanaDescBad).to_string()
            }
        }
        "bhakoot" => {
            if earned_points == 7.0 {
                t(locale, TK::KootaBhakootDescGood).to_string()
            } else {
                t(locale, TK::KootaBhakootDescBad).to_string()
            }
        }
        "nadi" => {
            if earned_points == 8.0 {
                t(locale, TK::KootaNadiDescGood).to_string()
            } else {
                t(locale, TK::KootaNadiDescBad).to_string()
            }
        }
        _ => "".to_string(),
    }
}

// ── Domain & Dynamic Helpers ──────────────────────────────────────────

pub fn translate_planet(locale: Locale, p: VedicPlanet) -> &'static str {
    match locale {
        Locale::Ko => match p {
            VedicPlanet::Sun => "태양 ☀️",
            VedicPlanet::Moon => "달 🌙",
            VedicPlanet::Mars => "화성 ♂",
            VedicPlanet::Mercury => "수성 ☿",
            VedicPlanet::Jupiter => "목성 ♃",
            VedicPlanet::Venus => "금성 ♀",
            VedicPlanet::Saturn => "토성 ♄",
            VedicPlanet::Rahu => "라후 ☊",
            VedicPlanet::Ketu => "케투 ☋",
            VedicPlanet::Ascendant => "라그나 ⬆️",
        },
        Locale::En => match p {
            VedicPlanet::Sun => "Sun ☀️",
            VedicPlanet::Moon => "Moon 🌙",
            VedicPlanet::Mars => "Mars ♂",
            VedicPlanet::Mercury => "Mercury ☿",
            VedicPlanet::Jupiter => "Jupiter ♃",
            VedicPlanet::Venus => "Venus ♀",
            VedicPlanet::Saturn => "Saturn ♄",
            VedicPlanet::Rahu => "Rahu ☊",
            VedicPlanet::Ketu => "Ketu ☋",
            VedicPlanet::Ascendant => "Lagna ⬆️",
        },
        Locale::Zh => match p {
            VedicPlanet::Sun => "太阳 ☀️",
            VedicPlanet::Moon => "太阴 🌙",
            VedicPlanet::Mars => "火星 ♂",
            VedicPlanet::Mercury => "水星 ☿",
            VedicPlanet::Jupiter => "木星 ♃",
            VedicPlanet::Venus => "金星 ♀",
            VedicPlanet::Saturn => "土星 ♄",
            VedicPlanet::Rahu => "罗睺 ☊",
            VedicPlanet::Ketu => "计都 ☋",
            VedicPlanet::Ascendant => "命宫 ⬆️",
        },
        Locale::Ru => match p {
            VedicPlanet::Sun => "Солнце ☀️",
            VedicPlanet::Moon => "Луна 🌙",
            VedicPlanet::Mars => "Марс ♂",
            VedicPlanet::Mercury => "Меркурий ☿",
            VedicPlanet::Jupiter => "Юпитер ♃",
            VedicPlanet::Venus => "Венера ♀",
            VedicPlanet::Saturn => "Сатурн ♄",
            VedicPlanet::Rahu => "Раху ☊",
            VedicPlanet::Ketu => "Кету ☋",
            VedicPlanet::Ascendant => "Лагна ⬆️",
        },
    }
}

pub fn translate_planet_str(locale: Locale, p_name: &str) -> &str {
    match p_name {
        "Sun" | "Surya" => translate_planet(locale, VedicPlanet::Sun),
        "Moon" | "Chandra" => translate_planet(locale, VedicPlanet::Moon),
        "Mars" | "Mangala" => translate_planet(locale, VedicPlanet::Mars),
        "Mercury" | "Budha" => translate_planet(locale, VedicPlanet::Mercury),
        "Jupiter" | "Guru" => translate_planet(locale, VedicPlanet::Jupiter),
        "Venus" | "Shukra" => translate_planet(locale, VedicPlanet::Venus),
        "Saturn" | "Shani" => translate_planet(locale, VedicPlanet::Saturn),
        "Rahu" => translate_planet(locale, VedicPlanet::Rahu),
        "Ketu" => translate_planet(locale, VedicPlanet::Ketu),
        "Ascendant" | "Lagna" => translate_planet(locale, VedicPlanet::Ascendant),
        _ => p_name,
    }
}

pub fn rasi_name(locale: Locale, rasi: u8) -> &'static str {
    match locale {
        Locale::Ko => match rasi {
            1 => "양자리(Aries)",
            2 => "황소자리(Taurus)",
            3 => "쌍둥이자리(Gemini)",
            4 => "게자리(Cancer)",
            5 => "사자자리(Leo)",
            6 => "처녀자리(Virgo)",
            7 => "천칭자리(Libra)",
            8 => "전갈자리(Scorpio)",
            9 => "사수자리(Sagittarius)",
            10 => "염소자리(Capricorn)",
            11 => "물병자리(Aquarius)",
            12 => "물고기자리(Pisces)",
            _ => "—",
        },
        Locale::En => match rasi {
            1 => "Aries",
            2 => "Taurus",
            3 => "Gemini",
            4 => "Cancer",
            5 => "Leo",
            6 => "Virgo",
            7 => "Libra",
            8 => "Scorpio",
            9 => "Sagittarius",
            10 => "Capricorn",
            11 => "Aquarius",
            12 => "Pisces",
            _ => "—",
        },
        Locale::Zh => match rasi {
            1 => "白羊座 (Aries)",
            2 => "金牛座 (Taurus)",
            3 => "双子座 (Gemini)",
            4 => "巨蟹座 (Cancer)",
            5 => "狮子座 (Leo)",
            6 => "处女座 (Virgo)",
            7 => "天秤座 (Libra)",
            8 => "天蝎座 (Scorpio)",
            9 => "射手座 (Sagittarius)",
            10 => "摩羯座 (Capricorn)",
            11 => "水瓶座 (Aquarius)",
            12 => "双鱼座 (Pisces)",
            _ => "—",
        },
        Locale::Ru => match rasi {
            1 => "Овен (Aries)",
            2 => "Телец (Taurus)",
            3 => "Близнецы (Gemini)",
            4 => "Рак (Cancer)",
            5 => "Лев (Leo)",
            6 => "Дева (Virgo)",
            7 => "Весы (Libra)",
            8 => "Скорпион (Scorpio)",
            9 => "Стрелец (Sagittarius)",
            10 => "Козерог (Capricorn)",
            11 => "Водолей (Aquarius)",
            12 => "Рыбы (Pisces)",
            _ => "—",
        },
    }
}

pub fn rasi_name_short(locale: Locale, rasi: u8) -> &'static str {
    match locale {
        Locale::Ko => match rasi {
            1 => "AR (양)",
            2 => "TA (황소)",
            3 => "GE (쌍둥)",
            4 => "CN (게)",
            5 => "LE (사자)",
            6 => "VI (처녀)",
            7 => "LI (천칭)",
            8 => "SC (전갈)",
            9 => "SG (사수)",
            10 => "CP (염소)",
            11 => "AQ (물병)",
            12 => "PI (물고기)",
            _ => "—",
        },
        Locale::En => match rasi {
            1 => "AR (Aries)",
            2 => "TA (Taurus)",
            3 => "GE (Gemini)",
            4 => "CN (Cancer)",
            5 => "LE (Leo)",
            6 => "VI (Virgo)",
            7 => "LI (Libra)",
            8 => "SC (Scorpio)",
            9 => "SG (Sagittarius)",
            10 => "CP (Capricorn)",
            11 => "AQ (Aquarius)",
            12 => "PI (Pisces)",
            _ => "—",
        },
        Locale::Zh => match rasi {
            1 => "AR (白羊)",
            2 => "TA (金牛)",
            3 => "GE (双子)",
            4 => "CN (巨蟹)",
            5 => "LE (狮子)",
            6 => "VI (处女)",
            7 => "LI (天秤)",
            8 => "SC (天蝎)",
            9 => "SG (射手)",
            10 => "CP (摩羯)",
            11 => "AQ (水瓶)",
            12 => "PI (双鱼)",
            _ => "—",
        },
        Locale::Ru => match rasi {
            1 => "AR (Овен)",
            2 => "TA (Телец)",
            3 => "GE (Близнецы)",
            4 => "CN (Рак)",
            5 => "LE (Лев)",
            6 => "VI (Дева)",
            7 => "LI (Весы)",
            8 => "SC (Скорпион)",
            9 => "SG (Стрелец)",
            10 => "CP (Козерог)",
            11 => "AQ (Водолей)",
            12 => "PI (Рыбы)",
            _ => "—",
        },
    }
}

pub fn translate_avastha(locale: Locale, av: &LajjitadiAvastha) -> &'static str {
    match locale {
        Locale::Ko => match av {
            LajjitadiAvastha::Lajjita => "수치 (Lajjita)",
            LajjitadiAvastha::Garvita => "자긍 (Garvita)",
            LajjitadiAvastha::Kshudhita => "갈망 (Kshudhita)",
            LajjitadiAvastha::Trishita => "갈증 (Trishita)",
            LajjitadiAvastha::Mudita => "환희 (Mudita)",
            LajjitadiAvastha::Kshobhita => "동요 (Kshobhita)",
            LajjitadiAvastha::Neutral => "평온 (Neutral)",
        },
        Locale::En => match av {
            LajjitadiAvastha::Lajjita => "Lajjita (Humiliated)",
            LajjitadiAvastha::Garvita => "Garvita (Proud)",
            LajjitadiAvastha::Kshudhita => "Kshudhita (Starved)",
            LajjitadiAvastha::Trishita => "Trishita (Thirsty)",
            LajjitadiAvastha::Mudita => "Mudita (Delighted)",
            LajjitadiAvastha::Kshobhita => "Kshobhita (Agitated)",
            LajjitadiAvastha::Neutral => "Neutral",
        },
        Locale::Zh => match av {
            LajjitadiAvastha::Lajjita => "羞愧 (Lajjita)",
            LajjitadiAvastha::Garvita => "自豪 (Garvita)",
            LajjitadiAvastha::Kshudhita => "饥饿 (Kshudhita)",
            LajjitadiAvastha::Trishita => "口渴 (Trishita)",
            LajjitadiAvastha::Mudita => "欢喜 (Mudita)",
            LajjitadiAvastha::Kshobhita => "动摇 (Kshobhita)",
            LajjitadiAvastha::Neutral => "平静 (Neutral)",
        },
        Locale::Ru => match av {
            LajjitadiAvastha::Lajjita => "Ладжжита (Стыд)",
            LajjitadiAvastha::Garvita => "Гарвита (Гордость)",
            LajjitadiAvastha::Kshudhita => "Кшудхита (Голод)",
            LajjitadiAvastha::Trishita => "Тришита (Жажда)",
            LajjitadiAvastha::Mudita => "Мудита (Радость)",
            LajjitadiAvastha::Kshobhita => "Кшобхита (Волнение)",
            LajjitadiAvastha::Neutral => "Нейтрально",
        },
    }
}

pub fn nakshatra_lord(nakshatra_idx: u8) -> Option<VedicPlanet> {
    if nakshatra_idx == 0 || nakshatra_idx > 27 { return None; }
    match (nakshatra_idx - 1) % 9 {
        0 => Some(VedicPlanet::Ketu),
        1 => Some(VedicPlanet::Venus),
        2 => Some(VedicPlanet::Sun),
        3 => Some(VedicPlanet::Moon),
        4 => Some(VedicPlanet::Mars),
        5 => Some(VedicPlanet::Rahu),
        6 => Some(VedicPlanet::Jupiter),
        7 => Some(VedicPlanet::Saturn),
        8 => Some(VedicPlanet::Mercury),
        _ => None
    }
}

pub fn rasi_lord(rasi_idx: u8) -> Option<VedicPlanet> {
    match rasi_idx {
        1 => Some(VedicPlanet::Mars),
        2 => Some(VedicPlanet::Venus),
        3 => Some(VedicPlanet::Mercury),
        4 => Some(VedicPlanet::Moon),
        5 => Some(VedicPlanet::Sun),
        6 => Some(VedicPlanet::Mercury),
        7 => Some(VedicPlanet::Venus),
        8 => Some(VedicPlanet::Mars),
        9 => Some(VedicPlanet::Jupiter),
        10 => Some(VedicPlanet::Saturn),
        11 => Some(VedicPlanet::Saturn),
        12 => Some(VedicPlanet::Jupiter),
        _ => None
    }
}

pub fn nakshatra_lord_localized(locale: Locale, nakshatra_idx: u8) -> &'static str {
    if let Some(p) = nakshatra_lord(nakshatra_idx) {
        translate_planet(locale, p)
    } else {
        "—"
    }
}

pub fn rasi_lord_localized(locale: Locale, rasi_idx: u8) -> &'static str {
    if let Some(p) = rasi_lord(rasi_idx) {
        translate_planet(locale, p)
    } else {
        "—"
    }
}


pub fn format_age(locale: Locale, age: i32) -> String {
    match locale {
        Locale::Ko => format!("만 {}세", age),
        Locale::En => format!("Age {}", age),
        Locale::Zh => format!("{}岁", age),
        Locale::Ru => format!("{} лет", age),
    }
}

pub fn format_age_range(locale: Locale, start: i32, end: i32) -> String {
    match locale {
        Locale::Ko => format!("만 {}세 ~ {}세 (10년간)", start, end),
        Locale::En => format!("Age {} to {} (10 yrs)", start, end),
        Locale::Zh => format!("{}岁 ~ {}岁 (10年间)", start, end),
        Locale::Ru => format!("{} - {} лет (10 лет)", start, end),
    }
}

pub fn format_age_shift(locale: Locale, age: i32) -> String {
    match locale {
        Locale::Ko => format!("만 {}세 교운", age),
        Locale::En => format!("Shift at age {}", age),
        Locale::Zh => format!("{}岁交运", age),
        Locale::Ru => format!("Смена в {} лет", age),
    }
}

pub fn format_age_from(locale: Locale, age: i32) -> String {
    match locale {
        Locale::Ko => format!("만 {}세~", age),
        Locale::En => format!("Age {}~", age),
        Locale::Zh => format!("{}岁起~", age),
        Locale::Ru => format!("С {} лет~", age),
    }
}

pub fn format_weight_score(locale: Locale, weight_pct: f64, score: f64) -> String {
    match locale {
        Locale::Ko => format!("가중치 {:.0}% | {:.0}점", weight_pct, score),
        Locale::En => format!("Weight {:.0}% | {:.0} pts", weight_pct, score),
        Locale::Zh => format!("权重 {:.0}% | {:.0}分", weight_pct, score),
        Locale::Ru => format!("Вес {:.0}% | {:.0} б.", weight_pct, score),
    }
}

pub fn format_strength_summary(locale: Locale, count: usize, score: f64) -> String {
    match locale {
        Locale::Ko => format!("조건 {}/4 충족 | 점수 {:.1}", count, score),
        Locale::En => format!("Condition {}/4 Met | Score {:.1}", count, score),
        Locale::Zh => format!("条件 {}/4 满足 | 分数 {:.1}", count, score),
        Locale::Ru => format!("Условие {}/4 выполнено | Балл {:.1}", count, score),
    }
}

pub fn translate_zwds_palace(locale: Locale, name: eon_zwds::types::PalaceName) -> &'static str {
    match locale {
        Locale::Ko => match name {
            eon_zwds::types::PalaceName::Ming => "명궁(命)",
            eon_zwds::types::PalaceName::Xiongdi => "형제궁",
            eon_zwds::types::PalaceName::Fuqi => "부처궁",
            eon_zwds::types::PalaceName::Zinv => "자녀궁",
            eon_zwds::types::PalaceName::Caibo => "재백궁",
            eon_zwds::types::PalaceName::Jie => "질액궁",
            eon_zwds::types::PalaceName::Qianyi => "천이궁",
            eon_zwds::types::PalaceName::Nupao => "노복궁",
            eon_zwds::types::PalaceName::Guanlu => "관록궁",
            eon_zwds::types::PalaceName::Tianzhai => "전택궁",
            eon_zwds::types::PalaceName::Fude => "복덕궁",
            eon_zwds::types::PalaceName::Fumu => "부모궁",
        },
        Locale::En => match name {
            eon_zwds::types::PalaceName::Ming => "Life Palace",
            eon_zwds::types::PalaceName::Xiongdi => "Siblings",
            eon_zwds::types::PalaceName::Fuqi => "Spouse",
            eon_zwds::types::PalaceName::Zinv => "Children",
            eon_zwds::types::PalaceName::Caibo => "Wealth",
            eon_zwds::types::PalaceName::Jie => "Health",
            eon_zwds::types::PalaceName::Qianyi => "Travel",
            eon_zwds::types::PalaceName::Nupao => "Friends",
            eon_zwds::types::PalaceName::Guanlu => "Career",
            eon_zwds::types::PalaceName::Tianzhai => "Property",
            eon_zwds::types::PalaceName::Fude => "Karma",
            eon_zwds::types::PalaceName::Fumu => "Parents",
        },
        Locale::Zh => match name {
            eon_zwds::types::PalaceName::Ming => "命宫",
            eon_zwds::types::PalaceName::Xiongdi => "兄弟宫",
            eon_zwds::types::PalaceName::Fuqi => "夫妻宫",
            eon_zwds::types::PalaceName::Zinv => "子女宫",
            eon_zwds::types::PalaceName::Caibo => "财帛宫",
            eon_zwds::types::PalaceName::Jie => "疾厄宫",
            eon_zwds::types::PalaceName::Qianyi => "迁移宫",
            eon_zwds::types::PalaceName::Nupao => "奴仆宫",
            eon_zwds::types::PalaceName::Guanlu => "官禄宫",
            eon_zwds::types::PalaceName::Tianzhai => "田宅宫",
            eon_zwds::types::PalaceName::Fude => "福德宫",
            eon_zwds::types::PalaceName::Fumu => "父母宫",
        },
        Locale::Ru => match name {
            eon_zwds::types::PalaceName::Ming => "Дворец Судьбы",
            eon_zwds::types::PalaceName::Xiongdi => "Дворец Братьев",
            eon_zwds::types::PalaceName::Fuqi => "Дворец Брака",
            eon_zwds::types::PalaceName::Zinv => "Дворец Детей",
            eon_zwds::types::PalaceName::Caibo => "Дворец Богатства",
            eon_zwds::types::PalaceName::Jie => "Дворец Здоровья",
            eon_zwds::types::PalaceName::Qianyi => "Дворец Переездов",
            eon_zwds::types::PalaceName::Nupao => "Дворец Друзей",
            eon_zwds::types::PalaceName::Guanlu => "Дворец Карьеры",
            eon_zwds::types::PalaceName::Tianzhai => "Дворец Имущества",
            eon_zwds::types::PalaceName::Fude => "Дворец Духовности",
            eon_zwds::types::PalaceName::Fumu => "Дворец Родителей",
        },
    }
}

pub fn translate_zwds_star(locale: Locale, star: eon_zwds::types::ZwdsStar) -> &'static str {
    match locale {
        Locale::Ko => star.korean(),
        Locale::En => match star {
            eon_zwds::types::ZwdsStar::ZiWei => "Emperor (Ziwei)",
            eon_zwds::types::ZwdsStar::TianJi => "Advisor (Tianji)",
            eon_zwds::types::ZwdsStar::TaiYang => "Sun (Taiyang)",
            eon_zwds::types::ZwdsStar::WuQu => "Finance (Wuqu)",
            eon_zwds::types::ZwdsStar::TianTong => "Pleasure (Tiantong)",
            eon_zwds::types::ZwdsStar::LianZhen => "Wicked (Lianzhen)",
            eon_zwds::types::ZwdsStar::TianFu => "Treasury (Tianfu)",
            eon_zwds::types::ZwdsStar::TaiYin => "Moon (Taiyin)",
            eon_zwds::types::ZwdsStar::TanLang => "Flirt (Tanlang)",
            eon_zwds::types::ZwdsStar::JuMen => "Gate (Jumen)",
            eon_zwds::types::ZwdsStar::TianXiang => "Minister (Tianxiang)",
            eon_zwds::types::ZwdsStar::TianLiang => "Blessing (Tianliang)",
            eon_zwds::types::ZwdsStar::QiSha => "General (Qisha)",
            eon_zwds::types::ZwdsStar::PoJun => "Ruiner (Pojun)",
            eon_zwds::types::ZwdsStar::WenChang => "Intellect (Wenchang)",
            eon_zwds::types::ZwdsStar::WenQu => "Artistry (Wenqu)",
            eon_zwds::types::ZwdsStar::ZuoFu => "Helper L (Zuofu)",
            eon_zwds::types::ZwdsStar::YouBi => "Helper R (Youbi)",
            eon_zwds::types::ZwdsStar::TianKui => "Laurel (Tiankui)",
            eon_zwds::types::ZwdsStar::TianYue => "Halberd (Tianyue)",
            eon_zwds::types::ZwdsStar::LuCun => "Wealth (Lucun)",
            eon_zwds::types::ZwdsStar::QingYang => "Sheep (Qingyang)",
            eon_zwds::types::ZwdsStar::TuoLuo => "Dagger (Tuoluo)",
            eon_zwds::types::ZwdsStar::TianMa => "Horse (Tianma)",
            eon_zwds::types::ZwdsStar::HuoXing => "Fire (Huoxing)",
            eon_zwds::types::ZwdsStar::LingXing => "Siren (Lingxing)",
            eon_zwds::types::ZwdsStar::DiJie => "Robbery (Dijie)",
            eon_zwds::types::ZwdsStar::DiKong => "Void (Dikong)",
            eon_zwds::types::ZwdsStar::HongLuan => "Red Phoenix (Hongluan)",
            eon_zwds::types::ZwdsStar::TianXi => "Heavenly Joy (Tianxi)",
            eon_zwds::types::ZwdsStar::TianXing => "Punishment (Tianxing)",
            eon_zwds::types::ZwdsStar::TianYao => "Romance (Tianyao)",
            eon_zwds::types::ZwdsStar::JieShen => "Resolution (Jieshen)",
            eon_zwds::types::ZwdsStar::TianWu => "Shaman (Tianwu)",
            eon_zwds::types::ZwdsStar::TianYueStar => "Monthly (Tianyue)",
            eon_zwds::types::ZwdsStar::YinSha => "Shadow (Yinsha)",
            eon_zwds::types::ZwdsStar::TaiFu => "Aide (Taifu)",
            eon_zwds::types::ZwdsStar::FengGao => "Patent (Fenggao)",
            eon_zwds::types::ZwdsStar::SanTai => "Three Steps (Santai)",
            eon_zwds::types::ZwdsStar::BaZuo => "Eight Seats (Bazuo)",
            eon_zwds::types::ZwdsStar::EnGuang => "Grace (Enguang)",
            eon_zwds::types::ZwdsStar::TianGui => "Honor (Tiangui)",
            eon_zwds::types::ZwdsStar::TianCai => "Talent (Tiancai)",
            eon_zwds::types::ZwdsStar::TianShou => "Longevity (Tianshou)",
            eon_zwds::types::ZwdsStar::LongChi => "Dragon Pond (Longchi)",
            eon_zwds::types::ZwdsStar::FengGe => "Phoenix Pavilion (Fengge)",
            eon_zwds::types::ZwdsStar::TianKu => "Crying (Tianku)",
            eon_zwds::types::ZwdsStar::TianXu => "Emptiness (Tianxu)",
            eon_zwds::types::ZwdsStar::HuaGai => "Canopy (Huagai)",
            eon_zwds::types::ZwdsStar::XianChi => "Bath (Xianchi)",
            eon_zwds::types::ZwdsStar::GuChen => "Solitary (Guchen)",
            eon_zwds::types::ZwdsStar::GuaSu => "Widowhood (Guasu)",
            eon_zwds::types::ZwdsStar::TianKong => "Sky Void (Tiankong)",
            eon_zwds::types::ZwdsStar::JieSha => "Disaster (Jiesha)",
            eon_zwds::types::ZwdsStar::TianChu => "Kitchen (Tianchu)",
            eon_zwds::types::ZwdsStar::TianGuan => "Officer (Tianguan)",
            eon_zwds::types::ZwdsStar::TianFu2 => "Blessing (Tianfu)",
        },
        Locale::Zh => star.hanja(),
        Locale::Ru => match star {
            eon_zwds::types::ZwdsStar::ZiWei => "Император (Ziwei)",
            eon_zwds::types::ZwdsStar::TianJi => "Советник (Tianji)",
            eon_zwds::types::ZwdsStar::TaiYang => "Солнце (Taiyang)",
            eon_zwds::types::ZwdsStar::WuQu => "Финансы (Wuqu)",
            eon_zwds::types::ZwdsStar::TianTong => "Удовольствие (Tiantong)",
            eon_zwds::types::ZwdsStar::LianZhen => "Порочность (Lianzhen)",
            eon_zwds::types::ZwdsStar::TianFu => "Сокровищница (Tianfu)",
            eon_zwds::types::ZwdsStar::TaiYin => "Луна (Taiyin)",
            eon_zwds::types::ZwdsStar::TanLang => "Соблазн (Tanlang)",
            eon_zwds::types::ZwdsStar::JuMen => "Врата (Jumen)",
            eon_zwds::types::ZwdsStar::TianXiang => "Министр (Tianxiang)",
            eon_zwds::types::ZwdsStar::TianLiang => "Благословение (Tianliang)",
            eon_zwds::types::ZwdsStar::QiSha => "Генерал (Qisha)",
            eon_zwds::types::ZwdsStar::PoJun => "Разрушитель (Pojun)",
            eon_zwds::types::ZwdsStar::WenChang => "Интеллект (Wenchang)",
            eon_zwds::types::ZwdsStar::WenQu => "Творчество (Wenqu)",
            eon_zwds::types::ZwdsStar::ZuoFu => "Помощник Л (Zuofu)",
            eon_zwds::types::ZwdsStar::YouBi => "Помощник П (Youbi)",
            eon_zwds::types::ZwdsStar::TianKui => "Лавр (Tiankui)",
            eon_zwds::types::ZwdsStar::TianYue => "Алебарда (Tianyue)",
            eon_zwds::types::ZwdsStar::LuCun => "Богатство (Lucun)",
            eon_zwds::types::ZwdsStar::QingYang => "Овен (Qingyang)",
            eon_zwds::types::ZwdsStar::TuoLuo => "Кинжал (Tuoluo)",
            eon_zwds::types::ZwdsStar::TianMa => "Лошадь (Tianma)",
            eon_zwds::types::ZwdsStar::HuoXing => "Огонь (Huoxing)",
            eon_zwds::types::ZwdsStar::LingXing => "Сирена (Lingxing)",
            eon_zwds::types::ZwdsStar::DiJie => "Грабеж (Dijie)",
            eon_zwds::types::ZwdsStar::DiKong => "Пустота (Dikong)",
            eon_zwds::types::ZwdsStar::HongLuan => "Красный Феникс (Hongluan)",
            eon_zwds::types::ZwdsStar::TianXi => "Радость (Tianxi)",
            eon_zwds::types::ZwdsStar::TianXing => "Наказание (Tianxing)",
            eon_zwds::types::ZwdsStar::TianYao => "Романтика (Tianyao)",
            eon_zwds::types::ZwdsStar::JieShen => "Разрешение (Jieshen)",
            eon_zwds::types::ZwdsStar::TianWu => "Шаман (Tianwu)",
            eon_zwds::types::ZwdsStar::TianYueStar => "Месячный (Tianyue)",
            eon_zwds::types::ZwdsStar::YinSha => "Тень (Yinsha)",
            eon_zwds::types::ZwdsStar::TaiFu => "Помощник (Taifu)",
            eon_zwds::types::ZwdsStar::FengGao => "Патент (Fenggao)",
            eon_zwds::types::ZwdsStar::SanTai => "Три ступени (Santai)",
            eon_zwds::types::ZwdsStar::BaZuo => "Восемь мест (Bazuo)",
            eon_zwds::types::ZwdsStar::EnGuang => "Благодать (Enguang)",
            eon_zwds::types::ZwdsStar::TianGui => "Честь (Tiangui)",
            eon_zwds::types::ZwdsStar::TianCai => "Талант (Tiancai)",
            eon_zwds::types::ZwdsStar::TianShou => "Долголетие (Tianshou)",
            eon_zwds::types::ZwdsStar::LongChi => "Драконий пруд (Longchi)",
            eon_zwds::types::ZwdsStar::FengGe => "Павильон Феникса (Fengge)",
            eon_zwds::types::ZwdsStar::TianKu => "Плач (Tianku)",
            eon_zwds::types::ZwdsStar::TianXu => "Пустота (Tianxu)",
            eon_zwds::types::ZwdsStar::HuaGai => "Балдахин (Huagai)",
            eon_zwds::types::ZwdsStar::XianChi => "Купание (Xianchi)",
            eon_zwds::types::ZwdsStar::GuChen => "Одиночество (Guchen)",
            eon_zwds::types::ZwdsStar::GuaSu => "Вдовство (Guasu)",
            eon_zwds::types::ZwdsStar::TianKong => "Небесная пустота (Tiankong)",
            eon_zwds::types::ZwdsStar::JieSha => "Бедствие (Jiesha)",
            eon_zwds::types::ZwdsStar::TianChu => "Кухня (Tianchu)",
            eon_zwds::types::ZwdsStar::TianGuan => "Чиновник (Tianguan)",
            eon_zwds::types::ZwdsStar::TianFu2 => "Благословение (Tianfu)",
        },
    }
}

pub fn translate_five_elements(locale: Locale, class: eon_zwds::types::FiveElementsClass) -> &'static str {
    match locale {
        Locale::Ko => class.korean(),
        Locale::En => match class {
            eon_zwds::types::FiveElementsClass::Water2 => "Water 2nd Bureau (Water 2)",
            eon_zwds::types::FiveElementsClass::Wood3 => "Wood 3rd Bureau (Wood 3)",
            eon_zwds::types::FiveElementsClass::Metal4 => "Metal 4th Bureau (Metal 4)",
            eon_zwds::types::FiveElementsClass::Earth5 => "Earth 5th Bureau (Earth 5)",
            eon_zwds::types::FiveElementsClass::Fire6 => "Fire 6th Bureau (Fire 6)",
        },
        Locale::Zh => match class {
            eon_zwds::types::FiveElementsClass::Water2 => "水二局",
            eon_zwds::types::FiveElementsClass::Wood3 => "木三局",
            eon_zwds::types::FiveElementsClass::Metal4 => "金四局",
            eon_zwds::types::FiveElementsClass::Earth5 => "土五局",
            eon_zwds::types::FiveElementsClass::Fire6 => "火六局",
        },
        Locale::Ru => match class {
            eon_zwds::types::FiveElementsClass::Water2 => "Водная 2-я палата",
            eon_zwds::types::FiveElementsClass::Wood3 => "Деревянная 3-я палата",
            eon_zwds::types::FiveElementsClass::Metal4 => "Металлическая 4-я палата",
            eon_zwds::types::FiveElementsClass::Earth5 => "Земляная 5-я палата",
            eon_zwds::types::FiveElementsClass::Fire6 => "Огненная 6-я палата",
        },
    }
}

pub fn translate_zwds_brightness(locale: Locale, brightness: eon_zwds::types::ZwdsBrightness) -> &'static str {
    match locale {
        Locale::Ko => match brightness {
            eon_zwds::types::ZwdsBrightness::Miao => "廟",
            eon_zwds::types::ZwdsBrightness::Wang => "旺",
            eon_zwds::types::ZwdsBrightness::De   => "得",
            eon_zwds::types::ZwdsBrightness::Li   => "利",
            eon_zwds::types::ZwdsBrightness::Ping => "平",
            eon_zwds::types::ZwdsBrightness::Bu   => "不",
            eon_zwds::types::ZwdsBrightness::Xian => "陷",
        },
        Locale::Zh => match brightness {
            eon_zwds::types::ZwdsBrightness::Miao => "庙",
            eon_zwds::types::ZwdsBrightness::Wang => "旺",
            eon_zwds::types::ZwdsBrightness::De   => "得",
            eon_zwds::types::ZwdsBrightness::Li   => "利",
            eon_zwds::types::ZwdsBrightness::Ping => "平",
            eon_zwds::types::ZwdsBrightness::Bu   => "不",
            eon_zwds::types::ZwdsBrightness::Xian => "陷",
        },
        Locale::En => match brightness {
            eon_zwds::types::ZwdsBrightness::Miao => "T",
            eon_zwds::types::ZwdsBrightness::Wang => "B",
            eon_zwds::types::ZwdsBrightness::De   => "G",
            eon_zwds::types::ZwdsBrightness::Li   => "b",
            eon_zwds::types::ZwdsBrightness::Ping => "P",
            eon_zwds::types::ZwdsBrightness::Bu   => "n",
            eon_zwds::types::ZwdsBrightness::Xian => "F",
        },
        Locale::Ru => match brightness {
            eon_zwds::types::ZwdsBrightness::Miao => "Х",
            eon_zwds::types::ZwdsBrightness::Wang => "А",
            eon_zwds::types::ZwdsBrightness::De   => "П",
            eon_zwds::types::ZwdsBrightness::Li   => "В",
            eon_zwds::types::ZwdsBrightness::Ping => "С",
            eon_zwds::types::ZwdsBrightness::Bu   => "Н",
            eon_zwds::types::ZwdsBrightness::Xian => "П",
        },
    }
}

pub fn translate_zwds_palace_abbr(locale: Locale, name: eon_zwds::types::PalaceName) -> &'static str {
    match locale {
        Locale::Ko => match name {
            eon_zwds::types::PalaceName::Ming => "명",
            eon_zwds::types::PalaceName::Xiongdi => "형",
            eon_zwds::types::PalaceName::Fuqi => "부",
            eon_zwds::types::PalaceName::Zinv => "자",
            eon_zwds::types::PalaceName::Caibo => "재",
            eon_zwds::types::PalaceName::Jie => "질",
            eon_zwds::types::PalaceName::Qianyi => "천",
            eon_zwds::types::PalaceName::Nupao => "노",
            eon_zwds::types::PalaceName::Guanlu => "관",
            eon_zwds::types::PalaceName::Tianzhai => "전",
            eon_zwds::types::PalaceName::Fude => "복",
            eon_zwds::types::PalaceName::Fumu => "모",
        },
        Locale::Zh => match name {
            eon_zwds::types::PalaceName::Ming => "命",
            eon_zwds::types::PalaceName::Xiongdi => "兄",
            eon_zwds::types::PalaceName::Fuqi => "夫",
            eon_zwds::types::PalaceName::Zinv => "子",
            eon_zwds::types::PalaceName::Caibo => "财",
            eon_zwds::types::PalaceName::Jie => "疾",
            eon_zwds::types::PalaceName::Qianyi => "迁",
            eon_zwds::types::PalaceName::Nupao => "奴",
            eon_zwds::types::PalaceName::Guanlu => "官",
            eon_zwds::types::PalaceName::Tianzhai => "田",
            eon_zwds::types::PalaceName::Fude => "福",
            eon_zwds::types::PalaceName::Fumu => "父",
        },
        Locale::En => match name {
            eon_zwds::types::PalaceName::Ming => "M",
            eon_zwds::types::PalaceName::Xiongdi => "B",
            eon_zwds::types::PalaceName::Fuqi => "C",
            eon_zwds::types::PalaceName::Zinv => "Ch",
            eon_zwds::types::PalaceName::Caibo => "W",
            eon_zwds::types::PalaceName::Jie => "H",
            eon_zwds::types::PalaceName::Qianyi => "T",
            eon_zwds::types::PalaceName::Nupao => "F",
            eon_zwds::types::PalaceName::Guanlu => "Ca",
            eon_zwds::types::PalaceName::Tianzhai => "P",
            eon_zwds::types::PalaceName::Fude => "K",
            eon_zwds::types::PalaceName::Fumu => "Pa",
        },
        Locale::Ru => match name {
            eon_zwds::types::PalaceName::Ming => "Суд",
            eon_zwds::types::PalaceName::Xiongdi => "Бр",
            eon_zwds::types::PalaceName::Fuqi => "Брк",
            eon_zwds::types::PalaceName::Zinv => "Дет",
            eon_zwds::types::PalaceName::Caibo => "Бог",
            eon_zwds::types::PalaceName::Jie => "Здр",
            eon_zwds::types::PalaceName::Qianyi => "Пер",
            eon_zwds::types::PalaceName::Nupao => "Др",
            eon_zwds::types::PalaceName::Guanlu => "Кар",
            eon_zwds::types::PalaceName::Tianzhai => "Им",
            eon_zwds::types::PalaceName::Fude => "Дух",
            eon_zwds::types::PalaceName::Fumu => "Род",
        },
    }
}

pub fn translate_saju_twelve_stage(locale: Locale, stage: eon_saju::core::twelve_stages::TwelveStage) -> &'static str {
    match locale {
        Locale::Ko => stage.hangul(),
        Locale::Zh => stage.hanja(),
        Locale::En => match stage {
            eon_saju::core::twelve_stages::TwelveStage::Changsheng => "Nurture",
            eon_saju::core::twelve_stages::TwelveStage::Muyu => "Bath",
            eon_saju::core::twelve_stages::TwelveStage::Guandai => "Youth",
            eon_saju::core::twelve_stages::TwelveStage::Jianlu => "Prosperity",
            eon_saju::core::twelve_stages::TwelveStage::Diwang => "Peak",
            eon_saju::core::twelve_stages::TwelveStage::Shuai => "Decline",
            eon_saju::core::twelve_stages::TwelveStage::Bing => "Illness",
            eon_saju::core::twelve_stages::TwelveStage::Si => "Death",
            eon_saju::core::twelve_stages::TwelveStage::Mu => "Grave",
            eon_saju::core::twelve_stages::TwelveStage::Jue => "Extinction",
            eon_saju::core::twelve_stages::TwelveStage::Tai => "Conception",
            eon_saju::core::twelve_stages::TwelveStage::Yang => "Gestation",
        },
        Locale::Ru => match stage {
            eon_saju::core::twelve_stages::TwelveStage::Changsheng => "Рождение",
            eon_saju::core::twelve_stages::TwelveStage::Muyu => "Купание",
            eon_saju::core::twelve_stages::TwelveStage::Guandai => "Половозрелость",
            eon_saju::core::twelve_stages::TwelveStage::Jianlu => "Служба",
            eon_saju::core::twelve_stages::TwelveStage::Diwang => "Пик",
            eon_saju::core::twelve_stages::TwelveStage::Shuai => "Увядание",
            eon_saju::core::twelve_stages::TwelveStage::Bing => "Болезнь",
            eon_saju::core::twelve_stages::TwelveStage::Si => "Смерть",
            eon_saju::core::twelve_stages::TwelveStage::Mu => "Могила",
            eon_saju::core::twelve_stages::TwelveStage::Jue => "Исчезновение",
            eon_saju::core::twelve_stages::TwelveStage::Tai => "Зачатие",
            eon_saju::core::twelve_stages::TwelveStage::Yang => "Вскармливание",
        },
    }
}

pub fn translate_saju_nayin(locale: Locale, nayin: eon_saju::core::nayin::NayinType) -> String {
    match locale {
        Locale::Ko => nayin.hangul().to_string(),
        Locale::Zh => match nayin {
            eon_saju::core::nayin::NayinType::SeaGold => "海中金",
            eon_saju::core::nayin::NayinType::FurnaceFire => "炉中火",
            eon_saju::core::nayin::NayinType::ForestWood => "大林木",
            eon_saju::core::nayin::NayinType::RoadEarth => "路旁土",
            eon_saju::core::nayin::NayinType::SwordGold => "剑锋金",
            eon_saju::core::nayin::NayinType::VolcanoFire => "山头火",
            eon_saju::core::nayin::NayinType::CaveWater => "涧下水",
            eon_saju::core::nayin::NayinType::FortressEarth => "城头土",
            eon_saju::core::nayin::NayinType::WaxGold => "白蜡金",
            eon_saju::core::nayin::NayinType::WillowWood => "杨柳木",
            eon_saju::core::nayin::NayinType::StreamWater => "泉中水",
            eon_saju::core::nayin::NayinType::RoofEarth => "屋上土",
            eon_saju::core::nayin::NayinType::LightningFire => "霹雳火",
            eon_saju::core::nayin::NayinType::PineWood => "松柏木",
            eon_saju::core::nayin::NayinType::RiverWater => "长流水",
            eon_saju::core::nayin::NayinType::SandGold => "沙中金",
            eon_saju::core::nayin::NayinType::ForestFire => "山下火",
            eon_saju::core::nayin::NayinType::MeadowWood => "平地木",
            eon_saju::core::nayin::NayinType::AdobeEarth => "壁上土",
            eon_saju::core::nayin::NayinType::PreciousGold => "金箔金",
            eon_saju::core::nayin::NayinType::LampFire => "覆灯火",
            eon_saju::core::nayin::NayinType::SkyWater => "天河水",
            eon_saju::core::nayin::NayinType::HighwayEarth => "大驿土",
            eon_saju::core::nayin::NayinType::JewelryGold => "钗钏金",
            eon_saju::core::nayin::NayinType::MulberryWood => "桑柘木",
            eon_saju::core::nayin::NayinType::RapidsWater => "大溪水",
            eon_saju::core::nayin::NayinType::DesertEarth => "沙中土",
            eon_saju::core::nayin::NayinType::SunFire => "天上火",
            eon_saju::core::nayin::NayinType::PomegranateWood => "石榴木",
            eon_saju::core::nayin::NayinType::OceanWater => "大海水",
        }.to_string(),
        Locale::En => nayin.english().to_string(),
        Locale::Ru => match nayin {
            eon_saju::core::nayin::NayinType::SeaGold => "Металл на дне моря",
            eon_saju::core::nayin::NayinType::FurnaceFire => "Огонь в печи",
            eon_saju::core::nayin::NayinType::ForestWood => "Дерево большого леса",
            eon_saju::core::nayin::NayinType::RoadEarth => "Земля у дороги",
            eon_saju::core::nayin::NayinType::SwordGold => "Металл на острие меча",
            eon_saju::core::nayin::NayinType::VolcanoFire => "Огонь на вершине горы",
            eon_saju::core::nayin::NayinType::CaveWater => "Вода под ущельем",
            eon_saju::core::nayin::NayinType::FortressEarth => "Земля на стене замка",
            eon_saju::core::nayin::NayinType::WaxGold => "Белый воск (Металл)",
            eon_saju::core::nayin::NayinType::WillowWood => "Дерево ивы",
            eon_saju::core::nayin::NayinType::StreamWater => "Вода из родника",
            eon_saju::core::nayin::NayinType::RoofEarth => "Земля на крыше",
            eon_saju::core::nayin::NayinType::LightningFire => "Огонь молнии",
            eon_saju::core::nayin::NayinType::PineWood => "Дерево сосны и кипариса",
            eon_saju::core::nayin::NayinType::RiverWater => "Длинная речная вода",
            eon_saju::core::nayin::NayinType::SandGold => "Металл в песке",
            eon_saju::core::nayin::NayinType::ForestFire => "Огонь под горой",
            eon_saju::core::nayin::NayinType::MeadowWood => "Дерево равнины",
            eon_saju::core::nayin::NayinType::AdobeEarth => "Земля на стене",
            eon_saju::core::nayin::NayinType::PreciousGold => "Сусальное золото (Металл)",
            eon_saju::core::nayin::NayinType::LampFire => "Свет светильника",
            eon_saju::core::nayin::NayinType::SkyWater => "Вода небесной реки",
            eon_saju::core::nayin::NayinType::HighwayEarth => "Земля большого тракта",
            eon_saju::core::nayin::NayinType::JewelryGold => "Золотые украшения (Металл)",
            eon_saju::core::nayin::NayinType::MulberryWood => "Дерево шелковицы",
            eon_saju::core::nayin::NayinType::RapidsWater => "Вода быстрого потока",
            eon_saju::core::nayin::NayinType::DesertEarth => "Земля в песке",
            eon_saju::core::nayin::NayinType::SunFire => "Небесный огонь (Солнце)",
            eon_saju::core::nayin::NayinType::PomegranateWood => "Дерево граната",
            eon_saju::core::nayin::NayinType::OceanWater => "Вода великого океана",
        }.to_string(),
    }
}

pub fn translate_saju_stem(locale: Locale, stem: eon_saju::core::stem::HeavenlyStem) -> &'static str {
    match locale {
        Locale::Ko => stem.hangul(),
        Locale::Zh => stem.hanja(),
        Locale::En => match stem {
            eon_saju::core::stem::HeavenlyStem::Jia => "Jia",
            eon_saju::core::stem::HeavenlyStem::Yi => "Yi",
            eon_saju::core::stem::HeavenlyStem::Bing => "Bing",
            eon_saju::core::stem::HeavenlyStem::Ding => "Ding",
            eon_saju::core::stem::HeavenlyStem::Wu => "Wu",
            eon_saju::core::stem::HeavenlyStem::Ji => "Ji",
            eon_saju::core::stem::HeavenlyStem::Geng => "Geng",
            eon_saju::core::stem::HeavenlyStem::Xin => "Xin",
            eon_saju::core::stem::HeavenlyStem::Ren => "Ren",
            eon_saju::core::stem::HeavenlyStem::Gui => "Gui",
        },
        Locale::Ru => match stem {
            eon_saju::core::stem::HeavenlyStem::Jia => "Цзя",
            eon_saju::core::stem::HeavenlyStem::Yi => "И",
            eon_saju::core::stem::HeavenlyStem::Bing => "Бин",
            eon_saju::core::stem::HeavenlyStem::Ding => "Дин",
            eon_saju::core::stem::HeavenlyStem::Wu => "У",
            eon_saju::core::stem::HeavenlyStem::Ji => "Цзи",
            eon_saju::core::stem::HeavenlyStem::Geng => "Гэн",
            eon_saju::core::stem::HeavenlyStem::Xin => "Синь",
            eon_saju::core::stem::HeavenlyStem::Ren => "Жэнь",
            eon_saju::core::stem::HeavenlyStem::Gui => "Гуй",
        },
    }
}

pub fn translate_saju_branch(locale: Locale, branch: eon_saju::core::branch::EarthlyBranch) -> &'static str {
    match locale {
        Locale::Ko => branch.hangul(),
        Locale::Zh => branch.hanja(),
        Locale::En => match branch {
            eon_saju::core::branch::EarthlyBranch::Zi => "Zi",
            eon_saju::core::branch::EarthlyBranch::Chou => "Chou",
            eon_saju::core::branch::EarthlyBranch::Yin => "Yin",
            eon_saju::core::branch::EarthlyBranch::Mao => "Mao",
            eon_saju::core::branch::EarthlyBranch::Chen => "Chen",
            eon_saju::core::branch::EarthlyBranch::Si => "Si",
            eon_saju::core::branch::EarthlyBranch::Wu => "Wu",
            eon_saju::core::branch::EarthlyBranch::Wei => "Wei",
            eon_saju::core::branch::EarthlyBranch::Shen => "Shen",
            eon_saju::core::branch::EarthlyBranch::You => "You",
            eon_saju::core::branch::EarthlyBranch::Xu => "Xu",
            eon_saju::core::branch::EarthlyBranch::Hai => "Hai",
        },
        Locale::Ru => match branch {
            eon_saju::core::branch::EarthlyBranch::Zi => "Цзы",
            eon_saju::core::branch::EarthlyBranch::Chou => "Чоу",
            eon_saju::core::branch::EarthlyBranch::Yin => "Инь",
            eon_saju::core::branch::EarthlyBranch::Mao => "Мао",
            eon_saju::core::branch::EarthlyBranch::Chen => "Чэнь",
            eon_saju::core::branch::EarthlyBranch::Si => "Сы",
            eon_saju::core::branch::EarthlyBranch::Wu => "У",
            eon_saju::core::branch::EarthlyBranch::Wei => "Вэй",
            eon_saju::core::branch::EarthlyBranch::Shen => "Шэнь",
            eon_saju::core::branch::EarthlyBranch::You => "Ю",
            eon_saju::core::branch::EarthlyBranch::Xu => "Сюй",
            eon_saju::core::branch::EarthlyBranch::Hai => "Хай",
        },
    }
}

pub fn translate_saju_element(locale: Locale, element: eon_saju::core::element::Element) -> &'static str {
    match locale {
        Locale::Ko => element.hangul(),
        Locale::Zh => match element {
            eon_saju::core::element::Element::Wood => "木",
            eon_saju::core::element::Element::Fire => "火",
            eon_saju::core::element::Element::Earth => "土",
            eon_saju::core::element::Element::Metal => "金",
            eon_saju::core::element::Element::Water => "水",
        },
        Locale::En => match element {
            eon_saju::core::element::Element::Wood => "Wood",
            eon_saju::core::element::Element::Fire => "Fire",
            eon_saju::core::element::Element::Earth => "Earth",
            eon_saju::core::element::Element::Metal => "Metal",
            eon_saju::core::element::Element::Water => "Water",
        },
        Locale::Ru => match element {
            eon_saju::core::element::Element::Wood => "Дерево",
            eon_saju::core::element::Element::Fire => "Огонь",
            eon_saju::core::element::Element::Earth => "Земля",
            eon_saju::core::element::Element::Metal => "Металл",
            eon_saju::core::element::Element::Water => "Вода",
        },
    }
}

pub fn translate_saju_ten_god(locale: Locale, god: eon_saju::core::ten_gods::TenGod) -> &'static str {
    match locale {
        Locale::Ko => god.hangul(),
        Locale::Zh => god.hanja(),
        Locale::En => match god {
            eon_saju::core::ten_gods::TenGod::Bijian => "Friend",
            eon_saju::core::ten_gods::TenGod::Jiecai => "Rob Wealth",
            eon_saju::core::ten_gods::TenGod::Shishen => "Eating God",
            eon_saju::core::ten_gods::TenGod::Shangguan => "Hurting Officer",
            eon_saju::core::ten_gods::TenGod::Piancai => "Indirect Wealth",
            eon_saju::core::ten_gods::TenGod::Zhengcai => "Direct Wealth",
            eon_saju::core::ten_gods::TenGod::Pianguan => "7 Killings",
            eon_saju::core::ten_gods::TenGod::Zhengguan => "Direct Officer",
            eon_saju::core::ten_gods::TenGod::Pianyin => "Indirect Resource",
            eon_saju::core::ten_gods::TenGod::Zhengyin => "Direct Resource",
        },
        Locale::Ru => match god {
            eon_saju::core::ten_gods::TenGod::Bijian => "Равное Плечо",
            eon_saju::core::ten_gods::TenGod::Jiecai => "Грабеж Богатства",
            eon_saju::core::ten_gods::TenGod::Shishen => "Дух Пищи",
            eon_saju::core::ten_gods::TenGod::Shangguan => "Вызов Власти",
            eon_saju::core::ten_gods::TenGod::Piancai => "Косвенное Богатство",
            eon_saju::core::ten_gods::TenGod::Zhengcai => "Прямое Богатство",
            eon_saju::core::ten_gods::TenGod::Pianguan => "7 Убийц",
            eon_saju::core::ten_gods::TenGod::Zhengguan => "Прямой Чиновник",
            eon_saju::core::ten_gods::TenGod::Pianyin => "Косвенный Ресурс",
            eon_saju::core::ten_gods::TenGod::Zhengyin => "Прямой Ресурс",
        },
    }
}

pub fn translate_saju_spirit_marker_name(locale: Locale, marker: eon_saju::analysis::spirit_markers::SpiritMarker) -> &'static str {
    match locale {
        Locale::Ko => marker.hangul(),
        Locale::Zh => match marker {
            eon_saju::analysis::spirit_markers::SpiritMarker::Tianyi => "天乙贵人",
            eon_saju::analysis::spirit_markers::SpiritMarker::Wenchang => "文昌贵人",
            eon_saju::analysis::spirit_markers::SpiritMarker::Taiji => "太极贵人",
            eon_saju::analysis::spirit_markers::SpiritMarker::Yuede => "月德贵人",
            eon_saju::analysis::spirit_markers::SpiritMarker::Tiande => "天德贵人",
            eon_saju::analysis::spirit_markers::SpiritMarker::Zhenglu => "正禄",
            eon_saju::analysis::spirit_markers::SpiritMarker::Jinyu => "金舆禄",
            eon_saju::analysis::spirit_markers::SpiritMarker::Anlu => "暗禄",
            eon_saju::analysis::spirit_markers::SpiritMarker::Xuetang => "学堂贵人",
            eon_saju::analysis::spirit_markers::SpiritMarker::TianyiMedical => "天医星",
            eon_saju::analysis::spirit_markers::SpiritMarker::Tianwen => "天文星",
            eon_saju::analysis::spirit_markers::SpiritMarker::Yima => "驿马煞",
            eon_saju::analysis::spirit_markers::SpiritMarker::Huagai => "华盖煞",
            eon_saju::analysis::spirit_markers::SpiritMarker::Kuigang => "魁罡煞",
            eon_saju::analysis::spirit_markers::SpiritMarker::Taohua => "桃花煞",
            eon_saju::analysis::spirit_markers::SpiritMarker::Hongyan => "红艳煞",
            eon_saju::analysis::spirit_markers::SpiritMarker::Guchen => "孤辰煞",
            eon_saju::analysis::spirit_markers::SpiritMarker::Guasu => "寡宿煞",
            eon_saju::analysis::spirit_markers::SpiritMarker::Xuanzhen => "悬针煞",
            eon_saju::analysis::spirit_markers::SpiritMarker::Baihu => "白虎煞",
            eon_saju::analysis::spirit_markers::SpiritMarker::Wangshen => "亡神煞",
            eon_saju::analysis::spirit_markers::SpiritMarker::Jiesha => "劫煞",
            eon_saju::analysis::spirit_markers::SpiritMarker::Yuanzhen => "怨嗔煞/元辰",
            eon_saju::analysis::spirit_markers::SpiritMarker::Jaesha => "灾煞",
            eon_saju::analysis::spirit_markers::SpiritMarker::Cheonsha => "天煞",
            eon_saju::analysis::spirit_markers::SpiritMarker::Jisha => "地煞",
            eon_saju::analysis::spirit_markers::SpiritMarker::Nyeonsha => "年煞",
            eon_saju::analysis::spirit_markers::SpiritMarker::Wolsha => "月煞",
            eon_saju::analysis::spirit_markers::SpiritMarker::Jangseong => "将星",
            eon_saju::analysis::spirit_markers::SpiritMarker::Banan => "潘鞍煞",
            eon_saju::analysis::spirit_markers::SpiritMarker::Yukhae => "六害煞",
        },
        Locale::En => match marker {
            eon_saju::analysis::spirit_markers::SpiritMarker::Tianyi => "Heavenly Noble",
            eon_saju::analysis::spirit_markers::SpiritMarker::Wenchang => "Intelligence (Wenchang)",
            eon_saju::analysis::spirit_markers::SpiritMarker::Taiji => "Taiji Noble",
            eon_saju::analysis::spirit_markers::SpiritMarker::Yuede => "Monthly Virtue",
            eon_saju::analysis::spirit_markers::SpiritMarker::Tiande => "Heavenly Virtue",
            eon_saju::analysis::spirit_markers::SpiritMarker::Zhenglu => "Prosperity (Lu)",
            eon_saju::analysis::spirit_markers::SpiritMarker::Jinyu => "Golden Carriage",
            eon_saju::analysis::spirit_markers::SpiritMarker::Anlu => "Dark Lu (Hidden)",
            eon_saju::analysis::spirit_markers::SpiritMarker::Xuetang => "Study Hall",
            eon_saju::analysis::spirit_markers::SpiritMarker::TianyiMedical => "Heavenly Doctor",
            eon_saju::analysis::spirit_markers::SpiritMarker::Tianwen => "Heavenly Gate",
            eon_saju::analysis::spirit_markers::SpiritMarker::Yima => "Traveling Horse",
            eon_saju::analysis::spirit_markers::SpiritMarker::Huagai => "Elegant Cover (Art)",
            eon_saju::analysis::spirit_markers::SpiritMarker::Kuigang => "Fierce Leader (Kui Gang)",
            eon_saju::analysis::spirit_markers::SpiritMarker::Taohua => "Peach Blossom",
            eon_saju::analysis::spirit_markers::SpiritMarker::Hongyan => "Red Beauty (Hong Yan)",
            eon_saju::analysis::spirit_markers::SpiritMarker::Guchen => "Solitary Loner",
            eon_saju::analysis::spirit_markers::SpiritMarker::Guasu => "Widow Star",
            eon_saju::analysis::spirit_markers::SpiritMarker::Xuanzhen => "Suspended Needle",
            eon_saju::analysis::spirit_markers::SpiritMarker::Baihu => "White Tiger",
            eon_saju::analysis::spirit_markers::SpiritMarker::Wangshen => "Humiliation",
            eon_saju::analysis::spirit_markers::SpiritMarker::Jiesha => "Robbery Sha",
            eon_saju::analysis::spirit_markers::SpiritMarker::Yuanzhen => "Resentment (Yuan Zhen)",
            eon_saju::analysis::spirit_markers::SpiritMarker::Jaesha => "Disaster Sha",
            eon_saju::analysis::spirit_markers::SpiritMarker::Cheonsha => "Heavenly Sha",
            eon_saju::analysis::spirit_markers::SpiritMarker::Jisha => "Earthly Sha",
            eon_saju::analysis::spirit_markers::SpiritMarker::Nyeonsha => "Yearly Peach Blossom",
            eon_saju::analysis::spirit_markers::SpiritMarker::Wolsha => "Monthly Sha",
            eon_saju::analysis::spirit_markers::SpiritMarker::Jangseong => "General Star",
            eon_saju::analysis::spirit_markers::SpiritMarker::Banan => "Saddle Star",
            eon_saju::analysis::spirit_markers::SpiritMarker::Yukhae => "Six Harms Sha",
        },
        Locale::Ru => match marker {
            eon_saju::analysis::spirit_markers::SpiritMarker::Tianyi => "Благородный Небесной Единицы",
            eon_saju::analysis::spirit_markers::SpiritMarker::Wenchang => "Звезда Академика",
            eon_saju::analysis::spirit_markers::SpiritMarker::Taiji => "Благородный Тайцзи",
            eon_saju::analysis::spirit_markers::SpiritMarker::Yuede => "Благородный Месячной Добродетели",
            eon_saju::analysis::spirit_markers::SpiritMarker::Tiande => "Благородный Небесной Добродетели",
            eon_saju::analysis::spirit_markers::SpiritMarker::Zhenglu => "Вознаграждение (Лу)",
            eon_saju::analysis::spirit_markers::SpiritMarker::Jinyu => "Золотая Карета",
            eon_saju::analysis::spirit_markers::SpiritMarker::Anlu => "Скрытое Вознаграждение",
            eon_saju::analysis::spirit_markers::SpiritMarker::Xuetang => "Академический Зал",
            eon_saju::analysis::spirit_markers::SpiritMarker::TianyiMedical => "Небесный Доктор",
            eon_saju::analysis::spirit_markers::SpiritMarker::Tianwen => "Небесные Ворота",
            eon_saju::analysis::spirit_markers::SpiritMarker::Yima => "Почтовая Лошадь",
            eon_saju::analysis::spirit_markers::SpiritMarker::Huagai => "Цветущий Балдахин",
            eon_saju::analysis::spirit_markers::SpiritMarker::Kuigang => "Куй Ганг",
            eon_saju::analysis::spirit_markers::SpiritMarker::Taohua => "Цветок Персика",
            eon_saju::analysis::spirit_markers::SpiritMarker::Hongyan => "Красная Луна (Хун Янь)",
            eon_saju::analysis::spirit_markers::SpiritMarker::Guchen => "Одинокий Приют",
            eon_saju::analysis::spirit_markers::SpiritMarker::Guasu => "Приют Одиночества",
            eon_saju::analysis::spirit_markers::SpiritMarker::Xuanzhen => "Висящая Игла",
            eon_saju::analysis::spirit_markers::SpiritMarker::Baihu => "Белый Тигр",
            eon_saju::analysis::spirit_markers::SpiritMarker::Wangshen => "Демон Уничтожения",
            eon_saju::analysis::spirit_markers::SpiritMarker::Jiesha => "Ша Ограбления",
            eon_saju::analysis::spirit_markers::SpiritMarker::Yuanzhen => "Взаимное Несогласие",
            eon_saju::analysis::spirit_markers::SpiritMarker::Jaesha => "Ша Катастрофы",
            eon_saju::analysis::spirit_markers::SpiritMarker::Cheonsha => "Небесная Ша",
            eon_saju::analysis::spirit_markers::SpiritMarker::Jisha => "Земная Ша",
            eon_saju::analysis::spirit_markers::SpiritMarker::Nyeonsha => "Годовая Ша (Персик)",
            eon_saju::analysis::spirit_markers::SpiritMarker::Wolsha => "Месячная Ша",
            eon_saju::analysis::spirit_markers::SpiritMarker::Jangseong => "Звезда Генерала",
            eon_saju::analysis::spirit_markers::SpiritMarker::Banan => "Золотая Седловина",
            eon_saju::analysis::spirit_markers::SpiritMarker::Yukhae => "Ша Шести Вредов",
        },
    }
}

pub fn translate_aux_shinsal(locale: Locale, name: &str) -> String {
    let marker = match name {
        "천을귀인" => Some(eon_saju::analysis::spirit_markers::SpiritMarker::Tianyi),
        "지살" => Some(eon_saju::analysis::spirit_markers::SpiritMarker::Jisha),
        "년살" | "년살(도화)" | "도화살" => Some(eon_saju::analysis::spirit_markers::SpiritMarker::Nyeonsha),
        "월살" => Some(eon_saju::analysis::spirit_markers::SpiritMarker::Wolsha),
        "망신살" => Some(eon_saju::analysis::spirit_markers::SpiritMarker::Wangshen),
        "장성살" => Some(eon_saju::analysis::spirit_markers::SpiritMarker::Jangseong),
        "반안살" => Some(eon_saju::analysis::spirit_markers::SpiritMarker::Banan),
        "역마살" => Some(eon_saju::analysis::spirit_markers::SpiritMarker::Yima),
        "육해살" => Some(eon_saju::analysis::spirit_markers::SpiritMarker::Yukhae),
        "화개살" => Some(eon_saju::analysis::spirit_markers::SpiritMarker::Huagai),
        "겁살" => Some(eon_saju::analysis::spirit_markers::SpiritMarker::Jiesha),
        "재살" => Some(eon_saju::analysis::spirit_markers::SpiritMarker::Jaesha),
        "천살" => Some(eon_saju::analysis::spirit_markers::SpiritMarker::Cheonsha),
        "원진살" | "원진" => Some(eon_saju::analysis::spirit_markers::SpiritMarker::Yuanzhen),
        _ => None,
    };
    if let Some(m) = marker {
        translate_saju_spirit_marker_name(locale, m).to_string()
    } else {
        name.to_string()
    }
}

pub fn translate_saju_spirit_marker_desc(locale: Locale, marker: eon_saju::analysis::spirit_markers::SpiritMarker) -> &'static str {
    match locale {
        Locale::Ko => marker.description(),
        Locale::Zh => match marker {
            eon_saju::analysis::spirit_markers::SpiritMarker::Tianyi => "得贵人相助，逢凶化吉，克服困难",
            eon_saju::analysis::spirit_markers::SpiritMarker::Wenchang => "利于学业与考试，文書契约相关吉利",
            eon_saju::analysis::spirit_markers::SpiritMarker::Taiji => "得到精神/灵性庇护，在宗教或哲学上有才华",
            eon_saju::analysis::spirit_markers::SpiritMarker::Yuede => "得月令功德，化解灾难祸患",
            eon_saju::analysis::spirit_markers::SpiritMarker::Tiande => "得天道福泽，避开凶险祸事",
            eon_saju::analysis::spirit_markers::SpiritMarker::Zhenglu => "获得正当财富，禄米俸禄丰厚",
            eon_saju::analysis::spirit_markers::SpiritMarker::Jinyu => "命运高贵华丽，易获贵人相助",
            eon_saju::analysis::spirit_markers::SpiritMarker::Anlu => "拥有暗中帮助的隐形财运",
            eon_saju::analysis::spirit_markers::SpiritMarker::Xuetang => "利于学术深造与功名成就",
            eon_saju::analysis::spirit_markers::SpiritMarker::TianyiMedical => "在医疗、康复、心理咨询等领域有天赋",
            eon_saju::analysis::spirit_markers::SpiritMarker::Tianwen => "在学术与艺术领域有特殊才华",
            eon_saju::analysis::spirit_markers::SpiritMarker::Yima => "奔波奔忙，多变动迁移，利于向外发展",
            eon_saju::analysis::spirit_markers::SpiritMarker::Huagai => "艺术感悟力极强，性格倾向于孤高独特",
            eon_saju::analysis::spirit_markers::SpiritMarker::Kuigang => "性格刚毅果断，具有强大的领导力与威严",
            eon_saju::analysis::spirit_markers::SpiritMarker::Taohua => "异性缘佳，人缘好，具吸引力，注意感情波动",
            eon_saju::analysis::spirit_markers::SpiritMarker::Hongyan => "魅力四射，招人喜爱，注意避免色情纠纷",
            eon_saju::analysis::spirit_markers::SpiritMarker::Guchen => "孤独感强，自主独立，适合独自摸索",
            eon_saju::analysis::spirit_markers::SpiritMarker::Guasu => "配偶缘分偏弱，性格偏向独处",
            eon_saju::analysis::spirit_markers::SpiritMarker::Xuanzhen => "性格敏锐犀利，直觉强，言语如针",
            eon_saju::analysis::spirit_markers::SpiritMarker::Baihu => "注意血光之灾、意外伤害 or 健康问题",
            eon_saju::analysis::spirit_markers::SpiritMarker::Wangshen => "易因疏忽暴露短处，但也伴随华丽的转机",
            eon_saju::analysis::spirit_markers::SpiritMarker::Jiesha => "外部环境带来的被迫变动或财产损失",
            eon_saju::analysis::spirit_markers::SpiritMarker::Yuanzhen => "人际关系容易出现误会、口舌与不和",
            eon_saju::analysis::spirit_markers::SpiritMarker::Jaesha => "易遭灾祸或受到他人限制、诉讼等约束",
            eon_saju::analysis::spirit_markers::SpiritMarker::Cheonsha => "自然法则或无法抗拒的外力带来的变化",
            eon_saju::analysis::spirit_markers::SpiritMarker::Jisha => "地理位置的变动、出差或活动领域扩展",
            eon_saju::analysis::spirit_markers::SpiritMarker::Nyeonsha => "充满大众魅力与时尚美感，受瞩目",
            eon_saju::analysis::spirit_markers::SpiritMarker::Wolsha => "在艰苦环境或危机中突然获得意外收获",
            eon_saju::analysis::spirit_markers::SpiritMarker::Jangseong => "在行业或团队中登上顶峰의 领袖气质",
            eon_saju::analysis::spirit_markers::SpiritMarker::Banan => "工作生活安稳，获得体面的名誉와 回报",
            eon_saju::analysis::spirit_markers::SpiritMarker::Yukhae => "身体微恙，但处理危机极其민첩快速",
        },
        Locale::En => match marker {
            eon_saju::analysis::spirit_markers::SpiritMarker::Tianyi => "Overcoming difficulties and finding luck with help from benefactors.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Wenchang => "Favorable for academic pursuits, exams, and signing official documents.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Taiji => "Spiritual protection and natural talent in religion, psychology, or philosophy.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Yuede => "Avoiding disasters and finding peace due to monthly virtues.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Tiande => "Escaping misfortunes and negative energies through heavenly protection.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Zhenglu => "Securing rightful wealth, stable income, and career prosperity.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Jinyu => "A noble and elegant destiny, attracting helper relationships.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Anlu => "Possessing hidden channels of wealth and unexpected support.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Xuetang => "Favorable for high academic achievements and deep research.",
            eon_saju::analysis::spirit_markers::SpiritMarker::TianyiMedical => "Natural talent in medicine, counseling, or therapeutic healing.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Tianwen => "Special academic and artistic talents.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Yima => "Frequent travels, migration, and positive changes through mobility.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Huagai => "High artistic sensitivity, creative talents, and solitary tendencies.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Kuigang => "Strong charisma, leadership, and unwavering determination.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Taohua => "Strong attraction to the opposite sex, popularity, and active social life.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Hongyan => "Deep romantic charm, charisma, caution needed against emotional conflicts.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Guchen => "Independent spirit, enjoying solitude and self-reliance.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Guasu => "Fewer opportunities with spouses, preferring independent living.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Xuanzhen => "Sharp intellect, critical thinking, and diagnostic/surgical talent.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Baihu => "Caution needed against physical injury, sudden accidents, or health issues.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Wangshen => "Risk of public mistakes or exposure, but brings opportunity for dramatic rebirth.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Jiesha => "Loss of property or forced changes due to external pressure.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Yuanzhen => "Potential disharmony, misunderstandings, or verbal disputes in relations.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Jaesha => "Accidents, disasters, or being restricted by others/litigation.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Cheonsha => "Inevitable changes shaped by natural laws or external forces.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Jisha => "Geographical relocation, business travel, or expanding activity spheres.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Nyeonsha => "Public popularity, glamour, and fashionable appeal (Peach Blossom).",
            eon_saju::analysis::spirit_markers::SpiritMarker::Wolsha => "Unexpected success or breakthroughs emerging from hard times.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Jangseong => "Leadership qualities that elevate you to the top of your domain.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Banan => "Comfortable social status, honorable achievements, and stable rewards.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Yukhae => "Minor illnesses, but endows you with extremely quick crisis response.",
        },
        Locale::Ru => match marker {
            eon_saju::analysis::spirit_markers::SpiritMarker::Tianyi => "Преодоление трудностей и обретение удачи с помощью благородных покровителей.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Wenchang => "Благоприятно для учебы, экзаменов и подписания официальных документов.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Taiji => "Духовная защита, талант в религии, психологии или философии.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Yuede => "Избежание бедствий и обретение покоя благодаря месячной добродетели.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Tiande => "Устранение невзгод через покровительство Небесной добродетели.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Zhenglu => "Обретение заслуженного богатства, стабильный доход и процветание.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Jinyu => "Благородная судьба, привлечение высокопоставленных партнеров.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Anlu => "Обладание скрытыми каналами богатства и неожиданной поддержкой.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Xuetang => "Благоприятно для академических успехов и глубоких исследований.",
            eon_saju::analysis::spirit_markers::SpiritMarker::TianyiMedical => "Природный дар в медицине, целительстве или психотерапии.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Tianwen => "Особые таланты в науках и искусстве.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Yima => "Частые поездки, переезды и позитивные перемены благодаря мобильности.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Huagai => "Тонкий художественный вкус, творчество и склонность к уединению.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Kuigang => "Сильная харизма, лидерство и непоколебимая решимость.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Taohua => "Привлекательность для противоположного пола, популярность.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Hongyan => "Романтическое очарование, требуется осторожность во избежание ссор.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Guchen => "Независимый дух, любовь к уединению и самодостаточность.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Guasu => "Ограниченные возможности для брака, предпочтение одиночеству.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Xuanzhen => "Острый ум, аналитика, хирургический или целительский талант.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Baihu => "Необходима осторожность из-за риска травм или проблем со здоровьем.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Wangshen => "Риск публичных ошибок, но дает шанс на кардинальное перерождение.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Jiesha => "Утрата имущества или вынужденные перемены под внешним давлением.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Yuanzhen => "Возможны недопонимания, ссоры или обиды в отношениях.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Jaesha => "Ша Катастрофы, несчастные случаи или ограничения со стороны других людей.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Cheonsha => "Неизбежные перемены под воздействием природных законов или высших сил.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Jisha => "Смена места жительства, командировки или расширение сферы деятельности.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Nyeonsha => "Популярность у публики, обаяние (Цветок Персика).",
            eon_saju::analysis::spirit_markers::SpiritMarker::Wolsha => "Неожиданный успех, рождающийся в трудные времена.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Jangseong => "Лидерские качества, возвышающие вас на вершину вашей сферы.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Banan => "Комфортный социальный статус, почетные награды и стабильный доход.",
            eon_saju::analysis::spirit_markers::SpiritMarker::Yukhae => "Ша Шести Вредов, легкие недомогания, но дает молниеносную реакцию.",
        },
    }
}

pub fn translate_saju_structure(locale: Locale, st: eon_saju::analysis::structure::StructureType) -> &'static str {
    match locale {
        Locale::Ko => st.hangul(),
        Locale::Zh => st.hanja(),
        Locale::En => match st {
            eon_saju::analysis::structure::StructureType::ShiShen => "Eating God Structure",
            eon_saju::analysis::structure::StructureType::ShangGuan => "Hurting Officer Structure",
            eon_saju::analysis::structure::StructureType::PianCai => "Indirect Wealth Structure",
            eon_saju::analysis::structure::StructureType::ZhengCai => "Direct Wealth Structure",
            eon_saju::analysis::structure::StructureType::PianGuan => "7 Killings Structure",
            eon_saju::analysis::structure::StructureType::ZhengGuan => "Direct Officer Structure",
            eon_saju::analysis::structure::StructureType::PianYin => "Indirect Resource Structure",
            eon_saju::analysis::structure::StructureType::ZhengYin => "Direct Resource Structure",
            eon_saju::analysis::structure::StructureType::JianLu => "Built Lu Structure (Prosperity)",
            eon_saju::analysis::structure::StructureType::YangIn => "Yang Blade Structure",
            eon_saju::analysis::structure::StructureType::Special => "Companion Structure",
            eon_saju::analysis::structure::StructureType::JongAh => "Follow Child Structure",
            eon_saju::analysis::structure::StructureType::JongJae => "Follow Wealth Structure",
            eon_saju::analysis::structure::StructureType::JongSal => "Follow Power Structure",
            eon_saju::analysis::structure::StructureType::JongGang => "Follow Strength Structure",
            eon_saju::analysis::structure::StructureType::JongWang => "Follow Dominance Structure",
            eon_saju::analysis::structure::StructureType::Follower => "Follower Structure",
            eon_saju::analysis::structure::StructureType::SpecialTransformation => "Dominant One-Element Structure",
        },
        Locale::Ru => match st {
            eon_saju::analysis::structure::StructureType::ShiShen => "Структура Духа Пищи",
            eon_saju::analysis::structure::StructureType::ShangGuan => "Структура Вызова Власти",
            eon_saju::analysis::structure::StructureType::PianCai => "Структура Косвенного Богатства",
            eon_saju::analysis::structure::StructureType::ZhengCai => "Структура Прямого Богатства",
            eon_saju::analysis::structure::StructureType::PianGuan => "Структура Семи Убийц",
            eon_saju::analysis::structure::StructureType::ZhengGuan => "Структура Прямого Чиновника",
            eon_saju::analysis::structure::StructureType::PianYin => "Структура Косвенного Ресурса",
            eon_saju::analysis::structure::StructureType::ZhengYin => "Структура Прямого Ресурса",
            eon_saju::analysis::structure::StructureType::JianLu => "Структура Службы (Лу)",
            eon_saju::analysis::structure::StructureType::YangIn => "Структура Овечьего Ножа",
            eon_saju::analysis::structure::StructureType::Special => "Структура Равного Плеча",
            eon_saju::analysis::structure::StructureType::JongAh => "Структура Следования за Дитя",
            eon_saju::analysis::structure::StructureType::JongJae => "Структура Следования за Богатством",
            eon_saju::analysis::structure::StructureType::JongSal => "Структура Следования за Властью",
            eon_saju::analysis::structure::StructureType::JongGang => "Структура Следования за Силой",
            eon_saju::analysis::structure::StructureType::JongWang => "Структура Следования за Пиком",
            eon_saju::analysis::structure::StructureType::Follower => "Структура Следования",
            eon_saju::analysis::structure::StructureType::SpecialTransformation => "Специальная Структура Трансформации",
        },
    }
}

pub fn translate_saju_structure_summary(
    locale: Locale,
    st: eon_saju::analysis::structure::StructureType,
    path: Option<&str>,
) -> String {
    let name = translate_saju_structure(locale, st);
    match locale {
        Locale::Ko => {
            if st == eon_saju::analysis::structure::StructureType::JianLu {
                "일간이 월지에서 기운을 얻은 건록격".to_string()
            } else if st == eon_saju::analysis::structure::StructureType::YangIn {
                "가장 강렬한 기운을 품은 양인격".to_string()
            } else if let Some(p) = path {
                let p_ko = match p {
                    "시간" => "시간",
                    "일간" => "일간",
                    "월간" => "월간",
                    "연간" => "연간",
                    _ => p,
                };
                format!("지장간의 기운이 {}에 투출된 {}", p_ko, name)
            } else {
                format!("월지의 본기를 격으로 삼은 {}", name)
            }
        }
        Locale::Zh => {
            if st == eon_saju::analysis::structure::StructureType::JianLu {
                "日干得月令之气建禄格".to_string()
            } else if st == eon_saju::analysis::structure::StructureType::YangIn {
                "蕴含最强气势的阳刃格".to_string()
            } else if let Some(p) = path {
                let p_zh = match p {
                    "시간" | "Hour" | "Hour Stem" | "시주" => "时干",
                    "일간" | "Day" | "Day Stem" | "일주" => "日干",
                    "월간" | "Month" | "Month Stem" | "월주" => "月干",
                    "연간" | "Year" | "Year Stem" | "연주" => "年干",
                    _ => p,
                };
                format!("地支藏干透出至{}之{}", p_zh, name)
            } else {
                format!("以月令本气定格局之{}", name)
            }
        }
        Locale::En => {
            if st == eon_saju::analysis::structure::StructureType::JianLu {
                "Jian Lu Structure (Built Lu) gaining strength from Month Branch".to_string()
            } else if st == eon_saju::analysis::structure::StructureType::YangIn {
                "Yang Blade Structure holding the strongest energy".to_string()
            } else if let Some(p) = path {
                let p_en = match p {
                    "시간" | "Hour" | "Hour Stem" | "시주" => "Hour Stem",
                    "일간" | "Day" | "Day Stem" | "일주" => "Day Stem",
                    "월간" | "Month" | "Month Stem" | "월주" => "Month Stem",
                    "연간" | "Year" | "Year Stem" | "연주" => "Year Stem",
                    _ => p,
                };
                format!("{} projected to {}", name, p_en)
            } else {
                format!("{} based on the main element of Month Branch", name)
            }
        }
        Locale::Ru => {
            if st == eon_saju::analysis::structure::StructureType::JianLu {
                "Структура Цзянь Лу (Служба), черпающая силу в земной ветви месяца".to_string()
            } else if st == eon_saju::analysis::structure::StructureType::YangIn {
                "Структура Янского Ножа, несущая самую мощную энергию".to_string()
            } else if let Some(p) = path {
                let p_ru = match p {
                    "시간" | "Hour" | "Hour Stem" | "시주" => "небесный ствол часа",
                    "일간" | "Day" | "Day Stem" | "일주" => "небесный ствол дня",
                    "월간" | "Month" | "Month Stem" | "월주" => "небесный ствол месяца",
                    "연간" | "Year" | "Year Stem" | "연주" => "небесный ствол года",
                    _ => p,
                };
                format!("{} проецируется на {}", name, p_ru)
            } else {
                format!("{} на основе основного элемента земной ветви месяца", name)
            }
        }
    }
}

pub fn translate_saju_structure_desc(
    locale: Locale,
    st: eon_saju::analysis::structure::StructureType,
    has_projection: bool,
) -> &'static str {
    match locale {
        Locale::Ko => {
            if st == eon_saju::analysis::structure::StructureType::JianLu {
                "일간이 가장 왕성한 기운을 가진 시기에 태어나 주관이 뚜렷하고 자수성가할 힘이 있습니다."
            } else if st == eon_saju::analysis::structure::StructureType::YangIn {
                "기운이 너무 강하여 칼을 든 것과 같으니, 이를 잘 다스리면 큰 권위를 얻습니다."
            } else if has_projection {
                "월지의 숨은 기운이 천간으로 고개를 내밀어 사주의 핵심 성격이 되었습니다."
            } else {
                "천간에 드러난 기운은 없으나 태어난 계절의 기운이 가장 강력한 성격을 형성합니다."
            }
        }
        Locale::Zh => {
            if st == eon_saju::analysis::structure::StructureType::JianLu {
                "日干诞生于最旺盛的时期，主观意识明确，具有白手起家的强大力量。"
            } else if st == eon_saju::analysis::structure::StructureType::YangIn {
                "气势过强如同手握利刃，若能妥善驾驭，必能获得极高权威与成就。"
            } else if has_projection {
                "月令蕴含的潜能透出至天干，显露无遗，成为八字格局的核心特质。"
            } else {
                "虽无天干显露，但所生季节的气场极强，塑造了最核心的性格底色。"
            }
        }
        Locale::En => {
            if st == eon_saju::analysis::structure::StructureType::JianLu {
                "Born during the most robust period, you possess strong self-determination and the power to achieve self-made success."
            } else if st == eon_saju::analysis::structure::StructureType::YangIn {
                "The energy is so intense it is like holding a sword; if handled and balanced well, it brings great authority."
            } else if has_projection {
                "The hidden potential of the Month Branch has projected to the Heavenly Stems, becoming the core character of the destiny chart."
            } else {
                "Although no energy is explicitly projected to the stems, the seasonal force of birth forms the most dominant character trait."
            }
        }
        Locale::Ru => {
            if st == eon_saju::analysis::structure::StructureType::JianLu {
                "Рожденный в самый сильный период, вы обладаете решительностью и силой для достижения успеха своими руками."
            } else if st == eon_saju::analysis::structure::StructureType::YangIn {
                "Энергия настолько интенсивна, что напоминает меч; при правильном балансе это приносит огромную власть и авторитет."
            } else if has_projection {
                "Скрытый потенциал Земной ветви месяца спроецировался на Небесные стволы, став ключевой структурой карты судьбы."
            } else {
                "Хотя на стволах нет явных проекций, доминирующая сила сезона рождения формирует основную черту характера."
            }
        }
    }
}

pub fn translate_saju_void_desc(locale: Locale, position: &str) -> &'static str {
    match locale {
        Locale::Ko => match position {
            "년주" | "Year" | "Year Pillar" => "선조나 국가적 혜택이 약하거나, 어린 시절의 근간이 흔들릴 수 있음을 의미합니다.",
            "월주" | "Month" | "Month Pillar" => "부모/형제운이 약하거나 직업적 정착에 더 많은 노력이 필요할 수 있습니다.",
            "시주" | "Hour" | "Hour Pillar" => "자녀나 말년의 결실이 예상보다 늦게 나타나거나 허망함이 있을 수 있습니다.",
            _ => "",
        },
        Locale::Zh => match position {
            "년주" | "Year" | "Year Pillar" => "意味着祖先或国家的庇护较弱，或者童年时期的根基可能不够稳固。",
            "월주" | "Month" | "Month Pillar" => "意味着父母/兄弟运势偏弱，或者在职业定位上需要付出更多努力。",
            "시주" | "Hour" | "Hour Pillar" => "意味着子女或晚年成果可能比预期来得晚，或者可能会感到虚空。",
            _ => "",
        },
        Locale::En => match position {
            "년주" | "Year" | "Year Pillar" => "Indicates that ancestral or state support might be weak, or the foundations of early childhood could be unstable.",
            "월주" | "Month" | "Month Pillar" => "Indicates that parent/sibling support might be weak, or more effort is needed to establish professional stability.",
            "시주" | "Hour" | "Hour Pillar" => "Indicates that children or late-life achievements might manifest later than expected, or there might be feelings of emptiness.",
            _ => "",
        },
        Locale::Ru => match position {
            "년주" | "Year" | "Year Pillar" => "Означает, что поддержка предков или государства может быть слабой, либо основы раннего детства могут быть нестабильными.",
            "월주" | "Month" | "Month Pillar" => "Означает, что поддержка родителей/братьев и сестер может быть слабой, либо требуется больше усилий для карьерной стабильности.",
            "시주" | "Hour" | "Hour Pillar" => "Означает, что дети или достижения в конце жизни могут проявиться позже, чем ожидалось, либо может возникнуть чувство опустошенности.",
            _ => "",
        },
    }
}

pub fn translate_saju_yongshin_type(locale: Locale, yt: eon_saju::analysis::yongshin::YongshinType) -> &'static str {
    match locale {
        Locale::Ko => yt.hangul(),
        Locale::Zh => match yt {
            eon_saju::analysis::yongshin::YongshinType::Eokbu => "抑扶用神",
            eon_saju::analysis::yongshin::YongshinType::Johu => "调候用神",
            eon_saju::analysis::yongshin::YongshinType::Tonggwan => "通关用神",
            eon_saju::analysis::yongshin::YongshinType::Byeongyak => "病药用神",
        },
        Locale::En => match yt {
            eon_saju::analysis::yongshin::YongshinType::Eokbu => "Balance (Eokbu)",
            eon_saju::analysis::yongshin::YongshinType::Johu => "Climate (Johu)",
            eon_saju::analysis::yongshin::YongshinType::Tonggwan => "Arbitration (Tonggwan)",
            eon_saju::analysis::yongshin::YongshinType::Byeongyak => "Remedy (Byeongyak)",
        },
        Locale::Ru => match yt {
            eon_saju::analysis::yongshin::YongshinType::Eokbu => "Ослабление/Усиление (Экбу)",
            eon_saju::analysis::yongshin::YongshinType::Johu => "Климат (Джоху)",
            eon_saju::analysis::yongshin::YongshinType::Tonggwan => "Посредничество (Тонгван)",
            eon_saju::analysis::yongshin::YongshinType::Byeongyak => "Лекарство и Болезнь (Бёньяк)",
        },
    }
}

pub fn translate_saju_strength_type(locale: Locale, st: eon_saju::analysis::strength::StrengthType) -> &'static str {
    match locale {
        Locale::Ko => match st {
            eon_saju::analysis::strength::StrengthType::Strong => "신강 (Strong)",
            eon_saju::analysis::strength::StrengthType::Weak => "신약 (Weak)",
            eon_saju::analysis::strength::StrengthType::Balanced => "중화 (Balanced)",
        },
        Locale::Zh => match st {
            eon_saju::analysis::strength::StrengthType::Strong => "身强",
            eon_saju::analysis::strength::StrengthType::Weak => "身弱",
            eon_saju::analysis::strength::StrengthType::Balanced => "中和",
        },
        Locale::En => match st {
            eon_saju::analysis::strength::StrengthType::Strong => "Strong",
            eon_saju::analysis::strength::StrengthType::Weak => "Weak",
            eon_saju::analysis::strength::StrengthType::Balanced => "Balanced",
        },
        Locale::Ru => match st {
            eon_saju::analysis::strength::StrengthType::Strong => "Сильный (Strong)",
            eon_saju::analysis::strength::StrengthType::Weak => "Слабый (Weak)",
            eon_saju::analysis::strength::StrengthType::Balanced => "Нейтральный (Balanced)",
        },
    }
}

pub fn translate_saju_stem_combination(locale: Locale, comb: eon_saju::analysis::relationships::StemCombination) -> &'static str {
    match locale {
        Locale::Ko => comb.hangul(),
        Locale::Zh => comb.hanja(),
        Locale::En => match comb {
            eon_saju::analysis::relationships::StemCombination::JiaJi => "Jia-Ji Harmony",
            eon_saju::analysis::relationships::StemCombination::YiGeng => "Yi-Geng Harmony",
            eon_saju::analysis::relationships::StemCombination::BingXin => "Bing-Xin Harmony",
            eon_saju::analysis::relationships::StemCombination::DingRen => "Ding-Ren Harmony",
            eon_saju::analysis::relationships::StemCombination::WuGui => "Wu-Gui Harmony",
        },
        Locale::Ru => match comb {
            eon_saju::analysis::relationships::StemCombination::JiaJi => "Слияние Цзя-Цзи",
            eon_saju::analysis::relationships::StemCombination::YiGeng => "Слияние И-Гэн",
            eon_saju::analysis::relationships::StemCombination::BingXin => "Слияние Бин-Синь",
            eon_saju::analysis::relationships::StemCombination::DingRen => "Слияние Дин-Жэнь",
            eon_saju::analysis::relationships::StemCombination::WuGui => "Слияние У-Гуй",
        },
    }
}

pub fn translate_saju_ganzi(locale: Locale, gz: &eon_saju::core::ganzi::GanZi) -> String {
    format!("{}{}", translate_saju_stem(locale, gz.stem), translate_saju_branch(locale, gz.branch))
}

pub fn translate_saju_twelve_stage_str(locale: Locale, stage: &str) -> &str {
    let parsed = match stage {
        "장생" | "Changsheng" => Some(eon_saju::core::twelve_stages::TwelveStage::Changsheng),
        "목욕" | "Muyu" => Some(eon_saju::core::twelve_stages::TwelveStage::Muyu),
        "관대" | "Guandai" => Some(eon_saju::core::twelve_stages::TwelveStage::Guandai),
        "건록" | "Jianlu" => Some(eon_saju::core::twelve_stages::TwelveStage::Jianlu),
        "제왕" | "Diwang" => Some(eon_saju::core::twelve_stages::TwelveStage::Diwang),
        "쇠" | "Shuai" => Some(eon_saju::core::twelve_stages::TwelveStage::Shuai),
        "병" | "Bing" => Some(eon_saju::core::twelve_stages::TwelveStage::Bing),
        "사" | "Si" => Some(eon_saju::core::twelve_stages::TwelveStage::Si),
        "묘" | "Mu" => Some(eon_saju::core::twelve_stages::TwelveStage::Mu),
        "절" | "Jue" => Some(eon_saju::core::twelve_stages::TwelveStage::Jue),
        "태" | "Tai" => Some(eon_saju::core::twelve_stages::TwelveStage::Tai),
        "양" | "Yang" => Some(eon_saju::core::twelve_stages::TwelveStage::Yang),
        _ => None,
    };
    if let Some(st) = parsed {
        translate_saju_twelve_stage(locale, st)
    } else {
        stage
    }
}

pub fn translate_saju_relation_str(locale: Locale, s: &str) -> String {
    if s.is_empty() { return s.to_string(); }
    
    match s {
        "천간합" => return match locale {
            Locale::Ko => "천간합".to_string(),
            Locale::En => "Stem Harmony".to_string(),
            Locale::Zh => "天干合".to_string(),
            Locale::Ru => "Слияние Небесных Стволов".to_string(),
        },
        "지합" | "지지합" => return match locale {
            Locale::Ko => "지합".to_string(),
            Locale::En => "Branch Harmony".to_string(),
            Locale::Zh => "地支合".to_string(),
            Locale::Ru => "Слияние Земных Ветвей".to_string(),
        },
        _ => {}
    }

    let mut stems_or_branches = Vec::new();
    let mut relation_type = "";
    
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        
        let stem = match c {
            '갑' => Some(eon_saju::core::stem::HeavenlyStem::Jia),
            '을' => Some(eon_saju::core::stem::HeavenlyStem::Yi),
            '병' => Some(eon_saju::core::stem::HeavenlyStem::Bing),
            '정' => Some(eon_saju::core::stem::HeavenlyStem::Ding),
            '무' => Some(eon_saju::core::stem::HeavenlyStem::Wu),
            '기' => Some(eon_saju::core::stem::HeavenlyStem::Ji),
            '경' => Some(eon_saju::core::stem::HeavenlyStem::Geng),
            '신' if i + 1 < chars.len() && (chars[i+1] == '살' || chars[i+1] == '구') => None,
            '신' => Some(eon_saju::core::stem::HeavenlyStem::Xin),
            '임' => Some(eon_saju::core::stem::HeavenlyStem::Ren),
            '계' => Some(eon_saju::core::stem::HeavenlyStem::Gui),
            _ => None,
        };
        
        if let Some(st) = stem {
            stems_or_branches.push(translate_saju_stem(locale, st));
            i += 1;
            continue;
        }

        let branch = match c {
            '자' if i + 1 < chars.len() && chars[i+1] == '형' => None,
            '자' => Some(eon_saju::core::branch::EarthlyBranch::Zi),
            '축' => Some(eon_saju::core::branch::EarthlyBranch::Chou),
            '인' => Some(eon_saju::core::branch::EarthlyBranch::Yin),
            '묘' => Some(eon_saju::core::branch::EarthlyBranch::Mao),
            '진' => Some(eon_saju::core::branch::EarthlyBranch::Chen),
            '사' => Some(eon_saju::core::branch::EarthlyBranch::Si),
            '오' => Some(eon_saju::core::branch::EarthlyBranch::Wu),
            '미' => Some(eon_saju::core::branch::EarthlyBranch::Wei),
            '신' => Some(eon_saju::core::branch::EarthlyBranch::Shen),
            '유' => Some(eon_saju::core::branch::EarthlyBranch::You),
            '술' => Some(eon_saju::core::branch::EarthlyBranch::Xu),
            '해' => Some(eon_saju::core::branch::EarthlyBranch::Hai),
            _ => None,
        };

        if let Some(br) = branch {
            stems_or_branches.push(translate_saju_branch(locale, br));
            i += 1;
            continue;
        }

        let remaining_chars = &chars[i..];
        if remaining_chars.starts_with(&['삼', '합']) {
            relation_type = "삼합";
            i += 2;
        } else if remaining_chars.starts_with(&['방', '합']) {
            relation_type = "방합";
            i += 2;
        } else if remaining_chars.starts_with(&['반', '합']) {
            relation_type = "반합";
            i += 2;
        } else if remaining_chars.starts_with(&['육', '합']) {
            relation_type = "육합";
            i += 2;
        } else if remaining_chars.starts_with(&['자', '형']) {
            relation_type = "자형";
            i += 2;
        } else if remaining_chars.starts_with(&['삼', '형']) {
            relation_type = "삼형";
            i += 2;
        } else if remaining_chars.starts_with(&['충']) {
            relation_type = "충";
            i += 1;
        } else if remaining_chars.starts_with(&['형']) {
            relation_type = "형";
            i += 1;
        } else if remaining_chars.starts_with(&['해']) {
            relation_type = "해";
            i += 1;
        } else if remaining_chars.starts_with(&['파']) {
            relation_type = "파";
            i += 1;
        } else if remaining_chars.starts_with(&['합']) {
            relation_type = "합";
            i += 1;
        } else if remaining_chars.starts_with(&['원', '진']) {
            relation_type = "원진";
            i += 2;
        } else if remaining_chars.starts_with(&['귀', '문']) {
            relation_type = "귀문";
            i += 2;
        } else {
            i += 1;
        }
    }

    if stems_or_branches.is_empty() {
        return s.to_string();
    }

    let joined_elements = stems_or_branches.join("-");
    match locale {
        Locale::Ko => s.to_string(),
        Locale::Zh => {
            let suffix = match relation_type {
                "삼합" => "三合",
                "방합" => "方合",
                "반합" => "半合",
                "육합" => "六合",
                "자형" => "自刑",
                "삼형" => "三刑",
                "충" => "冲",
                "형" => "刑",
                "해" => "害",
                "파" => "破",
                "합" => "合",
                "원진" => "怨嗔",
                "귀문" => "鬼门",
                _ => "",
            };
            format!("{}{}", stems_or_branches.join(""), suffix)
        },
        Locale::En => {
            let suffix = match relation_type {
                "삼합" => " Triple Harmony",
                "방합" => " Seasonal Harmony",
                "반합" => " Semi-Harmony",
                "육합" => " Six Harmony",
                "자형" => " Self Punishment",
                "삼형" => " Triple Punishment",
                "충" => " Clash",
                "형" => " Punishment",
                "해" => " Harm",
                "파" => " Destruction",
                "합" => " Harmony",
                "원진" => " Resentment (Yuan Zhen)",
                "귀문" => " Ghost Gate",
                _ => "",
            };
            format!("{}{}", joined_elements, suffix)
        },
        Locale::Ru => {
            let prefix = match relation_type {
                "삼합" => "Тройное Слияние",
                "방합" => "Сезонное Слияние",
                "반합" => "Полуслияние",
                "육합" => "Парное Слияние",
                "자형" => "Самонакзание",
                "삼형" => "Тройное Наказание",
                "충" => "Столкновение",
                "형" => "Наказание",
                "해" => "Вред",
                "파" => "Разрушение",
                "합" => "Слияние",
                "원진" => "Несогласие (Юань Чжэнь)",
                "귀문" => "Врата Демонов",
                _ => "",
            };
            if prefix.is_empty() {
                joined_elements
            } else {
                format!("{} {}", prefix, joined_elements)
            }
        }
    }
}

pub fn translate_saju_tag_str(locale: Locale, tag: &str) -> String {
    if locale == Locale::Ko { return tag.to_string(); }

    let mut t = tag.to_string();

    t = t.replace("대운", match locale {
        Locale::En => "Major ",
        Locale::Zh => "大运",
        Locale::Ru => "Б.удачи ",
        _ => "Major ",
    });
    t = t.replace("세운", match locale {
        Locale::En => "Yearly ",
        Locale::Zh => "流年",
        Locale::Ru => "Г.удачи ",
        _ => "Yearly ",
    });
    t = t.replace("월운", match locale {
        Locale::En => "Monthly ",
        Locale::Zh => "月运",
        Locale::Ru => "М.удачи ",
        _ => "Monthly ",
    });
    t = t.replace("일운", match locale {
        Locale::En => "Daily ",
        Locale::Zh => "日运",
        Locale::Ru => "Д.удачи ",
        _ => "Daily ",
    });

    t = t.replace("핵심운", match locale {
        Locale::En => "Core Luck",
        Locale::Zh => "核心运",
        Locale::Ru => "Ключевая удача",
        _ => "Core Luck",
    });
    t = t.replace("길운", match locale {
        Locale::En => "Good Luck",
        Locale::Zh => "吉运",
        Locale::Ru => "Благоприятная удача",
        _ => "Good Luck",
    });
    t = t.replace("기신운", match locale {
        Locale::En => "Bad Luck",
        Locale::Zh => "忌神运",
        Locale::Ru => "Неблагоприятная удача",
        _ => "Bad Luck",
    });

    t = t.replace("천간충:", match locale {
        Locale::En => "Stem Clash: ",
        Locale::Zh => "天干冲: ",
        Locale::Ru => "Столкновение Стволов: ",
        _ => "Stem Clash: ",
    });
    t = t.replace("육합:", match locale {
        Locale::En => "Six Combo: ",
        Locale::Zh => "六合: ",
        Locale::Ru => "Парное Слияние: ",
        _ => "Six Combo: ",
    });
    t = t.replace("삼합완성", match locale {
        Locale::En => "Triple Combo Complete",
        Locale::Zh => "三合完成",
        Locale::Ru => "Тройное Слияние",
        _ => "Triple Combo Complete",
    });
    t = t.replace("방합완성", match locale {
        Locale::En => "Seasonal Combo Complete",
        Locale::Zh => "方合完成",
        Locale::Ru => "Сезонное Слияние",
        _ => "Seasonal Combo Complete",
    });
    t = t.replace("합화:", match locale {
        Locale::En => "Transformation: ",
        Locale::Zh => "合化: ",
        Locale::Ru => "Превращение: ",
        _ => "Transformation: ",
    });

    t = t.replace("운성공망:", match locale {
        Locale::En => "Void Luck: ",
        Locale::Zh => "运星空亡: ",
        Locale::Ru => "Пустота Удачи: ",
        _ => "Void Luck: ",
    });
    t = t.replace("탈공:충", match locale {
        Locale::En => "Void Escaped: Clash",
        Locale::Zh => "脱空:冲",
        Locale::Ru => "Выход из Пустоты: Столкновение",
        _ => "Void Escaped: Clash",
    });
    t = t.replace("탈공:육합", match locale {
        Locale::En => "Void Escaped: Six Combo",
        Locale::Zh => "脱空:六合",
        Locale::Ru => "Выход из Пустоты: Слияние",
        _ => "Void Escaped: Six Combo",
    });
    t = t.replace("탈공:삼합", match locale {
        Locale::En => "Void Escaped: Triple Combo",
        Locale::Zh => "脱空:三合",
        Locale::Ru => "Выход из Пустоты: Тройное Слияние",
        _ => "Void Escaped: Triple Combo",
    });
    t = t.replace("탈공:방합", match locale {
        Locale::En => "Void Escaped: Seasonal Combo",
        Locale::Zh => "脱空:方合",
        Locale::Ru => "Выход из Пустоты: Сезонное Слияние",
        _ => "Void Escaped: Seasonal Combo",
    });

    t = t.replace("신살:", match locale {
        Locale::En => "Shen Sha: ",
        Locale::Zh => "神煞: ",
        Locale::Ru => "Символическая Звезда: ",
        _ => "Shen Sha: ",
    });
    t = t.replace("길신:", match locale {
        Locale::En => "Auspicious Spirit: ",
        Locale::Zh => "吉神: ",
        Locale::Ru => "Благородный: ",
        _ => "Auspicious Spirit: ",
    });
    t = t.replace("흉살:", match locale {
        Locale::En => "Evil Spirit: ",
        Locale::Zh => "凶煞: ",
        Locale::Ru => "Демон: ",
        _ => "Evil Spirit: ",
    });
    t = t.replace("운성:", match locale {
        Locale::En => "Life Stage: ",
        Locale::Zh => "运星: ",
        Locale::Ru => "Стадия Судьбы: ",
        _ => "Life Stage: ",
    });

    t = t.replace("패턴:상관견관", match locale {
        Locale::En => "Pattern: Hurting Officer Meets Officer",
        Locale::Zh => "格局:伤官见官",
        Locale::Ru => "Паттерн: Вызов Власти встречает Чиновника",
        _ => "Pattern: Hurting Officer Meets Officer",
    });
    t = t.replace("패턴:식신생재", match locale {
        Locale::En => "Pattern: Eating God Produces Wealth",
        Locale::Zh => "格局:食神生财",
        Locale::Ru => "Паттерн: Дух Пищи порождает Богатство",
        _ => "Pattern: Eating God Produces Wealth",
    });

    t = t.replace("신강약:신강", match locale {
        Locale::En => "Strength: Strong",
        Locale::Zh => "身强弱:身强",
        Locale::Ru => "Сила: Сильный",
        _ => "Strength: Strong",
    });
    t = t.replace("신강약:신약", match locale {
        Locale::En => "Strength: Weak",
        Locale::Zh => "身强弱:身弱",
        Locale::Ru => "Сила: Слабый",
        _ => "Strength: Weak",
    });
    t = t.replace("신강약:중화", match locale {
        Locale::En => "Strength: Balanced",
        Locale::Zh => "身强弱:中和",
        Locale::Ru => "Сила: Нейтральный",
        _ => "Strength: Balanced",
    });
    t = t.replace("득령", match locale {
        Locale::En => "Deuk-Ryeong",
        Locale::Zh => "得令",
        Locale::Ru => "Дэ-Рён",
        _ => "Deuk-Ryeong",
    });
    t = t.replace("득지", match locale {
        Locale::En => "Deuk-Ji",
        Locale::Zh => "得地",
        Locale::Ru => "Дэ-Джи",
        _ => "Deuk-Ji",
    });
    t = t.replace("득시", match locale {
        Locale::En => "Deuk-Si",
        Locale::Zh => "得时",
        Locale::Ru => "Дэ-Ши",
        _ => "Deuk-Si",
    });
    t = t.replace("득세", match locale {
        Locale::En => "Deuk-Se",
        Locale::Zh => "得势",
        Locale::Ru => "Дэ-Сэ",
        _ => "Deuk-Se",
    });

    t = t.replace("년지", match locale {
        Locale::En => "Year Branch",
        Locale::Zh => "年支",
        Locale::Ru => "Ветвь Года",
        _ => "Year Branch",
    });
    t = t.replace("월지", match locale {
        Locale::En => "Month Branch",
        Locale::Zh => "月支",
        Locale::Ru => "Ветвь Месяца",
        _ => "Month Branch",
    });
    t = t.replace("일지", match locale {
        Locale::En => "Day Branch",
        Locale::Zh => "日支",
        Locale::Ru => "Ветвь Дня",
        _ => "Day Branch",
    });
    t = t.replace("시지", match locale {
        Locale::En => "Hour Branch",
        Locale::Zh => "时支",
        Locale::Ru => "Ветвь Часа",
        _ => "Hour Branch",
    });

    t = t.replace("장생", translate_saju_twelve_stage_str(locale, "장생"));
    t = t.replace("목욕", translate_saju_twelve_stage_str(locale, "목욕"));
    t = t.replace("관대", translate_saju_twelve_stage_str(locale, "관대"));
    t = t.replace("건록", translate_saju_twelve_stage_str(locale, "건록"));
    t = t.replace("제왕", translate_saju_twelve_stage_str(locale, "제왕"));
    t = t.replace("쇠", translate_saju_twelve_stage_str(locale, "쇠"));
    t = t.replace("병", translate_saju_twelve_stage_str(locale, "병"));
    t = t.replace("사", translate_saju_twelve_stage_str(locale, "사"));
    t = t.replace("묘", translate_saju_twelve_stage_str(locale, "묘"));
    t = t.replace("절", translate_saju_twelve_stage_str(locale, "절"));
    t = t.replace("태", translate_saju_twelve_stage_str(locale, "태"));
    t = t.replace("양", translate_saju_twelve_stage_str(locale, "양"));

    t = t.replace("吉", match locale {
        Locale::Zh => "吉",
        _ => "Auspicious",
    });
    t = t.replace("凶", match locale {
        Locale::Zh => "凶",
        _ => "Caution",
    });

    t = t.replace("목", translate_saju_element(locale, eon_saju::core::element::Element::Wood));
    t = t.replace("화", translate_saju_element(locale, eon_saju::core::element::Element::Fire));
    t = t.replace("토", translate_saju_element(locale, eon_saju::core::element::Element::Earth));
    t = t.replace("금", translate_saju_element(locale, eon_saju::core::element::Element::Metal));
    t = t.replace("수", translate_saju_element(locale, eon_saju::core::element::Element::Water));

    t = t.replace("갑", translate_saju_stem(locale, eon_saju::core::stem::HeavenlyStem::Jia));
    t = t.replace("을", translate_saju_stem(locale, eon_saju::core::stem::HeavenlyStem::Yi));
    t = t.replace("병", translate_saju_stem(locale, eon_saju::core::stem::HeavenlyStem::Bing));
    t = t.replace("정", translate_saju_stem(locale, eon_saju::core::stem::HeavenlyStem::Ding));
    t = t.replace("무", translate_saju_stem(locale, eon_saju::core::stem::HeavenlyStem::Wu));
    t = t.replace("기", translate_saju_stem(locale, eon_saju::core::stem::HeavenlyStem::Ji));
    t = t.replace("경", translate_saju_stem(locale, eon_saju::core::stem::HeavenlyStem::Geng));
    t = t.replace("신", translate_saju_stem(locale, eon_saju::core::stem::HeavenlyStem::Xin));
    t = t.replace("임", translate_saju_stem(locale, eon_saju::core::stem::HeavenlyStem::Ren));
    t = t.replace("계", translate_saju_stem(locale, eon_saju::core::stem::HeavenlyStem::Gui));

    t = t.replace("자", translate_saju_branch(locale, eon_saju::core::branch::EarthlyBranch::Zi));
    t = t.replace("축", translate_saju_branch(locale, eon_saju::core::branch::EarthlyBranch::Chou));
    t = t.replace("인", translate_saju_branch(locale, eon_saju::core::branch::EarthlyBranch::Yin));
    t = t.replace("묘", translate_saju_branch(locale, eon_saju::core::branch::EarthlyBranch::Mao));
    t = t.replace("진", translate_saju_branch(locale, eon_saju::core::branch::EarthlyBranch::Chen));
    t = t.replace("사", translate_saju_branch(locale, eon_saju::core::branch::EarthlyBranch::Si));
    t = t.replace("오", translate_saju_branch(locale, eon_saju::core::branch::EarthlyBranch::Wu));
    t = t.replace("미", translate_saju_branch(locale, eon_saju::core::branch::EarthlyBranch::Wei));
    t = t.replace("신", translate_saju_branch(locale, eon_saju::core::branch::EarthlyBranch::Shen));
    t = t.replace("유", translate_saju_branch(locale, eon_saju::core::branch::EarthlyBranch::You));
    t = t.replace("술", translate_saju_branch(locale, eon_saju::core::branch::EarthlyBranch::Xu));
    t = t.replace("해", translate_saju_branch(locale, eon_saju::core::branch::EarthlyBranch::Hai));

    t = t.replace("Critical_Yongshin_Clash (용신 파괴)", match locale {
        Locale::En => "Critical Yongshin Clash",
        Locale::Zh => "用神破损 (临界冲克)",
        Locale::Ru => "Критическое столкновение Полезного Божества",
        _ => "Critical Yongshin Clash",
    });
    t = t.replace("Elemental_Overflow (기신 과다)", match locale {
        Locale::En => "Elemental Overflow (Harmful Deity)",
        Locale::Zh => "忌神过旺 (五行失衡)",
        Locale::Ru => "Избыток неблагоприятных элементов",
        _ => "Elemental Overflow",
    });
    t = t.replace("Structural_Instability (구조적 불안정)", match locale {
        Locale::En => "Structural Instability",
        Locale::Zh => "结构不稳定 (地支冲克)",
        Locale::Ru => "Структурная нестабильность",
        _ => "Structural Instability",
    });
    t = t.replace("Low_Energy_State (에너지 저하)", match locale {
        Locale::En => "Low Energy State",
        Locale::Zh => "能量低下状态",
        Locale::Ru => "Низкий энергетический уровень",
        _ => "Low Energy State",
    });

    t = translate_aux_shinsal(locale, &t);

    t
}

pub fn translate_saju_load_balancer(locale: Locale, reason: &str, strategy: &str) -> (String, String) {
    if locale == Locale::Ko {
        return (reason.to_string(), strategy.to_string());
    }

    fn extract_num(s: &str) -> String {
        let mut num_str = String::new();
        let mut started = false;
        for c in s.chars() {
            if c.is_ascii_digit() || c == '.' {
                num_str.push(c);
                started = true;
            } else if started {
                break;
            }
        }
        num_str
    }

    let r_trans = if reason.starts_with("운세 급등 구간") {
        let diff = extract_num(reason);
        match locale {
            Locale::En => format!("Fortune Surge (Rise by {} pts)", diff),
            Locale::Zh => format!("运势急剧上升 (上升幅度 {}分)", diff),
            Locale::Ru => format!("Резкий взлет фортуны (рост на {} б.)", diff),
            _ => reason.to_string(),
        }
    } else if reason.starts_with("운세 급락 구간") {
        let diff = extract_num(reason);
        match locale {
            Locale::En => format!("Fortune Plummet (Drop by {} pts)", diff),
            Locale::Zh => format!("运势急剧下降 (下降幅度 {}分)", diff),
            Locale::Ru => format!("Резкое падение фортуны (спад на {} б.)", diff),
            _ => reason.to_string(),
        }
    } else if reason.starts_with("운세 하락 흐름") {
        let diff = extract_num(reason);
        match locale {
            Locale::En => format!("Fortune Decline (Drop by {} pts)", diff),
            Locale::Zh => format!("运势下降趋势 (下降幅度 {}分)", diff),
            Locale::Ru => format!("Тенденция к снижению фортуны (спад на {} б.)", diff),
            _ => reason.to_string(),
        }
    } else if reason.starts_with("평온한 안정 구간") {
        let score = extract_num(reason);
        match locale {
            Locale::En => format!("Peaceful stability period (Maintaining around {} pts)", score),
            Locale::Zh => format!("平稳安定时期 (维持在{}分左右)", score),
            Locale::Ru => format!("Период стабильности (Поддержание около {} б.)", score),
            _ => reason.to_string(),
        }
    } else if reason.starts_with("장기 침체 구간") {
        let age = extract_num(reason);
        match locale {
            Locale::En => format!("Long-term stagnation period (Lasting for 3+ years starting from age {})", age),
            Locale::Zh => format!("长期低迷时期 (从{}岁起持续3年以上)", age),
            Locale::Ru => format!("Период длительного застоя (Длится 3+ лет с возраста {} лет)", age),
            _ => reason.to_string(),
        }
    } else if reason.starts_with("황금기 지속") {
        let age = extract_num(reason);
        match locale {
            Locale::En => format!("Golden period continuation (Favorable fortune lasting 3+ years starting from age {})", age),
            Locale::Zh => format!("黄金期持续 (从{}岁起持续3年以上的佳运)", age),
            Locale::Ru => format!("Продолжение золотого периода (Благоприятная фортуна длится 3+ лет с возраста {} лет)", age),
            _ => reason.to_string(),
        }
    } else {
        match reason {
            "좋은 기운이 강하게 몰리는 최고점 시기입니다." => match locale {
                Locale::En => "Peak period with strongly concentrated positive energy.",
                Locale::Zh => "好运强力汇聚的最高点时期。",
                Locale::Ru => "Пиковый период с сильной концентрацией благоприятной энергии.",
                _ => reason,
            },
            "어려움이 집중되는 저점 시기입니다." => match locale {
                Locale::En => "Low point period where difficulties are concentrated.",
                Locale::Zh => "困难集中的低谷时期。",
                Locale::Ru => "Период спада, в котором концентрируются трудности.",
                _ => reason,
            },
            "인생 최저 구간에 해당하는 매우 어려운 해입니다." => match locale {
                Locale::En => "A very difficult year representing the lowest life point.",
                Locale::Zh => "属于人生最低谷的极其艰难的一年。",
                Locale::Ru => "Очень сложный год, представляющий собой самую низкую точку жизни.",
                _ => reason,
            },
            "운세 반등 구간 — 어두운 시기가 지나고 회복 흐름이 시작됩니다." => match locale {
                Locale::En => "Fortune rebound period — dark times pass and recovery begins.",
                Locale::Zh => "运势反弹时期 — 黑暗时期已过，恢复趋势开始。",
                Locale::Ru => "Период отскока фортуны — темные времена проходят, начинается восстановление.",
                _ => reason,
            },
            _ => reason,
        }.to_string()
    };

    let s_trans = match strategy {
        "상승 에너지가 큰 시기입니다. 기회를 놓치지 않되, 과잉 확장은 주의하세요." => match locale {
            Locale::En => "A period of high rising energy. Grab opportunities, but avoid over-expansion.",
            Locale::Zh => "上升能量巨大的时期。莫失良机，但要注意避免过度扩张。",
            Locale::Ru => "Период сильного подъема энергии. Не упускайте возможности, но избегайте чрезмерного расширения.",
            _ => strategy,
        },
        "에너지가 급격히 줄어드는 시기입니다. 무리한 결정은 미루고 변화에 유연하게 대응하세요." => match locale {
            Locale::En => "A period of rapidly decreasing energy. Postpone crucial decisions and adapt flexibly.",
            Locale::Zh => "能量急剧减少的时期。推迟草率의 决定，灵活应对变化。",
            Locale::Ru => "Период быстрого снижения энергии. Отложите поспешные решения и гибко адаптируйтесь к изменениям.",
            _ => strategy,
        },
        "서서히 에너지가 줄어드는 흐름입니다. 지출과 새로운 투자를 자제하고 내실을 다지세요." => match locale {
            Locale::En => "A trend of gradually decreasing energy. Refrain from spending and new investments.",
            Locale::Zh => "能量逐渐减少的趋势。克制支出与新投资，注重内部稳固。",
            Locale::Ru => "Тенденция к постепенному снижению энергии. Воздержитесь от трат и новых инвестиций.",
            _ => strategy,
        },
        "성취욕이 높아지는 시기이지만 과욕을 경계하고 안정을 함께 챙기세요." => match locale {
            Locale::En => "A period of rising ambition; guard against greed and maintain stability.",
            Locale::Zh => "成就欲高涨的时期，但要警惕贪心，兼顾稳定。",
            Locale::Ru => "Период роста амбиций; остерегайтесь жадности и поддерживайте стабильность.",
            _ => strategy,
        },
        "새로운 도전보다는 현상 유지에 집중하고, 소중한 것을 지키는 데 에너지를 쏟으세요." => match locale {
            Locale::En => "Focus on maintaining status quo rather than new challenges; protect what is precious.",
            Locale::Zh => "专注于维持现状而非迎接新挑战，将能量用于守护珍贵之物。",
            Locale::Ru => "Сосредоточьтесь на сохранении статуса-кво, а не на новых вызовах; берегите то, что дорого.",
            _ => strategy,
        },
        "최소한의 안전망을 확보하고 중요한 결정은 모두 미루세요. 주변의 도움을 적극 구하세요." => match locale {
            Locale::En => "Secure a minimal safety net and postpone all major decisions. Seek help actively.",
            Locale::Zh => "确保最低限度的安全网，推迟所有重大决定。积极寻求周围人的帮助。",
            Locale::Ru => "Обеспечьте минимальную сеть безопасности и отложите все важные решения. Активно ищите помощи.",
            _ => strategy,
        },
        "새로운 시작을 위한 기회가 열리는 시기입니다. 망설임 없이 도전해 보세요." => match locale {
            Locale::En => "Opportunities for a new start are opening. Take on challenges without hesitation.",
            Locale::Zh => "开启新起点的良机已经到来。毫不犹豫地迎接挑战吧。",
            Locale::Ru => "Открываются возможности для нового старта. Принимайте вызовы без колебаний.",
            _ => strategy,
        },
        "큰 변화 없이 안정된 시기입니다. 조용히 역량을 쌓고 다음 기회를 준비하기 좋습니다." => match locale {
            Locale::En => "A stable period without major changes. Great for quietly building skills for next opportunities.",
            Locale::Zh => "没有太大变化的稳定时期。适合默默积蓄力量，为下一次机会做好准备。",
            Locale::Ru => "Стабильный период без серьезных изменений. Отлично подходит для накопления сил перед новыми возможностями.",
            _ => strategy,
        },
        "지금은 새로운 시작보다 내면을 가다듬고 힘을 비축하는 시기입니다. 가까운 사람에게 의지하세요." => match locale {
            Locale::En => "A time to refine your inner self and conserve strength rather than starting fresh. Rely on close ones.",
            Locale::Zh => "现在是收拾心情、积蓄力量的时期，而非开启新起点。多依靠身边亲近的人。",
            Locale::Ru => "Время для самосовершенствования и сбережения сил, а не для нового старта. Положитесь на близких.",
            _ => strategy,
        },
        "좋은 흐름이 이어지고 있습니다. 이 시기에 중요한 목표를 과감하게 추진하세요." => match locale {
            Locale::En => "A good flow continues. Push forward boldly with important goals during this period.",
            Locale::Zh => "良好的运势仍在持续。在此期间大胆推进重要目标。",
            Locale::Ru => "Благоприятный поток продолжается. Смело двигайтесь вперед к важным целям в этот период.",
            _ => strategy,
        },
        _ => strategy,
    }.to_string();

    (r_trans, s_trans)
}

pub fn format_spirit_marker_desc(
    locale: Locale,
    marker: eon_saju::analysis::spirit_markers::SpiritMarker,
    position: eon_saju::analysis::spirit_markers::PillarPosition,
) -> String {
    let base_desc = translate_saju_spirit_marker_desc(locale, marker);
    let pos_context = match position {
        eon_saju::analysis::spirit_markers::PillarPosition::Year => match locale {
            Locale::Ko => "어린 시절이나 조상의 음덕, 사회적 배경에 영향을 줍니다.",
            Locale::En => "Affects early childhood, ancestral blessings, and social background.",
            Locale::Zh => "影响童年时期、祖先庇护及社会背景。",
            Locale::Ru => "Влияет на раннее детство, благословение предков и социальное происхождение.",
        },
        eon_saju::analysis::spirit_markers::PillarPosition::Month => match locale {
            Locale::Ko => "직업적 환경이나 부모 형제, 사회 활동의 중심에서 작용합니다.",
            Locale::En => "Acts on professional environment, parents, siblings, and central social activities.",
            Locale::Zh => "作用于职业环境、父母兄弟及社会活动中心。",
            Locale::Ru => "Влияет на профессиональное окружение, родителей, братьев и сестер, а также на социальную активность.",
        },
        eon_saju::analysis::spirit_markers::PillarPosition::Day => match locale {
            Locale::Ko => "나의 내면적인 기질이나 배우자와의 관계에 깊이 관여합니다.",
            Locale::En => "Deeply involved in inner temperament and relationship with spouse.",
            Locale::Zh => "深切关联内在性格及与配偶的关系。",
            Locale::Ru => "Тесно связано с внутренним темпераментом и отношениями с супругом.",
        },
        eon_saju::analysis::spirit_markers::PillarPosition::Hour => match locale {
            Locale::Ko => "자식운이나 노년의 삶, 비밀스러운 내실을 상징하는 영역에서 나타납니다.",
            Locale::En => "Manifests in areas representing children, late-life, and private inner matters.",
            Locale::Zh => "显现于代表子女、晚年生活及隐秘内心世界的领域。",
            Locale::Ru => "Проявляется в сферах, представляющих детей, старость и тайную внутреннюю жизнь.",
        },
    };
    format!("{} \n\n* {}", base_desc, pos_context)
}

pub fn translate_saju_reason(locale: Locale, reason: &str) -> String {
    let mut r = reason.to_string();
    match locale {
        Locale::Ko => r,
        Locale::En => {
            r = r.replace("연지", "Year Branch");
            r = r.replace("월지", "Month Branch");
            r = r.replace("일지", "Day Branch");
            r = r.replace("시지", "Hour Branch");
            r = r.replace("연간", "Year Stem");
            r = r.replace("월간", "Month Stem");
            r = r.replace("일간", "Day Stem");
            r = r.replace("시간", "Hour Stem");
            r = r.replace("지장간", "Hidden Stem");
            r = r.replace("투출", "Projected");
            r = r.replace("기준", "Basis");
            r = r.replace("천간", "Stem");
            r = r.replace("지지", "Branch");
            r = r.replace("오행: 목", "Element: Wood");
            r = r.replace("오행: 화", "Element: Fire");
            r = r.replace("오행: 토", "Element: Earth");
            r = r.replace("오행: 금", "Element: Metal");
            r = r.replace("오행: 수", "Element: Water");
            r = r.replace("용/희신 적용", "Yong/Hee-shin applied");
            r = r.replace("기신/한신 영향", "Geeshin/Hanshin influence");
            r = r.replace("일간 기준 최고의 길신", "Best auspicious star based on Day Master");
            r = r.replace("우두머리 별의 기운", "Energy of the leader star");
            r = r.replace(" -> ", " → ");
            
            // Replace Korean stems/branches/elements in reasons if any
            r = r.replace("갑", "Jia");
            r = r.replace("을", "Yi");
            r = r.replace("병", "Bing");
            r = r.replace("정", "Ding");
            r = r.replace("무", "Wu");
            r = r.replace("기", "Ji");
            r = r.replace("경", "Geng");
            r = r.replace("신", "Xin");
            r = r.replace("임", "Ren");
            r = r.replace("계", "Gui");
            
            r = r.replace("자", "Zi");
            r = r.replace("축", "Chou");
            r = r.replace("인", "Yin");
            r = r.replace("묘", "Mao");
            r = r.replace("진", "Chen");
            r = r.replace("사", "Si");
            r = r.replace("오", "Wu");
            r = r.replace("미", "Wei");
            r = r.replace("신", "Shen");
            r = r.replace("유", "You");
            r = r.replace("술", "Xu");
            r = r.replace("해", "Hai");
            r
        },
        Locale::Zh => {
            r = r.replace("연지", "年支");
            r = r.replace("월지", "月支");
            r = r.replace("일지", "日支");
            r = r.replace("시지", "时支");
            r = r.replace("연간", "年干");
            r = r.replace("월간", "月干");
            r = r.replace("일간", "日干");
            r = r.replace("시간", "时干");
            r = r.replace("지장간", "藏干");
            r = r.replace("투출", "透出");
            r = r.replace("기준", "基准");
            r = r.replace("천간", "天干");
            r = r.replace("지지", "地支");
            r = r.replace("오행: 목", "五行: 木");
            r = r.replace("오행: 화", "五行: 火");
            r = r.replace("오행: 토", "五行: 土");
            r = r.replace("오행: 금", "五行: 金");
            r = r.replace("오행: 수", "五行: 水");
            r = r.replace("용/희신 적용", "用/喜神适用");
            r = r.replace("기신/한신 영향", "忌神/闲神影响");
            r = r.replace("일간 기준 최고의 길신", "日干基准最佳吉神");
            r = r.replace("우두머리 별의 기운", "领袖之星气场");
            r = r.replace(" -> ", " → ");
            r
        },
        Locale::Ru => {
            r = r.replace("연지", "Ветвь Года");
            r = r.replace("월지", "Ветвь Месяца");
            r = r.replace("일지", "Ветвь Дня");
            r = r.replace("시지", "Ветвь Часа");
            r = r.replace("연간", "Ствол Года");
            r = r.replace("월간", "Ствол Месяца");
            r = r.replace("일간", "Ствол Дня");
            r = r.replace("시간", "Ствол Часа");
            r = r.replace("지장간", "Скрытый Ствол");
            r = r.replace("투출", "Проявлено");
            r = r.replace("기준", "Основа");
            r = r.replace("천간", "Небесный ствол");
            r = r.replace("지지", "Земная ветвь");
            r = r.replace("오행: 목", "Стихия: Дерево");
            r = r.replace("오행: 화", "Стихия: Огонь");
            r = r.replace("오행: 토", "Стихия: Земля");
            r = r.replace("오행: 금", "Стихия: Металл");
            r = r.replace("오행: 수", "Стихия: Вода");
            r = r.replace("용/희신 적용", "Юнсин/Хисин применимо");
            r = r.replace("기신/한신 영향", "Влияние Гисин/Хансин");
            r = r.replace("일간 기준 최고의 길신", "Лучшая благоприятная звезда по Дневному Стволу");
            r = r.replace("우두머리 별의 기운", "Энергия звезды-лидера");
            r = r.replace(" -> ", " → ");
            
            r = r.replace("갑", "Цзя");
            r = r.replace("을", "И");
            r = r.replace("병", "Бин");
            r = r.replace("정", "Дин");
            r = r.replace("무", "У");
            r = r.replace("기", "Цзи");
            r = r.replace("경", "Гэн");
            r = r.replace("신", "Синь");
            r = r.replace("임", "Жэнь");
            r = r.replace("계", "Гуй");
            
            r = r.replace("자", "Цзы");
            r = r.replace("축", "Чоу");
            r = r.replace("인", "Инь");
            r = r.replace("묘", "Мао");
            r = r.replace("진", "Чэнь");
            r = r.replace("사", "Сы");
            r = r.replace("오", "У");
            r = r.replace("미", "Вэй");
            r = r.replace("신", "Шэнь");
            r = r.replace("유", "Ю");
            r = r.replace("술", "Сюй");
            r = r.replace("해", "Хай");
            r
        }
    }
}

pub fn translate_spirit_desc(
    locale: Locale,
    marker: eon_saju::analysis::spirit_markers::SpiritMarker,
    position: eon_saju::analysis::spirit_markers::PillarPosition,
    raw_desc: &str,
) -> String {
    if locale == Locale::Ko {
        return raw_desc.to_string();
    }
    let base_desc = translate_saju_spirit_marker_desc(locale, marker);
    let pos_context = match position {
        eon_saju::analysis::spirit_markers::PillarPosition::Year => match locale {
            Locale::Ko => "어린 시절이나 조상의 음덕, 사회적 배경에 영향을 줍니다.",
            Locale::En => "Affects early childhood, ancestral blessings, and social background.",
            Locale::Zh => "影响童年时期、祖先庇护及社会背景。",
            Locale::Ru => "Влияет на раннее детство, благословение предков и социальное происхождение.",
        },
        eon_saju::analysis::spirit_markers::PillarPosition::Month => match locale {
            Locale::Ko => "직업적 환경이나 부모 형제, 사회 활동의 중심에서 작용합니다.",
            Locale::En => "Acts on professional environment, parents, siblings, and central social activities.",
            Locale::Zh => "作用于职业环境、父母兄弟及社会活动中心。",
            Locale::Ru => "Влияет на профессиональное окружение, родителей, братьев и сестер, а также на социальную активность.",
        },
        eon_saju::analysis::spirit_markers::PillarPosition::Day => match locale {
            Locale::Ko => "나의 내면적인 기질이나 배우자와의 관계에 깊이 관여합니다.",
            Locale::En => "Deeply involved in inner temperament and relationship with spouse.",
            Locale::Zh => "深切关联内在性格及与配偶的关系。",
            Locale::Ru => "Тесно связано с внутренним темпераментом и отношениями с супругом.",
        },
        eon_saju::analysis::spirit_markers::PillarPosition::Hour => match locale {
            Locale::Ko => "자식운이나 노년의 삶, 비밀스러운 내실을 상징하는 영역에서 나타납니다.",
            Locale::En => "Manifests in areas representing children, late-life, and private inner matters.",
            Locale::Zh => "显现于代表子女、晚年生活及隐秘内心世界的领域。",
            Locale::Ru => "Проявляется в сферах, представляющих детей, старость и тайную внутреннюю жизнь.",
        },
    };
    
    let mut suffix = "";
    if raw_desc.contains("용신/희신에 해당하여 그 작용력이 더욱 강력하고 순수") {
        suffix = match locale {
            Locale::En => " (Its influence is more powerful and pure because it corresponds to Yongshin/Heeshin.)",
            Locale::Zh => " (由于属于用神/喜神，其作用力表现得更加强大与纯粹。)",
            Locale::Ru => " (Его влияние сильнее и чище, так как оно соответствует Юнсин/Хисин.)",
            _ => "",
        };
    } else if raw_desc.contains("불리한 살성이나, 용신/희신의 기운 위에 있어") {
        suffix = match locale {
            Locale::En => " (Although it is an unfavorable star, it sits on the energy of Yongshin/Heeshin, so its negativity is greatly suppressed or even sublimated into driving force.)",
            Locale::Zh => " (虽是不利凶煞，但因处于用神/喜神之上，其凶性被大大抑制，甚至升华为推动力。)",
            Locale::Ru => " (Хотя это неблагоприятная звезда, она находится под влиянием энергии Юнсин/Хисин, поэтому ее негатив сильно подавлен или даже возведен в движущую силу.)",
            _ => "",
        };
    } else if raw_desc.contains("길한 신살이지만, 기운이 비협조적이라 실제 체감") {
        suffix = match locale {
            Locale::En => " (Although it is an auspicious star, the energy is uncooperative, so the actual perceived help may be somewhat limited.)",
            Locale::Zh => " (虽是吉神，但因气运不协调，实际感受到的帮助可能会有所限制。)",
            Locale::Ru => " (Хотя это благоприятная звезда, энергия не содействует, поэтому реальная помощь может быть несколько ограниченной.)",
            _ => "",
        };
    }
    
    format!("{}{}\n\n* {}", base_desc, suffix, pos_context)
}

pub fn translate_aux_shinsal_tuple(locale: Locale, name: &str, basis: &str, result: &str) -> (String, String, String) {
    let name_t = match name {
        "태원" => match locale {
            Locale::Ko => "태원",
            Locale::En => "Taewon",
            Locale::Zh => "胎元",
            Locale::Ru => "Тэвон",
        },
        "명궁" => match locale {
            Locale::Ko => "명궁",
            Locale::En => "Myeonggung",
            Locale::Zh => "命宫",
            Locale::Ru => "Мёнгун",
        },
        "신궁" => match locale {
            Locale::Ko => "신궁",
            Locale::En => "Shingung",
            Locale::Zh => "身宫",
            Locale::Ru => "Шингун",
        },
        _ => name,
    }.to_string();

    let basis_t = match basis {
        "일지기준" => match locale {
            Locale::Ko => "일지기준",
            Locale::En => "Based on Day Branch",
            Locale::Zh => "基于日支",
            Locale::Ru => "По Ветви Дня",
        },
        "년지기준" => match locale {
            Locale::Ko => "년지기준",
            Locale::En => "Based on Year Branch",
            Locale::Zh => "基于年支",
            Locale::Ru => "По Ветви Года",
        },
        "일간기준" => match locale {
            Locale::Ko => "일간기준",
            Locale::En => "Based on Day Master",
            Locale::Zh => "基于日主",
            Locale::Ru => "По Дневному Доминанту",
        },
        _ => basis,
    }.to_string();

    let result_t = translate_aux_shinsal(locale, result);

    (name_t, basis_t, result_t)
}

pub fn translate_hd_center(locale: Locale, center: eon_human_design::HdCenter) -> &'static str {
    match center {
        eon_human_design::HdCenter::Head => t(locale, TK::HdCenterHead),
        eon_human_design::HdCenter::Ajna => t(locale, TK::HdCenterAjna),
        eon_human_design::HdCenter::Throat => t(locale, TK::HdCenterThroat),
        eon_human_design::HdCenter::SelfG => t(locale, TK::HdCenterSelf),
        eon_human_design::HdCenter::Heart => t(locale, TK::HdCenterHeart),
        eon_human_design::HdCenter::Sacral => t(locale, TK::HdCenterSacral),
        eon_human_design::HdCenter::Root => t(locale, TK::HdCenterRoot),
        eon_human_design::HdCenter::Spleen => t(locale, TK::HdCenterSpleen),
        eon_human_design::HdCenter::SolarPlexus => t(locale, TK::HdCenterSolarPlexus),
    }
}

pub fn translate_hd_type(locale: Locale, chart_type: &str) -> String {
    match chart_type {
        "Generator" => match locale {
            Locale::Ko => "제네레이터 (Generator)".to_string(),
            Locale::Zh => "生产者".to_string(),
            Locale::Ru => "Генератор".to_string(),
            _ => "Generator".to_string(),
        },
        "Manifesting Generator" => match locale {
            Locale::Ko => "매니페스팅 제네레이터 (Manifesting Generator)".to_string(),
            Locale::Zh => "显生者".to_string(),
            Locale::Ru => "Манифестирующий генератор".to_string(),
            _ => "Manifesting Generator".to_string(),
        },
        "Manifestor" => match locale {
            Locale::Ko => "매니페스토 (Manifestor)".to_string(),
            Locale::Zh => "发起者".to_string(),
            Locale::Ru => "Манифестор".to_string(),
            _ => "Manifestor".to_string(),
        },
        "Projector" => match locale {
            Locale::Ko => "프로젝터 (Projector)".to_string(),
            Locale::Zh => "投射者".to_string(),
            Locale::Ru => "Проектор".to_string(),
            _ => "Projector".to_string(),
        },
        "Reflector" => match locale {
            Locale::Ko => "리플렉터 (Reflector)".to_string(),
            Locale::Zh => "反映者".to_string(),
            Locale::Ru => "Рефлектор".to_string(),
            _ => "Reflector".to_string(),
        },
        _ => chart_type.to_string(),
    }
}

pub fn translate_hd_authority(locale: Locale, auth: &str) -> String {
    match auth {
        "Emotional" => match locale {
            Locale::Ko => "감정 권위 (Emotional)".to_string(),
            Locale::Zh => "情绪权威".to_string(),
            Locale::Ru => "Эмоциональный Авторитет".to_string(),
            _ => "Emotional".to_string(),
        },
        "Sacral" => match locale {
            Locale::Ko => "천골 권위 (Sacral)".to_string(),
            Locale::Zh => "荐骨权威".to_string(),
            Locale::Ru => "Сакральный Авторитет".to_string(),
            _ => "Sacral".to_string(),
        },
        "Splenic" => match locale {
            Locale::Ko => "직관/비장 권위 (Splenic)".to_string(),
            Locale::Zh => "脾脏权威".to_string(),
            Locale::Ru => "Селезеночный Авторитет".to_string(),
            _ => "Splenic".to_string(),
        },
        "Ego" => match locale {
            Locale::Ko => "에고 권위 (Ego)".to_string(),
            Locale::Zh => "意志力权威".to_string(),
            Locale::Ru => "Эго Авторитет".to_string(),
            _ => "Ego".to_string(),
        },
        "Self-Projected" => match locale {
            Locale::Ko => "자기 투사 권위 (Self-Projected)".to_string(),
            Locale::Zh => "自我投射权威".to_string(),
            Locale::Ru => "Самопроецируемый Авторитет".to_string(),
            _ => "Self-Projected".to_string(),
        },
        "Mental/Outer" => match locale {
            Locale::Ko => "정신/환경 권위 (Mental/Outer)".to_string(),
            Locale::Zh => "心智/无내权威".to_string(),
            Locale::Ru => "Ментальный Авторитет".to_string(),
            _ => "Mental/Outer".to_string(),
        },
        "None/Outer" => match locale {
            Locale::Ko => "없음/외적 권위 (None/Outer)".to_string(),
            Locale::Zh => "无权威".to_string(),
            Locale::Ru => "Нет Авторитета".to_string(),
            _ => "None/Outer".to_string(),
        },
        _ => auth.to_string(),
    }
}



