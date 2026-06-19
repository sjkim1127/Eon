use dioxus::prelude::*;
use crate::store::{AnalysisState, TaskStatus};
use crate::i18n::{t, TK};
use eon_service::dto::{VedicAnalysisInput, AnalysisInput, VedicCompatibilityInput, VedicCompatibilityOutput};
use eon_service::facade;
use eon_vedic::planets::VedicPlanet;
use crate::components::shared::birth_form::BirthForm;
use chrono_tz;

const RASI_NAMES_KR: &[&str] = &[
    "", "양자리(Aries)", "황소자리(Taurus)", "쌍둥이자리(Gemini)",
    "게자리(Cancer)", "사자자리(Leo)", "처녀자리(Virgo)",
    "천칭자리(Libra)", "전갈자리(Scorpio)", "사수자리(Sagittarius)",
    "염소자리(Capricorn)", "물병자리(Aquarius)", "물고기자리(Pisces)",
];

const NAKSHATRA_NAMES: &[&str] = &[
    "", "Ashwini", "Bharani", "Krittika", "Rohini", "Mrigashira",
    "Ardra", "Punarvasu", "Pushya", "Ashlesha", "Magha",
    "Purva Phalguni", "Uttara Phalguni", "Hasta", "Chitra",
    "Swati", "Vishakha", "Anuradha", "Jyeshtha", "Mula",
    "Purva Ashadha", "Uttara Ashadha", "Shravana", "Dhanishtha",
    "Shatabhisha", "Purva Bhadrapada", "Uttara Bhadrapada", "Revati",
];

fn planet_name_kr(planet: VedicPlanet) -> &'static str {
    match planet {
        VedicPlanet::Sun => "태양 ☀️",
        VedicPlanet::Moon => "달 🌙",
        VedicPlanet::Mars => "화성 ♂",
        VedicPlanet::Mercury => "수성 ☿",
        VedicPlanet::Jupiter => "목성 ♃",
        VedicPlanet::Venus => "금성 ♀",
        VedicPlanet::Saturn => "토성 ♄",
        VedicPlanet::Rahu => "라후 ☊",
        VedicPlanet::Ketu => "케투 ☋",
        VedicPlanet::Ascendant => "라그나 ⬆️",
    }
}

fn planet_color(planet: VedicPlanet) -> &'static str {
    match planet {
        VedicPlanet::Sun => "text-orange-400",
        VedicPlanet::Moon => "text-slate-200",
        VedicPlanet::Mars => "text-red-500",
        VedicPlanet::Mercury => "text-emerald-400",
        VedicPlanet::Jupiter => "text-yellow-400",
        VedicPlanet::Venus => "text-pink-400",
        VedicPlanet::Saturn => "text-indigo-400",
        VedicPlanet::Rahu => "text-purple-400",
        VedicPlanet::Ketu => "text-amber-700",
        VedicPlanet::Ascendant => "text-white",
    }
}

fn planet_name_kr_str(p_name: &str) -> &str {
    match p_name {
        "Sun" | "Surya" => "태양 ☀️",
        "Moon" | "Chandra" => "달 🌙",
        "Mars" | "Mangala" => "화성 ♂",
        "Mercury" | "Budha" => "수성 ☿",
        "Jupiter" | "Guru" => "목성 ♃",
        "Venus" | "Shukra" => "금성 ♀",
        "Saturn" | "Shani" => "토성 ♄",
        "Rahu" => "라후 ☊",
        "Ketu" => "케투 ☋",
        "Ascendant" | "Lagna" => "라그나 ⬆️",
        _ => p_name,
    }
}

fn planet_color_str(p_name: &str) -> &'static str {
    match p_name {
        "Sun" | "Surya" => "text-orange-400",
        "Moon" | "Chandra" => "text-slate-200",
        "Mars" | "Mangala" => "text-red-500",
        "Mercury" | "Budha" => "text-emerald-400",
        "Jupiter" | "Guru" => "text-yellow-400",
        "Venus" | "Shukra" => "text-pink-400",
        "Saturn" | "Shani" => "text-indigo-400",
        "Rahu" => "text-purple-400",
        "Ketu" => "text-amber-750",
        "Ascendant" | "Lagna" => "text-white",
        _ => "text-slate-400",
    }
}


fn rasi_name(rasi: u8) -> &'static str {
    if rasi == 0 || rasi > 12 { return "—" }
    RASI_NAMES_KR[rasi as usize]
}

fn nakshatra_name(n: u8) -> &'static str {
    if n == 0 || n > 27 { return "—" }
    NAKSHATRA_NAMES[n as usize]
}

fn lajjitadi_name_kr(av: &eon_vedic::analysis::avasthas::LajjitadiAvastha) -> &'static str {
    use eon_vedic::analysis::avasthas::LajjitadiAvastha;
    match av {
        LajjitadiAvastha::Lajjita => "수치 (Lajjita)",
        LajjitadiAvastha::Garvita => "자긍 (Garvita)",
        LajjitadiAvastha::Kshudhita => "갈망 (Kshudhita)",
        LajjitadiAvastha::Trishita => "갈증 (Trishita)",
        LajjitadiAvastha::Mudita => "환희 (Mudita)",
        LajjitadiAvastha::Kshobhita => "동요 (Kshobhita)",
        LajjitadiAvastha::Neutral => "평온 (Neutral)",
    }
}

fn lajjitadi_color(av: &eon_vedic::analysis::avasthas::LajjitadiAvastha) -> &'static str {
    use eon_vedic::analysis::avasthas::LajjitadiAvastha;
    match av {
        LajjitadiAvastha::Garvita => "text-emerald-400 bg-emerald-950/40 border border-emerald-800/40",
        LajjitadiAvastha::Mudita => "text-green-400 bg-green-950/40 border border-green-800/40",
        LajjitadiAvastha::Neutral => "text-slate-400 bg-slate-950/40 border border-slate-800/40",
        LajjitadiAvastha::Lajjita => "text-amber-400 bg-amber-950/40 border border-amber-800/40",
        LajjitadiAvastha::Trishita => "text-sky-400 bg-sky-950/40 border border-sky-800/40",
        LajjitadiAvastha::Kshobhita => "text-purple-400 bg-purple-950/40 border border-purple-800/40",
        LajjitadiAvastha::Kshudhita => "text-red-400 bg-red-950/40 border border-red-800/40",
    }
}

fn rasi_name_short(rasi: u8) -> &'static str {
    match rasi {
        1 => "AR (양)",
        2 => "TA (황소)",
        3 => "GE (쌍둥)",
        4 => "CN (게)",
        5 => "LE (사자)",
        6 => "VI (처녀)",
        7 => "LI (천칭)",
        8 => "SC (전갈)",
        9 => "SG (사수)",
        10 => "CP (염소)",
        11 => "AQ (물병)",
        12 => "PI (물고기)",
        _ => "—",
    }
}

// --- Tooltip & Interaction structures ---
#[derive(Clone, Debug, PartialEq)]
pub enum VedicTooltipTarget {
    Planet {
        name: String,
        symbol: String,
        rasi_num: u8,
        house_num: u8,
        longitude_str: String,
        nakshatra_name: String,
        nakshatra_lord: String,
        is_retrograde: bool,
        is_combust: bool,
        varga_label: String,
        deity: Option<String>,
        purpose: Option<String>,
    },
    House {
        house_num: u8,
        rasi_num: u8,
        rasi_name: String,
        rasi_lord: String,
        score: Option<f64>,
        varga_label: String,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct VedicTooltipData {
    pub target: VedicTooltipTarget,
    pub x: f64,
    pub y: f64,
}

fn planet_from_lbl(lbl: &str) -> Option<VedicPlanet> {
    match lbl {
        "Su" => Some(VedicPlanet::Sun),
        "Mo" => Some(VedicPlanet::Moon),
        "Ma" => Some(VedicPlanet::Mars),
        "Me" => Some(VedicPlanet::Mercury),
        "Ju" => Some(VedicPlanet::Jupiter),
        "Ve" => Some(VedicPlanet::Venus),
        "Sa" => Some(VedicPlanet::Saturn),
        "Ra" => Some(VedicPlanet::Rahu),
        "Ke" => Some(VedicPlanet::Ketu),
        "Asc" => Some(VedicPlanet::Ascendant),
        _ => None,
    }
}

fn nakshatra_lord_kr(nakshatra_idx: u8) -> &'static str {
    if nakshatra_idx == 0 || nakshatra_idx > 27 { return "—" }
    match (nakshatra_idx - 1) % 9 {
        0 => "케투 (Ketu)",
        1 => "금성 (Venus)",
        2 => "태양 (Sun)",
        3 => "달 (Moon)",
        4 => "화성 (Mars)",
        5 => "라후 (Rahu)",
        6 => "목성 (Jupiter)",
        7 => "토성 (Saturn)",
        8 => "수성 (Mercury)",
        _ => "—"
    }
}

fn rasi_lord_kr(rasi_idx: u8) -> &'static str {
    match rasi_idx {
        1 => "화성 (Mars)",
        2 => "금성 (Venus)",
        3 => "수성 (Mercury)",
        4 => "달 (Moon)",
        5 => "태양 (Sun)",
        6 => "수성 (Mercury)",
        7 => "금성 (Venus)",
        8 => "화성 (Mars)",
        9 => "목성 (Jupiter)",
        10 => "토성 (Saturn)",
        11 => "토성 (Saturn)",
        12 => "목성 (Jupiter)",
        _ => "—"
    }
}

fn get_d1_planet_tooltip(
    lbl: &str,
    d1_planets: &[eon_vedic::core::chart::VedicPosition],
    d1_ascendant: &eon_vedic::core::chart::VedicPosition,
    varga_label: &str,
) -> Option<VedicTooltipTarget> {
    let p_opt = planet_from_lbl(lbl);
    if let Some(planet) = p_opt {
        let pos = if planet == VedicPlanet::Ascendant {
            d1_ascendant
        } else {
            d1_planets.iter().find(|p| p.planet == planet)?
        };
        
        let nak_name = nakshatra_name(pos.nakshatra);
        let nak_lord = nakshatra_lord_kr(pos.nakshatra);
        
        let planet_name = planet_name_kr(pos.planet).to_string();
        let deg_within_sign = pos.sidereal_deg % 30.0;
        let deg_floor = deg_within_sign.floor() as i32;
        let min_val = ((deg_within_sign - deg_floor as f64) * 60.0).round() as i32;
        let longitude_str = format!("{}° {:02}'", deg_floor, min_val);

        Some(VedicTooltipTarget::Planet {
            name: planet_name,
            symbol: lbl.to_string(),
            rasi_num: pos.rasi,
            house_num: pos.house_index,
            longitude_str,
            nakshatra_name: format!("{} ({}단계)", nak_name, pos.pada),
            nakshatra_lord: nak_lord.to_string(),
            is_retrograde: pos.is_retrograde,
            is_combust: pos.is_combust,
            varga_label: varga_label.to_string(),
            deity: None,
            purpose: None,
        })
    } else {
        None
    }
}

fn get_varga_planet_tooltip(
    lbl: &str,
    rows: &[eon_vedic::analysis::varga_nakshatra_report::VargaNakshatraReportRow],
    varga_label: &str,
) -> Option<VedicTooltipTarget> {
    let row = rows.iter().find(|r| {
        let r_lbl = match r.planet.as_str() {
            "Sun" | "Surya" => "Su",
            "Moon" | "Chandra" => "Mo",
            "Mars" | "Mangala" => "Ma",
            "Mercury" | "Budha" => "Me",
            "Jupiter" | "Guru" => "Ju",
            "Venus" | "Shukra" => "Ve",
            "Saturn" | "Shani" => "Sa",
            "Rahu" => "Ra",
            "Ketu" => "Ke",
            "Lagna" | "Ascendant" => "Asc",
            _ => "",
        };
        r_lbl == lbl
    })?;

    let p_name_kr = planet_name_kr_str(&row.planet).to_string();

    Some(VedicTooltipTarget::Planet {
        name: p_name_kr,
        symbol: lbl.to_string(),
        rasi_num: row.sign,
        house_num: row.house,
        longitude_str: row.position_str.clone(),
        nakshatra_name: format!("{} ({}단계)", row.nakshatra_name, row.pada),
        nakshatra_lord: planet_name_kr_str(&row.nakshatra_lord).to_string(),
        is_retrograde: row.is_retrograde,
        is_combust: row.is_combust,
        varga_label: varga_label.to_string(),
        deity: Some(row.deity.clone()),
        purpose: Some(row.purpose.clone()),
    })
}

fn get_house_tooltip(
    house_num: u8,
    lagna_rasi: u8,
    varga_label: &str,
    bhava_strengths: Option<&[eon_vedic::analysis::bhava::BhavaStrength]>,
) -> Option<VedicTooltipTarget> {
    let rasi_num = (lagna_rasi + house_num - 2) % 12 + 1;
    let rasi_name_str = rasi_name(rasi_num).to_string();
    let rasi_lord = rasi_lord_kr(rasi_num).to_string();
    
    let score = bhava_strengths.and_then(|strengths| {
        strengths.iter().find(|s| s.house == house_num).map(|s| s.total_score)
    });

    Some(VedicTooltipTarget::House {
        house_num,
        rasi_num,
        rasi_name: rasi_name_str,
        rasi_lord,
        score,
        varga_label: varga_label.to_string(),
    })
}

