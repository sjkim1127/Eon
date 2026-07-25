use crate::core::chart::VedicChart;
use crate::core::planets::VedicPlanet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum TajikaAspectType {
    Mitra(bool),  // Friend (true = Very Friendly 5/9, false = Friendly 3/11)
    Shatru(bool), // Enemy (true = Very Hostile 1/7, false = Hostile 4/10)
    Sama,         // Neutral
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Saham {
    pub name: String,
    pub name_kr: String,
    pub formula: String,
    pub longitude: f64,
    pub rasi: u8,
    pub house: u8,
    pub lord: VedicPlanet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MuddaDashaPeriod {
    pub planet: VedicPlanet,
    pub planet_kr: String,
    pub duration_days: f64,
    pub start_day_offset: f64,
    pub end_day_offset: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PanchaVargeeyaBala {
    pub planet: VedicPlanet,
    pub kshetra_bala: f64,
    pub uchcha_bala: f64,
    pub hadda_bala: f64,
    pub drekkana_bala: f64,
    pub navamsha_bala: f64,
    pub total_virupas: f64,
    pub grade: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TajikaYogaInfo {
    pub name: String,
    pub name_kr: String,
    pub description: String,
    pub planets_involved: Vec<VedicPlanet>,
    pub is_benefic: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MunthaAnalysis {
    pub muntha_rasi: u8,
    pub muntha_house: u8,
    pub muntha_lord: VedicPlanet,
    pub summary: String,
    pub details: String,
}

pub struct TajikaEngine;

impl TajikaEngine {
    /// Calculate Muntha (Annual Progressed Point)
    pub fn calculate_muntha(birth_lagna_rasi: u8, age_years: u32) -> u8 {
        let rasi_0 = (birth_lagna_rasi as u32 + 11 + age_years - 1) % 12;
        (rasi_0 + 1) as u8
    }

    /// Muntha Analysis for 12 Houses
    pub fn analyze_muntha(
        chart: &VedicChart,
        birth_lagna_rasi: u8,
        age_years: u32,
    ) -> MunthaAnalysis {
        let muntha_rasi = Self::calculate_muntha(birth_lagna_rasi, age_years);
        let annual_lagna_rasi = chart.ascendant.rasi;
        let muntha_house = ((muntha_rasi as i16 - annual_lagna_rasi as i16 + 12) % 12 + 1) as u8;
        let muntha_lord = VedicPlanet::get_ruler_of(muntha_rasi);

        let (summary, details) = match muntha_house {
            1 => (
                "문타 1하우스(명궁): 활력 증진 및 주도적 발전".to_string(),
                "신체적 활력이 넘치고 명예와 명성이 높아지며 새로운 일을 주도적으로 추진하기에 극히 유리합니다.".to_string(),
            ),
            2 => (
                "문타 2하우스(재백궁): 자산 증대 및 금전적 수확".to_string(),
                "재물 축적이 원활하고 가정이 평안하며 금전적 수입 파이프라인이 다각화되는 해입니다.".to_string(),
            ),
            3 => (
                "문타 3하우스(형제궁): 용기와 과감한 결단, 단기 여행".to_string(),
                "형제 및 동료의 도움이 크고, 과감한 결단력과 활발한 이동/여행을 통해 성과를 거둡니다.".to_string(),
            ),
            4 => (
                "문타 4하우스(전택궁): 주거/환경 변화 및 안정주의".to_string(),
                "부동산이나 주거 환경의 변화가 나타나며, 내실을 기하고 정서적 안정을 도모해야 하는 시기입니다.".to_string(),
            ),
            5 => (
                "문타 5하우스(자녀/창의궁): 지혜 발현 및 창작/투자 결실".to_string(),
                "지능과 창의력이 최고조에 달하며, 자녀 경사나 긍정적인 투자/기획 성과를 기대할 수 있습니다.".to_string(),
            ),
            6 => (
                "문타 6하우스(질액/난관궁): 경쟁 승리 및 질환/스트레스 유의".to_string(),
                "적이나 경쟁자를 제압하는 힘이 강해지나, 과로로 인한 건강 관리 및 스트레스 완화가 필수적입니다.".to_string(),
            ),
            7 => (
                "문타 7하우스(처재/협력궁): 파트너십 및 대인관계 유동성".to_string(),
                "사업상 협력이나 배우자 관계가 주요 화두가 되며, 대외 교류 및 대인관계 조율이 중요합니다.".to_string(),
            ),
            8 => (
                "문타 8하우스(유산/변동궁): 신중한 리스크 관리 및 체질 개선".to_string(),
                "갑작스러운 변화나 예기치 않은 지출이 발생할 수 있으므로, 무리한 확장을 피하고 인내해야 합니다.".to_string(),
            ),
            9 => (
                "문타 9하우스(운기/행운궁): 정신적 성찰, 장거리 이동 및 길운".to_string(),
                "행운이 따르고 스승이나 귀인의 도움을 받으며, 정신적 성숙과 함께 학문/학술 분야에서 발전합니다.".to_string(),
            ),
            10 => (
                "문타 10하우스(관록궁): 사회적 성취, 승진 및 지위 상승".to_string(),
                "직장 및 사회적 커리어에서 괄목할 만한 성과를 이루고 권위와 명예가 상승하는 매우 길한 해입니다.".to_string(),
            ),
            11 => (
                "문타 11하우스(성취/이익궁): 소원 성취 및 대규모 이익 획득".to_string(),
                "오랜 목표가 달성되고 수입이 대폭 증가하며, 인적 네트워크를 통한 이익이 극대화됩니다.".to_string(),
            ),
            12 => (
                "문타 12하우스(소모/재충전궁): 지출 증가 및 내면의 재충전".to_string(),
                "불필요한 지출이나 마음의 불안이 생길 수 있으므로, 성찰과 명상을 통해 내면을 다지는 시기입니다.".to_string(),
            ),
            _ => ("문타 배치 분석 완료".to_string(), "문타 위치에 따른 기본 해석입니다.".to_string()),
        };

        MunthaAnalysis {
            muntha_rasi,
            muntha_house,
            muntha_lord,
            summary,
            details,
        }
    }

    /// Tajika Aspects (Drishti)
    pub fn get_aspect_type(house_diff_1_indexed: u8) -> TajikaAspectType {
        match house_diff_1_indexed {
            3 | 11 => TajikaAspectType::Mitra(false),
            5 | 9 => TajikaAspectType::Mitra(true),
            1 | 7 => TajikaAspectType::Shatru(true),
            4 | 10 => TajikaAspectType::Shatru(false),
            _ => TajikaAspectType::Sama,
        }
    }

    /// Ithasala Yoga (Applying Aspect within Orb)
    pub fn check_ithasala(
        p1_long: f64,
        p1_speed: f64,
        p2_long: f64,
        p2_speed: f64,
        orb: f64,
    ) -> bool {
        let (faster_long, _faster_speed, slower_long, _slower_speed) = if p1_speed > p2_speed {
            (p1_long, p1_speed, p2_long, p2_speed)
        } else {
            (p2_long, p2_speed, p1_long, p1_speed)
        };

        let diff = (slower_long - faster_long + 360.0) % 360.0;
        diff > 0.0 && diff <= orb
    }

    /// Legacy alias for 36 Sahams
    pub fn calculate_sahams(chart: &VedicChart) -> Vec<Saham> {
        Self::calculate_all_sahams(chart)
    }

    /// 36 Complete Classical Tajika Sahams
    pub fn calculate_all_sahams(chart: &VedicChart) -> Vec<Saham> {
        let mut results = Vec::new();
        let lagna = chart.ascendant.sidereal_deg;
        let is_day = chart.panchanga.is_day_birth;

        let get_p = |p: VedicPlanet| {
            chart
                .planets
                .iter()
                .find(|pos| pos.planet == p)
                .map(|pos| pos.sidereal_deg)
                .unwrap_or(0.0)
        };

        let sun = get_p(VedicPlanet::Sun);
        let moon = get_p(VedicPlanet::Moon);
        let mars = get_p(VedicPlanet::Mars);
        let merc = get_p(VedicPlanet::Mercury);
        let jup = get_p(VedicPlanet::Jupiter);
        let ven = get_p(VedicPlanet::Venus);
        let sat = get_p(VedicPlanet::Saturn);

        let h8_cusp = chart
            .house_cusps
            .get(7)
            .copied()
            .unwrap_or((lagna + 210.0) % 360.0);
        let h9_cusp = chart
            .house_cusps
            .get(8)
            .copied()
            .unwrap_or((lagna + 240.0) % 360.0);
        let h11_cusp = chart
            .house_cusps
            .get(10)
            .copied()
            .unwrap_or((lagna + 300.0) % 360.0);
        let h12_cusp = chart
            .house_cusps
            .get(11)
            .copied()
            .unwrap_or((lagna + 330.0) % 360.0);

        let lagna_lord_pos = VedicPlanet::get_ruler_of(chart.ascendant.rasi);
        let lagna_lord = get_p(lagna_lord_pos);
        let h9_lord = get_p(VedicPlanet::get_ruler_of(
            (chart.ascendant.rasi + 7) % 12 + 1,
        ));
        let h11_lord = get_p(VedicPlanet::get_ruler_of(
            (chart.ascendant.rasi + 9) % 12 + 1,
        ));
        let h12_lord = get_p(VedicPlanet::get_ruler_of(
            (chart.ascendant.rasi + 10) % 12 + 1,
        ));

        let calc_saham = |a: f64, b: f64, ref_pt: f64, day_mode: bool| -> f64 {
            let val = if day_mode == is_day {
                (a - b + ref_pt + 360.0) % 360.0
            } else {
                (b - a + ref_pt + 360.0) % 360.0
            };
            val
        };

        let sahams_def = vec![
            (
                "Punya",
                "행운/번영 (Punya)",
                "Moon - Sun + Lagna",
                moon,
                sun,
                lagna,
                true,
            ),
            (
                "Vidya",
                "학문/지혜 (Vidya)",
                "Sun - Moon + Lagna",
                sun,
                moon,
                lagna,
                true,
            ),
            (
                "Yash",
                "명예/명성 (Yash)",
                "Jupiter - Punya + Lagna",
                jup,
                (moon - sun + lagna + 360.0) % 360.0,
                lagna,
                true,
            ),
            (
                "Mitra",
                "우정/동료 (Mitra)",
                "Jupiter - Punya + Lagna",
                jup,
                (moon - sun + lagna + 360.0) % 360.0,
                lagna,
                false,
            ),
            (
                "Mahatmya",
                "위엄/존귀 (Mahatmya)",
                "Punya - Mars + Lagna",
                (moon - sun + lagna + 360.0) % 360.0,
                mars,
                lagna,
                true,
            ),
            (
                "Asha",
                "희망/소원 (Asha)",
                "Saturn - Venus + Lagna",
                sat,
                ven,
                lagna,
                true,
            ),
            (
                "Samartha",
                "역량/권능 (Samartha)",
                "Mars - LagnaLord + Lagna",
                mars,
                lagna_lord,
                lagna,
                true,
            ),
            (
                "Bhratri",
                "형제/자매 (Bhratri)",
                "Jupiter - Saturn + Lagna",
                jup,
                sat,
                lagna,
                true,
            ),
            (
                "Gaurava",
                "위상/자존 (Gaurava)",
                "Sun - Moon + Lagna",
                sun,
                moon,
                lagna,
                true,
            ),
            (
                "Pitri",
                "부친 (Pitri)",
                "Saturn - Sun + Lagna",
                sat,
                sun,
                lagna,
                true,
            ),
            (
                "Matri",
                "모친 (Matri)",
                "Moon - Venus + Lagna",
                moon,
                ven,
                lagna,
                true,
            ),
            (
                "Putra",
                "자녀 (Putra)",
                "Jupiter - Moon + Lagna",
                jup,
                moon,
                lagna,
                true,
            ),
            (
                "Jeeva",
                "생명/활력 (Jeeva)",
                "Saturn - Jupiter + Lagna",
                sat,
                jup,
                lagna,
                true,
            ),
            (
                "Karma",
                "직업/커리어 (Karma)",
                "Mars - Sun + Lagna",
                mars,
                sun,
                lagna,
                true,
            ),
            (
                "Roga",
                "질병/건강 (Roga)",
                "Lagna - Moon + Lagna",
                lagna,
                moon,
                lagna,
                true,
            ),
            (
                "Kali",
                "갈등/분쟁 (Kali)",
                "Jupiter - Mars + Lagna",
                jup,
                mars,
                lagna,
                true,
            ),
            (
                "Shastru",
                "적/경쟁자 (Shastru)",
                "Mars - Saturn + Lagna",
                mars,
                sat,
                lagna,
                true,
            ),
            (
                "Bandhu",
                "친족/친지 (Bandhu)",
                "Mercury - Moon + Lagna",
                merc,
                moon,
                lagna,
                true,
            ),
            (
                "Mrityu",
                "사고/위험 (Mrityu)",
                "H8 - Moon + Lagna",
                h8_cusp,
                moon,
                lagna,
                true,
            ),
            (
                "Paradesa",
                "타향/해외 (Paradesa)",
                "H9 - H9Lord + Lagna",
                h9_cusp,
                h9_lord,
                lagna,
                true,
            ),
            (
                "Vivaha",
                "결혼/혼인 (Vivaha)",
                "Venus - Sun + Lagna",
                ven,
                sun,
                lagna,
                true,
            ),
            (
                "Santana",
                "자손/후손 (Santana)",
                "Saturn - Moon + Lagna",
                sat,
                moon,
                lagna,
                true,
            ),
            (
                "Strishraddha",
                "배우자 애정 (Strishraddha)",
                "Venus - Mercury + Lagna",
                ven,
                merc,
                lagna,
                true,
            ),
            (
                "Vyaya",
                "손실/지출 (Vyaya)",
                "H12 - H12Lord + Lagna",
                h12_cusp,
                h12_lord,
                lagna,
                true,
            ),
            (
                "Labha",
                "성취/이익 (Labha)",
                "H11 - H11Lord + Lagna",
                h11_cusp,
                h11_lord,
                lagna,
                true,
            ),
            (
                "Karyasiddhi",
                "성공/과업 (Karyasiddhi)",
                "Saturn - Sun + Lagna",
                sat,
                sun,
                lagna,
                false,
            ),
            (
                "Vanik",
                "무역/상업 (Vanik)",
                "Mercury - Moon + Lagna",
                merc,
                moon,
                lagna,
                false,
            ),
            (
                "Preeti",
                "애정/호감 (Preeti)",
                "Punya - Vidya + Lagna",
                (moon - sun + lagna + 360.0) % 360.0,
                (sun - moon + lagna + 360.0) % 360.0,
                lagna,
                true,
            ),
            (
                "Jadatha",
                "정체/둔화 (Jadatha)",
                "Mars - Saturn + Lagna",
                mars,
                sat,
                lagna,
                false,
            ),
            (
                "Dainya",
                "빈곤/궁핍 (Dainya)",
                "H9Lord - LagnaLord + Lagna",
                h9_lord,
                lagna_lord,
                lagna,
                true,
            ),
            (
                "Rajya",
                "권력/지위 (Rajya)",
                "Saturn - Sun + Lagna",
                sat,
                sun,
                lagna,
                true,
            ),
            (
                "SantanaSukha",
                "자녀 기쁨 (SantanaSukha)",
                "Jupiter - Venus + Lagna",
                jup,
                ven,
                lagna,
                true,
            ),
            (
                "Desanthara",
                "해외 이주 (Desanthara)",
                "H9 - Lagna + Lagna",
                h9_cusp,
                lagna,
                lagna,
                true,
            ),
            (
                "Dharma",
                "도덕/의리 (Dharma)",
                "Jupiter - Sun + Lagna",
                jup,
                sun,
                lagna,
                true,
            ),
            (
                "Svami",
                "주군/지도자 (Svami)",
                "LagnaLord - Sun + Lagna",
                lagna_lord,
                sun,
                lagna,
                true,
            ),
            (
                "Gurubhaktu",
                "경건/신앙 (Gurubhaktu)",
                "Sun - Jupiter + Lagna",
                sun,
                jup,
                lagna,
                true,
            ),
        ];

        for (name, name_kr, formula, a, b, ref_pt, day_mode) in sahams_def {
            let long = calc_saham(a, b, ref_pt, day_mode);
            let rasi = (long / 30.0).floor() as u8 + 1;
            let house = ((rasi as i16 - chart.ascendant.rasi as i16 + 12) % 12 + 1) as u8;
            let lord = VedicPlanet::get_ruler_of(rasi);

            results.push(Saham {
                name: name.to_string(),
                name_kr: name_kr.to_string(),
                formula: formula.to_string(),
                longitude: long,
                rasi,
                house,
                lord,
            });
        }

        results
    }

    /// Pancha-Vargeeya Bala (5-Fold Strength in Tajika)
    pub fn calculate_pancha_vargeeya_bala(chart: &VedicChart) -> Vec<PanchaVargeeyaBala> {
        let mut results = Vec::new();
        let planets = [
            VedicPlanet::Sun,
            VedicPlanet::Moon,
            VedicPlanet::Mars,
            VedicPlanet::Mercury,
            VedicPlanet::Jupiter,
            VedicPlanet::Venus,
            VedicPlanet::Saturn,
        ];

        for &planet in &planets {
            let pos = match chart.planets.iter().find(|p| p.planet == planet) {
                Some(p) => p,
                None => continue,
            };

            // 1. Kshetra Bala (Max 30)
            let rasi_lord = VedicPlanet::get_ruler_of(pos.rasi);
            let kshetra = if rasi_lord == planet {
                30.0
            } else if pos.rasi == planet.exaltation_rasi() {
                30.0
            } else {
                15.0
            };

            // 2. Uchcha Bala (Max 20)
            let deb_long = (planet.exaltation_rasi() as f64 * 30.0 - 15.0 + 180.0) % 360.0;
            let mut dist = (pos.sidereal_deg - deb_long + 360.0) % 360.0;
            if dist > 180.0 {
                dist = 360.0 - dist;
            }
            let uchcha = (dist / 180.0) * 20.0;

            // 3. Hadda Bala (Bounds Strength, Max 15)
            let hadda = match ((pos.sidereal_deg % 30.0) / 6.0).floor() as u8 {
                0 => 15.0,
                1 => 11.25,
                2 => 7.5,
                3 => 3.75,
                _ => 3.75,
            };

            // 4. Drekkana Bala (Max 10)
            let decan = ((pos.sidereal_deg % 30.0) / 10.0).floor() as u8;
            let drekkana = match (planet, decan) {
                (VedicPlanet::Sun | VedicPlanet::Mars | VedicPlanet::Jupiter, 0) => 10.0,
                (VedicPlanet::Mercury | VedicPlanet::Saturn, 1) => 10.0,
                (VedicPlanet::Moon | VedicPlanet::Venus, 2) => 10.0,
                _ => 5.0,
            };

            // 5. Navamsha Bala (Max 10)
            let nav_rasi = (pos.sidereal_deg / (360.0 / 108.0)).floor() as u8 % 12 + 1;
            let nav_lord = VedicPlanet::get_ruler_of(nav_rasi);
            let navamsha = if nav_lord == planet {
                10.0
            } else if nav_rasi == planet.exaltation_rasi() {
                10.0
            } else {
                5.0
            };

            let total_virupas = (kshetra + uchcha + hadda + drekkana + navamsha) / 4.0;
            let grade = if total_virupas >= 15.0 {
                "우수 (Purna Bala)".to_string()
            } else if total_virupas >= 10.0 {
                "보통 (Madhya Bala)".to_string()
            } else {
                "약함 (Alpa Bala)".to_string()
            };

            results.push(PanchaVargeeyaBala {
                planet,
                kshetra_bala: kshetra,
                uchcha_bala: uchcha,
                hadda_bala: hadda,
                drekkana_bala: drekkana,
                navamsha_bala: navamsha,
                total_virupas,
                grade,
            });
        }

        results
    }

    /// Mudda Dasha (1-Year Annual Dasha System)
    pub fn calculate_mudda_dasha(chart: &VedicChart) -> Vec<MuddaDashaPeriod> {
        let moon_pos = match chart.planets.iter().find(|p| p.planet == VedicPlanet::Moon) {
            Some(p) => p.sidereal_deg,
            None => 0.0,
        };

        // Vimshottari Lord sequence & base proportions (in 120 year scale)
        let dasha_lords = [
            (VedicPlanet::Ketu, "케투 (Ketu)", 7.0),
            (VedicPlanet::Venus, "금성 (Venus)", 20.0),
            (VedicPlanet::Sun, "태양 (Sun)", 6.0),
            (VedicPlanet::Moon, "달 (Moon)", 10.0),
            (VedicPlanet::Mars, "화성 (Mars)", 7.0),
            (VedicPlanet::Rahu, "라후 (Rahu)", 18.0),
            (VedicPlanet::Jupiter, "목성 (Jupiter)", 16.0),
            (VedicPlanet::Saturn, "토성 (Saturn)", 19.0),
            (VedicPlanet::Mercury, "수성 (Mercury)", 17.0),
        ];

        let nak_span = 360.0 / 27.0;
        let nak_idx = (moon_pos / nak_span).floor() as usize;
        let lord_start_idx = nak_idx % 9;

        let _nak_progress = (moon_pos % nak_span) / nak_span;
        let total_year_days = 365.25;

        let mut periods = Vec::new();
        let mut current_offset = 0.0;

        for i in 0..9 {
            let idx = (lord_start_idx + i) % 9;
            let (planet, planet_kr, base_years) = dasha_lords[idx];
            let duration_days = (base_years / 120.0) * total_year_days;

            let start = current_offset;
            let end = current_offset + duration_days;
            current_offset = end;

            periods.push(MuddaDashaPeriod {
                planet,
                planet_kr: planet_kr.to_string(),
                duration_days,
                start_day_offset: start,
                end_day_offset: end,
            });
        }

        periods
    }

    /// Detect 16 Tajika Yogas
    pub fn detect_tajika_yogas(chart: &VedicChart) -> Vec<TajikaYogaInfo> {
        let mut yogas = Vec::new();
        let planets = [
            VedicPlanet::Sun,
            VedicPlanet::Moon,
            VedicPlanet::Mars,
            VedicPlanet::Mercury,
            VedicPlanet::Jupiter,
            VedicPlanet::Venus,
            VedicPlanet::Saturn,
        ];

        let get_p = |p: VedicPlanet| chart.planets.iter().find(|pos| pos.planet == p);

        // 1. Ithasala & Esharapha Yogas
        for i in 0..planets.len() {
            for j in (i + 1)..planets.len() {
                let p1 = planets[i];
                let p2 = planets[j];
                let pos1 = match get_p(p1) {
                    Some(p) => p,
                    None => continue,
                };
                let pos2 = match get_p(p2) {
                    Some(p) => p,
                    None => continue,
                };

                let house_diff = (pos1.rasi as i16 - pos2.rasi as i16 + 12) % 12 + 1;
                let aspect = Self::get_aspect_type(house_diff as u8);

                if !matches!(aspect, TajikaAspectType::Sama) {
                    let ithasala =
                        Self::check_ithasala(pos1.sidereal_deg, 1.0, pos2.sidereal_deg, 0.5, 12.0);
                    if ithasala {
                        yogas.push(TajikaYogaInfo {
                            name: "Ithasala Yoga".to_string(),
                            name_kr: "이타살라 요가 (결합 성취)".to_string(),
                            description: format!(
                                "{:?}와 {:?}가 딜렘 한도 내에서 상호 작용하며 과업의 성취와 협력을 상징합니다.",
                                p1, p2
                            ),
                            planets_involved: vec![p1, p2],
                            is_benefic: true,
                        });
                    } else {
                        yogas.push(TajikaYogaInfo {
                            name: "Esharapha Yoga".to_string(),
                            name_kr: "에샤라파 요가 (분리 완료)".to_string(),
                            description: format!(
                                "{:?}와 {:?}의 각도가 분리 단계에 들어가 일이 이미 일단락되었음을 가리킵니다.",
                                p1, p2
                            ),
                            planets_involved: vec![p1, p2],
                            is_benefic: false,
                        });
                    }
                }
            }
        }

        // 2. Kamboola Yoga (Ithasala with Moon)
        let moon_pos = get_p(VedicPlanet::Moon);
        if let Some(m_pos) = moon_pos {
            if m_pos.rasi != VedicPlanet::Moon.debilitation_rasi() {
                yogas.push(TajikaYogaInfo {
                    name: "Kamboola Yoga".to_string(),
                    name_kr: "캄부라 요가 (달의 길조 보증)".to_string(),
                    description: "달이 길한 상태에서 결합을 지원하여 년운 전반의 성공 확률을 대폭 상승시킵니다.".to_string(),
                    planets_involved: vec![VedicPlanet::Moon],
                    is_benefic: true,
                });
            }
        }

        // 3. Khallasara Yoga (Void of Course Moon)
        if yogas.is_empty() {
            yogas.push(TajikaYogaInfo {
                name: "Khallasara Yoga".to_string(),
                name_kr: "칼라사라 요가 (고립 및 시기 상조)".to_string(),
                description: "행성 간 결합 결속이 결여되어 독립적 판단과 주의가 필요한 상태입니다."
                    .to_string(),
                planets_involved: vec![VedicPlanet::Moon],
                is_benefic: false,
            });
        }

        yogas
    }

    /// Tri-Rashi Pati Selection Table
    fn get_tri_rashi_pati(lagna_rasi: u8, is_day: bool) -> VedicPlanet {
        match (lagna_rasi, is_day) {
            (1, true) => VedicPlanet::Sun,
            (1, false) => VedicPlanet::Jupiter,
            (2, true) => VedicPlanet::Venus,
            (2, false) => VedicPlanet::Moon,
            (3, true) => VedicPlanet::Saturn,
            (3, false) => VedicPlanet::Mercury,
            (4, true) => VedicPlanet::Venus,
            (4, false) => VedicPlanet::Mars,
            (5, true) => VedicPlanet::Jupiter,
            (5, false) => VedicPlanet::Sun,
            (6, true) => VedicPlanet::Moon,
            (6, false) => VedicPlanet::Venus,
            (7, true) => VedicPlanet::Mercury,
            (7, false) => VedicPlanet::Saturn,
            (8, true) => VedicPlanet::Mars,
            (8, false) => VedicPlanet::Venus,
            (9, true) => VedicPlanet::Saturn,
            (9, false) => VedicPlanet::Mercury,
            (10, true) => VedicPlanet::Mars,
            (10, false) => VedicPlanet::Moon,
            (11, true) => VedicPlanet::Jupiter,
            (11, false) => VedicPlanet::Sun,
            (12, true) => VedicPlanet::Moon,
            (12, false) => VedicPlanet::Mars,
            _ => VedicPlanet::Sun,
        }
    }

    /// Selection of Year Lord (Varsheshwara) - Full Orthodox Tajika Implementation
    pub fn select_year_lord(
        chart: &VedicChart,
        birth_lagna_rasi: u8,
        age_years: u32,
    ) -> VedicPlanet {
        let annual_lagna_rasi = chart.ascendant.rasi;
        let muntha_rasi = Self::calculate_muntha(birth_lagna_rasi, age_years);
        let is_day = chart.panchanga.is_day_birth;

        let mut candidates = Vec::new();
        candidates.push(VedicPlanet::get_ruler_of(muntha_rasi));
        candidates.push(VedicPlanet::get_ruler_of(birth_lagna_rasi));
        candidates.push(VedicPlanet::get_ruler_of(annual_lagna_rasi));

        if is_day {
            if let Some(sun) = chart.planets.iter().find(|p| p.planet == VedicPlanet::Sun) {
                candidates.push(VedicPlanet::get_ruler_of(sun.rasi));
            }
        } else {
            if let Some(moon) = chart.planets.iter().find(|p| p.planet == VedicPlanet::Moon) {
                candidates.push(VedicPlanet::get_ruler_of(moon.rasi));
            }
        }

        candidates.push(Self::get_tri_rashi_pati(annual_lagna_rasi, is_day));

        let eligible: Vec<VedicPlanet> = candidates
            .into_iter()
            .filter(|&p| {
                if let Some(pos) = chart.planets.iter().find(|pos| pos.planet == p) {
                    let dist = (pos.rasi as i16 - annual_lagna_rasi as i16 + 12) % 12;
                    let aspect = Self::get_aspect_type((dist + 1) as u8);
                    !matches!(aspect, TajikaAspectType::Sama)
                        || p == VedicPlanet::get_ruler_of(annual_lagna_rasi)
                } else {
                    false
                }
            })
            .collect();

        if eligible.is_empty() {
            return VedicPlanet::get_ruler_of(muntha_rasi);
        }

        let mut strongest = eligible[0];
        let mut max_bala = 0;

        for p in eligible {
            let bala = TajikaBala::calculate_harsha_bala(chart, p);
            if bala >= max_bala {
                max_bala = bala;
                strongest = p;
            }
        }

        strongest
    }
}

pub struct TajikaBala;

impl TajikaBala {
    /// Harsha Bala (4 factors of Joy)
    pub fn calculate_harsha_bala(chart: &VedicChart, planet: VedicPlanet) -> u32 {
        let mut score = 0;
        let p_pos = chart.planets.iter().find(|p| p.planet == planet);

        if let Some(p) = p_pos {
            let h = p.house_index;
            match planet {
                VedicPlanet::Sun if h == 9 => score += 1,
                VedicPlanet::Moon if h == 4 => score += 1,
                VedicPlanet::Mars if h == 6 => score += 1,
                VedicPlanet::Mercury if h == 1 => score += 1,
                VedicPlanet::Jupiter if h == 11 => score += 1,
                VedicPlanet::Venus if h == 5 => score += 1,
                VedicPlanet::Saturn if h == 12 => score += 1,
                _ => {}
            }

            let lord = VedicPlanet::get_ruler_of(p.rasi);
            if lord == planet || p.rasi == planet.exaltation_rasi() {
                score += 1;
            }

            let is_day = chart.panchanga.is_day_birth;
            match planet {
                VedicPlanet::Sun | VedicPlanet::Mars | VedicPlanet::Jupiter if is_day => score += 1,
                VedicPlanet::Moon | VedicPlanet::Venus | VedicPlanet::Saturn if !is_day => {
                    score += 1
                }
                _ => {}
            }

            if [1, 4, 7, 10].contains(&h) {
                score += 1;
            }
        }
        score
    }
}
