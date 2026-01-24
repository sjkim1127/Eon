use eon_saju::{FourPillars, SajuInput, LifePathEmulator};
use eon_ai::DestinyAIAuditor;

fn main() {
    // 1. 샘플 입력 (김성주님 사주)
    let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
    let pillars = FourPillars::calculate(&input).unwrap();
    
    // 2. 100년 시뮬레이션 데이터 생성 (김성주님: 남자, 2004년생)
    let emulator = LifePathEmulator::new(pillars.clone(), eon_core::Gender::Male, 2004);
    let _life_report = emulator.emulate();

    println!("====================================================");
    println!("        EON AI DESTINY AUDITOR - PROMPT GEN         ");
    println!("====================================================\n");

    // 3. AI 에이전트용 프롬프트 생성 (Tool Manifest 포함)
    let agent_prompt = DestinyAIAuditor::build_agent_prompt(&pillars);

    println!("AI 에이전트에게 전달될 프롬프트 (Tool Manifest 포함):\n");
    println!("{}\n", agent_prompt);

    println!("====================================================");
    println!("        REAL-TIME TOOL CALL SIMULATION              ");
    println!("====================================================\n");

    // 4. 실제 도구 호출 시뮬레이션 (LLM이 'analyze_entropy'를 호출했다고 가정)
    println!("[AI CALL]: analyze_entropy()");
    let result = eon_ai::EonToolbox::call(&pillars, "analyze_entropy", serde_json::json!({}));
    println!("[SYSTEM RESULT]: {}\n", serde_json::to_string_pretty(&result).unwrap());

    println!("[AI CALL]: fuzz_luck_vulnerabilities(major_ganzi_index: 10)");
    let result = eon_ai::EonToolbox::call(&pillars, "fuzz_luck_vulnerabilities", serde_json::json!({"major_ganzi_index": 10}));
    println!("[SYSTEM RESULT]: (총 {}개의 크래시 발견)\n", result["total_crashes"]);

    println!("----------------------------------------------------");
    println!("AI 에이전트는 위 도구들을 자유롭게 호출하며");
    println!("인생 시스템의 보안 취약점을 심층 조사하게 됩니다.");
    println!("----------------------------------------------------");
}
