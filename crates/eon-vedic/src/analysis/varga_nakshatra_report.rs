//! Varga Nakshatra Report: D1-style detailed nakshatra table for each varga chart.
//! Provides position, nakshatra, pada, lords, deity, purpose for each planet in D1, D9, D10, D108, etc.

use crate::analysis::nakshatra::NakshatraEngine;
use crate::calc::varga::VargaType;
use crate::chart::VedicChart;
use crate::core::names::get_rasi_name;
use serde::{Deserialize, Serialize};

/// Single row of a varga nakshatra report (one planet in one varga)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VargaNakshatraReportRow {
    pub planet: String,
    pub position_str: String,
    pub sign: u8,
    pub house: u8,
    pub nakshatra: u8,
    pub nakshatra_name: String,
    pub pada: u8,
    pub pada_range: String,
    pub nakshatra_lord: String,
    pub pada_lord: String,
    pub deity: String,
    pub purpose: String,
    pub is_retrograde: bool,
    pub is_combust: bool,
}

/// Full report for one varga (e.g. D1, D9, D10, D108)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VargaNakshatraReport {
    pub varga_id: String,
    pub varga_label: String,
    pub lagna_rasi: u8,
    pub rows: Vec<VargaNakshatraReportRow>,
}

/// Map of varga_id -> report (all D1~D144 vargas)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VargaNakshatraReports {
    pub reports: std::collections::HashMap<String, VargaNakshatraReport>,
}

/// VargaType -> (varga_id, varga_label) for frontend VARGA_DEFS compatibility
const VARGA_MAPPING: [(VargaType, &str, &str); 23] = [
    (VargaType::D1, "rasi", "D1 - Rasi"),
    (VargaType::D2, "hora", "D2 - Hora"),
    (VargaType::D3, "drekkana", "D3 - Drekkana"),
    (VargaType::D4, "chaturthamsha", "D4 - Chaturthamsha"),
    (VargaType::D5, "panchamsa", "D5 - Panchamsa"),
    (VargaType::D6, "shashtamsa", "D6 - Shashtamsa"),
    (VargaType::D7, "saptamsa", "D7 - Saptamsa"),
    (VargaType::D8, "ashtamsa", "D8 - Ashtamsa"),
    (VargaType::D9, "navamsa", "D9 - Navamsa"),
    (VargaType::D10, "dasamsa", "D10 - Dasamsa"),
    (VargaType::D11, "rudramsa", "D11 - Rudramsa"),
    (VargaType::D12, "dwadasamsa", "D12 - Dwadasamsa"),
    (VargaType::D16, "shodashamsa", "D16 - Shodashamsa"),
    (VargaType::D20, "vimsamsa", "D20 - Vimsamsa"),
    (VargaType::D24, "chaturvimshamsa", "D24 - Chaturvimshamsa"),
    (VargaType::D27, "saptavimsamsa", "D27 - Saptavimsamsa"),
    (VargaType::D30, "trimsamsa", "D30 - Trimsamsa"),
    (VargaType::D40, "khavedamsa", "D40 - Khavedamsa"),
    (VargaType::D45, "akshavedamsa", "D45 - Akshavedamsa"),
    (VargaType::D60, "shashtyamsa", "D60 - Shashtyamsa"),
    (VargaType::D81, "navanavamsa", "D81 - Navanavamsa"),
    (VargaType::D108, "ashtottaramsa", "D108 - Ashtottaramsa"),
    (VargaType::D144, "dwadasdwadasamsa", "D144 - Dwadasdwadasamsa"),
];

fn fmt_degree(deg: f64) -> String {
    let total = ((deg % 360.0) + 360.0) % 360.0;
    let mut sign_1based = (total / 30.0).floor() as u8 % 12 + 1;
    let deg_in_sign = total % 30.0;
    let mut dd = deg_in_sign.floor() as u32;
    let mut mm = ((deg_in_sign - dd as f64) * 60.0).round() as u32;
    
    if mm >= 60 {
        mm = 0;
        dd += 1;
        if dd >= 30 {
            dd = 0;
            sign_1based = (sign_1based % 12) + 1;
        }
    }
    
    let sign_name = get_rasi_name(sign_1based);
    format!("{}°{:02}' {}", dd, mm, sign_name)
}

