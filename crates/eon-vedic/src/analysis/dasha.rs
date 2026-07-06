use crate::planets::VedicPlanet;
use crate::core::chart::VedicChart;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DashaPeriod {
    pub lord: VedicPlanet,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub sub_dashas: Vec<DashaPeriod>,
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpretation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_favorable: Option<bool>,
}

pub struct VimshottariDasha;

impl VimshottariDasha {
    const CYCLE: [(VedicPlanet, f64); 9] = [
        (VedicPlanet::Ketu, 7.0),
        (VedicPlanet::Venus, 20.0),
        (VedicPlanet::Sun, 6.0),
        (VedicPlanet::Moon, 10.0),
        (VedicPlanet::Mars, 7.0),
        (VedicPlanet::Rahu, 18.0),
        (VedicPlanet::Jupiter, 16.0),
        (VedicPlanet::Saturn, 19.0),
        (VedicPlanet::Mercury, 17.0),
    ];

    const TOTAL_YEARS: f64 = 120.0;

    /// Calculate the full list of Mahadashas for a given birth time and Moon's longitude
    pub fn calculate_timeline(
        birth_time: DateTime<Utc>,
        moon_long: f64,
        levels: u8,
    ) -> Vec<DashaPeriod> {
        // Moon Nakshatra range is 13°20' (13.3333...)
        let nak_duration = 360.0 / 27.0;
        let nak_index_0 = (moon_long / nak_duration).floor() as usize;
        let start_lord_index = nak_index_0 % 9;

        // Calculate elapsed portion of the current Nakshatra
        let elapsed_in_nak = moon_long % nak_duration;
        let remaining_in_nak = nak_duration - elapsed_in_nak;
        let portion_remaining = remaining_in_nak / nak_duration;

        let mut timeline = Vec::new();
        let mut current_start = birth_time;

        // Initial Dasha balance
        let (first_lord, full_years) = Self::CYCLE[start_lord_index];
        let remaining_years = full_years * portion_remaining;

        // Approximate years to seconds (365.2425 days per year)
        let years_to_secs = |y: f64| (y * 365.2425 * 24.0 * 60.0 * 60.0) as i64;

        let first_end = current_start + Duration::seconds(years_to_secs(remaining_years));

        let mut first_dasha = DashaPeriod {
            lord: first_lord,
            start_time: current_start,
            end_time: first_end,
            sub_dashas: Vec::new(),
            name: None,
            interpretation: None,
            is_favorable: None,
        };

        if levels > 1 {
            first_dasha.sub_dashas = Self::calculate_sub_periods(
                first_lord,
                current_start,
                first_end,
                levels - 1,
                Some(portion_remaining),
            );
        }

        timeline.push(first_dasha);
        current_start = first_end;

        // Generate subsequent Mahadashas for the next 120 years
        for i in 1..9 {
            let idx = (start_lord_index + i) % 9;
            let (lord, years) = Self::CYCLE[idx];
            let end_time = current_start + Duration::seconds(years_to_secs(years));

            let mut dasha = DashaPeriod {
                lord,
                start_time: current_start,
                end_time,
                sub_dashas: Vec::new(),
                name: None,
                interpretation: None,
                is_favorable: None,
            };

            if levels > 1 {
                dasha.sub_dashas =
                    Self::calculate_sub_periods(lord, current_start, end_time, levels - 1, None);
            }

            timeline.push(dasha);
            current_start = end_time;
        }

        timeline
    }

