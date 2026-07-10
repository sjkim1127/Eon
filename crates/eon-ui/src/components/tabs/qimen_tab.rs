use crate::i18n::{t, TK};
use crate::store::{AnalysisState, TaskStatus};
use dioxus::prelude::*;
use eon_qimen::core::{Deity, Door, Palace, Star};
use eon_saju::core::stem::HeavenlyStem;

fn tk_palace(p: Palace) -> TK {
    match p {
        Palace::Kan1 => TK::QimenPalaceKan1,
        Palace::Kun2 => TK::QimenPalaceKun2,
        Palace::Zhen3 => TK::QimenPalaceZhen3,
        Palace::Xun4 => TK::QimenPalaceXun4,
        Palace::Center5 => TK::QimenPalaceCenter5,
        Palace::Qian6 => TK::QimenPalaceQian6,
        Palace::Dui7 => TK::QimenPalaceDui7,
        Palace::Gen8 => TK::QimenPalaceGen8,
        Palace::Li9 => TK::QimenPalaceLi9,
    }
}

fn tk_door(d: Door) -> TK {
    match d {
        Door::Xiu => TK::QimenDoorXiu,
        Door::Sheng => TK::QimenDoorSheng,
        Door::Shang => TK::QimenDoorShang,
        Door::Du => TK::QimenDoorDu,
        Door::Jing => TK::QimenDoorJing,
        Door::Si => TK::QimenDoorSi,
        Door::Jing2 => TK::QimenDoorJing2,
        Door::Kai => TK::QimenDoorKai,
    }
}

fn tk_star(s: Star) -> TK {
    match s {
        Star::Peng => TK::QimenStarPeng,
        Star::Ren => TK::QimenStarRen,
        Star::Chong => TK::QimenStarChong,
        Star::Fu => TK::QimenStarFu,
        Star::Ying => TK::QimenStarYing,
        Star::Rui => TK::QimenStarRui,
        Star::Zhu => TK::QimenStarZhu,
        Star::Xin => TK::QimenStarXin,
        Star::Qin => TK::QimenStarQin,
    }
}

fn tk_deity(d: Deity) -> TK {
    match d {
        Deity::ZhiFu => TK::QimenDeityZhiFu,
        Deity::TengShe => TK::QimenDeityTengShe,
        Deity::TaiYin => TK::QimenDeityTaiYin,
        Deity::LiuHe => TK::QimenDeityLiuHe,
        Deity::BaiHu => TK::QimenDeityBaiHu,
        Deity::XuanWu => TK::QimenDeityXuanWu,
        Deity::JiuDi => TK::QimenDeityJiuDi,
        Deity::JiuTian => TK::QimenDeityJiuTian,
    }
}

fn stem_str(s: HeavenlyStem) -> &'static str {
    match s {
        HeavenlyStem::Jia => "甲",
        HeavenlyStem::Yi => "乙",
        HeavenlyStem::Bing => "丙",
        HeavenlyStem::Ding => "丁",
        HeavenlyStem::Wu => "戊",
        HeavenlyStem::Ji => "己",
        HeavenlyStem::Geng => "庚",
        HeavenlyStem::Xin => "辛",
        HeavenlyStem::Ren => "壬",
        HeavenlyStem::Gui => "癸",
    }
}

