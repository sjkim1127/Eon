//! TraceTag: Structured luck analysis tags
//!
//! Defined as an enum to eliminate heap allocation and enable zero-copy pattern matching.
//! Formatted into display strings only when generating reports or traces.

use crate::analysis::dynamic_luck::GyeokStatus;
use crate::analysis::shinsal::TwelveShinsal;
use crate::analysis::structure::StructureType;
use crate::core::branch::EarthlyBranch;
use crate::core::element::Element;
use crate::core::stem::HeavenlyStem;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt;

/// Analysis tag - structured enum minimizing heap allocations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TraceTag {
    // === Luck Period Tags ===
    /// Core luck (hits primary yongshin)
    CoreLuck { period: LuckPeriod },
    /// Good luck (heeshin)
    GoodLuck { period: LuckPeriod },
    /// Bad luck (gishin)
    BadLuck { period: LuckPeriod },

    // === Combination & Clash Tags ===
    /// Branch clash
    BranchClash { clash_type: Cow<'static, str> },
    /// Stem clash
    StemClash { clash_type: Cow<'static, str> },
    /// Six combination
    SixCombination { combo_type: Cow<'static, str> },
    /// Triple alliance completed
    TripleCombination {
        element: Element,
        is_beneficial: bool,
    },
    /// Seasonal alliance completed
    SeasonalCombination {
        element: Element,
        is_beneficial: bool,
    },
    /// Transformation (elemental shift)
    Transformation { from: Element, to: Element },

    // === Branch Relationship Tags ===
    /// Punishment
    Punishment { punishment_type: Cow<'static, str> },
    /// Harm
    Harm { harm_type: Cow<'static, str> },
    /// Destruction
    Destruction { destruction_type: Cow<'static, str> },

    // === Void (Gongmang) Tags ===
    /// Void luck (unrescued void)
    VoidLuck { period: LuckPeriod },
    /// Escaped void via branch clash
    EscapedVoidClash { period: LuckPeriod },
    /// Escaped void via six combination
    EscapedVoidSixCombo { period: LuckPeriod },
    /// Escaped void via triple alliance
    EscapedVoidTriple { period: LuckPeriod },
    /// Escaped void via seasonal alliance
    EscapedVoidSeasonal { period: LuckPeriod },

    // === Storage Unsealing / Trapping / Structure Tags ===
    /// Storage Unsealing (GaeGo)
    GaeGo {
        branch: EarthlyBranch,
        unsealed_stem: HeavenlyStem,
    },
    /// Storage Trapping (IpMyo)
    IpMyo {
        element: Element,
        tomb_branch: EarthlyBranch,
    },
    /// Dynamic structure transformation
    DynamicGyeok {
        active_structure: StructureType,
        status: GyeokStatus,
    },

    // === Shinsal Tags ===
    /// 12 Shinsal
    TwelveShinsal {
        name: ShinsalName,
        period: LuckPeriod,
    },
    /// Auspicious spirits (Cheoneul, Munchang, etc.)
    AuspiciousSpirit {
        name: AuspiciousSpiritName,
        period: LuckPeriod,
    },
    /// Inauspicious spirits (Wonjin, Gwimun, etc.)
    InauspiciousSpirit {
        name: InauspiciousSpiritName,
        position: PillarPosition,
        period: LuckPeriod,
    },

    // === 12 Life Stages Tags ===
    /// 12 Life Stages (Changsheng, Jianlu, Diwang, etc.)
    LifeStage {
        stage: LifeStageName,
        period: LuckPeriod,
    },

    // === Ten Gods Pattern Tags ===
    /// Hurting Officer meets Direct Officer
    HurtingOfficerMeetsOfficer { period: LuckPeriod },
    /// Eating God produces Wealth
    EatingGodProducesWealth { period: LuckPeriod },

    // === Strength/Weakness Tags ===
    /// Strength type
    StrengthType { strength: StrengthTypeName },
    /// DeukRyeong (Month Command)
    DeukRyeong,
    /// DeukJi (Branch Root)
    DeukJi,
    /// DeukSi (Hour Support)
    DeukSi,
    /// DeukSe (Power Alignment)
    DeukSe,

    // === Interrupt Tags ===
    /// Hardware interrupt (Baekho, Goegang, etc.)
    Interrupt {
        irq_type: InterruptType,
        marker: Cow<'static, str>,
    },

    // === Other ===
    /// Custom tag (legacy compatibility)
    Custom(Cow<'static, str>),
}

/// Luck period type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LuckPeriod {
    Major,   // Major luck
    Yearly,  // Yearly luck
    Monthly, // Monthly luck
    Daily,   // Daily luck
    Hourly,  // Hourly luck
}

impl LuckPeriod {
    pub const fn hangul(&self) -> &'static str {
        match self {
            LuckPeriod::Major => "Major",
            LuckPeriod::Yearly => "Yearly",
            LuckPeriod::Monthly => "Monthly",
            LuckPeriod::Daily => "Daily",
            LuckPeriod::Hourly => "Hourly",
        }
    }

    /// Parse LuckPeriod from string
    pub fn from_label(label: &str) -> Self {
        match label {
            "Major" => LuckPeriod::Major,
            "Yearly" => LuckPeriod::Yearly,
            "Monthly" => LuckPeriod::Monthly,
            "Daily" => LuckPeriod::Daily,
            "Hourly" => LuckPeriod::Hourly,
            _ => LuckPeriod::Yearly, // fallback
        }
    }
}

/// 12 Shinsal names
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShinsalName {
    Jangseongsal,
    Banansal,
    Geopsal,
    Jaesal,
    Cheonsal,
    Yeokmasal,
    Jisal,
    Hwagaesal,
    Yuhaengsal,
    Yeonsal,
    Wolsal,
    Mangshinsal,
    Baekhosal,
    Goegangsal,
    Yanginsal,
    Gwirok,
    Gongmang,
    Dolsal,
    Wonsuksal,
    Dosasal,
}

impl ShinsalName {
    pub const fn hangul(&self) -> &'static str {
        match self {
            ShinsalName::Jangseongsal => "Jangseongsal",
            ShinsalName::Banansal => "Banansal",
            ShinsalName::Geopsal => "Geopsal",
            ShinsalName::Jaesal => "Jaesal",
            ShinsalName::Cheonsal => "Cheonsal",
            ShinsalName::Yeokmasal => "Yeokmasal",
            ShinsalName::Jisal => "Jisal",
            ShinsalName::Hwagaesal => "Hwagaesal",
            ShinsalName::Yuhaengsal => "Yukhaesal",
            ShinsalName::Yeonsal => "Yeonsal",
            ShinsalName::Wolsal => "Wolsal",
            ShinsalName::Mangshinsal => "Mangshinsal",
            ShinsalName::Baekhosal => "Baekhosal",
            ShinsalName::Goegangsal => "Goegangsal",
            ShinsalName::Yanginsal => "Yanginsal",
            ShinsalName::Gwirok => "Gwirok",
            ShinsalName::Gongmang => "Gongmang",
            ShinsalName::Dolsal => "Dolsal",
            ShinsalName::Wonsuksal => "Wonsuksal",
            ShinsalName::Dosasal => "Dosasal",
        }
    }
}

impl From<TwelveShinsal> for ShinsalName {
    fn from(s: TwelveShinsal) -> Self {
        match s {
            TwelveShinsal::Jisal => ShinsalName::Jisal,
            TwelveShinsal::Yeonsal => ShinsalName::Yeonsal,
            TwelveShinsal::Wolsal => ShinsalName::Wolsal,
            TwelveShinsal::Mangshinsal => ShinsalName::Mangshinsal,
            TwelveShinsal::Jangseongsal => ShinsalName::Jangseongsal,
            TwelveShinsal::Banansal => ShinsalName::Banansal,
            TwelveShinsal::Yeokmasal => ShinsalName::Yeokmasal,
            TwelveShinsal::Yukhaesal => ShinsalName::Yuhaengsal,
            TwelveShinsal::Hwagaesal => ShinsalName::Hwagaesal,
            TwelveShinsal::Geopsal => ShinsalName::Geopsal,
            TwelveShinsal::Jaesal => ShinsalName::Jaesal,
            TwelveShinsal::Cheonsal => ShinsalName::Cheonsal,
            TwelveShinsal::Baekhosal => ShinsalName::Baekhosal,
            TwelveShinsal::Goegangsal => ShinsalName::Goegangsal,
            TwelveShinsal::Yanginsal => ShinsalName::Yanginsal,
            TwelveShinsal::Gwirok => ShinsalName::Gwirok,
            TwelveShinsal::Gongmang => ShinsalName::Gongmang,
        }
    }
}

/// Auspicious spirit names
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuspiciousSpiritName {
    Cheoneul,
    Munchang,
    Taegueg,
    Woldeok,
    Cheongdeok,
}

impl AuspiciousSpiritName {
    pub const fn hangul(&self) -> &'static str {
        match self {
            AuspiciousSpiritName::Cheoneul => "Cheoneul",
            AuspiciousSpiritName::Munchang => "Munchang",
            AuspiciousSpiritName::Taegueg => "Taeguk",
            AuspiciousSpiritName::Woldeok => "Woldeok",
            AuspiciousSpiritName::Cheongdeok => "Cheondeok",
        }
    }
}

/// Inauspicious spirit names
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InauspiciousSpiritName {
    Wonjin,  // Wonjin
    Gwimun,  // Gwimun
    Baekho,  // Baekho
    Goesang, // Goesang
}

impl InauspiciousSpiritName {
    pub const fn hangul(&self) -> &'static str {
        match self {
            InauspiciousSpiritName::Wonjin => "Wonjin",
            InauspiciousSpiritName::Gwimun => "Gwimun",
            InauspiciousSpiritName::Baekho => "Baekho",
            InauspiciousSpiritName::Goesang => "Goesang",
        }
    }
}

/// Pillar position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PillarPosition {
    Year,
    Month,
    Day,
    Hour,
}

