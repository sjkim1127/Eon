use crate::builder::pan::*;
use eon_saju::core::branch::EarthlyBranch;
use eon_saju::core::ganzi::GanZi;
use eon_saju::core::stem::HeavenlyStem;

fn sample_pan(is_yin_ju: bool, ju_number: u8) -> crate::core::QimenPan {
    let pillar = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Zi);
    build_qimen_pan(
        chrono::DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
            .unwrap()
            .with_timezone(&chrono::Utc),
        pillar,
        pillar,
        pillar,
        pillar,
        is_yin_ju,
        ju_number,
    )
}

#[test]
fn test_xun_shou() {
    let jiazi = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Zi);
    assert_eq!(get_xun_shou(jiazi), HeavenlyStem::Wu);

    let jiayin = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Yin);
    assert_eq!(get_xun_shou(jiayin), HeavenlyStem::Gui);

    let xinwei = GanZi::new(HeavenlyStem::Xin, EarthlyBranch::Wei); // Jia-Zi Xun (Wei=7, Xin=7 => 7-7 = 0)
    assert_eq!(get_xun_shou(xinwei), HeavenlyStem::Wu);
}

#[test]
fn xun_head_mapping_covers_the_complete_sixty_cycle() {
    let expected = [
        HeavenlyStem::Wu,
        HeavenlyStem::Ji,
        HeavenlyStem::Geng,
        HeavenlyStem::Xin,
        HeavenlyStem::Ren,
        HeavenlyStem::Gui,
    ];

    for index in 0..60 {
        let pillar = GanZi::from_index(index);
        let xun = get_xun_shou(pillar);
        assert!(expected.contains(&xun));
        let decan_last = (index / 10) * 10 + 9;
        assert_eq!(
            get_xun_shou(GanZi::from_index(decan_last)),
            xun,
            "旬首 must remain constant within each ten-day period at index {index}"
        );
    }
}

#[test]
fn pan_preserves_nine_palaces_and_eight_plate_permutations() {
    for is_yin_ju in [false, true] {
        let pan = sample_pan(is_yin_ju, 1);
        assert_eq!(pan.palaces.len(), 9);
        assert_eq!(
            pan.palaces
                .iter()
                .map(|palace| palace.palace.as_u8())
                .collect::<Vec<_>>(),
            (1..=9).collect::<Vec<_>>()
        );

        assert_eq!(pan.palaces.iter().filter(|p| p.star.is_some()).count(), 9);
        assert_eq!(pan.palaces.iter().filter(|p| p.door.is_some()).count(), 8);
        assert_eq!(pan.palaces.iter().filter(|p| p.deity.is_some()).count(), 8);
        assert!(pan.palaces[4].star.is_some());
        assert!(pan.palaces[4].heaven_stem.is_some());
    }
}

#[test]
fn pan_changes_direction_between_yang_and_yin_deities() {
    let yang = sample_pan(false, 1);
    let yin = sample_pan(true, 1);
    let yang_positions = yang
        .palaces
        .iter()
        .map(|p| (p.palace.as_u8(), p.deity))
        .collect::<Vec<_>>();
    let yin_positions = yin
        .palaces
        .iter()
        .map(|p| (p.palace.as_u8(), p.deity))
        .collect::<Vec<_>>();
    assert_ne!(yang_positions, yin_positions);
}

#[test]
fn center_xun_head_uses_tian_qin_as_value_chief() {
    let pan = sample_pan(false, 5);
    assert_eq!(pan.value_chief_star, Some(crate::core::Star::Qin));
    assert_eq!(pan.value_envoy_door, Some(crate::core::Door::Si));
}

#[test]
fn center_palace_base_star_is_tian_qin() {
    assert_eq!(get_base_star(5), crate::core::Star::Qin);
}

// -------------------------------------------------------------------------------------------
// Reference tests. Expected values are derived by hand from the standard 轉盤 時家奇門 rules
// (拆補 Ju table, 洛書 order, clockwise ring 坎1-艮8-震3-巽4-離9-坤2-兌7-乾6) and are NOT
// computed with the crate's own tables.
// -------------------------------------------------------------------------------------------

use crate::builder::ju::{calculate_ju, get_yuan_index, JuType, JU_TABLE};
use crate::core::{Deity, Door, Star};
use chrono::TimeZone;