fn build_row_raw(
    name: &str,
    sidereal_deg: f64,
    varga_type: VargaType,
    lagna_rasi: u8,
    is_retrograde: bool,
    is_combust: bool,
) -> VargaNakshatraReportRow {
    let varga_rasi = varga_type.calculate_rasi(sidereal_deg);
    let effective_long = varga_type.effective_longitude_for_nakshatra(sidereal_deg, varga_rasi);
    let (nakshatra, pada) = NakshatraEngine::nakshatra_and_pada(effective_long);

    let nakshatra_name = NakshatraEngine::get_name(nakshatra).to_string();
    let nakshatra_lord = format!("{:?}", NakshatraEngine::get_lord(nakshatra));
    let pada_lord = format!("{:?}", NakshatraEngine::get_pada_lord(nakshatra, pada));
    let deity = NakshatraEngine::get_deity(nakshatra).to_string();
    let purpose = NakshatraEngine::get_purpose(nakshatra).to_string();

    // Pada range: each pada = 13.333/4 = 3.333 degrees
    let nak_start = (nakshatra as f64 - 1.0) * (360.0 / 27.0);
    let pada_size = (360.0 / 27.0) / 4.0;
    let pada_start = nak_start + (pada as f64 - 1.0) * pada_size;
    let pada_end = pada_start + pada_size;
    let pada_range = format!("{} – {}", fmt_degree(pada_start), fmt_degree(pada_end));

    let position_str = if matches!(varga_type, VargaType::D1) {
        fmt_degree(sidereal_deg)
    } else {
        fmt_degree(effective_long)
    };
    let house = ((varga_rasi as i32 - lagna_rasi as i32 + 12) % 12) as u8 + 1;

    VargaNakshatraReportRow {
        planet: name.to_string(),
        position_str,
        sign: varga_rasi,
        house,
        nakshatra,
        nakshatra_name,
        pada,
        pada_range,
        nakshatra_lord,
        pada_lord,
        deity,
        purpose,
        is_retrograde,
        is_combust,
    }
}

fn build_report_for_varga(
    chart: &VedicChart,
    varga_type: VargaType,
    varga_id: &str,
    varga_label: &str,
) -> VargaNakshatraReport {
    let lagna_rasi = chart.ascendant.varga_rasi(varga_type);
    let mut rows = Vec::new();

    for p in &chart.planets {
        rows.push(build_row_raw(
            &format!("{:?}", p.planet),
            p.sidereal_deg,
            varga_type,
            lagna_rasi,
            p.is_retrograde,
            p.is_combust,
        ));
    }

    // Add ASC
    rows.push(build_row_raw(
        "ASC",
        chart.ascendant.sidereal_deg,
        varga_type,
        lagna_rasi,
        false,
        false,
    ));

    // Add IC (H4), DSC (H7), MC (H10) if house_cusps available
    if chart.house_cusps.len() >= 12 {
        rows.push(build_row_raw(
            "IC (H4)",
            chart.house_cusps[3],
            varga_type,
            lagna_rasi,
            false,
            false,
        ));
        rows.push(build_row_raw(
            "DSC (H7)",
            chart.house_cusps[6],
            varga_type,
            lagna_rasi,
            false,
            false,
        ));
        rows.push(build_row_raw(
            "MC (H10)",
            chart.house_cusps[9],
            varga_type,
            lagna_rasi,
            false,
            false,
        ));
    }

    VargaNakshatraReport {
        varga_id: varga_id.to_string(),
        varga_label: varga_label.to_string(),
        lagna_rasi,
        rows,
    }
}

/// Build varga nakshatra reports for all D1~D144 vargas
pub fn build_varga_nakshatra_reports(chart: &VedicChart) -> VargaNakshatraReports {
    let mut reports = std::collections::HashMap::new();
    for (varga_type, varga_id, varga_label) in VARGA_MAPPING.iter() {
        let report = build_report_for_varga(chart, *varga_type, varga_id, varga_label);
        reports.insert(varga_id.to_string(), report);
    }
    VargaNakshatraReports { reports }
}
