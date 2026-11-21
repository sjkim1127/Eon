use super::insights::*;
use super::lore::{self, PatternKind};
use super::question::*;
use crate::builder::pan::{build_qimen_pan, get_base_door, get_base_star};
use crate::core::{Deity, Door, Palace, QimenPan, Star};
use eon_saju::core::branch::EarthlyBranch as B;
use eon_saju::core::ganzi::GanZi;
use eon_saju::core::stem::HeavenlyStem as S;

fn time() -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
        .unwrap()
        .with_timezone(&chrono::Utc)
}

fn pan_with(day: GanZi, hour: GanZi, is_yin: bool, ju: u8) -> QimenPan {
    let year = GanZi::new(S::Jia, B::Chen);
    let month = GanZi::new(S::Bing, B::Yin);
    build_qimen_pan(time(), year, month, day, hour, is_yin, ju)
}

fn plain_pan() -> QimenPan {
    pan_with(
        GanZi::new(S::Wu, B::Chen),
        GanZi::new(S::Ji, B::Wei),
        false,
        1,
    )
}

fn state_mut(pan: &mut QimenPan, palace: Palace) -> &mut crate::core::PalaceState {
    pan.palaces.iter_mut().find(|p| p.palace == palace).unwrap()
}

/// Clears every stem, door, star and deity so a test can place exactly what it needs.
fn blank(pan: &mut QimenPan) {
    for p in &mut pan.palaces {
        p.earth_stem = None;
        p.heaven_stem = None;
        p.door = None;
        p.star = None;
        p.deity = None;
    }
}

fn ids(insights: &QimenInsights) -> Vec<&str> {
    insights.patterns.iter().map(|p| p.id.as_str()).collect()
}

#[test]
fn void_branches_are_the_two_branches_after_the_xun() {
    // 甲子旬 空 戌亥; 甲寅旬 空 子丑; 甲戌旬 (壬午) 空 申酉.
    assert_eq!(void_branches(GanZi::new(S::Jia, B::Zi)), [B::Xu, B::Hai]);
    assert_eq!(void_branches(GanZi::new(S::Gui, B::You)), [B::Xu, B::Hai]);
    assert_eq!(void_branches(GanZi::new(S::Jia, B::Yin)), [B::Zi, B::Chou]);
    assert_eq!(void_branches(GanZi::new(S::Ren, B::Wu)), [B::Shen, B::You]);
    // Every pillar of one xun shares the same gap.
    for i in 0..60 {
        let first = void_branches(GanZi::from_index((i / 10) * 10));
        assert_eq!(void_branches(GanZi::from_index(i)), first);
    }
}

#[test]
fn horse_star_follows_the_three_harmony_groups() {
    assert_eq!(horse_branch(B::Zi), B::Yin);
    assert_eq!(horse_branch(B::Chen), B::Yin);
    assert_eq!(horse_branch(B::Wu), B::Shen);
    assert_eq!(horse_branch(B::Xu), B::Shen);
    assert_eq!(horse_branch(B::You), B::Hai);
    assert_eq!(horse_branch(B::Chou), B::Hai);
    assert_eq!(horse_branch(B::Mao), B::Si);
    assert_eq!(horse_branch(B::Wei), B::Si);
}

#[test]
fn branches_and_palaces_map_both_ways() {
    for branch in B::ALL {
        let palace = lore::palace_of_branch(branch);
        assert!(lore::palace_branches(palace).contains(&branch));
    }
    let covered: usize = (1..=9)
        .map(|n| lore::palace_branches(Palace::from_u8(n).unwrap()).len())
        .sum();
    assert_eq!(covered, 12);
}

#[test]
fn plate_reports_void_and_horse_palaces() {
    // Hour 己未 is in the 甲寅 xun (void 子丑 -> Kan 1 and Gen 8); the horse of 未 is 巳 -> Xun 4.
    let pan = plain_pan();
    let insights = analyze_plate(&pan);
    assert_eq!(insights.hour_void.palaces, vec![Palace::Kan1, Palace::Gen8]);
    assert_eq!(insights.horse_branch, Some(B::Si));
    assert_eq!(insights.horse_palace, Some(Palace::Xun4));
    assert!(insights.palace(Palace::Xun4).unwrap().horse);
    assert!(insights.palace(Palace::Kan1).unwrap().hour_void);
    assert!(!insights.palace(Palace::Qian6).unwrap().hour_void);
    // Day 戊辰 is in the 甲子 xun: void 戌亥 -> Qian 6.
    assert_eq!(insights.day_void.palaces, vec![Palace::Qian6]);
    assert!(insights.palace(Palace::Qian6).unwrap().day_void);
}