impl PillarPosition {
    pub const fn hangul(&self) -> &'static str {
        match self {
            PillarPosition::Year => "Year",
            PillarPosition::Month => "Month",
            PillarPosition::Day => "Day",
            PillarPosition::Hour => "Hour",
        }
    }

    pub fn from_label(label: &str) -> Self {
        match label {
            "Year" | "year_branch" => PillarPosition::Year,
            "Month" | "month_branch" => PillarPosition::Month,
            "Day" | "day_branch" => PillarPosition::Day,
            "Hour" | "hour_branch" => PillarPosition::Hour,
            _ => PillarPosition::Day,
        }
    }
}

/// 12 Life Stages names
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LifeStageName {
    Changsheng,
    Muyu,
    Guandai,
    Jianlu,
    Diwang,
    Shuai,
    Bing,
    Si,
    Mu,
    Jue,
    Tai,
    Yang,
}

impl LifeStageName {
    pub const fn hangul(&self) -> &'static str {
        match self {
            LifeStageName::Changsheng => "Changsheng",
            LifeStageName::Muyu => "Muyu",
            LifeStageName::Guandai => "Guandai",
            LifeStageName::Jianlu => "Jianlu",
            LifeStageName::Diwang => "Diwang",
            LifeStageName::Shuai => "Shuai",
            LifeStageName::Bing => "Bing",
            LifeStageName::Si => "Si",
            LifeStageName::Mu => "Mu",
            LifeStageName::Jue => "Jue",
            LifeStageName::Tai => "Tai",
            LifeStageName::Yang => "Yang",
        }
    }
}

