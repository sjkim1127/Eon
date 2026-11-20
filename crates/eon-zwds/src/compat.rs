//! 合盤 (two-chart comparison) for Zi Wei Dou Shu.
//!
//! **This is a heuristic**, not a classical verdict: there is no single canonical 合盤 score. The
//! module reads two things from a pair of charts:
//!
//! 1. 飛星 across charts: the stems of each person's 命宮 and 夫妻宮 are flown into the *other*
//!    person's chart (each stem gives 化祿·化權·化科·化忌, each landing on the palace that holds the
//!    transformed star in the other chart). This is the usual 合盤 technique ("A 的命宮化忌入 B 的
//!    夫妻宮").
//! 2. Star overlaps: stars that stand in both people's 命宮/夫妻宮 (and across 命宮 ↔ 夫妻宮).
//!
//! Scoring (`harmony_score`, 0–100): every flight contributes `type_weight × palace_weight`
//! to either the *supportive* or the *tense* side, and the score is the balance
//! `50 + 50 × (supportive − tense) / (supportive + tense)` (50 when nothing lands).
//!
//! | Transformation | Side | `type_weight` |
//! |----------------|------|---------------|
//! | 化祿 | supportive | 1.0 |
//! | 化科 | supportive | 0.75 |
//! | 化權 | supportive | 0.5 |
//! | 化忌 | tense | 1.5 |
//!
//! | Landing palace | `palace_weight` |
//! |----------------|-----------------|
//! | 命宮, 夫妻宮 | 3.0 |
//! | 福德宮 | 2.0 |
//! | any other | 1.0 |
//!
//! Star overlaps are reported but do not change the score.

use crate::types::{PalaceName, SiHuaType, ZwdsChart, ZwdsStar};
use serde::{Deserialize, Serialize};

/// Which person of the pair.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Person {
    A,
    B,
}

/// One transformation flown from a palace stem of one chart into the other chart.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrossSiHuaHit {
    /// Person whose stem is flying.
    pub from_person: Person,
    /// Palace (of `from_person`'s chart) whose stem flies.
    pub from_palace: PalaceName,
    /// Heavenly stem of that palace.
    pub stem_hanja: String,
    pub sihua_type: SiHuaType,
    pub star: ZwdsStar,
    /// Palace of the *other* person's chart where the transformed star stands.
    pub to_palace: PalaceName,
    /// Signed contribution to the score: positive = supportive, negative = tense.
    pub contribution: f64,
}

/// Stars standing in both of two compared palaces.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StarOverlap {
    pub palace_a: PalaceName,
    pub palace_b: PalaceName,
    pub shared_stars: Vec<ZwdsStar>,
}

/// Result of [`compare_charts`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZwdsCompatibility {
    /// 命宮 / 夫妻宮 stems of A flown into B's chart.
    pub a_to_b: Vec<CrossSiHuaHit>,
    /// 命宮 / 夫妻宮 stems of B flown into A's chart.
    pub b_to_a: Vec<CrossSiHuaHit>,
    /// Shared stars in 命宮 ↔ 命宮, 夫妻宮 ↔ 夫妻宮, A 命宮 ↔ B 夫妻宮 and A 夫妻宮 ↔ B 命宮.
    pub overlaps: Vec<StarOverlap>,
    /// Sum of the supportive contributions.
    pub supportive: f64,
    /// Sum of the tense contributions (positive number).
    pub tense: f64,
    /// 0–100 balance of supportive versus tense flights (heuristic, see the module docs).
    pub harmony_score: f64,
    pub summary: String,
    /// Reminder that the score is a heuristic.
    pub method_note: String,
}

/// Palaces whose stems are flown across the charts.
pub const SOURCE_PALACES: [PalaceName; 2] = [PalaceName::Ming, PalaceName::Fuqi];

fn type_weight(t: SiHuaType) -> f64 {
    match t {
        SiHuaType::HuaLu => 1.0,
        SiHuaType::HuaKe => 0.75,
        SiHuaType::HuaQuan => 0.5,
        SiHuaType::HuaJi => -1.5,
    }
}

fn palace_weight(p: PalaceName) -> f64 {
    match p {
        PalaceName::Ming | PalaceName::Fuqi => 3.0,
        PalaceName::Fude => 2.0,
        _ => 1.0,
    }
}

/// Fly the stems of `SOURCE_PALACES` of `from` into `to`.
pub fn cross_si_hua(from: &ZwdsChart, person: Person, to: &ZwdsChart) -> Vec<CrossSiHuaHit> {
    let mut hits = Vec::new();
    for palace in SOURCE_PALACES {
        let idx = from.palace_index_of(palace);
        let stem = from.palace_stem(idx);
        for landing in to.si_hua_landings_for_stem(stem) {
            hits.push(CrossSiHuaHit {
                from_person: person,
                from_palace: palace,
                stem_hanja: stem.hanja().to_string(),
                sihua_type: landing.sihua_type,
                star: landing.star,
                to_palace: landing.palace_name,
                contribution: type_weight(landing.sihua_type) * palace_weight(landing.palace_name),
            });
        }
    }
    hits
}

fn overlap(
    a: &ZwdsChart,
    palace_a: PalaceName,
    b: &ZwdsChart,
    palace_b: PalaceName,
) -> StarOverlap {
    let stars_b: Vec<ZwdsStar> = b.palace(palace_b).stars.iter().map(|s| s.star).collect();
    let shared_stars = a
        .palace(palace_a)
        .stars
        .iter()
        .map(|s| s.star)
        .filter(|s| stars_b.contains(s))
        .collect();
    StarOverlap {
        palace_a,
        palace_b,
        shared_stars,
    }
}