#[test]
fn stem_pair_formations_are_found_in_their_palace() {
    let mut pan = plain_pan();
    blank(&mut pan);
    for (palace, heaven, earth) in [
        (Palace::Gen8, S::Wu, S::Bing),
        (Palace::Li9, S::Bing, S::Wu),
        (Palace::Kan1, S::Geng, S::Bing),
        (Palace::Dui7, S::Gui, S::Ding),
        (Palace::Qian6, S::Geng, S::Gui),
    ] {
        let s = state_mut(&mut pan, palace);
        s.heaven_stem = Some(heaven);
        s.earth_stem = Some(earth);
    }
    let insights = analyze_plate(&pan);
    let find = |id: &str| insights.patterns.iter().find(|p| p.id == id);

    assert_eq!(
        find("qing_long_fan_shou").unwrap().palace,
        Some(Palace::Gen8)
    );
    assert_eq!(find("fei_niao_die_xue").unwrap().palace, Some(Palace::Li9));
    assert_eq!(find("tai_bai_ru_ying").unwrap().palace, Some(Palace::Kan1));
    assert_eq!(
        find("teng_she_yao_jiao").unwrap().palace,
        Some(Palace::Dui7)
    );
    assert_eq!(find("da_ge").unwrap().palace, Some(Palace::Qian6));
    assert_eq!(
        find("qing_long_fan_shou").unwrap().kind,
        PatternKind::Auspicious
    );
    assert_eq!(find("da_ge").unwrap().kind, PatternKind::Inauspicious);
    // Nothing else was placed.
    assert!(find("xiao_ge").is_none());
}

#[test]
fn wonders_in_their_hall_tomb_and_punishment_palaces() {
    let mut pan = plain_pan();
    blank(&mut pan);
    for (palace, heaven) in [
        (Palace::Zhen3, S::Yi),  // 乙 ascends its hall in 震
        (Palace::Li9, S::Bing),  // 丙 in 離
        (Palace::Kun2, S::Yi),   // 乙 enters the tomb in 坤
        (Palace::Gen8, S::Geng), // 庚 punished in 艮
        (Palace::Xun4, S::Ren),  // 壬 punished in 巽
    ] {
        let s = state_mut(&mut pan, palace);
        s.heaven_stem = Some(heaven);
        s.earth_stem = Some(S::Ji);
    }
    let insights = analyze_plate(&pan);
    let at = |id: &str| -> Vec<Option<Palace>> {
        insights
            .patterns
            .iter()
            .filter(|p| p.id == id)
            .map(|p| p.palace)
            .collect()
    };
    assert_eq!(
        at("san_qi_sheng_dian"),
        vec![Some(Palace::Zhen3), Some(Palace::Li9)]
    );
    assert_eq!(at("san_qi_ru_mu"), vec![Some(Palace::Kun2)]);
    assert_eq!(
        at("liu_yi_ji_xing"),
        vec![Some(Palace::Xun4), Some(Palace::Gen8)]
    );
}

#[test]
fn escape_formations_need_stem_door_and_deity() {
    let mut pan = plain_pan();
    blank(&mut pan);
    {
        let s = state_mut(&mut pan, Palace::Kan1);
        s.heaven_stem = Some(S::Bing);
        s.earth_stem = Some(S::Ding);
        s.door = Some(Door::Sheng);
    }
    {
        let s = state_mut(&mut pan, Palace::Zhen3);
        s.heaven_stem = Some(S::Yi);
        s.earth_stem = Some(S::Ji);
        s.door = Some(Door::Kai);
    }
    {
        let s = state_mut(&mut pan, Palace::Dui7);
        s.heaven_stem = Some(S::Ding);
        s.earth_stem = Some(S::Wu);
        s.door = Some(Door::Xiu);
        s.deity = Some(Deity::TaiYin);
    }
    {
        let s = state_mut(&mut pan, Palace::Li9);
        s.heaven_stem = Some(S::Ding);
        s.earth_stem = Some(S::Wu);
        s.door = Some(Door::Du);
        s.deity = Some(Deity::JiuDi);
    }
    {
        // Same stems as the heaven escape but the wrong door: no formation.
        let s = state_mut(&mut pan, Palace::Qian6);
        s.heaven_stem = Some(S::Bing);
        s.earth_stem = Some(S::Ding);
        s.door = Some(Door::Si);
    }
    let found = analyze_plate(&pan);
    let found = ids(&found);
    for id in ["tian_dun", "di_dun", "ren_dun", "gui_dun"] {
        assert!(found.contains(&id), "{id} missing in {found:?}");
    }
    assert_eq!(found.iter().filter(|i| **i == "tian_dun").count(), 1);
    assert!(!found.contains(&"shen_dun"));
}

