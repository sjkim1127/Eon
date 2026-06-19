use dioxus::prelude::*;
use crate::store::AnalysisState;
use crate::store::db::{self, UserProfile};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
struct NominatimResult {
    lat: String,
    lon: String,
    display_name: String,
}

#[component]
pub fn BirthForm() -> Element {
    let mut state = use_context::<AnalysisState>();

    // Local state
    let mut profiles = use_signal(Vec::<UserProfile>::new);
    let mut new_profile_name = use_signal(|| String::new());
    let mut city_input = use_signal(|| "서울".to_string());
    let mut geo_status = use_signal(|| String::new()); // "검색 중...", "서울, KR", "오류" 등

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
            // 도시 입력 필드도 위도/경도로 복원
            let lat = profile.form_state.lat;
            let lon = profile.form_state.lon;
            geo_status.set(format!("📍 {:.4}°N, {:.4}°E", lat, lon));
        }
    };

    // 도시 검색 (Nominatim)
    let on_city_geocode = move |_| {
        let query = city_input.read().clone();
        if query.trim().is_empty() {
            return;
        }
        geo_status.set("🔍 검색 중...".to_string());
        spawn(async move {
            let url = format!(
                "https://nominatim.openstreetmap.org/search?q={}&format=json&limit=1&accept-language=ko",
                urlencoding::encode(&query)
            );
            let client = reqwest::Client::builder()
                .build()
                .unwrap_or_else(|_| reqwest::Client::new());
            match client
                .get(&url)
                .header("User-Agent", "EonAstroApp/1.0")
                .send()
                .await
            {
                Ok(resp) => {
                    if let Ok(results) = resp.json::<Vec<NominatimResult>>().await {
                        if let Some(first) = results.into_iter().next() {
                            let lat: f64 = first.lat.parse().unwrap_or(37.5665);
                            let lon: f64 = first.lon.parse().unwrap_or(126.9780);
                            state.form.write().lat = lat;
                            state.form.write().lon = lon;
                            // 표시 이름 축약
                            let short_name = first.display_name
                                .split(',')
                                .take(2)
                                .collect::<Vec<_>>()
                                .join(",");
                            geo_status.set(format!("✅ {}", short_name.trim()));
                        } else {
                            geo_status.set("❌ 결과 없음".to_string());
                        }
                    } else {
                        geo_status.set("❌ 파싱 오류".to_string());
                    }
                }
                Err(_) => {
                    geo_status.set("❌ 네트워크 오류".to_string());
                }
            }
        });
    };

    // Enter 키 검색
    let on_city_keydown = move |evt: Event<KeyboardData>| {
        if evt.key() == Key::Enter {
            let query = city_input.read().clone();
            if query.trim().is_empty() {
                return;
            }
            geo_status.set("🔍 검색 중...".to_string());
            spawn(async move {
                let url = format!(
                    "https://nominatim.openstreetmap.org/search?q={}&format=json&limit=1&accept-language=ko",
                    urlencoding::encode(&query)
                );
                let client = reqwest::Client::new();
                match client
                    .get(&url)
                    .header("User-Agent", "EonAstroApp/1.0")
                    .send()
                    .await
                {
                    Ok(resp) => {
                        if let Ok(results) = resp.json::<Vec<NominatimResult>>().await {
                            if let Some(first) = results.into_iter().next() {
                                let lat: f64 = first.lat.parse().unwrap_or(37.5665);
                                let lon: f64 = first.lon.parse().unwrap_or(126.9780);
                                state.form.write().lat = lat;
                                state.form.write().lon = lon;
                                let short_name = first.display_name
                                    .split(',')
                                    .take(2)
                                    .collect::<Vec<_>>()
                                    .join(",");
                                geo_status.set(format!("✅ {}", short_name.trim()));
                            } else {
                                geo_status.set("❌ 결과 없음".to_string());
                            }
                        } else {
                            geo_status.set("❌ 파싱 오류".to_string());
                        }
                    }
                    Err(_) => {
                        geo_status.set("❌ 네트워크 오류".to_string());
                    }
                }
            });
        }
    };

    let lat_display = state.form.read().lat;
    let lon_display = state.form.read().lon;

    rsx! {
        div { class: "bg-slate-900 border border-slate-800 rounded-xl p-4 flex flex-col gap-3",

            // Profile Row
            div { class: "flex items-center gap-3 pb-3 border-b border-slate-800",
                div { class: "flex flex-col gap-1 flex-1 min-w-0",
                    label { class: "text-xs text-slate-400 font-medium", "저장된 프로필" }
                    select {
                        class: "bg-slate-950 border border-slate-700 rounded-lg px-3 py-2 text-sm focus:outline-none focus:border-violet-500 w-full",
                        onchange: on_select_profile,
                        option { value: "", "--- 불러오기 ---" }
                        {profiles.read().iter().map(|p| rsx! {
                            option { value: "{p.id}", "{p.name}" }
                        })}
                    }
                }
                div { class: "flex items-end gap-2 flex-1 min-w-0",
                    div { class: "flex flex-col gap-1 flex-1 min-w-0",
                        label { class: "text-xs text-slate-400 font-medium", "현재 설정 저장" }
                        input {
                            class: "w-full bg-slate-950 border border-slate-700 rounded-lg px-3 py-2 text-sm focus:outline-none focus:border-violet-500",
                            placeholder: "프로필 이름 (예: 홍길동)",
                            value: "{new_profile_name}",
                            oninput: move |evt| new_profile_name.set(evt.value()),
                        }
                    }
                    button {
                        class: "shrink-0 bg-violet-600 hover:bg-violet-500 text-white text-xs font-medium px-3 py-2 rounded-lg transition-colors",
                        onclick: on_save_profile,
                        "저장"
                    }
                }
            }

            // Input Row
            div { class: "flex flex-wrap gap-3 items-end",
                // 연도
                div { class: "flex flex-col gap-1",
                    label { class: "text-xs text-slate-400 font-medium", "연 (Year)" }
                    input {
                        class: "w-20 bg-slate-950 border border-slate-700 rounded-lg px-3 py-2 text-sm focus:outline-none focus:border-violet-500 transition-colors",
                        r#type: "number", min: "1900", max: "2100",
                        value: "{state.form.read().year}",
                        oninput: move |evt| {
                            if let Ok(v) = evt.value().parse::<i32>() { state.form.write().year = v; }
                        },
                    }
                }
                // 월
                div { class: "flex flex-col gap-1",
                    label { class: "text-xs text-slate-400 font-medium", "월" }
                    input {
                        class: "w-14 bg-slate-950 border border-slate-700 rounded-lg px-3 py-2 text-sm focus:outline-none focus:border-violet-500 transition-colors",
                        r#type: "number", min: "1", max: "12",
                        value: "{state.form.read().month}",
                        oninput: move |evt| {
                            if let Ok(v) = evt.value().parse::<u32>() { state.form.write().month = v; }
                        },
                    }
                }
                // 일
                div { class: "flex flex-col gap-1",
                    label { class: "text-xs text-slate-400 font-medium", "일" }
                    input {
                        class: "w-14 bg-slate-950 border border-slate-700 rounded-lg px-3 py-2 text-sm focus:outline-none focus:border-violet-500 transition-colors",
                        r#type: "number", min: "1", max: "31",
                        value: "{state.form.read().day}",
                        oninput: move |evt| {
                            if let Ok(v) = evt.value().parse::<u32>() { state.form.write().day = v; }
                        },
                    }
                }
                // 시
                div { class: "flex flex-col gap-1",
                    label { class: "text-xs text-slate-400 font-medium", "시" }
                    input {
                        class: "w-14 bg-slate-950 border border-slate-700 rounded-lg px-3 py-2 text-sm focus:outline-none focus:border-violet-500 transition-colors",
                        r#type: "number", min: "0", max: "23",
                        value: "{state.form.read().hour}",
                        oninput: move |evt| {
                            if let Ok(v) = evt.value().parse::<u32>() { state.form.write().hour = v; }
                        },
                    }
                }
                // 분
                div { class: "flex flex-col gap-1",
                    label { class: "text-xs text-slate-400 font-medium", "분" }
                    input {
                        class: "w-14 bg-slate-950 border border-slate-700 rounded-lg px-3 py-2 text-sm focus:outline-none focus:border-violet-500 transition-colors",
                        r#type: "number", min: "0", max: "59",
                        value: "{state.form.read().minute}",
                        oninput: move |evt| {
                            if let Ok(v) = evt.value().parse::<u32>() { state.form.write().minute = v; }
                        },
                    }
                }
                // 출생지 (텍스트 검색)
                div { class: "flex flex-col gap-1 min-w-0",
                    label { class: "text-xs text-slate-400 font-medium",
                        "출생지 (엔터 또는 🔍 로 검색)"
                    }
                    div { class: "flex gap-1",
                        input {
                            class: "w-36 bg-slate-950 border border-slate-700 rounded-lg px-3 py-2 text-sm focus:outline-none focus:border-violet-500 transition-colors",
                            placeholder: "도시명 (예: 서울)",
                            value: "{city_input}",
                            oninput: move |evt| city_input.set(evt.value()),
                            onkeydown: on_city_keydown,
                        }
                        button {
                            class: "bg-slate-700 hover:bg-slate-600 text-slate-200 text-sm px-2 py-2 rounded-lg transition-colors",
                            onclick: on_city_geocode,
                            "🔍"
                        }
                    }
                    // 지오코딩 결과 / 좌표 표시
                    div { class: "text-xs text-slate-500 mt-0.5",
                        if geo_status.read().is_empty() {
                            "📍 {lat_display:.4}°N, {lon_display:.4}°E"
                        } else {
                            "{geo_status}"
                        }
                    }
                }
                // 음력
                div { class: "flex items-center gap-2 mb-1",
                    input {
                        r#type: "checkbox", id: "is_lunar",
                        class: "w-4 h-4 rounded border-slate-700 bg-slate-950 accent-violet-600",
                        checked: "{state.form.read().is_lunar}",
                        onchange: move |evt| state.form.write().is_lunar = evt.value() == "true"
                    }
                    label { r#for: "is_lunar", class: "text-sm text-slate-300 select-none cursor-pointer whitespace-nowrap", "음력" }
                }
                // 남성
                div { class: "flex items-center gap-2 mb-1",
                    input {
                        r#type: "checkbox", id: "is_male",
                        class: "w-4 h-4 rounded border-slate-700 bg-slate-950 accent-violet-600",
                        checked: "{state.form.read().is_male}",
                        onchange: move |evt| state.form.write().is_male = evt.value() == "true"
                    }
                    label { r#for: "is_male", class: "text-sm text-slate-300 select-none cursor-pointer whitespace-nowrap", "남성" }
                }
            }
        }
    }
}
