//! 동적 운세 분석 (Dynamic Luck Analysis)
//!
//! 원국(Four Pillars)과 대운, 세운의 상호작용을 분석합니다.
//! 특히 대운/세운이 들어오면서 완성되는 합충(合沖)을 포착합니다.

use crate::analysis::relationships::{
    RelationshipAnalysis, SeasonalCombination, TripleCombination,
};
use crate::core::branch::EarthlyBranch;
use crate::core::ganzi::GanZi;
use crate::core::pillars::FourPillars;
use crate::core::stem::HeavenlyStem;
use serde::{Deserialize, Serialize};

/// 동적 분석 결과
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicLuckAnalysis {
    /// 기준 원국
    pub natal_relations: RelationshipAnalysis,
    /// 대운 영향
    pub major_influence: Option<LuckInfluence>,
    /// 세운 영향
    pub yearly_influence: Option<LuckInfluence>,
    /// 월운 영향
    pub monthly_influence: Option<LuckInfluence>,
    /// 일운 영향
    pub daily_influence: Option<LuckInfluence>,
    /// 시운 영향
    pub hourly_influence: Option<LuckInfluence>,
    /// 원국 + 모든 운이 결합된 종합 합충
    pub combined_relations: RelationshipAnalysis,
}

/// 특정 운(대운/세운)이 원국에 미치는 영향
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LuckInfluence {
    pub ganzi: GanZi,
    pub label: String, // "대운", "세운" 등
    pub relations_with_natal: Vec<String>,
}

impl DynamicLuckAnalysis {
    /// 원국, 대운, 세운을 통합하여 분석합니다.
    pub fn analyze(
        natal: &FourPillars,
        major: Option<GanZi>,
        yearly: Option<GanZi>,
        monthly: Option<GanZi>,
        daily: Option<GanZi>,
        hourly: Option<GanZi>,
    ) -> Self {
        let natal_relations = RelationshipAnalysis::from_pillars(natal);

        // 종합 분석을 위한 확장된 구성 요소들
        let mut stems = vec![
            ("년간", natal.year.stem),
            ("월간", natal.month.stem),
            ("일간", natal.day.stem),
            ("시간", natal.hour.stem),
        ];
        let mut branches = vec![
            ("년지", natal.year.branch),
            ("월지", natal.month.branch),
            ("일지", natal.day.branch),
            ("시지", natal.hour.branch),
        ];

        if let Some(m) = major {
            stems.push(("대운천간", m.stem));
            branches.push(("대운지지", m.branch));
        }
        if let Some(y) = yearly {
            stems.push(("세운천간", y.stem));
            branches.push(("세운지지", y.branch));
        }
        if let Some(m) = monthly {
            stems.push(("월운천간", m.stem));
            branches.push(("월운지지", m.branch));
        }
        if let Some(d) = daily {
            stems.push(("일운천간", d.stem));
            branches.push(("일운지지", d.branch));
        }
        if let Some(h) = hourly {
            stems.push(("시운천간", h.stem));
            branches.push(("시운지지", h.branch));
        }

        // 확장된 구성을 기반으로 종합 합충 분석
        let combined_relations = Self::analyze_expanded(&stems, &branches);

        let major_influence = major.map(|m| Self::get_influence(m, "대운", natal));
        let yearly_influence = yearly.map(|y| Self::get_influence(y, "세운", natal));
        let monthly_influence = monthly.map(|m| Self::get_influence(m, "월운", natal));
        let daily_influence = daily.map(|d| Self::get_influence(d, "일운", natal));
        let hourly_influence = hourly.map(|h| Self::get_influence(h, "시운", natal));

        Self {
            natal_relations,
            major_influence,
            yearly_influence,
            monthly_influence,
            daily_influence,
            hourly_influence,
            combined_relations,
        }
    }

