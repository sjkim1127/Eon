// crates/eon-ui/src/components/tabs/western_tab.rs
use dioxus::prelude::*;
use crate::store::{AnalysisState, TaskStatus};
use crate::i18n::{t, TK, Locale};
use eon_service::facade;
use eon_service::dto::WesternAnalysisInput;
use crate::components::shared::birth_form::BirthForm;

pub fn get_planet_emoji_and_name(name: &str, locale: Locale) -> (&'static str, String) {
    let (emoji, val) = match name {
        "Sun" => ("☀️", match locale { Locale::Ko => "태양 (Sun)", Locale::Zh => "太阳", Locale::Ru => "Солнце", _ => "Sun" }),
        "Moon" => ("🌙", match locale { Locale::Ko => "달 (Moon)", Locale::Zh => "月亮", Locale::Ru => "Луна", _ => "Moon" }),
        "Mercury" => ("☿", match locale { Locale::Ko => "수성 (Mercury)", Locale::Zh => "水星", Locale::Ru => "Меркурий", _ => "Mercury" }),
        "Venus" => ("♀", match locale { Locale::Ko => "금성 (Venus)", Locale::Zh => "金星", Locale::Ru => "Венера", _ => "Venus" }),
        "Mars" => ("♂", match locale { Locale::Ko => "화성 (Mars)", Locale::Zh => "火星", Locale::Ru => "Марс", _ => "Mars" }),
        "Jupiter" => ("♃", match locale { Locale::Ko => "목성 (Jupiter)", Locale::Zh => "木星", Locale::Ru => "Юпитер", _ => "Jupiter" }),
        "Saturn" => ("♄", match locale { Locale::Ko => "토성 (Saturn)", Locale::Zh => "土星", Locale::Ru => "Сатурн", _ => "Saturn" }),
        "Uranus" => ("♅", match locale { Locale::Ko => "천왕성 (Uranus)", Locale::Zh => "天王星", Locale::Ru => "Уран", _ => "Uranus" }),
        "Neptune" => ("♆", match locale { Locale::Ko => "해왕성 (Neptune)", Locale::Zh => "海王星", Locale::Ru => "Нептун", _ => "Neptune" }),
        "Pluto" => ("♇", match locale { Locale::Ko => "명왕성 (Pluto)", Locale::Zh => "冥王星", Locale::Ru => "Плутон", _ => "Pluto" }),
        "Chiron" => ("🔑", match locale { Locale::Ko => "키론 (Chiron)", Locale::Zh => "凯龙星", Locale::Ru => "Хирон", _ => "Chiron" }),
        "True Node" => ("☊", match locale { Locale::Ko => "북노드 (Node)", Locale::Zh => "北交点", Locale::Ru => "Северный узел", _ => "True Node" }),
        _ => ("🪐", name),
    };
    (emoji, val.to_string())
}

pub fn get_sign_emoji_and_name(idx: usize, locale: Locale) -> (&'static str, &'static str) {
    match idx {
        0 => ("♈", match locale { Locale::Ko => "백양자리 (Aries)", Locale::Zh => "白羊座", Locale::Ru => "Овен", _ => "Aries" }),
        1 => ("♉", match locale { Locale::Ko => "황소자리 (Taurus)", Locale::Zh => "金牛座", Locale::Ru => "Телец", _ => "Taurus" }),
        2 => ("♊", match locale { Locale::Ko => "쌍둥이자리 (Gemini)", Locale::Zh => "双子座", Locale::Ru => "Близнецы", _ => "Gemini" }),
        3 => ("♋", match locale { Locale::Ko => "게자리 (Cancer)", Locale::Zh => "巨蟹座", Locale::Ru => "Рак", _ => "Cancer" }),
        4 => ("♌", match locale { Locale::Ko => "사자자리 (Leo)", Locale::Zh => "狮子座", Locale::Ru => "Лев", _ => "Leo" }),
        5 => ("♍", match locale { Locale::Ko => "처녀자리 (Virgo)", Locale::Zh => "处女座", Locale::Ru => "Дева", _ => "Virgo" }),
        6 => ("♎", match locale { Locale::Ko => "천칭자리 (Libra)", Locale::Zh => "天秤座", Locale::Ru => "Весы", _ => "Libra" }),
        7 => ("♏", match locale { Locale::Ko => "전갈자리 (Scorpio)", Locale::Zh => "天蝎座", Locale::Ru => "Скорпион", _ => "Scorpio" }),
        8 => ("♐", match locale { Locale::Ko => "사수자리 (Sagittarius)", Locale::Zh => "射手座", Locale::Ru => "Стрелец", _ => "Sagittarius" }),
        9 => ("♑", match locale { Locale::Ko => "염소자리 (Capricorn)", Locale::Zh => "摩羯座", Locale::Ru => "Козерог", _ => "Capricorn" }),
        10 => ("♒", match locale { Locale::Ko => "물병자리 (Aquarius)", Locale::Zh => "水瓶座", Locale::Ru => "Водолей", _ => "Aquarius" }),
        11 => ("♓", match locale { Locale::Ko => "물고기자리 (Pisces)", Locale::Zh => "双鱼座", Locale::Ru => "Рыбы", _ => "Pisces" }),
        _ => ("❓", "Unknown"),
    }
}

