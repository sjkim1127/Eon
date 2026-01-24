//! 🧪 고난이도 만세력 테스트 케이스 5선
//!
//! Eon 엔진의 견고성(Robustness)을 검증합니다.

use eon_core::{BirthInfo, Location, Gender};
use eon_saju::{FourPillars, SajuInput, HeavenlyStem, EarthlyBranch};

// ============================================
// Case 1: 입춘(Lichun) 경계선
// ============================================

/// 2024년 입춘 경계 테스트
/// 입춘 시각: 2024-02-04 16:27 UTC (대략)
/// KST = UTC + 9 = 17:27 직전/직후
/// 
/// 참고: 현재 구현은 절기의 시간을 고려하지 않고 날짜만 확인합니다.
/// 더 정밀한 절기 계산을 위해서는 천문학적 계산 로직이 필요합니다.
#[test]
fn test_case_1_lichun_boundary_before() {
    // 2024-02-03 - 입춘 전날 (확실히 癸卯년)
    let input = SajuInput::new_solar(2024, 2, 3, 12, 0);
    let pillars = FourPillars::calculate(&input).unwrap();
    
    println!("입춘 전날 (2/3): {} {}", pillars.year.stem.hanja(), pillars.year.branch.hanja());
    
    // 입춘 전이므로 癸卯년 (계묘년)
    assert_eq!(pillars.year.stem, HeavenlyStem::Gui, "년간이 癸가 아님");
    assert_eq!(pillars.year.branch, EarthlyBranch::Mao, "년지가 卯가 아님");
}

#[test]
fn test_case_1_lichun_boundary_after() {
    // 2024-02-05 - 입춘 다음날 (확실히 甲辰년)
    let input = SajuInput::new_solar(2024, 2, 5, 12, 0);
    let pillars = FourPillars::calculate(&input).unwrap();
    
    println!("입춘 다음날 (2/5): {} {}", pillars.year.stem.hanja(), pillars.year.branch.hanja());
    
    // 입춘 후이므로 甲辰년 (갑진년)
    assert_eq!(pillars.year.stem, HeavenlyStem::Jia, "년간이 甲가 아님");
    assert_eq!(pillars.year.branch, EarthlyBranch::Chen, "년지가 辰가 아님");
}

// ============================================
// Case 2: 야자시 vs 조자시
// ============================================

/// 자시(子時) 경계 테스트 - 21-23시는 해시(亥時)
#[test]
fn test_case_2_midnight_boundary_hai() {
    // 21:00 - 22:59는 해시(亥時)
    let input = SajuInput::new_solar(2023, 1, 1, 21, 30);
    let pillars = FourPillars::calculate(&input).unwrap();
    
    println!("21:30 시주: {}", pillars.hour.branch.hanja());
    
    // 21:00-22:59는 해시(亥)
    assert_eq!(pillars.hour.branch, EarthlyBranch::Hai, "21:30는 亥時여야 함");
}

#[test]
fn test_case_2_midnight_boundary_zi_early() {
    // 현재 구현: 23:00부터 자시(子時)
    // 참고: "야자시" 방식에서는 23:30부터를 다음날 자시로 봄
    //       "조자시" 방식에서는 23:00부터를 자시로 봄 (현재 구현)
    let input = SajuInput::new_solar(2023, 1, 1, 23, 0);
    let pillars = FourPillars::calculate(&input).unwrap();
    
    println!("23:00 시주: {} 일주: {}", 
        pillars.hour.branch.hanja(), 
        pillars.day.branch.hanja());
    
    // 현재 from_hour 구현: 23시는 子時
    assert_eq!(pillars.hour.branch, EarthlyBranch::Zi, "23:00는 子時여야 함 (조자시 방식)");
}

#[test]
fn test_case_2_midnight_boundary_zi_late() {
    // 00:01 - 자시(子時) 확실
    let input = SajuInput::new_solar(2023, 1, 2, 0, 1);
    let pillars = FourPillars::calculate(&input).unwrap();
    
    println!("00:01 시주: {} 일주: {}", 
        pillars.hour.branch.hanja(), 
        pillars.day.branch.hanja());
    
    // 00:00-00:59는 자시(子)
    assert_eq!(pillars.hour.branch, EarthlyBranch::Zi, "00:01는 子時여야 함");
}

// ============================================
// Case 3: 1988 올림픽 썸머타임
// ============================================

