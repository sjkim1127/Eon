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

    let xinwei = GanZi::new(HeavenlyStem::Xin, EarthlyBranch::Wei); // 갑자순 (미=7, 신=7 => 7-7 = 0)
    assert_eq!(get_xun_shou(xinwei), HeavenlyStem::Wu);
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
