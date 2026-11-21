//! Choosing the 用神 (the palaces that stand for the matter asked about) and reading them.
//!
//! The choice of 用神 follows common manual practice; schools differ, so treat the result as one
//! reasonable reading. Scores are the heuristic palace scores of [`crate::analysis::insights`].

use crate::analysis::insights::{QimenInsights, Verdict};
use crate::core::{Deity, Door, Palace, QimenPan, Star};
use eon_saju::core::stem::HeavenlyStem;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum QimenQuestion {
    /// General reading: the querent and the matter of the hour
    General,
    Wealth,
    Career,
    Marriage,
    Health,
    Travel,
    /// Contest or lawsuit: the querent against the opponent
    Lawsuit,
    Study,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YongShenReading {
    pub label: String,
    pub palace: Palace,
    pub score: i32,
    pub verdict: Verdict,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionReading {
    pub question: QimenQuestion,
    /// Palace of the day stem (the querent) and its score
    pub self_palace: Option<Palace>,
    pub self_score: i32,
    pub yongshen: Vec<YongShenReading>,
    /// Aggregate questions: half the querent's score plus half the mean 用神 score.
    /// Contests: the querent's palace score minus the opponent's.
    pub overall_score: f64,
    pub verdict: Verdict,
    pub advice_en: String,
    pub advice_ko: String,
}

fn palace_of_door(pan: &QimenPan, door: Door) -> Option<Palace> {
    pan.palaces
        .iter()
        .find(|p| p.door == Some(door))
        .map(|p| p.palace)
}

fn palace_of_star(pan: &QimenPan, star: Star) -> Option<Palace> {
    pan.palaces
        .iter()
        .find(|p| p.palace != Palace::Center5 && p.star == Some(star))
        .map(|p| p.palace)
}

fn palace_of_deity(pan: &QimenPan, deity: Deity) -> Option<Palace> {
    pan.palaces
        .iter()
        .find(|p| p.deity == Some(deity))
        .map(|p| p.palace)
}

fn palace_of_heaven_stem(pan: &QimenPan, stem: HeavenlyStem) -> Option<Palace> {
    pan.palaces
        .iter()
        .find(|p| p.palace != Palace::Center5 && p.heaven_stem == Some(stem))
        .map(|p| p.palace)
}

/// Read the plate for a question. `is_male` only matters for marriage (the spouse stem differs).
pub fn read_question(
    pan: &QimenPan,
    insights: &QimenInsights,
    question: QimenQuestion,
    is_male: bool,
) -> QuestionReading {
    let score_of = |palace: Palace| insights.palace(palace).map(|p| p.score).unwrap_or(0);

    let mut yongshen: Vec<(String, Option<Palace>)> = Vec::new();
    let mut contest = false;
    match question {
        QimenQuestion::General => {
            yongshen.push(("值使門 (the matter in hand)".into(), insights.envoy_palace));
            yongshen.push(("時干宮 (the hour)".into(), insights.event_palace));
        }
        QimenQuestion::Wealth => {
            yongshen.push(("生門 (income)".into(), palace_of_door(pan, Door::Sheng)));
            yongshen.push((
                "戊 (capital)".into(),
                palace_of_heaven_stem(pan, HeavenlyStem::Wu),
            ));
        }
        QimenQuestion::Career => {
            yongshen.push(("開門 (office)".into(), palace_of_door(pan, Door::Kai)));
            yongshen.push((
                "值符 (authority)".into(),
                palace_of_deity(pan, Deity::ZhiFu),
            ));
        }
        QimenQuestion::Marriage => {
            let (label, stem) = if is_male {
                ("乙 (wife)", HeavenlyStem::Yi)
            } else {
                ("庚 (husband)", HeavenlyStem::Geng)
            };
            yongshen.push((label.into(), palace_of_heaven_stem(pan, stem)));
            yongshen.push(("六合 (union)".into(), palace_of_deity(pan, Deity::LiuHe)));
        }
        QimenQuestion::Health => {
            yongshen.push(("天心 (physician)".into(), palace_of_star(pan, Star::Xin)));
        }
        QimenQuestion::Travel => {
            yongshen.push(("值使門 (the journey)".into(), insights.envoy_palace));
            yongshen.push(("馬星 (movement)".into(), insights.horse_palace));
        }
        QimenQuestion::Lawsuit => {
            contest = true;
            yongshen.push(("時干宮 (opponent)".into(), insights.event_palace));
        }
        QimenQuestion::Study => {
            yongshen.push(("景門 (documents)".into(), palace_of_door(pan, Door::Jing)));
            yongshen.push(("天輔 (learning)".into(), palace_of_star(pan, Star::Fu)));
            yongshen.push((
                "丁 (scholar's wonder)".into(),
                palace_of_heaven_stem(pan, HeavenlyStem::Ding),
            ));
        }
    }

    let yongshen: Vec<YongShenReading> = yongshen
        .into_iter()
        .filter_map(|(label, palace)| {
            palace.map(|palace| {
                let score = score_of(palace);
                YongShenReading {
                    label,
                    palace,
                    score,
                    verdict: Verdict::from_score(f64::from(score)),
                }
            })
        })
        .collect();

    let self_score = insights.self_palace.map(score_of).unwrap_or(0);
    let mean = if yongshen.is_empty() {
        0.0
    } else {
        yongshen.iter().map(|y| f64::from(y.score)).sum::<f64>() / yongshen.len() as f64
    };
    let overall_score = if contest {
        f64::from(self_score) - mean
    } else {
        0.5 * f64::from(self_score) + 0.5 * mean
    };
    let verdict = Verdict::from_score(overall_score);

    let (advice_en, advice_ko) = match (verdict, contest) {
        (Verdict::VeryAuspicious | Verdict::Auspicious, false) => (
            "The palaces that stand for you and for the matter are supportive: a favourable moment to act.",
            "나와 사안을 뜻하는 궁이 모두 받쳐 주는 흐름입니다. 움직이기 좋은 때입니다.",
        ),
        (Verdict::Neutral, false) => (
            "Mixed or quiet signals: proceed carefully and keep options open.",
            "길흉이 섞이거나 잠잠한 신호입니다. 신중히 진행하고 선택지를 열어 두세요.",
        ),
        (Verdict::Inauspicious | Verdict::VeryInauspicious, false) => (
            "The palaces are under strain: postpone or reduce commitments if you can.",
            "관련 궁이 눌려 있습니다. 가능하면 미루거나 규모를 줄이세요.",
        ),
        (Verdict::VeryAuspicious | Verdict::Auspicious, true) => (
            "Your palace outweighs the opponent's: the position favours you.",
            "내 궁이 상대 궁보다 강합니다. 형세가 유리합니다.",
        ),
        (Verdict::Neutral, true) => (
            "The two sides are evenly matched: negotiation beats confrontation.",
            "양쪽이 비슷합니다. 대결보다 협상이 낫습니다.",
        ),
        (Verdict::Inauspicious | Verdict::VeryInauspicious, true) => (
            "The opponent's palace outweighs yours: avoid a head-on contest.",
            "상대 궁이 더 강합니다. 정면 승부는 피하세요.",
        ),
    };

    QuestionReading {
        question,
        self_palace: insights.self_palace,
        self_score,
        yongshen,
        overall_score,
        verdict,
        advice_en: advice_en.to_string(),
        advice_ko: advice_ko.to_string(),
    }
}
