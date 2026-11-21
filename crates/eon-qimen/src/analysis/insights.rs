//! Reading of a built plate: void palaces (空亡), the horse star (馬星), formations (格局) and a
//! per-palace auspice score.
//!
//! The scores are a transparent heuristic over classical tables (see [`crate::analysis::lore`]); they
//! rank palaces against each other and are not a prediction.

use crate::analysis::lore::{self, PatternInfo, PatternKind};
use crate::builder::pan::get_xun_shou;
use crate::core::{Deity, Door, Palace, QimenPan, Star};
use eon_saju::core::branch::EarthlyBranch;
use eon_saju::core::ganzi::GanZi;
use eon_saju::core::stem::HeavenlyStem;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Verdict {
    VeryAuspicious,
    Auspicious,
    Neutral,
    Inauspicious,
    VeryInauspicious,
}

impl Verdict {
    pub fn from_score(score: f64) -> Self {
        if score >= 3.0 {
            Verdict::VeryAuspicious
        } else if score >= 1.0 {
            Verdict::Auspicious
        } else if score > -1.0 {
            Verdict::Neutral
        } else if score > -3.0 {
            Verdict::Inauspicious
        } else {
            Verdict::VeryInauspicious
        }
    }

    pub fn korean(self) -> &'static str {
        match self {
            Verdict::VeryAuspicious => "대길",
            Verdict::Auspicious => "길",
            Verdict::Neutral => "평",
            Verdict::Inauspicious => "흉",
            Verdict::VeryInauspicious => "대흉",
        }
    }
}