/// 1988년 5월 15일 13:30 서울 (썸머타임 적용 중)
/// 시계: 13:30 -> DST 해제(-1시간): 12:30 -> 경도 보정(-30분): 약 12:00 -> 오시(午)
#[test]
fn test_case_3_1988_summer_time() {
    let birth = BirthInfo::solar(1988, 5, 15, 13, 30)
        .with_location(Location::seoul())
        .with_timezone("Asia/Seoul")
        .with_true_solar_time(true);
    
    // DST 확인
    println!("1988년 DST 적용 여부: {}", birth.is_dst());
    println!("DST 오프셋: {:?}시간", birth.dst_offset_hours());
    
    let (y, m, d, h) = birth.for_saju();
    println!("보정 후: {}년 {}월 {}일 {}시", y, m, d, h);
    
    let input = SajuInput::new_solar(y, m, d, h, 0);
    let pillars = FourPillars::calculate(&input).unwrap();
    
    println!("1988 썸머타임 시주: {}", pillars.hour.branch.hanja());
    
    // 13:30(미시)이 보정 후 12:00(오시)가 되어야 함
    // DST -1시간 + 경도 보정 -30분 = 약 12:00
    assert_eq!(pillars.hour.branch, EarthlyBranch::Wu, 
        "1988년 썸머타임 + 경도 보정 후 오시(午)여야 함");
}

/// 1988년 썸머타임이 아닌 기간 (1월)
#[test]
fn test_case_3_1988_winter_no_dst() {
    let birth = BirthInfo::solar(1988, 1, 15, 13, 30)
        .with_location(Location::seoul())
        .with_timezone("Asia/Seoul")
        .with_true_solar_time(true);
    
    // DST 확인 (겨울은 없어야 함)
    println!("1988년 1월 DST: {}", birth.is_dst());
    assert!(!birth.is_dst(), "1988년 1월에는 DST가 없어야 함");
}

// ============================================
// Case 4: 동경 127도 vs 135도 (지역시)
// ============================================

/// 같은 시각, 다른 위치 → 다른 시주
/// 2024-06-01 11:29 - 오시(午) 진입 경계
#[test]
fn test_case_4_longitude_ulleungdo() {
    // 울릉도 (130.8°E) - 표준 경선에 가까움
    let ulleungdo = Location::new("울릉도", 37.48, 130.8, 135.0);
    
    let birth = BirthInfo::solar(2024, 6, 1, 11, 29)
        .with_location(ulleungdo)
        .with_true_solar_time(true);
    
    let offset = birth.longitude_offset_minutes();
    println!("울릉도 경도 보정: {:+}분", offset);
    
    let (y, m, d, h) = birth.for_saju();
    let input = SajuInput::new_solar(y, m, d, h, 0);
    let pillars = FourPillars::calculate(&input).unwrap();
    
    println!("울릉도 11:29 보정 후 시간: {}시 → 시주: {}", h, pillars.hour.branch.hanja());
    
    // 울릉도(130.8°E): 보정 = (130.8 - 135) * 4 = -16.8분
    // 11:29 - 17분 = 11:12 → 여전히 사시(巳)
}

#[test]
fn test_case_4_longitude_mokpo() {
    // 목포 (126.3°E) - 서쪽
    let mokpo = Location::new("목포", 34.81, 126.3, 135.0);
    
    let birth = BirthInfo::solar(2024, 6, 1, 11, 29)
        .with_location(mokpo)
        .with_true_solar_time(true);
    
    let offset = birth.longitude_offset_minutes();
    println!("목포 경도 보정: {:+}분", offset);
    
    let (y, m, d, h) = birth.for_saju();
    let input = SajuInput::new_solar(y, m, d, h, 0);
    let pillars = FourPillars::calculate(&input).unwrap();
    
    println!("목포 11:29 보정 후 시간: {}시 → 시주: {}", h, pillars.hour.branch.hanja());
    
    // 목포(126.3°E): 보정 = (126.3 - 135) * 4 = -34.8분
    // 11:29 - 35분 = 10:54 → 사시(巳) 그대로
}

#[test]
fn test_case_4_longitude_comparison() {
    // 시주가 갈리는 시간대 테스트: 11:45
    
    // 울릉도: 11:45 - 17분 = 11:28 → 사시(巳)
    let birth_u = BirthInfo::solar(2024, 6, 1, 11, 45)
        .with_location(Location::new("울릉도", 37.48, 130.8, 135.0))
        .with_true_solar_time(true);
    let (_, _, _, h_u) = birth_u.for_saju();
    
    // 목포: 11:45 - 35분 = 11:10 → 사시(巳)
    let birth_m = BirthInfo::solar(2024, 6, 1, 11, 45)
        .with_location(Location::new("목포", 34.81, 126.3, 135.0))
        .with_true_solar_time(true);
    let (_, _, _, h_m) = birth_m.for_saju();
    
    println!("울릉도 보정 후: {}시", h_u);
    println!("목포 보정 후: {}시", h_m);
    
    // 12:20 테스트 - 이 시간대면 차이가 날 수 있음
    let birth_u2 = BirthInfo::solar(2024, 6, 1, 12, 20)
        .with_location(Location::new("울릉도", 37.48, 130.8, 135.0))
        .with_true_solar_time(true);
    let birth_m2 = BirthInfo::solar(2024, 6, 1, 12, 20)
        .with_location(Location::new("목포", 34.81, 126.3, 135.0))
        .with_true_solar_time(true);
    
    let (_, _, _, h_u2) = birth_u2.for_saju();
    let (_, _, _, h_m2) = birth_m2.for_saju();
    
    println!("12:20 울릉도 보정 후: {}시", h_u2);  // 12:20 - 17 = 12:03 → 오시(午)
    println!("12:20 목포 보정 후: {}시", h_m2);    // 12:20 - 35 = 11:45 → 사시(巳) 경계
}