fn render_vedic_chart(
    rasi_planets: &[Vec<(&'static str, &'static str)>],
    house_planets: &[Vec<(&'static str, &'static str)>],
    lagna_rasi: u8,
    style: &str,
    chart_title: &str,
    active_tooltip: Signal<Option<VedicTooltipData>>,
    selected_detail: Signal<Option<VedicTooltipData>>,
    varga_rows: Option<&[eon_vedic::analysis::varga_nakshatra_report::VargaNakshatraReportRow]>,
    d1_planets: Option<&[eon_vedic::core::chart::VedicPosition]>,
    d1_ascendant: Option<&eon_vedic::core::chart::VedicPosition>,
    bhava_strengths: Option<&[eon_vedic::analysis::bhava::BhavaStrength]>,
) -> Element {
    if style == "north" {
        let house_bounds = [
            (0, 0, 0, 0, 0), // dummy 0-index
            (1, 130, 75, 140, 70), // House 1
            (2, 60, 20, 100, 45),  // House 2
            (3, 10, 70, 50, 90),   // House 3
            (4, 75, 130, 85, 140),  // House 4
            (5, 10, 240, 50, 90),  // House 5
            (6, 60, 325, 100, 45), // House 6
            (7, 130, 245, 140, 70),// House 7
            (8, 240, 325, 100, 45),// House 8
            (9, 340, 240, 50, 90), // House 9
            (10, 240, 130, 85, 140),// House 10
            (11, 340, 70, 50, 90), // House 11
            (12, 240, 20, 100, 45), // House 12
        ];

        let sign_coords = [
            (0, 0, 0), // dummy
            (1, 200, 32),
            (2, 120, 20),
            (3, 20, 115),
            (4, 115, 205),
            (5, 20, 295),
            (6, 120, 388),
            (7, 200, 372),
            (8, 280, 388),
            (9, 380, 295),
            (10, 285, 205),
            (11, 380, 115),
            (12, 280, 20),
        ];

        rsx! {
            svg {
                view_box: "0 0 400 400",
                class: "w-full h-auto max-w-[400px] aspect-square select-none font-mono mx-auto",
                rect {
                    width: "400",
                    height: "400",
                    fill: "#0b0f19",
                    rx: "12",
                    stroke: "#1e293b",
                    stroke_width: "2"
                }
                line { x1: "0", y1: "0", x2: "400", y2: "400", stroke: "#1e293b", stroke_width: "1.5" }
                line { x1: "0", y1: "400", x2: "400", y2: "0", stroke: "#1e293b", stroke_width: "1.5" }
                polygon {
                    points: "200,0 400,200 200,400 0,200",
                    fill: "none",
                    stroke: "#1e293b",
                    stroke_width: "1.5"
                }
                rect {
                    x: "155",
                    y: "182",
                    width: "90",
                    height: "36",
                    fill: "#0f172a",
                    stroke: "#1e293b",
                    stroke_width: "1",
                    rx: "4"
                }
                text {
                    x: "200",
                    y: "204",
                    text_anchor: "middle",
                    font_size: "10px",
                    font_weight: "bold",
                    fill: "#cbd5e1",
                    "{chart_title}"
                }
                
                {house_bounds.iter().map(|&(h_idx, box_x, box_y, box_w, box_h)| {
                    let sign_num = (lagna_rasi + h_idx as u8 - 2) % 12 + 1;
                    let (_, s_x, s_y) = sign_coords[h_idx];
                    let h_planets = &house_planets[h_idx];
                    
                    let house_target = get_house_tooltip(h_idx as u8, lagna_rasi, chart_title, bhava_strengths);
                    
                    let mut active_tooltip_text = active_tooltip.clone();
                    let mut active_tooltip_fo = active_tooltip.clone();
                    let mut selected_detail_text = selected_detail.clone();
                    let mut selected_detail_fo = selected_detail.clone();
                    
                    let house_target_hover_text = house_target.clone();
                    let house_target_hover_fo = house_target.clone();
                    let house_target_click_text = house_target.clone();
                    let house_target_click_fo = house_target.clone();
                    
                    rsx! {
                        text {
                            x: "{s_x}",
                            y: "{s_y}",
                            text_anchor: "middle",
                            font_size: "10px",
                            font_weight: "bold",
                            fill: "#475569",
                            class: "cursor-pointer hover:fill-blue-400 transition-colors",
                            onmouseenter: move |e| {
                                if let Some(target) = &house_target_hover_text {
                                    let coords = e.client_coordinates();
                                    *active_tooltip_text.write() = Some(VedicTooltipData {
                                        target: target.clone(),
                                        x: coords.x,
                                        y: coords.y,
                                    });
                                }
                            },
                            onmouseleave: move |_| *active_tooltip_text.write() = None,
                            onclick: move |e| {
                                if let Some(target) = &house_target_click_text {
                                    let coords = e.client_coordinates();
                                    *selected_detail_text.write() = Some(VedicTooltipData {
                                        target: target.clone(),
                                        x: coords.x,
                                        y: coords.y,
                                    });
                                }
                            },
                            "{sign_num}"
                        }
                        foreignObject {
                            x: "{box_x}",
                            y: "{box_y}",
                            width: "{box_w}",
                            height: "{box_h}",
                            class: "cursor-pointer rounded hover:bg-slate-800/10 transition-colors",
                            onmouseenter: move |e| {
                                if let Some(target) = &house_target_hover_fo {
                                    let coords = e.client_coordinates();
                                    *active_tooltip_fo.write() = Some(VedicTooltipData {
                                        target: target.clone(),
                                        x: coords.x,
                                        y: coords.y,
                                    });
                                }
                            },
                            onmouseleave: move |_| *active_tooltip_fo.write() = None,
                            onclick: move |e| {
                                if let Some(target) = &house_target_click_fo {
                                    let coords = e.client_coordinates();
                                    *selected_detail_fo.write() = Some(VedicTooltipData {
                                        target: target.clone(),
                                        x: coords.x,
                                        y: coords.y,
                                    });
                                }
                            },
                            div { class: "flex flex-wrap justify-center items-center gap-1 p-0.5 h-full",
                                {h_planets.iter().map(|&(lbl, color_cls)| {
                                    let planet_target = if let Some(rows) = varga_rows {
                                        get_varga_planet_tooltip(lbl, rows, chart_title)
                                    } else if let (Some(planets), Some(asc)) = (d1_planets, d1_ascendant) {
                                        get_d1_planet_tooltip(lbl, planets, asc, chart_title)
                                    } else {
                                        None
                                    };
                                    
                                    let mut active_tooltip_clone_p = active_tooltip.clone();
                                    let mut selected_detail_clone_p = selected_detail.clone();
                                    let planet_target_hover = planet_target.clone();
                                    let planet_target_click = planet_target.clone();
                                    
                                    rsx! {
                                        span { 
                                            class: "{color_cls} text-[9px] bg-slate-950/80 border border-slate-900/60 px-1 py-0.5 rounded cursor-pointer hover:bg-slate-800 hover:border-slate-700 transition-all", 
                                            onmouseenter: move |e| {
                                                if let Some(target) = &planet_target_hover {
                                                    let coords = e.client_coordinates();
                                                    *active_tooltip_clone_p.write() = Some(VedicTooltipData {
                                                        target: target.clone(),
                                                        x: coords.x,
                                                        y: coords.y,
                                                    });
                                                }
                                            },
                                            onmouseleave: move |_| *active_tooltip_clone_p.write() = None,
                                            onclick: move |e| {
                                                e.stop_propagation();
                                                if let Some(target) = &planet_target_click {
                                                    let coords = e.client_coordinates();
                                                    *selected_detail_clone_p.write() = Some(VedicTooltipData {
                                                        target: target.clone(),
                                                        x: coords.x,
                                                        y: coords.y,
                                                    });
                                                }
                                            },
                                            "{lbl}" 
                                        }
                                    }
                                })}
                            }
                        }
                    }
                })}
            }
        }
    } else {
        let cell_coords = [
            (12, 0, 0),    // Pisces
            (1, 100, 0),   // Aries
            (2, 200, 0),   // Taurus
            (3, 300, 0),   // Gemini
            (4, 300, 100),  // Cancer
            (5, 300, 200),  // Leo
            (6, 300, 300),  // Virgo
            (7, 200, 300),  // Libra
            (8, 100, 300),  // Scorpio
            (9, 0, 300),   // Sagittarius
            (10, 0, 200),  // Capricorn
            (11, 0, 100),  // Aquarius
        ];

        rsx! {
            svg {
                view_box: "0 0 400 400",
                class: "w-full h-auto max-w-[400px] aspect-square select-none font-mono mx-auto",
                rect {
                    width: "400",
                    height: "400",
                    fill: "#0b0f19",
                    rx: "12",
                    stroke: "#1e293b",
                    stroke_width: "2"
                }
                rect {
                    x: "100",
                    y: "100",
                    width: "200",
                    height: "200",
                    fill: "#0f172a",
                    stroke: "#1e293b",
                    stroke_width: "1.5"
                }
                text {
                    x: "200",
                    y: "185",
                    text_anchor: "middle",
                    font_size: "14px",
                    font_weight: "bold",
                    fill: "#f8fafc",
                    "{chart_title}"
                }
                text {
                    x: "200",
                    y: "215",
                    text_anchor: "middle",
                    font_size: "10px",
                    fill: "#475569",
                    "South Indian Style"
                }

                {cell_coords.iter().map(|&(r_id, cell_x, cell_y)| {
                    let is_lagna = lagna_rasi == r_id;
                    let bg_color = if is_lagna { "#131e35" } else { "#0b0f19" };
                    let border_color = if is_lagna { "#38bdf8" } else { "#1e293b" };
                    let r_planets = &rasi_planets[r_id as usize];
                    
                    let house_num = (r_id as i16 - lagna_rasi as i16 + 12) % 12 + 1;
                    
                    let house_target = get_house_tooltip(house_num as u8, lagna_rasi, chart_title, bhava_strengths);
                    
                    let mut active_tooltip_clone = active_tooltip.clone();
                    let mut selected_detail_clone = selected_detail.clone();
                    let house_target_hover = house_target.clone();
                    let house_target_click = house_target.clone();
                    
                    rsx! {
                        rect {
                            x: "{cell_x}",
                            y: "{cell_y}",
                            width: "100",
                            height: "100",
                            fill: "{bg_color}",
                            stroke: "{border_color}",
                            stroke_width: "1.5",
                            class: "cursor-pointer hover:fill-slate-800/20 transition-colors",
                            onmouseenter: move |e| {
                                if let Some(target) = &house_target_hover {
                                    let coords = e.client_coordinates();
                                    *active_tooltip_clone.write() = Some(VedicTooltipData {
                                        target: target.clone(),
                                        x: coords.x,
                                        y: coords.y,
                                    });
                                }
                            },
                            onmouseleave: move |_| *active_tooltip_clone.write() = None,
                            onclick: move |e| {
                                if let Some(target) = &house_target_click {
                                    let coords = e.client_coordinates();
                                    *selected_detail_clone.write() = Some(VedicTooltipData {
                                        target: target.clone(),
                                        x: coords.x,
                                        y: coords.y,
                                    });
                                }
                            }
                        }
                        foreignObject {
                            x: "{cell_x}",
                            y: "{cell_y}",
                            width: "100",
                            height: "100",
                            pointer_events: "none",
                            div { class: "p-1.5 flex flex-col justify-between h-full select-none",
                                span { class: "text-[9px] text-slate-500 font-bold", "{rasi_name_short(r_id)}" }
                                div { class: "grid grid-cols-2 gap-x-1 gap-y-0.5 text-[9px] pointer-events-auto",
                                    {r_planets.iter().map(|&(lbl, color_cls)| {
                                        let planet_target = if let Some(rows) = varga_rows {
                                            get_varga_planet_tooltip(lbl, rows, chart_title)
                                        } else if let (Some(planets), Some(asc)) = (d1_planets, d1_ascendant) {
                                            get_d1_planet_tooltip(lbl, planets, asc, chart_title)
                                        } else {
                                            None
                                        };
                                        
                                        let mut active_tooltip_clone_p = active_tooltip.clone();
                                        let mut selected_detail_clone_p = selected_detail.clone();
                                        let planet_target_hover = planet_target.clone();
                                        let planet_target_click = planet_target.clone();
                                        
                                        rsx! {
                                            span { 
                                                class: "{color_cls} text-center bg-slate-950/40 border border-slate-900/30 rounded py-0.5 cursor-pointer hover:bg-slate-800 transition-all", 
                                                onmouseenter: move |e| {
                                                    if let Some(target) = &planet_target_hover {
                                                        let coords = e.client_coordinates();
                                                        *active_tooltip_clone_p.write() = Some(VedicTooltipData {
                                                            target: target.clone(),
                                                            x: coords.x,
                                                            y: coords.y,
                                                        });
                                                    }
                                                },
                                                onmouseleave: move |_| *active_tooltip_clone_p.write() = None,
                                                onclick: move |e| {
                                                    e.stop_propagation();
                                                    if let Some(target) = &planet_target_click {
                                                        let coords = e.client_coordinates();
                                                        *selected_detail_clone_p.write() = Some(VedicTooltipData {
                                                            target: target.clone(),
                                                            x: coords.x,
                                                            y: coords.y,
                                                        });
                                                    }
                                                },
                                                "{lbl}" 
                                            }
                                        }
                                    })}
                                }
                            }
                        }
                    }
                })}
            }
        }
    }
}

