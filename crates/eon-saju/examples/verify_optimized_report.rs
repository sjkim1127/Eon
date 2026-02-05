use eon_core::Gender;
use eon_saju::{FourPillars, SajuInput, SajuVM};
use eon_saju::report::SajuReport;
use eon_saju::analysis::analytics::Analyzer;

fn main() {
    println!("--- Saju Optimization & Report Verification ---\n");

    // 1. 임의의 사주 입력 (2004-10-16 음력 -> 2004-11-27 양력)
    let input = SajuInput::new_solar(2004, 11, 27, 22, 0).with_gender(Gender::Male);
    let pillars = FourPillars::calculate(&input).unwrap();
    
    // 2. SajuVM 시뮬레이션 (100년)
    println!("Running 100-year simulation (Parallel)...");
    let start_sim = std::time::Instant::now();
    let vm = SajuVM::new(pillars.clone());
    let frames = vm.simulate_life(1, 100);
    let duration_sim = start_sim.elapsed();
    println!("Simulation done in {:?}\n", duration_sim);

    // 3. 골든 타임 분석
    let golden_time = Analyzer::find_golden_time(&frames, 10);
    
    // 4. 고도화된 리포트 생성
    let mut report = SajuReport::new(pillars.clone())
        .with_vm_simulation(frames.clone());
    
    if let Some(gt) = golden_time {
        report = report.with_golden_time(gt);
    }

    let start_md = std::time::Instant::now();
    let markdown = report.to_markdown();
    let duration_md = start_md.elapsed();
    
    println!("Markdown report generated in {:?}\n", duration_md);

    // 5. 결과 확인 (첫 1000자 출력, 유니코드 경계 안전하게)
    println!("--- Report Preview (Snippet) ---\n");
    let preview_end = markdown.char_indices().nth(1000).map(|(i, _)| i).unwrap_or(markdown.len());
    println!("{}", &markdown[..preview_end]);
    println!("...\n");

    // 6. 결과 파일 저장
    let output_path = "verify_report.md";
    std::fs::write(output_path, markdown).unwrap();
    println!("Full report saved to: {}", output_path);
}
