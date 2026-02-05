//! TraceTag: 구조화된 운세 분석 태그
//!
//! String 힙 할당을 최소화하기 위해 태그를 Enum으로 정의합니다.
//! 최종 리포트 생성 시에만 문자열로 변환됩니다.

use serde::{Deserialize, Serialize};
use crate::core::element::Element;
use std::fmt;

/// 분석 태그 - 문자열 대신 구조화된 Enum 사용
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TraceTag {
    // === 운성 태그 (대운/세운) ===
    /// 핵심운 (용신 적중)
    CoreLuck { period: LuckPeriod },
    /// 길운
    GoodLuck { period: LuckPeriod },
    /// 기신운
    BadLuck { period: LuckPeriod },

    // === 합충 태그 ===
    /// 지지 충
    BranchClash { clash_type: String },
    /// 천간 충
    StemClash { clash_type: String },
    /// 육합
    SixCombination { combo_type: String },
    /// 삼합 완성
    TripleCombination { element: Element, is_beneficial: bool },
    /// 방합 완성
    SeasonalCombination { element: Element, is_beneficial: bool },
    /// 합화 (오행 변화)
    Transformation { from: Element, to: Element },

    // === 지지 관계 태그 ===
    /// 형 (刑)
    Punishment { punishment_type: String },
    /// 해 (害)
    Harm { harm_type: String },
    /// 파 (破)
    Destruction { destruction_type: String },

    // === 공망 태그 ===
    /// 운성 공망 (진공)
    VoidLuck { period: LuckPeriod },
    /// 탈공 (충에 의한)
    EscapedVoidClash { period: LuckPeriod },
    /// 탈공 (육합에 의한)
    EscapedVoidSixCombo { period: LuckPeriod },
    /// 탈공 (삼합에 의한)
    EscapedVoidTriple { period: LuckPeriod },
    /// 탈공 (방합에 의한)
    EscapedVoidSeasonal { period: LuckPeriod },

    // === 신살 태그 ===
    /// 12신살
    TwelveShinsal { name: ShinsalName, period: LuckPeriod },
    /// 길신 (천을귀인, 문창귀인 등)
    AuspiciousSpirit { name: AuspiciousSpiritName, period: LuckPeriod },
    /// 흉살 (원진, 귀문 등)
    InauspiciousSpirit { name: InauspiciousSpiritName, position: PillarPosition, period: LuckPeriod },

    // === 12운성 태그 ===
    /// 12운성 (장생, 건록, 제왕 등)
    LifeStage { stage: LifeStageName, period: LuckPeriod },

    // === 십신 패턴 태그 ===
    /// 상관견관 (상관 운 + 원국 정관)
    HurtingOfficerMeetsOfficer { period: LuckPeriod },
    /// 식신생재 (식신 운 + 원국 재성)
    EatingGodProducesWealth { period: LuckPeriod },

    // === 신강/신약 태그 ===
    /// 신강신약 타입
    StrengthType { strength: StrengthTypeName },
    /// 득령
    DeukRyeong,
    /// 득지
    DeukJi,
    /// 득시
    DeukSi,
    /// 득세
    DeukSe,

    // === 인터럽트 태그 ===
    /// 하드웨어 인터럽트 (백호살, 괴강살 등)
    Interrupt { irq_type: InterruptType, marker: String },

    // === 기타 ===
    /// 커스텀 태그 (레거시 호환용)
    Custom(String),
}

/// 운의 종류 (대운, 세운, 월운 등)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LuckPeriod {
    Major,   // 대운
    Yearly,  // 세운
    Monthly, // 월운
    Daily,   // 일운
    Hourly,  // 시운
}

impl LuckPeriod {
    pub const fn hangul(&self) -> &'static str {
        match self {
            LuckPeriod::Major => "대운",
            LuckPeriod::Yearly => "세운",
            LuckPeriod::Monthly => "월운",
            LuckPeriod::Daily => "일운",
            LuckPeriod::Hourly => "시운",
        }
    }

    /// 문자열로부터 LuckPeriod 변환
    pub fn from_label(label: &str) -> Self {
        match label {
            "대운" => LuckPeriod::Major,
            "세운" => LuckPeriod::Yearly,
            "월운" => LuckPeriod::Monthly,
            "일운" => LuckPeriod::Daily,
            "시운" => LuckPeriod::Hourly,
            _ => LuckPeriod::Yearly, // fallback
        }
    }
}