/// Branches that fall into the gap of the ten-day 旬 of a pillar, and the palaces holding them.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoidInfo {
    pub branches: Vec<EarthlyBranch>,
    pub palaces: Vec<Palace>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatternHit {
    pub id: String,
    pub hanja: String,
    pub english: String,
    pub korean: String,
    pub kind: PatternKind,
    /// `None` for formations of the whole plate (伏吟/反吟, 五不遇時)
    pub palace: Option<Palace>,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PalaceReading {
    pub palace: Palace,
    pub earth_stem: Option<HeavenlyStem>,
    pub heaven_stem: Option<HeavenlyStem>,
    pub door: Option<Door>,
    pub star: Option<Star>,
    pub deity: Option<Deity>,
    pub door_score: i32,
    pub star_score: i32,
    pub deity_score: i32,
    pub hour_void: bool,
    pub day_void: bool,
    pub horse: bool,
    /// The door's element controls the palace's element (門迫)
    pub door_oppresses_palace: bool,
    /// Ids of the formations located in this palace
    pub patterns: Vec<String>,
    pub score: i32,
    pub verdict: Verdict,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QimenInsights {
    pub hour_void: VoidInfo,
    pub day_void: VoidInfo,
    pub horse_branch: Option<EarthlyBranch>,
    pub horse_palace: Option<Palace>,
    /// Palace of the day stem on the earth plate (日干宮): the querent
    pub self_palace: Option<Palace>,
    /// Palace of the hour stem on the earth plate (時干宮): the matter at hand
    pub event_palace: Option<Palace>,
    /// Palace holding the value envoy door (值使門)
    pub envoy_palace: Option<Palace>,
    pub patterns: Vec<PatternHit>,
    pub palaces: Vec<PalaceReading>,
}

impl QimenInsights {
    pub fn palace(&self, palace: Palace) -> Option<&PalaceReading> {
        self.palaces.iter().find(|p| p.palace == palace)
    }
}

/// The ten-day 旬 gap: the two branches after the last branch of the xun.
pub fn void_branches(pillar: GanZi) -> [EarthlyBranch; 2] {
    let jia_branch = (pillar.branch.index() as i32 - pillar.stem.index() as i32).rem_euclid(12);
    [
        EarthlyBranch::from_index((jia_branch + 10) % 12),
        EarthlyBranch::from_index((jia_branch + 11) % 12),
    ]
}

/// Horse star (驛馬) of a branch: 申子辰 寅, 寅午戌 申, 巳酉丑 亥, 亥卯未 巳.
pub fn horse_branch(branch: EarthlyBranch) -> EarthlyBranch {
    use EarthlyBranch::*;
    match branch {
        Shen | Zi | Chen => Yin,
        Yin | Wu | Xu => Shen,
        Si | You | Chou => Hai,
        Hai | Mao | Wei => Si,
    }
}

fn void_info(pillar: GanZi) -> VoidInfo {
    let branches = void_branches(pillar).to_vec();
    let mut palaces: Vec<Palace> = branches
        .iter()
        .map(|b| lore::palace_of_branch(*b))
        .collect();
    palaces.dedup();
    VoidInfo { branches, palaces }
}

/// Palace of a stem on the earth plate. 甲 hides under the xun head of `pillar`; a stem that sits in
/// the centre is entrusted to Kun 2 (the same convention the plate builder uses).
fn earth_palace_of(pan: &QimenPan, stem: HeavenlyStem, pillar: GanZi) -> Option<Palace> {
    let target = if stem == HeavenlyStem::Jia {
        get_xun_shou(pillar)
    } else {
        stem
    };
    let found = pan
        .palaces
        .iter()
        .find(|p| p.earth_stem == Some(target))
        .map(|p| p.palace)?;
    Some(if found == Palace::Center5 {
        Palace::Kun2
    } else {
        found
    })
}

fn hit(info: PatternInfo, palace: Option<Palace>, detail: String) -> PatternHit {
    PatternHit {
        id: info.id.to_string(),
        hanja: info.hanja.to_string(),
        english: info.english.to_string(),
        korean: info.korean.to_string(),
        kind: info.kind,
        palace,
        detail,
    }
}

fn base_door_of(palace: u8) -> Door {
    crate::builder::pan::get_base_door(palace)
}

fn base_star_of(palace: u8) -> Star {
    crate::builder::pan::get_base_star(palace)
}

/// Formations of the whole plate.
fn plate_patterns(pan: &QimenPan) -> Vec<PatternHit> {
    let mut hits = Vec::new();
    let outer: Vec<_> = pan
        .palaces
        .iter()
        .filter(|p| p.palace != Palace::Center5)
        .collect();

    let all_stars_home = outer
        .iter()
        .all(|p| p.star == Some(base_star_of(p.palace.as_u8())));
    let all_stars_opposite = outer
        .iter()
        .all(|p| p.star == Some(base_star_of(10 - p.palace.as_u8())));
    if all_stars_home {
        hits.push(hit(
            lore::FU_YIN_STAR,
            None,
            "Every star sits in its home palace: stagnation, things stay as they are".to_string(),
        ));
    } else if all_stars_opposite {
        hits.push(hit(
            lore::FAN_YIN_STAR,
            None,
            "Every star sits opposite its home palace: reversal and upheaval".to_string(),
        ));
    }

    let all_doors_home = outer
        .iter()
        .all(|p| p.door == Some(base_door_of(p.palace.as_u8())));
    let all_doors_opposite = outer
        .iter()
        .all(|p| p.door == Some(base_door_of(10 - p.palace.as_u8())));
    if all_doors_home {
        hits.push(hit(
            lore::FU_YIN_DOOR,
            None,
            "Every door sits in its home palace: delay and standstill".to_string(),
        ));
    } else if all_doors_opposite {
        hits.push(hit(
            lore::FAN_YIN_DOOR,
            None,
            "Every door sits opposite its home palace: sudden change".to_string(),
        ));
    }

    // 五不遇時: the hour stem is the same-polarity controller (七殺) of the day stem.
    let (day, hour) = (pan.day_pillar.stem, pan.hour_pillar.stem);
    if hour.element().controls() == day.element() && hour.polarity() == day.polarity() {
        hits.push(hit(
            lore::WU_BU_YU_SHI,
            None,
            format!(
                "Hour stem {} attacks day stem {}: an unfavourable hour for any undertaking",
                hour.hanja(),
                day.hanja()
            ),
        ));
    }

    hits
}

/// Formations located in one palace.
fn palace_patterns(
    pan: &QimenPan,
    state: &crate::core::PalaceState,
    envoy: Option<Palace>,
) -> Vec<PatternHit> {
    let mut hits = Vec::new();
    let p = state.palace;
    if p == Palace::Center5 {
        return hits;
    }
    let (Some(heaven), Some(earth)) = (state.heaven_stem, state.earth_stem) else {
        return hits;
    };

    for (h, e, info) in lore::STEM_PAIRS {
        if heaven == h && earth == e {
            hits.push(hit(
                info,
                Some(p),
                format!("Heaven {} over earth {}", h.hanja(), e.hanja()),
            ));
        }
    }

    let is_wonder = matches!(
        heaven,
        HeavenlyStem::Yi | HeavenlyStem::Bing | HeavenlyStem::Ding
    );
    if is_wonder && lore::temple_palace(heaven) == Some(p) {
        hits.push(hit(
            lore::SAN_QI_SHENG_DIAN,
            Some(p),
            format!("{} stands in {}", heaven.hanja(), p.trigram()),
        ));
    }
    if is_wonder && lore::tomb_palace(heaven) == Some(p) {
        hits.push(hit(
            lore::SAN_QI_RU_MU,
            Some(p),
            format!(
                "{} falls into the tomb palace {}",
                heaven.hanja(),
                p.trigram()
            ),
        ));
    }
    if lore::punishment_palace(heaven) == Some(p) {
        hits.push(hit(
            lore::LIU_YI_JI_XING,
            Some(p),
            format!("{} is punished in {}", heaven.hanja(), p.trigram()),
        ));
    }

    if envoy == Some(p) && is_wonder {
        let info = if heaven == HeavenlyStem::Ding {
            lore::YU_NV_SHOU_MEN
        } else {
            lore::SAN_QI_ENTER_ENVOY
        };
        hits.push(hit(
            info,
            Some(p),
            format!("{} accompanies the value envoy door", heaven.hanja()),
        ));
    }

    let (door, deity) = (state.door, state.deity);
    if heaven == HeavenlyStem::Bing && earth == HeavenlyStem::Ding && door == Some(Door::Sheng) {
        hits.push(hit(
            lore::TIAN_DUN,
            Some(p),
            "丙 over 丁 with the Life door".to_string(),
        ));
    }
    if heaven == HeavenlyStem::Yi && earth == HeavenlyStem::Ji && door == Some(Door::Kai) {
        hits.push(hit(
            lore::DI_DUN,
            Some(p),
            "乙 over 己 with the Open door".to_string(),
        ));
    }
    if heaven == HeavenlyStem::Ding && deity == Some(Deity::TaiYin) && door == Some(Door::Xiu) {
        hits.push(hit(
            lore::REN_DUN,
            Some(p),
            "丁 with Tai Yin and the Rest door".to_string(),
        ));
    }
    if heaven == HeavenlyStem::Bing && deity == Some(Deity::JiuTian) && door == Some(Door::Sheng) {
        hits.push(hit(
            lore::SHEN_DUN,
            Some(p),
            "丙 with Jiu Tian and the Life door".to_string(),
        ));
    }
    if heaven == HeavenlyStem::Ding && deity == Some(Deity::JiuDi) && door == Some(Door::Du) {
        hits.push(hit(
            lore::GUI_DUN,
            Some(p),
            "丁 with Jiu Di and the Delusion door".to_string(),
        ));
    }

    // 庚格: heaven 庚 over the earth-plate position of a pillar's stem.
    if heaven == HeavenlyStem::Geng {
        for (info, pillar) in [
            (lore::NIAN_GE, pan.year_pillar),
            (lore::YUE_GE, pan.month_pillar),
            (lore::RI_GE, pan.day_pillar),
            (lore::SHI_GE, pan.hour_pillar),
        ] {
            if earth_palace_of(pan, pillar.stem, pillar) == Some(p) {
                hits.push(hit(
                    info,
                    Some(p),
                    format!("庚 over the position of {}", pillar.stem.hanja()),
                ));
            }
        }
    }

    hits
}

/// Compute every insight of a plate.
pub fn analyze_plate(pan: &QimenPan) -> QimenInsights {
    let hour_void = void_info(pan.hour_pillar);
    let day_void = void_info(pan.day_pillar);

    let horse_b = horse_branch(pan.hour_pillar.branch);
    let horse_palace = Some(lore::palace_of_branch(horse_b));

    let envoy_palace = pan.value_envoy_door.and_then(|door| {
        pan.palaces
            .iter()
            .find(|p| p.door == Some(door))
            .map(|p| p.palace)
    });

    let mut patterns = plate_patterns(pan);
    for state in &pan.palaces {
        patterns.extend(palace_patterns(pan, state, envoy_palace));
    }

    let palaces = pan
        .palaces
        .iter()
        .map(|state| {
            let door_score = state.door.map(lore::door_score).unwrap_or(0);
            let star_score = state.star.map(lore::star_score).unwrap_or(0);
            let deity_score = state.deity.map(lore::deity_score).unwrap_or(0);
            let hour_void_here = hour_void.palaces.contains(&state.palace);
            let day_void_here = day_void.palaces.contains(&state.palace);

            // A void palace keeps only half of what its door, star and deity promise.
            let mut score = door_score + star_score + deity_score;
            if hour_void_here || day_void_here {
                score /= 2;
            }

            let door_oppresses_palace = state
                .door
                .map(|d| lore::door_element(d).controls() == lore::palace_element(state.palace))
                .unwrap_or(false);
            if door_oppresses_palace {
                score -= 2;
            }

            let here: Vec<&PatternHit> = patterns
                .iter()
                .filter(|h| h.palace == Some(state.palace))
                .collect();
            for h in &here {
                score += match h.kind {
                    PatternKind::Auspicious => 2,
                    PatternKind::Inauspicious => -2,
                };
            }

            PalaceReading {
                palace: state.palace,
                earth_stem: state.earth_stem,
                heaven_stem: state.heaven_stem,
                door: state.door,
                star: state.star,
                deity: state.deity,
                door_score,
                star_score,
                deity_score,
                hour_void: hour_void_here,
                day_void: day_void_here,
                horse: horse_palace == Some(state.palace),
                door_oppresses_palace,
                patterns: here.iter().map(|h| h.id.clone()).collect(),
                score,
                verdict: Verdict::from_score(f64::from(score)),
            }
        })
        .collect();

    QimenInsights {
        self_palace: earth_palace_of(pan, pan.day_pillar.stem, pan.day_pillar),
        event_palace: earth_palace_of(pan, pan.hour_pillar.stem, pan.hour_pillar),
        envoy_palace,
        hour_void,
        day_void,
        horse_branch: Some(horse_b),
        horse_palace,
        patterns,
        palaces,
    }
}
