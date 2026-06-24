use dioxus::prelude::*;
use crate::store::{AnalysisState, TaskStatus};
use crate::i18n::{t, TK, Locale, translate_planet};
use eon_service::dto::{SajuAnalysisInput, VedicAnalysisInput, AnalysisInput};
use eon_service::facade;
use eon_vedic::analysis::strength::StrengthEngine;
use eon_vedic::planets::VedicPlanet;
use eon_saju::analysis::strength::StrengthType;
use crate::components::shared::birth_form::BirthForm;

// 세력 분석은 사주+베딕 두 결과를 모두 필요로 합니다.
// 전역 상태에서 이미 계산된 saju, vedic 데이터를 활용하거나
// 없으면 이 탭에서 직접 호출합니다.

fn planet_bar_color(p: VedicPlanet) -> &'static str {
    match p {
        VedicPlanet::Sun => "bg-orange-500",
        VedicPlanet::Moon => "bg-slate-300",
        VedicPlanet::Mars => "bg-red-600",
        VedicPlanet::Mercury => "bg-emerald-400",
        VedicPlanet::Jupiter => "bg-yellow-400",
        VedicPlanet::Venus => "bg-pink-400",
        VedicPlanet::Saturn => "bg-indigo-600",
        VedicPlanet::Rahu => "bg-purple-500",
        VedicPlanet::Ketu => "bg-amber-700",
        VedicPlanet::Ascendant => "bg-white",
    }
}