/// 12신살 이름
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShinsalName {
    Jangseongsal, // 장성살
    Banansal,     // 반안살
    Geopsal,      // 겁살
    Jaesal,       // 재살
    Cheonsal,     // 천살
    Yeokmasal,    // 역마살
    Jisal,        // 지살
    Hwagaesal,    // 화개살
    Yuhaengsal,   // 유해살
    Dolsal,       // 돌살
    Wonsuksal,    // 원숙살
    Dosasal,      // 도사살
}

impl ShinsalName {
    pub const fn hangul(&self) -> &'static str {
        match self {
            ShinsalName::Jangseongsal => "장성살",
            ShinsalName::Banansal => "반안살",
            ShinsalName::Geopsal => "겁살",
            ShinsalName::Jaesal => "재살",
            ShinsalName::Cheonsal => "천살",
            ShinsalName::Yeokmasal => "역마살",
            ShinsalName::Jisal => "지살",
            ShinsalName::Hwagaesal => "화개살",
            ShinsalName::Yuhaengsal => "유해살",
            ShinsalName::Dolsal => "돌살",
            ShinsalName::Wonsuksal => "원숙살",
            ShinsalName::Dosasal => "도사살",
        }
    }
}

/// 길신 이름
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuspiciousSpiritName {
    Cheoneul,   // 천을귀인
    Munchang,   // 문창귀인
    Taegueg,    // 태극귀인
    Woldeok,    // 월덕귀인
    Cheongdeok, // 천덕귀인
}

impl AuspiciousSpiritName {
    pub const fn hangul(&self) -> &'static str {
        match self {
            AuspiciousSpiritName::Cheoneul => "천을귀인",
            AuspiciousSpiritName::Munchang => "문창귀인",
            AuspiciousSpiritName::Taegueg => "태극귀인",
            AuspiciousSpiritName::Woldeok => "월덕귀인",
            AuspiciousSpiritName::Cheongdeok => "천덕귀인",
        }
    }
}

/// 흉살 이름
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InauspiciousSpiritName {
    Wonjin,  // 원진
    Gwimun,  // 귀문
    Baekho,  // 백호
    Goesang, // 괴상
}

impl InauspiciousSpiritName {
    pub const fn hangul(&self) -> &'static str {
        match self {
            InauspiciousSpiritName::Wonjin => "원진",
            InauspiciousSpiritName::Gwimun => "귀문",
            InauspiciousSpiritName::Baekho => "백호",
            InauspiciousSpiritName::Goesang => "괴상",
        }
    }
}

/// 주(柱) 위치
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
            PillarPosition::Year => "년지",
            PillarPosition::Month => "월지",
            PillarPosition::Day => "일지",
            PillarPosition::Hour => "시지",
        }
    }
}

/// 12운성 이름
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LifeStageName {
    Changsheng, // 장생
    Muyu,       // 목욕
    Guandai,    // 관대
    Jianlu,     // 건록
    Diwang,     // 제왕
    Shuai,      // 쇠
    Bing,       // 병
    Si,         // 사
    Mu,         // 묘
    Jue,        // 절
    Tai,        // 태
    Yang,       // 양
}

impl LifeStageName {
    pub const fn hangul(&self) -> &'static str {
        match self {
            LifeStageName::Changsheng => "장생",
            LifeStageName::Muyu => "목욕",
            LifeStageName::Guandai => "관대",
            LifeStageName::Jianlu => "건록",
            LifeStageName::Diwang => "제왕",
            LifeStageName::Shuai => "쇠",
            LifeStageName::Bing => "병",
            LifeStageName::Si => "사",
            LifeStageName::Mu => "묘",
            LifeStageName::Jue => "절",
            LifeStageName::Tai => "태",
            LifeStageName::Yang => "양",
        }
    }
}

/// 신강/신약 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StrengthTypeName {
    Strong,   // 신강
    Weak,     // 신약
    Balanced, // 중화
}

impl StrengthTypeName {
    pub const fn hangul(&self) -> &'static str {
        match self {
            StrengthTypeName::Strong => "신강",
            StrengthTypeName::Weak => "신약",
            StrengthTypeName::Balanced => "중화",
        }
    }
}