/// (term index counted from 立春, yin?, [上元, 中元, 下元]) — standard 24-term Ju table.
const REF_JU: [(u8, bool, [u8; 3]); 24] = [
    (0, false, [8, 5, 2]),  // 立春
    (1, false, [9, 6, 3]),  // 雨水
    (2, false, [1, 7, 4]),  // 驚蟄
    (3, false, [3, 9, 6]),  // 春分
    (4, false, [4, 1, 7]),  // 清明
    (5, false, [5, 2, 8]),  // 穀雨
    (6, false, [4, 1, 7]),  // 立夏
    (7, false, [5, 2, 8]),  // 小滿
    (8, false, [6, 3, 9]),  // 芒種
    (9, true, [9, 3, 6]),   // 夏至
    (10, true, [8, 2, 5]),  // 小暑
    (11, true, [7, 1, 4]),  // 大暑
    (12, true, [2, 5, 8]),  // 立秋
    (13, true, [1, 4, 7]),  // 處暑
    (14, true, [9, 3, 6]),  // 白露
    (15, true, [7, 1, 4]),  // 秋分
    (16, true, [6, 9, 3]),  // 寒露
    (17, true, [5, 8, 2]),  // 霜降
    (18, true, [6, 9, 3]),  // 立冬
    (19, true, [5, 8, 2]),  // 小雪
    (20, true, [4, 7, 1]),  // 大雪
    (21, false, [1, 7, 4]), // 冬至
    (22, false, [2, 8, 5]), // 小寒
    (23, false, [3, 9, 6]), // 大寒
];

#[test]
fn ju_table_matches_the_standard_twenty_four_term_table() {
    for (idx, yin, juns) in REF_JU {
        let (ju_type, term) = JU_TABLE[idx as usize];
        assert_eq!(ju_type == JuType::Yin, yin, "dun type of term {idx}");
        assert_eq!([term.0, term.1, term.2], juns, "Ju numbers of term {idx}");
    }
}

#[test]
fn yuan_follows_the_fifteen_day_futou_cycle_for_all_sixty_days() {
    // 符頭 days are 甲子/己卯/甲午/己酉 (cycle index 0, 15, 30, 45); each starts 上元,
    // and every 5 days the 元 advances (上元 -> 中元 -> 下元).
    for index in 0..60 {
        assert_eq!(
            get_yuan_index(GanZi::from_index(index)),
            ((index % 15) / 5) as usize,
            "day index {index}"
        );
    }
}

#[test]
fn calculate_ju_matches_reference_dates() {
    // (UTC y, m, d, h, day-pillar cycle index, expected yin?, expected Ju) — expected values
    // from an independent implementation on swisseph (sun longitude -> term).
    let cases: [(i32, u32, u32, u32, i32, bool, u8); 8] = [
        (2000, 1, 1, 12, 54, false, 7),
        (2023, 12, 22, 12, 50, false, 7),
        (2024, 6, 21, 12, 52, true, 3),
        (2024, 3, 20, 12, 19, false, 3),
        (1990, 5, 15, 1, 16, false, 4),
        (2024, 9, 22, 12, 25, true, 6),
        (2025, 1, 10, 6, 15, false, 2),
        (2024, 2, 4, 12, 34, false, 8),
    ];
    for (y, m, d, h, day_idx, yin, ju) in cases {
        let dt = chrono::Utc.with_ymd_and_hms(y, m, d, h, 0, 0).unwrap();
        let got = calculate_ju(dt, GanZi::from_index(day_idx)).unwrap();
        assert_eq!(got, (yin, ju), "{y}-{m}-{d} {h}:00Z");
    }
}

#[test]
fn earth_plate_lays_wu_ji_geng_xin_ren_gui_ding_bing_yi_in_palace_order() {
    use HeavenlyStem::*;
    let order = [Wu, Ji, Geng, Xin, Ren, Gui, Ding, Bing, Yi];
    for ju in 1..=9i32 {
        for yin in [false, true] {
            let plate = build_earth_plate(yin, ju as u8);
            for (i, stem) in order.iter().enumerate() {
                let i = i as i32;
                let palace = if yin {
                    (ju - 1 - i).rem_euclid(9) + 1
                } else {
                    (ju - 1 + i).rem_euclid(9) + 1
                };
                assert_eq!(
                    plate[(palace - 1) as usize],
                    Some(*stem),
                    "ju {ju} yin {yin}"
                );
            }
        }
    }
}

