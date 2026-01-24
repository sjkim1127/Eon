//! 사용자 사주 데이터 검증 테스트 (십성 분석 포함)

use eon_core::{BirthInfo, Gender, Location};
use eon_saju::{FourPillars, SajuInput, TenGodAnalysis, TenGod};

fn main() {
    println!("╔═══════════════════════════════════════════════════╗");
    println!("║     김성주님 사주 분석 (四柱 + 十神)              ║");
    println!("╚═══════════════════════════════════════════════════╝\n");

    // 출생 정보
    let birth = BirthInfo::solar(2004, 11, 27, 22, 0)
        .with_location(Location::ansan())
        .with_true_solar_time(true)
        .with_gender(Gender::Male);

    println!("【출생 정보】");
    println!("─────────────────────────────────────────────────────");
    println!("{}", birth);
    println!();

    // 사주 계산
    let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
    let pillars = FourPillars::calculate(&input).unwrap();

    println!("【사주 팔자】 甲申年 乙亥月 庚戌日 丁亥時");
    println!("─────────────────────────────────────────────────────");
    println!("{}", pillars);
    println!("한글: {}", pillars.hangul());
    println!("일간(日干): {} ({}, {})", 
        pillars.day_master(), 
        pillars.day_master().hangul(),
        pillars.day_master_element()
    );
    println!();

    // 십성 분석
    let analysis = TenGodAnalysis::from_pillars(&pillars);
    
    println!("【십성 분석】");
    println!("─────────────────────────────────────────────────────");
    println!("{}", analysis);
    println!();

    // 상세 십성 표시
    println!("【상세 십성】");
    println!("─────────────────────────────────────────────────────");
    println!("      │ 천간    │ 지지(정기)");
    println!("──────┼─────────┼────────────");
    println!("년주  │ {} {:6} │ {} {:6}", 
        pillars.year.stem.hanja(), analysis.year_stem.hangul(),
        pillars.year.branch.hanja(), analysis.year_branch.hangul()
    );
    println!("월주  │ {} {:6} │ {} {:6}", 
        pillars.month.stem.hanja(), analysis.month_stem.hangul(),
        pillars.month.branch.hanja(), analysis.month_branch.hangul()
    );
    println!("일주  │ {} {:6} │ {} {:6}", 
        pillars.day.stem.hanja(), analysis.day_stem.hangul(),
        pillars.day.branch.hanja(), analysis.day_branch.hangul()
    );
    println!("시주  │ {} {:6} │ {} {:6}", 
        pillars.hour.stem.hanja(), analysis.hour_stem.hangul(),
        pillars.hour.branch.hanja(), analysis.hour_branch.hangul()
    );
    println!();

    // 십성 집계
    println!("【십성 집계】");
    println!("─────────────────────────────────────────────────────");
    let counts = analysis.counts();
    for (god, count) in counts.iter() {
        if *count > 0 {
            let bar = "█".repeat(*count as usize);
            println!("{:6} │ {} ({}개)", god.hangul(), bar, count);
        }
    }
    println!();

    // 기대값과 비교 (사용자 제공 데이터 기준)
    println!("【검증 - 사용자 만세력과 비교】");
    println!("─────────────────────────────────────────────────────");
    println!("년간: 甲 → {} (기대: 편재) {}", 
        analysis.year_stem.hangul(),
        if analysis.year_stem == TenGod::Piancai { "✓" } else { "✗" }
    );
    println!("월간: 乙 → {} (기대: 정재) {}", 
        analysis.month_stem.hangul(),
        if analysis.month_stem == TenGod::Zhengcai { "✓" } else { "✗" }
    );
    println!("시간: 丁 → {} (기대: 정관) {}", 
        analysis.hour_stem.hangul(),
        if analysis.hour_stem == TenGod::Zhengguan { "✓" } else { "✗" }
    );
    println!("년지: 申 → {} (기대: 비견) {}", 
        analysis.year_branch.hangul(),
        if analysis.year_branch == TenGod::Bijian { "✓" } else { "✗" }
    );
    println!("월지: 亥 → {} (기대: 식신) {}", 
        analysis.month_branch.hangul(),
        if analysis.month_branch == TenGod::Shishen { "✓" } else { "✗" }
    );
    println!("일지: 戌 → {} (기대: 편인) {}", 
        analysis.day_branch.hangul(),
        if analysis.day_branch == TenGod::Pianyin { "✓" } else { "✗" }
    );
    println!("시지: 亥 → {} (기대: 식신) {}", 
        analysis.hour_branch.hangul(),
        if analysis.hour_branch == TenGod::Shishen { "✓" } else { "✗" }
    );
}
