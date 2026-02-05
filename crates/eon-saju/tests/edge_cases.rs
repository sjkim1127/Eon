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
/// 참고: 현재 구현은 eon-astro 엔진을 통해 절기의 정확한 시각을 계산하여 반영합니다.
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
    // 현재 구현: 기본값 use_night_rat_hour = false → 23시에 일주 변경(조자시)
    let input = SajuInput::new_solar(2023, 1, 1, 23, 0);
    let pillars = FourPillars::calculate(&input).unwrap();
    
    println!("23:00 시주: {} 일주: {}", 
        pillars.hour.branch.hanja(), 
        pillars.day.branch.hanja());
    
    // 23시는 子時
    assert_eq!(pillars.hour.branch, EarthlyBranch::Zi, "23:00는 子時여야 함");
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

/// 야자시(夜子時) vs 조자시(朝子時) 모드 비교 테스트
/// 
/// - 야자시(夜子時): 23:00~23:59를 당일 늦은 밤으로 보아 일주 유지
/// - 조자시(朝子時): 23:00~23:59를 다음날 자시로 보아 일주 변경 (기본값)
#[test]
fn test_case_2_night_rat_hour_comparison() {
    // 2023-01-01 23:30
    
    // 1) 조자시 모드 (기본값) - 일주가 다음날(1/2일)로 변경
    let jojasiinput = SajuInput::new_solar(2023, 1, 1, 23, 30);
    let jojasi_pillars = FourPillars::calculate(&jojasiinput).unwrap();
    
    // 2) 야자시 모드 - 일주 유지(1/1일 그대로)
    let yajasi_input = SajuInput::new_solar(2023, 1, 1, 23, 30)
        .with_night_rat_hour(true);
    let yajasi_pillars = FourPillars::calculate(&yajasi_input).unwrap();
    
    println!("=== 23:30 야자시 vs 조자시 비교 ===");
    println!("조자시(기본): 일주 {} 시주 {}", 
        jojasi_pillars.day.hangul(), jojasi_pillars.hour.hangul());
    println!("야자시: 일주 {} 시주 {}", 
        yajasi_pillars.day.hangul(), yajasi_pillars.hour.hangul());
    
    // 두 모드 모두 시주는 子時
    assert_eq!(jojasi_pillars.hour.branch, EarthlyBranch::Zi, "조자시: 시주 子");
    assert_eq!(yajasi_pillars.hour.branch, EarthlyBranch::Zi, "야자시: 시주 子");
    
    // 일주는 두 모드에서 다름 (조자시는 하루 뒤)
    assert_ne!(jojasi_pillars.day, yajasi_pillars.day, 
        "23시에 야자시/조자시 모드의 일주가 달라야 함");
    
    // 조자시의 일주가 야자시보다 1일 늦음 (60갑자에서 다음 간지)
    let yajasi_day_idx = yajasi_pillars.day.index();
    let jojasi_day_idx = jojasi_pillars.day.index();
    assert_eq!((yajasi_day_idx + 1) % 60, jojasi_day_idx, 
        "조자시 일주는 야자시 일주의 다음 간지여야 함");
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
    // BirthInfo.lunar를 사용하여 입력된 정보가 올바르게 설정되는지 확인합니다.
    
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
#[test]
fn test_case_6_advanced_analysis_refinements() {
    // 1. 충(Clash) 가중치 세분화 테스트
    let cardinal_input = SajuInput::new_solar(2020, 6, 21, 0, 0);
    let _cardinal_strength = FourPillars::calculate(&cardinal_input).unwrap().strength();
    
    let storage_input = SajuInput::new_solar(2021, 7, 15, 0, 0);
    let storage_strength = FourPillars::calculate(&storage_input).unwrap().strength();
    
    // 고지충(丑未) 발생 시 라벨 확인
    assert!(storage_strength.deuk_ji.root_positions.iter().any(|p| p.contains("고지충")));
    
    // 2. 신강 사주 가중치 및 억부용신 세분화 테스트
    let strong_input = SajuInput::new_solar(1978, 6, 3, 12, 0); 
    let s_pillars = FourPillars::calculate(&strong_input).unwrap();
    let s_strength = s_pillars.strength();
    let s_yongshin = s_pillars.yongshin();
    
    // 득지 가중치 합계 확인 (A급=1.0, B급=0.5 등 누적)
    assert!(s_strength.deuk_ji.stage_weight_sum >= 2.0);
    
    // 억부용신 상세 이유 확인
    let eokbu = s_yongshin.recommendations.iter()
        .find(|r| r.yongshin_type == eon_saju::analysis::yongshin::YongshinType::Eokbu)
        .expect("억부용신이 있어야 함");
        
    assert!(
        eokbu.reason.contains("용재파인") || 
        eokbu.reason.contains("관살제겁") || 
        eokbu.reason.contains("설기"),
        "억부 이유에 전문 용어가 포함되어야 함: {}", eokbu.reason
    );
}

#[test]
fn test_case_7_structure_yongshin_integration() {
    use eon_saju::core::pillars::{SajuInput, FourPillars};
    use eon_saju::analysis::structure::StructureType;
    
    // 1. 종격(종재격) 테스트 케이스: 일간이 매우 약하고 재성이 강한 경우
    // 1995년 5월 27일 12:00 (예시: 을해년 신사월 무신일 무오시 -> 실제로는 아닐 수 있으나 로직 검증용)
    let input = SajuInput::new_solar(1995, 5, 27, 12, 0);
    let pillars = FourPillars::calculate(&input).unwrap();
    let structure = pillars.structure();
    let yongshin = pillars.yongshin();
    
    // 만약 종격으로 판정되었다면
    if matches!(structure.structure, StructureType::JongAh | StructureType::JongJae | StructureType::JongSal) {
        let eokbu = yongshin.recommendations.iter()
            .find(|r| r.yongshin_type == eon_saju::analysis::yongshin::YongshinType::Eokbu)
            .expect("종격이어도 억부(격국) 용신 분류로 추천되어야 함");
            
        assert!(eokbu.reason.contains("종"), "이유에 '종'이라는 단어가 포함되어야 함: {}", eokbu.reason);
        assert!(eokbu.reason.contains(&structure.structure.hangul()), "이유에 격국 이름이 포함되어야 함: {}", eokbu.reason);
    }
}

#[test]
fn test_case_8_transformation_impact() {
    use eon_saju::core::pillars::{SajuInput, FourPillars};
    use eon_saju::analysis::strength::StrengthAnalysis;
    
    // 갑기합토(甲己合土)가 발생할 수 있는 사주 예시 (진월의 갑목 일간 등)
    // 1984년 4월 15일 12:00 (갑자년 무진월 갑술일 경오시 -> 갑기합은 아니지만 예시용)
    // 실제 갑기합이 발생하는 날짜를 찾아보거나, 인위적으로 생성된 시점 사용
    let input = SajuInput::new_solar(1984, 4, 15, 12, 0); 
    let pillars = FourPillars::calculate(&input).unwrap();
    
    // 1. 합화 미적용 분석 (기본 강약)
    let strength_raw = StrengthAnalysis::from_pillars_with_options(&pillars, false, &eon_saju::AnalysisConfig::default());
    
    // 2. 합화 적용 분석 (변화된 강약)
    let strength_transformed = StrengthAnalysis::from_pillars_with_options(&pillars, true, &eon_saju::AnalysisConfig::default());
    
    println!("Raw Strength: {:.1}%, Transformed Strength: {:.1}%", 
        strength_raw.strength_score, strength_transformed.strength_score);
    
    // 합화가 발생한 경우 두 결과가 달라야 함
    let trans = pillars.transformations();
    let has_stem_transform = trans.year_stem.reason.is_some() || 
                             trans.month_stem.reason.is_some() || 
                             trans.day_stem.reason.is_some() || 
                             trans.hour_stem.reason.is_some();
                             
    if has_stem_transform {
        // 합화가 강약 점수에 영향을 주었는지 확인 (반드시 다르지 않을 수도 있으나 로직 전파 확인용)
        // 여기서는 로직이 전파되는 환경 자체를 검증
        assert!(strength_transformed.deuk_se.support_ratio != strength_raw.deuk_se.support_ratio ||
                strength_transformed.strength_score != strength_raw.strength_score,
                "합화가 발생했다면 강약 분석 결과에 변화가 있어야 함");
    }
}

#[test]
fn test_case_9_vm_dynamic_combination() {
    use eon_saju::core::pillars::{SajuInput, FourPillars};
    use eon_saju::engine::vm::SajuVM;
    use eon_saju::core::ganzi::GanZi;
    
    // 원국에 申, 辰이 있는 사주 (申子辰 수국 완성 가능)
    // 1980년 8월 11일 (경신년 갑신월 병진일 -> 지지에 신, 진 존재)
    let input = SajuInput::new_solar(1980, 8, 11, 12, 0);
    let pillars = FourPillars::calculate(&input).unwrap();
    let vm = SajuVM::new(pillars);
    
    // 1. 삼합이 완성되지 않는 해 (예: 갑자년은 아니지만 자수가 없는 해)
    // 1984년이 갑자년이므로, 1983년(계해) 테스트
    let frame_no_triple = vm.step(4, GanZi::from_index(0), GanZi::from_index(59), None, None, None);
    
    // 2. 삼합(申子辰)이 완성되는 해 (갑자년, 1984년)
    // 갑자(甲子)는 인덱스 0
    let frame_triple = vm.step(5, GanZi::from_index(0), GanZi::from_index(0), None, None, None);
    
    println!("No Triple Score: {:.1}, Triple Score: {:.1}", frame_no_triple.score, frame_triple.score);
    
    // 삼합이 완성되면 '삼합완성' 태그가 있어야 함
    let has_tag = frame_triple.tags.iter().any(|t| t.contains_pattern("삼합완성"));
    assert!(has_tag, "삼합이 완성되는 해에는 관련 태그가 있어야 함. Tags: {:?}", frame_triple.tags);
}

#[test]
fn test_case_10_vm_parallel_simulation() {
    use eon_saju::core::pillars::{SajuInput, FourPillars};
    use eon_saju::engine::vm::SajuVM;
    
    let input = SajuInput::new_solar(1984, 4, 15, 12, 0);
    let pillars = FourPillars::calculate(&input).unwrap();
    let vm = SajuVM::new(pillars);
    
    // 100년 시뮬레이션 수행 (Rayon 활용)
    let start = std::time::Instant::now();
    let frames = vm.simulate_life(1, 100);
    let duration = start.elapsed();
    
    assert_eq!(frames.len(), 100);
    println!("100 years simulation took: {:?}", duration);
}

#[test]
fn test_case_11_vm_simulation_precision() {
    use eon_saju::core::pillars::{SajuInput, FourPillars};
    use eon_saju::engine::vm::SajuVM;
    
    // 2004년 11월 27일생 남자 (대운 시작 3세)
    let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
    let pillars = FourPillars::calculate(&input).unwrap();
    let vm = SajuVM::new(pillars);
    
    let frames = vm.simulate_life(1, 20);
    
    // 1~2세 대운과 3세~ 대운이 달라야 함 (교운기 반영)
    let luck_1 = frames.iter().find(|f| f.age == 1).unwrap().major_ganzi;
    let luck_3 = frames.iter().find(|f| f.age == 3).unwrap().major_ganzi;
    let luck_13 = frames.iter().find(|f| f.age == 13).unwrap().major_ganzi;
    
    println!("Age 1 Major Luck: {}, Age 3 Major Luck: {}, Age 13 Major Luck: {}", luck_1, luck_3, luck_13);
    
    // 대운 시작 전(1세)은 기본값(GanZi(0))이거나 초기 월주 기반일 수 있음 (현재 구현에서는 0 또는 첫 대운 전)
    // 3세와 13세는 서로 다른 대운이어야 함
    assert_ne!(luck_3, luck_13, "10년 주기로 대운이 바뀌어야 함");
    assert_ne!(luck_1, luck_3, "대운 시작 나이(3세) 전후로 대운이 달라야 함");
}

#[test]
fn test_case_12_vm_void_and_talgong() {
    use eon_saju::core::pillars::{SajuInput, FourPillars};
    use eon_saju::engine::vm::SajuVM;
    
    // 갑자 일주의 공망은 '술해(戌亥)'
    // 1984년 11월 20일 12시 -> 갑자년 을해월 갑자일 경오시 (일주 갑자 -> 술해 공망)
    let input = SajuInput::new_solar(1984, 11, 20, 12, 0); // 원국에 술(戌)이나 해(亥)가 있으면 안됨 (여기선 월지 해가 공망)
    let pillars = FourPillars::calculate(&input).unwrap();
    let vm = SajuVM::new(pillars);
    
    // 1. 공망인 세운 (술년/해년) 진입 테스트
    // 1994년(갑술년) -> 술토 공망 대운 혹은 세운 가정
    use eon_saju::core::ganzi::GanZi;
    use eon_saju::core::branch::EarthlyBranch;
    use eon_saju::core::stem::HeavenlyStem;

    let gap_sul = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Xu); // 갑술 (술 공망)
    // 인(寅)은 술(戌)과 인오술 합 -> 이것은 탈공 요인이 됨 (삼합)
    // 따라서 순수 공망을 보기 위해 충/합이 없는 글자로 테스트해야 함
    // 갑자 일주에게 경오시는 자오충.. 복잡함.
    
    // 단순화: step 메서드에 직접 공망 간지를 주입하여 점수 변화 관찰
    
    // A. 평범한 해 (무진년 등)
    let frame_normal = vm.step(20, GanZi::from_index(0), GanZi::from_index(4), None, None, None); // 무진
    
    // B. 공망 해 (갑술년) - 술토는 갑자일주의 공망
    let frame_void = vm.step(21, GanZi::from_index(0), gap_sul, None, None, None);
    
    println!("Normal Score: {:.1}, Void Score: {:.1}", frame_normal.score, frame_void.score);
    
    // 공망이면 점수가 깎여야 함 (단, 탈공 조건이 없어야 함)
    // 원국: 자(子), 해(亥), 자(子), 오(午)
    // 술(戌)이 들어오면... 오(午)와 인오술 반합? (인 없음), 신유술? (없음), 묘술?(합? 자묘형)..
    // 오술 합화(O-Sul Semi-Fire)... 반합이 성립하면 탈공됨.
    // 술해(戌亥) 천라지망...
    
    // 확실한 공망 테스트를 위해 원국 지지와 합이 안되는 공망 글자가 필요하지만
    // 여기서는 공망 태그가 붙었는지 확인하는 것이 확실함
    
    let has_void_tag = frame_void.tags.iter().any(|t| t.contains_pattern("운성공망"));
    let has_escape_tag = frame_void.tags.iter().any(|t| t.contains_pattern("탈공"));
    
    if has_escape_tag {
        println!("Void escaped due to combination/clash. Tags: {:?}", frame_void.tags);
    } else {
        assert!(has_void_tag, "공망 해에는 '운성공망' 태그가 있어야 함");
        assert!(frame_void.score < frame_normal.score - 5.0, "공망 시 점수가 유의미하게 하락해야 함");
    }
}

