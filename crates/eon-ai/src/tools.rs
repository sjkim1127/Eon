use serde::{Deserialize, Serialize};
use serde_json::Value;
use eon_saju::{
    FourPillars, DestinyEntropy, QiTopology, 
    DestinyDebugger, DestinyFuzzer, LifePathEmulator
};
use eon_core::Gender;

/// AI 에이전트가 호출 가능한 도구 정의
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinyTool {
    pub name: String,
    pub description: String,
    pub parameters: Value,
}

pub struct EonToolbox;

impl EonToolbox {
    /// 사용 가능한 도구 목록 반환 (LLM Manifest용)
    pub fn get_manifest() -> Vec<DestinyTool> {
        vec![
            DestinyTool {
                name: "analyze_entropy".to_string(),
                description: "사주의 에너지 난독화 등급 및 엔트로피 점수를 리턴합니다.".to_string(),
                parameters: serde_json::json!({}),
            },
            DestinyTool {
                name: "scan_topology".to_string(),
                description: "오행 네트워크의 트래픽 흐름, 대역폭 및 병목 구간을 분석합니다.".to_string(),
                parameters: serde_json::json!({}),
            },
            DestinyTool {
                name: "fuzz_luck_vulnerabilities".to_string(),
                description: "특정 대운 컨텍스트에서 발생할 수 있는 시스템 크래시(취약점)를 탐색합니다.".to_string(),
                parameters: serde_json::json!({
                    "major_ganzi_index": "0-59 사이의 정수 (대운 간지 인덱스)"
                }),
            },
            DestinyTool {
                name: "backtrace_root_cause".to_string(),
                description: "특정 나이의 특정 상태(예: '기신', '충')에 대한 근본 원인을 역추적합니다.".to_string(),
                parameters: serde_json::json!({
                    "target_age": "분석할 나이",
                    "target_tag": "추적할 태그명"
                }),
            },
        ]
    }

    /// 도구 호출 실행 (Dispatch)
    pub fn call(pillars: &FourPillars, tool_name: &str, params: Value) -> Result<Value, crate::error::AiError> {
        match tool_name {
            "analyze_entropy" => {
                let res = DestinyEntropy::analyze(pillars);
                serde_json::to_value(res).map_err(|e| e.into())
            }
            "scan_topology" => {
                let res = QiTopology::analyze(pillars);
                serde_json::to_value(res).map_err(|e| e.into())
            }
            "fuzz_luck_vulnerabilities" => {
                let idx = params["major_ganzi_index"].as_i64().unwrap_or(0) as i32;
                let major = eon_saju::GanZi::from_index(idx);
                let vm = eon_saju::SajuVM::new(pillars.clone());
                let fuzzer = DestinyFuzzer::new(vm);
                let res = fuzzer.audit(major);
                serde_json::to_value(res).map_err(|e| e.into())
            }
            "backtrace_root_cause" => {
                let age = params["target_age"].as_u64().unwrap_or(0) as u32;
                let tag = params["target_tag"].as_str().unwrap_or("");
                
                // 에뮬레이션 시뮬레이션 데이터 필요
                let emulator = LifePathEmulator::new(pillars.clone(), Gender::Male, 2004); // 예시용 고정 데이터
                match emulator.emulate() {
                    Ok(report) => {
                        let res = DestinyDebugger::backtrace(&report, age, tag);
                        serde_json::to_value(res).map_err(|e| e.into())
                    }
                    Err(e) => Ok(serde_json::json!({"error": format!("에뮬레이션 실패: {}", e)})),
                }
            }
            _ => Ok(serde_json::json!({"error": "Unknown tool"})),
        }
    }
}
