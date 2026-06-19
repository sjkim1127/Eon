use dioxus::prelude::*;
use crate::store::{AnalysisState, TaskStatus};
use eon_service::dto::{VedicAnalysisInput, AnalysisInput};
use eon_service::facade;
use eon_vedic::planets::VedicPlanet;
use crate::components::shared::birth_form::BirthForm;

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

fn rasi_name(rasi: u8) -> &'static str {
    if rasi == 0 || rasi > 12 { return "—" }
    RASI_NAMES_KR[rasi as usize]
}

fn nakshatra_name(n: u8) -> &'static str {
    if n == 0 || n > 27 { return "—" }
    NAKSHATRA_NAMES[n as usize]
}

#[component]
pub fn VedicTab() -> Element {
    let mut state = use_context::<AnalysisState>();

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

    rsx! {
        div { class: "space-y-6 animate-in fade-in duration-700",
            BirthForm {}

            div { class: "flex justify-between items-center",
                h2 { class: "text-2xl font-bold bg-gradient-to-r from-blue-200 to-indigo-400 bg-clip-text text-transparent",
                    "베딕 점성학 (Vedic Astrology)"
                }
                button {
                    class: "px-5 py-2.5 bg-gradient-to-r from-blue-700 to-indigo-700 hover:from-blue-600 hover:to-indigo-600 rounded-xl font-semibold text-white shadow-lg shadow-blue-900/30 transition-all duration-200 active:scale-95",
                    onclick: run_analysis,
                    "🔭 차트 생성"
                }
            }

            match &state.vedic.read().status {
                TaskStatus::Idle => rsx! {
                    div { class: "flex flex-col items-center justify-center py-20 gap-3 text-slate-500",
                        span { class: "text-5xl", "🪐" }
                        p { class: "text-lg font-medium", "출생 정보를 입력하고 차트를 생성하세요." }
                    }
                },
                TaskStatus::Loading => rsx! {
                    div { class: "flex flex-col items-center justify-center py-20 gap-3",
                        div { class: "w-12 h-12 rounded-full border-4 border-blue-500/30 border-t-blue-400 animate-spin" }
                        p { class: "text-blue-400 font-medium animate-pulse", "천문 계산 중... (Swiss Ephemeris)" }
                    }
                },
                TaskStatus::Error(e) => rsx! {
                    div { class: "p-4 rounded-xl bg-red-900/20 border border-red-800/50 text-red-400", "오류: {e}" }
                },
                TaskStatus::Success => {
                    if let Some(data) = &state.vedic.read().data {
                        rsx! {
                            // ── 1. 라그나 (Ascendant) 배지 ─────────────────────────
                            div { class: "flex items-center gap-4 p-5 bg-slate-900 border border-slate-800 rounded-2xl",
                                div { class: "text-5xl", "⬆️" }
                                div {
                                    p { class: "text-xs text-slate-400 uppercase tracking-widest font-semibold", "라그나 (Ascendant / Lagna)" }
                                    p { class: "text-2xl font-bold text-indigo-300 mt-1",
                                        "{rasi_name(data.chart.ascendant.rasi)}"
                                    }
                                    p { class: "text-sm text-slate-400 mt-1",
                                        "나크샤트라: {nakshatra_name(data.chart.ascendant.nakshatra)} pada {data.chart.ascendant.pada}"
                                        " | 황경: {data.chart.ascendant.sidereal_deg:.2}°"
                                    }
                                }
                                // 사데사티 경고
                                {
                                    use eon_vedic::analysis::gochara::SadeSatiPhase;
                                    match &data.report.sade_sati {
                                        SadeSatiPhase::None => rsx! { div {} },
                                        _phase => rsx! {
                                            div { class: "ml-auto px-4 py-2 rounded-xl bg-red-900/30 border border-red-700/50 text-red-300 text-sm font-bold animate-pulse",
                                                "⚠️ 사데사티 진행 중"
                                            }
                                        }
                                    }
                                }
                            }

                            // ── 2. 행성 위치 테이블 ────────────────────────────────
                            div { class: "bg-slate-900 border border-slate-800 rounded-2xl overflow-hidden",
                                div { class: "bg-slate-800/50 border-b border-slate-800 px-5 py-3",
                                    h3 { class: "font-semibold text-slate-200", "D1 라시 차트 — 행성 위치" }
                                }
                                div { class: "overflow-x-auto",
                                    table { class: "w-full text-sm",
                                        thead {
                                            tr { class: "bg-slate-800/30 text-xs text-slate-400 uppercase",
                                                th { class: "px-4 py-3 text-left font-medium", "행성" }
                                                th { class: "px-4 py-3 text-left font-medium", "라시 (Sign)" }
                                                th { class: "px-4 py-3 text-left font-medium", "황경 (Deg)" }
                                                th { class: "px-4 py-3 text-left font-medium", "나크샤트라" }
                                                th { class: "px-4 py-3 text-left font-medium", "파다" }
                                                th { class: "px-4 py-3 text-left font-medium", "하우스" }
                                                th { class: "px-4 py-3 text-left font-medium", "D9" }
                                                th { class: "px-4 py-3 text-center font-medium", "역행/컴버스트" }
                                            }
                                        }
                                        tbody { class: "divide-y divide-slate-800",
                                            {data.chart.planets.iter().map(|p| {
                                                let color = planet_color(p.planet);
                                                rsx! {
                                                    tr { class: "hover:bg-slate-800/20 transition-colors",
                                                        td { class: "px-4 py-3 font-bold {color}", "{planet_name_kr(p.planet)}" }
                                                        td { class: "px-4 py-3 text-slate-300", "{rasi_name(p.rasi)}" }
                                                        td { class: "px-4 py-3 font-mono text-slate-400 text-xs", "{p.sidereal_deg:.2}°" }
                                                        td { class: "px-4 py-3 text-slate-400 text-xs", "{nakshatra_name(p.nakshatra)}" }
                                                        td { class: "px-4 py-3 text-slate-500 text-xs", "P{p.pada}" }
                                                        td { class: "px-4 py-3 text-slate-400 text-xs font-mono", "H{p.house_index}" }
                                                        td { class: "px-4 py-3 text-indigo-400 text-xs", "{rasi_name(p.navamsa_rasi)}" }
                                                        td { class: "px-4 py-3 text-center",
                                                            if p.is_retrograde {
                                                                span { class: "px-1.5 py-0.5 rounded text-xs bg-purple-900/50 text-purple-300 border border-purple-700/50", "R" }
                                                            }
                                                            if p.is_combust {
                                                                span { class: "px-1.5 py-0.5 rounded text-xs bg-orange-900/50 text-orange-300 border border-orange-700/50 ml-1", "C" }
                                                            }
                                                        }
                                                    }
                                                }
                                            })}
                                        }
                                    }
                                }
                            }

                            // ── 3. 마하다샤 타임라인 ──────────────────────────────
                            if !data.report.dasha_timeline.is_empty() {
                                div { class: "bg-slate-900 border border-slate-800 rounded-2xl overflow-hidden",
                                    div { class: "bg-slate-800/50 border-b border-slate-800 px-5 py-3",
                                        h3 { class: "font-semibold text-slate-200", "빔쇼따리 마하다샤 (Vimshottari Mahadasha)" }
                                    }
                                    div { class: "overflow-x-auto",
                                        table { class: "w-full text-sm",
                                            thead {
                                                tr { class: "bg-slate-800/30 text-xs text-slate-400 uppercase",
                                                    th { class: "px-4 py-3 text-left font-medium", "행성 (Lord)" }
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
                                                    let row_class = if is_current {
                                                        "bg-indigo-900/20 hover:bg-indigo-900/30"
                                                    } else {
                                                        "hover:bg-slate-800/20"
                                                    };
                                                    rsx! {
                                                        tr { class: "{row_class} transition-colors",
                                                            td { class: "px-4 py-3 font-bold {color}", "{planet_name_kr(d.lord)}" }
                                                            td { class: "px-4 py-3 font-mono text-slate-300 text-xs", "{start_str}" }
                                                            td { class: "px-4 py-3 font-mono text-slate-400 text-xs", "{end_str}" }
                                                            td { class: "px-4 py-3 text-slate-400 text-xs", "{duration_years}년" }
                                                            td { class: "px-4 py-3",
                                                                if is_current {
                                                                    span { class: "px-2 py-0.5 rounded-full text-xs bg-indigo-600/40 text-indigo-200 border border-indigo-500/40 animate-pulse font-semibold",
                                                                        "⬤ 현재"
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
                            }

                            // ── 4. 고차라 현재 영향 ────────────────────────────────
                            div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5",
                                h3 { class: "text-sm font-semibold text-slate-400 uppercase tracking-widest mb-3", "고차라 현재 행성 영향 (Gochara)" }
                                div { class: "flex flex-wrap gap-2",
                                    {data.gochara.transits.iter().map(|inf| {
                                        let (bg, _text_c) = if inf.is_benefic_transit {
                                            ("bg-emerald-900/30 border-emerald-700/50 text-emerald-300", "emerald")
                                        } else {
                                            ("bg-red-900/30 border-red-700/50 text-red-300", "red")
                                        };
                                        let planet_c = planet_color(inf.planet);
                                        rsx! {
                                            div { class: "px-3 py-2 rounded-xl border {bg} flex items-center gap-2 text-sm",
                                                span { class: "font-bold {planet_c}", "{planet_name_kr(inf.planet)}" }
                                                span { class: "text-xs text-slate-400", "→ H{inf.house_from_moon}" }
                                                if inf.is_benefic_transit {
                                                    span { class: "text-xs", "✓" }
                                                } else {
                                                    span { class: "text-xs", "✗" }
                                                }
                                            }
                                        }
                                    })}
                                    if data.gochara.transits.is_empty() {
                                        p { class: "text-slate-500 text-sm", "고차라 데이터가 없습니다." }
                                    }
                                }
                            }

                            // ── 5. 카라카 (Karakas) ───────────────────────────────
                            div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5",
                                h3 { class: "text-sm font-semibold text-slate-400 uppercase tracking-widest mb-3", "카라카 (Jaimini Karakas)" }
                                div { class: "grid grid-cols-2 md:grid-cols-4 gap-3",
                                    KarakaCard { label: "아트마카라카 (영혼)", planet: data.report.primary_karakas.atmakaraka }
                                    KarakaCard { label: "아마티야카라카 (직업)", planet: data.report.primary_karakas.amatyakaraka }
                                    KarakaCard { label: "다라카라카 (배우자)", planet: data.report.primary_karakas.darakaraka }
                                    if let Some(pk) = data.report.primary_karakas.putrakaraka {
                                        KarakaCard { label: "뿌뜨라카라카 (자녀)", planet: pk }
                                    }
                                }
                            }
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
fn KarakaCard(label: &'static str, planet: VedicPlanet) -> Element {
    let color = planet_color(planet);
    rsx! {
        div { class: "p-3 rounded-xl bg-slate-800/50 border border-slate-700/50 flex flex-col gap-1",
            p { class: "text-xs text-slate-500", "{label}" }
            p { class: "font-bold text-sm {color}", "{planet_name_kr(planet)}" }
        }
    }
}
