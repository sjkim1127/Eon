//! 격국(格局, Structure/Pattern) 분석
//!
//! 월지(月支)의 지장간이 천간에 투출한 상태를 분석하여 사주의 격을 결정합니다.

use crate::analysis::Analyzable;
use crate::core::config::AnalysisConfig;
use crate::core::element::Polarity;
use crate::core::pillars::FourPillars;
use crate::core::stem::HeavenlyStem;
use crate::core::ten_gods::TenGod;
use serde::{Deserialize, Serialize};

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
    /// 진종아격 (眞從兒格) - 식상으로 순수 종함
    JongAh,
    /// 진종재격 (眞從財格) - 재성으로 순수 종함
    JongJae,
    /// 진종살격 (眞從殺格) - 관성으로 순수 종함
    JongSal,
    /// 진종강격 (眞從强格) - 인성으로 순수 종함
    JongGang,
    /// 진종왕격 (眞從旺格) - 비겁으로 순수 종함
    JongWang,

    /// 가종아격 (假從兒格) - 미약한 인성/비겁 근이 있는 가종
    GaJongAh,
    /// 가종재격 (假從財格) - 미약한 인성/비겁 근이 있는 가종
    GaJongJae,
    /// 가종살격 (假從殺格) - 미약한 인성/비겁 근이 있는 가종
    GaJongSal,
    /// 가종강격 (假從强格) - 미약한 식상/재성/관성 근이 있는 가종
    GaJongGang,
    /// 가종왕격 (假從旺格) - 미약한 식상/재성/관성 근이 있는 가종
    GaJongWang,

    /// 화기격 (化氣格) - 일간이 합화하여 성질이 변함
    HwaGi,

    /// 종격 (기타/일반)
    Follower,
    /// 전왕격 (專旺格) - 자신의 기운이 극도로 강함
    SpecialTransformation,

    // --- 전왕격 외격 5종 ---
    /// 곡직격 (曲直格 - 목 전왕)
    GokJik,
    /// 염상격 (炎上格 - 화 전왕)
    YeomSang,
    /// 가색격 (稼穡格 - 토 전왕)
    GaSaek,
    /// 종혁격 (從革格 - 금 전왕)
    JongHyeok,
    /// 윤하격 (潤下格 - 수 전왕)
    YoonHa,

    /// 관살혼잡격 (官殺混雜格)
    GwanSalHonJab,
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
            Self::JongAh => "진종아격",
            Self::JongJae => "진종재격",
            Self::JongSal => "진종살격",
            Self::JongGang => "진종강격",
            Self::JongWang => "진종왕격",
            Self::GaJongAh => "가종아격",
            Self::GaJongJae => "가종재격",
            Self::GaJongSal => "가종살격",
            Self::GaJongGang => "가종강격",
            Self::GaJongWang => "가종왕격",
            Self::HwaGi => "화기격",
            Self::Follower => "종격(從格)",
            Self::SpecialTransformation => "전왕격(專旺格)",
            Self::GokJik => "곡직격(木전왕)",
            Self::YeomSang => "염상격(火전왕)",
            Self::GaSaek => "가색격(土전왕)",
            Self::JongHyeok => "종혁격(金전왕)",
            Self::YoonHa => "윤하격(水전왕)",
            Self::GwanSalHonJab => "관살혼잡격",
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
            Self::JongAh => "眞從兒格",
            Self::JongJae => "眞從財格",
            Self::JongSal => "眞從殺格",
            Self::JongGang => "眞從强格",
            Self::JongWang => "眞從旺格",
            Self::GaJongAh => "假從兒格",
            Self::GaJongJae => "假從財格",
            Self::GaJongSal => "假從殺格",
            Self::GaJongGang => "假從强格",
            Self::GaJongWang => "假從旺格",
            Self::HwaGi => "化氣格",
            Self::Follower => "從格",
            Self::SpecialTransformation => "專旺格",
            Self::GokJik => "曲直格",
            Self::YeomSang => "炎上格",
            Self::GaSaek => "稼穡格",
            Self::JongHyeok => "從革格",
            Self::YoonHa => "潤下格",
            Self::GwanSalHonJab => "官殺混雜格",
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

        // -1. 화기격(HwaGi) 우선 판정 (파성 破星 및 쟁합/투합 검증)
        let mut hwagi_result = None;
        let adjacent_stems = [("월간", pillars.month.stem), ("시간", pillars.hour.stem)];
        for (path, stem_on_top) in &adjacent_stems {
            if let Some(combo) =
                crate::analysis::relationships::StemCombination::check(dm, *stem_on_top)
            {
                let transformed_elem = combo.transformed_element();
                if month_branch.element() == transformed_elem {
                    // 1) 합화 오행을 극하는 파성(破星) 존재 여부 검사
                    let breaker_elem = transformed_elem.controlled_by();
                    let all_stems = [
                        pillars.year.stem,
                        pillars.month.stem,
                        pillars.day.stem,
                        pillars.hour.stem,
                    ];
                    let has_break_star = all_stems.iter().any(|s| s.element() == breaker_elem);

                    // 2) 쟁합/투합 검사
                    let same_dm_count = all_stems.iter().filter(|&&s| s == dm).count();
                    let same_partner_count =
                        all_stems.iter().filter(|&&s| s == *stem_on_top).count();
                    let is_competing = same_dm_count > 1 || same_partner_count > 1;

                    if !has_break_star && !is_competing {
                        hwagi_result = Some((path, combo, transformed_elem, *stem_on_top));
                        break;
                    }
                }
            }
        }

        if let Some((path, combo, transformed_elem, matched_stem)) = hwagi_result {
            return Self {
                structure: StructureType::HwaGi,
                projected_stem: Some(matched_stem),
                projection_path: Some(path.to_string()),
                summary: format!("일간이 {}과 {}을 이루어 {}로 변하는 화기격", path, combo.hangul(), transformed_elem.hangul()),
                description: "일간이 인접한 천간과 합을 이루고, 태어난 월(계절)이 그 합화된 기운을 강하게 뒷받침하여 완전히 새로운 기운으로 변화(化)한 특별한 사주입니다.".to_string(),
                reasons: vec![
                    format!("일간 {}와 {} {}이 {}", dm.hanja(), path, matched_stem.hanja(), combo.hanja()),
                    format!("합화 오행({})이 월지 {}과 일치", transformed_elem.hanja(), month_branch.hanja()),
                    "파성(破星) 없음 및 쟁합/투합 없음 검증 완료".to_string(),
                ],
            };
        }

        // 0. 특수 격국(종격/전왕격) 우선 판정
        let strength = pillars.strength_with_config(config);
        let is_polarized = strength.deuk_se.support_ratio >= config.strength.polarized_high
            || strength.deuk_se.support_ratio <= config.strength.polarized_low;

        if is_polarized {
            let all_branches = [
                pillars.year.branch,
                pillars.month.branch,
                pillars.day.branch,
                pillars.hour.branch,
            ];

            // 일간 및 인성 통근(뿌리) 여부 검사 (진종 眞從 vs 가종 假從 판정용)
            let dm_el = dm.element();
            let yin_el = dm_el.generated_by();
            let has_dm_root = all_branches.iter().any(|b| {
                b.hidden_stems()
                    .iter()
                    .any(|hs| hs.element() == dm_el || hs.element() == yin_el)
            });

            if strength.deuk_se.support_ratio >= config.strength.polarized_high {
                // 삼합/방합 전왕 외격 5종 (곡직, 염상, 가색, 종혁, 윤하) 및 전왕격 판정
                use crate::core::branch::EarthlyBranch::*;
                use crate::core::element::Element;

                let (structure, name, desc) = match dm_el {
                    Element::Wood if matches!(month_branch, Yin | Mao | Chen | Hai) => (
                        StructureType::GokJik,
                        "곡직격(曲直格 - 木전왕)",
                        "목(木)의 청아하고 인자한 기운이 전왕을 이루어 성품이 어질고 학문과 예술에 탁월합니다.",
                    ),
                    Element::Fire if matches!(month_branch, Si | Wu | Wei | Yin) => (
                        StructureType::YeomSang,
                        "염상격(炎上格 - 火전왕)",
                        "화(火)의 뜨겁고 예의 바른 기운이 전왕을 이루어 명예와 예의가 높고 기상이 화려합니다.",
                    ),
                    Element::Earth if matches!(month_branch, Chen | Xu | Chou | Wei) => (
                        StructureType::GaSaek,
                        "가색격(稼穡格 - 土전왕)",
                        "토(土)의 두텁고 신의 있는 기운이 전왕을 이루어 포용력이 크고 재물이 넉넉합니다.",
                    ),
                    Element::Metal if matches!(month_branch, Shen | You | Xu | Si) => (
                        StructureType::JongHyeok,
                        "종혁격(從革格 - 金전왕)",
                        "금(金)의 과단하고 숙살하는 기운이 전왕을 이루어 결단력이 강하고 권위를 쥡니다.",
                    ),
                    Element::Water if matches!(month_branch, Hai | Zi | Chou | Shen) => (
                        StructureType::YoonHa,
                        "윤하격(潤下格 - 水전왕)",
                        "수(水)의 지혜롭고 흐르는 기운이 전왕을 이루어 지혜가 깊고 만물을 유통시킵니다.",
                    ),
                    _ => {
                        let yinxing = strength.deuk_se.yinxing_count;
                        let bijie = strength.deuk_se.bijie_count;
                        if bijie >= yinxing {
                            (
                                StructureType::JongWang,
                                "종왕격(從旺格)",
                                "자신의 기운이 극도로 강하여 그 기세를 유지해야 하는 전왕 격국입니다.",
                            )
                        } else {
                            (
                                StructureType::JongGang,
                                "종강격(從强格)",
                                "자신을 돕는 인성의 기운이 극도로 강하여 그 기세를 따르는 전왕 격국입니다.",
                            )
                        }
                    }
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
                use crate::analysis::power::{AnalysisOptions, IntegratedAnalysis};
                let options = AnalysisOptions {
                    apply_transform: false,
                    apply_correction: true,
                };
                let integrated = IntegratedAnalysis::calculate(pillars, options, config);

                let mut shishang_power = 0.0f32;
                let mut cai_power = 0.0f32;
                let mut guan_power = 0.0f32;

                for (tg, pct, _) in integrated.ten_god_scores {
                    match tg {
                        TenGod::Shishen | TenGod::Shangguan => shishang_power += pct,
                        TenGod::Zhengcai | TenGod::Piancai => cai_power += pct,
                        TenGod::Zhengguan | TenGod::Pianguan => guan_power += pct,
                        _ => {}
                    }
                }

                let (structure, name, desc) = if shishang_power >= cai_power
                    && shishang_power >= guan_power
                {
                    if has_dm_root {
                        (
                            StructureType::GaJongAh,
                            "가종아격(假從兒格)",
                            "일간이나 인성의 미약한 뿌리가 잔재하나 식상 세력을 따르는 가종격입니다.",
                        )
                    } else {
                        (
                            StructureType::JongAh,
                            "진종아격(眞從兒格)",
                            "일간/인성의 뿌리가 일절 없어 식상의 세력을 순수하게 따르는 진종격입니다.",
                        )
                    }
                } else if cai_power >= shishang_power && cai_power >= guan_power {
                    if has_dm_root {
                        (
                            StructureType::GaJongJae,
                            "가종재격(假從財格)",
                            "일간이나 인성의 미약한 뿌리가 잔재하나 재물 세력을 따르는 가종격입니다.",
                        )
                    } else {
                        (
                            StructureType::JongJae,
                            "진종재격(眞從財格)",
                            "일간/인성의 뿌리가 일절 없어 재물의 세력을 순수하게 따르는 진종격입니다.",
                        )
                    }
                } else {
                    if has_dm_root {
                        (
                            StructureType::GaJongSal,
                            "가종살격(假從殺格)",
                            "일간이나 인성의 미약한 뿌리가 잔재하나 관살 세력을 따르는 가종격입니다.",
                        )
                    } else {
                        (
                            StructureType::JongSal,
                            "진종살격(眞從殺格)",
                            "일간/인성의 뿌리가 일절 없어 관살의 세력을 순수하게 따르는 진종격입니다.",
                        )
                    }
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
                        format!(
                            "일간/인성 지지 통근: {}",
                            if has_dm_root {
                                "존재 (가종격 판정)"
                            } else {
                                "없음 (진종격 판정)"
                            }
                        ),
                    ],
                };
            }
        }

        // 0.5. 관살혼잡격(官殺混雜格) 판정
        // 년/월/시간에 정관과 편관이 동시에 투출하고 제화(합/극)되지 않은 경우
        let exposed_stems = [pillars.year.stem, pillars.month.stem, pillars.hour.stem];
        let exposed_tgs: Vec<TenGod> = exposed_stems
            .iter()
            .map(|&s| TenGod::from_stems(dm, s))
            .collect();
        let has_zhengguan = exposed_tgs.contains(&TenGod::Zhengguan);
        let has_pianguan = exposed_tgs.contains(&TenGod::Pianguan);
        if has_zhengguan && has_pianguan {
            return Self {
                structure: StructureType::GwanSalHonJab,
                projected_stem: None,
                projection_path: None,
                summary: "정관과 편관이 함께 투출하여 혼란스러운 관살혼잡격".to_string(),
                description: "원국 천간에 정관과 편관(칠살)이 동시에 드러나 있어 직업적 갈등이나 명예 선택의 난관이 따를 수 있으므로 정리가 필요합니다.".to_string(),
                reasons: vec![
                    "원국 천간에 정관(正官)과 편관(偏官/七殺)이 동시 투출됨".to_string(),
                    "관살혼잡(官殺混雜) 상태 형성".to_string(),
                ],
            };
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
        if stage == crate::core::twelve_stages::TwelveStage::Diwang
            && dm.polarity() == Polarity::Yang
        {
            return Self {
                structure: StructureType::YangIn,
                projected_stem: None,
                projection_path: None,
                summary: "가장 강렬한 기운을 품은 양인격".to_string(),
                description:
                    "기운이 너무 강하여 칼을 든 것과 같으니, 이를 잘 다스리면 큰 권위를 얻습니다."
                        .to_string(),
                reasons: vec![format!(
                    "양간 {}가 월지 {}에서 12운성 제왕(帝旺)임",
                    dm.hanja(),
                    month_branch.hanja()
                )],
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
            description:
                "천간에 드러난 기운은 없으나 태어난 계절의 기운이 가장 강력한 성격을 형성합니다."
                    .to_string(),
            reasons: vec![format!(
                "투출된 기운이 없어 월지 본기 {}를 기준으로 판정함",
                primary_stem.hanja()
            )],
        }
    }
}

impl std::fmt::Display for StructureAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "【격국(格局) 분석】")?;
        writeln!(f, "─────────────────────────────────")?;
        writeln!(
            f,
            "▶ {} ({})",
            self.structure.hangul(),
            self.structure.hanja()
        )?;
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
