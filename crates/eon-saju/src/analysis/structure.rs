//! 격국(格局, Structure/Pattern) 분석
//! 
//! 월지(月支)의 지장간이 천간에 투출한 상태를 분석하여 사주의 격을 결정합니다.

use serde::{Deserialize, Serialize};
use crate::core::stem::HeavenlyStem;
use crate::core::element::Polarity;
use crate::core::pillars::FourPillars;
use crate::core::ten_gods::TenGod;
use crate::core::config::thresholds::*;

/// 격국의 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StructureType {
    /// 식신격 (食神格)
    ShiShen,
    /// 상관격 (傷官格)
    ShangGuan,
    /// 편재격 (偏財格)
    PianCai,
    /// 정재격 (正財格)
    ZhengCai,
    /// 편관격 (偏官格)
    PianGuan,
    /// 정관격 (正官格)
    ZhengGuan,
    /// 편인격 (偏印格)
    PianYin,
    /// 정인격 (正印格)
    ZhengYin,
    /// 건록격 (建祿格) - 일간이 월지에서 건록
    JianLu,
    /// 양인격 (陽刃格) - 일간이 월지에서 제왕(양간)
    YangIn,
    /// 비견/겁재 (정격 외)
    Special,
    
    // --- 종격 (從格) ---
    /// 종아격 (從兒格) - 식상으로 종함
    JongAh,
    /// 종재격 (從財格) - 재성으로 종함
    JongJae,
    /// 종살격 (從殺格) - 관성으로 종함
    JongSal,
    /// 종강격 (從强格) - 인성으로 종함
    JongGang,
    /// 종왕격 (從旺格) - 비겁으로 종함
    JongWang,
    
    /// 종격 (기타/일반)
    Follower,
    /// 전왕격 (專旺格) - 자신의 기운이 극도로 강함
    SpecialTransformation,
}

impl StructureType {
    pub const fn hangul(&self) -> &'static str {
        match self {
            Self::ShiShen => "식신격",
            Self::ShangGuan => "상관격",
            Self::PianCai => "편재격",
            Self::ZhengCai => "정재격",
            Self::PianGuan => "편관격",
            Self::ZhengGuan => "정관격",
            Self::PianYin => "편인격",
            Self::ZhengYin => "정인격",
            Self::JianLu => "건록격",
            Self::YangIn => "양인격",
            Self::Special => "비겁격",
            Self::JongAh => "종아격",
            Self::JongJae => "종재격",
            Self::JongSal => "종살격",
            Self::JongGang => "종강격",
            Self::JongWang => "종왕격",
            Self::Follower => "종격(從格)",
            Self::SpecialTransformation => "전왕격(專旺格)",
        }
    }

    pub const fn hanja(&self) -> &'static str {
        match self {
            Self::ShiShen => "食神格",
            Self::ShangGuan => "傷官格",
            Self::PianCai => "偏財格",
            Self::ZhengCai => "正財格",
            Self::PianGuan => "偏官格",
            Self::ZhengGuan => "正官格",
            Self::PianYin => "偏印格",
            Self::ZhengYin => "正印格",
            Self::JianLu => "建祿格",
            Self::YangIn => "陽刃格",
            Self::Special => "特殊格",
            Self::JongAh => "從兒格",
            Self::JongJae => "從財格",
            Self::JongSal => "從殺格",
            Self::JongGang => "從强格",
            Self::JongWang => "從旺格",
            Self::Follower => "從格",
            Self::SpecialTransformation => "專旺格",
        }
    }

    /// 십성으로부터 격국 변환 (비겁 제외)
    pub fn from_ten_god(god: TenGod) -> Option<Self> {
        match god {
            TenGod::Shishen => Some(Self::ShiShen),
            TenGod::Shangguan => Some(Self::ShangGuan),
            TenGod::Piancai => Some(Self::PianCai),
            TenGod::Zhengcai => Some(Self::ZhengCai),
            TenGod::Pianguan => Some(Self::PianGuan),
            TenGod::Zhengguan => Some(Self::ZhengGuan),
            TenGod::Pianyin => Some(Self::PianYin),
            TenGod::Zhengyin => Some(Self::ZhengYin),
            _ => None,
        }
    }
}

/// 격국 분석 결과
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureAnalysis {
    /// 결정된 격국
    pub structure: StructureType,
    /// 투출된 천간 (있을 경우)
    pub projected_stem: Option<HeavenlyStem>,
    /// 투출 위치 (년간, 월간, 시간)
    pub projection_path: Option<String>,
    /// 격국 성립 이유
    pub reason: String,
}

