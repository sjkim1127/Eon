// crates/eon-ui/src/i18n/mod.rs
// Zero-dependency, compile-time static i18n for Eon UI.
// Locale persisted in global Signal<Locale> + localStorage.

pub mod en;
pub mod ko;
pub mod ru;
pub mod zh;

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
    NavAi,
    NavZwds,

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

    // ── AI Tab ───────────────────────────────────────────────────────────
    AiStartAudit,
    AiChatPlaceholderReady,
    AiChatPlaceholderWait,
    AiDisclaimer,
    AiApiKeyHint,
    AiWelcome,

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