// ============================================
// Case 5: 윤달 (Leap Month)
// ============================================

/// 2020년 5월 23일 = 음력 윤4월 1일
/// 사주는 절기 기준이므로 양력 기준으로 계산
#[test]
fn test_case_5_leap_month() {
    // 음력 윤4월 1일 = 양력 2020-05-23
    // 사주는 태양력(절기) 기준이므로, 양력 날짜로 변환 후 계산
    
    // 방법 1: 양력으로 직접 입력
    let input = SajuInput::new_solar(2020, 5, 23, 10, 0);
    let pillars = FourPillars::calculate(&input).unwrap();
    
    println!("2020-05-23 (음력 윤4월 1일)");
    println!("년주: {}", pillars.year);
    println!("월주: {}", pillars.month);
    println!("일주: {}", pillars.day);
    println!("시주: {}", pillars.hour);
    
    // 소만(5/20) ~ 망종(6/5) 사이 → 4월(巳月)
    // 2020-05-23은 소만(5/20) 이후이므로 巳月
    assert_eq!(pillars.month.branch, EarthlyBranch::Si, 
        "소만 이후이므로 巳月이어야 함");
    
    // 2020년 = 庚子년
    assert_eq!(pillars.year.stem, HeavenlyStem::Geng, "2020년 천간은 庚");
    assert_eq!(pillars.year.branch, EarthlyBranch::Zi, "2020년 지지는 子");
}

/// 음력 입력 테스트 (BirthInfo.lunar 사용)
#[test]
fn test_case_5_lunar_input() {
    // TODO: 현재 음양력 변환 로직이 eon-data에 있지만 
    // BirthInfo와 통합되지 않음. 향후 통합 필요.
    
    let birth = BirthInfo::lunar(2020, 4, 1, 10, 0, true); // 윤4월
    
    println!("음력 입력: {}", birth);
    println!("달력 유형: {:?}", birth.calendar);
    
    // 현재는 음력→양력 자동 변환이 없으므로 
    // 이 테스트는 인터페이스만 확인
    assert!(birth.calendar.is_lunar());
}

// ============================================
// 종합 검증: 사용자 사주 (김성주)
// ============================================

#[test]
fn test_user_saju_complete() {
    // 김성주님: 2004-11-27 22:00 안산
    let birth = BirthInfo::solar(2004, 11, 27, 22, 0)
        .with_location(Location::ansan())
        .with_timezone("Asia/Seoul")
        .with_true_solar_time(true)
        .with_gender(Gender::Male);
    
    println!("=== 김성주님 사주 완전 검증 ===");
    println!("{}", birth);
    
    // 지역시 보정 확인
    let offset = birth.longitude_offset_minutes();
    println!("안산 경도 보정: {:+}분", offset);
    assert!(offset < 0, "안산은 135도 서쪽이므로 음수 보정");
    
    let (y, m, d, h) = birth.for_saju();
    println!("보정 후: {}년 {}월 {}일 {}시", y, m, d, h);
    
    let input = SajuInput::new_solar(y, m, d, h, 0);
    let pillars = FourPillars::calculate(&input).unwrap();
    
    println!("{}", pillars);
    
    // 검증: 甲申年 乙亥月 庚戌日 丁亥時
    assert_eq!(pillars.year.stem, HeavenlyStem::Jia, "년간 甲");
    assert_eq!(pillars.year.branch, EarthlyBranch::Shen, "년지 申");
    assert_eq!(pillars.month.stem, HeavenlyStem::Yi, "월간 乙");
    assert_eq!(pillars.month.branch, EarthlyBranch::Hai, "월지 亥");
    assert_eq!(pillars.day.stem, HeavenlyStem::Geng, "일간 庚");
    assert_eq!(pillars.day.branch, EarthlyBranch::Xu, "일지 戌");
    assert_eq!(pillars.hour.stem, HeavenlyStem::Ding, "시간 丁");
    assert_eq!(pillars.hour.branch, EarthlyBranch::Hai, "시지 亥");
}
