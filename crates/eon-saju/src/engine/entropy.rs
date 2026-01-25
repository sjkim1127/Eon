use serde::{Deserialize, Serialize};
use crate::core::element::Element;
use crate::core::pillars::FourPillars;

/// 인생 데이터 난독화 등급
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObfuscationLevel {
    Plaintext,     // 해석이 매우 직관적이고 명확함
    Standard,      // 일반적인 분석 난이도
    Packed,        // 본모습이 숨겨져 있어 특정 시점에 언패킹이 필요함
    Encrypted,     // 고도로 난독화되어 전문적 분석 없이는 파악 불가
}

/// 엔트로피 분석 결과
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyAnalysis {
    pub score: f32,                // Shannon Entropy Score (0.0 ~ 2.322)
    pub level: ObfuscationLevel,
    pub description: String,
    pub unpacker_element: Option<Element>, // 복잡성을 해소해줄 '언패커' 오행
}

pub struct DestinyEntropy;

impl DestinyEntropy {
    pub fn analyze(pillars: &FourPillars) -> EntropyAnalysis {
        // 1. 모든 요소 수집 (천간 + 지지 + 지장간 포함하여 데이터 밀도 높임)
        let mut element_counts = [0.0f32; 5];
        let mut total_elements = 0.0f32;

        // 원국 8글자 반영
        for (el, count) in pillars.element_counts() {
            element_counts[el.index() as usize] += count as f32;
            total_elements += count as f32;
        }

        // 지장간(Hidden Stems) 데이터 추가 반영하여 엔트로피 정밀도 향상
        // 지장간은 내부 로직(난독화 데이터)으로 간주
        for branch in [&pillars.year.branch, &pillars.month.branch, &pillars.day.branch, &pillars.hour.branch] {
            for stem in branch.jijanggan() {
                element_counts[stem.element().index() as usize] += 0.3;
                total_elements += 0.3;
            }
        }

        // 2. Shannon Entropy 계산: H = -sum(pi * log2(pi))
        let mut entropy = 0.0f32;
        for count in element_counts {
            if count > 0.0 {
                let p = count / total_elements;
                entropy -= p * p.log2();
            }
        }

        // 3. 등급 판별 (DIE 기준 차용)
        // 최대 엔트로피는 log2(5) ≈ 2.32
        let (level, description) = if entropy < 1.0 {
            (ObfuscationLevel::Plaintext, "데이터가 한곳에 집중되어 경로가 매우 선형적입니다. (Plaintext Life)".to_string())
        } else if entropy < 1.8 {
            (ObfuscationLevel::Standard, "표준적인 데이터 분산을 가집니다. (Standard Binary)".to_string())
        } else if entropy < 2.1 {
            (ObfuscationLevel::Packed, "내부 로직이 복잡하게 얽혀 있어 본모습이 은폐되어 있습니다. (Packed Destiny)".to_string())
        } else {
            (ObfuscationLevel::Encrypted, "고도의 난독화가 적용되어 해석이 매우 난해합니다. (Encrypted/Obfuscated)".to_string())
        };

        // 4. Unpacker 탐색: 가장 부족하거나 엔트로피를 낮춰줄(균형을 잡을) 핵심 오행
        // 실제로는 용신과 유사하나 '복잡도 해소' 관점에서 접근
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
        writeln!(f, "【Destiny Entropy & Obfuscation Analysis】")?;
        writeln!(f, "Entropy Score : {:.3} (DIE Standard)", self.score)?;
        
        let level_str = match self.level {
            ObfuscationLevel::Plaintext => "🟢 Plaintext (해석 용이)",
            ObfuscationLevel::Standard => "🔵 Standard (일반)",
            ObfuscationLevel::Packed => "🟡 Packed (압축/은폐)",
            ObfuscationLevel::Encrypted => "🔴 Encrypted (난독화/난해)",
        };
        writeln!(f, "Analysis Level: {}", level_str)?;
        writeln!(f, "Description   : {}", self.description)?;
        
        if let Some(up) = self.unpacker_element {
            writeln!(f, "Unpacker Hint : '{}' 에너지가 유입될 때 인생의 압축이 해제(Unpacking)됩니다.", up.hangul())?;
        }
        Ok(())
    }
}
