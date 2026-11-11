//! Destiny Fuzzer: Vulnerability analysis engine
//!
//! Applies binary fuzzing methodologies to four pillars calculations,
//! identifying destructive edge-case crash vectors across luck cycles.

use crate::core::ganzi::GanZi;
use crate::engine::vm::{LifeFrame, SajuVM};
use serde::{Deserialize, Serialize};

/// Vulnerability report
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VulnerabilityReport {
    /// Total detected crash count
    pub total_crashes: usize,
    /// Top critical vulnerability vectors
    pub critical_vectors: Vec<Vulnerability>,
}

/// Individual crash vulnerability
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vulnerability {
    /// Energy score at time of crash
    pub crash_score: f32,
    /// Root cause reproduction vector
    pub vector: LuckVector,
    /// Vulnerability classification
    pub vulnerability_type: String,
    /// Key payloads (trace tags)
    pub tags: Vec<String>,
    /// Timestamp for reverse analysis (optional)
    pub timestamp: Option<String>,
}

/// Input vector for vulnerability reproduction
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LuckVector {
    pub major: GanZi,           // Major luck (Process Context)
    pub yearly: GanZi,          // Yearly luck (Yearly Instruction)
    pub monthly: Option<GanZi>, // Monthly luck (Monthly Segment)
    pub daily: Option<GanZi>,   // Daily luck (Daily Trace)
    pub hourly: Option<GanZi>,  // Hourly luck (Hourly Offset)
}

pub struct DestinyFuzzer {
    pub vm: SajuVM,
    /// Score threshold considered as a crash (default 20.0)
    pub crash_threshold: f32,
}

impl DestinyFuzzer {
    pub fn new(vm: SajuVM) -> Self {
        Self {
            vm,
            crash_threshold: 20.0,
        }
    }