impl From<crate::core::twelve_stages::TwelveStage> for LifeStageName {
    fn from(s: crate::core::twelve_stages::TwelveStage) -> Self {
        use crate::core::twelve_stages::TwelveStage::*;
        match s {
            Changsheng => LifeStageName::Changsheng,
            Muyu => LifeStageName::Muyu,
            Guandai => LifeStageName::Guandai,
            Jianlu => LifeStageName::Jianlu,
            Diwang => LifeStageName::Diwang,
            Shuai => LifeStageName::Shuai,
            Bing => LifeStageName::Bing,
            Si => LifeStageName::Si,
            Mu => LifeStageName::Mu,
            Jue => LifeStageName::Jue,
            Tai => LifeStageName::Tai,
            Yang => LifeStageName::Yang,
        }
    }
}

/// Strength type names
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StrengthTypeName {
    Strong,
    Weak,
    Balanced,
}

impl StrengthTypeName {
    pub const fn hangul(&self) -> &'static str {
        match self {
            StrengthTypeName::Strong => "Strong",
            StrengthTypeName::Weak => "Weak",
            StrengthTypeName::Balanced => "Balanced",
        }
    }
}

/// Interrupt type codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterruptType {
    CriticalException, // 0x01: Baekho, etc.
    ResourceOverflow,  // 0x02: Goegang, etc.
    SystemStall,       // 0x03: Gosin/Gwasuk, etc.
    ServiceInterrupt,  // 0x04: Mangshin/Geop, etc.
}

