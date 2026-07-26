use crate::i18n::{t, TK};
use crate::store::db::{self, UserProfile};
use crate::store::AnalysisState;
use dioxus::prelude::*;

use crate::utils::geocode::CityRecord;

#[component]
pub fn BirthForm() -> Element {
    let mut state = use_context::<AnalysisState>();
    let locale = *state.locale.read();

    // Local state
    let mut profiles = use_signal(Vec::<UserProfile>::new);
    let mut new_profile_name = use_signal(String::new);
    let mut city_input = use_signal(String::new);
    let mut geo_status = use_signal(String::new);
    let mut search_results = use_signal(Vec::<CityRecord>::new);

    // Load profiles on mount
    use_effect(move || {
        spawn(async move {
            if let Ok(loaded) = db::load_all_profiles().await {
                profiles.set(loaded);
            }
        });
    });

    let on_save_profile = move |_| {
        spawn(async move {
            let name = new_profile_name.read().clone();
            if !name.is_empty() {
                let form_data = state.form.read().clone();
                if let Ok(new_profile) = db::save_profile(name.clone(), form_data).await {
                    profiles.write().insert(0, new_profile);
                    new_profile_name.set(String::new());
                }
            }
        });
    };

    let on_select_profile = move |evt: Event<FormData>| {
        let selected_id = evt.value();
        if let Some(profile) = profiles.read().iter().find(|p| p.id == selected_id) {
            *state.form.write() = profile.form_state.clone();
            let lat = profile.form_state.lat;
            let lon = profile.form_state.lon;
            geo_status.set(format!("📍 {:.4}°N, {:.4}°E", lat, lon));
        }
    };

    let mut select_city = move |result: CityRecord| {
        state.form.write().lat = result.lat;
        state.form.write().lon = result.lon;
        // Optionally save timezone if added to form state later
        let display_name = if let Some(ko) = &result.name_ko {
            format!("{} ({})", ko, result.name)
        } else {
            result.name.clone()
        };
        geo_status.set(format!("✅ {}, {}", display_name, result.country));
        search_results.set(Vec::new());
        city_input.set(String::new());
    };

    let geo_searching_str = t(locale, TK::GeoSearching);
    let geo_no_result_str = t(locale, TK::GeoNoResult);

    // 도시 검색 (Local Offline DB)
    let on_city_geocode = move |_| {
        let query = city_input.read().clone();
        if query.trim().is_empty() {
            return;
        }
        geo_status.set(geo_searching_str.to_string());

        let results = crate::utils::geocode::search_city(&query, 15);
        if results.is_empty() {
            geo_status.set(geo_no_result_str.to_string());
            search_results.set(Vec::new());
        } else {
            geo_status.set(format!("✅ ({} results)", results.len()));
            search_results.set(results);
        }
    };

    // Enter key search
    let on_city_keydown = move |evt: Event<KeyboardData>| {
        if evt.key() == Key::Enter {
            let query = city_input.read().clone();
            if query.trim().is_empty() {
                return;
            }
            geo_status.set(geo_searching_str.to_string());
            let results = crate::utils::geocode::search_city(&query, 15);
            if results.is_empty() {
                geo_status.set(geo_no_result_str.to_string());
                search_results.set(Vec::new());
            } else {
                geo_status.set(format!("✅ ({} results)", results.len()));
                search_results.set(results);
            }
        }
    };

    let lat_display = state.form.read().lat;
    let lon_display = state.form.read().lon;

    rsx! {
        div { class: "glass-premium rounded-2xl p-4 sm:p-5 flex flex-col gap-4 border border-white/5",
            // Profile management
            div { class: "grid grid-cols-1 sm:grid-cols-2 gap-3 pt-1 border-b border-white/5 pb-3",
                div { class: "flex flex-col gap-1.5 min-w-0",
                    label { class: "text-xs text-slate-400 font-semibold tracking-wide", "{t(locale, TK::FormSavedProfiles)}" }
                    select {
                        class: "w-full bg-white/5 border border-white/10 rounded-xl px-3 min-h-[44px] py-2.5 text-sm text-slate-200 focus-glow transition-all outline-none cursor-pointer",
                        onchange: on_select_profile,
                        option { value: "", class: "bg-brand-950 text-slate-400", "-- {t(locale, TK::FormLoadProfile)} --" }
                        {profiles.read().iter().map(|p| rsx! {
                            option { value: "{p.id}", class: "bg-brand-950 text-slate-300", "{p.name}" }
                        })}
                    }
                }
                div { class: "flex items-end gap-2 min-w-0",
                    div { class: "flex flex-col gap-1.5 flex-1 min-w-0",
                        label { class: "text-xs text-slate-400 font-semibold tracking-wide", "{t(locale, TK::FormSaveProfile)}" }
                        input {
                            class: "w-full bg-white/5 border border-white/10 rounded-xl px-3 min-h-[44px] py-2 text-sm text-slate-200 focus-glow transition-all",
                            placeholder: "{t(locale, TK::FormProfileNamePlaceholder)}",
                            value: "{new_profile_name}",
                            oninput: move |evt| new_profile_name.set(evt.value()),
                        }
                    }
                    button {
                        class: "shrink-0 bg-gradient-to-r from-violet-600 to-indigo-600 hover:from-violet-500 hover:to-indigo-500 text-white text-xs font-semibold px-4 min-h-[44px] rounded-xl transition-all shadow-md shadow-violet-950/20 active:scale-95 cursor-pointer flex items-center justify-center",
                        onclick: on_save_profile,
                        "{t(locale, TK::FormSaveBtn)}"
                    }
                }
            }

            // Input Row (Responsive Grid for Mobile)
            div { class: "space-y-3",
                div { class: "grid grid-cols-3 sm:grid-cols-5 gap-2 sm:gap-3 items-end",
                    // Year
                    div { class: "flex flex-col gap-1.5 col-span-1",
                        label { class: "text-xs text-slate-400 font-semibold tracking-wide", "{t(locale, TK::FormYear)}" }
                        input {
                            class: "w-full bg-white/5 border border-white/10 rounded-xl px-2.5 min-h-[44px] text-sm text-slate-200 focus-glow transition-all text-center font-mono font-semibold",
                            r#type: "number", min: "1900", max: "2100",
                            value: "{state.form.read().year}",
                            oninput: move |evt| {
                                if let Ok(v) = evt.value().parse::<i32>() { state.form.write().year = v; }
                            },
                        }
                    }
                    // Month
                    div { class: "flex flex-col gap-1.5 col-span-1",
                        label { class: "text-xs text-slate-400 font-semibold tracking-wide", "{t(locale, TK::FormMonth)}" }
                        input {
                            class: "w-full bg-white/5 border border-white/10 rounded-xl px-2.5 min-h-[44px] text-sm text-slate-200 focus-glow transition-all text-center font-mono font-semibold",
                            r#type: "number", min: "1", max: "12",
                            value: "{state.form.read().month}",
                            oninput: move |evt| {
                                if let Ok(v) = evt.value().parse::<u32>() { state.form.write().month = v; }
                            },
                        }
                    }
                    // Day
                    div { class: "flex flex-col gap-1.5 col-span-1",
                        label { class: "text-xs text-slate-400 font-semibold tracking-wide", "{t(locale, TK::FormDay)}" }
                        input {
                            class: "w-full bg-white/5 border border-white/10 rounded-xl px-2.5 min-h-[44px] text-sm text-slate-200 focus-glow transition-all text-center font-mono font-semibold",
                            r#type: "number", min: "1", max: "31",
                            value: "{state.form.read().day}",
                            oninput: move |evt| {
                                if let Ok(v) = evt.value().parse::<u32>() { state.form.write().day = v; }
                            },
                        }
                    }
                    // Hour
                    div { class: "flex flex-col gap-1.5 col-span-1 sm:col-span-1",
                        label { class: "text-xs text-slate-400 font-semibold tracking-wide", "{t(locale, TK::FormHour)}" }
                        input {
                            class: "w-full bg-white/5 border border-white/10 rounded-xl px-2.5 min-h-[44px] text-sm text-slate-200 focus-glow transition-all text-center font-mono font-semibold",
                            r#type: "number", min: "0", max: "23",
                            value: "{state.form.read().hour}",
                            oninput: move |evt| {
                                if let Ok(v) = evt.value().parse::<u32>() { state.form.write().hour = v; }
                            },
                        }
                    }
                    // Minute
                    div { class: "flex flex-col gap-1.5 col-span-1 sm:col-span-1",
                        label { class: "text-xs text-slate-400 font-semibold tracking-wide", "{t(locale, TK::FormMinute)}" }
                        input {
                            class: "w-full bg-white/5 border border-white/10 rounded-xl px-2.5 min-h-[44px] text-sm text-slate-200 focus-glow transition-all text-center font-mono font-semibold",
                            r#type: "number", min: "0", max: "59",
                            value: "{state.form.read().minute}",
                            oninput: move |evt| {
                                if let Ok(v) = evt.value().parse::<u32>() { state.form.write().minute = v; }
                            },
                        }
                    }
                }

                // Birthplace & Checkbox Row
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-3 items-end pt-1",
                    // Birthplace (text search)
                    div { class: "flex flex-col gap-1.5 relative w-full",
                        label { class: "text-xs text-slate-400 font-semibold tracking-wide",
                            "{t(locale, TK::FormBirthplace)}"
                        }
                        div { class: "flex gap-1.5",
                            input {
                                class: "w-full bg-white/5 border border-white/10 rounded-xl px-3 min-h-[44px] text-sm text-slate-200 focus-glow transition-all",
                                placeholder: "{t(locale, TK::FormCityPlaceholder)}",
                                value: "{city_input}",
                                oninput: move |evt| city_input.set(evt.value()),
                                onkeydown: on_city_keydown,
                            }
                            button {
                                class: "bg-white/10 hover:bg-white/15 active:scale-95 text-slate-200 text-sm px-3.5 min-h-[44px] rounded-xl transition-all cursor-pointer border border-white/5 shrink-0 flex items-center justify-center",
                                onclick: on_city_geocode,
                                "🔍"
                            }
                        }
                        if !search_results.read().is_empty() {
                            div { class: "absolute top-full left-0 mt-1.5 w-full bg-[#0e0f22]/95 border border-white/10 rounded-xl shadow-2xl backdrop-blur-2xl z-50 overflow-hidden max-h-60 overflow-y-auto",
                                {search_results.read().iter().map(|result| {
                                    let r = result.clone();
                                    let display = if let Some(ko) = &r.name_ko {
                                        format!("{} ({}), {} - {}", ko, r.name, r.country, r.tz)
                                    } else {
                                        format!("{}, {} - {}", r.name, r.country, r.tz)
                                    };
                                    rsx! {
                                        div {
                                            class: "px-3.5 py-3 text-xs text-slate-300 hover:bg-violet-600/30 hover:text-violet-200 border-b border-white/5 last:border-0 cursor-pointer transition-colors truncate",
                                            onclick: move |_| select_city(r.clone()),
                                            "{display}"
                                        }
                                    }
                                })}
                            }
                        }
                        div { class: "text-[10px] text-slate-500 tracking-wider font-semibold",
                            if geo_status.read().is_empty() {
                                "📍 {lat_display:.4}°N, {lon_display:.4}°E"
                            } else {
                                "{geo_status}"
                            }
                        }
                    }

                    // Form Checkboxes Section (Touch-friendly Pill Toggles)
                    div { class: "grid grid-cols-3 gap-2 min-h-[44px]",
                        // Lunar calendar toggle
                        div {
                            class: if state.form.read().is_lunar {
                                "flex items-center justify-center gap-1.5 p-2 rounded-xl border border-violet-500/40 bg-violet-600/20 text-violet-200 font-bold text-xs cursor-pointer active:scale-95 transition-all select-none min-h-[44px]"
                            } else {
                                "flex items-center justify-center gap-1.5 p-2 rounded-xl border border-white/10 bg-white/5 text-slate-400 text-xs cursor-pointer hover:bg-white/10 active:scale-95 transition-all select-none min-h-[44px]"
                            },
                            onclick: move |_| {
                                let val = state.form.read().is_lunar;
                                state.form.write().is_lunar = !val;
                            },
                            span { if state.form.read().is_lunar { "🌙" } else { "☀️" } }
                            span { "{t(locale, TK::FormLunar)}" }
                        }
                        // Male/Female toggle
                        div {
                            class: if state.form.read().is_male {
                                "flex items-center justify-center gap-1.5 p-2 rounded-xl border border-indigo-500/40 bg-indigo-600/20 text-indigo-200 font-bold text-xs cursor-pointer active:scale-95 transition-all select-none min-h-[44px]"
                            } else {
                                "flex items-center justify-center gap-1.5 p-2 rounded-xl border border-pink-500/40 bg-pink-600/20 text-pink-200 font-bold text-xs cursor-pointer active:scale-95 transition-all select-none min-h-[44px]"
                            },
                            onclick: move |_| {
                                let val = state.form.read().is_male;
                                state.form.write().is_male = !val;
                            },
                            span { if state.form.read().is_male { "♂️" } else { "♀️" } }
                            span { "{t(locale, TK::FormMale)}" }
                        }
                        // Night Rat Hour toggle
                        div {
                            class: if state.form.read().use_night_rat_hour {
                                "flex items-center justify-center gap-1.5 p-2 rounded-xl border border-amber-500/40 bg-amber-600/20 text-amber-200 font-bold text-xs cursor-pointer active:scale-95 transition-all select-none min-h-[44px]"
                            } else {
                                "flex items-center justify-center gap-1.5 p-2 rounded-xl border border-white/10 bg-white/5 text-slate-400 text-xs cursor-pointer hover:bg-white/10 active:scale-95 transition-all select-none min-h-[44px]"
                            },
                            onclick: move |_| {
                                let val = state.form.read().use_night_rat_hour;
                                state.form.write().use_night_rat_hour = !val;
                            },
                            span { "🌌" }
                            span { "{t(locale, TK::FormUseNightRatHour)}" }
                        }
                    }
                }
            }
        }
    }
}