impl StructureAnalysis {
    pub fn from_pillars(pillars: &FourPillars) -> Self {
        let dm = pillars.day_master();
        let month_branch = pillars.month.branch;
        let hidden_stems = month_branch.hidden_stems();
        
        let other_stems = [
            ("년간", pillars.year.stem),
            ("월간", pillars.month.stem),
            ("시간", pillars.hour.stem),
        ];

        // 0. 특수 격국(종격/전왕격) 우선 판정
        let strength = pillars.strength();
        let is_polarized = strength.deuk_se.support_ratio >= POLARIZED_RATIO_HIGH || strength.deuk_se.support_ratio <= POLARIZED_RATIO_LOW;
        
        if is_polarized {
            if strength.deuk_se.support_ratio >= POLARIZED_RATIO_HIGH {
                // 비겁 vs 인성 비중 확인
                let yinxing = strength.deuk_se.yinxing_count;
                let bijie = strength.deuk_se.bijie_count;
                
                let (structure, name) = if bijie >= yinxing {
                    (StructureType::JongWang, "종왕격(從旺格)")
                } else {
                    (StructureType::JongGang, "종강격(從强格)")
                };

                return Self {
                    structure,
                    projected_stem: None,
                    projection_path: None,
                    reason: format!("일간의 세력({:.1}%)이 극단적으로 강하여 {}에 해당함", strength.deuk_se.support_ratio, name),
                };
            } else {
                // 식상 vs 재성 vs 관성 비중 확인
                let shishang = strength.deuk_se.shishang_count;
                let cai = strength.deuk_se.caisheng_count;
                let guan = strength.deuk_se.guanxing_count;

                let (structure, name) = if shishang >= cai && shishang >= guan {
                    (StructureType::JongAh, "종아격(從兒格)")
                } else if cai >= shishang && cai >= guan {
                    (StructureType::JongJae, "종재격(從財格)")
                } else {
                    (StructureType::JongSal, "종살격(從殺格)")
                };

                return Self {
                    structure,
                    projected_stem: None,
                    projection_path: None,
                    reason: format!("일간의 세력({:.1}%)이 극단적으로 약하여 월지 세력을 따르는 {}에 해당함", strength.deuk_se.support_ratio, name),
                };
            }
        }

        // 1. 건록격/양인격 우선 판정
        let stage = crate::core::twelve_stages::calculate_twelve_stage(dm, month_branch);
        if stage == crate::core::twelve_stages::TwelveStage::Jianlu {
            return Self {
                structure: StructureType::JianLu,
                projected_stem: None,
                projection_path: None,
                reason: format!("일간 {}이 월지 {}에서 건록에 해당함", dm.hanja(), month_branch.hanja()),
            };
        }
        if stage == crate::core::twelve_stages::TwelveStage::Diwang && dm.polarity() == Polarity::Yang {
            return Self {
                structure: StructureType::YangIn,
                projected_stem: None,
                projection_path: None,
                reason: format!("일간 {}이 월지 {}에서 제왕(양인)에 해당함", dm.hanja(), month_branch.hanja()),
            };
        }

        // 2. 투출(透出) 분석 - 정기(본기)부터 역순으로 확인하여 가장 강한 것 선택
        // 보통은 본기 투출이 가장 강력함
        for stem_in_branch in hidden_stems.iter().rev() {
            for (path, stem_on_top) in &other_stems {
                if stem_in_branch == stem_on_top {
                    let god = TenGod::from_stems(dm, *stem_on_top);
                    if let Some(structure) = StructureType::from_ten_god(god) {
                        return Self {
                            structure,
                            projected_stem: Some(*stem_on_top),
                            projection_path: Some(path.to_string()),
                            reason: format!("월지 지지 {}의 지장간 {}이 {}에 투출함", month_branch.hanja(), stem_in_branch.hanja(), path),
                        };
                    }
                }
            }
        }

        // 3. 투출된 것이 없으면 월지 본기(정기)로 판정 (월령 격국)
        let primary_stem = month_branch.primary_stem();
        let god = TenGod::from_stems(dm, primary_stem);
        let structure = StructureType::from_ten_god(god).unwrap_or(StructureType::Special);
        
        Self {
            structure,
            projected_stem: None,
            projection_path: None,
            reason: format!("투출된 기운이 없어 월지 본기 {}를 격으로 삼음", primary_stem.hanja()),
        }
    }
}

impl std::fmt::Display for StructureAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "【격국(格局) 분석】")?;
        writeln!(f, "─────────────────────────────────")?;
        writeln!(f, "▶ {} ({})", self.structure.hangul(), self.structure.hanja())?;
        writeln!(f, "  성립 이유: {}", self.reason)?;
        Ok(())
    }
}

impl FourPillars {
    /// 격국 분석
    pub fn structure(&self) -> StructureAnalysis {
        StructureAnalysis::from_pillars(self)
    }
}
