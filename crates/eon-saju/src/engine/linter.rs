use crate::analysis::strength::StrengthType;
use crate::analysis::yongshin::YongshinAnalysis;
use crate::core::element::Element;
use crate::core::pillars::FourPillars;
use serde::{Deserialize, Serialize};

/// Linter diagnostic severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LintSeverity {
    Error,
    Warning,
    Info,
}

/// Saju lint diagnostic outcome
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

        // E404: Missing core yongshin
        Self::check_missing_dependency(&counts, &yongshin, &mut diagnostics);
        // E405: Extreme Day Master weakness
        Self::check_extreme_weakness(&strength.strength_type, &mut diagnostics);
        // W105: Elemental suppression
        Self::check_unused_elements(&counts, &mut diagnostics);
        // W202: Elemental overconcentration
        Self::check_infinite_loop(&counts, &mut diagnostics);
        // W203: Elemental deficiency (2+ missing elements)
        Self::check_missing_elements(&counts, &mut diagnostics);
        // Shinsal checks
        Self::check_legacy_patterns(pillars, &mut diagnostics);

        diagnostics
    }

    fn check_missing_dependency(
        counts: &[(Element, u32); 5],
        yongshin: &YongshinAnalysis,
        lints: &mut Vec<SajuLint>,
    ) {
        let primary_count = counts
            .iter()
            .find(|(el, _)| *el == yongshin.primary)
            .map(|(_, c)| *c)
            .unwrap_or(0);

        if primary_count == 0 {
            lints.push(SajuLint {
                code: "E404".to_string(),
                severity: LintSeverity::Error,
                message: format!("Missing core energy: required '{}' energy is absent from chart.", yongshin.primary.hangul()),
                advice: format!("Manifestation of core competence may be limited until '{}' arrives in luck cycles.", yongshin.primary.hangul()),
            });
        }
    }

    fn check_extreme_weakness(strength_type: &StrengthType, lints: &mut Vec<SajuLint>) {
        if *strength_type == StrengthType::Weak {
            lints.push(SajuLint {
                code: "E405".to_string(),
                severity: LintSeverity::Error,
                message: "Extremely weak Day Master: personal vitality is fragile and easily destabilized by external conditions.".to_string(),
                advice: "Leverage luck cycles bringing Resource/Companion energy, and maintain physical/mental stability.".to_string(),
            });
        }
    }

    fn check_missing_elements(counts: &[(Element, u32); 5], lints: &mut Vec<SajuLint>) {
        let missing: Vec<_> = counts
            .iter()
            .filter(|(_, c)| *c == 0)
            .map(|(el, _)| el.hangul())
            .collect();
        if missing.len() >= 2 {
            lints.push(SajuLint {
                code: "W203".to_string(),
                severity: LintSeverity::Warning,
                message: format!("Elemental deficiency: completely missing '{}' elements, resulting in biased circulation.", missing.join(", ")),
                advice: "Pursue stabilizing habits and key decisions during luck periods bringing the deficient elements.".to_string(),
            });
        }
    }

    fn check_unused_elements(counts: &[(Element, u32); 5], lints: &mut Vec<SajuLint>) {
        for (el, count) in counts.iter() {
            if *count > 0 {
                let controller = el.controlled_by();
                let controller_count = counts
                    .iter()
                    .find(|(e, _)| *e == controller)
                    .map(|(_, c)| *c)
                    .unwrap_or(0);

                if controller_count >= 3 {
                    lints.push(SajuLint {
                        code: "W105".to_string(),
                        severity: LintSeverity::Warning,
                        message: format!(
                            "Energy suppressed: '{}' is overwhelmed by strong '{}' energy.",
                            el.hangul(),
                            controller.hangul()
                        ),
                        advice: format!(
                            "Balancing elements are needed to modulate '{}' energy.",
                            controller.hangul()
                        ),
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
                    message: format!("Elemental overconcentration: '{}' energy is excessively dominant.", el.hangul()),
                    advice: "Overconcentration in a single element unbalances the overall flow. Balancing with complementary elements is advised.".to_string(),
                });
            }
        }
    }

    fn check_legacy_patterns(pillars: &FourPillars, lints: &mut Vec<SajuLint>) {
        let analysis = pillars.spirit_markers();
        use crate::analysis::spirit_markers::SpiritMarker;
        let markers = &analysis.markers;

        // I303: Baekho — Strong drive
        if markers.iter().any(|m| m.marker == SpiritMarker::Baihu) {
            lints.push(SajuLint {
                code: "I303".to_string(),
                severity: LintSeverity::Info,
                message:
                    "Baekho spirit: modern interpretation channels this into fierce drive and professional charisma."
                        .to_string(),
                advice:
                    "Rather than traditional inauspicious interpretations, channel this energy toward mastering specialized competencies."
                        .to_string(),
            });
        }

        // I304: Yeokma — Mobility, change, global pursuits
        if markers.iter().any(|m| m.marker == SpiritMarker::Yima) {
            lints.push(SajuLint {
                code: "I304".to_string(),
                severity: LintSeverity::Info,
                message: "Yeokma spirit: dynamic mobility, transition, and global pursuits.".to_string(),
                advice: "Embrace flexible movement and environmental change rather than static stagnation.".to_string(),
            });
        }

        // I305: Tianyi — Benefactor assistance
        if markers.iter().any(|m| m.marker == SpiritMarker::Tianyi) {
            lints.push(SajuLint {
                code: "I305".to_string(),
                severity: LintSeverity::Info,
                message: "Tianyi Noble: blessed with assistance from benefactors during adversity.".to_string(),
                advice: "Do not hesitate to seek counsel in difficult moments; unexpected helpers often emerge.".to_string(),
            });
        }

        // I306: Wenchang — Academics, examinations
        if markers.iter().any(|m| m.marker == SpiritMarker::Wenchang) {
            lints.push(SajuLint {
                code: "I306".to_string(),
                severity: LintSeverity::Info,
                message: "Wenchang Noble: distinguished talent in academics, writing, and intellectual mastery.".to_string(),
                advice: "Investing in intellectual pursuits and certifications yields strong dividends.".to_string(),
            });
        }

        // I307: Taohua — Charisma, attraction, popularity
        if markers.iter().any(|m| m.marker == SpiritMarker::Taohua) {
            lints.push(SajuLint {
                code: "I307".to_string(),
                severity: LintSeverity::Info,
                message: "Taohua spirit: personal charisma, public appeal, and romantic attraction.".to_string(),
                advice: "Channel appeal constructively into interpersonal leadership while minding emotional fatigue.".to_string(),
            });
        }

        // W204: Guchen / Guasu — Solitude
        let has_lonely = markers
            .iter()
            .any(|m| m.marker == SpiritMarker::Guchen || m.marker == SpiritMarker::Guasu);
        if has_lonely {
            lints.push(SajuLint {
                code: "W204".to_string(),
                severity: LintSeverity::Warning,
                message: "Solitary spirits (Guchen / Guasu): strong independence accompanied by propensity for isolation.".to_string(),
                advice: "While self-reliance is a strength, consciously cultivate meaningful social bonds.".to_string(),
            });
        }

        // W205: Jiesha — Resource / opportunity vulnerability
        if markers.iter().any(|m| m.marker == SpiritMarker::Jiesha) {
            lints.push(SajuLint {
                code: "W205".to_string(),
                severity: LintSeverity::Warning,
                message: "Jiesha spirit: heightened vulnerability to sudden resource or opportunity drain.".to_string(),
                advice: "Exercise rigorous diligence in contracts and financial commitments.".to_string(),
            });
        }

        // W206: Yuanzhen — Relationship friction
        if markers.iter().any(|m| m.marker == SpiritMarker::Yuanzhen) {
            lints.push(SajuLint {
                code: "W206".to_string(),
                severity: LintSeverity::Warning,
                message: "Yuanzhen spirit: emotional friction and latent misunderstandings in relationships.".to_string(),
                advice: "Address grievances promptly through transparent communication.".to_string(),
            });
        }

        // W207: Wangshen — Reputational risk
        if markers.iter().any(|m| m.marker == SpiritMarker::Wangshen) {
            lints.push(SajuLint {
                code: "W207".to_string(),
                severity: LintSeverity::Warning,
                message: "Wangshen spirit: potential exposure or missteps leading to reputational strain."
                    .to_string(),
                advice: "Exercise discretion in speech, conduct, and confidential disclosures."
                    .to_string(),
            });
        }
    }
}
