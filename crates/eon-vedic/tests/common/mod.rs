use chrono::{TimeZone, Utc};
use eon_vedic::calc::varga::VargaType;
use eon_vedic::core::chart::{VedicChart, VedicChartCalculator};

#[allow(dead_code)]
pub fn get_varga_type(id: &str) -> Option<VargaType> {
    match id.to_lowercase().as_str() {
        "d1" | "rasi" => Some(VargaType::D1),
        "d2" | "hora" => Some(VargaType::D2),
        "d3" | "drekkana" => Some(VargaType::D3),
        "d4" | "chaturthamsha" => Some(VargaType::D4),
        "d5" | "panchamsa" => Some(VargaType::D5),
        "d6" | "shashtamsa" => Some(VargaType::D6),
        "d7" | "saptamsa" => Some(VargaType::D7),
        "d8" | "ashtamsa" => Some(VargaType::D8),
        "d9" | "navamsa" => Some(VargaType::D9),
        "d10" | "dasamsa" => Some(VargaType::D10),
        "d11" | "rudramsa" => Some(VargaType::D11),
        "d12" | "dwadasamsa" => Some(VargaType::D12),
        "d16" | "shodashamsa" => Some(VargaType::D16),
        "d20" | "vimsamsa" => Some(VargaType::D20),
        "d24" | "chaturvimshamsa" => Some(VargaType::D24),
        "d27" | "saptavimsamsa" => Some(VargaType::D27),
        "d30" | "trimsamsa" => Some(VargaType::D30),
        "d40" | "khavedamsa" => Some(VargaType::D40),
        "d45" | "akshavedamsa" => Some(VargaType::D45),
        "d60" | "shashtyamsa" => Some(VargaType::D60),
        "d81" | "navanavamsa" => Some(VargaType::D81),
        "d108" | "ashtottaramsa" => Some(VargaType::D108),
        "d144" | "dwadasdwadasamsa" => Some(VargaType::D144),
        _ => None,
    }
}

#[allow(dead_code)]
pub fn create_test_chart(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    lat: f64,
    lon: f64,
) -> VedicChart {
    let calc = VedicChartCalculator::default();
    let dt = Utc.with_ymd_and_hms(year, month, day, hour, 0, 0).unwrap();
    calc.calculate(dt, lat, lon).unwrap()
}

#[allow(dead_code)]
pub fn assert_approx_eq(actual: f64, expected: f64, epsilon: f64, msg: &str) {
    if (actual - expected).abs() > epsilon {
        panic!("{}: expected {}, got {}", msg, expected, actual);
    }
}

use chrono::DateTime;
use eon_vedic::config::AyanamsaSystem;
use eon_vedic::planets::VedicPlanet;

pub struct ExpectedJaimini {
    pub atmakaraka: Option<VedicPlanet>,
    pub arudha_lagna: Option<u8>,
    pub upapada_lagna: Option<u8>,
    pub a10_rasi: Option<u8>,
}

pub struct ExpectedVarga {
    pub navamsa_lagna: Option<u8>,
    pub dasamsa_lagna: Option<u8>,
    pub vargottama_planets: &'static [VedicPlanet],
    pub pushkar_planets: &'static [VedicPlanet],
}

pub struct ExpectedDasha {
    pub current_yogini: Option<&'static str>,
}

pub struct ExpectedTajika {
    pub punya_rasi: Option<u8>,
    pub vidya_rasi: Option<u8>,
    pub high_harsha_bala: &'static [VedicPlanet],
}

pub struct ExpectedAvastha {
    pub deeptaadi: &'static [(VedicPlanet, &'static str)],
}

pub struct VedicTestFixture {
    pub name: &'static str,
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub latitude: f64,
    pub longitude: f64,
    pub ayanamsa: AyanamsaSystem,

    pub jaimini: Option<ExpectedJaimini>,
    pub varga: Option<ExpectedVarga>,
    pub dasha: Option<ExpectedDasha>,
    pub tajika: Option<ExpectedTajika>,
    pub avastha: Option<ExpectedAvastha>,
}

impl VedicTestFixture {
    pub fn verify(&self, chart: &VedicChart) {
        if let Some(expected) = &self.jaimini {
            self.verify_jaimini(chart, expected);
        }
        if let Some(expected) = &self.varga {
            self.verify_varga(chart, expected);
        }
        if let Some(expected) = &self.dasha {
            self.verify_dasha(chart, expected);
        }
        if let Some(expected) = &self.tajika {
            self.verify_tajika(chart, expected);
        }
        if let Some(expected) = &self.avastha {
            self.verify_avastha(chart, expected);
        }
    }

