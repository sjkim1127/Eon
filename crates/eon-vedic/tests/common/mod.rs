use eon_vedic::chart::VedicChart;
use eon_vedic::planets::VedicPlanet;
use eon_vedic::config::AyanamsaSystem;
use chrono::{DateTime, Utc, TimeZone};

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
    
    // Expected Values
    pub expected_ak: VedicPlanet,
    pub expected_al_rasi: u8,
    pub expected_navamsa_lagna: u8,
    pub expected_dasamsa_lagna: u8,
    pub expected_current_yogini: &'static str,
}

impl VedicTestFixture {
    pub fn verify(&self, chart: &VedicChart) {
        // 1. Atmakaraka Verify
        let ak = chart.karakas.iter()
            .find(|k| matches!(k.role, eon_vedic::analysis::jaimini::JaiminiKarakaRole::Atmakaraka))
            .map(|k| k.planet);
        assert_eq!(ak, Some(self.expected_ak), "Fixture [{}]: Atmakaraka mismatch", self.name);

        // 2. Arudha Lagna Verify
        let al = chart.arudha_padas.iter().find(|a| a.house == 1).map(|a| a.rasi);
        assert_eq!(al, Some(self.expected_al_rasi), "Fixture [{}]: Arudha Lagna mismatch", self.name);

        // 3. Varga Lagna Verify
        assert_eq!(chart.ascendant.navamsa_rasi, self.expected_navamsa_lagna, "Fixture [{}]: Navamsa Lagna mismatch", self.name);
        assert_eq!(chart.ascendant.dasamsa_rasi, self.expected_dasamsa_lagna, "Fixture [{}]: Dasamsa Lagna mismatch", self.name);

        // 4. Yogini Dasha Verify
        if let Some(report) = &chart.analysis_report {
            let current_yogini = report.yogini_timeline.first()
                .and_then(|d| d.name.as_ref());
            assert_eq!(current_yogini.map(|s| s.as_str()), Some(self.expected_current_yogini), "Fixture [{}]: Current Yogini mismatch", self.name);
        }
    }

    pub fn get_time(&self) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(self.year, self.month, self.day, self.hour, self.minute, 0).unwrap()
    }
}
