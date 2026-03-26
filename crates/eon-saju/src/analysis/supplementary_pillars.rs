use serde::{Deserialize, Serialize};
use crate::core::ganzi::GanZi;
use crate::core::pillars::FourPillars;
use crate::core::stem::HeavenlyStem;
use crate::core::branch::EarthlyBranch;

/// 태원·명궁·신궁 등 보조 기둥 분석 결과
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplementaryPillars {
    /// 태원 (胎元): 잉태 시점의 기운
    pub taewon: GanZi,
    /// 명궁 (命宮): 정신적 지향점과 운의 바탕
    pub myeonggung: GanZi,
    /// 신궁 (身宮): 후천적 신체 환경과 실제적 삶의 태도
    pub shingung: GanZi,
    /// 메타 정보
    pub meta: SupplementaryPillarsMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplementaryPillarsMeta {
    /// 계산 공식 버전
    pub formula_version: String,
    /// 시간에 민감한지 여부 (명궁/신궁은 시간 필수)
    pub hour_sensitive: bool,
}

impl SupplementaryPillars {
    /// 기준 FourPillars로부터 보조 기둥 계산
    pub fn calculate(pillars: &FourPillars) -> Self {
        let taewon = Self::calculate_taewon(&pillars.month);
        let myeonggung = Self::calculate_myeonggung(pillars);
        let shingung = Self::calculate_shingung(pillars);

        Self {
            taewon,
            myeonggung,
            shingung,
            meta: SupplementaryPillarsMeta {
                formula_version: "Traditional v1.0".to_string(),
                hour_sensitive: true,
            },
        }
    }

    /// 태원(胎元) 계산
    /// 공식: 월주 천간 + 1, 월주 지지 + 3
    fn calculate_taewon(month_ganzi: &GanZi) -> GanZi {
        let stem_idx = (month_ganzi.stem.index() as i32 + 1) % 10;
        let branch_idx = (month_ganzi.branch.index() as i32 + 3) % 12;

        GanZi::new(
            HeavenlyStem::from_index(stem_idx),
            EarthlyBranch::from_index(branch_idx),
        )
    }

    /// 명궁(命宮) 계산
    /// 공식: 인월(寅, 2) 기준, 명궁 지지 = (14 - (월지_idx + 시지_idx)).rem_euclid(12)
    fn calculate_myeonggung(pillars: &FourPillars) -> GanZi {
        let m_idx = pillars.month.branch.index() as i32; // 0=子, 1=丑, 2=寅...
        let h_idx = pillars.hour.branch.index() as i32;

        // 전통 명궁 산법 (인월 2부터 시작하는 순번 기준)
        let m_val = if m_idx >= 2 { m_idx } else { m_idx + 12 };
        let h_val = if h_idx >= 2 { h_idx } else { h_idx + 12 };

        let mut mg_branch_val = 14 - (m_val + h_val) % 12;
        if mg_branch_val <= 0 { mg_branch_val += 12; }
        if mg_branch_val > 12 { mg_branch_val -= 12; }

        let mg_branch_idx = (mg_branch_val % 12) as i32;
        let branch = EarthlyBranch::from_index(mg_branch_idx);

        let year_stem = pillars.year.stem;
        let start_stem_idx = (year_stem.index() as i32 % 5) * 2 + 2;
        let mg_stem_idx = (start_stem_idx + (mg_branch_idx - 2).rem_euclid(12)) % 10;

        GanZi::new(
            HeavenlyStem::from_index(mg_stem_idx),
            branch,
        )
    }

    /// 신궁(身宮) 계산
    fn calculate_shingung(pillars: &FourPillars) -> GanZi {
        let m_idx = pillars.month.branch.index() as i32;
        let h_idx = pillars.hour.branch.index() as i32;

        let m_val = if m_idx >= 2 { m_idx } else { m_idx + 12 };
        let h_val = if h_idx >= 2 { h_idx } else { h_idx + 12 };
        
        let mut sg_branch_val = (m_val + h_val - 2) % 12;
        if sg_branch_val <= 0 { sg_branch_val += 12; }
        if sg_branch_val > 12 { sg_branch_val -= 12; }
        
        let sg_branch_idx = (sg_branch_val % 12) as i32;
        let branch = EarthlyBranch::from_index(sg_branch_idx);

        let year_stem = pillars.year.stem;
        let start_stem_idx = (year_stem.index() as i32 % 5) * 2 + 2;
        let sg_stem_idx = (start_stem_idx + (sg_branch_idx - 2).rem_euclid(12)) % 10;

        GanZi::new(
            HeavenlyStem::from_index(sg_stem_idx),
            branch,
        )
    }
}