    fn calculate_sub_periods(
        lord: VedicPlanet,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        levels_remaining: u8,
        initial_portion: Option<f64>,
    ) -> Vec<DashaPeriod> {
        let total_duration = end.signed_duration_since(start).num_seconds() as f64;

        // Find index of the main lord in the cycle
        let lord_idx = Self::CYCLE
            .iter()
            .position(|&(p, _)| p == lord)
            .unwrap_or(0);

        let mut sub_periods = Vec::new();
        let mut current_start = start;

        // If it's the first dasha, we only show the remaining sub-dashas
        let start_offset = if let Some(portion) = initial_portion {
            let elapsed_portion = 1.0 - portion;
            let mut cumulative_portion = 0.0;
            let mut start_sub_idx = 0;
            let mut sub_elapsed_in_period = 0.0;

            for i in 0..9 {
                let idx = (lord_idx + i) % 9;
                let (_, years) = Self::CYCLE[idx];
                let p = years / Self::TOTAL_YEARS;
                if cumulative_portion + p > elapsed_portion {
                    start_sub_idx = i;
                    sub_elapsed_in_period = elapsed_portion - cumulative_portion;
                    break;
                }
                cumulative_portion += p;
            }
            Some((start_sub_idx, sub_elapsed_in_period))
        } else {
            None
        };

        if let Some((start_sub_idx, sub_elapsed)) = start_offset {
            let idx = (lord_idx + start_sub_idx) % 9;
            let (p_lord, years) = Self::CYCLE[idx];
            let full_sub_portion = years / Self::TOTAL_YEARS;
            let remaining_sub_portion = full_sub_portion - sub_elapsed;
            let full_mahadasha_duration = total_duration / initial_portion.unwrap_or(1.0);
            let sub_duration = full_mahadasha_duration * remaining_sub_portion;
            let sub_end = current_start + Duration::seconds(sub_duration as i64);

            let mut sub = DashaPeriod {
                lord: p_lord,
                start_time: current_start,
                end_time: sub_end,
                sub_dashas: Vec::new(),
                name: None,
                interpretation: None,
                is_favorable: None,
            };

            if levels_remaining > 1 {
                sub.sub_dashas = Self::calculate_sub_periods(
                    p_lord,
                    current_start,
                    sub_end,
                    levels_remaining - 1,
                    Some(sub_elapsed / full_sub_portion),
                );
            }

            sub_periods.push(sub);
            current_start = sub_end;

            for i in (start_sub_idx + 1)..9 {
                let idx = (lord_idx + i) % 9;
                let (p_lord, years) = Self::CYCLE[idx];
                let sub_duration = full_mahadasha_duration * (years / Self::TOTAL_YEARS);
                let sub_end = current_start + Duration::seconds(sub_duration as i64);

                let mut sub = DashaPeriod {
                    lord: p_lord,
                    start_time: current_start,
                    end_time: sub_end,
                    sub_dashas: Vec::new(),
                    name: None,
                    interpretation: None,
                    is_favorable: None,
                };

                if levels_remaining > 1 {
                    sub.sub_dashas = Self::calculate_sub_periods(
                        p_lord,
                        current_start,
                        sub_end,
                        levels_remaining - 1,
                        None,
                    );
                }

                sub_periods.push(sub);
                current_start = sub_end;
            }
        } else {
            for i in 0..9 {
                let idx = (lord_idx + i) % 9;
                let (p_lord, years) = Self::CYCLE[idx];
                let sub_duration = total_duration * (years / Self::TOTAL_YEARS);
                let sub_end = current_start + Duration::seconds(sub_duration as i64);

                let mut sub = DashaPeriod {
                    lord: p_lord,
                    start_time: current_start,
                    end_time: sub_end,
                    sub_dashas: Vec::new(),
                    name: None,
                    interpretation: None,
                    is_favorable: None,
                };

                if levels_remaining > 1 {
                    sub.sub_dashas = Self::calculate_sub_periods(
                        p_lord,
                        current_start,
                        sub_end,
                        levels_remaining - 1,
                        None,
                    );
                }

                sub_periods.push(sub);
                current_start = sub_end;
            }
        }

        sub_periods
    }

