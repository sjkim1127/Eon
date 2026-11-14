//! Dynamic Luck Analysis
//!
//! Analyzes dynamic interactions between Natal Four Pillars and luck periods (Major, Yearly, Monthly, Daily, Hourly).
//! Specifically captures combinations, clashes, unsealing, and structure transformations triggered by luck cycles.

use crate::analysis::relationships::{
    RelationshipAnalysis, SeasonalCombination, TripleCombination,
};
use crate::core::branch::EarthlyBranch;
use crate::core::ganzi::GanZi;
use crate::core::pillars::FourPillars;
use crate::core::stem::HeavenlyStem;
use serde::{Deserialize, Serialize};

use crate::core::element::Element;

/// Storage Branch Unsealing (GaeGo) event
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GaeGoEvent {
    pub branch: EarthlyBranch,
    pub position: String,
    pub trigger: String,
    pub unsealed_stems: Vec<HeavenlyStem>,
}

/// Storage Trapping (IpMyo) event
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IpMyoEvent {
    pub element: Element,
    pub tomb_branch: EarthlyBranch,
    pub trigger: String,
}

/// Dynamic structure transformation status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GyeokStatus {
    /// Stable (maintaining natal structure)
    Stable,
    /// Transformed (shifted into new structure)
    Transformed,
    /// Broken (structure broken or damaged)
    Broken,
    /// Fulfilled (structure completed via yongshin manifestation or alliance)
    Fulfilled,
}

impl GyeokStatus {
    pub const fn hangul(&self) -> &'static str {
        match self {
            Self::Stable => "Stable",
            Self::Transformed => "Transformed",
            Self::Broken => "Broken",
            Self::Fulfilled => "Fulfilled",
        }
    }
}

/// Dynamic structure state tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicStructureState {
    pub base_structure: crate::analysis::structure::StructureType,
    pub active_structure: crate::analysis::structure::StructureType,
    pub status: GyeokStatus,
    pub description: String,
}

/// Dynamic luck analysis outcome
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicLuckAnalysis {
    /// Base natal chart relations
    pub natal_relations: RelationshipAnalysis,
    /// Major luck influence
    pub major_influence: Option<LuckInfluence>,
    /// Yearly luck influence
    pub yearly_influence: Option<LuckInfluence>,
    /// Monthly luck influence
    pub monthly_influence: Option<LuckInfluence>,
    /// Daily luck influence
    pub daily_influence: Option<LuckInfluence>,
    /// Hourly luck influence
    pub hourly_influence: Option<LuckInfluence>,
    /// Comprehensive combined interactions across natal and luck cycles
    pub combined_relations: RelationshipAnalysis,
    /// Storage unsealing events
    pub gaego_events: Vec<GaeGoEvent>,
    /// Storage trapping events
    pub ipmyo_events: Vec<IpMyoEvent>,
    /// Dynamic structure transformation status
    pub structure_state: DynamicStructureState,
}

/// Domain impact structure representing life domain influences
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DomainImpact {
    pub target_position: String,
    pub ten_god: String,
    pub interaction: String,
    pub description: String,
}

/// Influence of a specific luck period on the natal chart
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LuckInfluence {
    pub ganzi: GanZi,
    pub label: String, // "Major", "Yearly", etc.
    pub relations_with_natal: Vec<String>,
    pub domain_impacts: Vec<DomainImpact>,
}

