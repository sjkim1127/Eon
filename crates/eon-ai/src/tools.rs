use eon_core::Gender;
use eon_saju::{
    DestinyDebugger, DestinyEntropy, DestinyFuzzer, FourPillars, LifePathEmulator, QiTopology,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// AI agent callable tool definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinyTool {
    pub name: String,
    pub description: String,
    pub parameters: Value,
}

pub struct EonToolbox;

impl EonToolbox {
    /// Returns available tools list (for LLM Tool Manifest)
    pub fn get_manifest() -> Vec<DestinyTool> {
        vec![
            DestinyTool {
                name: "analyze_entropy".to_string(),
                description: "Returns Four Pillars energy obfuscation tier and entropy score."
                    .to_string(),
                parameters: serde_json::json!({}),
            },
            DestinyTool {
                name: "scan_topology".to_string(),
                description:
                    "Analyzes Five Elements network traffic flow, bandwidth, and bottlenecks."
                        .to_string(),
                parameters: serde_json::json!({}),
            },
            DestinyTool {
                name: "fuzz_luck_vulnerabilities".to_string(),
                description:
                    "Fuzzes system crash vulnerabilities and critical risks under specific Major Luck contexts."
                        .to_string(),
                parameters: serde_json::json!({
                    "major_ganzi_index": "Integer between 0-59 (Major Luck Ganzhi index)"
                }),
            },
            DestinyTool {
                name: "backtrace_root_cause".to_string(),
                description:
                    "Backtraces root cause of an adverse state or event at a target age."
                        .to_string(),
                parameters: serde_json::json!({
                    "target_age": "Target age to analyze",
                    "target_tag": "Target tag name or pattern to track"
                }),
            },
        ]
    }

    /// Execute tool invocation (Dispatch)
    pub fn call(
        pillars: &FourPillars,
        tool_name: &str,
        params: Value,
    ) -> Result<Value, crate::error::AiError> {
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
