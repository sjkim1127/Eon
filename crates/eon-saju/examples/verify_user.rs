//! 사용자 사주 데이터 검증 테스트 (지역시 보정 포함)

use eon_core::{BirthInfo, Gender, Location};
use eon_saju::{FourPillars, SajuInput, HeavenlyStem, EarthlyBranch};

fn main() {
    println!("===========================================");
    println!("  김성주님 사주 검증 (지역시 보정 포함)");
    println!("===========================================\n");

    // 방법 1: 기존 SajuInput 사용 (표준시 기준)
    println!("【방법 1】 표준시 기준 (22:00)");
    println!("---------------------------------------------");
    let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
    let pillars = FourPillars::calculate(&input).unwrap();
    println!("{}", pillars);
    println!("한글: {}", pillars.hangul());
    println!();

    // 방법 2: BirthInfo 사용 (지역시 보정)
    println!("【방법 2】 진태양시 기준 (안산, -33분 보정)");
    println!("---------------------------------------------");
    let birth = BirthInfo::solar(2004, 11, 27, 22, 0)
        .with_location(Location::ansan())
        .with_true_solar_time(true)
        .with_gender(Gender::Male);

    println!("출생 정보: {}", birth);
    println!("지역시 보정: {:+}분", birth.time_offset_minutes());
    
    let (hour, minute) = birth.corrected_time();
    println!("보정된 시간: {:02}:{:02}", hour, minute);
    
    // 보정된 시간으로 사주 계산
    let (year, month, day, hour) = birth.for_saju();
    let corrected_input = SajuInput::new_solar(year, month, day, hour, 0);
    let corrected_pillars = FourPillars::calculate(&corrected_input).unwrap();
    
    println!();
    println!("{}", corrected_pillars);
    println!("한글: {}", corrected_pillars.hangul());
    println!();

    // 시주 비교
    println!("【시주 비교】");
    println!("---------------------------------------------");
    println!("표준시(22:00)  시주: {} ({}시)", pillars.hour, pillars.hour.branch.zodiac());
    println!("진태양시(21:27) 시주: {} ({}시)", corrected_pillars.hour, corrected_pillars.hour.branch.zodiac());
    
    if pillars.hour == corrected_pillars.hour {
        println!("\n✓ 시주가 동일합니다 (같은 시진 내)");
    } else {
        println!("\n⚠ 시주가 다릅니다! 지역시 보정이 시주에 영향을 미칩니다.");
    }
    
    println!("\n===========================================");
    println!("  한국 주요 도시 지역시 보정값");
    println!("===========================================");
    let cities = [
        Location::seoul(),
        Location::ansan(),
        Location::incheon(),
        Location::busan(),
        Location::daegu(),
        Location::daejeon(),
        Location::gwangju(),
        Location::ulsan(),
        Location::suwon(),
        Location::jeju(),
    ];
    
    for city in cities {
        println!("{:8}: {:+3}분 ({:.2}°E)", city.name, city.time_offset_minutes(), city.longitude);
    }
}