/// 인터럽트 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterruptType {
    CriticalException, // 0x01: 백호살 등
    ResourceOverflow,  // 0x02: 괴강살 등
    SystemStall,       // 0x03: 고신/과숙 등
    ServiceInterrupt,  // 0x04: 망신/겁살 등
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
            // 운성 태그
            TraceTag::CoreLuck { period } => write!(f, "{}핵심운", period.hangul()),
            TraceTag::GoodLuck { period } => write!(f, "{}길운", period.hangul()),
            TraceTag::BadLuck { period } => write!(f, "{}기신운", period.hangul()),
            
            // 합충 태그
            TraceTag::BranchClash { clash_type } => write!(f, "{}", clash_type),
            TraceTag::StemClash { clash_type } => write!(f, "천간충:{}", clash_type),
            TraceTag::SixCombination { combo_type } => write!(f, "육합:{}", combo_type),
            TraceTag::TripleCombination { element, is_beneficial } => {
                let suffix = if *is_beneficial { "吉" } else { "凶" };
                write!(f, "삼합완성({}-{})", element.hangul(), suffix)
            }
            TraceTag::SeasonalCombination { element, is_beneficial } => {
                let suffix = if *is_beneficial { "吉" } else { "凶" };
                write!(f, "방합완성({}-{})", element.hangul(), suffix)
            }
            TraceTag::Transformation { from, to } => {
                write!(f, "합화:{}→{}", from.hangul(), to.hangul())
            }
            
            // 지지 관계
            TraceTag::Punishment { punishment_type } => write!(f, "{}", punishment_type),
            TraceTag::Harm { harm_type } => write!(f, "{}", harm_type),
            TraceTag::Destruction { destruction_type } => write!(f, "{}", destruction_type),
            
            // 공망
            TraceTag::VoidLuck { period } => write!(f, "운성공망:{}", period.hangul()),
            TraceTag::EscapedVoidClash { period } => write!(f, "탈공:충({})", period.hangul()),
            TraceTag::EscapedVoidSixCombo { period } => write!(f, "탈공:육합({})", period.hangul()),
            TraceTag::EscapedVoidTriple { period } => write!(f, "탈공:삼합({})", period.hangul()),
            TraceTag::EscapedVoidSeasonal { period } => write!(f, "탈공:방합({})", period.hangul()),
            
            // 신살
            TraceTag::TwelveShinsal { name, period } => {
                write!(f, "신살:{}({})", name.hangul(), period.hangul())
            }
            TraceTag::AuspiciousSpirit { name, period } => {
                write!(f, "길신:{}({})", name.hangul(), period.hangul())
            }
            TraceTag::InauspiciousSpirit { name, position, period } => {
                write!(f, "흉살:{}({}-{})", name.hangul(), position.hangul(), period.hangul())
            }
            
            // 12운성
            TraceTag::LifeStage { stage, period } => {
                write!(f, "운성:{}({})", stage.hangul(), period.hangul())
            }
            
            // 십신 패턴
            TraceTag::HurtingOfficerMeetsOfficer { period } => {
                write!(f, "패턴:상관견관({})", period.hangul())
            }
            TraceTag::EatingGodProducesWealth { period } => {
                write!(f, "패턴:식신생재({})", period.hangul())
            }
            
            // 신강/신약
            TraceTag::StrengthType { strength } => write!(f, "신강약:{}", strength.hangul()),
            TraceTag::DeukRyeong => write!(f, "득령"),
            TraceTag::DeukJi => write!(f, "득지"),
            TraceTag::DeukSi => write!(f, "득시"),
            TraceTag::DeukSe => write!(f, "득세"),
            
            // 인터럽트
            TraceTag::Interrupt { irq_type, marker } => {
                write!(f, "{}:{}", irq_type.code(), marker)
            }
            
            // 커스텀
            TraceTag::Custom(s) => write!(f, "{}", s),
        }
    }
}

impl TraceTag {
    /// 문자열 패턴이 이 태그에 포함되는지 확인 (레거시 contains() 호환용)
    pub fn contains_pattern(&self, pattern: &str) -> bool {
        self.to_string().contains(pattern)
    }
}

/// TraceTag 컬렉션을 Vec<String>으로 변환 (레거시 호환용)
pub fn tags_to_strings(tags: &[TraceTag]) -> Vec<String> {
    tags.iter().map(|t| t.to_string()).collect()
}

/// Vec<String>에서 TraceTag 컬렉션으로 변환 (레거시 호환용)
pub fn strings_to_tags(strings: &[String]) -> Vec<TraceTag> {
    strings.iter().map(|s| TraceTag::Custom(s.clone())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_display() {
        let tag = TraceTag::CoreLuck { period: LuckPeriod::Major };
        assert_eq!(tag.to_string(), "대운핵심운");

        let tag = TraceTag::TripleCombination { 
            element: Element::Wood, 
            is_beneficial: true 
        };
        assert_eq!(tag.to_string(), "삼합완성(목-吉)");
    }

    #[test]
    fn test_tags_to_strings() {
        let tags = vec![
            TraceTag::DeukRyeong,
            TraceTag::DeukJi,
            TraceTag::StrengthType { strength: StrengthTypeName::Strong },
        ];
        let strings = tags_to_strings(&tags);
        assert_eq!(strings, vec!["득령", "득지", "신강약:신강"]);
    }
}