    pub fn attach_interpretations(timeline: &mut Vec<DashaPeriod>, chart: &VedicChart) {
        for maha in timeline.iter_mut() {
            let maha_lord = maha.lord;
            let maha_pos = chart.planets.iter().find(|p| p.planet == maha_lord);
            
            if let Some(pos) = maha_pos {
                let house = pos.house_index;
                let is_exalted = pos.rasi == maha_lord.exaltation_rasi();
                let is_debilitated = pos.rasi == maha_lord.debilitation_rasi();
                let is_own = VedicPlanet::get_ruler_of(pos.rasi) == maha_lord;
                
                let (favorable, dignity_desc) = if is_exalted {
                    (true, "고양(Exalted)되어 매우 강한 긍정적 힘을 발휘합니다.")
                } else if is_own {
                    (true, "본연의 별자리(Own House)에 있어 안정적이고 긍정적입니다.")
                } else if is_debilitated {
                    (false, "쇠락(Debilitated)하여 에너지가 약화되고 도전 과제가 주어집니다.")
                } else {
                    (true, "일반적인 상태로 본연의 역할을 수행합니다.")
                };
                
                let theme = match house {
                    1 => "자아 성장, 건강, 새로운 시작",
                    2 => "재물, 가족, 언어, 가치관",
                    3 => "의지력, 형제, 짧은 여행, 취미",
                    4 => "내면의 평화, 어머니, 부동산, 학위",
                    5 => "창의성, 자녀, 투자, 로맨스",
                    6 => "경쟁, 건강 관리, 부채 극복, 서비스",
                    7 => "배우자, 동업자, 대인관계",
                    8 => "비밀, 상속, 급격한 변화, 신비로움",
                    9 => "행운, 고등 교육, 종교, 철학, 아버지",
                    10 => "사회적 명성, 직업, 업적",
                    11 => "소득, 인맥, 목표 달성",
                    12 => "영성, 해외 연수, 지출, 은둔",
                    _ => "해당 하우스",
                };

                let lord_desc = match maha_lord {
                    VedicPlanet::Sun => "자신감과 리더십의 발현기",
                    VedicPlanet::Moon => "감정과 내면의 변화기",
                    VedicPlanet::Mars => "투지와 행동력의 시기",
                    VedicPlanet::Mercury => "지성과 소통, 상업의 시기",
                    VedicPlanet::Jupiter => "지혜, 확장, 축복의 시기",
                    VedicPlanet::Venus => "관계, 예술, 물질적 풍요의 시기",
                    VedicPlanet::Saturn => "인내, 책임, 노력에 대한 결실의 시기",
                    VedicPlanet::Rahu => "세속적 야망과 예기치 못한 확장의 시기",
                    VedicPlanet::Ketu => "영적 통찰과 분리, 과거의 청산 시기",
                    _ => "",
                };

                maha.is_favorable = Some(favorable);
                maha.interpretation = Some(format!(
                    "[{}] 이 시기는 {}하우스의 테마({})가 두드러집니다. 행성이 {} {}",
                    lord_desc, house, theme, dignity_desc,
                    if !favorable { "어려움을 극복하는 지혜가 필요합니다." } else { "" }
                ).trim().to_string());

                for antar in maha.sub_dashas.iter_mut() {
                    let antar_lord = antar.lord;
                    let antar_pos = chart.planets.iter().find(|p| p.planet == antar_lord);
                    
                    if let Some(a_pos) = antar_pos {
                        let diff = (a_pos.house_index as i32 - pos.house_index as i32).rem_euclid(12) + 1;
                        
                        let (a_favorable, rel_desc) = match diff {
                            1 | 5 | 9 => (true, "대운과 소운 행성이 1/5/9 트리콘(Trikona) 조화로운 관계에 있어 발전과 행운이 따릅니다."),
                            3 | 11 => (true, "노력에 따른 성과와 인맥 확장이 이루어집니다."),
                            4 | 10 => (true, "안정과 성취, 사회적 기틀을 다지는 켄드라(Kendra) 시기입니다."),
                            2 | 12 => (false, "수입과 지출, 혹은 가족과 단절의 이슈가 교차하는 시기입니다."),
                            6 | 8 => (false, "대운 행성과 6/8 축을 형성하여 갑작스러운 변동, 갈등, 건강 이슈 등 시련을 통해 배우는 시기입니다."),
                            7 => (true, "대인관계나 파트너십을 통한 전환점이 발생합니다."),
                            _ => (true, "무난한 흐름입니다."),
                        };

                        let a_lord_desc = match antar_lord {
                            VedicPlanet::Sun => "자아 실현",
                            VedicPlanet::Moon => "감정적 교류",
                            VedicPlanet::Mars => "추진력",
                            VedicPlanet::Mercury => "지적 활동",
                            VedicPlanet::Jupiter => "긍정적 확장",
                            VedicPlanet::Venus => "관계와 즐거움",
                            VedicPlanet::Saturn => "책임과 인내",
                            VedicPlanet::Rahu => "세속적 성취욕",
                            VedicPlanet::Ketu => "영적 통찰",
                            _ => "",
                        };

                        antar.is_favorable = Some(favorable && a_favorable);
                        antar.interpretation = Some(format!(
                            "{}의 분위기 속에서 {}. {}",
                            a_lord_desc, rel_desc,
                            if !a_favorable { "돌발적인 변화에 대비하고 무리한 확장을 자제하는 것이 좋습니다." } else { "" }
                        ).trim().to_string());
                    }
                }
            }
        }
    }
}

pub struct YoginiDasha;

impl YoginiDasha {
    const CYCLE: [(&'static str, VedicPlanet, f64); 8] = [
        ("Mangala", VedicPlanet::Moon, 1.0),
        ("Pingala", VedicPlanet::Sun, 2.0),
        ("Dhanya", VedicPlanet::Jupiter, 3.0),
        ("Bhramari", VedicPlanet::Mars, 4.0),
        ("Bhadrika", VedicPlanet::Mercury, 5.0),
        ("Ulka", VedicPlanet::Saturn, 6.0),
        ("Siddha", VedicPlanet::Venus, 7.0),
        ("Sankata", VedicPlanet::Rahu, 8.0),
    ];

    pub fn calculate_timeline(birth_time: DateTime<Utc>, moon_long: f64) -> Vec<DashaPeriod> {
        let nak_duration = 360.0 / 27.0;
        let nak_index_1 = (moon_long / nak_duration).floor() as usize + 1;
        let mut start_idx = (nak_index_1 + 3) % 8;
        if start_idx == 0 {
            start_idx = 8;
        }
        let start_idx_0 = start_idx - 1;

        let elapsed_in_nak = moon_long % nak_duration;
        let remaining_in_nak = nak_duration - elapsed_in_nak;
        let portion_remaining = remaining_in_nak / nak_duration;

        let mut timeline = Vec::new();
        let mut current_start = birth_time;
        let years_to_secs = |y: f64| (y * 365.2425 * 24.0 * 60.0 * 60.0) as i64;

        for cycle_num in 0..3 {
            for i in 0..8 {
                let idx = (start_idx_0 + i) % 8;
                let (name, lord, full_years) = Self::CYCLE[idx];
                let actual_years = if cycle_num == 0 && i == 0 {
                    full_years * portion_remaining
                } else {
                    full_years
                };
                let end_time = current_start + Duration::seconds(years_to_secs(actual_years));
                timeline.push(DashaPeriod {
                    lord,
                    start_time: current_start,
                    end_time,
                    sub_dashas: Vec::new(),
                    name: Some(name.to_string()),
                    interpretation: None,
                    is_favorable: None,
                });
                current_start = end_time;
            }
        }
        timeline
    }
}
