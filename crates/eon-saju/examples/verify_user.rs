//! 사용자 사주 데이터 정밀 보정 분석 테스트

use eon_core::{BirthInfo, Gender, Location};
use eon_saju::{FourPillars, SajuInput, AnalysisOptions};

fn main() {
    println!("╔═══════════════════════════════════════════════════════╗");
    println!("║       김성주님 사주 정밀 분석 (4가지 보정 모드)       ║");
    println!("╚═══════════════════════════════════════════════════════╝\n");

    // 1. 출생 정보
    let birth = BirthInfo::solar(2004, 11, 27, 22, 0)
        .with_location(Location::ansan())
        .with_korea_timezone()
        .with_true_solar_time(true)
        .with_gender(Gender::Male);

    // 2. 사주 팔자 계산
    let (y, m, d, h) = birth.for_saju();
    let input = SajuInput::new_solar(y, m, d, h, 0);
    let pillars = FourPillars::calculate(&input).unwrap();

    println!("【사주 팔자】: {}\n", pillars.hangul());

    // 격국 분석
    println!("{}", pillars.structure());
    println!();

    // 용신 분석
    println!("{}", pillars.yongshin());
    println!();

    // 공망 분석
    println!("{}", pillars.void_analysis());
    println!();

    // 월령분금(사령) 분석
    println!("{}", pillars.saryeong(27));
    println!();

    // 대운 정밀 분석 (2004년 대설 절입 시각: 12월 7일 03:48)
    println!("【대운 정밀 분석】");
    let luck = pillars.major_luck_precise(
        Gender::Male, 
        2004, 11, 27, 22, 0, // 출생
        2004, 12, 7, 3, 48   // 다음 절입 (대설)
    );
    println!("{}", luck);
    println!();

    // 3. 4가지 모드 분석 실행
    let cases = [
        (false, false, "1. 기본 (보정 X)"),
        (true, false, "2. 합화만 적용"),
        (true, true, "3. 합화 + 궁성/조후 보정 적용"),
        (false, true, "4. 궁성/조후 보정만 적용"),
    ];

    for (transform, correction, title) in cases {
        let options = AnalysisOptions {
            apply_transform: transform,
            apply_correction: correction,
        };
        
        let result = pillars.integrated_analysis(options);
        
        println!("----------------------------------------------------");
        println!("{}", title);
        println!("{}", result);
    }
}