impl DynamicLuckAnalysis {
    /// Perform comprehensive dynamic analysis combining natal and luck cycles
    pub fn analyze(
        natal: &FourPillars,
        major: Option<GanZi>,
        yearly: Option<GanZi>,
        monthly: Option<GanZi>,
        daily: Option<GanZi>,
        hourly: Option<GanZi>,
    ) -> Self {
        let natal_relations = RelationshipAnalysis::from_pillars(natal);

        // Extended components for comprehensive dynamic analysis
        let mut stems = vec![
            ("year_stem", natal.year.stem),
            ("month_stem", natal.month.stem),
            ("day_stem", natal.day.stem),
        ];
        if let Some(hour) = natal.hour {
            stems.push(("hour_stem", hour.stem));
        }
        let mut branches = vec![
            ("year_branch", natal.year.branch),
            ("month_branch", natal.month.branch),
            ("day_branch", natal.day.branch),
        ];
        if let Some(hour) = natal.hour {
            branches.push(("hour_branch", hour.branch));
        }

        if let Some(m) = major {
            stems.push(("major_luck_stem", m.stem));
            branches.push(("major_luck_branch", m.branch));
        }
        if let Some(y) = yearly {
            stems.push(("yearly_luck_stem", y.stem));
            branches.push(("yearly_luck_branch", y.branch));
        }
        if let Some(m) = monthly {
            stems.push(("monthly_luck_stem", m.stem));
            branches.push(("monthly_luck_branch", m.branch));
        }
        if let Some(d) = daily {
            stems.push(("daily_luck_stem", d.stem));
            branches.push(("daily_luck_branch", d.branch));
        }
        if let Some(h) = hourly {
            stems.push(("hourly_luck_stem", h.stem));
            branches.push(("hourly_luck_branch", h.branch));
        }

        // Comprehensive interaction analysis based on extended configuration
        let combined_relations = Self::analyze_expanded(&stems, &branches);
        let gaego_events = Self::evaluate_gaego_events(&branches, &combined_relations);
        let ipmyo_events = Self::evaluate_ipmyo_events(natal, major, yearly);
        let structure_state =
            Self::evaluate_structure_state(natal, major, yearly, &combined_relations);

        let major_influence = major.map(|m| Self::get_influence(m, "Major", natal));
        let yearly_influence = yearly.map(|y| Self::get_influence(y, "Yearly", natal));
        let monthly_influence = monthly.map(|m| Self::get_influence(m, "Monthly", natal));
        let daily_influence = daily.map(|d| Self::get_influence(d, "Daily", natal));
        let hourly_influence = hourly.map(|h| Self::get_influence(h, "Hourly", natal));

        Self {
            natal_relations,
            major_influence,
            yearly_influence,
            monthly_influence,
            daily_influence,
            hourly_influence,
            combined_relations,
            gaego_events,
            ipmyo_events,
            structure_state,
        }
    }

    /// Analyze interactions across extended stem/branch collections
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

        // Evaluate all pairs across stems and branches
        use crate::analysis::relationships::{
            Amhap, BranchClash, BranchDestruction, BranchHarm, BranchPunishment, MyungAmHap,
            SemiCombination, SixCombination, StemClash, StemCombination,
        };

        // Analyze heavenly stems
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

        // Analyze earthly branches
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

                // Hidden combinations
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

        // Manifest-hidden combinations
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

        // Check completion of triple and seasonal alliances
        let all_b: Vec<_> = branches.iter().map(|(_, b)| *b).collect();
        analysis.triple_combinations = TripleCombination::check(&all_b);
        analysis.seasonal_combinations = SeasonalCombination::check(&all_b);

        // Dynamic Precedence Hierarchy:
        // Positions participating in completed Triple or Seasonal Alliances
        // suppress lower-precedence branch clashes and six combinations
        let mut alliance_positions = std::collections::HashSet::new();
        for tc in &analysis.triple_combinations {
            let req = tc.branches();
            let mut used_positions = std::collections::HashSet::new();
            for &req_b in &req {
                if let Some((pos, _)) = branches
                    .iter()
                    .find(|(pos, b)| *b == req_b && !used_positions.contains(pos))
                {
                    used_positions.insert(*pos);
                    alliance_positions.insert(*pos);
                }
            }
        }
        for sc in &analysis.seasonal_combinations {
            let req = sc.branches();
            let mut used_positions = std::collections::HashSet::new();
            for &req_b in &req {
                if let Some((pos, _)) = branches
                    .iter()
                    .find(|(pos, b)| *b == req_b && !used_positions.contains(pos))
                {
                    used_positions.insert(*pos);
                    alliance_positions.insert(*pos);
                }
            }
        }

        if !alliance_positions.is_empty() {
            analysis.six_combinations.retain(|(_, p1, p2)| {
                !alliance_positions.contains(p1.as_str())
                    && !alliance_positions.contains(p2.as_str())
            });
            analysis.branch_clashes.retain(|(_, p1, p2)| {
                !alliance_positions.contains(p1.as_str())
                    && !alliance_positions.contains(p2.as_str())
            });
        }

        let is_luck = |p: &str| {
            p.contains("luck")
                || p == "Major"
                || p == "Yearly"
                || p == "Monthly"
                || p == "Daily"
                || p == "Hourly"
        };

