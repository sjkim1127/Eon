//! 공망(空亡, Void/Emptiness) 분석
//! 
//! 일주(日柱)를 기준으로 천간 10자와 지지 12자의 짝을 맞출 때 
//! 남게 되는 두 개의 지지를 분석합니다.

use serde::{Deserialize, Serialize};
use crate::core::branch::EarthlyBranch;
use crate::core::ganzi::GanZi;
use crate::core::pillars::FourPillars;
use crate::core::ten_gods::TenGod;

/// 공망 분석 결과
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoidAnalysis {
    /// 해당 사주의 공망 지지 (2개)
    pub void_branches: [EarthlyBranch; 2],
    /// 공망이 발생한 위치 (년, 월, 시)
    pub void_positions: Vec<String>,
    /// 공망된 십성 목록
    pub void_ten_gods: Vec<TenGod>,
    /// 공망 그룹 이름 (예: 갑자순)
    pub xun_group: String,
}

impl VoidAnalysis {
    /// 사주 팔자로부터 공망 분석
    pub fn from_pillars(pillars: &FourPillars) -> Self {
        let day_pillar = pillars.day;
        let (void_branches, xun_group) = calculate_void_branches(day_pillar);
        
        let mut void_positions = Vec::new();
        let mut void_ten_gods = Vec::new();
        
        let checks = [
            ("년주", pillars.year),
            ("월주", pillars.month),
            ("시주", pillars.hour),
        ];
        
        let dm = pillars.day_master();
        
        for (pos, ganzi) in checks {
            if void_branches.contains(&ganzi.branch) {
                void_positions.push(pos.to_string());
                void_ten_gods.push(TenGod::from_stem_and_branch(dm, ganzi.branch));
            }
        }
        
        Self {
            void_branches,
            void_positions,
            void_ten_gods,
            xun_group,
        }
    }
}

/// 특정 간지의 공망 지지와 순(旬) 그룹 산출
pub fn calculate_void_branches(ganzi: GanZi) -> ([EarthlyBranch; 2], String) {
    let s_idx = ganzi.stem.index() as i32;
    let b_idx = ganzi.branch.index() as i32;
    
    // 순(旬)의 시작점 계산
    let xun_start_idx = (b_idx - s_idx).rem_euclid(12);
    
    let xun_name = match xun_start_idx {
        0 => "갑자순(甲子旬)",
        10 => "갑술순(甲戌旬)",
        8 => "갑신순(甲申旬)",
        6 => "갑오순(甲午旬)",
        4 => "갑진순(甲辰旬)",
        2 => "갑인순(甲寅旬)",
        _ => "기타",
    };
    
    // 공망은 순의 시작점에서 2개 앞 (역순)
    let v1_idx = (xun_start_idx - 2).rem_euclid(12);
    let v2_idx = (xun_start_idx - 1).rem_euclid(12);
    
    let v1 = EarthlyBranch::from_index(v1_idx);
    let v2 = EarthlyBranch::from_index(v2_idx);
    
    ([v1, v2], xun_name.to_string())
}

impl std::fmt::Display for VoidAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "【공망(空亡) 분석】")?;
        writeln!(f, "─────────────────────────────────")?;
        writeln!(f, "일주 기준: {}", self.xun_group)?;
        writeln!(f, "공망 지지: {} ({}), {} ({})", 
            self.void_branches[0].hangul(), self.void_branches[0].hanja(),
            self.void_branches[1].hangul(), self.void_branches[1].hanja())?;
        
        if self.void_positions.is_empty() {
            writeln!(f, "▶ 원국(사주) 내에 공망이 없습니다.")?;
        } else {
            for (idx, pos) in self.void_positions.iter().enumerate() {
                writeln!(f, "▶ {}에 공망 발생 (십성: {})", pos, self.void_ten_gods[idx].hangul())?;
            }
            writeln!(f, "  * 해당 육친이나 사회적 기운의 실효성이 낮아질 수 있습니다.")?;
        }
        Ok(())
    }
}

impl FourPillars {
    /// 공망 분석
    pub fn void_analysis(&self) -> VoidAnalysis {
        VoidAnalysis::from_pillars(self)
    }
}