    /// 확장된 천간/지지 목록을 바탕으로 합충 분석
    fn analyze_expanded(
        stems: &[(&'static str, HeavenlyStem)],
        branches: &[(&'static str, EarthlyBranch)],
    ) -> RelationshipAnalysis {
        let mut analysis = RelationshipAnalysis {
            stem_combinations: Vec::new(),
            stem_clashes: Vec::new(),
            triple_combinations: Vec::new(),
            seasonal_combinations: Vec::new(),
            dominant_semi_combinations: Vec::new(),
            weak_semi_combinations: Vec::new(),
            six_combinations: Vec::new(),
            branch_clashes: Vec::new(),
            branch_punishments: Vec::new(),
            branch_harms: Vec::new(),
            branch_destructions: Vec::new(),
            am_combinations: Vec::new(),
            myung_am_combinations: Vec::new(),
            mapped_relationships: Vec::new(),
        };

        // 모든 쌍에 대한 분석 로직 호출
        use crate::analysis::relationships::{
            Amhap, BranchClash, BranchDestruction, BranchHarm, BranchPunishment, MyungAmHap,
            SemiCombination, SixCombination, StemClash, StemCombination,
        };

        // 천간 분석
        for i in 0..stems.len() {
            for j in (i + 1)..stems.len() {
                let (p1, s1) = stems[i];
                let (p2, s2) = stems[j];
                if let Some(c) = StemCombination::check(s1, s2) {
                    analysis
                        .stem_combinations
                        .push((c, p1.to_string(), p2.to_string()));
                }
                if let Some(c) = StemClash::check(s1, s2) {
                    analysis
                        .stem_clashes
                        .push((c, p1.to_string(), p2.to_string()));
                }
            }
        }

        // 지지 분석
        for i in 0..branches.len() {
            for j in (i + 1)..branches.len() {
                let (p1, b1) = branches[i];
                let (p2, b2) = branches[j];
                if let Some(s) = SemiCombination::check(b1, b2) {
                    if s.is_dominant() {
                        analysis.dominant_semi_combinations.push((
                            s,
                            p1.to_string(),
                            p2.to_string(),
                        ));
                    } else {
                        analysis
                            .weak_semi_combinations
                            .push((s, p1.to_string(), p2.to_string()));
                    }
                }
                if let Some(s) = SixCombination::check(b1, b2) {
                    analysis
                        .six_combinations
                        .push((s, p1.to_string(), p2.to_string()));
                }
                if let Some(c) = BranchClash::check(b1, b2) {
                    analysis
                        .branch_clashes
                        .push((c, p1.to_string(), p2.to_string()));
                }
                if let Some(h) = BranchHarm::check(b1, b2) {
                    analysis
                        .branch_harms
                        .push((h, p1.to_string(), p2.to_string()));
                }
                if let Some(d) = BranchDestruction::check(b1, b2) {
                    analysis
                        .branch_destructions
                        .push((d, p1.to_string(), p2.to_string()));
                }
                if let Some(p) = BranchPunishment::check_self(b1, b2) {
                    analysis
                        .branch_punishments
                        .push((p, p1.to_string(), p2.to_string()));
                }

                // 암합
                let ams = RelationshipAnalysis::check_am_combinations(b1, b2);
                for am in ams {
                    analysis.am_combinations.push((
                        Amhap {
                            branches: (b1, b2),
                            combination: am,
                        },
                        p1.to_string(),
                        p2.to_string(),
                    ));
                }
            }
        }

        // 명암합
        for (sp, s) in stems {
            for (bp, b) in branches {
                let mas = RelationshipAnalysis::check_myung_am_combinations(*s, *b);
                for ma in mas {
                    analysis.myung_am_combinations.push((
                        MyungAmHap {
                            stem: *s,
                            branch: *b,
                            combination: ma,
                        },
                        sp.to_string(),
                        bp.to_string(),
                    ));
                }
            }
        }

        // 삼합/방합 완성 체크
        let all_b: Vec<_> = branches.iter().map(|(_, b)| *b).collect();
        analysis.triple_combinations = TripleCombination::check(&all_b);
        analysis.seasonal_combinations = SeasonalCombination::check(&all_b);

        let is_luck =
            |p: &str| p == "대운" || p == "세운" || p == "월운" || p == "일운" || p == "시운";

        let mut mapped = Vec::new();
        let make_detail = |rel_type: &str, name: &str, p1: &str, p2: &str| {
            crate::analysis::relationships::RelationshipDetail {
                relation_type: rel_type.to_string(),
                name: name.to_string(),
                positions: vec![p1.to_string(), p2.to_string()],
                level: crate::analysis::supplementary_pillars::InterpretationLevel::Neutral,
                summary: format!("{}과 {}의 {}", p1, p2, name),
                description: "".to_string(),
                reasons: vec![],
                transformed_element: None,
            }
        };

        for (_, p1, p2) in &analysis.stem_combinations {
            if is_luck(p1) && is_luck(p2) {
                mapped.push(make_detail("합", "천간합", p1, p2));
            }
        }
        for (_, p1, p2) in &analysis.stem_clashes {
            if is_luck(p1) && is_luck(p2) {
                mapped.push(make_detail("충", "천간충", p1, p2));
            }
        }
        for (_, p1, p2) in &analysis.dominant_semi_combinations {
            if is_luck(p1) && is_luck(p2) {
                mapped.push(make_detail("합", "반합(주도)", p1, p2));
            }
        }
        for (_, p1, p2) in &analysis.weak_semi_combinations {
            if is_luck(p1) && is_luck(p2) {
                mapped.push(make_detail("합", "반합(보조)", p1, p2));
            }
        }
        for (_, p1, p2) in &analysis.six_combinations {
            if is_luck(p1) && is_luck(p2) {
                mapped.push(make_detail("합", "육합", p1, p2));
            }
        }
        for (_, p1, p2) in &analysis.branch_clashes {
            if is_luck(p1) && is_luck(p2) {
                mapped.push(make_detail("충", "지지충", p1, p2));
            }
        }
        for (_, p1, p2) in &analysis.branch_punishments {
            if is_luck(p1) && is_luck(p2) {
                mapped.push(make_detail("형", "형", p1, p2));
            }
        }
        for (_, p1, p2) in &analysis.branch_harms {
            if is_luck(p1) && is_luck(p2) {
                mapped.push(make_detail("해", "해", p1, p2));
            }
        }
        for (_, p1, p2) in &analysis.branch_destructions {
            if is_luck(p1) && is_luck(p2) {
                mapped.push(make_detail("파", "파", p1, p2));
            }
        }

        analysis.mapped_relationships = mapped;

        analysis
    }

    pub fn get_influence(luck: GanZi, label: &str, natal: &FourPillars) -> LuckInfluence {
        let mut relations = Vec::new();
        // 원국과의 관계 정리
        let n_stems = [
            natal.year.stem,
            natal.month.stem,
            natal.day.stem,
            natal.hour.stem,
        ];
        let n_branches = [
            natal.year.branch,
            natal.month.branch,
            natal.day.branch,
            natal.hour.branch,
        ];

        use crate::analysis::relationships::*;

        // 1. 천간 관계
        for s in &n_stems {
            if let Some(_c) = StemCombination::check(luck.stem, *s) {
                relations.push(format!("천간합: {} - {}", luck.stem.hanja(), s.hanja()));
            }
            if let Some(_c) = StemClash::check(luck.stem, *s) {
                relations.push(format!("천간충: {} - {}", luck.stem.hanja(), s.hanja()));
            }
        }

        // 2. 지지 관계
        for b in &n_branches {
            // 육합
            if let Some(_c) = SixCombination::check(luck.branch, *b) {
                relations.push(format!("육합: {} - {}", luck.branch.hanja(), b.hanja()));
            }
            // 반합
            if let Some(semi) = SemiCombination::check(luck.branch, *b) {
                relations.push(format!(
                    "반합: {} ({}-{})",
                    semi.hangul(),
                    luck.branch.hanja(),
                    b.hanja()
                ));
            }
            // 충
            if let Some(_c) = BranchClash::check(luck.branch, *b) {
                relations.push(format!("지지충: {} - {}", luck.branch.hanja(), b.hanja()));
            }
            // 형
            if let Some(p) = BranchPunishment::check_self(luck.branch, *b) {
                relations.push(format!("지지형: {}", p.hangul()));
            }
            // 해
            if let Some(_h) = BranchHarm::check(luck.branch, *b) {
                relations.push(format!("지지해: {} - {}", luck.branch.hanja(), b.hanja()));
            }
            // 파
            if let Some(_d) = BranchDestruction::check(luck.branch, *b) {
                relations.push(format!("지지파: {} - {}", luck.branch.hanja(), b.hanja()));
            }
        }

        LuckInfluence {
            ganzi: luck,
            label: label.to_string(),
            relations_with_natal: relations,
        }
    }
}

impl std::fmt::Display for DynamicLuckAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "【동적 종합 분석 (원국 + 대운 + 세운)】")?;
        writeln!(f, "─────────────────────────────────")?;

        if let Some(infl) = &self.major_influence {
            writeln!(
                f,
                "▶ 대운 영향 ({}): {}",
                infl.ganzi,
                infl.relations_with_natal.join(", ")
            )?;
        }
        if let Some(infl) = &self.yearly_influence {
            writeln!(
                f,
                "▶ 세운 영향 ({}): {}",
                infl.ganzi,
                infl.relations_with_natal.join(", ")
            )?;
        }

        writeln!(f, "\n[종합 합충 결과]")?;
        write!(f, "{}", self.combined_relations)?;

        Ok(())
    }
}