        let mut mapped = Vec::new();
        let make_detail = |rel_type: &str, name: &str, p1: &str, p2: &str| {
            crate::analysis::relationships::RelationshipDetail {
                relation_type: rel_type.to_string(),
                name: name.to_string(),
                positions: vec![p1.to_string(), p2.to_string()],
                level: crate::analysis::supplementary_pillars::InterpretationLevel::Neutral,
                summary: format!("{} between {} and {}", name, p1, p2),
                description: "".to_string(),
                reasons: vec![],
                transformed_element: None,
            }
        };

        for (_, p1, p2) in &analysis.stem_combinations {
            if is_luck(p1) && is_luck(p2) {
                mapped.push(make_detail("Combination", "Stem Combination", p1, p2));
            }
        }
        for (_, p1, p2) in &analysis.stem_clashes {
            if is_luck(p1) && is_luck(p2) {
                mapped.push(make_detail("Clash", "Stem Clash", p1, p2));
            }
        }
        for (_, p1, p2) in &analysis.dominant_semi_combinations {
            if is_luck(p1) && is_luck(p2) {
                mapped.push(make_detail(
                    "Combination",
                    "Semi-Alliance (Dominant)",
                    p1,
                    p2,
                ));
            }
        }
        for (_, p1, p2) in &analysis.weak_semi_combinations {
            if is_luck(p1) && is_luck(p2) {
                mapped.push(make_detail("Combination", "Semi-Alliance (Weak)", p1, p2));
            }
        }
        for (_, p1, p2) in &analysis.six_combinations {
            if is_luck(p1) && is_luck(p2) {
                mapped.push(make_detail("Combination", "Six Combination", p1, p2));
            }
        }
        for (_, p1, p2) in &analysis.branch_clashes {
            if is_luck(p1) && is_luck(p2) {
                mapped.push(make_detail("Clash", "Branch Clash", p1, p2));
            }
        }
        for (_, p1, p2) in &analysis.branch_punishments {
            if is_luck(p1) && is_luck(p2) {
                mapped.push(make_detail("Punishment", "Punishment", p1, p2));
            }
        }
        for (_, p1, p2) in &analysis.branch_harms {
            if is_luck(p1) && is_luck(p2) {
                mapped.push(make_detail("Harm", "Harm", p1, p2));
            }
        }
        for (_, p1, p2) in &analysis.branch_destructions {
            if is_luck(p1) && is_luck(p2) {
                mapped.push(make_detail("Destruction", "Destruction", p1, p2));
            }
        }

        analysis.mapped_relationships = mapped;

