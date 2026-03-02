use serde::{Deserialize, Serialize};
use crate::core::pillars::FourPillars;
use crate::core::element::Element;
use crate::analysis::yongshin::YongshinAnalysis;

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
                message: format!("핵심 기운 부재: 필요한 '{}' 기운이 사주에 없습니다.", yongshin.primary.hangul()),
                advice: format!("대운이나 세운에서 '{}' 기운이 들어올 때까지 핵심 역량 발휘가 제한될 수 있습니다.", yongshin.primary.hangul()),
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
                        message: format!("기운 억제: '{}' 기운이 강한 '{}' 기운에 눌려 제대로 발휘되지 못하고 있습니다.", el.hangul(), controller.hangul()),
                        advice: format!("'{}' 기운을 조절하거나 균형을 잡아줄 보완 요소가 필요합니다.", controller.hangul()),
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
                    message: format!("기운 편중: '{}' 기운이 지나치게 강합니다.", el.hangul()),
                    advice: "한 가지 기운에 치우치면 전체 흐름이 불균형해집니다. 부족한 기운을 보완하는 것이 중요합니다.".to_string(),
                });
            }
        }
    }

    fn check_legacy_patterns(pillars: &FourPillars, lints: &mut Vec<SajuLint>) {
        let analysis = pillars.spirit_markers();
        use crate::analysis::spirit_markers::SpiritMarker;
        
        if analysis.markers.iter().any(|m| m.marker == SpiritMarker::Baihu) {
             lints.push(SajuLint {
                code: "I303".to_string(),
                severity: LintSeverity::Info,
                message: "백호(白虎) 신살: 현대에는 강한 추진력과 전문직 카리스마로 발현될 수 있습니다.".to_string(),
                advice: "전통적 흉신 해석보다는 전문 역량을 강화하는 방향으로 활용하면 긍정적입니다.".to_string(),
            });
        }
    }
}
