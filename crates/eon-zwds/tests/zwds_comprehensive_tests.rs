use eon_core::birth::{BirthInfo, Gender};
use eon_saju::core::stem::HeavenlyStem;
use eon_zwds::{
    build_chart, collisions_zihua::*, monthly_daily::*, transformations::get_sihua_stars,
};

fn get_test_birth_info() -> BirthInfo {
    BirthInfo::solar(1990, 5, 15, 10, 30)
        .with_gender(Gender::Male)
        .with_location(eon_core::Location::seoul())
        .with_korea_timezone()
}

#[test]
fn test_zwds_chart_build_and_advanced_analysis() {
    let birth = get_test_birth_info();
    let chart = build_chart(&birth).unwrap();

    assert_eq!(chart.palaces.len(), 12);
    assert!(!chart.destiny_patterns.is_empty());

    // Test ZiHua detection
    let zihua_list = detect_zi_hua(&chart.palaces);
    println!("Detected {} Zihua entries", zihua_list.len());

    // Test Borrowed Stars detection
    let borrowed_list = detect_borrowed_stars(&chart.palaces);
    println!("Detected {} Borrowed Stars entries", borrowed_list.len());

    // Test Triple Sihua Collision detection
    let collisions = detect_triple_sihua_collisions(&chart.palaces, &chart.daxian, None);
    println!("Detected {} Triple Sihua Collisions", collisions.len());
}

#[test]
fn test_monthly_and_daily_luck_calculation() {
    let annual_palace_idx = 2; // Example: Tiger Palace
    let annual_stem = HeavenlyStem::Bing;

    let liuyue = calculate_liuyue(annual_palace_idx, annual_stem, 6); // 6th Lunar Month
    assert_eq!(liuyue.month, 6);
    assert_eq!(liuyue.palace_idx, (annual_palace_idx + 5) % 12);
    assert_eq!(liuyue.si_hua, get_sihua_stars(HeavenlyStem::Yi)); // Bing year -> 1st month Geng -> 6th month Yi

    let liuri = calculate_liuri(liuyue.palace_idx, 15); // 15th Lunar Day
    assert_eq!(liuri.day, 15);
    assert_eq!(liuri.palace_idx, (liuyue.palace_idx + 14) % 12);
}
