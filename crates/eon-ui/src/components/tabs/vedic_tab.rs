use dioxus::prelude::*;
use crate::store::{AnalysisState, TaskStatus};
use eon_service::dto::{VedicAnalysisInput, AnalysisInput};
use eon_service::facade;
use chrono::Utc;

use crate::components::shared::birth_form::BirthForm;

#[component]
pub fn VedicTab() -> Element {
    let mut state = use_context::<AnalysisState>();

    let run_analysis = move |_| {
        spawn(async move {
            state.vedic.write().status = TaskStatus::Loading;
            
            let form = state.form.read().clone();

            let input = VedicAnalysisInput::new(
                AnalysisInput {
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
                },
                Some(form.is_male),
                None
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
                    "베딕 차트 (Vedic Charts)"
                }
                button {
                    class: "px-4 py-2 bg-indigo-600 hover:bg-indigo-500 rounded-lg font-medium transition-colors",
                    onclick: run_analysis,
                    "차트 생성 (Test)"
                }
            }
            
            match &state.vedic.read().status {
                TaskStatus::Idle => rsx! { div { class: "text-slate-400", "분석을 실행해주세요." } },
                TaskStatus::Loading => rsx! { div { class: "text-indigo-400 animate-pulse", "생성 중..." } },
                TaskStatus::Error(e) => rsx! { div { class: "text-red-400", "오류 발생: {e}" } },
                TaskStatus::Success => {
                    if let Some(data) = &state.vedic.read().data {
                        rsx! {
                            div { class: "grid grid-cols-2 gap-8",
                                div { class: "space-y-4",
                                    h3 { class: "text-xl font-semibold", "D1 (Rasi) Chart" }
                                    div { class: "grid grid-cols-4 grid-rows-4 gap-2 h-96",
                                        // TODO: Implement South/North Indian chart grid
                                        div { class: "col-span-4 row-span-4 border border-slate-700 rounded-lg bg-slate-800/50 flex items-center justify-center p-4",
                                            pre { class: "text-sm text-slate-300",
                                                "Ascendant Rasi: {data.chart.ascendant.rasi}\nSun Rasi: {data.chart.planets.iter().find(|p| p.planet == eon_vedic::planets::VedicPlanet::Sun).unwrap().rasi}"
                                            }
                                        }
                                    }
                                }
                                div { class: "space-y-4",
                                    h3 { class: "text-xl font-semibold", "D9 (Navamsa) Chart" }
                                    div { class: "grid grid-cols-4 grid-rows-4 gap-2 h-96",
                                        div { class: "col-span-4 row-span-4 border border-slate-700 rounded-lg bg-slate-800/50 flex items-center justify-center p-4",
                                            pre { class: "text-sm text-slate-300",
                                                "Asc Navamsa: {data.chart.ascendant.navamsa_rasi}\nSun Navamsa: {data.chart.planets.iter().find(|p| p.planet == eon_vedic::planets::VedicPlanet::Sun).unwrap().navamsa_rasi}"
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
            }
        }
    }
}
