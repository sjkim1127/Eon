use dioxus::prelude::*;
use crate::store::AnalysisState;
use crate::store::db::{self, UserProfile};

#[component]
pub fn BirthForm() -> Element {
    let mut state = use_context::<AnalysisState>();
    
    // Local state for profiles and saving
    let mut profiles = use_signal(Vec::<UserProfile>::new);
    let mut new_profile_name = use_signal(|| String::new());

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
        }
    };

    // We bind inputs directly to our signals
    let on_year_change = move |evt: Event<FormData>| {
        if let Ok(v) = evt.value().parse::<i32>() {
            state.form.write().year = v;
        }
    };
    
    let on_month_change = move |evt: Event<FormData>| {
        if let Ok(v) = evt.value().parse::<u32>() {
            state.form.write().month = v;
        }
    };
    
    let on_day_change = move |evt: Event<FormData>| {
        if let Ok(v) = evt.value().parse::<u32>() {
            state.form.write().day = v;
        }
    };

    rsx! {
        div { class: "bg-slate-900 border border-slate-800 rounded-xl p-4 flex flex-col gap-4",
            
            // Profile Selector Row
            div { class: "flex items-center gap-4 pb-4 border-b border-slate-800",
                div { class: "flex flex-col gap-1 w-1/3",
                    label { class: "text-xs text-slate-400 font-medium", "저장된 프로필 불러오기" }
                    select {
                        class: "bg-slate-950 border border-slate-700 rounded-lg px-3 py-2 text-sm focus:outline-none focus:border-violet-500",
                        onchange: on_select_profile,
                        option { value: "", "--- 선택하세요 ---" }
                        {profiles.read().iter().map(|p| rsx! {
                            option { value: "{p.id}", "{p.name}" }
                        })}
                    }
                }
                div { class: "flex items-end gap-2 w-2/3",
                    div { class: "flex flex-col gap-1 flex-1",
                        label { class: "text-xs text-slate-400 font-medium", "현재 설정 저장하기" }
                        input {
                            class: "w-full bg-slate-950 border border-slate-700 rounded-lg px-3 py-2 text-sm focus:outline-none focus:border-violet-500",
                            placeholder: "프로필 이름 (예: 친구 홍길동)",
                            value: "{new_profile_name}",
                            oninput: move |evt| new_profile_name.set(evt.value()),
                        }
                    }
                    button {
                        class: "bg-violet-600 hover:bg-violet-500 text-white text-sm font-medium px-4 py-2 rounded-lg transition-colors",
                        onclick: on_save_profile,
                        "저장"
                    }
                }
            }

            // Form Row
            div { class: "flex flex-wrap gap-4 items-end",
                div { class: "flex flex-col gap-1",
                    label { class: "text-xs text-slate-400 font-medium", "연도 (Year)" }
                    input {
                        class: "w-24 bg-slate-950 border border-slate-700 rounded-lg px-3 py-2 text-sm focus:outline-none focus:border-violet-500 transition-colors",
                        r#type: "number",
                        value: "{state.form.read().year}",
                        oninput: on_year_change,
                    }
                }
                div { class: "flex flex-col gap-1",
                    label { class: "text-xs text-slate-400 font-medium", "월 (Month)" }
                    input {
                        class: "w-16 bg-slate-950 border border-slate-700 rounded-lg px-3 py-2 text-sm focus:outline-none focus:border-violet-500 transition-colors",
                        r#type: "number",
                        value: "{state.form.read().month}",
                        oninput: on_month_change,
                    }
                }
                div { class: "flex flex-col gap-1",
                    label { class: "text-xs text-slate-400 font-medium", "일 (Day)" }
                    input {
                        class: "w-16 bg-slate-950 border border-slate-700 rounded-lg px-3 py-2 text-sm focus:outline-none focus:border-violet-500 transition-colors",
                        r#type: "number",
                        value: "{state.form.read().day}",
                        oninput: on_day_change,
                    }
                }
                div { class: "flex items-center gap-2 mb-2 ml-4",
                    input {
                        r#type: "checkbox",
                        id: "is_lunar",
                        class: "w-4 h-4 rounded border-slate-700 bg-slate-950 accent-violet-600",
                        checked: "{state.form.read().is_lunar}",
                        onchange: move |evt| state.form.write().is_lunar = evt.value() == "true"
                    }
                    label { r#for: "is_lunar", class: "text-sm text-slate-300 select-none cursor-pointer", "음력 (Lunar)" }
                }
                div { class: "flex items-center gap-2 mb-2 ml-4",
                    input {
                        r#type: "checkbox",
                        id: "is_male",
                        class: "w-4 h-4 rounded border-slate-700 bg-slate-950 accent-violet-600",
                        checked: "{state.form.read().is_male}",
                        onchange: move |evt| state.form.write().is_male = evt.value() == "true"
                    }
                    label { r#for: "is_male", class: "text-sm text-slate-300 select-none cursor-pointer", "남성 (Male)" }
                }
            }
        }
    }
}
