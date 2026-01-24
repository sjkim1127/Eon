use serde::{Deserialize, Serialize};
use eon_saju::{
    DestinyEntropy, QiTopology, KarmaLoadBalancer, 
    DestinyComplexity, DestinyDebugger, DestinyFuzzer,
    LifePathReport
};

pub mod tools;
pub use tools::EonToolbox;

/// AI 기반 운명 보안 감사 리포트 구조
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIAuditReport {
    pub system_id: String,
    pub timestamp: String,
    pub security_score: f32,
    pub technical_summary: String,
    pub auditing_log: Vec<String>,
    pub recommended_patches: Vec<String>,
}

pub struct DestinyAIAuditor;

impl DestinyAIAuditor {
    /// 모든 CS 엔진의 출력을 수집하여 LLM용 컨텍스트 데이터 생성
    pub fn generate_context(
        pillars: &eon_saju::FourPillars, 
        life_report: &LifePathReport,
    ) -> String {
        let entropy = DestinyEntropy::analyze(pillars);
        let topology = QiTopology::analyze(pillars);
        let load_diagnostics = KarmaLoadBalancer::diagnose(&life_report.frames);
        let load_status = load_diagnostics.first()
            .map(|d| d.status.clone())
            .unwrap_or(eon_saju::TrafficStatus::Normal);
        let complexity = DestinyComplexity::analyze(&life_report.frames);
        
        let vm = eon_saju::SajuVM::new(pillars.clone());
        let fuzzer = DestinyFuzzer::new(vm);
        
        // 현재 원국의 월주를 기준으로 기본 정적 감사 수행 (대운 컨텍스트 없이 세운만 60개 퍼징)
        let fuzz_report = fuzzer.audit(pillars.month);

        // TTD Backtrace (예시로 가장 낮은 지점 분석)
        let valley_age = life_report.valley_age;
        let root_cause = DestinyDebugger::backtrace(life_report, valley_age, "기신");

        let mut context = String::new();
        context.push_str("=== [DESTINY SYSTEM CORE DUMP] ===\n");
        context.push_str(&format!("* Entropy/DIE Level: {:?}\n", entropy.level));
        context.push_str(&format!("* Topology Trace: {:?}\n", topology.nodes.iter().map(|n| (n.element.hangul(), n.output)).collect::<Vec<_>>()));
        context.push_str(&format!("* Current Load Status: {:?}\n", load_status));
        context.push_str(&format!("* Cyclomatic Complexity (M): {}\n", complexity.cyclomatic_complexity));
        context.push_str(&format!("* Stability Grade: {}\n", complexity.stability_grade));
        context.push_str(&format!("* Fuzzing Audit: Found {} vulnerabilities in static scan\n", fuzz_report.total_crashes));
        if let Some(rc) = root_cause {
            context.push_str(&format!("* TTD Backtrace Log: {}\n", rc.reason));
        }
        
        context
    }

    /// LLM에게 전달할 최종 프롬프트 생성 (Tool-Calling / Agentic Style)
    pub fn build_agent_prompt(pillars: &eon_saju::FourPillars) -> String {
        let manifest = EonToolbox::get_manifest();
        let manifest_json = serde_json::to_string_pretty(&manifest).unwrap();

        let mut prompt = String::new();
        prompt.push_str("당신은 'Eon Destiny Security Agency'의 수석 에이전트 분석관입니다.\n");
        prompt.push_str("분석 대상 시스템의 기본 정보는 다음과 같습니다:\n");
        prompt.push_str(&format!("* Target System GanZi: {}\n\n", pillars.hangul()));
        
        prompt.push_str("당신은 다음 도구들을 호출하여 시스템의 상세 데이터를 조사할 수 있습니다:\n");
        prompt.push_str(&format!("{}\n\n", manifest_json));

        prompt.push_str("[지침]\n");
        prompt.push_str("1. 먼저 필요한 도구를 호출하여 시스템 상태를 파악하십시오.\n");
        prompt.push_str("2. 수집된 데이터를 바탕으로 '시스템 보안 감사 보고서(Security Audit Report)'를 작성하십시오.\n");
        prompt.push_str("3. 보고서에는 취약점, 병목 구간, 유지보수 복잡도, 해결책(Patch)을 기술 전문 용어로 포함하십시오.\n\n");
        
        prompt.push_str("분석을 시작하기 위해 첫 번째 도구 호출을 수행하거나 계획을 세우십시오.");

        prompt
    }
}