#[test]
fn jade_maiden_and_three_wonders_attach_to_the_envoy_door() {
    let mut pan = plain_pan();
    blank(&mut pan);
    pan.value_envoy_door = Some(Door::Kai);
    {
        let s = state_mut(&mut pan, Palace::Qian6);
        s.door = Some(Door::Kai);
        s.heaven_stem = Some(S::Ding);
        s.earth_stem = Some(S::Ji);
    }
    let insights = analyze_plate(&pan);
    assert_eq!(insights.envoy_palace, Some(Palace::Qian6));
    assert!(ids(&insights).contains(&"yu_nv_shou_men"));
    assert!(!ids(&insights).contains(&"san_qi_de_shi"));

    state_mut(&mut pan, Palace::Qian6).heaven_stem = Some(S::Bing);
    let insights = analyze_plate(&pan);
    assert!(ids(&insights).contains(&"san_qi_de_shi"));
    assert!(!ids(&insights).contains(&"yu_nv_shou_men"));
}

#[test]
fn geng_over_a_pillar_stem_is_an_obstruction_of_that_pillar() {
    // Day 戊辰, hour 己未: 戊 and 己 are placed on the earth plate; put heaven 庚 over each.
    let mut pan = plain_pan();
    let day_palace = pan
        .palaces
        .iter()
        .find(|p| p.earth_stem == Some(S::Wu))
        .unwrap()
        .palace;
    let hour_palace = pan
        .palaces
        .iter()
        .find(|p| p.earth_stem == Some(S::Ji))
        .unwrap()
        .palace;
    assert_ne!(day_palace, Palace::Center5);
    assert_ne!(hour_palace, Palace::Center5);
    for p in &mut pan.palaces {
        p.heaven_stem = Some(S::Yi);
    }
    state_mut(&mut pan, day_palace).heaven_stem = Some(S::Geng);
    let found = analyze_plate(&pan);
    assert!(ids(&found).contains(&"ri_ge"));
    assert!(!ids(&found).contains(&"shi_ge"));

    state_mut(&mut pan, day_palace).heaven_stem = Some(S::Yi);
    state_mut(&mut pan, hour_palace).heaven_stem = Some(S::Geng);
    let found = analyze_plate(&pan);
    assert!(ids(&found).contains(&"shi_ge"));
    assert!(!ids(&found).contains(&"ri_ge"));
}

#[test]
fn hidden_and_reversed_plates_are_detected() {
    // Hour 甲子: the value star and door do not move, so stars and doors stay home.
    let home = pan_with(
        GanZi::new(S::Wu, B::Chen),
        GanZi::new(S::Jia, B::Zi),
        false,
        1,
    );
    let found = analyze_plate(&home);
    assert!(ids(&found).contains(&"fu_yin_star"));
    assert!(ids(&found).contains(&"fu_yin_door"));

    let mut reversed = plain_pan();
    for p in &mut reversed.palaces {
        if p.palace != Palace::Center5 {
            let n = p.palace.as_u8();
            p.star = Some(get_base_star(10 - n));
            p.door = Some(get_base_door(10 - n));
        }
    }
    let found = analyze_plate(&reversed);
    assert!(ids(&found).contains(&"fan_yin_star"));
    assert!(ids(&found).contains(&"fan_yin_door"));
    assert!(!ids(&found).contains(&"fu_yin_star"));

    let ordinary = analyze_plate(&plain_pan());
    assert!(!ids(&ordinary).contains(&"fu_yin_star"));
    assert!(!ids(&ordinary).contains(&"fan_yin_star"));
}

#[test]
fn five_no_meeting_hour_is_the_same_polarity_controller_of_the_day_stem() {
    // The ten classical hours, day stem -> hour stem.
    let pairs = [
        (S::Jia, S::Geng),
        (S::Yi, S::Xin),
        (S::Bing, S::Ren),
        (S::Ding, S::Gui),
        (S::Wu, S::Jia),
        (S::Ji, S::Yi),
        (S::Geng, S::Bing),
        (S::Xin, S::Ding),
        (S::Ren, S::Wu),
        (S::Gui, S::Ji),
    ];
    // Branch index = stem index keeps every pillar a valid sexagenary pair.
    let pillar = |stem: S| GanZi::new(stem, B::from_index(i32::from(stem.index())));
    for (day, hour) in pairs {
        let pan = pan_with(pillar(day), pillar(hour), false, 1);
        assert!(
            ids(&analyze_plate(&pan)).contains(&"wu_bu_yu_shi"),
            "{} day with {} hour",
            day.hanja(),
            hour.hanja()
        );
    }
    // A controller of the other polarity does not count.
    let pan = pan_with(pillar(S::Jia), pillar(S::Xin), false, 1);
    assert!(!ids(&analyze_plate(&pan)).contains(&"wu_bu_yu_shi"));
}

