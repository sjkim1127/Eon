use dioxus::prelude::*;
use crate::store::{AnalysisState, TaskStatus};
use eon_service::dto::{AnalysisInput, SajuAnalysisInput, VedicAnalysisInput, DestinyTierRequest};
use eon_service::facade;

use crate::components::shared::birth_form::BirthForm;

#[component]
pub fn TierTab() -> Element {
    let mut state = use_context::<AnalysisState>();

    let run_analysis = move |_| {
        spawn(async move {
            state.tier.write().status = TaskStatus::Loading;
            
            let form = state.form.read().clone();

            let base_input = AnalysisInput {
                year: form.year,
                month: form.month,
                day: form.day,
                hour: form.hour,
                minute: form.minute,
                is_lunar: form.is_lunar,
                is_leap_month: form.is_leap_month,
                lat: form.lat,
                lon: form.lon,
                timezone: "Asia/Seoul".to_string(),
            };

            let saju_input = SajuAnalysisInput::new(base_input.clone(), form.is_male, false, Some(false));
            let vedic_input = VedicAnalysisInput::new(base_input.clone(), Some(form.is_male), None);
            
            let saju_res = facade::analyze_saju(saju_input).unwrap();
            let vedic_res = facade::analyze_vedic(vedic_input).unwrap();
            
            match facade::analyze_destiny_tier(saju_res, vedic_res, None) {
                Ok(res) => {
                    state.tier.write().data = Some(res);
                    state.tier.write().status = TaskStatus::Success;
                }
                Err(e) => {
                    state.tier.write().error = Some(e.to_string());
                    state.tier.write().status = TaskStatus::Error(e.to_string());
                }
            }
        });
    };

    rsx! {
        div { class: "space-y-6 animate-in fade-in duration-700",
            BirthForm {}
            div { class: "flex justify-between items-center",
                h2 { class: "text-2xl font-bold bg-gradient-to-r from-yellow-200 to-amber-500 bg-clip-text text-transparent",
                    "데스티니 티어 (Destiny Tier)"
                }
                button {
                    class: "px-4 py-2 bg-amber-600 hover:bg-amber-500 rounded-lg font-medium transition-colors",
                    onclick: run_analysis,
                    "티어 산출 (Test)"
                }
            }
            
            match &state.tier.read().status {
                TaskStatus::Idle => rsx! { div { class: "text-slate-400", "분석을 실행해주세요." } },
                TaskStatus::Loading => rsx! { div { class: "text-amber-400 animate-pulse", "분석 중..." } },
                TaskStatus::Error(e) => rsx! { div { class: "text-red-400", "오류 발생: {e}" } },
                TaskStatus::Success => {
                    if let Some(data) = &state.tier.read().data {
                        rsx! {
                            div { class: "p-8 rounded-2xl bg-slate-800/50 border border-slate-700/50 flex flex-col items-center",
                                h3 { class: "text-xl text-slate-400 mb-4", "종합 잠재력 티어" }
                                div { class: "text-7xl font-black bg-gradient-to-br from-yellow-300 via-amber-400 to-orange-500 text-transparent bg-clip-text drop-shadow-[0_0_30px_rgba(251,191,36,0.3)] mb-2",
                                    "{data.destiny_tier.grade}"
                                }
                                div { class: "text-2xl text-amber-200 font-semibold mb-6", "{data.destiny_tier.label}" }
                                
                                div { class: "w-full max-w-md h-4 bg-slate-700 rounded-full overflow-hidden mt-4",
                                    div { 
                                        class: "h-full bg-gradient-to-r from-amber-500 to-yellow-400 transition-all duration-1000",
                                        style: "width: {data.destiny_score}%"
                                    }
                                }
                                div { class: "mt-2 text-slate-300 text-sm", "Score: {data.destiny_score:.1} / 100" }
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