pub fn get_aspect_emoji_and_name(aspect: eon_western::AspectType, locale: Locale) -> (&'static str, &'static str) {
    match aspect {
        eon_western::AspectType::Conjunction => ("☌", match locale { Locale::Ko => "합 (Conjunction)", Locale::Zh => "合相", Locale::Ru => "Соединение", _ => "Conjunction" }),
        eon_western::AspectType::Sextile => ("⚹", match locale { Locale::Ko => "육분의 (Sextile)", Locale::Zh => "六分相", Locale::Ru => "Секстиль", _ => "Sextile" }),
        eon_western::AspectType::Square => ("□", match locale { Locale::Ko => "스퀘어 (Square)", Locale::Zh => "三分相", Locale::Ru => "Квадратура", _ => "Square" }),
        eon_western::AspectType::Trine => ("△", match locale { Locale::Ko => "트라인 (Trine)", Locale::Zh => "三分相", Locale::Ru => "Тригон", _ => "Trine" }),
        eon_western::AspectType::Opposition => ("☍", match locale { Locale::Ko => "대립 (Opposition)", Locale::Zh => "对分相", Locale::Ru => "Оппозиция", _ => "Opposition" }),
    }
}

#[component]
pub fn WesternTab() -> Element {
    let mut state = use_context::<AnalysisState>();
    let locale = *state.locale.read();

    // 현재 선택된 하우스 시스템 상태 (기본값: 'P' Placidus)
    let mut selected_house_sys = use_signal(|| "P".to_string());
    let mut copied_feedback = use_signal(|| false);

    // 분석 실행 함수
    let run_analysis = move |house_sys: String| {
        spawn(async move {
            state.western.write().status = TaskStatus::Loading;
            let form = state.form.read().clone();

            let base_input = form.to_analysis_input();

            let west_input = WesternAnalysisInput {
                base: base_input,
                house_system: house_sys,
            };

            match facade::analyze_western(west_input) {
                Ok(res) => {
                    state.western.write().data = Some(res);
                    state.western.write().status = TaskStatus::Success;
                }
                Err(e) => {
                    state.western.write().error = Some(e.to_string());
                    state.western.write().status = TaskStatus::Error(e.to_string());
                }
            }
        });
    };

    rsx! {
        div { class: "space-y-6 animate-in fade-in duration-700",
            // ── 출생 정보 입력 폼 ──────────────────────────────────────────
            BirthForm {}

            // ── 타이틀 및 상단 컨트롤 버튼 ──────────────────────────────
            div { class: "flex justify-between items-center",
                h2 { class: "text-2xl font-bold bg-gradient-to-r from-pink-400 via-purple-400 to-indigo-400 bg-clip-text text-transparent",
                    "{t(locale, TK::WestTitle)}"
                }
                div { class: "flex items-center gap-3",
                    // 하우스 시스템 드롭다운
                    div { class: "flex items-center bg-slate-900/80 border border-slate-700/50 rounded-xl px-3 py-1.5 gap-2 shadow-inner",
                        span { class: "text-xs font-semibold text-slate-400", "{t(locale, TK::WestHouseSystem)}:" }
                        select {
                            class: "bg-transparent text-slate-200 border-none outline-none text-xs font-bold cursor-pointer",
                            value: "{selected_house_sys.read()}",
                            onchange: move |evt| {
                                let val = evt.value();
                                selected_house_sys.set(val.clone());
                                // 데이터가 이미 로드된 적이 있는 경우 자동 재조회
                                if let TaskStatus::Success = &state.western.read().status {
                                    run_analysis(val);
                                }
                            },
                            option { value: "P", "{t(locale, TK::WestPlacidus)}" }
                            option { value: "K", "{t(locale, TK::WestKoch)}" }
                            option { value: "W", "{t(locale, TK::WestWholeSign)}" }
                            option { value: "E", "{t(locale, TK::WestEqual)}" }
                        }
                    }

                    if let TaskStatus::Success = &state.western.read().status {
                        if let Some(output) = &state.western.read().data {
                            {
                                let data_cloned = output.clone();
                                let form_cloned = state.form.read().clone();
                                let btn_text = if *copied_feedback.read() {
                                    t(locale, TK::WestReportCopiedBtn)
                                } else {
                                    t(locale, TK::WestReportCopyBtn)
                                };
                                let btn_cls = if *copied_feedback.read() {
                                    "px-5 py-2.5 bg-emerald-600 text-white rounded-xl font-semibold shadow-lg transition-all duration-200 active:scale-95 cursor-pointer text-sm"
                                } else {
                                    "px-5 py-2.5 bg-slate-800 hover:bg-slate-700/80 text-slate-200 rounded-xl font-semibold border border-slate-700/60 shadow-lg transition-all duration-200 active:scale-95 cursor-pointer text-sm"
                                };
                                rsx! {
                                    button {
                                        class: "{btn_cls}",
                                        onclick: move |_| {
                                            let txt = crate::components::shared::export_markdown::export_western_to_markdown(&data_cloned, &form_cloned, locale);
                                            crate::components::shared::export_markdown::copy_to_clipboard(&txt);
                                            copied_feedback.set(true);
                                            spawn(async move {
                                                gloo_timers::future::TimeoutFuture::new(2000).await;
                                                copied_feedback.set(false);
                                            });
                                        },
                                        "{btn_text}"
                                    }
                                }
                            }
                        }
                    }

                    button {
                        class: "px-5 py-2.5 bg-gradient-to-r from-purple-600 to-indigo-600 hover:from-purple-500 hover:to-indigo-500 rounded-xl font-semibold text-white shadow-lg shadow-purple-900/30 transition-all duration-200 active:scale-95 cursor-pointer text-sm",
                        onclick: move |_| run_analysis(selected_house_sys.read().clone()),
                        "🔮 {t(locale, TK::FormAnalyzeBtn)}"
                    }
                }
            }

            // ── 상태별 분기 ───────────────────────────────────────────────
            match &state.western.read().status {
                TaskStatus::Idle => rsx! {
                    div { class: "flex flex-col items-center justify-center py-20 gap-3 text-slate-500 bg-slate-900/20 border border-slate-800/40 rounded-2xl backdrop-blur-sm",
                        span { class: "text-5xl animate-bounce", "🪐" }
                        p { class: "text-lg font-medium", "{t(locale, TK::StatusIdleHint)}" }
                    }
                },
                TaskStatus::Loading => rsx! {
                    div { class: "flex flex-col items-center justify-center py-20 gap-3",
                        div { class: "w-12 h-12 rounded-full border-4 border-purple-500/30 border-t-purple-400 animate-spin" }
                        p { class: "text-purple-400 font-medium animate-pulse", "{t(locale, TK::StatusLoading)}" }
                    }
                },
                TaskStatus::Error(err) => rsx! {
                    div { class: "p-6 bg-rose-500/10 border border-rose-500/30 rounded-2xl text-rose-200 text-center space-y-2",
                        h4 { class: "font-bold text-lg", "{t(locale, TK::StatusError)}" }
                        p { class: "text-sm", "{err}" }
                    }
                },
                TaskStatus::Success => {
                    if let Some(out) = &state.western.read().data {
                        let res = &out.result;
                        let asc_sign_idx = (res.ascendant / 30.0).floor() as usize;
                        let (asc_emoji, asc_name) = get_sign_emoji_and_name(asc_sign_idx, locale);
                        let asc_deg = res.ascendant % 30.0;
                        let asc_deg_str = format!("{:.0}° {:.0}'", asc_deg.floor(), (asc_deg.fract() * 60.0).round());
                        
                        let (_, cr_korean_name) = get_planet_emoji_and_name(&res.chart_ruler, locale);
                        
                        let (el_emoji, el_name) = match res.dominant_element.as_str() {
                            "Fire" => ("🔥", match locale { Locale::Ko => "불 (Fire)", _ => "Fire" }),
                            "Earth" => ("⛰️", match locale { Locale::Ko => "흙 (Earth)", _ => "Earth" }),
                            "Air" => ("💨", match locale { Locale::Ko => "공기 (Air)", _ => "Air" }),
                            "Water" => ("💧", match locale { Locale::Ko => "물 (Water)", _ => "Water" }),
                            _ => ("❓", res.dominant_element.as_str()),
                        };

                        let (mo_emoji, mo_name) = match res.dominant_modality.as_str() {
                            "Cardinal" => ("⚡", match locale { Locale::Ko => "활동궁 (Cardinal)", _ => "Cardinal" }),
                            "Fixed" => ("🔒", match locale { Locale::Ko => "고정궁 (Fixed)", _ => "Fixed" }),
                            "Mutable" => ("🌀", match locale { Locale::Ko => "변통궁 (Mutable)", _ => "Mutable" }),
                            _ => ("❓", res.dominant_modality.as_str()),
                        };

                        rsx! {
                            // ── Top Summary Grid (핵심 지표 카드) ──
                            div { class: "grid grid-cols-1 md:grid-cols-4 gap-4",
                                // ASC 카드
                                div { class: "bg-slate-900/60 border border-slate-800/50 rounded-2xl p-5 backdrop-blur-md relative overflow-hidden group shadow-lg",
                                    div { class: "absolute -right-4 -bottom-4 text-slate-800/30 text-7xl font-bold transition-all duration-300 group-hover:scale-110", "ASC" }
                                    p { class: "text-xs font-semibold text-slate-400 uppercase tracking-widest", "Ascendant" }
                                    h3 { class: "text-lg font-bold text-violet-300 mt-2 flex items-center gap-1.5", 
                                        span { "{asc_emoji}" }
                                        span { "{asc_name}" }
                                    }
                                    p { class: "text-sm font-semibold text-slate-300 mt-0.5", "{asc_deg_str}" }
                                }
                                // Chart Ruler 카드
                                div { class: "bg-slate-900/60 border border-slate-800/50 rounded-2xl p-5 backdrop-blur-md relative overflow-hidden group shadow-lg",
                                    div { class: "absolute -right-4 -bottom-4 text-slate-800/30 text-7xl font-bold transition-all duration-300 group-hover:scale-110", "RUL" }
                                    p { class: "text-xs font-semibold text-slate-400 uppercase tracking-widest", "{t(locale, TK::WestChartRuler)}" }
                                    h3 { class: "text-lg font-bold text-pink-300 mt-2 flex items-center gap-1.5",
                                        span { "{cr_korean_name}" }
                                    }
                                    p { class: "text-[11px] text-slate-500 mt-0.5", "Ascendant Ruler" }
                                }
                                // Dominant Element 카드
                                div { class: "bg-slate-900/60 border border-slate-800/50 rounded-2xl p-5 backdrop-blur-md relative overflow-hidden group shadow-lg",
                                    div { class: "absolute -right-4 -bottom-4 text-slate-800/30 text-7xl font-bold transition-all duration-300 group-hover:scale-110", "ELE" }
                                    p { class: "text-xs font-semibold text-slate-400 uppercase tracking-widest", "Dominant Element" }
                                    h3 { class: "text-lg font-bold text-amber-300 mt-2 flex items-center gap-1.5",
                                        span { "{el_emoji}" }
                                        span { "{el_name}" }
                                    }
                                    p { class: "text-[11px] text-slate-500 mt-0.5", "Elemental Temperament" }
                                }
                                // Dominant Modality 카드
                                div { class: "bg-slate-900/60 border border-slate-800/50 rounded-2xl p-5 backdrop-blur-md relative overflow-hidden group shadow-lg",
                                    div { class: "absolute -right-4 -bottom-4 text-slate-800/30 text-7xl font-bold transition-all duration-300 group-hover:scale-110", "MOD" }
                                    p { class: "text-xs font-semibold text-slate-400 uppercase tracking-widest", "Dominant Modality" }
                                    h3 { class: "text-lg font-bold text-emerald-300 mt-2 flex items-center gap-1.5",
                                        span { "{mo_emoji}" }
                                        span { "{mo_name}" }
                                    }
                                    p { class: "text-[11px] text-slate-500 mt-0.5", "Behavioral Modality" }
                                }
                            }

                            // ── Main Body: 2 Column Layout ──
                            div { class: "grid grid-cols-1 lg:grid-cols-12 gap-6",
                                // Left Column (행성 표 및 Cusp 표)
                                div { class: "lg:col-span-7 space-y-6",
                                    // 행성 테이블 카드
                                    div { class: "bg-slate-900/60 border border-slate-800/50 rounded-2xl p-5 backdrop-blur-md shadow-lg",
                                        h3 { class: "text-base font-bold text-slate-300 mb-4 border-b border-slate-800/60 pb-2", "Planet Positions" }
                                        div { class: "overflow-x-auto",
                                            table { class: "w-full text-sm text-left border-collapse",
                                                thead { class: "text-slate-500 text-xs font-semibold border-b border-slate-800",
                                                    tr {
                                                        th { class: "py-2 px-3", "{t(locale, TK::WestPlanet)}" }
                                                        th { class: "py-2 px-3", "{t(locale, TK::WestSign)}" }
                                                        th { class: "py-2 px-3", "{t(locale, TK::WestDegree)}" }
                                                        th { class: "py-2 px-3 text-center", "{t(locale, TK::WestHouse)}" }
                                                    }
                                                }
                                                tbody { class: "divide-y divide-slate-800/50",
                                                    {res.planets.iter().map(|p| {
                                                        let (p_emoji, p_name) = get_planet_emoji_and_name(&p.name, locale);
                                                        let (s_emoji, s_name) = get_sign_emoji_and_name(p.sign_index, locale);
                                                        let p_deg = p.degree_in_sign;
                                                        let p_deg_str = format!("{:.0}° {:.0}'", p_deg.floor(), (p_deg.fract() * 60.0).round());
                                                        
                                                        rsx! {
                                                            tr { key: "{p.id}", class: "hover:bg-slate-800/20 transition-colors",
                                                                // 행성 명칭 및 역행 배지
                                                                td { class: "py-3 px-3 font-medium text-slate-200 flex items-center gap-2",
                                                                    span { class: "text-lg text-slate-400", "{p_emoji}" }
                                                                    span { "{p_name}" }
                                                                    if p.is_retrograde {
                                                                        span { class: "text-[10px] font-bold text-rose-400 bg-rose-500/10 border border-rose-500/20 px-1 rounded", "℞" }
                                                                    }
                                                                }
                                                                // 사인
                                                                td { class: "py-3 px-3 text-slate-300 font-semibold",
                                                                    span { class: "mr-1 text-slate-400", "{s_emoji}" }
                                                                    span { "{s_name}" }
                                                                }
                                                                // 황경도
                                                                td { class: "py-3 px-3 text-slate-400 font-mono text-xs", "{p_deg_str}" }
                                                                // 하우스 배속
                                                                td { class: "py-3 px-3 text-center",
                                                                    span { class: "inline-block bg-indigo-500/10 text-indigo-300 border border-indigo-500/20 text-xs px-2.5 py-0.5 rounded-full font-bold",
                                                                        "{p.house_number}"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    })}
                                                }
                                            }
                                        }
                                    }

                                    // Cusp 테이블 카드
                                    div { class: "bg-slate-900/60 border border-slate-800/50 rounded-2xl p-5 backdrop-blur-md shadow-lg",
                                        h3 { class: "text-base font-bold text-slate-300 mb-4 border-b border-slate-800/60 pb-2", "House Cusps" }
                                        div { class: "grid grid-cols-2 md:grid-cols-3 gap-3",
                                            {res.houses.iter().map(|h| {
                                                let (s_emoji, s_name) = get_sign_emoji_and_name(h.sign_index, locale);
                                                let h_deg = h.degree_in_sign;
                                                let h_deg_str = format!("{:.0}° {:.0}'", h_deg.floor(), (h_deg.fract() * 60.0).round());
                                                
                                                rsx! {
                                                    div { key: "{h.house_number}", class: "flex flex-col bg-slate-950/40 border border-slate-800/60 p-2.5 rounded-xl text-center hover:border-violet-500/40 transition-colors",
                                                        span { class: "text-[10px] text-slate-500 font-bold uppercase tracking-wider", "House {h.house_number}" }
                                                        span { class: "text-xs font-semibold text-slate-200 mt-1 flex items-center justify-center gap-1",
                                                            span { "{s_emoji}" }
                                                            span { "{s_name}" }
                                                        }
                                                        span { class: "text-[10px] text-slate-400 font-mono mt-0.5", "{h_deg_str}" }
                                                    }
                                                }
                                            })}
                                        }
                                    }
                                }

                                // Right Column (지표 분배 및 Aspects)
                                div { class: "lg:col-span-5 space-y-6",
                                    // 원소 및 양태 카드
                                    div { class: "bg-slate-900/60 border border-slate-800/50 rounded-2xl p-5 backdrop-blur-md shadow-lg space-y-5",
                                        // 원소(Elements)
                                        div { class: "space-y-3",
                                            h4 { class: "text-sm font-bold text-slate-400 uppercase tracking-widest", "{t(locale, TK::WestElementsTitle)}" }
                                            div { class: "space-y-2.5",
                                                // Fire
                                                div {
                                                    div { class: "flex justify-between text-xs font-bold text-slate-300",
                                                        span { "🔥 Fire (불)" }
                                                        span { "{res.elements.fire:.1}%" }
                                                    }
                                                    div { class: "h-2 w-full bg-slate-800 rounded-full overflow-hidden mt-1",
                                                        div { class: "h-full bg-gradient-to-r from-red-500 to-amber-500 rounded-full", style: "width: {res.elements.fire}%" }
                                                    }
                                                }
                                                // Earth
                                                div {
                                                    div { class: "flex justify-between text-xs font-bold text-slate-300",
                                                        span { "⛰️ Earth (흙)" }
                                                        span { "{res.elements.earth:.1}%" }
                                                    }
                                                    div { class: "h-2 w-full bg-slate-800 rounded-full overflow-hidden mt-1",
                                                        div { class: "h-full bg-gradient-to-r from-emerald-500 to-teal-500 rounded-full", style: "width: {res.elements.earth}%" }
                                                    }
                                                }
                                                // Air
                                                div {
                                                    div { class: "flex justify-between text-xs font-bold text-slate-300",
                                                        span { "💨 Air (공기)" }
                                                        span { "{res.elements.air:.1}%" }
                                                    }
                                                    div { class: "h-2 w-full bg-slate-800 rounded-full overflow-hidden mt-1",
                                                        div { class: "h-full bg-gradient-to-r from-cyan-400 to-sky-400 rounded-full", style: "width: {res.elements.air}%" }
                                                    }
                                                }
                                                // Water
                                                div {
                                                    div { class: "flex justify-between text-xs font-bold text-slate-300",
                                                        span { "💧 Water (물)" }
                                                        span { "{res.elements.water:.1}%" }
                                                    }
                                                    div { class: "h-2 w-full bg-slate-800 rounded-full overflow-hidden mt-1",
                                                        div { class: "h-full bg-gradient-to-r from-blue-500 to-indigo-500 rounded-full", style: "width: {res.elements.water}%" }
                                                    }
                                                }
                                            }
                                        }
                                        
                                        // 양태(Modalities)
                                        div { class: "space-y-3 pt-3 border-t border-slate-800/60",
                                            h4 { class: "text-sm font-bold text-slate-400 uppercase tracking-widest", "{t(locale, TK::WestModalitiesTitle)}" }
                                            div { class: "space-y-2.5",
                                                // Cardinal
                                                div {
                                                    div { class: "flex justify-between text-xs font-bold text-slate-300",
                                                        span { "⚡ Cardinal (활동)" }
                                                        span { "{res.modalities.cardinal:.1}%" }
                                                    }
                                                    div { class: "h-2 w-full bg-slate-800 rounded-full overflow-hidden mt-1",
                                                        div { class: "h-full bg-gradient-to-r from-purple-500 to-pink-500 rounded-full", style: "width: {res.modalities.cardinal}%" }
                                                    }
                                                }
                                                // Fixed
                                                div {
                                                    div { class: "flex justify-between text-xs font-bold text-slate-300",
                                                        span { "🔒 Fixed (고정)" }
                                                        span { "{res.modalities.fixed:.1}%" }
                                                    }
                                                    div { class: "h-2 w-full bg-slate-800 rounded-full overflow-hidden mt-1",
                                                        div { class: "h-full bg-gradient-to-r from-indigo-500 to-blue-500 rounded-full", style: "width: {res.modalities.fixed}%" }
                                                    }
                                                }
                                                // Mutable
                                                div {
                                                    div { class: "flex justify-between text-xs font-bold text-slate-300",
                                                        span { "🌀 Mutable (변통)" }
                                                        span { "{res.modalities.mutable:.1}%" }
                                                    }
                                                    div { class: "h-2 w-full bg-slate-800 rounded-full overflow-hidden mt-1",
                                                        div { class: "h-full bg-gradient-to-r from-teal-500 to-emerald-400 rounded-full", style: "width: {res.modalities.mutable}%" }
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    // 아스펙트 카드
                                    div { class: "bg-slate-900/60 border border-slate-800/50 rounded-2xl p-5 backdrop-blur-md shadow-lg",
                                        h3 { class: "text-base font-bold text-slate-300 mb-4 border-b border-slate-800/60 pb-2", "{t(locale, TK::WestAspectsTitle)}" }
                                        div { class: "max-h-[480px] overflow-y-auto space-y-2 pr-1",
                                            if res.aspects.is_empty() {
                                                p { class: "text-sm text-slate-500 text-center py-4", "No aspects found." }
                                            } else {
                                                {res.aspects.iter().map(|asp| {
                                                    let (b_a_emoji, b_a_name) = get_planet_emoji_and_name(&asp.body_a_name, locale);
                                                    let (b_b_emoji, b_b_name) = get_planet_emoji_and_name(&asp.body_b_name, locale);
                                                    let (asp_emoji, asp_name) = get_aspect_emoji_and_name(asp.aspect_type, locale);
                                                    
                                                    // 아스펙트 유형에 따른 테두리 색 구분
                                                    let border_color = match asp.aspect_type {
                                                        eon_western::AspectType::Conjunction => "border-violet-500/20 hover:border-violet-500/40 bg-violet-500/5",
                                                        eon_western::AspectType::Sextile => "border-sky-500/20 hover:border-sky-500/40 bg-sky-500/5",
                                                        eon_western::AspectType::Square => "border-red-500/20 hover:border-red-500/40 bg-red-500/5",
                                                        eon_western::AspectType::Trine => "border-emerald-500/20 hover:border-emerald-500/40 bg-emerald-500/5",
                                                        eon_western::AspectType::Opposition => "border-amber-500/20 hover:border-amber-500/40 bg-amber-500/5",
                                                    };
                                                    
                                                    rsx! {
                                                        div {
                                                            key: "{asp.body_a_name}-{asp.body_b_name}-{asp.aspect_type.angle()}",
                                                            class: "flex items-center justify-between p-3 rounded-xl border transition-all duration-200 {border_color}",
                                                            
                                                            div { class: "flex items-center gap-2",
                                                                // 첫번째 천체
                                                                div { class: "flex items-center gap-1.5 text-slate-200 text-xs font-semibold",
                                                                    span { "{b_a_emoji}" }
                                                                    span { "{b_a_name}" }
                                                                }
                                                                
                                                                // 아스펙트 결합 기호
                                                                span { class: "text-slate-400 font-bold", "—" }
                                                                
                                                                // 두번째 천체
                                                                div { class: "flex items-center gap-1.5 text-slate-200 text-xs font-semibold",
                                                                    span { "{b_b_emoji}" }
                                                                    span { "{b_b_name}" }
                                                                }
                                                            }
                                                            
                                                            // 아스펙트 세부 수치 (오브 및 각도 표시)
                                                            div { class: "text-right flex flex-col justify-center",
                                                                span { class: "text-xs font-bold text-slate-300", "{asp_emoji} {asp_name}" }
                                                                span { class: "text-[10px] text-slate-500 font-mono", "Diff: {asp.angle_diff:.1}°, Orb: {asp.orb:.2}°" }
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
                            div { class: "text-slate-500 text-center py-10", "No analysis data." }
                        }
                    }
                }
            }
        }
    }
}