impl InterruptType {
    pub const fn code(&self) -> &'static str {
        match self {
            InterruptType::CriticalException => "IRQ_0x01",
            InterruptType::ResourceOverflow => "IRQ_0x02",
            InterruptType::SystemStall => "IRQ_0x03",
            InterruptType::ServiceInterrupt => "IRQ_0x04",
        }
    }
}

impl fmt::Display for TraceTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Luck period tags
            TraceTag::CoreLuck { period } => write!(f, "{}CoreLuck", period.hangul()),
            TraceTag::GoodLuck { period } => write!(f, "{}GoodLuck", period.hangul()),
            TraceTag::BadLuck { period } => write!(f, "{}BadLuck", period.hangul()),

            // Combination & clash tags
            TraceTag::BranchClash { clash_type } => write!(f, "{}", clash_type),
            TraceTag::StemClash { clash_type } => write!(f, "StemClash:{}", clash_type),
            TraceTag::SixCombination { combo_type } => write!(f, "SixCombo:{}", combo_type),
            TraceTag::TripleCombination {
                element,
                is_beneficial,
            } => {
                let suffix = if *is_beneficial {
                    "Auspicious"
                } else {
                    "Inauspicious"
                };
                write!(f, "TripleComboCompleted({}-{})", element.hangul(), suffix)
            }
            TraceTag::SeasonalCombination {
                element,
                is_beneficial,
            } => {
                let suffix = if *is_beneficial {
                    "Auspicious"
                } else {
                    "Inauspicious"
                };
                write!(f, "SeasonalComboCompleted({}-{})", element.hangul(), suffix)
            }
            TraceTag::Transformation { from, to } => {
                write!(f, "Transform:{}->{}", from.hangul(), to.hangul())
            }

            // Branch relationships
            TraceTag::Punishment { punishment_type } => write!(f, "{}", punishment_type),
            TraceTag::Harm { harm_type } => write!(f, "{}", harm_type),
            TraceTag::Destruction { destruction_type } => write!(f, "{}", destruction_type),

            // Void
            TraceTag::VoidLuck { period } => write!(f, "VoidLuck:{}", period.hangul()),
            TraceTag::EscapedVoidClash { period } => {
                write!(f, "EscapedVoid:Clash({})", period.hangul())
            }
            TraceTag::EscapedVoidSixCombo { period } => {
                write!(f, "EscapedVoid:SixCombo({})", period.hangul())
            }
            TraceTag::EscapedVoidTriple { period } => {
                write!(f, "EscapedVoid:TripleCombo({})", period.hangul())
            }
            TraceTag::EscapedVoidSeasonal { period } => {
                write!(f, "EscapedVoid:SeasonalCombo({})", period.hangul())
            }

            // Unsealing / Trapping / Structure
            TraceTag::GaeGo {
                branch,
                unsealed_stem,
            } => {
                write!(f, "GaeGo:{}({})", branch.hangul(), unsealed_stem.hangul())
            }
            TraceTag::IpMyo {
                element,
                tomb_branch,
            } => {
                write!(f, "IpMyo:{}({})", element.hangul(), tomb_branch.hangul())
            }
            TraceTag::DynamicGyeok {
                active_structure,
                status,
            } => {
                write!(
                    f,
                    "Structure:{}({})",
                    active_structure.hangul(),
                    status.hangul()
                )
            }

            // Shinsal
            TraceTag::TwelveShinsal { name, period } => {
                write!(f, "Shinsal:{}({})", name.hangul(), period.hangul())
            }
            TraceTag::AuspiciousSpirit { name, period } => {
                write!(f, "AuspiciousSpirit:{}({})", name.hangul(), period.hangul())
            }
            TraceTag::InauspiciousSpirit {
                name,
                position,
                period,
            } => {
                write!(
                    f,
                    "InauspiciousSpirit:{}({}-{})",
                    name.hangul(),
                    position.hangul(),
                    period.hangul()
                )
            }

            // 12 Life Stages
            TraceTag::LifeStage { stage, period } => {
                write!(f, "LifeStage:{}({})", stage.hangul(), period.hangul())
            }

            // Ten Gods patterns
            TraceTag::HurtingOfficerMeetsOfficer { period } => {
                write!(
                    f,
                    "Pattern:HurtingOfficerControlsOfficer({})",
                    period.hangul()
                )
            }
            TraceTag::EatingGodProducesWealth { period } => {
                write!(f, "Pattern:EatingGodProducesWealth({})", period.hangul())
            }

            // Strength/Weakness
            TraceTag::StrengthType { strength } => write!(f, "Strength:{}", strength.hangul()),
            TraceTag::DeukRyeong => write!(f, "DeukRyeong"),
            TraceTag::DeukJi => write!(f, "DeukJi"),
            TraceTag::DeukSi => write!(f, "DeukSi"),
            TraceTag::DeukSe => write!(f, "DeukSe"),

            // Interrupt
            TraceTag::Interrupt { irq_type, marker } => {
                write!(f, "{}:{}", irq_type.code(), marker)
            }

            // Custom
            TraceTag::Custom(s) => write!(f, "{}", s),
        }
    }
}

