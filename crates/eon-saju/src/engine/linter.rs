use serde::{Deserialize, Serialize};
use crate::core::pillars::FourPillars;
use crate::core::element::Element;
use crate::analysis::yongshin::YongshinAnalysis;
use crate::analysis::strength::StrengthType;

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
        let strength = pillars.strength();
        let counts = pillars.element_counts();

        // E404: 핵심 용신 부재
        Self::check_missing_dependency(&counts, &yongshin, &mut diagnostics);
        // E405: 일간 극약 (4득 전무)
        Self::check_extreme_weakness(&strength.strength_type, &mut diagnostics);
        // W105: 오행 억제 (한 오행이 다른 오행을 압도)
        Self::check_unused_elements(&counts, &mut diagnostics);
        // W202: 오행 편중
        Self::check_infinite_loop(&counts, &mut diagnostics);
        // W203: 오행 결핍 2종 이상
        Self::check_missing_elements(&counts, &mut diagnostics);
        // 신살 체크
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

    fn check_extreme_weakness(strength_type: &StrengthType, lints: &mut Vec<SajuLint>) {
        if *strength_type == StrengthType::Weak {
            lints.push(SajuLint {
                code: "E405".to_string(),
                severity: LintSeverity::Error,
                message: "일간 극약: 자신의 기운이 매우 약해 외부 환경에 쉽게 흔들릴 수 있습니다.".to_string(),
                advice: "일간을 돕는 기운(인성·비겁)이 들어오는 대운·세운을 적극 활용하고, 평소 심신 안정에 집중하세요.".to_string(),
            });
        }
    }

    fn check_missing_elements(counts: &[(Element, u32); 5], lints: &mut Vec<SajuLint>) {
        let missing: Vec<_> = counts.iter().filter(|(_, c)| *c == 0).map(|(el, _)| el.hangul()).collect();
        if missing.len() >= 2 {
            lints.push(SajuLint {
                code: "W203".to_string(),
                severity: LintSeverity::Warning,
                message: format!("오행 결핍: '{}' 기운이 사주에 완전히 없어 흐름이 편향되어 있습니다.", missing.join("·")),
                advice: "부족한 기운이 들어오는 시기(대운·세운)에 중요한 결정을 내리거나 해당 기운을 보완하는 활동을 이어가세요.".to_string(),
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
        let markers = &analysis.markers;

        // I303: 백호살 — 강한 추진력
        if markers.iter().any(|m| m.marker == SpiritMarker::Baihu) {
            lints.push(SajuLint {
                code: "I303".to_string(),
                severity: LintSeverity::Info,
                message: "백호(白虎) 신살: 현대에는 강한 추진력과 전문직 카리스마로 발현될 수 있습니다.".to_string(),
                advice: "전통적 흉신 해석보다는 전문 역량을 강화하는 방향으로 활용하면 긍정적입니다.".to_string(),
            });
        }

        // I304: 역마살 — 이동·변화·해외
        if markers.iter().any(|m| m.marker == SpiritMarker::Yima) {
            lints.push(SajuLint {
                code: "I304".to_string(),
                severity: LintSeverity::Info,
                message: "역마(驛馬) 신살: 이동, 변화, 해외 활동에 관한 기운이 강합니다.".to_string(),
                advice: "잦은 환경 변화를 기회로 활용하면 좋습니다. 한 곳에 오래 머물기보다 유연하게 움직이는 삶이 맞습니다.".to_string(),
            });
        }

        // I305: 천을귀인 — 귀인 조력
        if markers.iter().any(|m| m.marker == SpiritMarker::Tianyi) {
            lints.push(SajuLint {
                code: "I305".to_string(),
                severity: LintSeverity::Info,
                message: "천을귀인(天乙貴人): 위기 상황에서 귀인의 도움을 받는 복이 있습니다.".to_string(),
                advice: "어려울 때 주변에 도움을 구하는 것을 두려워하지 마세요. 뜻밖의 조력자가 힘이 되어줄 수 있습니다.".to_string(),
            });
        }

        // I306: 문창귀인 — 학문·시험
        if markers.iter().any(|m| m.marker == SpiritMarker::Wenchang) {
            lints.push(SajuLint {
                code: "I306".to_string(),
                severity: LintSeverity::Info,
                message: "문창귀인(文昌貴人): 학문, 언어, 창작 분야에서 두각을 나타낼 수 있습니다.".to_string(),
                advice: "지적 탐구와 학습에 투자하면 큰 성과를 거둘 수 있습니다. 시험이나 자격증에도 유리합니다.".to_string(),
            });
        }

        // I307: 도화살 — 이성·매력·인기
        if markers.iter().any(|m| m.marker == SpiritMarker::Taohua) {
            lints.push(SajuLint {
                code: "I307".to_string(),
                severity: LintSeverity::Info,
                message: "도화(桃花) 신살: 외적 매력과 인기가 강조되는 기운입니다.".to_string(),
                advice: "대인 관계와 이성 운이 활발합니다. 매력을 긍정적으로 활용하되 감정 소모에 주의하세요.".to_string(),
            });
        }

        // W204: 고신살·과숙살 — 고독
        let has_lonely = markers.iter().any(|m| m.marker == SpiritMarker::Guchen || m.marker == SpiritMarker::Guasu);
        if has_lonely {
            lints.push(SajuLint {
                code: "W204".to_string(),
                severity: LintSeverity::Warning,
                message: "고독 신살(孤辰·寡宿): 독립적인 기운이 강해 관계에서 외로움을 느끼기 쉽습니다.".to_string(),
                advice: "자립심이 강한 것은 장점이지만, 의도적으로 신뢰하는 사람과의 유대를 만들어 가는 것이 중요합니다.".to_string(),
            });
        }

        // W205: 겁살 — 재물·기회 손실
        if markers.iter().any(|m| m.marker == SpiritMarker::Jiesha) {
            lints.push(SajuLint {
                code: "W205".to_string(),
                severity: LintSeverity::Warning,
                message: "겁살(劫煞): 재물이나 기회를 빼앗기는 상황이 생기기 쉽습니다.".to_string(),
                advice: "계약·투자·보증 등에서 꼼꼼히 확인하는 습관을 들이세요. 충동적인 금전 결정은 피하세요.".to_string(),
            });
        }

        // W206: 원진살 — 갈등·오해
        if markers.iter().any(|m| m.marker == SpiritMarker::Yuanzhen) {
            lints.push(SajuLint {
                code: "W206".to_string(),
                severity: LintSeverity::Warning,
                message: "원진살(怨嗔煞): 특정 관계에서 원한과 오해가 쌓이기 쉽습니다.".to_string(),
                advice: "오래된 갈등을 방치하지 말고 솔직한 대화로 풀어가세요. 감정이 쌓이면 관계가 멀어지기 쉽습니다.".to_string(),
            });
        }

        // W207: 망신살 — 명예 손실 위험
        if markers.iter().any(|m| m.marker == SpiritMarker::Wangshen) {
            lints.push(SajuLint {
                code: "W207".to_string(),
                severity: LintSeverity::Warning,
                message: "망신살(亡身煞): 실수나 노출로 인해 명예가 손상될 수 있는 기운입니다.".to_string(),
                advice: "언행에 신중을 기하고, 사생활이나 금전 관련 정보는 함부로 노출하지 마세요.".to_string(),
            });
        }
    }
}