        analysis
    }

    fn evaluate_gaego_events(
        branches: &[(&'static str, EarthlyBranch)],
        combined: &RelationshipAnalysis,
    ) -> Vec<GaeGoEvent> {
        let mut events = Vec::new();
        let is_storage = |b: EarthlyBranch| {
            matches!(
                b,
                EarthlyBranch::Chen | EarthlyBranch::Xu | EarthlyBranch::Chou | EarthlyBranch::Wei
            )
        };

        for &(pos, b) in branches {
            if is_storage(b) {
                let mut trigger = None;
                for (clash, p1, p2) in &combined.branch_clashes {
                    if (p1 == pos || p2 == pos) && (p1.contains("luck") || p2.contains("luck")) {
                        trigger = Some(format!("BranchClash:{}", clash.hangul()));
                        break;
                    }
                }
                if trigger.is_none() {
                    for tri in &combined.triple_combinations {
                        if tri.branches().contains(&b)
                            && branches
                                .iter()
                                .any(|(p, br)| p.contains("luck") && tri.branches().contains(br))
                        {
                            trigger = Some(format!("TripleAlliance:{}", tri.hangul()));
                            break;
                        }
                    }
                }
                if trigger.is_none() {
                    for sea in &combined.seasonal_combinations {
                        if sea.branches().contains(&b)
                            && branches
                                .iter()
                                .any(|(p, br)| p.contains("luck") && sea.branches().contains(br))
                        {
                            trigger = Some(format!("SeasonalAlliance:{}", sea.hangul()));
                            break;
                        }
                    }
                }
                if trigger.is_none() {
                    for (six, p1, p2) in &combined.six_combinations {
                        if (p1 == pos || p2 == pos) && (p1.contains("luck") || p2.contains("luck"))
                        {
                            trigger = Some(format!("SixCombination:{}", six.hangul()));
                            break;
                        }
                    }
                }

                if let Some(trig) = trigger {
                    events.push(GaeGoEvent {
                        branch: b,
                        position: pos.to_string(),
                        trigger: trig,
                        unsealed_stems: b.jijanggan().to_vec(),
                    });
                }
            }
        }
        events
    }

    fn evaluate_ipmyo_events(
        natal: &FourPillars,
        major: Option<GanZi>,
        yearly: Option<GanZi>,
    ) -> Vec<IpMyoEvent> {
        let dm = natal.day_master();
        let mut events = Vec::new();

        for (luck_ganzi, label) in [(major, "Major"), (yearly, "Yearly")] {
            if let Some(g) = luck_ganzi {
                let stage = crate::core::twelve_stages::calculate_twelve_stage(dm, g.branch);
                if stage == crate::core::twelve_stages::TwelveStage::Mu {
                    let trapped_el = dm.element();
                    events.push(IpMyoEvent {
                        element: trapped_el,
                        tomb_branch: g.branch,
                        trigger: format!("{} 12 Stage Tomb (Mu) Trapping", label),
                    });
                }
            }
        }
        events
    }

    fn evaluate_structure_state(
        natal: &FourPillars,
        major: Option<GanZi>,
        yearly: Option<GanZi>,
        combined: &RelationshipAnalysis,
    ) -> DynamicStructureState {
        use crate::analysis::structure::{StructureAnalysis, StructureType};
        let base_struct = StructureAnalysis::from_pillars(natal).structure;

        let mut active_struct = base_struct;
        let mut status = GyeokStatus::Stable;
        let mut desc = format!("Maintaining base structure {}", base_struct.hangul());

        if let Some(tri) = combined.triple_combinations.first() {
            let el = tri.element();
            active_struct = match el {
                Element::Wood => StructureType::GokJik,
                Element::Fire => StructureType::YeomSang,
                Element::Earth => StructureType::GaSaek,
                Element::Metal => StructureType::JongHyeok,
                Element::Water => StructureType::YoonHa,
            };
            status = GyeokStatus::Transformed;
            desc = format!(
                "Transformed to {} through completion of Triple Alliance ({})",
                tri.hangul(),
                active_struct.hangul()
            );
        } else if let Some(sea) = combined.seasonal_combinations.first() {
            let el = sea.element();
            active_struct = match el {
                Element::Wood => StructureType::GokJik,
                Element::Fire => StructureType::YeomSang,
                Element::Earth => StructureType::GaSaek,
                Element::Metal => StructureType::JongHyeok,
                Element::Water => StructureType::YoonHa,
            };
            status = GyeokStatus::Transformed;
            desc = format!(
                "Transformed to {} through completion of Seasonal Alliance ({})",
                sea.hangul(),
                active_struct.hangul()
            );
        }

        if status == GyeokStatus::Stable {
            let dm = natal.day_master();
            for (luck_stem, label) in [
                (major.map(|m| m.stem), "Major"),
                (yearly.map(|y| y.stem), "Yearly"),
            ] {
                if let Some(s) = luck_stem {
                    let god = crate::core::ten_gods::TenGod::from_stems(dm, s);
                    let is_valid_gyeok_god = !matches!(
                        god,
                        crate::core::ten_gods::TenGod::Bijian
                            | crate::core::ten_gods::TenGod::Jiecai
                    );
                    let is_month_root = natal.month.branch.hidden_stems().contains(&s);
                    if is_month_root && is_valid_gyeok_god {
                        status = GyeokStatus::Fulfilled;
                        desc = format!(
                            "Structure fulfilled by manifestation of {} stem {}",
                            label,
                            god.hangul()
                        );
                        break;
                    }
                }
            }
        }

        if combined
            .branch_clashes
            .iter()
            .any(|(_, p1, p2)| p1 == "month_branch" || p2 == "month_branch")
        {
            status = GyeokStatus::Broken;
            desc = format!(
                "Structure {} broken due to clash with month branch",
                active_struct.hangul()
            );
        }

        DynamicStructureState {
            base_structure: base_struct,
            active_structure: active_struct,
            status,
            description: desc,
        }
    }

    pub fn get_influence(luck: GanZi, label: &str, natal: &FourPillars) -> LuckInfluence {
        let mut relations = Vec::new();
        let mut impacts = Vec::new();

        let mut n_stems = vec![
            ("year_stem", natal.year.stem),
            ("month_stem", natal.month.stem),
            ("day_stem", natal.day.stem),
        ];
        let mut n_branches = vec![
            ("year_branch", natal.year.branch),
            ("month_branch", natal.month.branch),
            ("day_branch", natal.day.branch),
        ];
        if let Some(hour) = natal.hour {
            n_stems.push(("hour_stem", hour.stem));
            n_branches.push(("hour_branch", hour.branch));
        }

        use crate::analysis::relationships::*;
        use crate::core::ten_gods::TenGod;

        let dm = natal.day_master();

        // 1. Heavenly Stem Relations
        for (pos, s) in &n_stems {
            let tg = TenGod::from_stems(dm, *s);
            if let Some(c) = StemCombination::check(luck.stem, *s) {
                relations.push(format!(
                    "Stem Combination: {} - {}",
                    luck.stem.hanja(),
                    s.hanja()
                ));
                impacts.push(DomainImpact {
                    target_position: pos.to_string(),
                    ten_god: tg.hangul().to_string(),
                    interaction: format!("{} Combination", c.hangul()),
                    description: Self::get_stem_impact_description(pos, tg, true),
                });
            }
            if let Some(c) = StemClash::check(luck.stem, *s) {
                relations.push(format!("Stem Clash: {} - {}", luck.stem.hanja(), s.hanja()));
                impacts.push(DomainImpact {
                    target_position: pos.to_string(),
                    ten_god: tg.hangul().to_string(),
                    interaction: format!("{} Clash", c.hangul()),
                    description: Self::get_stem_impact_description(pos, tg, false),
                });
            }
        }

        // 2. Earthly Branch Relations
        for (pos, b) in &n_branches {
            let hidden = b.hidden_stems();
            let main_qi = hidden.last().unwrap();
            let tg = TenGod::from_stems(dm, *main_qi);

            let mut add_impact = |interaction: &str, is_positive: bool| {
                impacts.push(DomainImpact {
                    target_position: pos.to_string(),
                    ten_god: tg.hangul().to_string(),
                    interaction: interaction.to_string(),
                    description: Self::get_branch_impact_description(pos, tg, is_positive),
                });
            };

            // Six Combination
            if let Some(_c) = SixCombination::check(luck.branch, *b) {
                relations.push(format!(
                    "Six Combination: {} - {}",
                    luck.branch.hanja(),
                    b.hanja()
                ));
                add_impact("Six Combination", true);
            }
            // Semi-Alliance
            if let Some(semi) = SemiCombination::check(luck.branch, *b) {
                relations.push(format!(
                    "Semi-Alliance: {} ({}-{})",
                    semi.hangul(),
                    luck.branch.hanja(),
                    b.hanja()
                ));
                add_impact("Semi-Alliance", true);
            }
            // Clash
            if let Some(_c) = BranchClash::check(luck.branch, *b) {
                relations.push(format!(
                    "Branch Clash: {} - {}",
                    luck.branch.hanja(),
                    b.hanja()
                ));
                add_impact("Branch Clash", false);
            }
            // Punishment
            if let Some(p) = BranchPunishment::check_self(luck.branch, *b) {
                relations.push(format!("Branch Punishment: {}", p.hangul()));
                add_impact("Branch Punishment", false);
            }
            // Harm
            if let Some(_h) = BranchHarm::check(luck.branch, *b) {
                relations.push(format!(
                    "Branch Harm: {} - {}",
                    luck.branch.hanja(),
                    b.hanja()
                ));
                add_impact("Branch Harm", false);
            }
            // Destruction
            if let Some(_d) = BranchDestruction::check(luck.branch, *b) {
                relations.push(format!(
                    "Branch Destruction: {} - {}",
                    luck.branch.hanja(),
                    b.hanja()
                ));
                add_impact("Branch Destruction", false);
            }
        }

        LuckInfluence {
            ganzi: luck,
            label: label.to_string(),
            relations_with_natal: relations,
            domain_impacts: impacts,
        }
    }

    fn get_stem_impact_description(
        pos: &str,
        tg: crate::core::ten_gods::TenGod,
        is_positive: bool,
    ) -> String {
        let pos_desc = match pos {
            "year_stem" => "national/social environment and relationships with superiors",
            "month_stem" => "social life, career, and public reputation",
            "day_stem" => "personal identity and core convictions",
            "hour_stem" => "private life and long-term personal aspirations",
            _ => "corresponding pillar",
        };

        let tg_desc = match tg {
            crate::core::ten_gods::TenGod::Bijian | crate::core::ten_gods::TenGod::Jiecai => {
                "peers, interpersonal relationships, and autonomy"
            }
            crate::core::ten_gods::TenGod::Shishen | crate::core::ten_gods::TenGod::Shangguan => {
                "drive, vocational path, and expressiveness"
            }
            crate::core::ten_gods::TenGod::Zhengcai | crate::core::ten_gods::TenGod::Piancai => {
                "wealth, achievements, and tangible outcomes"
            }
            crate::core::ten_gods::TenGod::Zhengguan | crate::core::ten_gods::TenGod::Pianguan => {
                "career, reputation, and responsibility"
            }
            crate::core::ten_gods::TenGod::Zhengyin | crate::core::ten_gods::TenGod::Pianyin => {
                "studies, documents, and qualifications/contracts"
            }
        };

        if is_positive {
            format!("Positive bonding and stability (combination) occurs in {}, favorably enhancing {} or opening new opportunities.", pos_desc, tg_desc)
        } else {
            format!("Fluctuation or friction (clash) occurs in {}, bringing instability or unexpected shifts concerning {}.", pos_desc, tg_desc)
        }
    }

    fn get_branch_impact_description(
        pos: &str,
        tg: crate::core::ten_gods::TenGod,
        is_positive: bool,
    ) -> String {
        let pos_desc = match pos {
            "year_branch" => "ancestral foundation, origins, and macro-level environment",
            "month_branch" => "career, parental lineage, social stage, and residential context",
            "day_branch" => "spouse, partnership, and intimate personal domain",
            "hour_branch" => "children, juniors, and confidential endeavors",
            _ => "corresponding pillar",
        };

        let tg_desc = match tg {
            crate::core::ten_gods::TenGod::Bijian | crate::core::ten_gods::TenGod::Jiecai => {
                "competitive dynamics and decisive self-determination"
            }
            crate::core::ten_gods::TenGod::Shishen | crate::core::ten_gods::TenGod::Shangguan => {
                "investments, entrepreneurial actions, and pathfinding"
            }
            crate::core::ten_gods::TenGod::Zhengcai | crate::core::ten_gods::TenGod::Piancai => {
                "practical revenue streams and asset management"
            }
            crate::core::ten_gods::TenGod::Zhengguan | crate::core::ten_gods::TenGod::Pianguan => {
                "organizational transitions, accountability, and standing"
            }
            crate::core::ten_gods::TenGod::Zhengyin | crate::core::ten_gods::TenGod::Pianyin => {
                "real estate, contract execution, and receptivity"
            }
        };

        if is_positive {
            format!("Stability achieved through combination binding in {}, fostering smooth momentum and supportive aid regarding {}.", pos_desc, tg_desc)
        } else {
            format!("Volatility, relocation, or friction active in {}, presenting abrupt shifts, expenditure, or stress regarding {}.", pos_desc, tg_desc)
        }
    }
}

impl std::fmt::Display for DynamicLuckAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "[Dynamic Comprehensive Analysis (Natal + Major + Yearly)]"
        )?;
        writeln!(f, "---------------------------------")?;

        let print_influence = |f: &mut std::fmt::Formatter<'_>,
                               infl: &LuckInfluence|
         -> std::fmt::Result {
            writeln!(
                f,
                "> {} Influence ({}): {}",
                infl.label,
                infl.ganzi,
                infl.relations_with_natal.join(", ")
            )?;
            for impact in &infl.domain_impacts {
                writeln!(
                    f,
                    "   - [{}({}) {}]: {}",
                    impact.target_position, impact.ten_god, impact.interaction, impact.description
                )?;
            }
            Ok(())
        };

        if let Some(infl) = &self.major_influence {
            print_influence(f, infl)?;
        }
        if let Some(infl) = &self.yearly_influence {
            print_influence(f, infl)?;
        }
        if let Some(infl) = &self.monthly_influence {
            print_influence(f, infl)?;
        }
        if let Some(infl) = &self.daily_influence {
            print_influence(f, infl)?;
        }
        if let Some(infl) = &self.hourly_influence {
            print_influence(f, infl)?;
        }

        writeln!(
            f,
            "
[Combined Interactions]"
        )?;
        write!(f, "{}", self.combined_relations)?;

        Ok(())
    }
}