#[component]
pub fn QimenTab() -> Element {
    let state = use_context::<AnalysisState>();
    let locale = *state.locale.read();
    let qimen = state.qimen.read().clone();
    let mut copied_feedback = use_signal(|| false);

    // 9궁 표시 순서 (UI상 낙서 구궁 배열: 4,9,2 / 3,5,7 / 8,1,6 궁)
    // palace 배열은 0부터 8까지 1~9궁을 나타내므로, 인덱스로 변환하면 3,8,1 / 2,4,6 / 7,0,5
    let ui_order = [3, 8, 1, 2, 4, 6, 7, 0, 5];

    rsx! {
        div { class: "w-full max-w-7xl mx-auto space-y-6 md:space-y-8 animate-fade-in",
            match qimen.status {
                TaskStatus::Idle | TaskStatus::Loading => rsx! {
                    div { class: "flex justify-center items-center h-64",
                        div { class: "text-slate-400 font-medium animate-pulse",
                            "{t(locale, TK::QimenAnalyzing)}"
                        }
                    }
                },
                TaskStatus::Error(e) => rsx! {
                    div { class: "bg-red-950/30 border border-red-500/50 rounded-2xl p-6 text-red-200",
                        div { class: "font-bold mb-2 flex items-center gap-2",
                            "{t(locale, TK::QimenError)}"
                        }
                        div { class: "text-sm opacity-90", "{e}" }
                    }
                },
                TaskStatus::Success => rsx! {
                    if let Some(data) = qimen.data {
                        div { class: "bg-slate-900/50 backdrop-blur-md rounded-3xl border border-slate-700/50 p-6 md:p-8 shadow-2xl",
                            div { class: "flex justify-between items-center mb-6",
                                h2 { class: "text-2xl font-bold text-slate-100 flex items-center gap-3",
                                    span { class: "text-3xl", "🧭" }
                                    "{t(locale, TK::QimenTitle)}"
                                }
                                div { class: "flex items-center gap-2",
                                    button {
                                        class: if *copied_feedback.read() {
                                            "px-4 py-2 bg-emerald-650 hover:bg-emerald-600 text-white border border-emerald-500/50 rounded-lg transition-all cursor-pointer flex items-center justify-center active:scale-95 gap-1"
                                        } else {
                                            "px-4 py-2 bg-slate-800 hover:bg-slate-700 border border-slate-700/50 rounded-lg text-slate-300 transition-all cursor-pointer flex items-center justify-center active:scale-95 gap-1"
                                        },
                                        title: "Copy Report",
                                        onclick: move |_| {
                                            if let Some(ref data) = state.qimen.read().data.as_ref() {
                                                let txt = crate::components::shared::export_markdown::export_qimen_to_markdown(data, &state.form.read(), locale);
                                                crate::components::shared::export_markdown::copy_to_clipboard(&txt);
                                                copied_feedback.set(true);
                                                spawn(async move {
                                                    gloo_timers::future::TimeoutFuture::new(2000).await;
                                                    copied_feedback.set(false);
                                                });
                                            }
                                        },
                                        if *copied_feedback.read() {
                                            span { class: "text-xs font-bold", "✓ Copied" }
                                        } else {
                                            span { class: "text-xs font-medium", "📋 Copy" }
                                        }
                                    }
                                    div { class: "text-sm text-slate-300 font-medium px-4 py-2 bg-slate-800/80 rounded-lg",
                                        if data.report.pan.is_yin_ju {
                                            "{t(locale, TK::QimenYinJu)} "
                                        } else {
                                            "{t(locale, TK::QimenYangJu)} "
                                        }
                                        "{data.report.pan.ju_number}{t(locale, TK::QimenJu)}"
                                    }
                                }
                            }

                            // 3x3 Grid
                            div { class: "grid grid-cols-3 gap-2 md:gap-4 max-w-3xl mx-auto aspect-square",
                                for idx in ui_order {
                                    if let Some(p) = data.report.pan.palaces.get(idx) {
                                        div { class: "bg-slate-800/40 border border-slate-600/30 rounded-xl p-2 md:p-4 flex flex-col justify-between hover:bg-slate-700/40 transition-colors relative",
                                            // Top Row (Deity, Star)
                                            div { class: "flex justify-between items-start text-xs md:text-sm",
                                                div { class: "text-red-400 font-medium",
                                                    if let Some(d) = p.deity { "{t(locale, tk_deity(d))}" }
                                                }
                                                div { class: "text-blue-400 font-medium",
                                                    if let Some(s) = p.star { "{t(locale, tk_star(s))}" }
                                                }
                                            }

                                            // Middle Row (Heaven Stem, Earth Stem)
                                            div { class: "flex flex-col items-center justify-center py-2 space-y-1",
                                                div { class: "text-xl md:text-3xl font-bold text-amber-100",
                                                    if let Some(h) = p.heaven_stem { "{stem_str(h)}" }
                                                }
                                                div { class: "text-lg md:text-2xl font-bold text-slate-400",
                                                    if let Some(e) = p.earth_stem { "{stem_str(e)}" }
                                                }
                                            }

                                            // Bottom Row (Door, Palace)
                                            div { class: "flex justify-between items-end text-xs md:text-sm",
                                                div { class: "text-emerald-400 font-bold",
                                                    if let Some(d) = p.door { "{t(locale, tk_door(d))}" }
                                                }
                                                div { class: "text-slate-500 text-[10px] md:text-xs text-right leading-tight",
                                                    "{t(locale, tk_palace(p.palace))}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            div { class: "mt-8 flex justify-center items-center gap-6 text-sm text-slate-400",
                                if let Some(chief) = data.report.pan.value_chief_star {
                                    div { class: "flex items-center gap-2",
                                        span { class: "px-2 py-1 bg-blue-900/30 text-blue-300 rounded", "{t(locale, TK::QimenValueChief)}" }
                                        span { class: "font-bold text-slate-200", "{t(locale, tk_star(chief))}" }
                                    }
                                }
                                if let Some(envoy) = data.report.pan.value_envoy_door {
                                    div { class: "flex items-center gap-2",
                                        span { class: "px-2 py-1 bg-emerald-900/30 text-emerald-300 rounded", "{t(locale, TK::QimenValueEnvoy)}" }
                                        span { class: "font-bold text-slate-200", "{t(locale, tk_door(envoy))}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
