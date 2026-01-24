//! 사용자 사주 데이터 검증 테스트 (대운 포함)

use eon_core::{BirthInfo, Gender, Location};
use eon_saju::{FourPillars, SajuInput, TenGodAnalysis, MajorLuckAnalysis};

fn main() {
    println!("╔═══════════════════════════════════════════════════════╗");
    println!("║       김성주님 사주 완전 분석 (四柱 + 十神 + 大運)    ║");
    println!("╚═══════════════════════════════════════════════════════╝\n");

    // 출생 정보
    let birth = BirthInfo::solar(2004, 11, 27, 22, 0)
        .with_location(Location::ansan())
        .with_korea_timezone()
        .with_true_solar_time(true)
        .with_gender(Gender::Male);

    println!("【출생 정보】");
    println!("─────────────────────────────────────────────────────────");
    println!("{}", birth);
    println!();

    // 사주 계산
    let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
    let pillars = FourPillars::calculate(&input).unwrap();

    println!("【사주 팔자】");
    println!("─────────────────────────────────────────────────────────");
    println!("{}", pillars);
    println!("한글: {}", pillars.hangul());
    println!("일간(日干): {} ({}, {})", 
        pillars.day_master(), 
        pillars.day_master().hangul(),
        pillars.day_master_element()
    );
    println!();

    // 십성 분석
    let ten_gods = TenGodAnalysis::from_pillars(&pillars);
    println!("【십성 분석】");
    println!("─────────────────────────────────────────────────────────");
    println!("{}", ten_gods);

    // 십성 집계
    println!("【십성 집계】");
    println!("─────────────────────────────────────────────────────────");
    let counts = ten_gods.counts();
    for (god, count) in counts.iter() {
        if *count > 0 {
            let bar = "█".repeat(*count as usize);
            println!("{:6} │ {} ({}개)", god.hangul(), bar, count);
        }
    }
    println!();

    // 대운 분석
    let major_luck = pillars.major_luck(Gender::Male, 2004, 11, 27);
    
    println!("【대운 분석】");
    println!("─────────────────────────────────────────────────────────");
    println!("방향: {} (양년생 남자 → 순행)", major_luck.direction);
    println!("시작: {}세", major_luck.start_age);
    println!();
    
    println!("┌──────┬──────┬──────────────┬──────────────┐");
    println!("│ 나이 │ 간지 │ 천간 십성    │ 지지 십성    │");
    println!("├──────┼──────┼──────────────┼──────────────┤");
    
    for luck in &major_luck.cycles {
        println!("│ {:2}~{:2} │  {}  │ {:^12} │ {:^12} │",
            luck.start_age, luck.end_age,
            luck.ganzi,
            luck.stem_god.hangul(),
            luck.branch_god.hangul()
        );
    }
    println!("└──────┴──────┴──────────────┴──────────────┘");
    println!();

    // 현재 대운 (2026년 기준, 22세)
    let current_age = 22;
    if let Some(current) = major_luck.at_age(current_age) {
        println!("【현재 대운 ({}세)】", current_age);
        println!("─────────────────────────────────────────────────────────");
        println!("간지: {} ({})", current.ganzi, current.ganzi.hangul());
        println!("천간: {} → {}", current.ganzi.stem.hanja(), current.stem_god.hangul());
        println!("지지: {} → {}", current.ganzi.branch.hanja(), current.branch_god.hangul());
    }

    println!();
    println!("【검증 - 사용자 만세력과 비교】");
    println!("─────────────────────────────────────────────────────────");
    println!("사용자 데이터: 3세 丙巳(편관), 13세 丁子(정관), 23세 戊丑(편인)...");
    println!("계산된 첫 대운: {}세 {}", major_luck.cycles[0].start_age, major_luck.cycles[0].ganzi);
}
