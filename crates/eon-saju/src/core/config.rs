//! Saju analysis weights, thresholds, and configurations
//!
//! Struct-based configuration management for ML optimization and user tuning.

use serde::{Deserialize, Serialize};

fn default_true() -> bool {
    true
}

/// Global Analysis Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig {
    pub weights: WeightsConfig,
    pub strength: StrengthConfig,
    pub root: RootConfig,
    pub thermal: ThermalConfig,
    pub vm: VmConfig,
    #[serde(default = "default_true")]
    pub yin_stem_reverse: bool,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            weights: WeightsConfig::default(),
            strength: StrengthConfig::default(),
            root: RootConfig::default(),
            thermal: ThermalConfig::default(),
            vm: VmConfig::default(),
            yin_stem_reverse: true,
        }
    }
}

/// Position weights (110 points method)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeightsConfig {
    pub total_weight: f32,
    pub month_branch: f32,
    pub day_branch: f32,
    pub other_branch: f32,
    pub stem: f32,
}

impl Default for WeightsConfig {
    fn default() -> Self {
        Self {
            total_weight: 11.0,
            month_branch: 3.5,
            day_branch: 1.5,
            other_branch: 1.0,
            stem: 1.0,
        }
    }
}

/// Strength score calculation weights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrengthConfig {
    pub criteria_score: f32,
    pub deuk_se_threshold: f32,
    pub deuk_se_weight: f32,
    pub polarized_high: f32,
    pub polarized_low: f32,
}

impl Default for StrengthConfig {
    fn default() -> Self {
        Self {
            criteria_score: 25.0,
            deuk_se_threshold: 5.5,
            deuk_se_weight: 0.25,
            polarized_high: 80.0,
            polarized_low: 20.0,
        }
    }
}

/// Hidden stem grounding (root) weights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootConfig {
    pub main_root: f32,
    pub middle_root: f32,
    pub remain_root: f32,
    pub saryeong_bonus: f32,
    pub min_deuk_ji_score: f32,
}

impl Default for RootConfig {
    fn default() -> Self {
        Self {
            main_root: 1.0,
            middle_root: 0.6,
            remain_root: 0.3,
            saryeong_bonus: 1.2,
            min_deuk_ji_score: 3.0,
        }
    }
}

/// Climate balance index thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalConfig {
    pub extreme: i32,
    pub moderate: i32,
}

impl Default for ThermalConfig {
    fn default() -> Self {
        Self {
            extreme: 40,
            moderate: 25,
        }
    }
}

/// VM simulation parameters
///
/// Weights and penalties used by SajuVM lifecycle emulation.
/// Modifiable externally for ML optimization or user customization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmConfig {
    /// Baseline starting score (midpoint of 0-100 range)
    pub base_score: f32,

    /// Penalty for clashing favorable/useful god (negative)
    pub clash_bad: f32,
    /// Bonus for clashing unfavorable/hostile god (positive, storage release effect)
    pub clash_good: f32,
    /// Default penalty for neutral clash
    pub clash_default: f32,

    // Pipeline weights (baseline influence of each luck level)
    /// Major Luck (10-year cycle)
    pub pipeline_major: f32,
    /// Yearly Luck (1-year cycle)
    pub pipeline_yearly: f32,
    /// Monthly Luck
    pub pipeline_monthly: f32,
    /// Daily Luck
    pub pipeline_daily: f32,
    /// Hourly Luck
    pub pipeline_hourly: f32,

    // Interrupt penalties (system interrupts triggered by inauspicious spirits)
    /// 0x01: High-priority system clash (Baihu, etc.)
    pub irq_critical: f32,
    /// 0x02: Resource overflow (Kuigang, etc.)
    pub irq_overflow: f32,
    /// 0x03: System freeze/isolation (Guxin/Guasu, etc.)
    pub irq_stall: f32,
    /// 0x04: Temporary service interruption (Wangshin/JieSha, etc.)
    pub irq_service: f32,

    // Pipeline control parameters
    /// Stall efficiency multiplier when conflicting with previous stage (0.0-1.0)
    pub stall_penalty: f32,
    /// Forwarding efficiency boost multiplier when transformed energy is favorable (1.0+)
    pub forwarding_bonus: f32,
    /// Hidden stem memory dump influence factor released by clash (0.0-1.0)
    pub memory_dump_weight: f32,
    /// Energy decay factor over time progression (0.0-1.0)
    pub decay_factor: f32,
}

impl Default for VmConfig {
    fn default() -> Self {
        Self {
            base_score: 50.0,
            clash_bad: -20.0,
            clash_good: 10.0,
            clash_default: -5.0,

            pipeline_major: 10.0,
            pipeline_yearly: 15.0,
            pipeline_monthly: 5.0,
            pipeline_daily: 2.0,
            pipeline_hourly: 1.0,

            irq_critical: 20.0,
            irq_overflow: 15.0,
            irq_stall: 10.0,
            irq_service: 7.0,

            stall_penalty: 0.5,
            forwarding_bonus: 1.2,
            memory_dump_weight: 0.3,
            decay_factor: 0.95,
        }
    }
}