fn render_detail_card(
    detail_opt: Signal<Option<VedicTooltipData>>,
) -> Element {
    let detail_val = detail_opt.read().clone();
    if let Some(detail) = detail_val {
        let mut detail_opt_clear = detail_opt.clone();
        match detail.target {
            VedicTooltipTarget::Planet {
                name,
                symbol,
                rasi_num,
                house_num,
                longitude_str,
                nakshatra_name,
                nakshatra_lord,
                is_retrograde,
                is_combust,
                varga_label,
                deity,
                purpose,
            } => {
                let sign_name = rasi_name(rasi_num);
                rsx! {
                    div { class: "bg-slate-950/60 border border-slate-800 rounded-xl p-4 mt-3 space-y-3.5 shadow-lg animate-in slide-in-from-bottom-2 duration-200 relative overflow-hidden",
                        div { class: "absolute -right-4 -bottom-4 text-slate-800/10 text-7xl font-bold select-none", "{symbol}" }
                        
                        div { class: "flex justify-between items-center border-b border-slate-900 pb-2",
                            div { class: "flex items-center gap-2",
                                span { class: "px-2 py-0.5 rounded text-[10px] bg-blue-950/50 text-blue-400 border border-blue-900/30 font-semibold", "{varga_label}" }
                                h4 { class: "font-bold text-slate-200 text-sm", "{name} 상세 정보" }
                            }
                            button {
                                class: "text-slate-500 hover:text-slate-300 text-xs transition-colors p-1",
                                onclick: move |_| *detail_opt_clear.write() = None,
                                "닫기 ✕"
                            }
                        }
                        
                        div { class: "grid grid-cols-2 sm:grid-cols-3 gap-3 text-xs",
                            div { class: "space-y-0.5",
                                span { class: "text-slate-500 text-[10px] block", "위치" }
                                span { class: "text-slate-300 font-medium", "{sign_name} / {house_num}하우스" }
                            }
                            div { class: "space-y-0.5",
                                span { class: "text-slate-500 text-[10px] block", "정밀 황경 (Longitude)" }
                                span { class: "text-slate-300 font-mono font-medium", "{longitude_str}" }
                            }
                            div { class: "space-y-0.5",
                                span { class: "text-slate-500 text-[10px] block", "나크샤트라 (Nakshatra)" }
                                span { class: "text-slate-300 font-medium", "{nakshatra_name}" }
                            }
                            div { class: "space-y-0.5",
                                span { class: "text-slate-500 text-[10px] block", "나크샤트라 지배성" }
                                span { class: "text-indigo-400 font-medium", "{nakshatra_lord}" }
                            }
                            if let Some(d) = deity {
                                div { class: "space-y-0.5",
                                    span { class: "text-slate-500 text-[10px] block", "수호신 (Deity)" }
                                    span { class: "text-amber-400 font-semibold", "{d}" }
                                }
                            }
                            if is_retrograde || is_combust {
                                div { class: "space-y-0.5",
                                    span { class: "text-slate-500 text-[10px] block", "특수 상태" }
                                    div { class: "flex gap-1",
                                        if is_retrograde {
                                            span { class: "px-1.5 py-0.5 rounded text-[9px] bg-purple-950 text-purple-400 border border-purple-900/40 font-bold", "역행" }
                                        }
                                        if is_combust {
                                            span { class: "px-1.5 py-0.5 rounded text-[9px] bg-orange-950 text-orange-400 border border-orange-900/40 font-bold", "태비(연소)" }
                                        }
                                    }
                                }
                            }
                        }
                        if let Some(p) = &purpose {
                            if !p.is_empty() {
                                div { class: "border-t border-slate-900 pt-2.5 text-xs text-slate-400 leading-normal max-w-full whitespace-normal",
                                    span { class: "text-slate-500 text-[10px] block mb-1", "우주적 의미 & 길흉 해석" }
                                    "{p}"
                                }
                            }
                        }
                    }
                }
            }
            VedicTooltipTarget::House {
                house_num,
                rasi_num: _,
                rasi_name,
                rasi_lord,
                score,
                varga_label,
            } => {
                rsx! {
                    div { class: "bg-slate-950/60 border border-slate-800 rounded-xl p-4 mt-3 space-y-3 shadow-lg animate-in slide-in-from-bottom-2 duration-200 relative overflow-hidden",
                        div { class: "absolute -right-4 -bottom-4 text-slate-800/10 text-7xl font-bold select-none", "H{house_num}" }
                        
                        div { class: "flex justify-between items-center border-b border-slate-900 pb-2",
                            div { class: "flex items-center gap-2",
                                span { class: "px-2 py-0.5 rounded text-[10px] bg-blue-950/50 text-blue-400 border border-blue-900/30 font-semibold", "{varga_label}" }
                                h4 { class: "font-bold text-slate-200 text-sm", "{house_num}하우스 상세 정보" }
                            }
                            button {
                                class: "text-slate-500 hover:text-slate-300 text-xs transition-colors p-1",
                                onclick: move |_| *detail_opt_clear.write() = None,
                                "닫기 ✕"
                            }
                        }
                        
                        div { class: "grid grid-cols-2 sm:grid-cols-3 gap-3 text-xs",
                            div { class: "space-y-0.5",
                                span { class: "text-slate-500 text-[10px] block", "지정 성좌 (Rasi Sign)" }
                                span { class: "text-slate-300 font-medium", "{rasi_name}" }
                            }
                            div { class: "space-y-0.5",
                                span { class: "text-slate-500 text-[10px] block", "성좌 지배성 (Lord)" }
                                span { class: "text-indigo-400 font-medium", "{rasi_lord}" }
                            }
                            if let Some(s) = score {
                                div { class: "space-y-0.5",
                                    span { class: "text-slate-500 text-[10px] block", "하우스 평점 (House Rating)" }
                                    span { class: "text-emerald-400 font-bold font-mono", "{s:.2} pts" }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        rsx! { div {} }
    }
}

fn render_floating_tooltip(
    tooltip_opt: Signal<Option<VedicTooltipData>>,
) -> Element {
    let tooltip_val = tooltip_opt.read().clone();
    if let Some(tooltip) = tooltip_val {
        let x_px = tooltip.x + 15.0;
        let y_px = tooltip.y + 15.0;
        
        match tooltip.target {
            VedicTooltipTarget::Planet {
                name,
                symbol,
                rasi_num,
                house_num,
                longitude_str,
                nakshatra_name,
                nakshatra_lord,
                is_retrograde,
                is_combust,
                varga_label,
                deity,
                ..
            } => {
                let sign_name = rasi_name(rasi_num);
                rsx! {
                    div { 
                        class: "fixed z-50 bg-slate-950/95 border border-slate-800/80 backdrop-blur-md rounded-lg p-3 shadow-2xl pointer-events-none text-xs text-slate-200 w-[220px] transition-all duration-75 space-y-1.5",
                        style: "left: {x_px}px; top: {y_px}px;",
                        div { class: "flex items-center justify-between border-b border-slate-900 pb-1.5",
                            span { class: "font-bold text-slate-100", "{name} ({symbol})" }
                            span { class: "text-[9px] text-blue-400 bg-blue-950/40 px-1.5 py-0.5 rounded border border-blue-900/30", "{varga_label}" }
                        }
                        div { class: "grid grid-cols-2 gap-x-2 gap-y-1 text-[10px]",
                            span { class: "text-slate-500", "위치:" }
                            span { class: "text-slate-300 font-medium text-right", "{sign_name} ({house_num}H)" }
                            span { class: "text-slate-500", "황경:" }
                            span { class: "text-slate-300 font-mono font-medium text-right", "{longitude_str}" }
                            span { class: "text-slate-500", "성수(Star):" }
                            span { class: "text-slate-300 font-medium text-right", "{nakshatra_name}" }
                            span { class: "text-slate-500", "성수주(Lord):" }
                            span { class: "text-indigo-400 font-medium text-right", "{nakshatra_lord}" }
                            if let Some(d) = deity {
                                if !d.is_empty() {
                                    span { class: "text-slate-500", "수호신:" }
                                    span { class: "text-amber-400 font-semibold text-right", "{d}" }
                                }
                            }
                        }
                        if is_retrograde || is_combust {
                            div { class: "flex gap-1 pt-1 border-t border-slate-900",
                                if is_retrograde {
                                    span { class: "px-1 py-0.5 rounded text-[8px] bg-purple-950 text-purple-400 border border-purple-900/40 font-bold", "역행" }
                                }
                                if is_combust {
                                    span { class: "px-1 py-0.5 rounded text-[8px] bg-orange-950 text-orange-400 border border-orange-900/40 font-bold", "태비" }
                                }
                            }
                        }
                    }
                }
            }
            VedicTooltipTarget::House {
                house_num,
                rasi_num: _,
                rasi_name,
                rasi_lord,
                score,
                varga_label,
            } => {
                rsx! {
                    div {
                        class: "fixed z-50 bg-slate-950/95 border border-slate-800/80 backdrop-blur-md rounded-lg p-3 shadow-2xl pointer-events-none text-xs text-slate-200 w-[180px] transition-all duration-75 space-y-1.5",
                        style: "left: {x_px}px; top: {y_px}px;",
                        div { class: "flex items-center justify-between border-b border-slate-900 pb-1.5",
                            span { class: "font-bold text-slate-100", "{house_num}하우스" }
                            span { class: "text-[9px] text-blue-400 bg-blue-950/40 px-1.5 py-0.5 rounded border border-blue-900/30", "{varga_label}" }
                        }
                        div { class: "grid grid-cols-2 gap-x-2 gap-y-1 text-[10px]",
                            span { class: "text-slate-500", "조디악 사인:" }
                            span { class: "text-slate-300 font-medium text-right", "{rasi_name}" }
                            span { class: "text-slate-500", "지배성(Lord):" }
                            span { class: "text-indigo-400 font-medium text-right", "{rasi_lord}" }
                            if let Some(s) = score {
                                span { class: "text-slate-500", "하우스 평점:" }
                                span { class: "text-emerald-400 font-bold font-mono text-right", "{s:.2} pts" }
                            }
                        }
                    }
                }
            }
        }
    } else {
        rsx! { div {} }
    }
}

#[component]
pub fn VedicTab() -> Element {
    let mut state = use_context::<AnalysisState>();
    let locale = *state.locale.read();
    
    // Sub-tab selection state: 0 = Basic D1, 1 = KP System, 2 = Dashas, 3 = Compatibility
    let mut active_subtab = use_signal(|| 0);
    let mut active_reduction_view = use_signal(|| 0);
    let mut chart_style = use_signal(|| "south".to_string());
    let mut varga_chart_style = use_signal(|| "south".to_string());
    let active_tooltip = use_signal(|| Option::<VedicTooltipData>::None);
    let selected_detail = use_signal(|| Option::<VedicTooltipData>::None);

    // Compatibility form states
    let mut partner_year = use_signal(|| 1992);
    let mut partner_month = use_signal(|| 8);
    let mut partner_day = use_signal(|| 24);
    let mut partner_hour = use_signal(|| 14);
    let mut partner_minute = use_signal(|| 30);
    let mut partner_lat = use_signal(|| 37.5665);
    let mut partner_lon = use_signal(|| 126.9780);
    let mut compat_status = use_signal(|| TaskStatus::Idle);
    let mut compat_data = use_signal(|| Option::<VedicCompatibilityOutput>::None);

    // Dasha inner selection: 0 = Vimshottari, 1 = Chara, 2 = Kala Chakra
    let mut active_dasha_type = use_signal(|| 0);
    let mut selected_varga = use_signal(|| "rasi".to_string());


    let run_analysis = move |_| {
        spawn(async move {
            state.vedic.write().status = TaskStatus::Loading;
            let form = state.form.read().clone();
            let input = VedicAnalysisInput::new(
                AnalysisInput {
                    year: form.year, month: form.month, day: form.day,
                    hour: form.hour, minute: form.minute,
                    is_lunar: form.is_lunar, is_leap_month: form.is_leap_month,
                    lat: form.lat, lon: form.lon,
                    timezone: "Asia/Seoul".to_string(),
                },
                Some(false), None,
            );
            match facade::analyze_vedic(input) {
                Ok(res) => {
                    state.vedic.write().data = Some(res);
                    state.vedic.write().status = TaskStatus::Success;
                }
                Err(e) => {
                    state.vedic.write().error = Some(e.to_string());
                    state.vedic.write().status = TaskStatus::Error(e.to_string());
                }
            }
        });
    };

    let run_compatibility = move |_| {
        spawn(async move {
            compat_status.write();
            *compat_status.write() = TaskStatus::Loading;
            let form = state.form.read().clone();
            let input = VedicCompatibilityInput {
                male: AnalysisInput {
                    year: form.year, month: form.month, day: form.day,
                    hour: form.hour, minute: form.minute,
                    is_lunar: form.is_lunar, is_leap_month: form.is_leap_month,
                    lat: form.lat, lon: form.lon,
                    timezone: "Asia/Seoul".to_string(),
                },
                female: AnalysisInput {
                    year: *partner_year.read(), month: *partner_month.read(), day: *partner_day.read(),
                    hour: *partner_hour.read(), minute: *partner_minute.read(),
                    is_lunar: false, is_leap_month: false,
                    lat: *partner_lat.read(), lon: *partner_lon.read(),
                    timezone: "Asia/Seoul".to_string(),
                }
            };
            match facade::analyze_vedic_compatibility(input) {
                Ok(res) => {
                    *compat_data.write() = Some(res);
                    *compat_status.write() = TaskStatus::Success;
                }
                Err(e) => {
                    *compat_status.write() = TaskStatus::Error(e.to_string());
                }
            }
        });
    };

    rsx! {
        div { class: "space-y-6 animate-in fade-in duration-700",
            BirthForm {}

            div { class: "flex justify-between items-center",
                h2 { class: "text-2xl font-bold bg-gradient-to-r from-blue-200 to-indigo-400 bg-clip-text text-transparent",
                    "{t(locale, TK::SectionVedicChart)}"
                }
                button {
                    class: "px-5 py-2.5 bg-gradient-to-r from-blue-700 to-indigo-700 hover:from-blue-600 hover:to-indigo-600 rounded-xl font-semibold text-white shadow-lg shadow-blue-900/30 transition-all duration-200 active:scale-95",
                    onclick: run_analysis,
                    "{t(locale, TK::BtnCalculate)} 🔭"
                }
            }

            match &state.vedic.read().status {
                TaskStatus::Idle => rsx! {
                    div { class: "flex flex-col items-center justify-center py-20 gap-3 text-slate-500",
                        span { class: "text-5xl", "🪐" }
                        p { class: "text-lg font-medium", "{t(locale, TK::StatusIdleHint)}" }
                    }
                },
                TaskStatus::Loading => rsx! {
                    div { class: "flex flex-col items-center justify-center py-20 gap-3",
                        div { class: "w-12 h-12 rounded-full border-4 border-blue-500/30 border-t-blue-400 animate-spin" }
                        p { class: "text-blue-400 font-medium animate-pulse", "{t(locale, TK::StatusLoadingVedic)}" }
                    }
                },
                TaskStatus::Error(e) => rsx! {
                    div { class: "p-4 rounded-xl bg-red-900/20 border border-red-800/50 text-red-400", "{t(locale, TK::StatusError)}: {e}" }
                },
                TaskStatus::Success => {
                    if let Some(data) = &state.vedic.read().data {
                        rsx! {
                            // ── 서브 탭 네비게이션 ────────────────────────────────
                            div { class: "flex border-b border-slate-800 gap-2 overflow-x-auto",
                                button {
                                    class: if *active_subtab.read() == 0 {
                                        "px-4 py-2.5 font-medium text-sm transition-colors border-b-2 border-blue-500 text-blue-400"
                                    } else {
                                        "px-4 py-2.5 font-medium text-sm transition-colors border-b-2 border-transparent text-slate-400 hover:text-slate-200"
                                    },
                                    onclick: move |_| *active_subtab.write() = 0,
                                    "✨ D1 라시 & 판창가"
                                }
                                button {
                                    class: if *active_subtab.read() == 1 {
                                        "px-4 py-2.5 font-medium text-sm transition-colors border-b-2 border-blue-500 text-blue-400"
                                    } else {
                                        "px-4 py-2.5 font-medium text-sm transition-colors border-b-2 border-transparent text-slate-400 hover:text-slate-200"
                                    },
                                    onclick: move |_| *active_subtab.write() = 1,
                                    "📐 KP unequal 하우스"
                                }
                                button {
                                    class: if *active_subtab.read() == 2 {
                                        "px-4 py-2.5 font-medium text-sm transition-colors border-b-2 border-blue-500 text-blue-400"
                                    } else {
                                        "px-4 py-2.5 font-medium text-sm transition-colors border-b-2 border-transparent text-slate-400 hover:text-slate-200"
                                    },
                                    onclick: move |_| *active_subtab.write() = 2,
                                    "📅 정밀 다샤 타임라인"
                                }
                                button {
                                    class: if *active_subtab.read() == 3 {
                                        "px-4 py-2.5 font-medium text-sm transition-colors border-b-2 border-blue-500 text-blue-400"
                                    } else {
                                        "px-4 py-2.5 font-medium text-sm transition-colors border-b-2 border-transparent text-slate-400 hover:text-slate-200"
                                    },
                                    onclick: move |_| *active_subtab.write() = 3,
                                    "💞 Ashtakoota 궁합매칭"
                                }
                                button {
                                    class: if *active_subtab.read() == 4 {
                                        "px-4 py-2.5 font-medium text-sm transition-colors border-b-2 border-blue-500 text-blue-400"
                                    } else {
                                        "px-4 py-2.5 font-medium text-sm transition-colors border-b-2 border-transparent text-slate-400 hover:text-slate-200"
                                    },
                                    onclick: move |_| *active_subtab.write() = 4,
                                    "🪐 고차라 & 사데사티"
                                }
                                if data.tajika_report.is_some() {
                                    button {
                                        class: if *active_subtab.read() == 5 {
                                            "px-4 py-2.5 font-medium text-sm transition-colors border-b-2 border-blue-500 text-blue-400"
                                        } else {
                                            "px-4 py-2.5 font-medium text-sm transition-colors border-b-2 border-transparent text-slate-400 hover:text-slate-200"
                                        },
                                        onclick: move |_| *active_subtab.write() = 5,
                                        "📅 타지카 연간운세"
                                    }
                                }
                                button {
                                    class: if *active_subtab.read() == 6 {
                                        "px-4 py-2.5 font-medium text-sm transition-colors border-b-2 border-blue-500 text-blue-400"
                                    } else {
                                        "px-4 py-2.5 font-medium text-sm transition-colors border-b-2 border-transparent text-slate-400 hover:text-slate-200"
                                    },
                                    onclick: move |_| *active_subtab.write() = 6,
                                    "🔮 D1~D144 분할차트"
                                }
                                button {
                                    class: if *active_subtab.read() == 7 {
                                        "px-4 py-2.5 font-medium text-sm transition-colors border-b-2 border-blue-500 text-blue-400"
                                    } else {
                                        "px-4 py-2.5 font-medium text-sm transition-colors border-b-2 border-transparent text-slate-400 hover:text-slate-200"
                                    },
                                    onclick: move |_| *active_subtab.write() = 7,
                                    "📝 분할차트 상세 해석 (D9/D10)"
                                }
                                button {
                                    class: if *active_subtab.read() == 8 {
                                        "px-4 py-2.5 font-medium text-sm transition-colors border-b-2 border-blue-500 text-blue-400"
                                    } else {
                                        "px-4 py-2.5 font-medium text-sm transition-colors border-b-2 border-transparent text-slate-400 hover:text-slate-200"
                                    },
                                    onclick: move |_| *active_subtab.write() = 8,
                                    "📊 아쉬타카바르가"
                                }
                            }

                            // ── 서브 탭 콘텐츠 ─────────────────────────────────
                            match *active_subtab.read() {
                                0 => rsx! {
                                    // D1 Rasi & Basic Info
                                    div { class: "space-y-6",
                                        // ── 베딕 운명 프로필 요약 (Vedic Destiny Profile Summary) ─────────
                                        div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5 space-y-3.5 shadow-xl",
                                            div { class: "flex justify-between items-center border-b border-slate-800/60 pb-2.5 flex-wrap gap-2",
                                                h3 { class: "font-semibold text-slate-200 text-sm uppercase tracking-wider", "베딕 운명 지표 요약 (Vedic Destiny Overview)" }
                                                span { class: "text-xs font-bold text-indigo-400 bg-indigo-950/20 px-3 py-1 rounded border border-indigo-900/30",
                                                    "종합 활성도 점수 (Overall Strength): {data.report.overall_strength_score:.1}점"
                                                }
                                            }
                                            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                                div { class: "p-3 rounded-xl bg-slate-800/20 border border-slate-800/60 space-y-1",
                                                    p { class: "text-[10px] text-slate-500 font-bold uppercase tracking-wider", "출생 성좌(Nakshatra) 운명 특징" }
                                                    p { class: "text-xs text-slate-350 leading-relaxed font-semibold", "{data.report.nakshatra_info}" }
                                                }
                                                div { class: "p-3 rounded-xl bg-slate-800/20 border border-slate-800/60 space-y-1",
                                                    p { class: "text-[10px] text-slate-500 font-bold uppercase tracking-wider", "현재 대운/소운 주기 포커스" }
                                                    p { class: "text-xs text-slate-350 leading-relaxed font-semibold", "{data.report.dasha_focus}" }
                                                }
                                            }
                                        }

                                        div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                                            // 1) Lagna Badge
                                            div { class: "flex items-center gap-4 p-5 bg-slate-900 border border-slate-800 rounded-2xl",
                                                div { class: "text-4xl", "⬆️" }
                                                div {
                                                    p { class: "text-xs text-slate-500 font-semibold tracking-wider", "라그나 (Lagna)" }
                                                    p { class: "text-xl font-bold text-indigo-300 mt-1", "{rasi_name(data.chart.ascendant.rasi)}" }
                                                    p { class: "text-xs text-slate-400 mt-0.5", "pada {data.chart.ascendant.pada} | {data.chart.ascendant.sidereal_deg:.2}°" }
                                                    p { class: "text-[10px] text-slate-500 font-mono mt-0.5", "Ayanamsa: {data.chart.ayanamsa:.4}°" }
                                                }
                                            }
                                            // 2) Yogi Point
                                            div { class: "flex items-center gap-4 p-5 bg-slate-900 border border-slate-800 rounded-2xl",
                                                div { class: "text-4xl", "🧘" }
                                                div {
                                                    p { class: "text-xs text-slate-500 font-semibold tracking-wider", "요기 포인트 (부/길성)" }
                                                    p { class: "text-xl font-bold text-emerald-400 mt-1", "{planet_name_kr(data.chart.panchanga.yogi_planet)}" }
                                                    p { class: "text-xs text-slate-400 mt-0.5", "Yogi: {data.chart.panchanga.yogi_point:.2}°" }
                                                }
                                            }
                                            // 3) Avayogi Point
                                            div { class: "flex items-center gap-4 p-5 bg-slate-900 border border-slate-800 rounded-2xl",
                                                div { class: "text-4xl", "⚡" }
                                                div {
                                                    p { class: "text-xs text-slate-500 font-semibold tracking-wider", "아바요기 포인트 (흉성)" }
                                                    p { class: "text-xl font-bold text-rose-400 mt-1", "{planet_name_kr(data.chart.panchanga.avayogi_planet)}" }
                                                    p { class: "text-xs text-slate-400 mt-0.5", "Dagdha: {data.chart.panchanga.dagdha_rashis.iter().map(|&r| rasi_name(r)).collect::<Vec<_>>().join(\", \")}" }
                                                }
                                            }
                                        }

                                        // ── 베딕 천도 배치도 (Vedic Birth Chart SVG) ──────────────────────
                                        {
                                            let mut rasi_planets: Vec<Vec<(&'static str, &'static str)>> = vec![vec![]; 13];
                                            let mut house_planets: Vec<Vec<(&'static str, &'static str)>> = vec![vec![]; 13];

                                            house_planets[1].push(("Asc", "text-sky-400 font-bold"));
                                            if data.chart.ascendant.rasi >= 1 && data.chart.ascendant.rasi <= 12 {
                                                rasi_planets[data.chart.ascendant.rasi as usize].push(("Asc", "text-sky-400 font-bold"));
                                            }

                                            for p in &data.chart.planets {
                                                let p_lbl = match p.planet {
                                                    VedicPlanet::Sun => "Su",
                                                    VedicPlanet::Moon => "Mo",
                                                    VedicPlanet::Mars => "Ma",
                                                    VedicPlanet::Mercury => "Me",
                                                    VedicPlanet::Jupiter => "Ju",
                                                    VedicPlanet::Venus => "Ve",
                                                    VedicPlanet::Saturn => "Sa",
                                                    VedicPlanet::Rahu => "Ra",
                                                    VedicPlanet::Ketu => "Ke",
                                                    VedicPlanet::Ascendant => "Asc",
                                                };
                                                let p_color = match p.planet {
                                                    VedicPlanet::Sun => "text-orange-400 font-bold",
                                                    VedicPlanet::Moon => "text-slate-355 font-bold",
                                                    VedicPlanet::Mars => "text-red-400 font-bold",
                                                    VedicPlanet::Mercury => "text-emerald-400 font-bold",
                                                    VedicPlanet::Jupiter => "text-yellow-400 font-bold",
                                                    VedicPlanet::Venus => "text-pink-400 font-bold",
                                                    VedicPlanet::Saturn => "text-indigo-400 font-bold",
                                                    VedicPlanet::Rahu => "text-purple-400 font-bold",
                                                    VedicPlanet::Ketu => "text-amber-600 font-bold",
                                                    VedicPlanet::Ascendant => "text-sky-400 font-bold",
                                                };
                                                if p_lbl != "Asc" {
                                                    if p.rasi >= 1 && p.rasi <= 12 {
                                                        rasi_planets[p.rasi as usize].push((p_lbl, p_color));
                                                    }
                                                    if p.house_index >= 1 && p.house_index <= 12 {
                                                        house_planets[p.house_index as usize].push((p_lbl, p_color));
                                                    }
                                                }
                                            }

                                            let cur_style = chart_style.read().clone();
                                            rsx! {
                                                div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5 space-y-4 shadow-xl animate-in fade-in duration-500",
                                                    div { class: "flex justify-between items-center border-b border-slate-800/60 pb-3 flex-wrap gap-2",
                                                        div { class: "space-y-0.5",
                                                            h3 { class: "font-semibold text-slate-200 text-sm uppercase tracking-wider", "베딕 천도 배치도 (Vedic Birth Chart)" }
                                                            p { class: "text-xs text-slate-500", "라시 차트(D1)의 각 행성 및 라그나 배치도 시각화" }
                                                        }
                                                        div { class: "flex items-center gap-1.5 bg-slate-950 p-1 rounded-xl border border-slate-850",
                                                            button {
                                                                class: if cur_style == "south" {
                                                                    "px-3 py-1.5 text-xs font-bold text-blue-400 bg-slate-850 rounded-lg transition-colors border border-slate-700/50"
                                                                } else {
                                                                    "px-3 py-1.5 text-xs font-semibold text-slate-400 hover:text-slate-200 transition-colors"
                                                                },
                                                                onclick: move |_| *chart_style.write() = "south".to_string(),
                                                                "남인도식 (South)"
                                                            }
                                                            button {
                                                                class: if cur_style == "north" {
                                                                    "px-3 py-1.5 text-xs font-bold text-blue-400 bg-slate-855 rounded-lg transition-colors border border-slate-700/50"
                                                                } else {
                                                                    "px-3 py-1.5 text-xs font-semibold text-slate-400 hover:text-slate-200 transition-colors"
                                                                },
                                                                onclick: move |_| *chart_style.write() = "north".to_string(),
                                                                "북인도식 (North)"
                                                            }
                                                        }
                                                    }
                                                                                    div { class: "py-2 flex flex-col items-center justify-center w-full",
                                                        {render_vedic_chart(
                                                            &rasi_planets,
                                                            &house_planets,
                                                            data.chart.ascendant.rasi,
                                                            &cur_style,
                                                            "Rasi D1",
                                                            active_tooltip,
                                                            selected_detail,
                                                            None,
                                                            Some(&data.chart.planets),
                                                            Some(&data.chart.ascendant),
                                                            Some(&data.chart.bhava_strengths),
                                                        )}
                                                        div { class: "w-full max-w-[400px] mx-auto",
                                                            {render_detail_card(selected_detail)}
                                                        }
                                                    }
                                                }
                                            }
                                        }

                                        // ── 5대 판창가 (Panchanga Limbs & Solar Info) ─────────────────
                                        div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5 space-y-4 shadow-xl",
                                            div { class: "flex items-center gap-2 border-b border-slate-800/60 pb-2.5",
                                                span { class: "text-xl", "📅" }
                                                h3 { class: "font-semibold text-slate-200 text-sm uppercase tracking-wider", "5대 판창가 (Panchanga Limbs & Solar Info)" }
                                                span { class: "text-xs text-slate-500", "출생 시간 기준 우주적 다섯 요소 및 천문 정보" }
                                            }
                                            div { class: "grid grid-cols-2 sm:grid-cols-3 md:grid-cols-5 gap-3.5",
                                                div { class: "p-3 rounded-xl bg-slate-800/20 border border-slate-800/60 flex flex-col gap-1",
                                                    span { class: "text-[10px] text-slate-500 font-semibold tracking-wider", "요일 (Vara)" }
                                                    span { class: "text-sm font-bold text-slate-200", "{data.chart.panchanga.vara}" }
                                                    span { class: "text-[9px] text-slate-400", "지배성: {planet_name_kr(data.chart.panchanga.day_lord)}" }
                                                }
                                                div { class: "p-3 rounded-xl bg-slate-800/20 border border-slate-800/60 flex flex-col gap-1",
                                                    span { class: "text-[10px] text-slate-500 font-semibold tracking-wider", "티티 (Tithi)" }
                                                    span { class: "text-sm font-bold text-slate-200", "{data.chart.panchanga.tithi_name}" }
                                                    span { class: "text-[9px] text-slate-400 font-mono", "Tithi #{data.chart.panchanga.tithi}" }
                                                }
                                                div { class: "p-3 rounded-xl bg-slate-800/20 border border-slate-800/60 flex flex-col gap-1",
                                                    span { class: "text-[10px] text-slate-500 font-semibold tracking-wider", "나크샤트라 (Nakshatra)" }
                                                    span { class: "text-sm font-bold text-slate-200", "{nakshatra_name(data.chart.panchanga.nakshatra)}" }
                                                    span { class: "text-[9px] text-slate-405 font-mono", "Nakshatra #{data.chart.panchanga.nakshatra}" }
                                                }
                                                div { class: "p-3 rounded-xl bg-slate-800/20 border border-slate-800/60 flex flex-col gap-1",
                                                    span { class: "text-[10px] text-slate-500 font-semibold tracking-wider", "요가 (Nitya Yoga)" }
                                                    span { class: "text-sm font-bold text-slate-200", "Yoga {data.chart.panchanga.yoga}" }
                                                    span { class: "text-[9px] text-slate-500", "27대 요가 요소" }
                                                }
                                                div { class: "p-3 rounded-xl bg-slate-800/20 border border-slate-800/60 flex flex-col gap-1",
                                                    span { class: "text-[10px] text-slate-500 font-semibold tracking-wider", "카라나 (Karana)" }
                                                    span { class: "text-sm font-bold text-slate-200", "{data.chart.panchanga.karana_name}" }
                                                    span { class: "text-[9px] text-slate-405 font-mono", "Karana #{data.chart.panchanga.karana}" }
                                                }
                                            }
                                            {
                                                let form = state.form.read();
                                                let local_timezone = "Asia/Seoul";
                                                let tz_res: Result<chrono_tz::Tz, _> = local_timezone.parse();
                                                let formatted_sunrise = if let Ok(tz) = tz_res {
                                                    data.chart.panchanga.sunrise.with_timezone(&tz).format("%H:%M:%S").to_string()
                                                } else {
                                                    data.chart.panchanga.sunrise.format("%H:%M:%S").to_string()
                                                };
                                                let formatted_sunset = if let Ok(tz) = tz_res {
                                                    data.chart.panchanga.sunset.with_timezone(&tz).format("%H:%M:%S").to_string()
                                                } else {
                                                    data.chart.panchanga.sunset.format("%H:%M:%S").to_string()
                                                };
                                                let birth_time_lbl = format!("{:02}:{:02}", form.hour, form.minute);
                                                let birth_day_or_night = if data.chart.panchanga.is_day_birth { "낮 출생 ☀️" } else { "밤 출생 🌙" };
                                                rsx! {
                                                    div { class: "p-3.5 rounded-xl bg-slate-900/40 border border-slate-800/80 flex flex-wrap gap-x-6 gap-y-2 text-xs text-slate-400 font-mono justify-between items-center",
                                                        div { "출생 시각: "
                                                            span { class: "text-slate-200 font-bold", "{form.year}년 {form.month}월 {form.day}일 {birth_time_lbl}" }
                                                        }
                                                        div { "출생지: "
                                                            span { class: "text-slate-200 font-bold", "위도 {form.lat:.4}° / 경도 {form.lon:.4}°" }
                                                        }
                                                        div { "출생 시간대: "
                                                            span { class: "text-indigo-400 font-bold", "{birth_day_or_night}" }
                                                        }
                                                        div { "일출: "
                                                            span { class: "text-orange-400 font-bold", "{formatted_sunrise}" }
                                                        }
                                                        div { "일몰: "
                                                            span { class: "text-indigo-400 font-bold", "{formatted_sunset}" }
                                                        }
                                                        div { "시간 지배성 (Hora Lord): "
                                                            span { class: "text-yellow-400 font-bold", "{planet_name_kr(data.chart.panchanga.hour_lord)}" }
                                                        }
                                                    }
                                                }
                                            }
                                        }

                                        // ── 12개 아루다 파다 (Arudha Padas - 영역별 평판/거울 이미지) ───────────
                                        div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5 space-y-4 shadow-xl",
                                            div { class: "flex items-center gap-2 border-b border-slate-800/60 pb-2.5",
                                                span { class: "text-xl", "🪞" }
                                                h3 { class: "font-semibold text-slate-200 text-sm uppercase tracking-wider", "12대 아루다 파다 (Arudha Padas — 사회적 평판 및 거울)" }
                                                span { class: "text-xs text-slate-500", "현실 세계에 투영되는 나의 평판과 인생 영역별 실체" }
                                            }
                                            div { class: "grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-3",
                                                {data.chart.arudha_padas.iter().map(|ap| {
                                                    let purpose = match ap.house {
                                                        1 => "AL (사회적 지위, 평판, 인상)",
                                                        2 => "A2 (재물 상태, 재산, 언어력)",
                                                        3 => "A3 (형제 자매, 모험력, 주관)",
                                                        4 => "A4 (가정, 어머니, 평화, 자산)",
                                                        5 => "A5 (자녀, 학업, 투자, 지혜)",
                                                        6 => "A6 (부채, 건강, 경쟁, 하수인)",
                                                        7 => "A7 (배우자 성향, 대인 파트너십)",
                                                        8 => "A8 (급격한 기복, 수명, 영적 지식)",
                                                        9 => "A9 (상생, 부친, 종교, 고등 교육)",
                                                        10 => "A10 (사회적 활동성, 직업적 명성)",
                                                        11 => "A11 (재물 획득, 소원 성취, 동료)",
                                                        12 => "UL (결혼 지속성, 해탈, 사생활)",
                                                        _ => "기타 인생 영역",
                                                    };
                                                    let border_cls = if ap.house == 1 {
                                                        "border-amber-950/60 bg-amber-950/10 hover:border-amber-900/60"
                                                    } else if ap.house == 12 {
                                                        "border-pink-950/60 bg-pink-950/10 hover:border-pink-900/60"
                                                    } else {
                                                        "border-slate-800 bg-slate-800/20 hover:border-slate-750"
                                                    };
                                                    let label_cls = if ap.house == 1 {
                                                        "text-amber-300"
                                                    } else if ap.house == 12 {
                                                        "text-pink-300"
                                                    } else {
                                                        "text-indigo-300"
                                                    };
                                                    rsx! {
                                                        div { class: "p-3 rounded-xl border flex flex-col justify-between gap-1 transition-all duration-300 {border_cls}",
                                                            p { class: "text-[10px] text-slate-500 font-semibold tracking-wider", "{purpose}" }
                                                            p { class: "text-sm font-extrabold {label_cls} mt-0.5", "{ap.name}" }
                                                            p { class: "text-[10px] text-slate-400 mt-1 font-semibold", "성좌: {rasi_name(ap.rasi)}" }
                                                        }
                                                    }
                                                })}
                                            }
                                        }

                                        // ── 아루다, 우파파다 및 특수 라그나 요약 ─────────────────
                                        div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                                            // 4) Arudha Lagna (AL)
                                            div { class: "flex items-center gap-4 p-5 bg-slate-900 border border-slate-800 rounded-2xl",
                                                div { class: "text-4xl", "👑" }
                                                div {
                                                    p { class: "text-xs text-slate-500 font-semibold tracking-wider", "아루다 라그나 (AL - 외부 사회적 지위)" }
                                                    p { class: "text-xl font-bold text-amber-400 mt-1", "{rasi_name(data.report.arudha_lagna)}" }
                                                    p { class: "text-xs text-slate-400 mt-0.5", "사회적 명성 및 타인에게 보여지는 평판" }
                                                }
                                            }
                                            // 5) Upapada Lagna (UL)
                                            div { class: "flex items-center gap-4 p-5 bg-slate-900 border border-slate-800 rounded-2xl",
                                                div { class: "text-4xl", "💍" }
                                                div {
                                                    p { class: "text-xs text-slate-500 font-semibold tracking-wider", "우파파다 라그나 (UL - 배우자/결혼)" }
                                                    p { class: "text-xl font-bold text-pink-400 mt-1", "{rasi_name(data.report.upapada_lagna)}" }
                                                    p { class: "text-xs text-slate-400 mt-0.5", "실제 배우자복과 혼인 인연의 에너지" }
                                                }
                                            }
                                            // 6) Special Lagnas Summary
                                            div { class: "flex items-center gap-4 p-5 bg-slate-900 border border-slate-800 rounded-2xl",
                                                div { class: "text-4xl", "🔮" }
                                                div { class: "w-full",
                                                    p { class: "text-xs text-slate-500 font-semibold tracking-wider mb-1.5", "주요 특수 라그나 (Special Lagnas)" }
                                                    div { class: "flex flex-wrap gap-1.5",
                                                        {data.report.special_lagnas_summary.iter().map(|(name, rasi)| {
                                                            let short_name = match name.as_str() {
                                                                "Shri Lagna" => "SL (번영)",
                                                                "Indu Lagna" => "IL (부)",
                                                                "Hora Lagna" => "HL (자산)",
                                                                "Ghati Lagna" => "GL (지위)",
                                                                "Pranapada Lagna" => "PL (생명)",
                                                                _ => name.as_str(),
                                                            };
                                                            rsx! {
                                                                span { class: "text-[10px] font-bold px-1.5 py-0.5 bg-slate-800 border border-slate-700/60 rounded text-slate-300",
                                                                    "{short_name}: {rasi_name(*rasi)}"
                                                                }
                                                            }
                                                        })}
                                                    }
                                                }
                                            }
                                        }

                                        // Planets positions & Lajjitadi Avasthas Table
                                        div { class: "bg-slate-900 border border-slate-800 rounded-2xl overflow-hidden",
                                            div { class: "bg-slate-800/50 border-b border-slate-800 px-5 py-3",
                                                h3 { class: "font-semibold text-slate-200", "D1 라시 차트 — 행성 상세 및 아바스타 (Avasthas)" }
                                            }
                                            div { class: "overflow-x-auto",
                                                table { class: "w-full text-sm",
                                                    thead {
                                                        tr { class: "bg-slate-800/30 text-xs text-slate-400 uppercase",
                                                            th { class: "px-4 py-3 text-left font-medium", "행성" }
                                                            th { class: "px-4 py-3 text-left font-medium", "라시 (Sign)" }
                                                            th { class: "px-4 py-3 text-left font-medium", "황경" }
                                                            th { class: "px-4 py-3 text-left font-medium", "나크샤트라" }
                                                            th { class: "px-4 py-3 text-left font-medium", "하우스" }
                                                            th { class: "px-4 py-3 text-left font-medium", "애스펙트 (Drishti)" }
                                                            th { class: "px-4 py-3 text-left font-medium", "빔쇼파카 강도" }
                                                            th { class: "px-4 py-3 text-left font-medium", "라지타디 Avastha" }
                                                            th { class: "px-4 py-3 text-center font-medium", "태비/역행" }
                                                        }
                                                    }
                                                    tbody { class: "divide-y divide-slate-800",
                                                        {data.chart.planets.iter().map(|p| {
                                                            let color = planet_color(p.planet);
                                                            let av = data.chart.avasthas.iter().find(|a| a.planet == p.planet);
                                                            let aspect_rel = data.chart.aspects.iter().find(|a| a.aspecting_planet == p.planet);
                                                            let aspect_str = if let Some(a) = aspect_rel {
                                                                a.aspected_houses.iter().map(|h| format!("H{}", h)).collect::<Vec<_>>().join(", ")
                                                            } else {
                                                                "없음".to_string()
                                                            };
                                                            let v_score_opt = data.chart.vimshopaka_scores.iter().find(|(pl, _)| *pl == p.planet);
                                                            let v_score_str = if let Some((_, vs)) = v_score_opt {
                                                                format!("{:.1} / 20", vs.shadvarga_score)
                                                            } else {
                                                                "—".to_string()
                                                            };
                                                            rsx! {
                                                                tr { class: "hover:bg-slate-800/20 transition-colors",
                                                                    td { class: "px-4 py-3 font-bold {color}", "{planet_name_kr(p.planet)}" }
                                                                    td { class: "px-4 py-3 text-slate-300", "{rasi_name(p.rasi)}" }
                                                                    td { class: "px-4 py-3 font-mono text-slate-400 text-xs", "{p.sidereal_deg:.2}°" }
                                                                    td { class: "px-4 py-3 text-slate-400 text-xs", "{nakshatra_name(p.nakshatra)} P{p.pada}" }
                                                                    td { class: "px-4 py-3 text-slate-400 text-xs font-mono", "H{p.house_index}" }
                                                                    td { class: "px-4 py-3 text-slate-450 font-mono text-xs", "{aspect_str}" }
                                                                    td { class: "px-4 py-3 text-indigo-400 font-mono text-xs font-bold", "{v_score_str}" }
                                                                    td { class: "px-4 py-3",
                                                                        if let Some(a) = av {
                                                                            span { class: "px-2 py-0.5 rounded-full text-xs font-semibold {lajjitadi_color(&a.lajjitadi)}",
                                                                                "{lajjitadi_name_kr(&a.lajjitadi)}"
                                                                            }
                                                                        }
                                                                    }
                                                                    td { class: "px-4 py-3 text-center",
                                                                        if p.is_retrograde {
                                                                            span { class: "px-1.5 py-0.5 rounded text-xs bg-purple-900/50 text-purple-300 border border-purple-700/50 mr-1", "R" }
                                                                        }
                                                                        if p.is_combust {
                                                                            span { class: "px-1.5 py-0.5 rounded text-xs bg-orange-900/50 text-orange-300 border border-orange-700/50", "C" }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        })}
                                                    }
                                                }
                                            }
                                        }

                                        // Yogas & Karakas Grid
                                        div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                            div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5 space-y-3",
                                                h3 { class: "text-sm font-semibold text-slate-400 uppercase tracking-widest", "감지된 요가 (Vedic Yogas)" }
                                                div { class: "space-y-2",
                                                    {data.report.yogas.iter().take(5).map(|y| rsx! {
                                                        div { class: "p-3 rounded-xl bg-slate-800/50 border border-slate-700/50",
                                                            p { class: "font-semibold text-sm text-indigo-300", "{y.name}" }
                                                            p { class: "text-xs text-slate-400 mt-1", "{y.description}" }
                                                        }
                                                    })}
                                                }
                                            }
                                            div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5 space-y-3",
                                                h3 { class: "text-sm font-semibold text-slate-400 uppercase tracking-widest", "자이미니 8대 카라카 (Jaimini Karakas)" }
                                                div { class: "grid grid-cols-2 gap-3",
                                                    {data.report.all_karakas.iter().map(|k| {
                                                        let label = match k.role {
                                                            eon_vedic::analysis::jaimini::JaiminiKarakaRole::Atmakaraka => "아트마카라카 (영혼/AK)",
                                                            eon_vedic::analysis::jaimini::JaiminiKarakaRole::Amatyakaraka => "아마티야카라카 (직업/AmK)",
                                                            eon_vedic::analysis::jaimini::JaiminiKarakaRole::Bhratrukaraka => "브라뜨루카라카 (형제/BK)",
                                                            eon_vedic::analysis::jaimini::JaiminiKarakaRole::Matrukaraka => "마트루카라카 (어머니/MK)",
                                                            eon_vedic::analysis::jaimini::JaiminiKarakaRole::Pitrikaraka => "피트리카라카 (아버지/PiK)",
                                                            eon_vedic::analysis::jaimini::JaiminiKarakaRole::Putrakaraka => "뿌뜨라카라카 (자녀/PK)",
                                                            eon_vedic::analysis::jaimini::JaiminiKarakaRole::Gnatikaraka => "냐티카라카 (경쟁자/GK)",
                                                            eon_vedic::analysis::jaimini::JaiminiKarakaRole::Darakaraka => "다라카라카 (배우자/DK)",
                                                        }.to_string();
                                                        rsx! {
                                                            KarakaCard { label: label, planet: k.planet }
                                                        }
                                                    })}
                                                }
                                            }
                                        }

                                        // ── 하우스별 활성도 & 성향 강도 분석 (House Ratings) ───────
                                        div { class: "bg-slate-900 border border-slate-800 rounded-2xl overflow-hidden shadow-xl",
                                            div { class: "bg-slate-800/50 border-b border-slate-800 px-5 py-3.5 flex justify-between items-center flex-wrap gap-2",
                                                h3 { class: "font-semibold text-slate-200 text-sm uppercase tracking-widest", "하우스별 성향 및 에너지 강도 (House Ratings)" }
                                                span { class: "text-xs text-slate-500", "총 12개 하우스의 정량적 강도 및 특수 분석 결과" }
                                            }
                                            div { class: "overflow-x-auto",
                                                table { class: "w-full text-sm",
                                                    thead {
                                                        tr { class: "bg-slate-800/30 text-xs text-slate-400 uppercase border-b border-slate-800",
                                                            th { class: "px-4 py-3 text-left font-medium w-24", "하우스" }
                                                            th { class: "px-4 py-3 text-left font-medium w-48", "담당 생애 영역" }
                                                            th { class: "px-4 py-3 text-left font-medium w-32", "에너지 등급" }
                                                            th { class: "px-4 py-3 text-left font-medium w-24", "강도 점수" }
                                                            th { class: "px-4 py-3 text-left font-medium", "에너지 판별 근거" }
                                                        }
                                                    }
                                                    tbody { class: "divide-y divide-slate-800",
                                                        {data.report.house_summary.iter().map(|h| {
                                                            let rating_cls = match h.rating.as_str() {
                                                                "Excellent" => "text-emerald-400 bg-emerald-950/20 border-emerald-900/30",
                                                                "Strong" => "text-indigo-400 bg-indigo-950/20 border-indigo-900/30",
                                                                "Average" => "text-slate-350 bg-slate-800/40 border-slate-700/30",
                                                                "Weak" => "text-rose-400 bg-rose-950/20 border-rose-900/30",
                                                                _ => "text-slate-400 bg-slate-800/20 border-slate-800",
                                                            };
                                                            let rating_kr = match h.rating.as_str() {
                                                                "Excellent" => "최상 (Excellent)",
                                                                "Strong" => "강함 (Strong)",
                                                                "Average" => "보통 (Average)",
                                                                "Weak" => "약함 (Weak)",
                                                                _ => &h.rating,
                                                            };
                                                            let house_purpose_kr = match h.house {
                                                                1 => "자아, 기질 및 전반적 생명력",
                                                                2 => "재물운, 지지 기반 및 언어능력",
                                                                3 => "형제/자매, 사적 용기 및 모험심",
                                                                4 => "어머니, 모국, 가정의 평화 및 부동산",
                                                                5 => "지성/학업, 창의적 소질 및 자녀운",
                                                                6 => "질병/부상, 부채 극복력 및 경쟁력",
                                                                7 => "배우자, 결혼 생활 및 대인 파트너십",
                                                                8 => "수명, 급격한 변화 및 오컬트/영성",
                                                                9 => "종교, 철학, 종조상 및 고등학문",
                                                                10 => "직업적 천직, 사회적 성공 및 권위",
                                                                11 => "투자의 결실, 소망 성취 및 동료 네트워크",
                                                                12 => "지출/손실, 은둔, 격리 및 해탈/영성",
                                                                _ => "기타 인생 영역",
                                                            };
                                                            let bhava_opt = data.chart.bhava_strengths.iter().find(|bs| bs.house == h.house);
                                                            let bhava_str = if let Some(bs) = bhava_opt {
                                                                format!("L:{:.0} D:{:.0} A:{:.0}", bs.lord_score, bs.dig_score, bs.drishti_score)
                                                            } else {
                                                                "".to_string()
                                                            };
                                                            rsx! {
                                                                tr { class: "hover:bg-slate-800/20 transition-colors",
                                                                    td { class: "px-4 py-3 font-bold text-indigo-300 font-mono", "House {h.house}" }
                                                                    td { class: "px-4 py-3 text-xs",
                                                                        div { class: "flex flex-col gap-0.5",
                                                                            span { class: "font-semibold text-slate-200", "{house_purpose_kr}" }
                                                                            span { class: "text-[10px] text-slate-500 font-mono", "{h.summary}" }
                                                                        }
                                                                    }
                                                                    td { class: "px-4 py-3 text-xs",
                                                                        span { class: "px-2 py-0.5 rounded border text-[10px] font-bold {rating_cls}", "{rating_kr}" }
                                                                    }
                                                                    td { class: "px-4 py-3 text-xs",
                                                                        div { class: "flex flex-col gap-0.5",
                                                                            span { class: "font-mono font-bold text-slate-300", "{h.total_score:.1}" }
                                                                            if !bhava_str.is_empty() {
                                                                                span { class: "text-[9px] text-slate-500 font-mono whitespace-nowrap", "{bhava_str}" }
                                                                            }
                                                                        }
                                                                    }
                                                                    td { class: "px-4 py-3 text-xs",
                                                                        if h.reasons.is_empty() {
                                                                            span { class: "text-slate-500 italic", "특이 요인 없음" }
                                                                        } else {
                                                                            div { class: "flex flex-wrap gap-1.5",
                                                                                {h.reasons.iter().map(|reason| rsx! {
                                                                                    span { class: "px-1.5 py-0.5 bg-slate-900 border border-slate-800 rounded text-[10px] text-slate-450", "{reason}" }
                                                                                })}
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        })}
                                                    }
                                                }
                                            }
                                        }

                                        // ── 분석 메타 정보 (Analysis Metadata Card) ───────────────────────
                                        div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5 flex flex-col gap-3 shadow-xl",
                                            h3 { class: "text-xs font-bold text-slate-500 uppercase tracking-widest", "분석 정보 (Analysis Info)" }
                                            div { class: "space-y-2 text-xs font-mono text-slate-400",
                                                div { class: "flex justify-between border-b border-slate-850 pb-1.5",
                                                    span { "입력 시각" }
                                                    span { class: "text-slate-200 font-bold", "{data.meta.input_time}" }
                                                }
                                                div { class: "flex justify-between border-b border-slate-850 pb-1.5",
                                                    span { "교정 시각 (정밀도)" }
                                                    span { class: "text-slate-200 font-bold", "{data.meta.corrected_time}" }
                                                }
                                                div { class: "flex justify-between border-b border-slate-850 pb-1.5",
                                                    span { "분석 기준 타임존" }
                                                    span { class: "text-slate-200 font-bold", "{data.meta.analysis_timezone}" }
                                                }
                                                div { class: "flex justify-between",
                                                    span { "서머타임 (DST)" }
                                                    span { class: "text-slate-200 font-bold", if data.meta.is_dst { "적용됨" } else { "해당없음" } }
                                                }
                                            }
                                        }
                                    }
                                },
                                1 => rsx! {
                                    // KP System Tab
                                    if let Some(kp) = &data.kp_analysis {
                                        div { class: "grid grid-cols-1 lg:grid-cols-2 gap-6",
                                            // Cusps
                                            div { class: "bg-slate-900 border border-slate-800 rounded-2xl overflow-hidden",
                                                div { class: "bg-slate-800/50 border-b border-slate-800 px-5 py-3",
                                                    h3 { class: "font-semibold text-slate-200", "KP Unequal House Cusps (하우스 경계)" }
                                                }
                                                div { class: "overflow-x-auto",
                                                    table { class: "w-full text-sm",
                                                        thead {
                                                            tr { class: "bg-slate-800/30 text-xs text-slate-400 uppercase",
                                                                th { class: "px-4 py-3 text-left font-medium", "하우스" }
                                                                th { class: "px-4 py-3 text-left font-medium", "황경 (Sidereal)" }
                                                                th { class: "px-4 py-3 text-left font-medium", "Sign Lord" }
                                                                th { class: "px-4 py-3 text-left font-medium", "Star Lord" }
                                                                th { class: "px-4 py-3 text-left font-medium", "Sub Lord" }
                                                            }
                                                        }
                                                        tbody { class: "divide-y divide-slate-800",
                                                            {kp.cusps.iter().map(|c| rsx! {
                                                                tr { class: "hover:bg-slate-800/20 transition-colors",
                                                                    td { class: "px-4 py-3 font-bold text-indigo-300", "{c.name}" }
                                                                    td { class: "px-4 py-3 font-mono text-xs text-slate-400", "{c.longitude:.2}° ({rasi_name(c.rasi)})" }
                                                                    td { class: "px-4 py-3 text-xs font-semibold text-orange-400", "{planet_name_kr(c.sign_lord)}" }
                                                                    td { class: "px-4 py-3 text-xs font-semibold text-emerald-400", "{planet_name_kr(c.star_lord)}" }
                                                                    td { class: "px-4 py-3 text-xs font-semibold text-yellow-400", "{planet_name_kr(c.sub_lord)}" }
                                                                }
                                                            })}
                                                        }
                                                    }
                                                }
                                            }
                                            // Planets
                                            div { class: "bg-slate-900 border border-slate-800 rounded-2xl overflow-hidden",
                                                div { class: "bg-slate-800/50 border-b border-slate-800 px-5 py-3",
                                                    h3 { class: "font-semibold text-slate-200", "KP Planet Significators (행성 지배자)" }
                                                }
                                                div { class: "overflow-x-auto",
                                                    table { class: "w-full text-sm",
                                                        thead {
                                                            tr { class: "bg-slate-800/30 text-xs text-slate-400 uppercase",
                                                                th { class: "px-4 py-3 text-left font-medium", "행성" }
                                                                th { class: "px-4 py-3 text-left font-medium", "황경 (Sidereal)" }
                                                                th { class: "px-4 py-3 text-left font-medium", "Sign Lord" }
                                                                th { class: "px-4 py-3 text-left font-medium", "Star Lord" }
                                                                th { class: "px-4 py-3 text-left font-medium", "Sub Lord" }
                                                            }
                                                        }
                                                        tbody { class: "divide-y divide-slate-800",
                                                            {kp.planets.iter().map(|p| rsx! {
                                                                tr { class: "hover:bg-slate-800/20 transition-colors",
                                                                    td { class: "px-4 py-3 font-bold text-slate-200", "{p.name}" }
                                                                    td { class: "px-4 py-3 font-mono text-xs text-slate-400", "{p.longitude:.2}° ({rasi_name(p.rasi)})" }
                                                                    td { class: "px-4 py-3 text-xs font-semibold text-orange-400", "{planet_name_kr(p.sign_lord)}" }
                                                                    td { class: "px-4 py-3 text-xs font-semibold text-emerald-400", "{planet_name_kr(p.star_lord)}" }
                                                                    td { class: "px-4 py-3 text-xs font-semibold text-yellow-400", "{planet_name_kr(p.sub_lord)}" }
                                                                }
                                                            })}
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        p { class: "text-slate-500", "KP 분석 데이터가 없습니다." }
                                    }
                                },
                                2 => rsx! {
                                    // Dashas Tab
                                    div { class: "space-y-6",
                                        div { class: "flex bg-slate-800 p-1.5 rounded-xl gap-2 w-max self-center mx-auto",
                                            button {
                                                class: if *active_dasha_type.read() == 0 {
                                                    "px-4 py-1.5 text-xs font-semibold rounded-lg transition-colors bg-blue-600 text-white"
                                                } else {
                                                    "px-4 py-1.5 text-xs font-semibold rounded-lg transition-colors text-slate-400 hover:text-slate-200"
                                                },
                                                onclick: move |_| *active_dasha_type.write() = 0,
                                                "빔쇼따리 (Vimshottari)"
                                            }
                                            button {
                                                class: if *active_dasha_type.read() == 1 {
                                                    "px-4 py-1.5 text-xs font-semibold rounded-lg transition-colors bg-blue-600 text-white"
                                                } else {
                                                    "px-4 py-1.5 text-xs font-semibold rounded-lg transition-colors text-slate-400 hover:text-slate-200"
                                                },
                                                onclick: move |_| *active_dasha_type.write() = 1,
                                                "차라 다샤 (Chara Dasha)"
                                            }
                                            button {
                                                class: if *active_dasha_type.read() == 2 {
                                                    "px-4 py-1.5 text-xs font-semibold rounded-lg transition-colors bg-blue-600 text-white"
                                                } else {
                                                    "px-4 py-1.5 text-xs font-semibold rounded-lg transition-colors text-slate-400 hover:text-slate-200"
                                                },
                                                onclick: move |_| *active_dasha_type.write() = 2,
                                                "칼라 차크라 (Kala Chakra)"
                                            }
                                            button {
                                                class: if *active_dasha_type.read() == 3 {
                                                    "px-4 py-1.5 text-xs font-semibold rounded-lg transition-colors bg-blue-600 text-white"
                                                } else {
                                                    "px-4 py-1.5 text-xs font-semibold rounded-lg transition-colors text-slate-400 hover:text-slate-200"
                                                },
                                                onclick: move |_| *active_dasha_type.write() = 3,
                                                "요기니 다샤 (Yogini)"
                                            }
                                        }

                                        match *active_dasha_type.read() {
                                            0 => rsx! {
                                                div { class: "bg-slate-900 border border-slate-800 rounded-2xl overflow-hidden",
                                                    div { class: "bg-slate-800/50 border-b border-slate-800 px-5 py-3",
                                                        h3 { class: "font-semibold text-slate-200", "빔쇼따리 마하다샤 (Vimshottari Mahadasha)" }
                                                    }
                                                    div { class: "overflow-x-auto",
                                                        table { class: "w-full text-sm",
                                                            thead {
                                                                tr { class: "bg-slate-800/30 text-xs text-slate-400 uppercase",
                                                                    th { class: "px-4 py-3 text-left font-medium", "Lord" }
                                                                    th { class: "px-4 py-3 text-left font-medium", "시작" }
                                                                    th { class: "px-4 py-3 text-left font-medium", "종료" }
                                                                    th { class: "px-4 py-3 text-left font-medium", "기간" }
                                                                    th { class: "px-4 py-3 text-left font-medium", "상태" }
                                                                }
                                                            }
                                                            tbody { class: "divide-y divide-slate-800",
                                                                {data.report.dasha_timeline.iter().map(|d| {
                                                                    let color = planet_color(d.lord);
                                                                    let start_str = d.start_time.format("%Y-%m").to_string();
                                                                    let end_str = d.end_time.format("%Y-%m").to_string();
                                                                    let duration_years = (d.end_time - d.start_time).num_days() / 365;
                                                                    let now = chrono::Utc::now();
                                                                    let is_current = d.start_time <= now && now < d.end_time;
                                                                    rsx! {
                                                                        tr { class: "hover:bg-slate-800/20 transition-colors",
                                                                            td { class: "px-4 py-3 font-bold {color}", "{planet_name_kr(d.lord)}" }
                                                                            td { class: "px-4 py-3 font-mono text-slate-300 text-xs", "{start_str}" }
                                                                            td { class: "px-4 py-3 font-mono text-slate-400 text-xs", "{end_str}" }
                                                                            td { class: "px-4 py-3 text-slate-400 text-xs", "{duration_years}년" }
                                                                            td { class: "px-4 py-3",
                                                                                if is_current {
                                                                                    span { class: "px-2 py-0.5 rounded-full text-xs bg-blue-600/40 text-blue-200 border border-blue-500/40 font-semibold", "⬤ 현재 대운" }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                })}
                                                            }
                                                        }
                                                    }
                                                }
                                            },
                                            1 => rsx! {
                                                div { class: "bg-slate-900 border border-slate-800 rounded-2xl overflow-hidden",
                                                    div { class: "bg-slate-800/50 border-b border-slate-800 px-5 py-3",
                                                        h3 { class: "font-semibold text-slate-200", "자이미니 차라 다샤 (Jaimini Chara Dasha)" }
                                                    }
                                                    div { class: "overflow-x-auto",
                                                        table { class: "w-full text-sm",
                                                            thead {
                                                                tr { class: "bg-slate-800/30 text-xs text-slate-400 uppercase",
                                                                    th { class: "px-4 py-3 text-left font-medium", "대운 성좌 (Sign)" }
                                                                    th { class: "px-4 py-3 text-left font-medium", "시작" }
                                                                    th { class: "px-4 py-3 text-left font-medium", "종료" }
                                                                    th { class: "px-4 py-3 text-left font-medium", "상태" }
                                                                }
                                                            }
                                                            tbody { class: "divide-y divide-slate-800",
                                                                {data.report.chara_dasha_timeline.iter().map(|d| {
                                                                    let start_str = d.start_time.format("%Y-%m").to_string();
                                                                    let end_str = d.end_time.format("%Y-%m").to_string();
                                                                    let now = chrono::Utc::now();
                                                                    let is_current = d.start_time <= now && now < d.end_time;
                                                                    rsx! {
                                                                        tr { class: "hover:bg-slate-800/20 transition-colors",
                                                                            td { class: "px-4 py-3 font-bold text-indigo-300", "{rasi_name(d.rasi)}" }
                                                                            td { class: "px-4 py-3 font-mono text-slate-300 text-xs", "{start_str}" }
                                                                            td { class: "px-4 py-3 font-mono text-slate-400 text-xs", "{end_str}" }
                                                                            td { class: "px-4 py-3",
                                                                                if is_current {
                                                                                    span { class: "px-2 py-0.5 rounded-full text-xs bg-indigo-600/40 text-indigo-200 border border-indigo-500/40 font-semibold", "⬤ 진행 중" }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                })}
                                                            }
                                                        }
                                                    }
                                                }
                                            },
                                            2 => rsx! {
                                                div { class: "bg-slate-900 border border-slate-800 rounded-2xl overflow-hidden",
                                                    div { class: "bg-slate-800/50 border-b border-slate-800 px-5 py-3",
                                                        h3 { class: "font-semibold text-slate-200", "칼라 차크라 다샤 (Kala Chakra Dasha)" }
                                                    }
                                                    div { class: "overflow-x-auto",
                                                        table { class: "w-full text-sm",
                                                            thead {
                                                                tr { class: "bg-slate-800/30 text-xs text-slate-400 uppercase",
                                                                    th { class: "px-4 py-3 text-left font-medium", "대운 성좌 (Sign)" }
                                                                    th { class: "px-4 py-3 text-left font-medium", "시작" }
                                                                    th { class: "px-4 py-3 text-left font-medium", "종료" }
                                                                    th { class: "px-4 py-3 text-left font-medium", "기간" }
                                                                    th { class: "px-4 py-3 text-left font-medium", "상태" }
                                                                }
                                                            }
                                                            tbody { class: "divide-y divide-slate-800",
                                                                {data.report.kalachakra_timeline.iter().map(|d| {
                                                                    let start_str = d.start_time.format("%Y-%m").to_string();
                                                                    let end_str = d.end_time.format("%Y-%m").to_string();
                                                                    let now = chrono::Utc::now();
                                                                    let is_current = d.start_time <= now && now < d.end_time;
                                                                    rsx! {
                                                                        tr { class: "hover:bg-slate-800/20 transition-colors",
                                                                            td { class: "px-4 py-3 font-bold text-amber-500", "{rasi_name(d.rasi)}" }
                                                                            td { class: "px-4 py-3 font-mono text-slate-300 text-xs", "{start_str}" }
                                                                            td { class: "px-4 py-3 font-mono text-slate-400 text-xs", "{end_str}" }
                                                                            td { class: "px-4 py-3 text-slate-400 text-xs", "{d.duration_years}년" }
                                                                            td { class: "px-4 py-3",
                                                                                if is_current {
                                                                                    span { class: "px-2 py-0.5 rounded-full text-xs bg-amber-600/40 text-amber-200 border border-amber-500/40 font-semibold", "⬤ 진행 중" }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                })}
                                                            }
                                                        }
                                                    }
                                                }
                                            },
                                            3 => rsx! {
                                                div { class: "bg-slate-900 border border-slate-800 rounded-2xl overflow-hidden",
                                                    div { class: "bg-slate-800/50 border-b border-slate-800 px-5 py-3",
                                                        h3 { class: "font-semibold text-slate-200", "요기니 마하다샤 (Yogini Mahadasha)" }
                                                    }
                                                    div { class: "overflow-x-auto",
                                                        table { class: "w-full text-sm",
                                                            thead {
                                                                tr { class: "bg-slate-800/30 text-xs text-slate-400 uppercase",
                                                                    th { class: "px-4 py-3 text-left font-medium", "Yogini (지배성)" }
                                                                    th { class: "px-4 py-3 text-left font-medium", "시작" }
                                                                    th { class: "px-4 py-3 text-left font-medium", "종료" }
                                                                    th { class: "px-4 py-3 text-left font-medium", "기간" }
                                                                    th { class: "px-4 py-3 text-left font-medium", "상태" }
                                                                }
                                                            }
                                                            tbody { class: "divide-y divide-slate-800",
                                                                {data.report.yogini_timeline.iter().map(|d| {
                                                                    let color = planet_color(d.lord);
                                                                    let start_str = d.start_time.format("%Y-%m").to_string();
                                                                    let end_str = d.end_time.format("%Y-%m").to_string();
                                                                    let duration_years = (d.end_time - d.start_time).num_days() / 365;
                                                                    let now = chrono::Utc::now();
                                                                    let is_current = d.start_time <= now && now < d.end_time;
                                                                    let yogini_label = if let Some(n) = &d.name {
                                                                        format!("{} ({})", n, planet_name_kr(d.lord))
                                                                    } else {
                                                                        planet_name_kr(d.lord).to_string()
                                                                    };
                                                                    rsx! {
                                                                        tr { class: "hover:bg-slate-800/20 transition-colors",
                                                                            td { class: "px-4 py-3 font-bold {color}", "{yogini_label}" }
                                                                            td { class: "px-4 py-3 font-mono text-slate-300 text-xs", "{start_str}" }
                                                                            td { class: "px-4 py-3 font-mono text-slate-400 text-xs", "{end_str}" }
                                                                            td { class: "px-4 py-3 text-slate-400 text-xs", "{duration_years}년" }
                                                                            td { class: "px-4 py-3",
                                                                                if is_current {
                                                                                    span { class: "px-2 py-0.5 rounded-full text-xs bg-blue-600/40 text-blue-200 border border-blue-500/40 font-semibold", "⬤ 현재 대운" }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                })}
                                                            }
                                                        }
                                                    }
                                                }
                                            },
                                            _ => rsx! { div {} }
                                        }
                                    }
                                },
                                3 => rsx! {
                                    // Compatibility Tab
                                    div { class: "space-y-6",
                                        div { class: "p-5 bg-slate-900 border border-slate-800 rounded-2xl space-y-4",
                                            h3 { class: "text-lg font-semibold text-slate-200", "상대방 출생 정보 입력" }
                                            div { class: "grid grid-cols-2 md:grid-cols-5 gap-3",
                                                div { class: "flex flex-col gap-1",
                                                    label { class: "text-xs text-slate-500", "년도" }
                                                    input {
                                                        class: "bg-slate-800 border border-slate-700 rounded-xl px-3 py-2 text-slate-200",
                                                        r#type: "number",
                                                        value: "{partner_year}",
                                                        oninput: move |e| *partner_year.write() = e.value().parse().unwrap_or(1992)
                                                    }
                                                }
                                                div { class: "flex flex-col gap-1",
                                                    label { class: "text-xs text-slate-500", "월" }
                                                    input {
                                                        class: "bg-slate-800 border border-slate-700 rounded-xl px-3 py-2 text-slate-200",
                                                        r#type: "number",
                                                        value: "{partner_month}",
                                                        oninput: move |e| *partner_month.write() = e.value().parse().unwrap_or(8)
                                                    }
                                                }
                                                div { class: "flex flex-col gap-1",
                                                    label { class: "text-xs text-slate-500", "일" }
                                                    input {
                                                        class: "bg-slate-800 border border-slate-700 rounded-xl px-3 py-2 text-slate-200",
                                                        r#type: "number",
                                                        value: "{partner_day}",
                                                        oninput: move |e| *partner_day.write() = e.value().parse().unwrap_or(24)
                                                    }
                                                }
                                                div { class: "flex flex-col gap-1",
                                                    label { class: "text-xs text-slate-500", "시간 (시)" }
                                                    input {
                                                        class: "bg-slate-800 border border-slate-700 rounded-xl px-3 py-2 text-slate-200",
                                                        r#type: "number",
                                                        value: "{partner_hour}",
                                                        oninput: move |e| *partner_hour.write() = e.value().parse().unwrap_or(14)
                                                    }
                                                }
                                                div { class: "flex flex-col gap-1",
                                                    label { class: "text-xs text-slate-500", "시간 (분)" }
                                                    input {
                                                        class: "bg-slate-800 border border-slate-700 rounded-xl px-3 py-2 text-slate-200",
                                                        r#type: "number",
                                                        value: "{partner_minute}",
                                                        oninput: move |e| *partner_minute.write() = e.value().parse().unwrap_or(30)
                                                    }
                                                }
                                            }
                                            div { class: "grid grid-cols-2 gap-3",
                                                div { class: "flex flex-col gap-1",
                                                    label { class: "text-xs text-slate-500", "위도 (Latitude)" }
                                                    input {
                                                        class: "bg-slate-800 border border-slate-700 rounded-xl px-3 py-2 text-slate-200",
                                                        r#type: "number",
                                                        step: "any",
                                                        value: "{partner_lat}",
                                                        oninput: move |e| *partner_lat.write() = e.value().parse().unwrap_or(37.5665)
                                                    }
                                                }
                                                div { class: "flex flex-col gap-1",
                                                    label { class: "text-xs text-slate-500", "경도 (Longitude)" }
                                                    input {
                                                        class: "bg-slate-800 border border-slate-700 rounded-xl px-3 py-2 text-slate-200",
                                                        r#type: "number",
                                                        step: "any",
                                                        value: "{partner_lon}",
                                                        oninput: move |e| *partner_lon.write() = e.value().parse().unwrap_or(126.9780)
                                                    }
                                                }
                                            }
                                            button {
                                                class: "w-full py-3 bg-gradient-to-r from-purple-700 to-indigo-700 hover:from-purple-600 hover:to-indigo-600 rounded-xl font-bold text-white shadow-lg",
                                                onclick: run_compatibility,
                                                "💞 궁합 분석 실행"
                                            }
                                        }

                                        match &*compat_status.read() {
                                            TaskStatus::Loading => rsx! {
                                                div { class: "flex flex-col items-center py-10 gap-2",
                                                    div { class: "w-10 h-10 rounded-full border-4 border-purple-500/30 border-t-purple-400 animate-spin" }
                                                    p { class: "text-purple-400 font-medium text-sm animate-pulse", "궁합 연산 중..." }
                                                }
                                            },
                                            TaskStatus::Error(e) => rsx! {
                                                div { class: "p-4 rounded-xl bg-red-900/20 border border-red-800/50 text-red-400 text-sm", "오류: {e}" }
                                            },
                                            TaskStatus::Success => {
                                                if let Some(compat) = &*compat_data.read() {
                                                    rsx! {
                                                        div { class: "space-y-6 animate-in fade-in duration-500",
                                                            // Overall Compatibility Header
                                                            div { class: "p-5 bg-slate-900 border border-slate-800 rounded-2xl flex flex-col md:flex-row md:items-center justify-between gap-4",
                                                                div {
                                                                    h3 { class: "text-xs text-slate-500 uppercase tracking-widest font-bold", "종합 매칭 판정" }
                                                                    p { class: "text-2xl font-bold text-slate-200 mt-1",
                                                                        "호환성 점수: "
                                                                        span { class: "text-purple-400", "{compat.report.total_score} / 36 Gunas" }
                                                                    }
                                                                    p { class: "text-sm text-slate-400 mt-1.5", "{compat.report.explanation}" }
                                                                }
                                                                div { class: "flex gap-2",
                                                                    if compat.report.is_compatible {
                                                                        span { class: "px-4 py-2 rounded-xl bg-emerald-950/60 border border-emerald-800/60 text-emerald-400 text-sm font-bold", "✓ 상성 우수" }
                                                                    } else {
                                                                        span { class: "px-4 py-2 rounded-xl bg-amber-950/60 border border-amber-800/60 text-amber-400 text-sm font-bold", "⚠️ 신중함 요구" }
                                                                    }
                                                                }
                                                            }

                                                            // Mangal Dosha Card
                                                            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                                                div { class: "p-4 rounded-2xl bg-slate-900 border border-slate-800 space-y-2",
                                                                    h4 { class: "text-xs text-slate-500 font-semibold uppercase tracking-wider", "남성 화성살 (Male Mangal Dosha)" }
                                                                    p {
                                                                        class: if compat.report.male_mangal_dosha {
                                                                            "text-lg font-bold text-red-400"
                                                                        } else {
                                                                            "text-lg font-bold text-emerald-400"
                                                                        },
                                                                        if compat.report.male_mangal_dosha { "🔥 화성살(Manglik) 감지" } else { "✓ 해당 없음 (양호)" }
                                                                    }
                                                                }
                                                                div { class: "p-4 rounded-2xl bg-slate-900 border border-slate-800 space-y-2",
                                                                    h4 { class: "text-xs text-slate-500 font-semibold uppercase tracking-wider", "여성 화성살 (Female Mangal Dosha)" }
                                                                    p {
                                                                        class: if compat.report.female_mangal_dosha {
                                                                            "text-lg font-bold text-red-400"
                                                                        } else {
                                                                            "text-lg font-bold text-emerald-400"
                                                                        },
                                                                        if compat.report.female_mangal_dosha { "🔥 화성살(Manglik) 감지" } else { "✓ 해당 없음 (양호)" }
                                                                    }
                                                                }
                                                            }
                                                            if compat.report.mangal_dosha_cancelled {
                                                                div { class: "p-4 rounded-xl bg-blue-950/40 border border-blue-800/40 text-blue-300 text-xs font-semibold",
                                                                    "ℹ️ 상호 화성살 상쇄(Dosha Samya)가 성립되어 화성살의 부정적 영향이 소멸되었습니다."
                                                                }
                                                            }

                                                            // Ashtakoota Scorecard Table
                                                            div { class: "bg-slate-900 border border-slate-800 rounded-2xl overflow-hidden",
                                                                div { class: "bg-slate-800/50 border-b border-slate-800 px-5 py-3",
                                                                    h3 { class: "font-semibold text-slate-200", "아쉬타쿠타(Ashtakoota) 세부 매칭 평점표" }
                                                                }
                                                                div { class: "overflow-x-auto",
                                                                    table { class: "w-full text-sm",
                                                                        thead {
                                                                            tr { class: "bg-slate-800/30 text-xs text-slate-400 uppercase",
                                                                                th { class: "px-4 py-3 text-left font-medium", "매칭 요인 (Koota)" }
                                                                                th { class: "px-4 py-3 text-center font-medium", "가중치 (Max)" }
                                                                                th { class: "px-4 py-3 text-center font-medium", "획득 점수" }
                                                                                th { class: "px-4 py-3 text-left font-medium", "설명" }
                                                                            }
                                                                        }
                                                                        tbody { class: "divide-y divide-slate-800",
                                                                            {compat.report.kootas.iter().map(|k| rsx! {
                                                                                tr { class: "hover:bg-slate-800/20 transition-colors",
                                                                                    td { class: "px-4 py-3 font-semibold text-slate-300", "{k.name}" }
                                                                                    td { class: "px-4 py-3 text-center font-mono text-slate-500", "{k.max_points:.1}" }
                                                                                    td {
                                                                                        class: if k.earned_points > 0.0 {
                                                                                            "px-4 py-3 text-center font-bold font-mono text-purple-400"
                                                                                        } else {
                                                                                            "px-4 py-3 text-center font-bold font-mono text-slate-600"
                                                                                        },
                                                                                        "{k.earned_points:.1}"
                                                                                    }
                                                                                    td { class: "px-4 py-3 text-xs text-slate-400", "{k.description}" }
                                                                                }
                                                                            })}
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    rsx! { div {} }
                                                }
                                            },
                                            _ => rsx! { div {} }
                                        }
                                    }
                                },
                                4 => rsx! {
                                    // Gochara & Sade Sati Tab
                                    div { class: "space-y-6 animate-in fade-in duration-500",
                                        // 1) Sade Sati Card
                                        {
                                            use eon_vedic::analysis::gochara::SadeSatiPhase;
                                            let (phase_title, phase_desc, card_color) = match data.gochara.sade_sati {
                                                SadeSatiPhase::Rising => (
                                                    "토성 사데사티 시작기 (Rising Phase)",
                                                    "토성이 출생 달(Moon) 기준 12하우스에 머무르는 시기입니다. 내적인 성찰, 장기적 계획 수립, 조용히 책임을 다하는 시기입니다. 겉으로 드러나는 변화보다 내적인 단단함을 길러야 합니다.",
                                                    "border-yellow-800/50 bg-yellow-950/20 text-yellow-300"
                                                ),
                                                SadeSatiPhase::Peak => (
                                                    "토성 사데사티 절정기 (Peak Phase) 🔥",
                                                    "토성이 출생 달(Moon)과 1하우스에서 만나는 가장 강렬한 시기입니다. 인생의 중요한 시험대이자, 강한 인내와 헌신을 통해 삶을 완전히 재정비하고 거대한 성장의 발판을 마련하는 때입니다.",
                                                    "border-rose-800/50 bg-rose-950/20 text-rose-300"
                                                ),
                                                SadeSatiPhase::Setting => (
                                                    "토성 사데사티 마무리기 (Setting Phase)",
                                                    "토성이 출생 달(Moon) 기준 2하우스로 이동하여 7년 반 동안의 사데사티 시련을 정리하는 단계입니다. 그동안의 경험이 구체적인 삶의 결실과 지혜로 치환되기 시작하며, 재정/가족 문제의 안정화가 진행됩니다.",
                                                    "border-blue-800/50 bg-blue-950/20 text-blue-300"
                                                ),
                                                SadeSatiPhase::None => (
                                                    "사데사티 영향 없음 (Sade Sati Inactive)",
                                                    "현재 토성은 사데사티 영향권(달 기준 12, 1, 2하우스)을 벗어나 있습니다. 감정적으로 편안하고 안정된 주기를 보내실 수 있습니다.",
                                                    "border-emerald-800/50 bg-emerald-950/20 text-emerald-300"
                                                ),
                                            };
                                            rsx! {
                                                div { class: "p-5 border rounded-2xl flex gap-4 {card_color}",
                                                    div { class: "text-4xl shrink-0 mt-1", "🪐" }
                                                    div { class: "space-y-1.5",
                                                        h4 { class: "text-lg font-bold", "{phase_title}" }
                                                        p { class: "text-sm leading-relaxed opacity-90", "{phase_desc}" }
                                                        p { class: "text-xs opacity-60 font-mono mt-2", "출생 달 위치 기준 토성의 상대적 트랜짓 분석" }
                                                    }
                                                }
                                            }
                                        }

                                        // 2) Gochara Transits Table
                                        div { class: "bg-slate-900 border border-slate-800 rounded-2xl overflow-hidden shadow-xl",
                                            div { class: "bg-slate-800/50 border-b border-slate-800 px-5 py-3.5 flex items-center justify-between",
                                                h3 { class: "font-semibold text-slate-200", "실시간 고차라 트랜짓 분석 (Gochara Transits)" }
                                                span { class: "text-xs text-slate-400 font-mono", "달(Moon) 기준 하우스 배치" }
                                            }
                                            div { class: "overflow-x-auto",
                                                table { class: "w-full text-sm",
                                                    thead {
                                                        tr { class: "bg-slate-800/30 text-xs text-slate-400 uppercase",
                                                            th { class: "px-4 py-3 text-left font-medium", "행성" }
                                                            th { class: "px-4 py-3 text-left font-medium", "현재 성좌 (Rasi)" }
                                                            th { class: "px-4 py-3 text-left font-medium", "달 기준 위치" }
                                                            th { class: "px-4 py-3 text-left font-medium", "무르티 (Murti Nirnaya)" }
                                                            th { class: "px-4 py-3 text-center font-medium", "영향력 / 베다 차단" }
                                                            th { class: "px-4 py-3 text-left font-medium", "핵심 암시" }
                                                        }
                                                    }
                                                    tbody { class: "divide-y divide-slate-800",
                                                        {data.gochara.transits.iter().map(|tr_pos| {
                                                            let color = planet_color(tr_pos.planet);
                                                            let (murti_cls, murti_name) = match tr_pos.murti {
                                                                eon_vedic::analysis::gochara::MurtiType::Gold => ("text-yellow-400 bg-yellow-950/30 border border-yellow-800/30", "황금 (Suvarna - 대길)"),
                                                                eon_vedic::analysis::gochara::MurtiType::Silver => ("text-slate-200 bg-slate-800 border border-slate-700", "은 (Rajata - 길)"),
                                                                eon_vedic::analysis::gochara::MurtiType::Copper => ("text-amber-500 bg-amber-950/20 border border-amber-900/30", "구리 (Tamra - 보통/약흉)"),
                                                                eon_vedic::analysis::gochara::MurtiType::Iron => ("text-red-400 bg-red-950/30 border border-red-900/40", "철 (Loha - 매우흉)"),
                                                                _ => ("text-slate-500 bg-slate-900 border border-slate-800", "미상"),
                                                            };
                                                            rsx! {
                                                                tr { class: "hover:bg-slate-800/20 transition-colors",
                                                                    td { class: "px-4 py-3.5 font-bold {color} whitespace-nowrap", "{planet_name_kr(tr_pos.planet)}" }
                                                                    td { class: "px-4 py-3.5 text-slate-300 font-medium whitespace-nowrap", "{rasi_name(tr_pos.current_rasi)}" }
                                                                    td { class: "px-4 py-3.5 text-slate-400 font-mono text-xs whitespace-nowrap", "달 기준 {tr_pos.house_from_moon}하우스" }
                                                                    td { class: "px-4 py-3.5 whitespace-nowrap",
                                                                        span { class: "px-2.5 py-0.5 rounded-full text-xs font-bold {murti_cls}", "{murti_name}" }
                                                                    }
                                                                    td { class: "px-4 py-3.5 text-center whitespace-nowrap space-x-1.5",
                                                                        if tr_pos.is_benefic_transit {
                                                                            span { class: "px-1.5 py-0.5 rounded text-[10px] bg-emerald-950/50 text-emerald-400 border border-emerald-800/50 font-bold", "길(Benefic)" }
                                                                        } else {
                                                                            span { class: "px-1.5 py-0.5 rounded text-[10px] bg-rose-950/50 text-rose-400 border border-rose-800/50 font-bold", "흉(Malefic)" }
                                                                        }
                                                                        if tr_pos.is_blocked {
                                                                            span { class: "px-1.5 py-0.5 rounded text-[10px] bg-purple-950/50 text-purple-400 border border-purple-800/50 font-bold", "베다 차단(Vedha)" }
                                                                        }
                                                                    }
                                                                    td { class: "px-4 py-3.5 text-xs text-slate-300 leading-relaxed min-w-[200px]",
                                                                        p { class: "font-semibold text-slate-200", "{tr_pos.summary}" }
                                                                        p { class: "text-slate-400 mt-0.5 text-[11px]", "{tr_pos.description}" }
                                                                    }
                                                                }
                                                            }
                                                        })}
                                                    }
                                                }
                                            }
                                        }
                                    }
                                },
                                5 => {
                                    if let Some(tajika) = &data.tajika_report {
                                        rsx! {
                                            div { class: "space-y-6 animate-in fade-in duration-500",
                                                // 1) Summary and Year Lord / Muntha
                                                div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                                                    // Year Lord
                                                    div { class: "p-5 bg-slate-900 border border-slate-800 rounded-2xl flex items-center gap-4",
                                                        div { class: "text-4xl", "👑" }
                                                        div {
                                                            p { class: "text-xs text-slate-500 font-semibold tracking-wider", "올해의 지배자 (Year Lord)" }
                                                            {
                                                                if let Some(yl) = tajika.year_lord {
                                                                    let color = planet_color(yl);
                                                                    rsx! { p { class: "text-xl font-bold {color} mt-1", "{planet_name_kr(yl)}" } }
                                                                } else {
                                                                    rsx! { p { class: "text-xl font-bold text-slate-400 mt-1", "미지정" } }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    // Muntha Rasi
                                                    div { class: "p-5 bg-slate-900 border border-slate-800 rounded-2xl flex items-center gap-4",
                                                        div { class: "text-4xl", "🎯" }
                                                        div {
                                                            p { class: "text-xs text-slate-500 font-semibold tracking-wider", "올해의 집중 포커스 (Muntha)" }
                                                            p { class: "text-xl font-bold text-indigo-300 mt-1", "{rasi_name(tajika.muntha_rasi)}" }
                                                            p { class: "text-xs text-slate-500 mt-0.5", "매년 한 하우스씩 전진하는 발전 기운" }
                                                        }
                                                    }
                                                    // Year Summary
                                                    div { class: "p-5 bg-slate-900 border border-slate-800 rounded-2xl flex items-center gap-4 md:col-span-1",
                                                        div { class: "text-4xl", "📝" }
                                                        div { class: "flex-1 min-w-0",
                                                            p { class: "text-xs text-slate-500 font-semibold tracking-wider", "타지카 총평" }
                                                            p { class: "text-xs text-slate-300 mt-1 leading-relaxed truncate", "{tajika.summary}" }
                                                        }
                                                    }
                                                }

                                                // Full summary text
                                                div { class: "p-5 bg-slate-900/60 border border-slate-800 rounded-2xl space-y-2",
                                                    h4 { class: "text-xs text-slate-500 uppercase tracking-widest font-bold", "타지카 연간 거시 운세 해설" }
                                                    p { class: "text-sm text-slate-300 leading-relaxed font-medium", "{tajika.summary}" }
                                                }

                                                // 2) Sahams Table
                                                div { class: "bg-slate-900 border border-slate-800 rounded-2xl overflow-hidden shadow-xl",
                                                    div { class: "bg-slate-800/50 border-b border-slate-800 px-5 py-3",
                                                        h3 { class: "font-semibold text-slate-200", "연간 민감점 (Tajika Sahams)" }
                                                    }
                                                    div { class: "overflow-x-auto",
                                                        table { class: "w-full text-sm",
                                                            thead {
                                                                tr { class: "bg-slate-800/30 text-xs text-slate-400 uppercase",
                                                                    th { class: "px-4 py-3 text-left font-medium", "이름 (Saham)" }
                                                                    th { class: "px-4 py-3 text-left font-medium", "성좌 (Rasi)" }
                                                                    th { class: "px-4 py-3 text-left font-medium", "황경 (Longitude)" }
                                                                    th { class: "px-4 py-3 text-left font-medium", "동작/특성" }
                                                                }
                                                            }
                                                            tbody { class: "divide-y divide-slate-800",
                                                                {tajika.sahams.iter().map(|saham| {
                                                                    let desc = match saham.name.as_str() {
                                                                        "Punya Saham" => "재물, 풍요, 행운과 전반적 조력",
                                                                        "Vidya Saham" => "지혜, 지식 습득, 학업 및 연구 성과",
                                                                        "Vivaha Saham" => "결혼, 파트너십, 대인 상생 관계",
                                                                        "Karma Saham" => "직업, 비즈니스 성과, 승진 및 명예",
                                                                        "Roga Saham" => "질병 예방, 건강 관리 필요 구역",
                                                                        _ => "올해 활성화되는 개별 운세 감지점",
                                                                    };
                                                                    rsx! {
                                                                        tr { class: "hover:bg-slate-800/20 transition-colors",
                                                                            td { class: "px-4 py-3 font-bold text-amber-300", "{saham.name}" }
                                                                            td { class: "px-4 py-3 text-slate-300", "{rasi_name(saham.rasi)}" }
                                                                            td { class: "px-4 py-3 font-mono text-slate-400 text-xs", "{saham.longitude:.2}°" }
                                                                            td { class: "px-4 py-3 text-xs text-slate-400", "{desc}" }
                                                                        }
                                                                    }
                                                                })}
                                                            }
                                                        }
                                                    }
                                                }

                                                // 3) Harsha Bala Grid
                                                if !tajika.harsha_bala_summary.is_empty() {
                                                    div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5 space-y-4 shadow-xl",
                                                        h3 { class: "text-sm font-semibold text-slate-400 uppercase tracking-widest", "하르샤 발라 강도 (Harsha Bala - 연간 행성 행복도)" }
                                                        div { class: "grid grid-cols-3 sm:grid-cols-5 md:grid-cols-9 gap-3",
                                                            {tajika.harsha_bala_summary.iter().map(|&(planet, strength)| {
                                                                let color = planet_color(planet);
                                                                let rating_stars = match strength {
                                                                    4 => "★★★★ (최고)",
                                                                    3 => "★★★ (강함)",
                                                                    2 => "★★ (보통)",
                                                                    1 => "★ (약함)",
                                                                    _ => "☆ (없음)",
                                                                };
                                                                let rating_color = match strength {
                                                                    4 | 3 => "text-emerald-400",
                                                                    2 => "text-yellow-400",
                                                                    _ => "text-slate-500",
                                                                };
                                                                rsx! {
                                                                    div { class: "p-3 rounded-xl bg-slate-800/40 border border-slate-850 flex flex-col items-center gap-1.5 text-center shadow-inner",
                                                                        span { class: "text-xs font-bold {color}", "{planet_name_kr(planet)}" }
                                                                        span { class: "text-xs font-extrabold {rating_color} font-mono", "{strength}/4" }
                                                                        span { class: "text-[9px] text-slate-500 font-mono", "{rating_stars}" }
                                                                    }
                                                                }
                                                            })}
                                                        }
                                                    }
                                                }

                                                if data.annual_chart.is_some() {
                                                    div { class: "bg-slate-900 border border-slate-800 rounded-2xl overflow-hidden shadow-xl mt-6",
                                                        div { class: "bg-slate-800/50 border-b border-slate-800 px-5 py-3.5 flex justify-between items-center",
                                                            h3 { class: "font-semibold text-slate-200 text-sm uppercase tracking-widest", "타지카 연간 분점 차트 — 행성 상세 (Tajika Annual Chart)" }
                                                            span { class: "text-xs font-bold text-amber-400 bg-amber-950/20 px-3 py-1 rounded border border-amber-900/30",
                                                                "연간 라그나: {rasi_name(data.annual_chart.as_ref().unwrap().ascendant.rasi)} ({data.annual_chart.as_ref().unwrap().ascendant.sidereal_deg:.2}°)"
                                                            }
                                                        }
                                                        div { class: "overflow-x-auto",
                                                            table { class: "w-full text-sm",
                                                                thead {
                                                                    tr { class: "bg-slate-800/30 text-xs text-slate-400 uppercase",
                                                                        th { class: "px-4 py-3 text-left font-medium", "행성" }
                                                                        th { class: "px-4 py-3 text-left font-medium", "라시 (Sign)" }
                                                                        th { class: "px-4 py-3 text-left font-medium", "황경" }
                                                                        th { class: "px-4 py-3 text-left font-medium", "나크샤트라" }
                                                                        th { class: "px-4 py-3 text-left font-medium", "하우스" }
                                                                        th { class: "px-4 py-3 text-center font-medium", "태비/역행" }
                                                                    }
                                                                }
                                                                tbody { class: "divide-y divide-slate-800",
                                                                    {data.annual_chart.as_ref().unwrap().planets.iter().map(|p| {
                                                                        let color = planet_color(p.planet);
                                                                        rsx! {
                                                                            tr { class: "hover:bg-slate-800/20 transition-colors",
                                                                                td { class: "px-4 py-3 font-bold {color}", "{planet_name_kr(p.planet)}" }
                                                                                td { class: "px-4 py-3 text-slate-300", "{rasi_name(p.rasi)}" }
                                                                                td { class: "px-4 py-3 font-mono text-slate-400 text-xs", "{p.sidereal_deg:.2}°" }
                                                                                td { class: "px-4 py-3 text-slate-400 text-xs", "{nakshatra_name(p.nakshatra)} P{p.pada}" }
                                                                                td { class: "px-4 py-3 text-slate-400 text-xs font-mono", "H{p.house_index}" }
                                                                                td { class: "px-4 py-3 text-center",
                                                                                    if p.is_retrograde {
                                                                                        span { class: "px-1.5 py-0.5 rounded text-xs bg-purple-900/50 text-purple-300 border border-purple-700/50 mr-1", "R" }
                                                                                    }
                                                                                    if p.is_combust {
                                                                                        span { class: "px-1.5 py-0.5 rounded text-xs bg-orange-900/50 text-orange-300 border border-orange-700/50", "C" }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    })}
                                                                 }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        rsx! {
                                            div { class: "p-8 text-center text-slate-500 border border-dashed border-slate-800 rounded-2xl",
                                                "올해의 타지카 연간 차트 분석 데이터가 없습니다."
                                            }
                                        }
                                    }
                                },
                                6 => rsx! {
                                    // Varga Reports Tab
                                    div { class: "space-y-6",
                                        // Selector Card
                                        div { class: "p-5 bg-slate-900 border border-slate-800 rounded-2xl flex flex-col md:flex-row gap-4 items-center justify-between shadow-xl",
                                            div { class: "space-y-1",
                                                h3 { class: "text-lg font-bold text-slate-200", "베딕 분할차트 상세 분석 (D1~D144 Vargas)" }
                                                p { class: "text-xs text-slate-500", "라시(D1)부터 세분화된 세부 분할차트(Varga)의 행성 위치와 지배자, 신화적 수호신(Deity), 우주적 목적(Purpose)을 조회합니다." }
                                            }
                                            div { class: "flex items-center gap-2.5",
                                                span { class: "text-xs font-bold text-slate-400 shrink-0", "분할차트 선택:" }
                                                select {
                                                    class: "px-4 py-2 bg-slate-800 border border-slate-700 rounded-xl text-sm font-semibold text-slate-200 focus:outline-none focus:border-blue-500 transition-colors shadow-inner",
                                                    value: "{selected_varga}",
                                                    onchange: move |e| *selected_varga.write() = e.value().clone(),
                                                    option { value: "rasi", "D1 - Rasi (라시)" }
                                                    option { value: "hora", "D2 - Hora (호라)" }
                                                    option { value: "drekkana", "D3 - Drekkana (드레카나)" }
                                                    option { value: "chaturthamsha", "D4 - Chaturthamsha (차투르탐샤)" }
                                                    option { value: "panchamsa", "D5 - Panchamsa (판참사)" }
                                                    option { value: "shashtamsa", "D6 - Shashtamsa (샤쉬탐사)" }
                                                    option { value: "saptamsa", "D7 - Saptamsa (사프탐사)" }
                                                    option { value: "ashtamsa", "D8 - Ashtamsa (아쉬탐사)" }
                                                    option { value: "navamsa", "D9 - Navamsa (나밤사)" }
                                                    option { value: "dasamsa", "D10 - Dasamsa (다삼사)" }
                                                    option { value: "rudramsa", "D11 - Rudramsa (루드람사)" }
                                                    option { value: "dwadasamsa", "D12 - Dwadasamsa (드와다삼사)" }
                                                    option { value: "shodashamsa", "D16 - Shodashamsa (쇼다삼사)" }
                                                    option { value: "vimsamsa", "D20 - Vimsamsa (빔삼사)" }
                                                    option { value: "chaturvimshamsa", "D24 - Chaturvimshamsa (차투르빔삼사)" }
                                                    option { value: "saptavimsamsa", "D27 - Saptavimsamsa (사프타빔삼사)" }
                                                    option { value: "trimsamsa", "D30 - Trimsamsa (트림삼사)" }
                                                    option { value: "khavedamsa", "D40 - Khavedamsa (카베담사)" }
                                                    option { value: "akshavedamsa", "D45 - Akshavedamsa (악샤베담사)" }
                                                    option { value: "shashtyamsa", "D60 - Shashtyamsa (샤쉬티암사)" }
                                                    option { value: "navanavamsa", "D81 - Navanavamsa (나바나밤사)" }
                                                    option { value: "ashtottaramsa", "D108 - Ashtottaramsa (아쉬토따람사)" }
                                                    option { value: "dwadasdwadasamsa", "D144 - Dwadasdwadasamsa (드와다스-드와다삼사)" }
                                                }
                                            }
                                        }

                                        // Varga Report Table
                                        {
                                            let v_id = selected_varga.read().clone();
                                            if let Some(v_report) = data.varga_nakshatra_reports.reports.get(&v_id) {
                                                let lagna_name = rasi_name(v_report.lagna_rasi);

                                                // Calculate Varga planets
                                                let mut v_rasi_planets: Vec<Vec<(&'static str, &'static str)>> = vec![vec![]; 13];
                                                let mut v_house_planets: Vec<Vec<(&'static str, &'static str)>> = vec![vec![]; 13];

                                                v_house_planets[1].push(("Asc", "text-sky-400 font-bold"));
                                                if v_report.lagna_rasi >= 1 && v_report.lagna_rasi <= 12 {
                                                    v_rasi_planets[v_report.lagna_rasi as usize].push(("Asc", "text-sky-400 font-bold"));
                                                }

                                                for row in &v_report.rows {
                                                    let p_name = row.planet.as_str();
                                                    let p_lbl = match p_name {
                                                        "Sun" | "Surya" => "Su",
                                                        "Moon" | "Chandra" => "Mo",
                                                        "Mars" | "Mangala" => "Ma",
                                                        "Mercury" | "Budha" => "Me",
                                                        "Jupiter" | "Guru" => "Ju",
                                                        "Venus" | "Shukra" => "Ve",
                                                        "Saturn" | "Shani" => "Sa",
                                                        "Rahu" => "Ra",
                                                        "Ketu" => "Ke",
                                                        "Lagna" | "Ascendant" => "Asc",
                                                        _ => "",
                                                    };
                                                    if p_lbl.is_empty() || p_lbl == "Asc" {
                                                        continue;
                                                    }
                                                    let p_color = match p_name {
                                                        "Sun" | "Surya" => "text-orange-400 font-bold",
                                                        "Moon" | "Chandra" => "text-slate-355 font-bold",
                                                        "Mars" | "Mangala" => "text-red-400 font-bold",
                                                        "Mercury" | "Budha" => "text-emerald-400 font-bold",
                                                        "Jupiter" | "Guru" => "text-yellow-400 font-bold",
                                                        "Venus" | "Shukra" => "text-pink-400 font-bold",
                                                        "Saturn" | "Shani" => "text-indigo-400 font-bold",
                                                        "Rahu" => "text-purple-400 font-bold",
                                                        "Ketu" => "text-amber-600 font-bold",
                                                        _ => "text-slate-405",
                                                    };
                                                    if row.sign >= 1 && row.sign <= 12 {
                                                        v_rasi_planets[row.sign as usize].push((p_lbl, p_color));
                                                    }
                                                    if row.house >= 1 && row.house <= 12 {
                                                        v_house_planets[row.house as usize].push((p_lbl, p_color));
                                                    }
                                                }

                                                let cur_v_style = varga_chart_style.read().clone();

                                                rsx! {
                                                    div { class: "space-y-6 animate-in fade-in duration-500",
                                                        // 1) Visual Chart Card
                                                        div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5 space-y-4 shadow-xl",
                                                            div { class: "flex justify-between items-center border-b border-slate-800/60 pb-3 flex-wrap gap-2",
                                                                div { class: "space-y-0.5",
                                                                    h3 { class: "font-semibold text-slate-200 text-sm uppercase tracking-wider", "{v_report.varga_label} 배치도 (Visual Chart)" }
                                                                    p { class: "text-xs text-slate-500", "선택된 분할차트의 각 행성 및 라그나 배치도 시각화" }
                                                                }
                                                                div { class: "flex items-center gap-1.5 bg-slate-950 p-1 rounded-xl border border-slate-855",
                                                                    button {
                                                                        class: if cur_v_style == "south" {
                                                                            "px-3 py-1.5 text-xs font-bold text-blue-400 bg-slate-850 rounded-lg transition-colors border border-slate-700/50"
                                                                        } else {
                                                                            "px-3 py-1.5 text-xs font-semibold text-slate-400 hover:text-slate-200 transition-colors"
                                                                        },
                                                                        onclick: move |_| *varga_chart_style.write() = "south".to_string(),
                                                                        "남인도식 (South)"
                                                                    }
                                                                    button {
                                                                        class: if cur_v_style == "north" {
                                                                            "px-3 py-1.5 text-xs font-bold text-blue-400 bg-slate-855 rounded-lg transition-colors border border-slate-700/50"
                                                                        } else {
                                                                            "px-3 py-1.5 text-xs font-semibold text-slate-400 hover:text-slate-200 transition-colors"
                                                                        },
                                                                        onclick: move |_| *varga_chart_style.write() = "north".to_string(),
                                                                        "북인도식 (North)"
                                                                    }
                                                                }
                                                            }
                                                                                            div { class: "py-2 flex flex-col items-center justify-center w-full",
                                                                {render_vedic_chart(
                                                                    &v_rasi_planets,
                                                                    &v_house_planets,
                                                                    v_report.lagna_rasi,
                                                                    &cur_v_style,
                                                                    &v_report.varga_label,
                                                                    active_tooltip,
                                                                    selected_detail,
                                                                    Some(&v_report.rows),
                                                                    None,
                                                                    None,
                                                                    None,
                                                                )}
                                                                div { class: "w-full max-w-[400px] mx-auto",
                                                                    {render_detail_card(selected_detail)}
                                                                }
                                                            }
                                                        }

                                                        // 2) Details Table Card
                                                        div { class: "bg-slate-900 border border-slate-800 rounded-2xl overflow-hidden shadow-xl",
                                                            div { class: "bg-slate-800/50 border-b border-slate-800 px-5 py-3.5 flex justify-between items-center flex-wrap gap-2",
                                                                h3 { class: "font-semibold text-slate-200", "{v_report.varga_label} 상세 표" }
                                                                span { class: "text-xs font-bold text-indigo-400 bg-indigo-950/20 px-3 py-1 rounded border border-indigo-900/30",
                                                                    "라그나 성좌: {lagna_name}"
                                                                }
                                                            }
                                                            div { class: "overflow-x-auto",
                                                                table { class: "w-full text-sm",
                                                                    thead {
                                                                        tr { class: "bg-slate-800/30 text-xs text-slate-400 uppercase border-b border-slate-800",
                                                                            th { class: "px-4 py-3 text-left font-medium", "행성" }
                                                                            th { class: "px-4 py-3 text-left font-medium", "도수 / 성좌" }
                                                                            th { class: "px-4 py-3 text-left font-medium", "하우스" }
                                                                            th { class: "px-4 py-3 text-left font-medium", "나크샤트라 (Pada)" }
                                                                            th { class: "px-4 py-3 text-left font-medium", "지배성" }
                                                                            th { class: "px-4 py-3 text-left font-medium", "수호신 (Deity)" }
                                                                            th { class: "px-4 py-3 text-left font-medium", "우주적 의미 (Purpose)" }
                                                                            th { class: "px-4 py-3 text-center font-medium", "태비/역행" }
                                                                        }
                                                                    }
                                                                    tbody { class: "divide-y divide-slate-800",
                                                                        {v_report.rows.iter().map(|row| {
                                                                            let planet_lbl = planet_name_kr_str(&row.planet);
                                                                            let planet_color_cls = planet_color_str(&row.planet);
                                                                            let sign_lbl = rasi_name(row.sign);
                                                                            let nak_lord = planet_name_kr_str(&row.nakshatra_lord);
                                                                            let pad_lord = planet_name_kr_str(&row.pada_lord);
                                                                            let deities = row.deity.clone();
                                                                            let purposes = row.purpose.clone();
                                                                            let pos_lbl = row.position_str.clone();
                                                                            let nak_name = row.nakshatra_name.clone();
                                                                            let pada_val = row.pada;
                                                                            let house_val = row.house;
                                                                            rsx! {
                                                                                tr { class: "hover:bg-slate-800/20 transition-colors",
                                                                                    td { class: "px-4 py-3 font-bold {planet_color_cls}", "{planet_lbl}" }
                                                                                    td { class: "px-4 py-3 text-slate-300 font-mono text-xs", "{pos_lbl} ({sign_lbl})" }
                                                                                    td { class: "px-4 py-3 text-slate-400 text-xs font-mono", "H{house_val}" }
                                                                                    td { class: "px-4 py-3 text-slate-400 text-xs", "{nak_name} ({pada_val}단계)" }
                                                                                    td { class: "px-4 py-3 text-xs",
                                                                                        div { class: "flex flex-col gap-0.5",
                                                                                            span { class: "text-[10px] text-slate-500", "Star: {nak_lord}" }
                                                                                            span { class: "text-[10px] text-slate-500", "Pada: {pad_lord}" }
                                                                                        }
                                                                                    }
                                                                                    td { class: "px-4 py-3 text-xs text-amber-400 font-semibold", "{deities}" }
                                                                                    td { class: "px-4 py-3 text-xs text-slate-400 leading-normal max-w-xs whitespace-normal", "{purposes}" }
                                                                                    td { class: "px-4 py-3 text-center",
                                                                                        if row.is_retrograde {
                                                                                            span { class: "px-1.5 py-0.5 rounded text-[10px] bg-purple-900/50 text-purple-300 border border-purple-700/50 mr-1 font-bold", "역행" }
                                                                                        }
                                                                                        if row.is_combust {
                                                                                            span { class: "px-1.5 py-0.5 rounded text-[10px] bg-orange-900/50 text-orange-300 border border-orange-700/50 font-bold", "태비" }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        })}
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                rsx! {
                                                    div { class: "p-8 text-center text-slate-500 border border-dashed border-slate-800 rounded-2xl",
                                                        "선택한 분할차트 분석 데이터를 찾을 수 없습니다."
                                                    }
                                                }
                                            }
                                        }
                                    }
                                },
                                7 => rsx! {
                                    // Varga Interpretations & Text Reports Tab
                                    div { class: "space-y-6",
                                        // Top grid for D9 & D10 Text Analysis
                                        div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                            // D9 Marriage Analysis
                                            div { class: "p-5 bg-slate-900 border border-slate-800 rounded-2xl space-y-3 shadow-xl",
                                                div { class: "flex items-center gap-2 border-b border-slate-800/60 pb-2.5",
                                                    span { class: "text-xl", "💍" }
                                                    h3 { class: "font-bold text-slate-200 text-sm uppercase tracking-wider", "D9 나밤사 (Navamsa) — 관계 및 배우자 해석" }
                                                }
                                                p { class: "text-xs text-slate-400 leading-relaxed whitespace-pre-line", "{data.report.d9_marriage_analysis}" }
                                            }
                                            // D10 Career Analysis
                                            div { class: "p-5 bg-slate-900 border border-slate-800 rounded-2xl space-y-3 shadow-xl",
                                                div { class: "flex items-center gap-2 border-b border-slate-800/60 pb-2.5",
                                                    span { class: "text-xl", "💼" }
                                                    h3 { class: "font-bold text-slate-200 text-sm uppercase tracking-wider", "D10 다삼사 (Dasamsa) — 직업 및 사회적 성취" }
                                                }
                                                p { class: "text-xs text-slate-400 leading-relaxed whitespace-pre-line", "{data.report.d10_career_analysis}" }
                                            }
                                        }

                                        // Varga Planet Interpretations List
                                        div { class: "bg-slate-900 border border-slate-800 rounded-2xl overflow-hidden shadow-xl",
                                            div { class: "bg-slate-800/50 border-b border-slate-800 px-5 py-3.5",
                                                h3 { class: "font-semibold text-slate-200 text-sm uppercase tracking-widest", "분할차트 통합 행성 강약 분석 (Varga Interpretations)" }
                                            }
                                            div { class: "p-5 space-y-4",
                                                {data.report.varga_interpretations.iter().map(|vi| {
                                                    let p_lbl = planet_name_kr(vi.planet);
                                                    let p_color_cls = planet_color(vi.planet);
                                                    let d9_sgn = rasi_name(vi.d9_rasi);
                                                    let d10_sgn = rasi_name(vi.d10_rasi);
                                                    let d60_sgn = rasi_name(vi.d60_rasi);
                                                    rsx! {
                                                        div { class: "p-4 rounded-xl bg-slate-800/20 border border-slate-800 space-y-2.5 hover:border-slate-750 transition-colors",
                                                            div { class: "flex items-center justify-between flex-wrap gap-2",
                                                                div { class: "flex items-center gap-2.5",
                                                                    span { class: "font-bold text-sm {p_color_cls}", "{p_lbl}" }
                                                                    span { class: "text-[10px] font-semibold text-indigo-400 bg-indigo-950/20 px-2 py-0.5 rounded border border-indigo-900/30",
                                                                        "{vi.summary}"
                                                                    }
                                                                }
                                                                div { class: "flex items-center gap-2",
                                                                    if vi.is_vargottama {
                                                                        span { class: "px-2 py-0.5 rounded text-[9px] font-bold bg-emerald-950/40 text-emerald-450 border border-emerald-900/30", "바르고타마 (Vargottama)" }
                                                                    }
                                                                    if vi.is_pushkar_navamsa {
                                                                        span { class: "px-2 py-0.5 rounded text-[9px] font-bold bg-amber-950/40 text-amber-450 border border-amber-900/30", "푸쉬카르 나밤사" }
                                                                    }
                                                                }
                                                            }
                                                            div { class: "grid grid-cols-3 gap-2 text-[10px] py-1.5 px-3 bg-slate-900/40 rounded-lg border border-slate-800/60 font-mono text-slate-400",
                                                                div { "D9 나밤사: {d9_sgn}" }
                                                                div { "D10 다삼사: {d10_sgn}" }
                                                                div { "D60 샤쉬티암사: {d60_sgn}" }
                                                            }
                                                            p { class: "text-xs text-slate-400 leading-relaxed", "{vi.description}" }
                                                            if !vi.reasons.is_empty() {
                                                                div { class: "space-y-1 pt-1 border-t border-slate-800/50",
                                                                    p { class: "text-[9px] font-bold text-slate-500", "세부 판단 요인:" }
                                                                    ul { class: "list-disc pl-4 text-[9px] text-slate-500 space-y-0.5",
                                                                        {vi.reasons.iter().map(|reason| rsx! {
                                                                            li { "{reason}" }
                                                                        })}
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                })}
                                            }
                                        }
                                    }
                                },
                                8 => rsx! {
                                    // Ashtakavarga Tab
                                    div { class: "space-y-6 animate-in fade-in duration-500",
                                        // 1) Sarvashtakavarga (SAV - 종합 강도) Card/Table
                                        div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5 space-y-4 shadow-xl",
                                            div { class: "flex items-center gap-2 border-b border-slate-800/60 pb-2.5",
                                                span { class: "text-xl", "📊" }
                                                h3 { class: "font-semibold text-slate-200 text-sm uppercase tracking-wider", "사르바쉬타카바르가 (SAV — 종합 에너지 총합)" }
                                                span { class: "text-xs text-slate-500", "12개 하우스의 전반적인 에너지 총량 (평균 28점)" }
                                            }
                                            div { class: "grid grid-cols-3 sm:grid-cols-6 md:grid-cols-12 gap-3",
                                                {data.chart.sav.points.iter().enumerate().map(|(idx, &pts)| {
                                                    let house = idx + 1;
                                                    let text_color = if pts >= 30 {
                                                        "text-emerald-400"
                                                    } else if pts < 25 {
                                                        "text-rose-400"
                                                    } else {
                                                        "text-indigo-300"
                                                    };
                                                    let bg_color = if pts >= 30 {
                                                        "bg-emerald-950/10 border-emerald-900/30 hover:border-emerald-800/50"
                                                    } else if pts < 25 {
                                                        "bg-rose-950/10 border-rose-900/30 hover:border-rose-800/50"
                                                    } else {
                                                        "bg-slate-800/20 border-slate-850 hover:border-slate-750"
                                                    };
                                                    rsx! {
                                                        div { class: "p-3 rounded-xl border flex flex-col items-center gap-1.5 transition-all duration-300 {bg_color}",
                                                            span { class: "text-[10px] text-slate-500 font-bold", "H{house}" }
                                                            span { class: "text-lg font-extrabold {text_color} font-mono", "{pts}" }
                                                            span { class: "text-[8px] text-slate-550",
                                                                if pts >= 30 { "풍요" } else if pts < 25 { "부족" } else { "보통" }
                                                            }
                                                        }
                                                    }
                                                })}
                                            }
                                        }

                                        // 2) Bhinna Ashtakavarga (BAV - 행성별 세부 강도) Table with Reduction Switcher
                                        div { class: "bg-slate-900 border border-slate-800 rounded-2xl overflow-hidden shadow-xl space-y-4",
                                            div { class: "bg-slate-800/50 border-b border-slate-800 px-5 py-3.5 flex flex-wrap justify-between items-center gap-4",
                                                div { class: "space-y-0.5",
                                                    h3 { class: "font-semibold text-slate-200 text-sm uppercase tracking-wider", "빈나 아쉬타카바르가 (BAV - 행성별 세부 강도)" }
                                                    p { class: "text-[10px] text-slate-500", "7대 행성의 각 하우스별 길흉 점수와 감쇄 분석 및 최종 소디아 핀다" }
                                                }
                                                // Reduction View Selector Buttons
                                                div { class: "flex bg-slate-900 p-0.5 rounded-lg border border-slate-800 text-xs font-semibold text-slate-400",
                                                    button {
                                                        class: if *active_reduction_view.read() == 0 {
                                                            "px-3 py-1.5 rounded-md bg-indigo-600 text-white transition-colors font-bold"
                                                        } else {
                                                            "px-3 py-1.5 rounded-md hover:text-slate-200 transition-colors"
                                                        },
                                                        onclick: move |_| *active_reduction_view.write() = 0,
                                                        "순수 BAV (Raw)"
                                                    }
                                                    button {
                                                        class: if *active_reduction_view.read() == 1 {
                                                            "px-3 py-1.5 rounded-md bg-indigo-600 text-white transition-colors font-bold"
                                                        } else {
                                                            "px-3 py-1.5 rounded-md hover:text-slate-200 transition-colors"
                                                        },
                                                        onclick: move |_| *active_reduction_view.write() = 1,
                                                        "삼각 감쇄 (Trikona)"
                                                    }
                                                    button {
                                                        class: if *active_reduction_view.read() == 2 {
                                                            "px-3 py-1.5 rounded-md bg-indigo-600 text-white transition-colors font-bold"
                                                        } else {
                                                            "px-3 py-1.5 rounded-md hover:text-slate-200 transition-colors"
                                                        },
                                                        onclick: move |_| *active_reduction_view.write() = 2,
                                                        "일주 감쇄 (Ekadhipatya)"
                                                    }
                                                }
                                            }

                                            div { class: "overflow-x-auto p-1",
                                                table { class: "w-full text-sm font-mono",
                                                    thead {
                                                        tr { class: "bg-slate-800/20 text-xs text-slate-400 uppercase border-b border-slate-800",
                                                            th { class: "px-4 py-3 text-left font-medium w-28", "행성 (Planet)" }
                                                            { (1..=12).map(|h| rsx! {
                                                                th { class: "px-2 py-3 text-center font-medium w-12", "H{h}" }
                                                            }) }
                                                            th { class: "px-4 py-3 text-center font-medium w-32", "소디아 핀다 (Pinda)" }
                                                        }
                                                    }
                                                    tbody { class: "divide-y divide-slate-800/60",
                                                        {data.chart.bav.iter().map(|b| {
                                                            let p_name = planet_name_kr(b.planet);
                                                            let p_color = planet_color(b.planet);
                                                            let pts_arr = match *active_reduction_view.read() {
                                                                1 => &b.trikona_points,
                                                                2 => &b.shodhana_points,
                                                                _ => &b.points,
                                                            };
                                                            rsx! {
                                                                tr { class: "hover:bg-slate-800/10 transition-colors",
                                                                    td { class: "px-4 py-3 font-bold text-left",
                                                                        span { class: "{p_color}", "{p_name}" }
                                                                    }
                                                                    { pts_arr.iter().map(|&pt| {
                                                                        let text_color = if pt > 4 {
                                                                            "text-emerald-400 font-bold"
                                                                        } else if pt == 0 {
                                                                            "text-slate-650"
                                                                        } else {
                                                                            "text-slate-350"
                                                                        };
                                                                        rsx! {
                                                                            td { class: "px-2 py-3 text-center {text_color}", "{pt}" }
                                                                        }
                                                                    }) }
                                                                    td { class: "px-4 py-3 text-center font-extrabold text-amber-400",
                                                                        "{b.sodya_pinda}"
                                                                    }
                                                                }
                                                            }
                                                        })}
                                                    }
                                                }
                                            }
                                        }
                                    }
                                },
                                _ => rsx! { div {} }
                            }
                            {render_floating_tooltip(active_tooltip)}
                        }
                    } else {
                        rsx! { div {} }
                    }
                }
            }
        }
    }
}

#[component]
fn KarakaCard(label: String, planet: VedicPlanet) -> Element {
    let color = planet_color(planet);
    rsx! {
        div { class: "p-3 rounded-xl bg-slate-800/50 border border-slate-700/50 flex flex-col gap-1",
            p { class: "text-xs text-slate-500", "{label}" }
            p { class: "font-bold text-sm {color}", "{planet_name_kr(planet)}" }
        }
    }
}