#[test]
fn test_case_13_vm_shinsal_and_patterns() {
    use eon_saju::core::pillars::{SajuInput, FourPillars};
    use eon_saju::engine::vm::SajuVM;
    use eon_saju::core::ganzi::GanZi;
    use eon_saju::core::branch::EarthlyBranch;
    use eon_saju::core::stem::HeavenlyStem;

    // 김성주님 사주: 갑신년 을해월 경인일 정해시
    let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
    let pillars = FourPillars::calculate(&input).unwrap();
    let vm = SajuVM::new(pillars);

    // 1. 신살 테스트: 역마살 (Yeokmasal)
    // 일지 '인(寅)' 기준, 운에서 '신(申)'이 오면 역마 (인신충이기도 함)
    let gyeong_shen = GanZi::new(HeavenlyStem::Geng, EarthlyBranch::Shen); // 경신
    
    // 2. 12운성 테스트: 왕지(JeWang/GeonRok)
    // 일간 '경(庚)' 기준, '신(申)'은 건록(GeonRok) -> 에너지 강함
    
    let frame_shen = vm.step(20, GanZi::from_index(0), gyeong_shen, None, None, None);
    
    println!("Shen Year Tags: {:?}", frame_shen.tags);
    
    // 역마살 확인
    let has_yeokma = frame_shen.tags.iter().any(|t| t.contains_pattern("신살:역마살"));
    assert!(has_yeokma, "신년(申年)은 인일주에게 역마살이어야 함");
    
    // 12운성 왕성 확인
    let has_geonrok = frame_shen.tags.iter().any(|t| t.contains_pattern("운성:건록"));
    assert!(has_geonrok, "신년(申年)은 경일간에게 건록지여야 함");
    
    // 3. 십신 패턴 테스트: 상관견관
    // 원국에 정관(丁화)가 있음 (시실).
    // 운에서 상관(계수 癸)가 들어오면 상관견관 성립
    let gui_you = GanZi::new(HeavenlyStem::Gui, EarthlyBranch::You); // 계유
    let frame_gui = vm.step(21, GanZi::from_index(0), gui_you, None, None, None);
    
    println!("Gui Year Tags: {:?}", frame_gui.tags);
    
    let has_shangguan_gyeongwan = frame_gui.tags.iter().any(|t| t.contains_pattern("패턴:상관견관"));
    assert!(has_shangguan_gyeongwan, "계수(상관) 운과 정화(정관)이 만나면 상관견관 패턴이 떠야 함");
    
    // 점수 하락 확인 (상관견관은 흉)
    let frame_normal = vm.step(22, GanZi::from_index(0), GanZi::from_index(0), None, None, None);
    println!("Gui Score: {:.1}, Normal Score: {:.1}", frame_gui.score, frame_normal.score);
    assert!(frame_gui.score < frame_normal.score, "상관견관 시 점수가 낮아야 함");
}
