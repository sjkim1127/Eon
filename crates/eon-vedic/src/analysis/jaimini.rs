use crate::chart::VedicChart;
use crate::planets::VedicPlanet;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum JaiminiKarakaRole {
    Atmakaraka,    // AK - Soul
    Amatyakaraka,  // AmK - Career/Minister
    Bhratrukaraka, // BK - Siblings
    Matrukaraka,   // MK - Mother
    Pitrikaraka,   // PiK - Father (Used in 8-Karaka)
    Putrakaraka,   // PK - Children
    Gnatikaraka,   // GK - Rivals/Cousins
    Darakaraka,    // DK - Spouse
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KarakaAssignment {
    pub planet: VedicPlanet,
    pub role: JaiminiKarakaRole,
    pub degree_in_rasi: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArudhaPada {
    pub house: u8,    // 1~12
    pub rasi: u8,     // 1~12
    pub name: String, // e.g., "Arudha Lagna (AL)", "Dhanapada (A2)"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpecialLagna {
    pub name: String,
    pub longitude: f64,
    pub rasi: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignDashaSubPeriod {
    pub rasi: u8,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub duration_days: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignDashaPeriod {
    pub rasi: u8,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub years: u32,
    #[serde(default)]
    pub sub_periods: Vec<SignDashaSubPeriod>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KarakamshaAnalysis {
    pub atmakaraka: VedicPlanet,
    pub karakamsha_rasi: u8,
    pub ishta_devata_planet: Option<VedicPlanet>,
    pub ishta_devata_deity: String,
    pub spiritual_summary: String,
    pub career_talent_summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArudhaAnalysisInfo {
    pub house: u8,
    pub rasi: u8,
    pub name: String,
    pub interpretation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArgalaRasiInfo {
    pub rasi: u8,
    pub primary_argala_score: f64,
    pub virodhargala_score: f64,
    pub net_argala_score: f64,
    pub status: String,
    pub details: Vec<String>,
}

pub struct JaiminiEngine;

impl JaiminiEngine {
    /// Calculate 7 or 8 Chara Karakas
    pub fn calculate_karakas(chart: &VedicChart, use_8_karakas: bool) -> Vec<KarakaAssignment> {
        let mut planets_data: Vec<(VedicPlanet, f64)> = chart
            .planets
            .iter()
            .filter(|p| {
                let is_base = matches!(
                    p.planet,
                    VedicPlanet::Sun
                        | VedicPlanet::Moon
                        | VedicPlanet::Mars
                        | VedicPlanet::Mercury
                        | VedicPlanet::Jupiter
                        | VedicPlanet::Venus
                        | VedicPlanet::Saturn
                );
                if use_8_karakas {
                    is_base || p.planet == VedicPlanet::Rahu
                } else {
                    is_base
                }
            })
            .map(|p| {
                let mut deg = p.sidereal_deg % 30.0;
                // Special Rule for Rahu in Jaimini: Reverse degree because Node is retrograde
                if p.planet == VedicPlanet::Rahu {
                    deg = 30.0 - deg;
                }
                (p.planet, deg)
            })
            .collect();

        // Sort by degree within the Rasi (descending: AK first)
        planets_data.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let mut roles = vec![
            JaiminiKarakaRole::Atmakaraka,
            JaiminiKarakaRole::Amatyakaraka,
            JaiminiKarakaRole::Bhratrukaraka,
            JaiminiKarakaRole::Matrukaraka,
        ];

        if use_8_karakas {
            roles.push(JaiminiKarakaRole::Pitrikaraka);
        }

        roles.extend([
            JaiminiKarakaRole::Putrakaraka,
            JaiminiKarakaRole::Gnatikaraka,
            JaiminiKarakaRole::Darakaraka,
        ]);

        let mut assignments = Vec::new();
        for (idx, (planet, deg)) in planets_data.iter().enumerate() {
            if idx < roles.len() {
                assignments.push(KarakaAssignment {
                    planet: *planet,
                    role: roles[idx].clone(),
                    degree_in_rasi: *deg,
                });
            }
        }

        assignments
    }

    /// Calculate Arudha Padas for all 12 houses with interpretations
    pub fn calculate_arudha_padas(chart: &VedicChart) -> Vec<ArudhaPada> {
        let mut results = Vec::new();
        let lagna_rasi = chart.ascendant.rasi;

        let names = [
            "Arudha Lagna (AL)",
            "Dhanapada (A2)",
            "Vikramapada (A3)",
            "Matrupada (A4)",
            "Putrapada (A5)",
            "Shatrupada (A6)",
            "Darapada (A7)",
            "Mrityupada (A8)",
            "Bhagyapada (A9)",
            "Rajyapada (A10)",
            "Labhapada (A11)",
            "Upapada Lagna (UL/A12)",
        ];

        for house in 1..=12 {
            let house_rasi = ((lagna_rasi as i16 + house as i16 - 2) % 12 + 1) as u8;
            let lord = VedicPlanet::get_ruler_of(house_rasi);

            // Find lord's position in D1
            if let Some(lord_pos) = chart.planets.iter().find(|p| p.planet == lord) {
                let lord_rasi = lord_pos.rasi;

                // Distance from house to lord
                let dist = (lord_rasi as i16 - house_rasi as i16 + 12) % 12;

                // Arudha = Lord + Distance
                let mut arudha_rasi = ((lord_rasi as i16 + dist - 1) % 12 + 1) as u8;

                if arudha_rasi == house_rasi {
                    // If Arudha is in the house itself, final Arudha is 10th from house
                    arudha_rasi = ((house_rasi as i16 + 10 - 2) % 12 + 1) as u8;
                } else if arudha_rasi == ((house_rasi as i16 + 7 - 2) % 12 + 1) as u8 {
                    // If Arudha is in the 7th from house, final Arudha is 4th from house
                    arudha_rasi = ((house_rasi as i16 + 4 - 2) % 12 + 1) as u8;
                }

                results.push(ArudhaPada {
                    house,
                    rasi: arudha_rasi,
                    name: names[house as usize - 1].to_string(),
                });
            }
        }

        results
    }

    /// Calculate detailed Arudha Analysis including AL and UL interpretations
    pub fn analyze_arudha_padas(chart: &VedicChart) -> Vec<ArudhaAnalysisInfo> {
        let padas = Self::calculate_arudha_padas(chart);
        let rasi_names = [
            "양자리(Aries)", "황소자리(Taurus)", "쌍둥이자리(Gemini)", "게자리(Cancer)",
            "사자자리(Leo)", "처녀자리(Virgo)", "천칭자리(Libra)", "전갈자리(Scorpio)",
            "궁수자리(Sagittarius)", "염소자리(Capricorn)", "물병자리(Aquarius)", "물고기자리(Pisces)",
        ];

        padas
            .into_iter()
            .map(|p| {
                let r_name = rasi_names.get(p.rasi as usize - 1).unwrap_or(&"Unknown");
                let interp = match p.house {
                    1 => format!(
                        "아루다 라그나(AL)가 {}에 위치합니다. 대외적 이미지, 사회적 위상, 타인이 보는 본인의 신분과 명성을 주도합니다.",
                        r_name
                    ),
                    2 => format!("Dhanapada(A2)가 {}에 위치합니다. 물질적 자산, 가족 재정 흐름 및 일상의 수입원을 다룹니다.", r_name),
                    7 => format!("Darapada(A7)가 {}에 위치합니다. 대인 관계, 비즈니스 파트너십 및 사회적 교류 능력을 상징합니다.", r_name),
                    12 => format!(
                        "우파파다 라그나(UL/A12)가 {}에 위치합니다. 배우자의 성향, 결혼의 내실과 유지력, 배우자 가문과의 인연을 나타냅니다.",
                        r_name
                    ),
                    _ => format!("{}가 {}에 위치하여 해당 분야의 외형적 성과를 주도합니다.", p.name, r_name),
                };

                ArudhaAnalysisInfo {
                    house: p.house,
                    rasi: p.rasi,
                    name: p.name,
                    interpretation: interp,
                }
            })
            .collect()
    }

    /// Calculate Special Lagnas from BPHS
    pub fn calculate_special_lagnas(chart: &VedicChart) -> Vec<SpecialLagna> {
        let mut results = Vec::new();

        let sun_pos = chart.planets.iter().find(|p| p.planet == VedicPlanet::Sun);
        if let (Some(sun), sunrise) = (sun_pos, chart.panchanga.sunrise) {
            let birth_time = chart.panchanga.current_time;
            let diff_mins = birth_time.signed_duration_since(sunrise).num_minutes() as f64;

            // Bhava Lagna (BL): 1 Rasi (30 deg) per 24 mins (1 Ghati)
            let bl_long = (sun.sidereal_deg + diff_mins * (30.0 / 24.0)) % 360.0;
            results.push(SpecialLagna {
                name: "Bhava Lagna (BL)".to_string(),
                longitude: bl_long,
                rasi: (bl_long / 30.0).floor() as u8 + 1,
            });

            // Hora Lagna (HL): 1 Rasi (30 deg) per 60 mins (2.5 Ghati)
            let hl_long = (sun.sidereal_deg + diff_mins * (30.0 / 60.0)) % 360.0;
            results.push(SpecialLagna {
                name: "Hora Lagna (HL)".to_string(),
                longitude: hl_long,
                rasi: (hl_long / 30.0).floor() as u8 + 1,
            });

            // Ghati Lagna (GL): Rate of 1.25 signs per Ghati (24 mins)
            let gl_long = (sun.sidereal_deg + diff_mins * 1.5625) % 360.0;
            results.push(SpecialLagna {
                name: "Ghati Lagna (GL)".to_string(),
                longitude: gl_long,
                rasi: (gl_long / 30.0).floor() as u8 + 1,
            });
        }

        results
    }

    /// Calculate Chara Dasha with 2-tier Antardasha sub-periods (True KN Rao method)
    pub fn calculate_chara_dasha(chart: &VedicChart) -> Vec<SignDashaPeriod> {
        let lagna_rasi = chart.ascendant.rasi;
        let birth_time = chart.panchanga.current_time;

        let forward_signs = [1, 5, 6, 7, 11, 12];
        let is_forward_seq = forward_signs.contains(&lagna_rasi);

        let mut sequence = Vec::new();
        for i in 0..12 {
            let rasi = if is_forward_seq {
                ((lagna_rasi as i16 + i - 1) % 12 + 1) as u8
            } else {
                ((lagna_rasi as i16 - i + 11) % 12 + 1) as u8
            };
            sequence.push(rasi);
        }

        let mut timeline = Vec::new();
        let mut current_start = birth_time;

        for rasi in sequence {
            let years = Self::calculate_chara_dasha_years(chart, rasi);
            let duration_seconds = years as f64 * 365.2425 * 86400.0;
            let end_time = current_start + Duration::seconds(duration_seconds as i64);

            // Compute 12 Antardasha Sub-Periods
            let sub_duration_seconds = duration_seconds / 12.0;
            let is_sub_forward = forward_signs.contains(&rasi);
            let mut sub_periods = Vec::new();
            let mut sub_start = current_start;

            for j in 0..12 {
                let sub_rasi = if is_sub_forward {
                    ((rasi as i16 + j - 1) % 12 + 1) as u8
                } else {
                    ((rasi as i16 - j + 11) % 12 + 1) as u8
                };
                let sub_end = sub_start + Duration::seconds(sub_duration_seconds as i64);
                sub_periods.push(SignDashaSubPeriod {
                    rasi: sub_rasi,
                    start_time: sub_start,
                    end_time: sub_end,
                    duration_days: sub_duration_seconds / 86400.0,
                });
                sub_start = sub_end;
            }

            timeline.push(SignDashaPeriod {
                rasi,
                start_time: current_start,
                end_time,
                years,
                sub_periods,
            });

            current_start = end_time;
        }

        timeline
    }

    /// Calculate years for a sign in Chara Dasha
    fn calculate_chara_dasha_years(chart: &VedicChart, rasi: u8) -> u32 {
        let forward_signs = [1, 5, 6, 7, 11, 12];
        let is_counting_forward = forward_signs.contains(&rasi);

        let lord_rasi = match rasi {
            8 => Self::evaluate_co_ruler_strength(chart, 8, VedicPlanet::Mars, VedicPlanet::Ketu),
            11 => {
                Self::evaluate_co_ruler_strength(chart, 11, VedicPlanet::Saturn, VedicPlanet::Rahu)
            }
            _ => {
                let lord = VedicPlanet::get_ruler_of(rasi);
                chart
                    .planets
                    .iter()
                    .find(|p| p.planet == lord)
                    .map(|p| p.rasi)
                    .unwrap_or(rasi)
            }
        };

        let dist = if is_counting_forward {
            (lord_rasi as i16 - rasi as i16 + 12) % 12
        } else {
            (rasi as i16 - lord_rasi as i16 + 12) % 12
        };

        if dist == 0 {
            12
        } else {
            dist as u32
        }
    }

    /// Selection rules for Scorpio/Aquarius co-rulers (Jaimini / KN Rao)
    fn evaluate_co_ruler_strength(
        chart: &VedicChart,
        rasi: u8,
        p1: VedicPlanet,
        p2: VedicPlanet,
    ) -> u8 {
        let pos1 = match chart.planets.iter().find(|p| p.planet == p1) {
            Some(p) => p,
            None => return 1,
        };
        let pos2 = match chart.planets.iter().find(|p| p.planet == p2) {
            Some(p) => p,
            None => return 2,
        };

        let count_conj = |p_rasi: u8, planet: VedicPlanet| {
            chart
                .planets
                .iter()
                .filter(|p| p.rasi == p_rasi && p.planet != planet)
                .count()
        };

        let c1 = count_conj(pos1.rasi, p1);
        let c2 = count_conj(pos2.rasi, p2);

        if c1 > c2 {
            return pos1.rasi;
        }
        if c2 > c1 {
            return pos2.rasi;
        }

        let is_stronger = |p: VedicPlanet, prasi: u8| {
            if prasi == rasi {
                2
            } else if prasi == p.exaltation_rasi() {
                3
            } else {
                1
            }
        };

        let s1 = is_stronger(p1, pos1.rasi);
        let s2 = is_stronger(p2, pos2.rasi);

        if s1 > s2 {
            return pos1.rasi;
        }
        if s2 > s1 {
            return pos2.rasi;
        }

        if pos1.sidereal_deg % 30.0 > pos2.sidereal_deg % 30.0 {
            pos1.rasi
        } else {
            pos2.rasi
        }
    }

    /// Jaimini Rashi Drishti (Sign Aspects)
    pub fn get_rashi_drishti(rasi: u8) -> Vec<u8> {
        match rasi {
            1 => vec![5, 8, 11],
            2 => vec![4, 7, 10],
            3 => vec![6, 9, 12],
            4 => vec![2, 8, 11],
            5 => vec![1, 7, 10],
            6 => vec![3, 9, 12],
            7 => vec![2, 5, 11],
            8 => vec![1, 4, 10],
            9 => vec![3, 6, 12],
            10 => vec![2, 5, 8],
            11 => vec![1, 4, 7],
            12 => vec![3, 6, 9],
            _ => vec![],
        }
    }

    /// Calculate Karakamsha & Ishta Devata Spiritual Focus
    pub fn analyze_karakamsha(chart: &VedicChart) -> KarakamshaAnalysis {
        let karakas = Self::calculate_karakas(chart, false);
        let ak = karakas
            .iter()
            .find(|k| matches!(k.role, JaiminiKarakaRole::Atmakaraka))
            .map(|k| k.planet)
            .unwrap_or(VedicPlanet::Sun);

        // Find AK in D9 Navamsha rasi
        let ak_pos = chart.planets.iter().find(|p| p.planet == ak);
        let karakamsha_rasi = ak_pos.map(|p| p.navamsa_rasi).unwrap_or(1);

        // 12th from Karakamsha in D9 is the Ishta Devata house
        let ishta_house_rasi = ((karakamsha_rasi as i16 + 12 - 2) % 12 + 1) as u8;
        let ishta_lord = VedicPlanet::get_ruler_of(ishta_house_rasi);

        let ishta_deity = match ishta_lord {
            VedicPlanet::Sun => "비슈누 / 시바 (Shiva / Vishnu - 빛과 권능의 수호신)",
            VedicPlanet::Moon => "가우리 / 사라스바티 (Gouri / Saraswati - 지혜와 번영의 수호신)",
            VedicPlanet::Mars => "카르티케야 / 나라심하 (Subrahmanya / Narasimha - 용맹과 보호의 신)",
            VedicPlanet::Mercury => "비슈누 / 부다 (Narayana - 지성, 지혜, 학문의 수호신)",
            VedicPlanet::Jupiter => "바마나 / 스승 (Vamana / Dakshinamurthy - 영적 스승, 자비의 신)",
            VedicPlanet::Venus => "락슈미 / 파르바티 (Lakshmi / Parvati - 풍요, 아름다움, 조화의 신)",
            VedicPlanet::Saturn => "나레이나 / 하누만 (Kurma / Hanuman - 인내, 구원, 시련 극복의 신)",
            VedicPlanet::Rahu => "두르가 / 바라하 (Durga - 악을 물리치는 권능의 신)",
            VedicPlanet::Ketu => "가네샤 (Ganesha - 장애를 제거하고 깨달음을 주는 신)",
            _ => "영적 원천 (Universal Source)",
        }
        .to_string();

        let rasi_names = [
            "양자리(Aries)", "황소자리(Taurus)", "쌍둥이자리(Gemini)", "게자리(Cancer)",
            "사자자리(Leo)", "처녀자리(Virgo)", "천칭자리(Libra)", "전갈자리(Scorpio)",
            "궁수자리(Sagittarius)", "염소자리(Capricorn)", "물병자리(Aquarius)", "물고기자리(Pisces)",
        ];
        let r_name = rasi_names.get(karakamsha_rasi as usize - 1).unwrap_or(&"Unknown");

        let spiritual_summary = format!(
            "아트마카라카(AK: {:?})가 D9 나밤샤의 {}에 위치하여 카라캄샤를 형성합니다. Ishta Devata(영적 수호신)는 {}이며, 영혼의 깨달음과 진정한 라이프 사명을 부여합니다.",
            ak, r_name, ishta_deity
        );

        let career_talent_summary = format!(
            "카라캄샤 {:?}의 영향으로 지적 미학, 리더십, 철학적 탐구 및 깊은 분석력이 강화되어 인생의 핵심 소명을 실현합니다.",
            ak
        );

        KarakamshaAnalysis {
            atmakaraka: ak,
            karakamsha_rasi,
            ishta_devata_planet: Some(ishta_lord),
            ishta_devata_deity: ishta_deity,
            spiritual_summary,
            career_talent_summary,
        }
    }

    /// Calculate Argala (Intervention) & Virodhargala (Obstruction) Matrix for all 12 Rasis
    pub fn analyze_argala(chart: &VedicChart) -> Vec<ArgalaRasiInfo> {
        let mut results = Vec::new();

        for rasi in 1..=12 {
            // Count planet presence in Argala & Virodhargala positions
            let count_planets = |dist: u8| {
                let target_rasi = ((rasi as i16 + dist as i16 - 2) % 12 + 1) as u8;
                chart.planets.iter().filter(|p| p.rasi == target_rasi).count() as f64
            };

            // Primary Argala: 2nd, 4th, 11th from Rasi
            let p2 = count_planets(2);
            let p4 = count_planets(4);
            let p11 = count_planets(11);
            let primary_score = p2 * 1.5 + p4 * 1.0 + p11 * 2.0;

            // Virodhargala (Obstructions): 12th obstructs 2nd, 10th obstructs 4th, 3rd obstructs 11th
            let v12 = count_planets(12);
            let v10 = count_planets(10);
            let v3 = count_planets(3);
            let virodha_score = v12 * 1.5 + v10 * 1.0 + v3 * 2.0;

            // Secondary Argala: 5th (Obstructed by 9th)
            let p5 = count_planets(5);
            let v9 = count_planets(9);

            let net_score = (primary_score + p5 * 1.0) - (virodha_score + v9 * 1.0);

            let status = if net_score > 1.0 {
                "강한 지원 (Strong Argala)"
            } else if net_score < -1.0 {
                "방해 우세 (Obstructed Virodhargala)"
            } else {
                "중립/균형 (Balanced)"
            }
            .to_string();

            let mut details = Vec::new();
            if primary_score > 0.0 {
                details.push(format!("Primary Argala (+{:.1})", primary_score));
            }
            if virodha_score > 0.0 {
                details.push(format!("Virodhargala (-{:.1})", virodha_score));
            }

            results.push(ArgalaRasiInfo {
                rasi,
                primary_argala_score: primary_score + p5,
                virodhargala_score: virodha_score + v9,
                net_argala_score: net_score,
                status,
                details,
            });
        }

        results
    }

    /// Calculate Argala (Intervention) for a sign/planet position
    pub fn get_argala(rasi: u8) -> Vec<(u8, String)> {
        let mut results = Vec::new();
        let primary = [(2, "Wealth/Speech"), (4, "Happiness/Home"), (11, "Gains")];
        for (dist, desc) in primary {
            let target = ((rasi as i16 + dist - 2) % 12 + 1) as u8;
            results.push((target, format!("Primary Argala ({})", desc)));
        }
        let secondary = (5, "Knowledge/Children");
        let target = ((rasi as i16 + secondary.0 - 2) % 12 + 1) as u8;
        results.push((target, format!("Secondary Argala ({})", secondary.1)));
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::chart::VedicChart;
    use crate::core::chart::VedicPosition;
    use crate::core::planets::VedicPlanet;
    use chrono::{TimeZone, Utc};

    fn mock_position(planet: VedicPlanet, rasi: u8, deg: f64) -> VedicPosition {
        VedicPosition {
            planet,
            rasi,
            sidereal_deg: (rasi as f64 - 1.0) * 30.0 + deg,
            tropical_deg: 0.0,
            nakshatra: 1,
            pada: 1,
            house_index: 1,
            speed: 1.0,
            is_retrograde: false,
            is_combust: false,
            declination: 0.0,
            hora_rasi: 1,
            drekkana_rasi: 1,
            chaturthamsha_rasi: 1,
            panchamsa_rasi: 1,
            saptamsa_rasi: 1,
            ashtamsa_rasi: 1,
            navamsa_rasi: 1,
            dasamsa_rasi: 1,
            shashtamsa_rasi: 1,
            rudramsa_rasi: 1,
            dwadasamsa_rasi: 1,
            shodashamsa_rasi: 1,
            vimsamsa_rasi: 1,
            chaturvimshamsa_rasi: 1,
            saptavimsamsa_rasi: 1,
            trimsamsa_rasi: 1,
            khavedamsa_rasi: 1,
            akshavedamsa_rasi: 1,
            shashtyamsa_rasi: 1,
            navanavamsa_rasi: 1,
            ashtottaramsa_rasi: 1,
            dwadasdwadasamsa_rasi: 1,
        }
    }

    fn mock_chart(planets: Vec<VedicPosition>, lagna_rasi: u8) -> VedicChart {
        let ascendant = mock_position(VedicPlanet::Ascendant, lagna_rasi, 10.0);
        VedicChart {
            ascendant,
            planets,
            aspects: vec![],
            sav: crate::analysis::ashtakavarga::Sarvashtakavarga { points: [0u8; 12] },
            bav: vec![],
            house_cusps: vec![0.0; 12],
            karakas: vec![],
            arudha_padas: vec![],
            special_lagnas: vec![],
            bhava_strengths: vec![],
            vimshopaka_scores: vec![],
            avasthas: vec![],
            shadbalas: vec![],
            panchanga: crate::calc::panchanga::Panchanga {
                current_time: Utc.with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap(),
                ..Default::default()
            },
            analysis_report: None,
            ayanamsa: 23.0,
        }
    }

    #[test]
    fn test_chara_dasha_years_basic() {
        // Aries (1) lord Mars in Gemini (3)
        // 1 is forward sign. Dist: (3 - 1) = 2. Years: 2 - 1 = 1 year?
        // Wait, my impl says (dist == 0 ? 12 : dist).
        // 3-1 = 2. dist = 2.
        let planets = vec![mock_position(VedicPlanet::Mars, 3, 5.0)];
        let chart = mock_chart(planets, 1);
        let years = JaiminiEngine::calculate_chara_dasha_years(&chart, 1);
        assert_eq!(years, 2); // 1st to 3rd is 3 signs, but Rao uses diff.
                              // Actually dist is sign index difference. 1 to 3 is 2.
                              // If dist=2, years=2.
    }

    #[test]
    fn test_scorpio_co_ruler_strength() {
        // Scorpio (8) lords Mars (4) and Ketu (12)
        // Put Ketu with Sun in 12. Ketu has 1 conjunction. Mars is alone in 4.
        // Ketu should win.
        let planets = vec![
            mock_position(VedicPlanet::Mars, 4, 10.0),
            mock_position(VedicPlanet::Ketu, 12, 15.0),
            mock_position(VedicPlanet::Sun, 12, 20.0),
        ];
        let chart = mock_chart(planets, 1);
        let lord_rasi = JaiminiEngine::evaluate_co_ruler_strength(
            &chart,
            8,
            VedicPlanet::Mars,
            VedicPlanet::Ketu,
        );
        assert_eq!(lord_rasi, 12);
    }

    #[test]
    fn test_arudha_pada_7th_exception() {
        // Lagna (1) lord Mars in 4.
        // 4 is 4th from 1. 4th from 4 is 7.
        // This is the 7th house exception. Should move 4 houses from 7 -> 10.
        let planets = vec![mock_position(VedicPlanet::Mars, 4, 10.0)];
        let chart = mock_chart(planets, 1);
        let padas = JaiminiEngine::calculate_arudha_padas(&chart);
        let l1_arudha = padas.iter().find(|p| p.house == 1).unwrap();
        assert_eq!(l1_arudha.rasi, 4);
    }
}
