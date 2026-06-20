use dioxus::prelude::*;
use crate::store::{AnalysisState, TaskStatus};
use crate::i18n::{t, TK, Locale};
use eon_service::dto::{SajuAnalysisInput, TransitAnalysisInput, AnalysisInput};
use eon_service::facade;
use eon_saju::LuckDirection;
use crate::components::shared::birth_form::BirthForm;

#[component]
pub fn TransitTab() -> Element {
    let mut state = use_context::<AnalysisState>();
    let locale = *state.locale.read();

    let run_analysis = move |_| {
        spawn(async move {
            state.transit.write().status = TaskStatus::Loading;

            let form = state.form.read().clone();
            let base = AnalysisInput {
                year: form.year, month: form.month, day: form.day,
                hour: form.hour, minute: form.minute,
                is_lunar: form.is_lunar, is_leap_month: form.is_leap_month,
                lat: form.lat, lon: form.lon,
                timezone: "Asia/Seoul".to_string(),
            };
            let saju_input = SajuAnalysisInput::new(base, form.is_male, false, Some(false));
            let transit_input = TransitAnalysisInput::new(saju_input, None);

            match facade::analyze_transit(transit_input) {
                Ok(res) => {
                    state.transit.write().data = Some(res);
                    state.transit.write().status = TaskStatus::Success;
                }
                Err(e) => {
                    state.transit.write().error = Some(e.to_string());
                    state.transit.write().status = TaskStatus::Error(e.to_string());
                }
            }

            // 사주 데이터도 없으면 함께 분석 (대운 타임라인 위해)
            if !matches!(state.saju.read().status, TaskStatus::Success) {
                let form2 = state.form.read().clone();
                let base2 = AnalysisInput {
                    year: form2.year, month: form2.month, day: form2.day,
                    hour: form2.hour, minute: form2.minute,
                    is_lunar: form2.is_lunar, is_leap_month: form2.is_leap_month,
                    lat: form2.lat, lon: form2.lon,
                    timezone: "Asia/Seoul".to_string(),
                };
                if let Ok(saju_res) = facade::analyze_saju(SajuAnalysisInput::new(base2, form2.is_male, false, Some(false))) {
                    state.saju.write().data = Some(saju_res);
                    state.saju.write().status = TaskStatus::Success;
                }
            }
        });
    };

    rsx! {
        div { class: "space-y-6 animate-in fade-in duration-700",
            BirthForm {}

            div { class: "flex justify-between items-center",
                h2 { class: "text-2xl font-bold bg-gradient-to-r from-sky-200 to-blue-400 bg-clip-text text-transparent",
                    "{t(locale, TK::SectionTransit)}"
                }
                button {
                    class: "px-5 py-2.5 bg-gradient-to-r from-sky-700 to-blue-700 hover:from-sky-600 hover:to-blue-600 rounded-xl font-semibold text-white shadow-lg transition-all duration-200 active:scale-95",
                    onclick: run_analysis,
                    "{t(locale, TK::BtnAnalyze)} ⏳"
                }
            }

            match &state.transit.read().status {
                TaskStatus::Idle => rsx! {
                    div { class: "flex flex-col items-center justify-center py-20 gap-3 text-slate-500",
                        span { class: "text-5xl", "🌊" }
                        p { class: "text-lg font-medium", "{t(locale, TK::StatusIdleHint)}" }
                    }
                },
                TaskStatus::Loading => rsx! {
                    div { class: "flex flex-col items-center justify-center py-16 gap-3",
                        div { class: "w-12 h-12 rounded-full border-4 border-sky-500/30 border-t-sky-400 animate-spin" }
                        p { class: "text-sky-400 font-medium animate-pulse", "{t(locale, TK::StatusLoadingTransit)}" }
                    }
                },
                TaskStatus::Error(e) => rsx! {
                    div { class: "p-4 rounded-xl bg-red-900/20 border border-red-800/50 text-red-400", "{t(locale, TK::StatusError)}: {e}" }
                },
                TaskStatus::Success => {
                    if let Some(transit) = &state.transit.read().data {
                        rsx! {
                            // ── 현재 운세 요약 카드 ─────────────────────────────
                            {
                                let age_lbl = t(locale, TK::LabelAge);
                                let age_val = match locale {
                                    Locale::Ko => format!("만 {}세", transit.current_age),
                                    Locale::En => format!("Age {}", transit.current_age),
                                    Locale::Zh => format!("{}岁", transit.current_age),
                                    Locale::Ru => format!("{} лет", transit.current_age),
                                };
                                let yearly_luck_title = match locale {
                                    Locale::Ko => format!("세운 {}년", transit.yearly_luck.year),
                                    Locale::En => format!("Annual Luck {}", transit.yearly_luck.year),
                                    Locale::Zh => format!("流年 {}年", transit.yearly_luck.year),
                                    Locale::Ru => format!("Год {}", transit.yearly_luck.year),
                                };
                                let monthly_luck_title = match locale {
                                    Locale::Ko => format!("월운 {}월", transit.monthly_luck.month),
                                    Locale::En => format!("Monthly Luck {}", transit.monthly_luck.month),
                                    Locale::Zh => format!("流月 {}月", transit.monthly_luck.month),
                                    Locale::Ru => format!("Месяц {}", transit.monthly_luck.month),
                                };
                                let daily_luck_title = match locale {
                                    Locale::Ko => "일운 (오늘)".to_string(),
                                    Locale::En => "Daily Luck (Today)".to_string(),
                                    Locale::Zh => "流日 (今日)".to_string(),
                                    Locale::Ru => "День (Сегодня)".to_string(),
                                };
                                let transit_notes_title = match locale {
                                    Locale::Ko => "⚠️ 세운 특이사항",
                                    Locale::En => "⚠️ Annual Luck Highlights",
                                    Locale::Zh => "⚠️ 流年特别注意事项",
                                    Locale::Ru => "⚠️ Особые события года",
                                };
                                rsx! {
                                    div { class: "grid grid-cols-1 md:grid-cols-2 xl:grid-cols-4 gap-4",
                                        // 현재 나이
                                        LuckCard {
                                            title: "{age_lbl}",
                                            value: "{age_val}",
                                            sub: "".to_string(),
                                            color: "from-slate-800 to-slate-900 border-slate-700",
                                            icon: "👤",
                                        }
                                        // 세운 (연운)
                                        LuckCard {
                                            title: "{yearly_luck_title}",
                                            value: format!("{} ({})", transit.yearly_luck.ganzi.hanja(), transit.yearly_luck.ganzi.hangul()),
                                            sub: format!("{}/{}", transit.yearly_luck.stem_god.hangul(), transit.yearly_luck.branch_god.hangul()),
                                            color: "from-amber-900/40 to-slate-900 border-amber-700/40",
                                            icon: "🌟",
                                        }
                                        // 월운
                                        LuckCard {
                                            title: "{monthly_luck_title}",
                                            value: format!("{} ({})", transit.monthly_luck.ganzi.hanja(), transit.monthly_luck.ganzi.hangul()),
                                            sub: format!("{}/{}", transit.monthly_luck.stem_god.hangul(), transit.monthly_luck.branch_god.hangul()),
                                            color: "from-blue-900/40 to-slate-900 border-blue-700/40",
                                            icon: "📅",
                                        }
                                        // 일운
                                        LuckCard {
                                            title: "{daily_luck_title}",
                                            value: format!("{} ({})", transit.daily_luck.ganzi.hanja(), transit.daily_luck.ganzi.hangul()),
                                            sub: format!("{}/{}", transit.daily_luck.stem_god.hangul(), transit.daily_luck.branch_god.hangul()),
                                            color: "from-emerald-900/40 to-slate-900 border-emerald-700/40",
                                            icon: "📆",
                                        }
                                    }

                                    // ── 세운 특이사항 ─────────────────────────────────
                                    if !transit.yearly_luck.special_events.is_empty() {
                                        div { class: "p-4 rounded-xl bg-amber-900/20 border border-amber-700/40",
                                            p { class: "text-sm font-semibold text-amber-300 mb-2", "{transit_notes_title}" }
                                            div { class: "flex flex-wrap gap-2",
                                                {transit.yearly_luck.special_events.iter().map(|e| rsx! {
                                                    span { class: "px-2 py-1 rounded-lg bg-amber-900/40 text-amber-200 text-xs border border-amber-700/50", "{e}" }
                                                })}
                                            }
                                        }
                                    }
                                }
                            }

                            // ── 대운 타임라인 (사주에서) ───────────────────────
                            if let Some(saju) = &state.saju.read().data {
                                if let Some(ml) = &saju.report.major_luck {
                                    {
                                        let major_luck_title = match locale {
                                            Locale::Ko => "대운 (大運) 타임라인",
                                            Locale::En => "Major Luck Timeline",
                                            Locale::Zh => "大运时间线",
                                            Locale::Ru => "Хронология периодов Да Вун",
                                        };
                                        let major_luck_sub = match locale {
                                            Locale::Ko => format!("만 {}세 교운 | {}", ml.start_age, ml.direction),
                                            Locale::En => format!("Shift age {} | {}", ml.start_age, if ml.direction == LuckDirection::Forward { "Direct" } else { "Reverse" }),
                                            Locale::Zh => format!("{}岁交运 | {}", ml.start_age, if ml.direction == LuckDirection::Forward { "顺行" } else { "逆行" }),
                                            Locale::Ru => format!("Смена в {} лет | {}", ml.start_age, if ml.direction == LuckDirection::Forward { "Прямо" } else { "Обратно" }),
                                        };
                                        let th_age = match locale {
                                            Locale::Ko => "나이", Locale::En => "Age", Locale::Zh => "年龄", Locale::Ru => "Возраст",
                                        };
                                        let th_pillar = match locale {
                                            Locale::Ko => "간지", Locale::En => "Pillar", Locale::Zh => "干支", Locale::Ru => "Столп",
                                        };
                                        let th_stem_god = match locale {
                                            Locale::Ko => "천간 십성", Locale::En => "Stem Ten God", Locale::Zh => "天干十神", Locale::Ru => "Божество НС",
                                        };
                                        let th_branch_god = match locale {
                                            Locale::Ko => "지지 십성", Locale::En => "Branch Ten God", Locale::Zh => "地支十神", Locale::Ru => "Божество ЗВ",
                                        };
                                        let th_status = match locale {
                                            Locale::Ko => "상태", Locale::En => "Status", Locale::Zh => "状态", Locale::Ru => "Статус",
                                        };
                                        let current_lbl = match locale {
                                            Locale::Ko => "⬤ 현재", Locale::En => "⬤ Current", Locale::Zh => "⬤ 当前", Locale::Ru => "⬤ Текущий",
                                        };

                                        rsx! {
                                            div { class: "bg-slate-900 border border-slate-800 rounded-2xl overflow-hidden",
                                                div { class: "bg-slate-800/50 border-b border-slate-800 px-5 py-3 flex items-center justify-between",
                                                    h3 { class: "font-semibold text-slate-200", "{major_luck_title}" }
                                                    span { class: "text-xs text-slate-400", "{major_luck_sub}" }
                                                }
                                                div { class: "overflow-x-auto",
                                                    table { class: "w-full text-sm",
                                                        thead {
                                                            tr { class: "bg-slate-800/30 text-xs text-slate-400 uppercase",
                                                                th { class: "px-4 py-3 text-left font-medium", "{th_age}" }
                                                                th { class: "px-4 py-3 text-left font-medium", "{th_pillar}" }
                                                                th { class: "px-4 py-3 text-left font-medium", "{th_stem_god}" }
                                                                th { class: "px-4 py-3 text-left font-medium", "{th_branch_god}" }
                                                                th { class: "px-4 py-3 text-left font-medium", "{th_status}" }
                                                            }
                                                        }
                                                        tbody { class: "divide-y divide-slate-800",
                                                            {ml.cycles.iter().map(|cycle| {
                                                                let is_current = cycle.start_age <= transit.current_age && transit.current_age <= cycle.end_age;
                                                                let row_class = if is_current {
                                                                    "bg-amber-900/20 hover:bg-amber-900/30"
                                                                } else {
                                                                    "hover:bg-slate-800/20"
                                                                };
                                                                let age_range_str = match locale {
                                                                    Locale::Ko => format!("만 {}~{}세", cycle.start_age, cycle.end_age),
                                                                    Locale::En => format!("Age {}~{}", cycle.start_age, cycle.end_age),
                                                                    Locale::Zh => format!("{}~{}岁", cycle.start_age, cycle.end_age),
                                                                    Locale::Ru => format!("{}~{} лет", cycle.start_age, cycle.end_age),
                                                                };
                                                                rsx! {
                                                                    tr { class: "{row_class} transition-colors",
                                                                        td { class: "px-4 py-3 font-mono text-slate-300 text-xs", "{age_range_str}" }
                                                                        td { class: "px-4 py-3 font-bold text-amber-300 font-serif", "{cycle.ganzi.hanja()} ({cycle.ganzi.hangul()})" }
                                                                        td { class: "px-4 py-3 text-slate-300", "{cycle.stem_god.hangul()}" }
                                                                        td { class: "px-4 py-3 text-slate-300", "{cycle.branch_god.hangul()}" }
                                                                        td { class: "px-4 py-3",
                                                                            if is_current {
                                                                                span { class: "px-2 py-0.5 rounded-full text-xs bg-amber-600/40 text-amber-200 border border-amber-500/40 font-semibold animate-pulse", "{current_lbl}" }
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
                            }

                            // ── 12개월 운세 카드 ───────────────────────────────
                            div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5",
                                h3 { class: "font-semibold text-slate-200 mb-4", "12개월 월운 (月運)" }
                                div { class: "grid grid-cols-3 md:grid-cols-4 xl:grid-cols-6 gap-3",
                                    {transit.monthly_lucks.iter().map(|ml| {
                                        let is_current = ml.month == transit.monthly_luck.month && ml.year == transit.monthly_luck.year;
                                        let card_class = if is_current {
                                            "bg-blue-900/30 border-blue-600/50"
                                        } else {
                                            "bg-slate-800/40 border-slate-700/40 hover:border-slate-600/60"
                                        };
                                        rsx! {
                                            div { class: "p-3 rounded-xl border {card_class} flex flex-col items-center gap-1 transition-colors",
                                                p { class: "text-xs text-slate-400", "{ml.month}월" }
                                                p { class: "text-xl font-serif text-amber-300 font-bold", "{ml.ganzi.hanja()}" }
                                                p { class: "text-xs text-slate-400", "{ml.ganzi.hangul()}" }
                                                div { class: "flex gap-1 flex-wrap justify-center",
                                                    span { class: "text-xs text-amber-400", "{ml.stem_god.hangul()}" }
                                                    span { class: "text-xs text-slate-500", "/" }
                                                    span { class: "text-xs text-blue-400", "{ml.branch_god.hangul()}" }
                                                }
                                                if is_current {
                                                    span { class: "text-xs text-blue-300 font-semibold", "◉" }
                                                }
                                            }
                                        }
                                    })}
                                }
                            }

                            // ── 시운 ──────────────────────────────────────────
                            div { class: "p-5 bg-slate-900 border border-slate-800 rounded-2xl",
                                h3 { class: "font-semibold text-slate-200 mb-3", "시운 (時運) — 현재 시간대" }
                                div { class: "flex items-center gap-4",
                                    div { class: "text-4xl font-serif text-emerald-300 font-bold", "{transit.hourly_luck.ganzi.hanja()}" }
                                    div {
                                        p { class: "text-lg text-slate-300 font-medium", "{transit.hourly_luck.ganzi.hangul()}" }
                                        p { class: "text-sm text-slate-400",
                                            "{transit.hourly_luck.stem_god.hangul()} / {transit.hourly_luck.branch_god.hangul()}"
                                        }
                                        if let Some(ts) = &transit.hourly_luck.twelve_stage {
                                            p { class: "text-xs text-slate-500", "12운성: {ts}" }
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

#[component]
fn LuckCard(title: String, value: String, sub: String, color: String, icon: &'static str) -> Element {
    rsx! {
        div { class: "p-4 rounded-2xl bg-gradient-to-b {color} border flex flex-col gap-2",
            div { class: "flex items-center gap-2",
                span { class: "text-xl", "{icon}" }
                span { class: "text-xs text-slate-400 font-medium", "{title}" }
            }
            p { class: "text-xl font-bold text-slate-100 font-serif", "{value}" }
            if !sub.is_empty() {
                p { class: "text-sm text-slate-400", "{sub}" }
            }
        }
    }
}