#[test]
fn palace_scores_combine_door_star_deity_void_and_oppression() {
    let mut pan = plain_pan();
    blank(&mut pan);
    {
        let s = state_mut(&mut pan, Palace::Li9); // fire palace, not void, not horse
        s.door = Some(Door::Kai); // +2
        s.star = Some(Star::Fu); // +2
        s.deity = Some(Deity::ZhiFu); // +2
    }
    {
        // Metal door in a wood palace oppresses it (門迫): -2 on top of the door score.
        let s = state_mut(&mut pan, Palace::Zhen3);
        s.door = Some(Door::Kai); // +2
        s.star = Some(Star::Peng); // -2
        s.deity = Some(Deity::BaiHu); // -2
    }
    {
        // Qian 6 is the day void of plain_pan: only half of +5 remains.
        let s = state_mut(&mut pan, Palace::Qian6);
        s.door = Some(Door::Sheng);
        s.star = Some(Star::Xin);
        s.deity = Some(Deity::JiuTian);
    }
    {
        // Kan 1 is the hour void: -3 -2 -2 halves toward zero to -3, and the Death door (earth)
        // oppresses the water palace for another -2.
        let s = state_mut(&mut pan, Palace::Kan1);
        s.door = Some(Door::Si);
        s.star = Some(Star::Rui);
        s.deity = Some(Deity::BaiHu);
    }
    let insights = analyze_plate(&pan);

    let li = insights.palace(Palace::Li9).unwrap();
    assert!(!li.door_oppresses_palace);
    assert_eq!(li.score, 6);
    assert_eq!(li.verdict, Verdict::VeryAuspicious);

    let zhen = insights.palace(Palace::Zhen3).unwrap();
    assert!(zhen.door_oppresses_palace);
    assert_eq!(zhen.score, 2 - 2 - 2 - 2);
    assert_eq!(zhen.verdict, Verdict::VeryInauspicious);

    let qian = insights.palace(Palace::Qian6).unwrap();
    assert!(qian.day_void && !qian.hour_void);
    assert_eq!(qian.score, 5 / 2);

    let kan = insights.palace(Palace::Kan1).unwrap();
    assert!(kan.hour_void && kan.door_oppresses_palace);
    assert_eq!(kan.score, -7 / 2 - 2);
}

#[test]
fn formations_move_the_palace_score() {
    let mut pan = plain_pan();
    blank(&mut pan);
    let s = state_mut(&mut pan, Palace::Gen8);
    s.heaven_stem = Some(S::Wu);
    s.earth_stem = Some(S::Bing); // 青龍返首: +2
    let insights = analyze_plate(&pan);
    assert_eq!(insights.palace(Palace::Gen8).unwrap().score, 2);
    assert_eq!(
        insights.palace(Palace::Gen8).unwrap().patterns,
        vec!["qing_long_fan_shou"]
    );
}

#[test]
fn analysis_is_deterministic() {
    let pan = plain_pan();
    assert_eq!(analyze_plate(&pan), analyze_plate(&pan));
}

fn question_pan() -> (QimenPan, QimenInsights) {
    let mut pan = plain_pan();
    blank(&mut pan);
    // Day 戊辰 -> 戊 on the earth plate of Kan 1 (the querent); hour 己未 -> 己 on Kun 2.
    state_mut(&mut pan, Palace::Kan1).earth_stem = Some(S::Wu);
    state_mut(&mut pan, Palace::Kun2).earth_stem = Some(S::Ji);
    {
        let s = state_mut(&mut pan, Palace::Kan1);
        s.door = Some(Door::Xiu); // +2
        s.deity = Some(Deity::ZhiFu); // +2 (querent total 4, halved: hour void)
    }
    {
        let s = state_mut(&mut pan, Palace::Qian6);
        s.door = Some(Door::Sheng);
        s.star = Some(Star::Xin);
        s.heaven_stem = Some(S::Wu);
        s.deity = Some(Deity::LiuHe);
    }
    {
        let s = state_mut(&mut pan, Palace::Zhen3);
        s.door = Some(Door::Kai);
        s.heaven_stem = Some(S::Yi);
    }
    {
        let s = state_mut(&mut pan, Palace::Kun2);
        s.door = Some(Door::Si); // -3; querent's opponent palace
        s.star = Some(Star::Rui); // -2
    }
    pan.value_envoy_door = Some(Door::Kai);
    let insights = analyze_plate(&pan);
    (pan, insights)
}

