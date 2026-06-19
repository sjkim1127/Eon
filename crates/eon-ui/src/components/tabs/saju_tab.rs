use dioxus::prelude::*;
use crate::store::{AnalysisState, TaskStatus};
use eon_service::dto::{SajuAnalysisInput, AnalysisInput};
use eon_service::facade;
use chrono::Utc;

use crate::components::shared::birth_form::BirthForm;

#[component]
pub fn SajuTab() -> Element {
    let mut state = use_context::<AnalysisState>();

    let run_analysis = move |_| {
        spawn(async move {
            state.saju.write().status = TaskStatus::Loading;
            
            let form = state.form.read().clone();

            let input = SajuAnalysisInput::new(
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
                form.is_male,
                false,
                Some(false)
            );

            match facade::analyze_saju(input) {
                Ok(res) => {
                    state.saju.write().data = Some(res);
                    state.saju.write().status = TaskStatus::Success;
                }
                Err(e) => {
                    state.saju.write().error = Some(e.to_string());
                    state.saju.write().status = TaskStatus::Error(e.to_string());
                }
            }
        });
    };

    rsx! {
        div { class: "space-y-6 animate-in fade-in slide-in-from-bottom-4 duration-700",
            BirthForm {}
            div { class: "flex justify-between items-center",
                h2 { class: "text-2xl font-bold bg-gradient-to-r from-amber-200 to-orange-400 bg-clip-text text-transparent",
                    "사주 명식 (Saju Chart)"
                }
                button {
                    class: "px-4 py-2 bg-violet-600 hover:bg-violet-500 rounded-lg font-medium transition-colors",
                    onclick: run_analysis,
                    "분석 실행 (Test)"
                }
            }
            
            match &state.saju.read().status {
                TaskStatus::Idle => rsx! { div { class: "text-slate-400", "분석을 실행해주세요." } },
                TaskStatus::Loading => rsx! { div { class: "text-amber-400 animate-pulse", "분석 중..." } },
                TaskStatus::Error(e) => rsx! { div { class: "text-red-400", "오류 발생: {e}" } },
                TaskStatus::Success => {
                    if let Some(data) = &state.saju.read().data {
                        rsx! {
                            div { class: "grid grid-cols-4 gap-4",
                                PillarCard { title: "시주(Hour)", hanja: data.report.pillars.hour.hanja(), hangul: data.report.pillars.hour.hangul() }
                                PillarCard { title: "일주(Day)", hanja: data.report.pillars.day.hanja(), hangul: data.report.pillars.day.hangul() }
                                PillarCard { title: "월주(Month)", hanja: data.report.pillars.month.hanja(), hangul: data.report.pillars.month.hangul() }
                                PillarCard { title: "연주(Year)", hanja: data.report.pillars.year.hanja(), hangul: data.report.pillars.year.hangul() }
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
fn PillarCard(title: &'static str, hanja: String, hangul: String) -> Element {
    rsx! {
        div { class: "p-6 rounded-2xl bg-slate-800/50 border border-slate-700/50 flex flex-col items-center justify-center gap-2",
            span { class: "text-sm text-slate-400 font-medium", "{title}" }
            span { class: "text-4xl font-serif text-slate-100", "{hanja}" }
            span { class: "text-lg text-slate-300", "{hangul}" }
        }
    }
}
