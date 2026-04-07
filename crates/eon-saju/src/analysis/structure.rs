//! 격국(格局, Structure/Pattern) 분석
//! 
//! 월지(月支)의 지장간이 천간에 투출한 상태를 분석하여 사주의 격을 결정합니다.

use serde::{Deserialize, Serialize};
use crate::core::stem::HeavenlyStem;
use crate::core::element::Polarity;
use crate::core::pillars::FourPillars;
use crate::core::ten_gods::TenGod;
use crate::core::config::AnalysisConfig;
use crate::analysis::Analyzable;

/// 격국의 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct StructureAnalysis {
    /// 결정된 격국
    pub structure: StructureType,
    /// 투출된 천간 (있을 경우)
    pub projected_stem: Option<HeavenlyStem>,
    /// 투출 위치 (년간, 월간, 시간)
    pub projection_path: Option<String>,
    /// 격국 요약
    pub summary: String,
    /// 격국 상세 설명
    pub description: String,
    /// 격국 성립 이유 (근거 목록)
    pub reasons: Vec<String>,
}

impl StructureAnalysis {
    pub fn from_pillars(pillars: &FourPillars) -> Self {
        Self::from_pillars_with_config(pillars, &AnalysisConfig::default())
    }

    pub fn from_pillars_with_config(pillars: &FourPillars, config: &AnalysisConfig) -> Self {
        let dm = pillars.day_master();
        let month_branch = pillars.month.branch;
        let hidden_stems = month_branch.hidden_stems();
        
        let other_stems = [
            ("년간", pillars.year.stem),
            ("월간", pillars.month.stem),
            ("시간", pillars.hour.stem),
        ];

        // 0. 특수 격국(종격/전왕격) 우선 판정
        let strength = pillars.strength_with_config(config);
        let is_polarized = strength.deuk_se.support_ratio >= config.strength.polarized_high || strength.deuk_se.support_ratio <= config.strength.polarized_low;
        
        if is_polarized {
            if strength.deuk_se.support_ratio >= config.strength.polarized_high {
                // 비겁 vs 인성 비중 확인
                let yinxing = strength.deuk_se.yinxing_count;
                let bijie = strength.deuk_se.bijie_count;
                
                let (structure, name, desc) = if bijie >= yinxing {
                    (StructureType::JongWang, "종왕격(從旺格)", "자신의 기운이 극도로 강하여 그 기세를 유지해야 하는 격국입니다.")
                } else {
                    (StructureType::JongGang, "종강격(從强格)", "자신을 돕는 인성의 기운이 극도로 강하여 그 기세를 따르는 격국입니다.")
                };

                return Self {
                    structure,
                    projected_stem: None,
                    projection_path: None,
                    summary: format!("일간이 극강하여 기세를 따르는 {}", name),
                    description: desc.to_string(),
                    reasons: vec![
                        format!("일간 세력비율: {:.1}%", strength.deuk_se.support_ratio),
                        format!("전왕 임계치: {:.1}% 이상", config.strength.polarized_high),
                    ],
                };
            } else {
                // 식상 vs 재성 vs 관성 비중 확인
                let shishang = strength.deuk_se.shishang_count;
                let cai = strength.deuk_se.caisheng_count;
                let guan = strength.deuk_se.guanxing_count;

                let (structure, name, desc) = if shishang >= cai && shishang >= guan {
                    (StructureType::JongAh, "종아격(從兒格)", "일간보다 자식(식상)의 세력을 따르는 격국입니다.")
                } else if cai >= shishang && cai >= guan {
                    (StructureType::JongJae, "종재격(從財格)", "일간보다 재물의 세력을 따르는 격국입니다.")
                } else {
                    (StructureType::JongSal, "종살격(從殺格)", "일간보다 관성의 세력을 따르는 격국입니다.")
                };

                return Self {
                    structure,
                    projected_stem: None,
                    projection_path: None,
                    summary: format!("일간이 극약하여 세력을 따르는 {}", name),
                    description: desc.to_string(),
                    reasons: vec![
                        format!("일간 세력비율: {:.1}%", strength.deuk_se.support_ratio),
                        format!("종격 임계치: {:.1}% 이하", config.strength.polarized_low),
                    ],
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
                summary: "일간이 월지에서 기운을 얻은 건록격".to_string(),
                description: "일간이 가장 왕성한 기운을 가진 시기에 태어나 주관이 뚜렷하고 자수성가할 힘이 있습니다.".to_string(),
                reasons: vec![format!("일간 {}가 월지 {}에서 12운성 건록(建祿)임", dm.hanja(), month_branch.hanja())],
            };
        }
        if stage == crate::core::twelve_stages::TwelveStage::Diwang && dm.polarity() == Polarity::Yang {
            return Self {
                structure: StructureType::YangIn,
                projected_stem: None,
                projection_path: None,
                summary: "가장 강렬한 기운을 품은 양인격".to_string(),
                description: "기운이 너무 강하여 칼을 든 것과 같으니, 이를 잘 다스리면 큰 권위를 얻습니다.".to_string(),
                reasons: vec![format!("양간 {}가 월지 {}에서 12운성 제왕(帝旺)임", dm.hanja(), month_branch.hanja())],
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
                            summary: format!("지장간의 기운이 {}에 투출된 {}", path, structure.hangul()),
                            description: "월지의 숨은 기운이 천간으로 고개를 내밀어 사주의 핵심 성격이 되었습니다.".to_string(),
                            reasons: vec![format!("월지 {}의 지장간 {}이 {} {}에 투출함", month_branch.hanja(), stem_in_branch.hanja(), path, stem_on_top.hanja())],
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
            summary: format!("월지의 본기를 격으로 삼은 {}", structure.hangul()),
            description: "천간에 드러난 기운은 없으나 태어난 계절의 기운이 가장 강력한 성격을 형성합니다.".to_string(),
            reasons: vec![format!("투출된 기운이 없어 월지 본기 {}를 기준으로 판정함", primary_stem.hanja())],
        }
    }
}

impl std::fmt::Display for StructureAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "【격국(格局) 분석】")?;
        writeln!(f, "─────────────────────────────────")?;
        writeln!(f, "▶ {} ({})", self.structure.hangul(), self.structure.hanja())?;
        writeln!(f, "  요약: {}", self.summary)?;
        writeln!(f, "  설명: {}", self.description)?;
        if !self.reasons.is_empty() {
            writeln!(f, "  근거: {}", self.reasons.join(", "))?;
        }
        Ok(())
    }
}

impl FourPillars {
    /// 격국 분석
    pub fn structure(&self) -> StructureAnalysis {
        StructureAnalysis::from_pillars(self)
    }

    /// 설정을 포함한 격국 분석
    pub fn structure_with_config(&self, config: &AnalysisConfig) -> StructureAnalysis {
        StructureAnalysis::from_pillars_with_config(self, config)
    }
}

impl Analyzable for StructureAnalysis {
    type Output = StructureAnalysis;
    fn analyze(pillars: &FourPillars, config: &AnalysisConfig) -> Self::Output {
        StructureAnalysis::from_pillars_with_config(pillars, config)
    }
}