#[test]
fn self_and_event_palaces_come_from_the_day_and_hour_stems() {
    let (_, insights) = question_pan();
    assert_eq!(insights.self_palace, Some(Palace::Kan1));
    assert_eq!(insights.event_palace, Some(Palace::Kun2));
    assert_eq!(insights.envoy_palace, Some(Palace::Zhen3));
}

#[test]
fn wealth_reads_the_life_door_and_wu() {
    let (pan, insights) = question_pan();
    let r = read_question(&pan, &insights, QimenQuestion::Wealth, true);
    assert_eq!(r.yongshen.len(), 2);
    assert!(r.yongshen.iter().all(|y| y.palace == Palace::Qian6));
    // Qian 6 is the day void: its +2 +2 +1 (Sheng, Xin, LiuHe) is halved.
    assert_eq!(r.yongshen[0].score, 5 / 2);
    // The querent's palace (Kan 1, the hour void) keeps half of +2 +2.
    assert_eq!(r.self_score, 2);
    assert!((r.overall_score - (0.5 * 2.0 + 0.5 * 2.0)).abs() < 1e-9);
    assert_eq!(r.verdict, Verdict::Auspicious);
    assert!(!r.advice_en.is_empty() && !r.advice_ko.is_empty());
}

#[test]
fn marriage_looks_for_yi_for_a_man_and_geng_for_a_woman() {
    let (pan, insights) = question_pan();
    let man = read_question(&pan, &insights, QimenQuestion::Marriage, true);
    assert!(man.yongshen[0].label.starts_with('乙'));
    assert_eq!(man.yongshen[0].palace, Palace::Zhen3);

    // No heaven 庚 on this plate: the woman's first 用神 is simply missing.
    let woman = read_question(&pan, &insights, QimenQuestion::Marriage, false);
    assert!(woman.yongshen.iter().all(|y| !y.label.starts_with('庚')));
    assert!(woman.yongshen.iter().any(|y| y.label.starts_with("六合")));
}

#[test]
fn a_lawsuit_compares_the_querent_with_the_opponent() {
    let (pan, insights) = question_pan();
    let r = read_question(&pan, &insights, QimenQuestion::Lawsuit, true);
    assert_eq!(r.yongshen.len(), 1);
    assert_eq!(r.yongshen[0].palace, Palace::Kun2);
    // Querent +2 against the opponent's -5.
    assert_eq!(r.yongshen[0].score, -5);
    assert!((r.overall_score - 7.0).abs() < 1e-9);
    assert_eq!(r.verdict, Verdict::VeryAuspicious);
}

#[test]
fn health_travel_career_and_study_pick_their_yongshen() {
    let (pan, insights) = question_pan();
    let health = read_question(&pan, &insights, QimenQuestion::Health, true);
    assert_eq!(health.yongshen[0].palace, Palace::Qian6); // 天心

    let travel = read_question(&pan, &insights, QimenQuestion::Travel, true);
    assert_eq!(travel.yongshen[0].palace, Palace::Zhen3); // 值使門
    assert_eq!(travel.yongshen[1].palace, Palace::Xun4); // 馬星 of 未 = 巳

    let career = read_question(&pan, &insights, QimenQuestion::Career, true);
    assert_eq!(career.yongshen[0].palace, Palace::Zhen3); // 開門
    assert_eq!(career.yongshen[1].palace, Palace::Kan1); // 值符

    let study = read_question(&pan, &insights, QimenQuestion::Study, true);
    // 景門 and 天輔 are not on this plate; only the 丁 lookup could apply and it is missing too.
    assert!(study.yongshen.is_empty());
    assert_eq!(
        study.verdict,
        Verdict::from_score(0.5 * f64::from(study.self_score))
    );
}

#[test]
fn verdict_thresholds() {
    assert_eq!(Verdict::from_score(3.0), Verdict::VeryAuspicious);
    assert_eq!(Verdict::from_score(1.0), Verdict::Auspicious);
    assert_eq!(Verdict::from_score(0.9), Verdict::Neutral);
    assert_eq!(Verdict::from_score(-0.9), Verdict::Neutral);
    assert_eq!(Verdict::from_score(-1.0), Verdict::Inauspicious);
    assert_eq!(Verdict::from_score(-3.0), Verdict::VeryInauspicious);
}
