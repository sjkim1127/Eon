use crate::core::element::Element;
use crate::core::pillars::FourPillars;
use serde::{Deserialize, Serialize};

/// Life complexity / obfuscation level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObfuscationLevel {
    #[serde(rename = "Plaintext")]
    Plaintext, // Elements concentrated; path is clear
    #[serde(rename = "Standard")]
    Standard, // Evenly distributed and balanced
    #[serde(rename = "Packed")]
    Packed, // Diverse energies intertwined
    #[serde(rename = "Encrypted")]
    Encrypted, // Highly entangled; requires deep analysis
}

/// Entropy analysis outcome
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyAnalysis {
    pub score: f32, // Shannon Entropy Score (0.0 ~ 2.322)
    pub level: ObfuscationLevel,
    pub description: String,
    pub unpacker_element: Option<Element>, // Unpacker element resolving complexity
}

pub struct DestinyEntropy;

impl DestinyEntropy {
    pub fn analyze(pillars: &FourPillars) -> EntropyAnalysis {
        // 1. Collect all elements (stems + branches + hidden stems for higher data density)
        let mut element_counts = [0.0f32; 5];
        let mut total_elements = 0.0f32;

        // Base 8 characters
        for (el, count) in pillars.element_counts() {
            element_counts[el.index() as usize] += count as f32;
            total_elements += count as f32;
        }

        // Add hidden stems (Jijanggan) data
        for branch in [
            Some(&pillars.year.branch),
            Some(&pillars.month.branch),
            Some(&pillars.day.branch),
            pillars.hour.as_ref().map(|h| &h.branch),
        ]
        .into_iter()
        .flatten()
        {
            for stem in branch.jijanggan() {
                element_counts[stem.element().index() as usize] += 0.3;
                total_elements += 0.3;
            }
        }

        // 2. Compute Shannon Entropy: H = -sum(pi * log2(pi))
        let mut entropy = 0.0f32;
        for count in element_counts {
            if count > 0.0 {
                let p = count / total_elements;
                entropy -= p * p.log2();
            }
        }

        // 3. Classify level
        // Max entropy is log2(5) ≈ 2.32
        let (level, description) = if entropy < 1.0 {
            (ObfuscationLevel::Plaintext, "Five elements are concentrated in one domain, providing a clear path. Goals are distinct, though flexibility may be limited.".to_string())
        } else if entropy < 1.8 {
            (
                ObfuscationLevel::Standard,
                "Five elements are evenly distributed, conferring balanced life momentum."
                    .to_string(),
            )
        } else if entropy < 2.1 {
            (
                ObfuscationLevel::Packed,
                "Diverse energies are intricately bound, revealing their true character at critical junctures.".to_string(),
            )
        } else {
            (ObfuscationLevel::Encrypted, "Five elements are highly entangled, presenting complex life dynamics requiring in-depth analysis.".to_string())
        };

        // 4. Find unpacker element: primary yongshin resolving complexity
        let unpacker = pillars.yongshin().primary;

        EntropyAnalysis {
            score: entropy,
            level,
            description,
            unpacker_element: Some(unpacker),
        }
    }
}

impl std::fmt::Display for EntropyAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[Destiny Entropy & Obfuscation Analysis]")?;
        writeln!(f, "Entropy Score : {:.3} (DIE Standard)", self.score)?;

        let level_str = match self.level {
            ObfuscationLevel::Plaintext => "Plaintext (High transparency)",
            ObfuscationLevel::Standard => "Standard (Normal equilibrium)",
            ObfuscationLevel::Packed => "Packed (Compressed/Concealed)",
            ObfuscationLevel::Encrypted => "Encrypted (High complexity)",
        };
        writeln!(f, "Analysis Level: {}", level_str)?;
        writeln!(f, "Description   : {}", self.description)?;

        if let Some(up) = self.unpacker_element {
            writeln!(
                f,
                "Unpacker Hint : Unpacking is triggered when '{:?}' element energy enters the cycle.",
                up
            )?;
        }
        Ok(())
    }
}