    fn verify_jaimini(&self, chart: &VedicChart, expected: &ExpectedJaimini) {
        if let Some(expected_ak) = expected.atmakaraka {
            let ak = chart
                .karakas
                .iter()
                .find(|k| {
                    matches!(
                        k.role,
                        eon_vedic::analysis::jaimini::JaiminiKarakaRole::Atmakaraka
                    )
                })
                .map(|k| k.planet);
            assert_eq!(
                ak,
                Some(expected_ak),
                "Fixture [{}]: Atmakaraka mismatch",
                self.name
            );
        }

        if let Some(expected_al) = expected.arudha_lagna {
            let al = chart
                .arudha_padas
                .iter()
                .find(|a| a.house == 1)
                .map(|a| a.rasi);
            assert_eq!(
                al,
                Some(expected_al),
                "Fixture [{}]: Arudha Lagna mismatch",
                self.name
            );
        }

        if let Some(expected_ul) = expected.upapada_lagna {
            let ul = chart
                .arudha_padas
                .iter()
                .find(|a| a.house == 12)
                .map(|a| a.rasi);
            assert_eq!(
                ul,
                Some(expected_ul),
                "Fixture [{}]: Upapada Lagna mismatch",
                self.name
            );
        }

        if let Some(expected_a10) = expected.a10_rasi {
            let a10 = chart
                .arudha_padas
                .iter()
                .find(|a| a.house == 10)
                .map(|a| a.rasi);
            assert_eq!(
                a10,
                Some(expected_a10),
                "Fixture [{}]: A10 mismatch",
                self.name
            );
        }
    }

    fn verify_varga(&self, chart: &VedicChart, expected: &ExpectedVarga) {
        if let Some(expected_nl) = expected.navamsa_lagna {
            assert_eq!(
                chart.ascendant.navamsa_rasi, expected_nl,
                "Fixture [{}]: Navamsa Lagna mismatch",
                self.name
            );
        }
        if let Some(expected_dl) = expected.dasamsa_lagna {
            assert_eq!(
                chart.ascendant.dasamsa_rasi, expected_dl,
                "Fixture [{}]: Dasamsa Lagna mismatch",
                self.name
            );
        }

        if let Some(report) = &chart.analysis_report {
            for expected_p in expected.vargottama_planets {
                let found = report
                    .varga_interpretations
                    .iter()
                    .any(|vi| vi.planet == *expected_p && vi.is_vargottama);
                assert!(
                    found,
                    "Fixture [{}]: Planet {:?} expected to be Vargottama",
                    self.name, expected_p
                );
            }

            for expected_p in expected.pushkar_planets {
                let found = report
                    .varga_interpretations
                    .iter()
                    .any(|vi| vi.planet == *expected_p && vi.is_pushkar_navamsa);
                assert!(
                    found,
                    "Fixture [{}]: Planet {:?} expected to be in Pushkar Navamsa",
                    self.name, expected_p
                );
            }
        }
    }

    fn verify_dasha(&self, chart: &VedicChart, expected: &ExpectedDasha) {
        if let Some(expected_yogini) = expected.current_yogini {
            if let Some(report) = &chart.analysis_report {
                let current_yogini = report.yogini_timeline.first().and_then(|d| d.name.as_ref());
                assert_eq!(
                    current_yogini.map(|s| s.as_str()),
                    Some(expected_yogini),
                    "Fixture [{}]: Current Yogini mismatch",
                    self.name
                );
            }
        }
    }

    fn verify_tajika(&self, chart: &VedicChart, expected: &ExpectedTajika) {
        let sahams = eon_vedic::analysis::tajika::TajikaEngine::calculate_sahams(chart);
        if let Some(expected_punya) = expected.punya_rasi {
            let punya = sahams
                .iter()
                .find(|s| s.name.contains("Punya"))
                .map(|s| s.rasi);
            assert_eq!(
                punya,
                Some(expected_punya),
                "Fixture [{}]: Punya Saham mismatch",
                self.name
            );
        }
        if let Some(expected_vidya) = expected.vidya_rasi {
            let vidya = sahams
                .iter()
                .find(|s| s.name.contains("Vidya"))
                .map(|s| s.rasi);
            assert_eq!(
                vidya,
                Some(expected_vidya),
                "Fixture [{}]: Vidya Saham mismatch",
                self.name
            );
        }

        for expected_p in expected.high_harsha_bala {
            let score =
                eon_vedic::analysis::tajika::TajikaBala::calculate_harsha_bala(chart, *expected_p);
            assert!(
                score >= 2,
                "Fixture [{}]: Planet {:?} expected to have high Harsha Bala",
                self.name,
                expected_p
            );
        }
    }

    fn verify_avastha(&self, chart: &VedicChart, expected: &ExpectedAvastha) {
        for (p, expected_label) in expected.deeptaadi {
            let avastha = chart.avasthas.iter().find(|a| a.planet == *p);
            assert!(
                avastha.is_some(),
                "Fixture [{}]: Avastha for {:?} not found",
                self.name,
                p
            );

            if let Some(a) = avastha {
                let actual_label = format!("{:?}", a.deeptaadi);
                assert_eq!(
                    actual_label, *expected_label,
                    "Fixture [{}]: Deeptaadi mismatch for {:?}",
                    self.name, p
                );
            }
        }
    }

    pub fn get_time(&self) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(self.year, self.month, self.day, self.hour, self.minute, 0)
            .unwrap()
    }
}
