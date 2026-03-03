use eon_core::Gender;
use eon_saju::analysis::analytics::Analyzer;
use eon_saju::analysis::major_luck::MajorLuckAnalysis;
use eon_saju::core::pillars::{FourPillars, SajuInput};
use eon_saju::engine::batch::BatchSimulator;
use std::time::Instant;

fn main() {
    println!("=== Hardware Accelerated Saju Audit: 1000-Year Analysis ===");

    // 김성주님 사주 (Ansan -33m)
    let input = SajuInput::new_solar_with_offset(2004, 11, 27, 22, 0, -33);
    let pillars = FourPillars::calculate(&input).expect("Calculation failed");

    // 대운 미리 계산 (김성주님 기준)
    let major_luck =
        MajorLuckAnalysis::calculate_astro(&pillars, Gender::Male, 2004, 11, 27, 22, 0).unwrap();

    let simulator = BatchSimulator::new(pillars);

    // 1000년 전수 조사 벤치마크
    println!("\n[1. Benchmarking Parallel Simulation (1000 Years)]");
    let start = Instant::now();
    let frames = simulator.simulate_1000_years(2004, &major_luck);
    let duration = start.elapsed();

    println!("Simulation Complete.");
    println!("Duration: {:?} for 1,000 yearly frames", duration);
    println!(
        "Throughput: {:.2} frames/sec",
        1000.0 / duration.as_secs_f32()
    );

    // 10만년 스트레스 테스트
    println!("\n[1-2. Stress Test: 100,000 Years]");
    let start_stress = Instant::now();
    // 10만년 시뮬레이션 (2004 ~ 102004)
    let _frames_stress = simulator.simulate_range(2004, 102004, &major_luck);
    let duration_stress = start_stress.elapsed();
    println!("Stress Test Complete.");
    println!("Duration: {:?} for 100,000 yearly frames", duration_stress);
    println!(
        "Throughput: {:.2} frames/sec",
        100000.0 / duration_stress.as_secs_f32()
    );

    // 메모리 해제 (골든 타임 분석은 앞서 계산한 1000년 데이터로 진행)
    drop(_frames_stress);

    // 골든 타임 추출 (10년 윈도우)
    println!("\n[2. Golden Time Extraction (Sliding Window: 10 Years)]");
    let start_anal = Instant::now();
    let timeline: Vec<eon_saju::engine::emulator::YearlyScore> = frames
        .iter()
        .map(|f| eon_saju::engine::emulator::YearlyScore {
            year: 2004 + f.age as i32,
            age: f.age,
            total_score: f.score as f64,
            wealth_score: 0.0,
            career_score: 0.0,
            academic_score: 0.0,
            health_score: 0.0,
            volatility_index: 0.0,
            is_transition_period: false,
            trend_ma_5yr: None,
        })
        .collect();
    if let Some(golden) = Analyzer::find_golden_time(&timeline, 10) {
        let duration_anal = start_anal.elapsed();
        println!("Analysis Complete in {:?}", duration_anal);
        println!("\n>>> GOLDEN TIME DETECTED <<<");
        println!("Period: {}세 ~ {}세", golden.start_age, golden.end_age);
        println!("Average Score: {:.2}/100.0", golden.average_score);
        println!("Status: {}", golden.description);

        // 해당 구간의 구체적인 세운 점수 확인
        println!("\n[Detail for Golden Time Period]");
        let _start_idx = golden.start_age as usize;
        /*
        for j in 0..10 {
            if let Some(f) = frames.get(start_idx + j) {
                println!("  Age {:2}: {:5} 년 - Score: {:.1} (Tags: {:?})",
                    f.age, f.ganzi.hangul(), f.score, f.tags);
            }
        }
        */
    } else {
        println!("No Golden Time found with the given criteria.");
    }

    println!("\n=== Audit Complete ===");
}