impl TraceTag {
    /// Check whether tag display contains pattern string (legacy compatibility)
    pub fn contains_pattern(&self, pattern: &str) -> bool {
        self.to_string().contains(pattern)
    }
}

/// Convert TraceTag collection to Vec<String> (legacy compatibility)
pub fn tags_to_strings(tags: &[TraceTag]) -> Vec<String> {
    tags.iter().map(|t| t.to_string()).collect()
}

/// Convert Vec<String> to TraceTag collection (legacy compatibility)
pub fn strings_to_tags(strings: &[String]) -> Vec<TraceTag> {
    strings
        .iter()
        .map(|s| TraceTag::Custom(Cow::Owned(s.clone())))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_display() {
        let tag = TraceTag::CoreLuck {
            period: LuckPeriod::Major,
        };
        assert_eq!(tag.to_string(), "MajorCoreLuck");

        let tag = TraceTag::TripleCombination {
            element: Element::Wood,
            is_beneficial: true,
        };
        assert_eq!(tag.to_string(), "TripleComboCompleted(Wood-Auspicious)");
    }

    #[test]
    fn test_tags_to_strings() {
        let tags = vec![
            TraceTag::DeukRyeong,
            TraceTag::DeukJi,
            TraceTag::StrengthType {
                strength: StrengthTypeName::Strong,
            },
        ];
        let strings = tags_to_strings(&tags);
        assert_eq!(strings, vec!["DeukRyeong", "DeukJi", "Strength:Strong"]);
    }
}
