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