type Row = (
    u8,
    HeavenlyStem,
    HeavenlyStem,
    Option<Door>,
    Star,
    Option<Deity>,
);

fn assert_pan(pan: &crate::core::QimenPan, rows: &[Row; 9]) {
    for &(num, earth, heaven, door, star, deity) in rows {
        let p = &pan.palaces[(num - 1) as usize];
        assert_eq!(p.earth_stem, Some(earth), "earth stem of palace {num}");
        assert_eq!(p.heaven_stem, Some(heaven), "heaven stem of palace {num}");
        assert_eq!(p.door, door, "door of palace {num}");
        assert_eq!(p.star, Some(star), "star of palace {num}");
        assert_eq!(p.deity, deity, "deity of palace {num}");
    }
}

fn pan_for(hour: GanZi, yin: bool, ju: u8) -> crate::core::QimenPan {
    let any = GanZi::from_index(0);
    build_qimen_pan(chrono::Utc::now(), any, any, any, hour, yin, ju)
}

#[test]
fn yang_ju_1_jia_zi_hour_is_the_home_pan() {
    use HeavenlyStem::*;
    // 旬首 戊 sits in palace 1 and the hour stem is 甲 (hidden under 戊): nothing rotates.
    let pan = pan_for(GanZi::from_index(0), false, 1);
    assert_eq!(pan.value_chief_star, Some(Star::Peng));
    assert_eq!(pan.value_envoy_door, Some(Door::Xiu));
    assert_pan(
        &pan,
        &[
            (1, Wu, Wu, Some(Door::Xiu), Star::Peng, Some(Deity::ZhiFu)),
            (2, Ji, Ji, Some(Door::Si), Star::Rui, Some(Deity::XuanWu)),
            (
                3,
                Geng,
                Geng,
                Some(Door::Shang),
                Star::Chong,
                Some(Deity::TaiYin),
            ),
            (4, Xin, Xin, Some(Door::Du), Star::Fu, Some(Deity::LiuHe)),
            (5, Ren, Ren, None, Star::Qin, None),
            (
                6,
                Gui,
                Gui,
                Some(Door::Kai),
                Star::Xin,
                Some(Deity::JiuTian),
            ),
            (
                7,
                Ding,
                Ding,
                Some(Door::Jing2),
                Star::Zhu,
                Some(Deity::JiuDi),
            ),
            (
                8,
                Bing,
                Bing,
                Some(Door::Sheng),
                Star::Ren,
                Some(Deity::TengShe),
            ),
            (9, Yi, Yi, Some(Door::Jing), Star::Ying, Some(Deity::BaiHu)),
        ],
    );
}

#[test]
fn yin_ju_9_bing_yin_hour_rotates_stars_doors_and_deities() {
    use HeavenlyStem::*;
    // 丙寅 hour in the 甲子 xun: 旬首 戊 is in palace 9 (天英 / 景門); hour stem 丙 is in
    // palace 2, so the stars advance one ring step; two hours after 旬首 the 值使 door counts
    // back from 9 to 7 (two ring steps). Yin Ju deities run counter-clockwise from 值符.
    let pan = pan_for(GanZi::from_index(2), true, 9);
    assert_eq!(pan.value_chief_star, Some(Star::Ying));
    assert_eq!(pan.value_envoy_door, Some(Door::Jing));
    assert_pan(
        &pan,
        &[
            (
                1,
                Yi,
                Xin,
                Some(Door::Jing2),
                Star::Xin,
                Some(Deity::XuanWu),
            ),
            (2, Bing, Wu, Some(Door::Du), Star::Ying, Some(Deity::ZhiFu)),
            (3, Ding, Ji, Some(Door::Xiu), Star::Ren, Some(Deity::LiuHe)),
            (
                4,
                Gui,
                Ding,
                Some(Door::Sheng),
                Star::Chong,
                Some(Deity::TaiYin),
            ),
            (5, Ren, Ren, None, Star::Qin, None),
            (6, Xin, Geng, Some(Door::Si), Star::Zhu, Some(Deity::JiuDi)),
            (
                7,
                Geng,
                Bing,
                Some(Door::Jing),
                Star::Rui,
                Some(Deity::JiuTian),
            ),
            (8, Ji, Yi, Some(Door::Kai), Star::Peng, Some(Deity::BaiHu)),
            (
                9,
                Wu,
                Gui,
                Some(Door::Shang),
                Star::Fu,
                Some(Deity::TengShe),
            ),
        ],
    );
}