    /// Systematic exploration across all 60 sexagenary yearly luck cycles within major luck context
    pub fn audit(&self, major_context: GanZi) -> VulnerabilityReport {
        let mut vulnerabilities = Vec::new();

        for i in 0..60 {
            let yearly = GanZi::from_index(i);
            let frame = self.vm.step(0, major_context, yearly, None, None, None);

            if frame.score <= self.crash_threshold {
                vulnerabilities.push(Vulnerability {
                    crash_score: frame.score,
                    vector: LuckVector {
                        major: major_context,
                        yearly,
                        monthly: None,
                        daily: None,
                        hourly: None,
                    },
                    vulnerability_type: self.determine_vuln_type(&frame),
                    tags: frame.tags_as_strings(),
                    timestamp: None,
                });
            }
        }

        // Sort by lowest score ascending
        vulnerabilities.sort_by(|a, b| {
            a.crash_score
                .partial_cmp(&b.crash_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        VulnerabilityReport {
            total_crashes: vulnerabilities.len(),
            critical_vectors: vulnerabilities.into_iter().take(5).collect(),
        }
    }

    /// Random brute-force fuzzing across randomized major, yearly, and monthly luck cycles
    #[cfg(feature = "parallel")]
    pub fn fuzz_random(&self, iterations: usize) -> VulnerabilityReport {
        let mut vulnerabilities = Vec::new();
        use rand::Rng;
        let mut rng = rand::thread_rng();

        for _ in 0..iterations {
            let major = GanZi::from_index(rng.gen_range(0..60));
            let yearly = GanZi::from_index(rng.gen_range(0..60));
            let monthly = GanZi::from_index(rng.gen_range(0..60));

            let frame = self.vm.step(0, major, yearly, Some(monthly), None, None);

            if frame.score <= self.crash_threshold {
                vulnerabilities.push(Vulnerability {
                    crash_score: frame.score,
                    vector: LuckVector {
                        major,
                        yearly,
                        monthly: Some(monthly),
                        daily: None,
                        hourly: None,
                    },
                    vulnerability_type: self.determine_vuln_type(&frame),
                    tags: frame.tags_as_strings(),
                    timestamp: None,
                });
            }
        }

        vulnerabilities.sort_by(|a, b| {
            a.crash_score
                .partial_cmp(&b.crash_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        vulnerabilities.dedup_by(|a, b| {
            a.vector.major == b.vector.major && a.vector.yearly == b.vector.yearly
        });

        VulnerabilityReport {
            total_crashes: vulnerabilities.len(),
            critical_vectors: vulnerabilities.into_iter().take(5).collect(),
        }
    }

    /// Full Spectrum Timeline Audit
    /// Exhaustive fuzzing across 100-year high-resolution time slots
    pub fn audit_high_res(
        &self,
        birth_year: i32,
        major_luck: &crate::analysis::major_luck::MajorLuckAnalysis,
    ) -> VulnerabilityReport {
        let mut vulnerabilities = Vec::new();

        // 100-year exhaustive audit
        for age in 0..100 {
            let year = birth_year + age as i32;
            let yearly_ganzi = GanZi::from_year(year);
            let major_context = major_luck
                .at_age(age)
                .map(|m| m.ganzi)
                .unwrap_or(yearly_ganzi);

            for month in 1..=12 {
                let monthly_ganzi = self.calculate_month_ganzi(year, month);

                // Sampling: 1st, 11th, 21st days of each month
                for day in [1, 11, 21] {
                    let daily_ganzi = self.calculate_day_ganzi(year, month as u32, day);

                    for hour_idx in 0..12 {
                        let hour = hour_idx * 2;
                        let hourly_ganzi = self.calculate_hour_ganzi(daily_ganzi, hour);

                        let frame = self.vm.step(
                            age,
                            major_context,
                            yearly_ganzi,
                            Some(monthly_ganzi),
                            Some(daily_ganzi),
                            Some(hourly_ganzi),
                        );

                        if frame.score <= self.crash_threshold {
                            vulnerabilities.push(Vulnerability {
                                crash_score: frame.score,
                                vector: LuckVector {
                                    major: major_context,
                                    yearly: yearly_ganzi,
                                    monthly: Some(monthly_ganzi),
                                    daily: Some(daily_ganzi),
                                    hourly: Some(hourly_ganzi),
                                },
                                vulnerability_type: self.determine_vuln_type(&frame),
                                tags: frame.tags_as_strings(),
                                timestamp: Some(format!(
                                    "{}-{:02}-{:02} {:02}:00",
                                    year, month, day, hour
                                )),
                            });
                        }
                    }
                }
            }
        }

        vulnerabilities.sort_by(|a, b| {
            a.crash_score
                .partial_cmp(&b.crash_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        VulnerabilityReport {
            total_crashes: vulnerabilities.len(),
            critical_vectors: vulnerabilities.into_iter().take(10).collect(),
        }
    }

    // === Helper methods delegating to core::ganzi_utils ===

    fn calculate_month_ganzi(&self, year: i32, month: i32) -> GanZi {
        crate::core::ganzi_utils::calculate_month_ganzi(year, month)
    }

    fn calculate_day_ganzi(&self, year: i32, month: u32, day: u32) -> GanZi {
        // Calculate exact day GanZi using core utility
        crate::core::ganzi_utils::calculate_day_ganzi(year, month, day)
    }

    fn calculate_hour_ganzi(&self, daily_ganzi: GanZi, hour: u32) -> GanZi {
        crate::core::ganzi_utils::calculate_hour_ganzi(daily_ganzi, hour)
    }

    /// Vulnerability classification heuristics
    fn determine_vuln_type(&self, frame: &LifeFrame) -> String {
        if frame
            .tags
            .iter()
            .any(|t| t.contains_pattern("Clash") && t.contains_pattern("CoreLuck"))
        {
            "Critical_Yongshin_Clash".to_string()
        } else if frame.tags.iter().any(|t| t.contains_pattern("BadLuck")) {
            "Elemental_Overflow".to_string()
        } else if frame.tags.iter().any(|t| t.contains_pattern("Clash")) {
            "Structural_Instability".to_string()
        } else {
            "Low_Energy_State".to_string()
        }
    }
}

impl std::fmt::Display for Vulnerability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[!] CRASH at Score {:.1} | Type: {} | Vector: ({}, {}) | Tags: {:?}",
            self.crash_score,
            self.vulnerability_type,
            self.vector.major,
            self.vector.yearly,
            self.tags
        )
    }
}