#[component]
pub fn StrengthTab() -> Element {
    let mut state = use_context::<AnalysisState>();
    let locale = *state.locale.read();

    let run_analysis = move |_| {
        spawn(async move {
            state.saju.write().status = TaskStatus::Loading;
            state.vedic.write().status = TaskStatus::Loading;

            let form = state.form.read().clone();
            let base = AnalysisInput {
                year: form.year, month: form.month, day: form.day,
                hour: form.hour, minute: form.minute,
                is_lunar: form.is_lunar, is_leap_month: form.is_leap_month,
                lat: form.lat, lon: form.lon,
                timezone: "Asia/Seoul".to_string(),
            };

            // 병렬 계산 (사주 먼저, 베딕 이후)
            match facade::analyze_saju(SajuAnalysisInput::new(base.clone(), form.is_male, form.use_night_rat_hour, Some(false))) {
                Ok(res) => {
                    state.saju.write().data = Some(res);
                    state.saju.write().status = TaskStatus::Success;
                }
                Err(e) => {
                    state.saju.write().status = TaskStatus::Error(e.to_string());
                }
            }

            match facade::analyze_vedic(VedicAnalysisInput::new(base, Some(false), None)) {
                Ok(res) => {
                    state.vedic.write().data = Some(res);
                    state.vedic.write().status = TaskStatus::Success;
                }
                Err(e) => {
                    state.vedic.write().status = TaskStatus::Error(e.to_string());
                }
            }
        });
    };

    let is_loading = matches!(state.saju.read().status, TaskStatus::Loading)
        || matches!(state.vedic.read().status, TaskStatus::Loading);
    let has_saju = matches!(state.saju.read().status, TaskStatus::Success);
    let has_vedic = matches!(state.vedic.read().status, TaskStatus::Success);

    rsx! {
        div { class: "space-y-6 animate-in fade-in duration-700",
            BirthForm {}

            div { class: "flex justify-between items-center",
                h2 { class: "text-2xl font-bold bg-gradient-to-r from-emerald-200 to-green-400 bg-clip-text text-transparent",
                    "{t(locale, TK::SectionStrength)}"
                }
                button {
                    class: "px-5 py-2.5 bg-gradient-to-r from-emerald-700 to-green-700 hover:from-emerald-600 hover:to-green-600 rounded-xl font-semibold text-white shadow-lg transition-all duration-200 active:scale-95 disabled:opacity-50",
                    onclick: run_analysis,
                    disabled: is_loading,
                    if is_loading { "{t(locale, TK::StatusLoading)}" } else { "{t(locale, TK::BtnAnalyze)} 💪" }
                }
            }

            if !has_saju && !has_vedic {
                div { class: "flex flex-col items-center justify-center py-20 gap-3 text-slate-500",
                    span { class: "text-5xl", "⚖️" }
                    p { class: "text-lg font-medium", "{t(locale, TK::StatusIdleHint)}" }
                }
            }

            if is_loading {
                div { class: "flex flex-col items-center justify-center py-16 gap-3",
                    div { class: "w-12 h-12 rounded-full border-4 border-emerald-500/30 border-t-emerald-400 animate-spin" }
                    p { class: "text-emerald-400 font-medium animate-pulse", "{t(locale, TK::StatusLoadingStrength)}" }
                }
            }

            if has_saju {
                if let Some(saju) = &state.saju.read().data {
                    div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5",
                        // 헤더
                        div { class: "flex items-center justify-between mb-4",
                            h3 { class: "text-lg font-semibold text-emerald-300", "{t(locale, TK::SajuPowerWeighted)}" }
                            {
                                let st = saju.report.strength.strength_type;
                                let (badge_color, icon) = match st {
                                    StrengthType::Strong => ("bg-red-500/20 text-red-300 border-red-500/50", "🔥"),
                                    StrengthType::Weak => ("bg-blue-500/20 text-blue-300 border-blue-500/50", "💧"),
                                    StrengthType::Balanced => ("bg-emerald-500/20 text-emerald-300 border-emerald-500/50", "⚖️"),
                                };
                                let st_lbl = match locale {
                                    Locale::Ko => format!("{} ({})", st.hangul(), st.hanja()),
                                    Locale::En => match st {
                                        StrengthType::Strong => "Strong".to_string(),
                                        StrengthType::Weak => "Weak".to_string(),
                                        StrengthType::Balanced => "Balanced".to_string(),
                                    },
                                    Locale::Zh => match st {
                                        StrengthType::Strong => "身强".to_string(),
                                        StrengthType::Weak => "身弱".to_string(),
                                        StrengthType::Balanced => "中和".to_string(),
                                    },
                                    Locale::Ru => match st {
                                        StrengthType::Strong => "Сильный".to_string(),
                                        StrengthType::Weak => "Слабый".to_string(),
                                        StrengthType::Balanced => "Сбалансирован".to_string(),
                                    },
                                };
                                rsx! {
                                    span { class: "flex items-center gap-1 px-3 py-1 rounded-full border text-sm font-bold {badge_color}",
                                        "{icon} {st_lbl}"
                                    }
                                }
                            }
                        }

                        div { class: "space-y-3",
                            {
                                let se = &saju.report.strength.deuk_se;
                                let total = (se.bijie_count + se.yinxing_count + se.shishang_count + se.caisheng_count + se.guanxing_count).max(1) as f32;
                                let dm_el = saju.report.strength.day_master.element();

                                let bars = vec![
                                    ("비겁(比劫)", dm_el.hangul(), dm_el.hanja(), se.bijie_count, "bg-violet-500"),
                                    ("인성(印星)", dm_el.generated_by().hangul(), dm_el.generated_by().hanja(), se.yinxing_count, "bg-blue-500"),
                                    ("식상(食傷)", dm_el.generates().hangul(), dm_el.generates().hanja(), se.shishang_count, "bg-emerald-500"),
                                    ("재성(財星)", dm_el.controls().hangul(), dm_el.controls().hanja(), se.caisheng_count, "bg-amber-500"),
                                    ("관성(官星)", dm_el.controlled_by().hangul(), dm_el.controlled_by().hanja(), se.guanxing_count, "bg-red-500"),
                                ];

                                rsx! {
                                    {bars.iter().map(|(ten_god, el_name, el_hanja, count, color)| {
                                        let pct = (*count as f32 / total * 100.0) as u32;
                                        let ten_god_lbl = match locale {
                                            Locale::Ko => *ten_god,
                                            Locale::En => match *ten_god {
                                                "비겁(比劫)" => "Companion",
                                                "인성(印星)" => "Resource",
                                                "식상(食傷)" => "Output",
                                                "재성(財星)" => "Wealth",
                                                "관성(官星)" => "Influence",
                                                _ => *ten_god,
                                            },
                                            Locale::Zh => match *ten_god {
                                                "비겁(比劫)" => "比劫",
                                                "인성(印星)" => "印星",
                                                "식상(食傷)" => "食伤",
                                                "재성(財星)" => "财星",
                                                "관성(官星)" => "官星",
                                                _ => *ten_god,
                                            },
                                            Locale::Ru => match *ten_god {
                                                "비겁(比劫)" => "Братство",
                                                "인성(印星)" => "Ресурсы",
                                                "식상(食傷)" => "Самовыражение",
                                                "재성(財星)" => "Богатство",
                                                "관성(官星)" => "Власть",
                                                _ => *ten_god,
                                            },
                                        };
                                        let count_str = match locale {
                                            Locale::Ko => format!("{}개", count),
                                            Locale::Zh => format!("{}个", count),
                                            _ => format!("{}", count),
                                        };
                                        rsx! {
                                            div { class: "flex items-center gap-3",
                                                div { class: "w-28 text-right shrink-0",
                                                    span { class: "text-sm font-medium text-slate-300", "{ten_god_lbl}" }
                                                    span { class: "text-xs text-slate-500 ml-1", "{el_name}({el_hanja})" }
                                                }
                                                div { class: "flex-1 h-4 bg-slate-800 rounded-full overflow-hidden",
                                                    div {
                                                        class: "h-full {color} rounded-full transition-all duration-1000",
                                                        style: "width: {pct}%"
                                                    }
                                                }
                                                div { class: "w-16 text-xs font-mono text-slate-400 text-right shrink-0",
                                                    "{count_str} ({pct}%)"
                                                }
                                            }
                                        }
                                    })}
                                }
                            }
                        }

                        // 득령/득지/득시/득세 상세
                        div { class: "mt-4 pt-4 border-t border-slate-800 grid grid-cols-2 md:grid-cols-4 gap-3 text-sm",
                            {
                                let deuk_ji_desc = match locale {
                                    Locale::Ko => format!("통근 {}개 | 강한 운성 {}개", saju.report.strength.deuk_ji.root_count, saju.report.strength.deuk_ji.strong_stage_count),
                                    Locale::En => format!("Roots: {} | Strong: {}", saju.report.strength.deuk_ji.root_count, saju.report.strength.deuk_ji.strong_stage_count),
                                    Locale::Zh => format!("通根 {}个 | 强星 {}个", saju.report.strength.deuk_ji.root_count, saju.report.strength.deuk_ji.strong_stage_count),
                                    Locale::Ru => format!("Корни: {} | Сильные: {}", saju.report.strength.deuk_ji.root_count, saju.report.strength.deuk_ji.strong_stage_count),
                                };
                                let deuk_se_desc = match locale {
                                    Locale::Ko => format!("지지 세력 {:.1}%", saju.report.strength.deuk_se.support_ratio),
                                    Locale::En => format!("Branch Support: {:.1}%", saju.report.strength.deuk_se.support_ratio),
                                    Locale::Zh => format!("地支势力 {:.1}%", saju.report.strength.deuk_se.support_ratio),
                                    Locale::Ru => format!("Поддержка ветвей: {:.1}%", saju.report.strength.deuk_se.support_ratio),
                                };
                                rsx! {
                                    DeukDetail { label: t(locale, TK::SajuDeukRyeong), acquired: saju.report.strength.deuk_ryeong.acquired, desc: saju.report.strength.deuk_ryeong.relation().to_string() }
                                    DeukDetail { label: t(locale, TK::SajuDeukJi), acquired: saju.report.strength.deuk_ji.acquired, desc: deuk_ji_desc }
                                    DeukDetail { label: t(locale, TK::SajuDeukSi), acquired: saju.report.strength.deuk_si.acquired, desc: saju.report.strength.deuk_si.relation().to_string() }
                                    DeukDetail { label: t(locale, TK::SajuDeukSe), acquired: saju.report.strength.deuk_se.acquired, desc: deuk_se_desc }
                                }
                            }
                        }
                    }
                }
            }

            // 베딕 샤드발라
            if has_vedic {
                if let Some(vedic) = &state.vedic.read().data {
                    div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-6 space-y-6 shadow-xl",
                        div { class: "flex items-center justify-between border-b border-slate-800 pb-4",
                            h3 { class: "text-lg font-bold bg-gradient-to-r from-blue-200 to-indigo-400 bg-clip-text text-transparent",
                                "{t(locale, TK::SectionStrength)} (Shadbala)"
                            }
                            span { class: "text-xs text-slate-500 font-mono", "1 Rupa = 60 Shashtiamsa" }
                        }
                        
                        div { class: "grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6",
                            {
                                // 7대 행성만 필터링하여 Shadbala 계산
                                let target_planets = [
                                    VedicPlanet::Sun, VedicPlanet::Moon, VedicPlanet::Mars,
                                    VedicPlanet::Mercury, VedicPlanet::Jupiter, VedicPlanet::Venus,
                                    VedicPlanet::Saturn
                                ];
                                
                                let strengths: Vec<_> = vedic.chart.planets.iter()
                                    .filter(|p| target_planets.contains(&p.planet))
                                    .map(|p| (p.planet, StrengthEngine::calculate(p, &vedic.chart)))
                                    .collect();

                                rsx! {
                                    {strengths.iter().map(|(planet, s)| {
                                        let rupa_req = match planet {
                                            VedicPlanet::Sun => 6.5,
                                            VedicPlanet::Moon => 6.0,
                                            VedicPlanet::Mars => 5.0,
                                            VedicPlanet::Mercury => 7.0,
                                            VedicPlanet::Jupiter => 6.5,
                                            VedicPlanet::Venus => 5.5,
                                            VedicPlanet::Saturn => 5.0,
                                            _ => 0.0,
                                        };
                                        let req_score = rupa_req * 60.0;
                                        let sat_ratio = (s.total_score / req_score * 100.0) as u32;
                                        let is_satisfied = s.total_score >= req_score;
                                        
                                        let badge_class = if is_satisfied {
                                            "bg-emerald-950/40 text-emerald-400 border-emerald-900/50"
                                        } else {
                                            "bg-rose-950/40 text-rose-400 border-rose-900/50"
                                        };
                                        
                                        let badge_lbl = if is_satisfied {
                                            t(locale, TK::ShadbalaSatisfied)
                                        } else {
                                            t(locale, TK::ShadbalaUnsatisfied)
                                        };
                                        
                                        let bar_color = planet_bar_color(*planet);
                                        
                                        let factors = vec![
                                            (t(locale, TK::ShadbalaSthanaBala), s.sthana_bala, "bg-blue-500"),
                                            (t(locale, TK::ShadbalaDigBala), s.dig_bala, "bg-green-500"),
                                            (t(locale, TK::ShadbalaKalaBala), s.kala_bala, "bg-purple-500"),
                                            (t(locale, TK::ShadbalaChestaBala), s.chesta_bala, "bg-cyan-500"),
                                            (t(locale, TK::ShadbalaNaisargikaBala), s.naisargika_bala, "bg-amber-500"),
                                            (t(locale, TK::ShadbalaDrikBala), s.drik_bala, "bg-red-500"),
                                        ];

                                        rsx! {
                                            div { class: "bg-slate-950/40 border border-slate-800/80 rounded-xl p-4 space-y-4 hover:border-slate-700/60 transition-all duration-300 shadow-md",
                                                // 카드 헤더
                                                div { class: "flex justify-between items-center",
                                                    div { class: "flex items-center gap-2",
                                                        span { class: "w-2.5 h-2.5 rounded-full {bar_color}" }
                                                        span { class: "text-sm font-bold text-slate-200", "{translate_planet(locale, *planet)}" }
                                                    }
                                                    span { class: "px-2 py-0.5 rounded text-[10px] font-bold border {badge_class}",
                                                        "{badge_lbl} ({sat_ratio}%)"
                                                    }
                                                }
                                                
                                                // 총점 요약
                                                div { class: "bg-slate-900/50 border border-slate-900 rounded-lg p-2.5 flex justify-between items-center text-xs font-mono",
                                                    div { class: "space-y-0.5",
                                                        p { class: "text-[9px] text-slate-500 uppercase tracking-wider", "{t(locale, TK::ShadbalaActual)}" }
                                                        p { class: "text-slate-200 font-bold", "{s.total_score:.1} / {req_score:.0} pt" }
                                                    }
                                                    div { class: "text-right space-y-0.5",
                                                        p { class: "text-[9px] text-slate-500 uppercase tracking-wider", "{t(locale, TK::ShadbalaRequired)}" }
                                                        p { class: "text-indigo-400 font-bold", "{s.total_score / 60.0:.2} / {rupa_req:.1} R" }
                                                    }
                                                }
                                                
                                                // 6대 강도 요인 리스트
                                                div { class: "space-y-2 pt-1",
                                                    {factors.iter().map(|(label, val, f_color)| {
                                                        let f_pct = (*val / 60.0 * 100.0).clamp(0.0, 100.0) as u32;
                                                        rsx! {
                                                            div { class: "space-y-1",
                                                                div { class: "flex justify-between text-[10px] text-slate-400 font-medium",
                                                                    span { "{label}" }
                                                                    span { class: "font-mono font-bold text-slate-350", "{val:.1}" }
                                                                }
                                                                div { class: "h-1.5 bg-slate-900 rounded-full overflow-hidden",
                                                                    div {
                                                                        class: "h-full {f_color} rounded-full transition-all duration-500",
                                                                        style: "width: {f_pct}%"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    })}
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
}

#[component]
fn DeukDetail(label: &'static str, acquired: bool, desc: String) -> Element {
    let (bg, icon_color) = if acquired {
        ("bg-emerald-900/20 border-emerald-700/40", "text-emerald-400")
    } else {
        ("bg-slate-800/40 border-slate-700/40", "text-slate-500")
    };
    rsx! {
        div { class: "p-3 rounded-xl border {bg}",
            div { class: "flex items-center gap-1.5 mb-1",
                span { class: "text-xs font-mono {icon_color}", if acquired { "○" } else { "✗" } }
                span { class: "text-xs font-semibold text-slate-300", "{label}" }
            }
            p { class: "text-xs text-slate-500 leading-snug", "{desc}" }
        }
    }
}
