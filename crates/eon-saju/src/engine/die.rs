//! Destiny It Easy (DIE) Engine
//! Computes entropy of Four Pillars and detects packed energy states.

use crate::analysis::relationships::RelationshipAnalysis;
use crate::core::element::Element;
use crate::core::pillars::FourPillars;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DieAnalysis {
    /// Shannon entropy (0.0 ~ 2.32)
    /// Higher indicates dispersed energy; lower indicates concentrated energy
    pub entropy: f32,

    /// Packed status (Is Packed?)
    pub is_packed: bool,
    pub packer_name: Option<String>, // e.g. "Heavenly Stem Combine Packer"

    /// Compiler info (Month branch season / structure)
    pub compiler_info: String,

    /// File type (Strong/Weak analogue)
    pub file_type: String,
}

pub struct DestinyItEasy;

impl DestinyItEasy {
    pub fn scan(pillars: &FourPillars) -> DieAnalysis {
        let counts = pillars.element_counts();
        let packer = Self::detect_packer(pillars);

        DieAnalysis {
            entropy: Self::calculate_entropy(&counts),
            is_packed: packer.is_some(),
            packer_name: packer,
            compiler_info: Self::detect_compiler(pillars),
            file_type: Self::detect_file_type(pillars),
        }
    }

    /// 1. Shannon Entropy calculation
    /// H(X) = -∑ P(x) * log2(P(x))
    fn calculate_entropy(counts: &[(Element, u32); 5]) -> f32 {
        let total: u32 = counts.iter().map(|(_, c)| c).sum();
        if total == 0 {
            return 0.0;
        }

        let mut entropy = 0.0;
        for (_, count) in counts {
            if *count > 0 {
                let p = *count as f32 / total as f32;
                entropy -= p * p.log2();
            }
        }
        entropy
    }

    /// 2. Packer detection (Dense combinations represent packed state)
    fn detect_packer(pillars: &FourPillars) -> Option<String> {
        let relations = RelationshipAnalysis::from_pillars(pillars);

        // Multiple stem or branch combinations indicate tightly packed energy
        let combine_count = relations.stem_combinations.len()
            + relations.six_combinations.len()
            + relations.triple_combinations.len()
            + relations.seasonal_combinations.len();

        if combine_count >= 2 {
            Some(format!(
                "Dense_Energy_Packer v{}.0 (Combines: {})",
                combine_count, combine_count
            ))
        } else if combine_count == 1 {
            Some("Simple_Bond_Packer v1.0".to_string())
        } else {
            None // Raw binary
        }
    }

    /// 3. Compiler detection (based on month branch)
    fn detect_compiler(pillars: &FourPillars) -> String {
        let month_branch = pillars.month.branch;
        let season = month_branch.season();

        format!("Built on {:?} Season ({:?} Core)", season, month_branch)
    }

    /// 4. File type (Strong / Weak analogue)
    fn detect_file_type(pillars: &FourPillars) -> String {
        let analysis = pillars.strength();

        match analysis.strength_type {
            crate::analysis::strength::StrengthType::Strong => {
                "Standalone Executable (Strong)".to_string()
            }
            crate::analysis::strength::StrengthType::Weak => "Dependent Library (Weak)".to_string(),
            crate::analysis::strength::StrengthType::Balanced => {
                "Shared Object (Balanced)".to_string()
            }
        }
    }
}

impl std::fmt::Display for DieAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[Destiny It Easy v3.01 Scan Result]")?;
        writeln!(f, "* File Type: {}", self.file_type)?;
        writeln!(f, "* Compiler : {}", self.compiler_info)?;
        writeln!(
            f,
            "* Entropy  : {:.2} ({})",
            self.entropy,
            if self.entropy > 2.0 { "High" } else { "Normal" }
        )?;

        if self.is_packed {
            writeln!(f, "* Packer   : [!] DETECTED")?;
            writeln!(f, "  └─ Info  : {}", self.packer_name.as_ref().unwrap())?;
            writeln!(
                f,
                "  └─ Status: Energy is tightly packed, which may delay manifest expression."
            )?;
        } else {
            writeln!(f, "* Packer   : None (Raw Binary)")?;
        }

        Ok(())
    }
}
