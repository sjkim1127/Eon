use serde::{Deserialize, Serialize};
use crate::pillars::FourPillars;
use crate::element::Element;
use crate::yongshin::YongshinAnalysis;

/// 린터 진단 레벨
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LintSeverity {
    Error,
    Warning,
    Info,
}

/// 사주 린트 결과 (Diagnostic)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SajuLint {
    pub code: String,
    pub severity: LintSeverity,
    pub message: String,
    pub advice: String,
}

pub struct DestinyLinter;

impl DestinyLinter {
    pub fn lint(pillars: &FourPillars) -> Vec<SajuLint> {
        let mut diagnostics = Vec::new();
        let yongshin = pillars.yongshin();
        let counts = pillars.element_counts();

        // 1. Missing Dependency (E404): 핵심 용신 부재 또는 극심한 약화
        Self::check_missing_dependency(&counts, &yongshin, &mut diagnostics);

        // 2. Unused Variable (W105): 고립되거나 사용되지 못하는 오행
        Self::check_unused_elements(&counts, &mut diagnostics);

        // 3. Infinite Loop (W202): 에너지 순환의 정체 (특정 오행 편중)
        Self::check_infinite_loop(&counts, &mut diagnostics);

        // 4. Deprecated API (I303): 현대적 마이그레이션이 필요한 고전 패턴
        Self::check_legacy_patterns(pillars, &mut diagnostics);

        diagnostics
    }

    fn check_missing_dependency(counts: &[(Element, u32); 5], yongshin: &YongshinAnalysis, lints: &mut Vec<SajuLint>) {
        let primary_count = counts.iter().find(|(el, _)| *el == yongshin.primary).map(|(_, c)| *c).unwrap_or(0);
        
        if primary_count == 0 {
            lints.push(SajuLint {
                code: "E404".to_string(),
                severity: LintSeverity::Error,
                message: format!("Missing_Dependency: 핵심 라이브러리 '{}'가 선언되지 않았습니다.", yongshin.primary.hangul()),
                advice: format!("외부 운(Luck)에서 '{}'을(를) Import할 때까지 시스템 성능이 제한됩니다.", yongshin.primary.hangul()),
            });
        }
    }

    fn check_unused_elements(counts: &[(Element, u32); 5], lints: &mut Vec<SajuLint>) {
        for (el, count) in counts.iter() {
            if *count > 0 {
                let controller = el.controlled_by();
                let controller_count = counts.iter().find(|(e, _)| *e == controller).map(|(_, c)| *c).unwrap_or(0);
                
                if controller_count >= 3 {
                    lints.push(SajuLint {
                        code: "W105".to_string(),
                        severity: LintSeverity::Warning,
                        message: format!("Unused_Variable: '{}' 자원이 선언되었으나 '{}'에 의해 점유(Lock)되었습니다.", el.hangul(), controller.hangul()),
                        advice: format!("'{}' 프로세스를 강제 종료하거나 제어할 수 있는 보조 루틴이 필요합니다.", controller.hangul()),
                    });
                }
            }
        }
    }

    fn check_infinite_loop(counts: &[(Element, u32); 5], lints: &mut Vec<SajuLint>) {
        for (el, count) in counts.iter() {
            if *count >= 4 {
                lints.push(SajuLint {
                    code: "W202".to_string(),
                    severity: LintSeverity::Warning,
                    message: format!("Resource_Exhaustion: '{}' 오행의 메모리 점유율이 너무 높습니다.", el.hangul()),
                    advice: "특정 모듈에 리소스가 쏠려 전체 시스템의 부하가 예상됩니다. 부하 분산(Load Balancing)이 시급합니다.".to_string(),
                });
            }
        }
    }

    fn check_legacy_patterns(pillars: &FourPillars, lints: &mut Vec<SajuLint>) {
        let analysis = pillars.spirit_markers();
        use crate::spirit_markers::SpiritMarker;
        
        if analysis.markers.iter().any(|m| m.marker == SpiritMarker::Baihu) {
             lints.push(SajuLint {
                code: "I303".to_string(),
                severity: LintSeverity::Info,
                message: "Deprecated_API: '백호(Baekho)' 패턴은 현대 사회에서 '전문직 카리스마'로 대체되었습니다.".to_string(),
                advice: "Legacy 해석을 중단하고 전문 역량 강화를 위한 API로 마이그레이션하십시오.".to_string(),
            });
        }
    }
}