/// Compare two charts (see the module docs for the method and its limits).
pub fn compare_charts(a: &ZwdsChart, b: &ZwdsChart) -> ZwdsCompatibility {
    let a_to_b = cross_si_hua(a, Person::A, b);
    let b_to_a = cross_si_hua(b, Person::B, a);

    let overlaps = vec![
        overlap(a, PalaceName::Ming, b, PalaceName::Ming),
        overlap(a, PalaceName::Fuqi, b, PalaceName::Fuqi),
        overlap(a, PalaceName::Ming, b, PalaceName::Fuqi),
        overlap(a, PalaceName::Fuqi, b, PalaceName::Ming),
    ];

    let (mut supportive, mut tense) = (0.0, 0.0);
    for hit in a_to_b.iter().chain(&b_to_a) {
        if hit.contribution >= 0.0 {
            supportive += hit.contribution;
        } else {
            tense -= hit.contribution;
        }
    }
    let total = supportive + tense;
    let harmony_score = if total > 0.0 {
        50.0 + 50.0 * (supportive - tense) / total
    } else {
        50.0
    };

    let mut strongest: Vec<&CrossSiHuaHit> = a_to_b.iter().chain(&b_to_a).collect();
    strongest.sort_by(|x, y| y.contribution.abs().total_cmp(&x.contribution.abs()));
    let mut parts: Vec<String> = strongest
        .iter()
        .take(3)
        .map(|h| {
            format!(
                "{}'s {} stem sends {} ({}) into {}'s {}",
                person_label(h.from_person),
                h.from_palace.hanja(),
                h.sihua_type.emoji(),
                h.star.hanja(),
                person_label(match h.from_person {
                    Person::A => Person::B,
                    Person::B => Person::A,
                }),
                h.to_palace.hanja(),
            )
        })
        .collect();
    if parts.is_empty() {
        parts.push("no cross-chart transformations landed".to_string());
    }
    let shared: usize = overlaps.iter().map(|o| o.shared_stars.len()).sum();
    let summary = format!(
        "Harmony {:.1}/100 (supportive {:.2} vs tense {:.2}); strongest flights: {}; {} shared star(s) across 命宮/夫妻宮.",
        harmony_score,
        supportive,
        tense,
        parts.join("; "),
        shared
    );

    ZwdsCompatibility {
        a_to_b,
        b_to_a,
        overlaps,
        supportive,
        tense,
        harmony_score,
        summary,
        method_note: "Heuristic: balance of weighted cross-chart 四化 flights from the 命宮 and 夫妻宮 stems; star overlaps are informational. Not a classical verdict.".to_string(),
    }
}

fn person_label(p: Person) -> &'static str {
    match p {
        Person::A => "A",
        Person::B => "B",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendar::lunar_to_birth_info;
    use crate::chart::build_chart_from_lunar;
    use eon_core::birth::{BirthInfo, Gender};

    fn chart(year: i32, month: u32, day: u32, hour: u32, gender: Gender) -> ZwdsChart {
        let lunar = lunar_to_birth_info(year, month, day, false, hour);
        let birth = BirthInfo::solar(year, 1, 1, hour, 0).with_gender(gender);
        build_chart_from_lunar(&lunar, &birth).unwrap()
    }

    #[test]
    fn cross_flights_land_where_the_other_charts_stars_stand() {
        let a = chart(1990, 4, 21, 10, Gender::Male);
        let b = chart(1984, 6, 1, 20, Gender::Female);
        let r = compare_charts(&a, &b);
        // 2 source palaces × 4 transformations, each direction.
        assert_eq!(r.a_to_b.len(), 8);
        assert_eq!(r.b_to_a.len(), 8);
        // A's 命宮 stem is 戊 (子 palace of a 庚 year): 化祿 貪狼. It must land in the palace
        // where B's chart holds 貪狼.
        let lu = r
            .a_to_b
            .iter()
            .find(|h| h.from_palace == PalaceName::Ming && h.sihua_type == SiHuaType::HuaLu)
            .unwrap();
        assert_eq!(lu.stem_hanja, "戊");
        assert_eq!(lu.star, ZwdsStar::TanLang);
        let tan_lang_in_b = b.star_palace(ZwdsStar::TanLang).unwrap();
        assert_eq!(b.palace_name_at(tan_lang_in_b), lu.to_palace);
    }

    #[test]
    fn score_is_a_bounded_balance() {
        let a = chart(1990, 4, 21, 10, Gender::Male);
        let b = chart(1984, 6, 1, 20, Gender::Female);
        let r = compare_charts(&a, &b);
        assert!((0.0..=100.0).contains(&r.harmony_score));
        let expected = 50.0 + 50.0 * (r.supportive - r.tense) / (r.supportive + r.tense);
        assert!((r.harmony_score - expected).abs() < 1e-9);
        // Deterministic: the same pair yields the identical result.
        assert_eq!(r, compare_charts(&a, &b));
        // A chart compared with itself has overlapping stars in 命宮 ↔ 命宮.
        let same = compare_charts(&a, &a);
        assert_eq!(
            same.overlaps[0].shared_stars.len(),
            a.palace(PalaceName::Ming).stars.len()
        );
    }

    #[test]
    fn contribution_signs_follow_the_transformation() {
        let a = chart(1990, 4, 21, 10, Gender::Male);
        let b = chart(2001, 11, 3, 4, Gender::Female);
        let r = compare_charts(&a, &b);
        for h in r.a_to_b.iter().chain(&r.b_to_a) {
            match h.sihua_type {
                SiHuaType::HuaJi => assert!(h.contribution < 0.0),
                _ => assert!(h.contribution > 0.0),
            }
        }
    }
}
