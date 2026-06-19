use dioxus::prelude::*;
use crate::store::AnalysisState;

#[component]
pub fn BirthForm() -> Element {
    let mut state = use_context::<AnalysisState>();
    
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
        div { class: "bg-slate-900 border border-slate-800 rounded-xl p-4 flex flex-wrap gap-4 items-end",
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
