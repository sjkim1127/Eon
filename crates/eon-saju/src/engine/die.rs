//! Destiny It Easy (DIE) Engine
//! 사주의 엔트로피(복잡도)를 계산하고, 에너지가 묶인(Packed) 상태를 탐지합니다.

use serde::{Deserialize, Serialize};
use crate::core::pillars::FourPillars;
use crate::core::element::Element;
use crate::analysis::relationships::RelationshipAnalysis;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DieAnalysis {
    /// 섀넌 엔트로피 (0.0 ~ 2.32)
    /// 높을수록 에너지가 산만하고, 낮을수록 집중됨
    pub entropy: f32,
    
    /// 압축 상태 (Is Packed?)
    pub is_packed: bool,
    pub packer_name: Option<String>, // 예: "Heavenly Stem Combine Packer"
    
    /// 컴파일러 정보 (월령/격국)
    pub compiler_info: String,
    
    /// 파일 타입 (신강/신약)
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

    /// 1. 엔트로피 계산 (Shannon Entropy)
    /// H(X) = -∑ P(x) * log2(P(x))
    fn calculate_entropy(counts: &[(Element, u32); 5]) -> f32 {
        let total: u32 = counts.iter().map(|(_, c)| c).sum();
        if total == 0 { return 0.0; }

        let mut entropy = 0.0;
        for (_, count) in counts {
            if *count > 0 {
                let p = *count as f32 / total as f32;
                entropy -= p * p.log2();
            }
        }
        entropy
    }

    /// 2. 패커 탐지 (합이 많으면 Packed 상태)
    fn detect_packer(pillars: &FourPillars) -> Option<String> {
        let relations = RelationshipAnalysis::from_pillars(pillars);
        
        // 천간합이나 지지합(육합/삼합/방합)이 많으면 "강하게 압축됨"
        let combine_count = relations.stem_combinations.len() 
            + relations.six_combinations.len()
            + relations.triple_combinations.len()
            + relations.seasonal_combinations.len();
        
        if combine_count >= 2 {
            Some(format!("Dense_Energy_Packer v{}.0 (Combines: {})", combine_count, combine_count))
        } else if combine_count == 1 {
            Some("Simple_Bond_Packer v1.0".to_string())
        } else {
            None // 순정(Raw) 바이너리
        }
    }

    /// 3. 컴파일러 탐지 (월지 기준)
    fn detect_compiler(pillars: &FourPillars) -> String {
        let month_branch = pillars.month.branch;
        let season = month_branch.season();
        
        // 예: "Built on Winter Season (Juta Framework)"
        format!("Built on {:?} Season ({:?} Core)", season, month_branch)
    }

    /// 4. 파일 타입 (신강/신약 = EXE/DLL?)
    fn detect_file_type(pillars: &FourPillars) -> String {
        let analysis = pillars.strength();
        
        match analysis.strength_type {
            crate::analysis::strength::StrengthType::Strong => "Standalone Executable (신강)".to_string(),
            crate::analysis::strength::StrengthType::Weak => "Dependent Library (신약)".to_string(),
            crate::analysis::strength::StrengthType::Balanced => "Shared Object (중화)".to_string(),
        }
    }
}

impl std::fmt::Display for DieAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "【Destiny It Easy v3.01 Scan Result】")?;
        writeln!(f, "* File Type: {}", self.file_type)?;
        writeln!(f, "* Compiler : {}", self.compiler_info)?;
        writeln!(f, "* Entropy  : {:.2} ({})", self.entropy, if self.entropy > 2.0 { "High" } else { "Normal" })?;
        
        if self.is_packed {
            writeln!(f, "* Packer   : ⚠️ DETECTED!")?;
            writeln!(f, "  └─ Info  : {}", self.packer_name.as_ref().unwrap())?;
            writeln!(f, "  └─ Status: 에너지가 강력하게 압축(Packed)되어 분석 및 발현이 지연될 수 있습니다.")?;
        } else {
            writeln!(f, "* Packer   : None (Raw Binary)")?;
        }
        
        Ok(())
    }
}
