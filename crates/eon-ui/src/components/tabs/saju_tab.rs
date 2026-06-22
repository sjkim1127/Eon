use dioxus::prelude::*;
use crate::store::{AnalysisState, TaskStatus};
use crate::i18n::{t, TK, Locale, format_strength_summary, format_age, format_age_shift, format_age_from};
use eon_service::dto::{SajuAnalysisInput, AnalysisInput};
use eon_service::facade;
use eon_saju::analysis::strength::StrengthType;
use eon_saju::analysis::supplementary_pillars::InterpretationLevel;
use crate::components::shared::birth_form::BirthForm;
use eon_saju::core::branch::EarthlyBranch;
use eon_saju::core::stem::HeavenlyStem;
use eon_saju::core::ten_gods::TenGod;

#[component]
pub fn SajuTab() -> Element {
    let mut state = use_context::<AnalysisState>();
    let locale = *state.locale.read();

    let run_analysis = move |_| {
        spawn(async move {
            state.saju.write().status = TaskStatus::Loading;
            let form = state.form.read().clone();
            let input = SajuAnalysisInput::new(
                AnalysisInput {
                    year: form.year, month: form.month, day: form.day,
                    hour: form.hour, minute: form.minute,
                    is_lunar: form.is_lunar, is_leap_month: form.is_leap_month,
                    lat: form.lat, lon: form.lon,
                    timezone: "Asia/Seoul".to_string(),
                },
                form.is_male, false, Some(false),
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
                    "{t(locale, TK::SectionSajuChart)}"
                }
                button {
                    class: "px-5 py-2.5 bg-gradient-to-r from-amber-600 to-orange-600 hover:from-amber-500 hover:to-orange-500 rounded-xl font-semibold text-white shadow-lg shadow-amber-900/30 transition-all duration-200 active:scale-95",
                    onclick: run_analysis,
                    "{t(locale, TK::SajuAnalyzeBtn)}"
                }
            }

            {
                match &state.saju.read().status {
                TaskStatus::Idle => rsx! {
                    div { class: "flex flex-col items-center justify-center py-20 gap-3 text-slate-500",
                        span { class: "text-5xl", "🌌" }
                        p { class: "text-lg font-medium", "{t(locale, TK::StatusIdleHint)}" }
                    }
                },
                TaskStatus::Loading => rsx! {
                    div { class: "flex flex-col items-center justify-center py-20 gap-3",
                        div { class: "w-12 h-12 rounded-full border-4 border-amber-500/30 border-t-amber-400 animate-spin" }
                        p { class: "text-amber-400 font-medium animate-pulse", "{t(locale, TK::StatusLoadingSaju)}" }
                    }
                },
                TaskStatus::Error(e) => rsx! {
                    div { class: "p-4 rounded-xl bg-red-900/20 border border-red-800/50 text-red-400", "{t(locale, TK::StatusError)}: {e}" }
                },
                TaskStatus::Success => {
                    if let Some(data) = &state.saju.read().data {
                        let twelve_stages = data.report.pillars.twelve_stages();
                        let shinsals_for = |pos: eon_saju::analysis::spirit_markers::PillarPosition| {
                            data.report.spirit_markers.mapped_markers.iter()
                                .filter(|m| m.position == pos)
                                .map(|m| m.marker.hangul().to_string())
                                .collect::<Vec<String>>()
                        };
                        let unpacker_info = data.entropy.unpacker_element.map(|unpacker| {
                            (element_style(unpacker.hangul()).0, format!("{}({})", unpacker.hangul(), unpacker.hanja()))
                        });
                        let bottleneck_info = data.qi_topology.bottleneck.map(|btn| {
                            (element_style(btn.hangul()).0, format!("{}({})", btn.hangul(), btn.hanja()))
                        });
                        let complexity_info = data.complexity.as_ref().map(|comp| {
                            let comp_label = format!("M = {} ({})", comp.cyclomatic_complexity, comp.stability_grade);
                            let entropy_str = format!("{:.2}", comp.entropy);
                            let decision_ages = comp.decision_nodes.iter().map(|&a| format_age(locale, a as i32)).collect::<Vec<_>>().join(", ");
                            (comp_label, entropy_str, decision_ages)
                        });

                        let major_luck_info = data.report.major_luck.as_ref().map(|ml| {
                            let dir_str = match locale {
                                Locale::Ko => if ml.direction == eon_saju::LuckDirection::Forward { "순행" } else { "역행" },
                                Locale::En => if ml.direction == eon_saju::LuckDirection::Forward { "Direct" } else { "Reverse" },
                                Locale::Zh => if ml.direction == eon_saju::LuckDirection::Forward { "顺行" } else { "逆行" },
                                Locale::Ru => if ml.direction == eon_saju::LuckDirection::Forward { "Прямо" } else { "Обратно" },
                            };
                            (ml, dir_str)
                        });

                        let crashes_lbl = t(locale, TK::SajuFuzzerCrashes).replace("{}", &data.crash_count.to_string());

                        let day_master = data.report.pillars.day.stem;
                        let stems = vec![
                            data.report.pillars.hour.stem,
                            data.report.pillars.day.stem,
                            data.report.pillars.month.stem,
                            data.report.pillars.year.stem,
                        ];

                        let make_jijanggans = |branch: eon_saju::core::branch::EarthlyBranch, is_month: bool| -> Vec<JijangganDisplayItem> {
                            get_jijanggan_items(branch)
                                .into_iter()
                                .map(|item| {
                                    let is_projected = stems.contains(&item.stem);
                                    let is_main = is_month && is_projected;
                                    let ten_god = TenGod::from_stems(day_master, item.stem);
                                    JijangganDisplayItem {
                                        stem: item.stem,
                                        ratio: item.ratio,
                                        type_key: item.type_key,
                                        ten_god,
                                        is_projected,
                                        is_main,
                                    }
                                })
                                .collect()
                        };

                        let hour_jijanggans = make_jijanggans(data.report.pillars.hour.branch, false);
                        let day_jijanggans = make_jijanggans(data.report.pillars.day.branch, false);
                        let month_jijanggans = make_jijanggans(data.report.pillars.month.branch, true);
                        let year_jijanggans = make_jijanggans(data.report.pillars.year.branch, false);

                        let mut projected_instances = Vec::new();

                        let branch_positions = vec![
                            (data.report.pillars.hour.branch, false, "시지", TK::SajuHourPillar),
                            (data.report.pillars.day.branch, false, "일지", TK::SajuDayPillar),
                            (data.report.pillars.month.branch, true, "월지", TK::SajuMonthPillar),
                            (data.report.pillars.year.branch, false, "연지", TK::SajuYearPillar),
                        ];

                        let target_stems = vec![
                            (data.report.pillars.hour.stem, "시간", TK::SajuHourPillar),
                            (data.report.pillars.day.stem, "일간", TK::SajuDayPillar),
                            (data.report.pillars.month.stem, "월간", TK::SajuMonthPillar),
                            (data.report.pillars.year.stem, "연간", TK::SajuYearPillar),
                        ];

                        for &(branch, is_month, b_pos_lbl, b_pos_tk) in &branch_positions {
                            for j_item in get_jijanggan_items(branch) {
                                for &(t_stem, t_pos_lbl, t_pos_tk) in &target_stems {
                                    if j_item.stem == t_stem {
                                        let ten_god = TenGod::from_stems(day_master, j_item.stem);
                                        projected_instances.push((
                                            b_pos_lbl,
                                            b_pos_tk,
                                            branch,
                                            j_item.stem,
                                            j_item.ratio,
                                            j_item.type_key,
                                            ten_god,
                                            t_pos_lbl,
                                            t_pos_tk,
                                            is_month,
                                        ));
                                    }
                                }
                            }
                        }

                        let no_proj_msg = match locale {
                            Locale::Ko => "천간으로 투출된 지장간이 없습니다.",
                            Locale::En => "No hidden stems are projected to the heavenly stems.",
                            Locale::Zh => "无地支藏干透出至天干。",
                            Locale::Ru => "Нет скрытых небесных стволов, проецирующихся на небесные стволы.",
                        };

                        rsx! {
                            // ── 1. 사주 원국 (천간/지지/십성/12운성/신살) ─────────
                            div { class: "grid grid-cols-4 gap-3.5",
                                PillarCard {
                                    title: t(locale, TK::SajuHourPillar),
                                    stem_god: data.report.ten_gods.hour_stem.hangul().to_string(),
                                    stem_hanja: data.report.pillars.hour.stem.hanja().to_string(),
                                    stem_hangul: data.report.pillars.hour.stem.hangul().to_string(),
                                    stem_element: data.report.pillars.hour.stem.element().hangul().to_string(),
                                    branch_god: data.report.ten_gods.hour_branch.hangul().to_string(),
                                    branch_hanja: data.report.pillars.hour.branch.hanja().to_string(),
                                    branch_hangul: data.report.pillars.hour.branch.hangul().to_string(),
                                    branch_element: data.report.pillars.hour.branch.element().hangul().to_string(),
                                    twelve_stage: twelve_stages.hour_stage.hangul().to_string(),
                                    shinsals: shinsals_for(eon_saju::analysis::spirit_markers::PillarPosition::Hour),
                                    jijanggans: hour_jijanggans
                                }
                                PillarCard {
                                    title: t(locale, TK::SajuDayPillar),
                                    stem_god: t(locale, TK::SajuDayMaster).to_string(),
                                    stem_hanja: data.report.pillars.day.stem.hanja().to_string(),
                                    stem_hangul: data.report.pillars.day.stem.hangul().to_string(),
                                    stem_element: data.report.pillars.day.stem.element().hangul().to_string(),
                                    branch_god: data.report.ten_gods.day_branch.hangul().to_string(),
                                    branch_hanja: data.report.pillars.day.branch.hanja().to_string(),
                                    branch_hangul: data.report.pillars.day.branch.hangul().to_string(),
                                    branch_element: data.report.pillars.day.branch.element().hangul().to_string(),
                                    twelve_stage: twelve_stages.day_stage.hangul().to_string(),
                                    shinsals: shinsals_for(eon_saju::analysis::spirit_markers::PillarPosition::Day),
                                    jijanggans: day_jijanggans
                                }
                                PillarCard {
                                    title: t(locale, TK::SajuMonthPillar),
                                    stem_god: data.report.ten_gods.month_stem.hangul().to_string(),
                                    stem_hanja: data.report.pillars.month.stem.hanja().to_string(),
                                    stem_hangul: data.report.pillars.month.stem.hangul().to_string(),
                                    stem_element: data.report.pillars.month.stem.element().hangul().to_string(),
                                    branch_god: data.report.ten_gods.month_branch.hangul().to_string(),
                                    branch_hanja: data.report.pillars.month.branch.hanja().to_string(),
                                    branch_hangul: data.report.pillars.month.branch.hangul().to_string(),
                                    branch_element: data.report.pillars.month.branch.element().hangul().to_string(),
                                    twelve_stage: twelve_stages.month_stage.hangul().to_string(),
                                    shinsals: shinsals_for(eon_saju::analysis::spirit_markers::PillarPosition::Month),
                                    jijanggans: month_jijanggans
                                }
                                PillarCard {
                                    title: t(locale, TK::SajuYearPillar),
                                    stem_god: data.report.ten_gods.year_stem.hangul().to_string(),
                                    stem_hanja: data.report.pillars.year.stem.hanja().to_string(),
                                    stem_hangul: data.report.pillars.year.stem.hangul().to_string(),
                                    stem_element: data.report.pillars.year.stem.element().hangul().to_string(),
                                    branch_god: data.report.ten_gods.year_branch.hangul().to_string(),
                                    branch_hanja: data.report.pillars.year.branch.hanja().to_string(),
                                    branch_hangul: data.report.pillars.year.branch.hangul().to_string(),
                                    branch_element: data.report.pillars.year.branch.element().hangul().to_string(),
                                    twelve_stage: twelve_stages.year_stage.hangul().to_string(),
                                    shinsals: shinsals_for(eon_saju::analysis::spirit_markers::PillarPosition::Year),
                                    jijanggans: year_jijanggans
                                }
                            }

                            // ── 1.2 지장간 투출 분석 (Hidden Stems Projection) ─────
                            div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5 space-y-4 shadow-xl",
                                div { class: "flex items-center justify-between border-b border-slate-800/60 pb-3 flex-wrap gap-2",
                                    div { class: "space-y-0.5",
                                        h3 { class: "text-sm font-semibold text-slate-200 uppercase tracking-widest", "{t(locale, TK::SajuProjectionTitle)}" }
                                        p { class: "text-xs text-slate-500", "{t(locale, TK::SajuProjectionDesc)}" }
                                    }
                                }
                                if projected_instances.is_empty() {
                                    p { class: "text-slate-500 text-xs py-4 text-center", "{no_proj_msg}" }
                                } else {
                                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-3.5",
                                        {projected_instances.iter().map(|&(b_pos_lbl, _b_pos_tk, _branch, stem, ratio, _type_key, ten_god, t_pos_lbl, _t_pos_tk, is_main)| {
                                            let (el_color, _el_bg, el_icon) = element_card_style(stem.element().hangul());
                                            let proj_type_lbl = if is_main { t(locale, TK::SajuProjLevelMain) } else { t(locale, TK::SajuProjLevelSub) };
                                            let badge_cls = if is_main {
                                                "bg-amber-500/20 text-amber-300 border-amber-500/30"
                                            } else {
                                                "bg-indigo-500/20 text-indigo-300 border-indigo-500/30"
                                            };
                                            let (pos_branch_name, pos_stem_name) = match locale {
                                                Locale::Ko => (
                                                    match b_pos_lbl {
                                                        "시지" => "시지(時支)",
                                                        "일지" => "일지(日支)",
                                                        "월지" => "월지(月支)",
                                                        "연지" => "연지(年支)",
                                                        _ => b_pos_lbl,
                                                    },
                                                    match t_pos_lbl {
                                                        "시간" => "시간(時干)",
                                                        "일간" => "일간(日干)",
                                                        "월간" => "월간(月干)",
                                                        "연간" => "연간(年干)",
                                                        _ => t_pos_lbl,
                                                    }
                                                ),
                                                Locale::En => (
                                                    match b_pos_lbl {
                                                        "시지" => "Hour Branch",
                                                        "일지" => "Day Branch",
                                                        "월지" => "Month Branch",
                                                        "연지" => "Year Branch",
                                                        _ => b_pos_lbl,
                                                    },
                                                    match t_pos_lbl {
                                                        "시간" => "Hour Stem",
                                                        "일간" => "Day Stem",
                                                        "월간" => "Month Stem",
                                                        "연간" => "Year Stem",
                                                        _ => t_pos_lbl,
                                                    }
                                                ),
                                                Locale::Zh => (
                                                    match b_pos_lbl {
                                                        "시지" => "时支",
                                                        "일지" => "日支",
                                                        "월지" => "月支",
                                                        "연지" => "年支",
                                                        _ => b_pos_lbl,
                                                    },
                                                    match t_pos_lbl {
                                                        "시간" => "时干",
                                                        "일간" => "日干",
                                                        "월간" => "月干",
                                                        "연간" => "年干",
                                                        _ => t_pos_lbl,
                                                    }
                                                ),
                                                Locale::Ru => (
                                                    match b_pos_lbl {
                                                        "시지" => "Земная ветвь часа",
                                                        "일지" => "Земная ветвь дня",
                                                        "월지" => "Земная ветвь месяца",
                                                        "연지" => "Земная ветвь года",
                                                        _ => b_pos_lbl,
                                                    },
                                                    match t_pos_lbl {
                                                        "시간" => "Небесный ствол часа",
                                                        "일간" => "Небесный ствол дня",
                                                        "월간" => "Небесный ствол месяца",
                                                        "연간" => "Небесный ствол года",
                                                        _ => t_pos_lbl,
                                                    }
                                                ),
                                            };
                                            let stage_desc = match locale {
                                                Locale::Ko => format!(
                                                    "{}의 지장간 {}({}) [{}]이 {}로 투출하여 외부로 강하게 발현됩니다. (가중치: {}%)",
                                                    pos_branch_name, stem.hanja(), stem.hangul(), ten_god.hangul(), pos_stem_name, ratio
                                                ),
                                                Locale::En => format!(
                                                    "Hidden stem {}({}) [{}] in {} is projected to {}, manifesting strongly. (Weight: {}%)",
                                                    stem.hanja(), stem.hangul(), ten_god.hangul(), pos_branch_name, pos_stem_name, ratio
                                                ),
                                                Locale::Zh => format!(
                                                    "{}藏干{}({})[{}]透出至{}，外部作用力显著增强。（权重：{}%）",
                                                    pos_branch_name, stem.hanja(), stem.hangul(), ten_god.hangul(), pos_stem_name, ratio
                                                ),
                                                Locale::Ru => format!(
                                                    "Скрытый ствол {}({}) [{}] из {} проецируется на {}, сильно проявляясь вовне. (Вес: {}%)",
                                                    stem.hanja(), stem.hangul(), ten_god.hangul(), pos_branch_name, pos_stem_name, ratio
                                                ),
                                            };
                                            rsx! {
                                                div { class: "p-4 rounded-xl bg-slate-850/40 border border-slate-800/80 hover:border-slate-700 transition-colors flex gap-3 shadow-inner",
                                                    span { class: "text-2xl shrink-0 mt-0.5", "{el_icon}" }
                                                    div { class: "space-y-1.5 flex-1",
                                                        div { class: "flex items-center justify-between flex-wrap gap-2",
                                                            div { class: "flex items-center gap-1.5",
                                                                span { class: "font-serif font-extrabold text-sm {el_color}", "{stem.hanja()}({stem.hangul()})" }
                                                                span { class: "text-xs font-semibold text-slate-350", "{ten_god.hangul()}" }
                                                            }
                                                            span { class: "px-2 py-0.5 rounded border text-[9px] font-extrabold {badge_cls}", "{proj_type_lbl}" }
                                                        }
                                                        p { class: "text-xs text-slate-400 leading-relaxed", "{stage_desc}" }
                                                    }
                                                }
                                            }
                                        })}
                                    }
                                }
                            }

                            // ── 1.1 사주 린트 및 조언 ──────────────────────────
                            if !data.lints.is_empty() {
                                div { class: "grid grid-cols-1 md:grid-cols-2 gap-3.5",
                                    {data.lints.iter().map(|lint| {
                                        let severity_cls = match lint.severity {
                                            eon_saju::engine::linter::LintSeverity::Error =>
                                                "border-red-800/50 bg-red-950/20 text-red-400",
                                            eon_saju::engine::linter::LintSeverity::Warning =>
                                                "border-amber-800/50 bg-amber-950/20 text-amber-400",
                                            eon_saju::engine::linter::LintSeverity::Info =>
                                                "border-blue-800/50 bg-blue-950/20 text-blue-400",
                                        };
                                        let icon = match lint.severity {
                                            eon_saju::engine::linter::LintSeverity::Error => "🚨",
                                            eon_saju::engine::linter::LintSeverity::Warning => "⚠️",
                                            eon_saju::engine::linter::LintSeverity::Info => "ℹ️",
                                        };
                                        rsx! {
                                            div { class: "p-4 rounded-2xl border flex gap-3 {severity_cls}",
                                                span { class: "text-2xl shrink-0 mt-0.5", "{icon}" }
                                                div { class: "space-y-1",
                                                    p { class: "font-bold text-sm", "[{lint.code}] {lint.message}" }
                                                    p { class: "text-xs opacity-90 leading-relaxed", "{lint.advice}" }
                                                }
                                            }
                                        }
                                    })}
                                }
                            }

                            // ── 2. 신강/신약 + 메타 ────────────────────────────
                            div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                                // 신강신약 배지
                                div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5 flex flex-col gap-3",
                                    h3 { class: "text-sm font-semibold text-slate-400 uppercase tracking-widest", "신강/신약" }
                                    div { class: "flex items-center gap-3",
                                        {
                                            let st = data.report.strength.strength_type;
                                            let (badge_color, icon) = match st {
                                                StrengthType::Strong =>
                                                    ("bg-red-500/20 text-red-300 border-red-500/50", "🔥"),
                                                StrengthType::Weak =>
                                                    ("bg-blue-500/20 text-blue-300 border-blue-500/50", "💧"),
                                                StrengthType::Balanced =>
                                                    ("bg-emerald-500/20 text-emerald-300 border-emerald-500/50", "⚖️"),
                                            };
                                            let strength_desc = format_strength_summary(
                                                locale,
                                                data.report.strength.acquired_count as usize,
                                                data.report.strength.strength_score as f64,
                                            );
                                            rsx! {
                                                span { class: "text-3xl", "{icon}" }
                                                div {
                                                    div { class: "inline-block px-3 py-1 rounded-full border text-sm font-bold {badge_color}",
                                                        "{st.hangul()} ({st.hanja()})"
                                                    }
                                                    p { class: "text-xs text-slate-500 mt-1",
                                                        "{strength_desc}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    // 득령/득지/득시/득세
                                    div { class: "grid grid-cols-2 gap-1.5 text-xs",
                                        DeukBadge { label: t(locale, TK::SajuDeukRyeong), acquired: data.report.strength.deuk_ryeong.acquired }
                                        DeukBadge { label: t(locale, TK::SajuDeukJi), acquired: data.report.strength.deuk_ji.acquired }
                                        DeukBadge { label: t(locale, TK::SajuDeukSi), acquired: data.report.strength.deuk_si.acquired }
                                        DeukBadge { label: t(locale, TK::SajuDeukSe), acquired: data.report.strength.deuk_se.acquired }
                                    }
                                }

                                // 용신
                                div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5 flex flex-col gap-3",
                                    h3 { class: "text-sm font-semibold text-slate-400 uppercase tracking-widest", "{t(locale, TK::SajuYongShen)}" }
                                    {
                                        let yn = &data.report.yongshin;
                                        let primary_el = yn.primary;
                                        let assist_el = yn.assistant;
                                        let (p_color, p_icon) = element_style(primary_el.hangul());
                                        let (a_color, _) = element_style(assist_el.hangul());
                                        rsx! {
                                            div { class: "space-y-2",
                                                div { class: "flex items-center gap-2",
                                                    span { class: "text-lg", "{p_icon}" }
                                                    div { class: "flex-1",
                                                        p { class: "text-xs text-slate-400", "{t(locale, TK::SajuPrimaryYongShen)}" }
                                                        p { class: "font-bold {p_color}", "{primary_el.hangul()} ({primary_el.hanja()})" }
                                                    }
                                                }
                                                div { class: "flex items-center gap-2",
                                                    div { class: "flex-1",
                                                        p { class: "text-xs text-slate-400", "{t(locale, TK::SajuHeeShen)}" }
                                                        p { class: "font-semibold {a_color}", "{assist_el.hangul()} ({assist_el.hanja()})" }
                                                    }
                                                }
                                                if !yn.recommendations.is_empty() {
                                                    div { class: "mt-3 pt-3 border-t border-slate-800 space-y-2.5",
                                                        p { class: "text-[10px] font-bold text-slate-500 uppercase tracking-wider", "{t(locale, TK::SajuYongShenDetail)}" }
                                                        div { class: "space-y-2",
                                                            {yn.recommendations.iter().map(|rec| {
                                                                let (el_color, el_icon) = element_style(rec.element.hangul());
                                                                let type_name = rec.yongshin_type.hangul();
                                                                rsx! {
                                                                    div { class: "p-2.5 rounded-lg bg-slate-900/50 border border-slate-800/80 space-y-1 text-xs",
                                                                        div { class: "flex justify-between items-center",
                                                                            span { class: "text-[10px] font-bold text-slate-400", "{type_name}" }
                                                                            span { class: "font-bold text-xs {el_color}", "{el_icon} {rec.element.hangul()}({rec.element.hanja()})" }
                                                                        }
                                                                        p { class: "text-xs text-slate-355 font-semibold", "{rec.summary}" }
                                                                        p { class: "text-[11px] text-slate-555 leading-relaxed", "{rec.description}" }
                                                                        if !rec.reasons.is_empty() {
                                                                            div { class: "flex flex-wrap gap-1 mt-1",
                                                                                {rec.reasons.iter().map(|r| rsx! {
                                                                                    span { class: "text-[9px] px-1 bg-slate-955/60 border border-slate-855 rounded text-slate-500", "{r}" }
                                                                                })}
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

                                // 분석 메타
                                div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5 flex flex-col gap-3",
                                    h3 { class: "text-sm font-semibold text-slate-400 uppercase tracking-widest", "{t(locale, TK::SajuAnalysisMetaTitle)}" }
                                    div { class: "space-y-2 text-sm",
                                        MetaRow { label: t(locale, TK::SajuInfoInputTime), value: data.meta.input_time.clone() }
                                        MetaRow { label: t(locale, TK::SajuInfoCorrectedTime), value: data.meta.corrected_time.clone() }
                                        MetaRow { label: t(locale, TK::SajuInfoTimezone), value: data.meta.analysis_timezone.clone() }
                                        MetaRow { label: t(locale, TK::SajuInfoDst), value: if data.meta.is_dst { t(locale, TK::SajuInfoDstApplied).to_string() } else { t(locale, TK::SajuInfoDstNone).to_string() } }
                                    }
                                }
                            }

                            // ── 3. 오행 및 십성 상세 점수 (Power Analysis) ───────
                            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                                // 오행 상세 세기
                                div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5 space-y-4",
                                    div { class: "flex justify-between items-center",
                                        h3 { class: "text-sm font-semibold text-slate-400 uppercase tracking-widest", "오행 상세 세기 (Weighted Five Elements)" }
                                        span { class: "text-xs font-bold text-amber-400 bg-amber-950/20 px-2.5 py-0.5 rounded border border-amber-900/30",
                                            "대표: {data.report.power.dominant_element.hangul()}"
                                        }
                                    }
                                    div { class: "space-y-3.5",
                                        {
                                            data.report.power.element_scores.iter().map(|item| {
                                                let (el, pct, score) = (item.0, item.1, item.2);
                                                let (color, bar_color) = match el {
                                                    eon_saju::core::element::Element::Wood => ("text-emerald-400", "bg-emerald-500"),
                                                    eon_saju::core::element::Element::Fire => ("text-rose-400", "bg-rose-500"),
                                                    eon_saju::core::element::Element::Earth => ("text-yellow-400", "bg-amber-500"),
                                                    eon_saju::core::element::Element::Metal => ("text-slate-300", "bg-slate-400"),
                                                    eon_saju::core::element::Element::Water => ("text-blue-400", "bg-blue-500"),
                                                };
                                                let label = format!("{}({})", el.hangul(), el.hanja());
                                                let score_str = format!("{:.1}점", score);
                                                let pct_str = format!(" ({:.1}%)", pct);
                                                rsx! {
                                                    div { class: "flex items-center gap-3",
                                                        div { class: "w-20 text-sm font-bold {color} text-right shrink-0",
                                                            "{label}"
                                                        }
                                                        div { class: "flex-1 h-3 bg-slate-800 rounded-full overflow-hidden shadow-inner",
                                                            div {
                                                                class: "h-full {bar_color} rounded-full transition-all duration-1000",
                                                                style: "width: {pct}%"
                                                            }
                                                        }
                                                        div { class: "w-24 text-xs font-mono text-slate-500 text-right shrink-0",
                                                            span { class: "text-slate-300 font-bold", "{score_str}" }
                                                            "{pct_str}"
                                                        }
                                                    }
                                                }
                                            })
                                        }
                                    }
                                },

                                // 십성 상세 세기
                                div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5 space-y-4",
                                    div { class: "flex justify-between items-center",
                                        h3 { class: "text-sm font-semibold text-slate-400 uppercase tracking-widest", "십성 세기 (Ten Gods Power)" }
                                        span { class: "text-xs font-bold text-indigo-400 bg-indigo-950/20 px-2.5 py-0.5 rounded border border-indigo-900/30",
                                            "대표: {data.report.power.dominant_ten_god.hangul()}"
                                        }
                                    }
                                    div { class: "space-y-2.5 max-h-[220px] overflow-y-auto pr-1",
                                        {
                                            let mut sorted_ten_gods = data.report.power.ten_god_scores.clone();
                                            sorted_ten_gods.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
                                            sorted_ten_gods.into_iter().map(|(tg, pct, score)| {
                                                let is_support = tg.is_supportive();
                                                let bar_color = if is_support { "bg-indigo-500" } else { "bg-orange-500" };
                                                let text_color = if is_support { "text-indigo-300" } else { "text-orange-300" };
                                                let label = format!("{}({})", tg.hangul(), tg.hanja());
                                                let score_str = format!("{:.1}점", score);
                                                let pct_str = format!(" ({:.1}%)", pct);
                                                rsx! {
                                                    div { class: "flex items-center gap-3",
                                                        div { class: "w-20 text-xs font-bold {text_color} text-right shrink-0",
                                                            "{label}"
                                                        }
                                                        div { class: "flex-1 h-2 bg-slate-800 rounded-full overflow-hidden",
                                                            div {
                                                                class: "h-full {bar_color} rounded-full transition-all duration-1000",
                                                                style: "width: {pct}%"
                                                            }
                                                        }
                                                        div { class: "w-24 text-[11px] font-mono text-slate-500 text-right shrink-0",
                                                            span { class: "text-slate-300 font-bold", "{score_str}" }
                                                            "{pct_str}"
                                                        }
                                                    }
                                                }
                                            })
                                        }
                                    }
                                }
                            }

                            // ── 4. 대운 타임라인 ──────────────────────────────
                            if let Some((ml, dir_str)) = &major_luck_info {
                                div { class: "bg-slate-900 border border-slate-800 rounded-2xl overflow-hidden",
                                    div { class: "bg-slate-800/50 border-b border-slate-800 px-5 py-3 flex items-center justify-between",
                                        h3 { class: "font-semibold text-slate-200", "{t(locale, TK::SajuLuckMajor)} — {dir_str}" }
                                        span { class: "text-xs text-slate-400", "{format_age_shift(locale, ml.start_age as i32)}" }
                                    }
                                    div { class: "p-4 overflow-x-auto",
                                        div { class: "flex gap-2 min-w-max",
                                            {ml.cycles.iter().map(|cycle| rsx! {
                                                div { class: "flex flex-col items-center gap-1 p-3 rounded-xl bg-slate-800/50 border border-slate-700/50 hover:border-amber-700/60 transition-colors min-w-[80px]",
                                                    span { class: "text-xs text-slate-400 font-mono", "{format_age_from(locale, cycle.start_age as i32)}" }
                                                    span { class: "text-2xl font-serif text-amber-300 font-bold", "{cycle.ganzi.hanja()}" }
                                                    span { class: "text-sm text-slate-300", "{cycle.ganzi.hangul()}" }
                                                    div { class: "flex gap-1 flex-wrap justify-center",
                                                        span { class: "text-xs px-1.5 py-0.5 rounded bg-slate-700 text-amber-400", "{cycle.stem_god.hangul()}" }
                                                        span { class: "text-xs px-1.5 py-0.5 rounded bg-slate-700 text-blue-400", "{cycle.branch_god.hangul()}" }
                                                    }
                                                }
                                            })}
                                        }
                                    }
                                }
                            }

                            // ── 5. 보조 기둥 (태원·명궁·신궁) ─────────────────────
                            div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5 space-y-4",
                                h3 { class: "text-sm font-semibold text-slate-400 uppercase tracking-widest", "{t(locale, TK::SajuPillarsSupplementary)}" }
                                div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                                    {
                                        let sp = &data.report.supplementary_pillars;
                                        let aux_pillars = vec![
                                            (t(locale, TK::SajuPillarTaiYuan), &sp.taewon, "taewon"),
                                            (t(locale, TK::SajuPillarMingGong), &sp.myeonggung, "myeonggung"),
                                            (t(locale, TK::SajuPillarShenGong), &sp.shingung, "shingung"),
                                        ];
                                        aux_pillars.into_iter().map(|(label, gz, code)| {
                                            let s_el = gz.stem.element().hangul();
                                            let b_el = gz.branch.element().hangul();
                                            let (s_color, _, _) = element_card_style(s_el);
                                            let (b_color, _, _) = element_card_style(b_el);
                                            let s_shinsals: Vec<String> = data.report.spirit_markers.aux_shinsals.iter()
                                                .filter(|s| s.0.contains(code) || (code == "taewon" && s.0 == "태원") || (code == "myeonggung" && s.0 == "명궁") || (code == "shingung" && s.0 == "신궁"))
                                                .map(|s| s.2.clone())
                                                .collect();
                                            rsx! {
                                                div { class: "p-4 rounded-xl bg-slate-800/40 border border-slate-800 flex flex-col gap-2 shadow-inner",
                                                    p { class: "text-xs font-bold text-slate-500", "{label}" }
                                                    div { class: "flex items-center gap-2",
                                                        span { class: "text-3xl font-extrabold font-serif {s_color}", "{gz.stem.hanja()}" }
                                                        span { class: "text-3xl font-extrabold font-serif {b_color}", "{gz.branch.hanja()}" }
                                                        span { class: "text-sm text-slate-300 font-bold", "{gz.stem.hangul()}{gz.branch.hangul()}" }
                                                    }
                                                    if !s_shinsals.is_empty() {
                                                        div { class: "flex flex-wrap gap-1 mt-1",
                                                            {s_shinsals.iter().map(|s| rsx! {
                                                                span { class: "text-[9px] py-0.5 px-2 rounded-full bg-indigo-950/40 border border-indigo-900/40 text-indigo-300 font-bold",
                                                                    "✦ {s}"
                                                                }
                                                            })}
                                                        }
                                                    }
                                                }
                                            }
                                        })
                                    }
                                }
                            }

                            // ── 5.1 공망 분석 (Void / Emptiness) ──────────────────
                            div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5 space-y-4",
                                h3 { class: "text-sm font-semibold text-slate-400 uppercase tracking-widest", "{t(locale, TK::SajuVoidTitle)}" }
                                div { class: "p-4 rounded-xl bg-slate-800/40 border border-slate-800 flex flex-col gap-3 shadow-inner",
                                    div { class: "flex flex-wrap gap-x-6 gap-y-2 text-sm",
                                        p { class: "text-slate-400 font-bold",
                                            "{t(locale, TK::SajuVoidXun)}: "
                                            span { class: "text-amber-400", {data.report.voids.xun_group.clone()} }
                                        }
                                        p { class: "text-slate-400 font-bold",
                                            "{t(locale, TK::SajuVoidBranches)}: "
                                            span { class: "text-red-400 font-serif",
                                                {format!("{}({}), {}({})",
                                                    data.report.voids.void_branches[0].hanja(),
                                                    data.report.voids.void_branches[0].hangul(),
                                                    data.report.voids.void_branches[1].hanja(),
                                                    data.report.voids.void_branches[1].hangul()
                                                )}
                                            }
                                        }
                                        if !data.report.voids.void_ten_gods.is_empty() {
                                            p { class: "text-slate-400 font-bold",
                                                "{t(locale, TK::SajuVoidTenGods)}: "
                                                span { class: "text-indigo-400",
                                                    {data.report.voids.void_ten_gods.iter().map(|tg| tg.hangul()).collect::<Vec<_>>().join(", ")}
                                                }
                                            }
                                        }
                                    }
                                    if !data.report.voids.mapped_voids.is_empty() {
                                        div { class: "border-t border-slate-800/80 pt-2 space-y-2",
                                            {data.report.voids.mapped_voids.iter().map(|void_dt| {
                                                let lvl_cls = match void_dt.level {
                                                    eon_saju::analysis::supplementary_pillars::InterpretationLevel::Auspicious => "text-emerald-400 bg-emerald-950/20 border-emerald-800/30",
                                                    eon_saju::analysis::supplementary_pillars::InterpretationLevel::Caution => "text-rose-400 bg-rose-950/20 border-rose-800/30",
                                                    eon_saju::analysis::supplementary_pillars::InterpretationLevel::Neutral => "text-slate-400 bg-slate-800 border-slate-700",
                                                };
                                                let label_branch = format!("{}({})", void_dt.branch.hanja(), void_dt.branch.hangul());
                                                let label_tg = void_dt.ten_god.hangul().to_string();
                                                let pos_str = match void_dt.position.to_string().as_str() {
                                                    "연주" | "Year" => t(locale, TK::SajuYearPillar),
                                                    "월주" | "Month" => t(locale, TK::SajuMonthPillar),
                                                    "일주" | "Day" => t(locale, TK::SajuDayPillar),
                                                    "시주" | "Hour" => t(locale, TK::SajuHourPillar),
                                                    _ => &void_dt.position.to_string(),
                                                };
                                                let label_pos = format!("{} {}", pos_str, t(locale, TK::SajuVoidSuffix));
                                                rsx! {
                                                    div { class: "p-3 rounded-lg bg-slate-900/40 border border-slate-800/60 space-y-1 text-xs hover:border-slate-750 transition-colors",
                                                        div { class: "flex justify-between items-center gap-2 flex-wrap",
                                                            div { class: "flex items-center gap-1.5",
                                                                span { class: "font-serif text-slate-200 font-bold text-sm", "{label_branch}" }
                                                                span { class: "text-amber-400/90 font-semibold", "{label_tg}" }
                                                                span { class: "px-2 py-0.5 rounded border text-[9px] font-bold {lvl_cls}",
                                                                    {match void_dt.level {
                                                                        eon_saju::analysis::supplementary_pillars::InterpretationLevel::Auspicious => t(locale, TK::SajuLevelAuspicious),
                                                                        eon_saju::analysis::supplementary_pillars::InterpretationLevel::Caution => t(locale, TK::SajuLevelCaution),
                                                                        eon_saju::analysis::supplementary_pillars::InterpretationLevel::Neutral => t(locale, TK::LabelNeutral),
                                                                    }}
                                                                }
                                                            }
                                                            span { class: "text-[10px] text-slate-500 font-mono", "{label_pos}" }
                                                        }
                                                        p { class: "text-slate-300 font-semibold", "{void_dt.summary}" }
                                                        p { class: "text-slate-400 leading-relaxed", "{void_dt.description}" }
                                                    }
                                                }
                                            })}
                                        }
                                    }
                                }
                            }

                            // ── 5.1.2 격국 분석 (Structure / Pattern) ─────────────
                            {
                                let struct_dt = &data.report.structure;
                                let struct_lbl = format!("{}({})", struct_dt.structure.hangul(), struct_dt.structure.hanja());
                                let proj_stem_lbl = struct_dt.projected_stem.map(|s| format!("{}({})", s.hanja(), s.hangul()));
                                rsx! {
                                    div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5 space-y-4 shadow-xl",
                                        div { class: "flex items-center justify-between border-b border-slate-800/60 pb-3 flex-wrap gap-2",
                                            div { class: "space-y-0.5",
                                                h3 { class: "text-sm font-semibold text-slate-200 uppercase tracking-widest", "격국 분석 (Structure & Pattern)" }
                                                p { class: "text-xs text-slate-500", "월지 지장간의 투출 상태를 근거로 타고난 사회적 그릇과 직업적 성향의 격식을 판별합니다." }
                                            }
                                            span { class: "text-xs font-bold text-amber-400 bg-amber-950/20 px-3 py-1 rounded border border-amber-900/30",
                                                "격국 결정: {struct_lbl}"
                                            }
                                        }
                                        div { class: "p-4 rounded-xl bg-slate-800/40 border border-slate-800 space-y-3.5 shadow-inner",
                                            if let Some(stem_lbl) = proj_stem_lbl {
                                                div { class: "flex items-center gap-4 text-xs font-mono text-slate-400 flex-wrap",
                                                    div { "투출 천간 (Projected Stem): "
                                                        span { class: "text-indigo-400 font-bold text-sm ml-1", "{stem_lbl}" }
                                                    }
                                                    if let Some(path) = &struct_dt.projection_path {
                                                        div { "투출 위치 (Projection Path): "
                                                            span { class: "text-indigo-400 font-bold text-sm ml-1", "{path}" }
                                                        }
                                                    }
                                                }
                                            }
                                            div { class: "space-y-1.5",
                                                p { class: "text-sm text-slate-200 font-bold", "{struct_dt.summary}" }
                                                p { class: "text-xs text-slate-400 leading-relaxed", "{struct_dt.description}" }
                                            }
                                            if !struct_dt.reasons.is_empty() {
                                                div { class: "border-t border-slate-800/80 pt-3 space-y-2",
                                                    p { class: "text-[10px] font-bold text-slate-500 uppercase tracking-wider", "격국 성립 근거" }
                                                    div { class: "flex flex-wrap gap-2",
                                                        {struct_dt.reasons.iter().map(|reason| rsx! {
                                                            span { class: "px-2 py-0.5 bg-slate-900 border border-slate-800/80 text-[10px] text-slate-400 rounded-md", "{reason}" }
                                                        })}
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // ── 5.2 합충형해 분석 (Harmony & Clashes) ──────────────
                            if !data.report.relationships.mapped_relationships.is_empty() {
                                div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5 space-y-4",
                                    h3 { class: "text-sm font-semibold text-slate-400 uppercase tracking-widest", "합충형해 분석 (Harmony & Clashes)" }
                                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-3.5",
                                        {data.report.relationships.mapped_relationships.iter().map(|rel| {
                                            let lvl_cls = match rel.level {
                                                eon_saju::analysis::supplementary_pillars::InterpretationLevel::Auspicious => "text-emerald-400 bg-emerald-950/30 border-emerald-800/40",
                                                eon_saju::analysis::supplementary_pillars::InterpretationLevel::Caution => "text-rose-400 bg-rose-950/30 border-rose-800/40",
                                                eon_saju::analysis::supplementary_pillars::InterpretationLevel::Neutral => "text-slate-400 bg-slate-850 border-slate-800",
                                            };
                                            let pos_str = rel.positions.join("-");
                                            let rel_name = rel.name.clone();
                                            let trans_label = rel.transformed_element.map(|el| format!("{}({})", el.hangul(), el.hanja()));
                                            rsx! {
                                                div { class: "p-4 rounded-xl bg-slate-800/20 border border-slate-800/60 space-y-2 hover:border-slate-700 transition-colors",
                                                    div { class: "flex items-center justify-between gap-2 flex-wrap",
                                                        div { class: "flex items-center gap-2",
                                                            span { class: "text-base font-bold text-slate-200", "{rel_name}" }
                                                            span { class: "text-xs px-2 py-0.5 rounded border font-bold {lvl_cls}",
                                                                {match rel.level {
                                                                    eon_saju::analysis::supplementary_pillars::InterpretationLevel::Auspicious => "조화(吉)",
                                                                    eon_saju::analysis::supplementary_pillars::InterpretationLevel::Caution => "대립(凶)",
                                                                    eon_saju::analysis::supplementary_pillars::InterpretationLevel::Neutral => "작용",
                                                                }}
                                                             }
                                                        }
                                                        span { class: "text-[10px] font-mono font-bold text-indigo-400/80 bg-indigo-950/20 px-2 py-0.5 rounded border border-indigo-900/30",
                                                            "{pos_str}"
                                                        }
                                                    }
                                                    p { class: "text-sm text-slate-300 font-semibold", "{rel.summary}" }
                                                    p { class: "text-xs text-slate-400 leading-relaxed", "{rel.description}" }
                                                    if let Some(trans_el_str) = trans_label {
                                                        div { class: "pt-1 flex items-center gap-1 text-[10px]",
                                                            span { class: "text-slate-500 font-bold", "합화 오행 (Transformation):" }
                                                            span { class: "px-2 py-0.5 bg-amber-950/20 border border-amber-900/30 text-amber-400 rounded-md font-bold font-mono",
                                                                "{trans_el_str}"
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        })}
                                    }
                                }
                            }

                            // ── 5.2.2 지장간 암합 (暗合) 및 명암합 (明暗合) ───────────
                            if !data.report.relationships.am_combinations.is_empty() || !data.report.relationships.myung_am_combinations.is_empty() {
                                div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5 space-y-4 shadow-xl",
                                    div { class: "flex items-center justify-between border-b border-slate-800/60 pb-3",
                                        div { class: "space-y-0.5",
                                            h3 { class: "text-sm font-semibold text-slate-200 uppercase tracking-widest", "지장간 암합 & 명암합 (Hidden Harmony)" }
                                            p { class: "text-xs text-slate-500", "드러나지 않는 내면적 심리 결합 및 은밀하게 이루어지는 상호작용을 분석합니다." }
                                        }
                                    }
                                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-5",
                                        // 암합 (지장간끼리의 비밀스런 합)
                                        if !data.report.relationships.am_combinations.is_empty() {
                                            div { class: "space-y-3",
                                                h4 { class: "text-xs font-bold text-amber-400/90 tracking-wider flex items-center gap-1.5", 
                                                    span { "🔒" }
                                                    span { "지장간 암합 (暗合 — 내밀한 계약/비밀 연대)" }
                                                }
                                                div { class: "space-y-2",
                                                    {data.report.relationships.am_combinations.iter().map(|(am, p1, p2)| {
                                                        let b1_lbl = format!("{}({})", am.branches.0.hanja(), am.branches.0.hangul());
                                                        let b2_lbl = format!("{}({})", am.branches.1.hanja(), am.branches.1.hangul());
                                                        let comb_lbl = format!("{}({})", am.combination.hanja(), am.combination.hangul());
                                                        let trans_el = am.combination.transformed_element();
                                                        let trans_lbl = format!("{}({})", trans_el.hangul(), trans_el.hanja());
                                                        rsx! {
                                                            div { class: "p-3 rounded-xl bg-slate-800/20 border border-slate-800 space-y-1.5 hover:border-slate-750 transition-colors",
                                                                div { class: "flex items-center justify-between",
                                                                    span { class: "text-sm font-bold text-slate-300", "{b1_lbl} ⇄ {b2_lbl}" }
                                                                    span { class: "text-[10px] font-mono font-bold text-indigo-400 bg-indigo-950/20 px-2 py-0.5 rounded border border-indigo-900/30", "{p1} ⇄ {p2}" }
                                                                }
                                                                div { class: "flex flex-wrap gap-x-3 gap-y-1 items-center text-xs",
                                                                    div { class: "flex items-center gap-1",
                                                                        span { class: "text-slate-500", "성립합:" }
                                                                        span { class: "text-amber-400 font-bold", "{comb_lbl}" }
                                                                    }
                                                                    div { class: "flex items-center gap-1",
                                                                        span { class: "text-slate-500", "합화오행:" }
                                                                        span { class: "px-1.5 py-0.2 bg-amber-950/30 border border-amber-900/30 text-amber-500 rounded text-[10px] font-bold", "{trans_lbl}" }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    })}
                                                }
                                            }
                                        }
                                        // 명암합 (천간과 지장간 사이의 합)
                                        if !data.report.relationships.myung_am_combinations.is_empty() {
                                            div { class: "space-y-3",
                                                h4 { class: "text-xs font-bold text-indigo-400 tracking-wider flex items-center gap-1.5", 
                                                    span { "🔓" }
                                                    span { "명암합 (明暗合 — 대외적 명분과 실리의 결착)" }
                                                }
                                                div { class: "space-y-2",
                                                    {data.report.relationships.myung_am_combinations.iter().map(|(ma, p1, p2)| {
                                                        let stem_lbl = format!("{}({})", ma.stem.hanja(), ma.stem.hangul());
                                                        let branch_lbl = format!("{}({})", ma.branch.hanja(), ma.branch.hangul());
                                                        let comb_lbl = format!("{}({})", ma.combination.hanja(), ma.combination.hangul());
                                                        let trans_el = ma.combination.transformed_element();
                                                        let trans_lbl = format!("{}({})", trans_el.hangul(), trans_el.hanja());
                                                        rsx! {
                                                            div { class: "p-3 rounded-xl bg-slate-800/20 border border-slate-800 space-y-1.5 hover:border-slate-750 transition-colors",
                                                                div { class: "flex items-center justify-between",
                                                                    span { class: "text-sm font-bold text-slate-350", "{stem_lbl} (천간) ⇄ {branch_lbl} (지지)" }
                                                                    span { class: "text-[10px] font-mono font-bold text-indigo-400 bg-indigo-950/20 px-2 py-0.5 rounded border border-indigo-900/30", "{p1} ⇄ {p2}" }
                                                                }
                                                                div { class: "flex flex-wrap gap-x-3 gap-y-1 items-center text-xs",
                                                                    div { class: "flex items-center gap-1",
                                                                        span { class: "text-slate-500", "성립합:" }
                                                                        span { class: "text-amber-400 font-bold", "{comb_lbl}" }
                                                                    }
                                                                    div { class: "flex items-center gap-1",
                                                                        span { class: "text-slate-500", "합화오행:" }
                                                                        span { class: "px-1.5 py-0.2 bg-indigo-950/30 border border-indigo-900/30 text-indigo-400 rounded text-[10px] font-bold", "{trans_lbl}" }
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

                            // ── 5.3 시스템 공학 진단 (System Engineering Diagnostics) ──
                            div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5 space-y-4 shadow-xl",
                                h3 { class: "text-sm font-semibold text-slate-400 uppercase tracking-widest", "시스템 공학 진단 (System Engineering & Topology)" }
                                div { class: "grid grid-cols-1 md:grid-cols-3 gap-4.5",
                                    // 1) Entropy & Obfuscation
                                    div { class: "p-4 rounded-xl bg-slate-850/50 border border-slate-800 space-y-3 flex flex-col justify-between shadow-inner",
                                        div { class: "space-y-1.5",
                                            p { class: "text-xs font-bold text-slate-500 uppercase tracking-wider", "운명 난독화 및 엔트로피" }
                                            p { class: "text-lg font-bold text-indigo-300",
                                                {match &data.entropy.level {
                                                    eon_saju::engine::entropy::ObfuscationLevel::Plaintext => "단순형 (Plaintext)",
                                                    eon_saju::engine::entropy::ObfuscationLevel::Standard => "보통형 (Standard)",
                                                    eon_saju::engine::entropy::ObfuscationLevel::Packed => "복합형 (Packed)",
                                                    eon_saju::engine::entropy::ObfuscationLevel::Encrypted => "복잡형 (Encrypted)",
                                                }}
                                            }
                                            p { class: "text-xs text-slate-400 leading-relaxed", {data.entropy.description.clone()} }
                                        }
                                        div { class: "border-t border-slate-800/80 pt-2 flex flex-col gap-1.5 text-xs text-slate-500",
                                            p {
                                                "Shannon Entropy: "
                                                span { class: "font-mono font-bold text-slate-300", {format!("{:.3} / 2.322", data.entropy.score)} }
                                            }
                                            if let Some((style, label)) = &unpacker_info {
                                                p {
                                                    "디컴파일 오행 (Unpacker): "
                                                    span { class: "font-bold {style}", "{label}" }
                                                }
                                            }
                                        }
                                    }

                                    // 2) Qi Network Flow Topology
                                    div { class: "p-4 rounded-xl bg-slate-850/50 border border-slate-800 space-y-3 flex flex-col justify-between shadow-inner",
                                        div { class: "space-y-1.5",
                                            p { class: "text-xs font-bold text-slate-500 uppercase tracking-wider", "에너지 네트워크 토폴로지" }
                                            p { class: "text-lg font-bold text-emerald-400", {format!("유동 효율: {:.1}%", data.qi_topology.throughput * 100.0)} }
                                            div { class: "space-y-1 mt-1 text-[11px] font-mono",
                                                {data.qi_topology.nodes.iter().map(|node| {
                                                    let (txt_color, _) = element_style(node.element.hangul());
                                                    let capacity_pct = if node.capacity > 0.0 { (node.output / node.capacity * 100.0).min(100.0) } else { 0.0 };
                                                    let node_el_hangul = node.element.hangul();
                                                    let node_label = format!("대역폭: {:.0} | 출력: {:.1} ({:.0}%)", node.capacity, node.output, capacity_pct);
                                                    rsx! {
                                                        div { class: "flex justify-between items-center",
                                                            span { class: "font-bold {txt_color}", "{node_el_hangul}" }
                                                            span { class: "text-slate-400", "{node_label}" }
                                                        }
                                                    }
                                                })}
                                            }
                                        }
                                        div { class: "border-t border-slate-800/80 pt-2 flex flex-col gap-1 text-xs text-slate-500",
                                            if let Some((style, label)) = &bottleneck_info {
                                                p {
                                                    "흐름 정체 구간 (Bottleneck): "
                                                    span { class: "font-bold {style}", "{label}" }
                                                }
                                            } else {
                                                p { "흐름 정체 구간 (Bottleneck): 없음" }
                                            }
                                        }
                                    }

                                    // 3) Cyclomatic Complexity (VM execution)
                                    div { class: "p-4 rounded-xl bg-slate-850/50 border border-slate-800 space-y-3 flex flex-col justify-between shadow-inner",
                                        div { class: "space-y-1.5",
                                            p { class: "text-xs font-bold text-slate-500 uppercase tracking-wider", "가상머신(VM) 순환 복잡도" }
                                            if let Some((comp_label, _, _)) = &complexity_info {
                                                p { class: "text-lg font-bold text-amber-400", "{comp_label}" }
                                                p { class: "text-xs text-slate-400 leading-relaxed",
                                                    "인생 시뮬레이션 상 감지된 주요 결정 분기점(Decision Points) 개수와 시스템 안정성 등급입니다."
                                                }
                                            } else {
                                                p { class: "text-slate-500 text-xs", "VM 시뮬레이션 복잡도 분석 결과 없음" }
                                            }
                                        }
                                        div { class: "border-t border-slate-800/80 pt-2 flex flex-col gap-1 text-xs text-slate-500",
                                            if let Some((_, entropy_str, decision_ages)) = &complexity_info {
                                                p {
                                                    "유지보수 엔트로피: "
                                                    span { class: "font-mono font-bold text-slate-300", "{entropy_str}" }
                                                }
                                                if !decision_ages.is_empty() {
                                                    p { class: "truncate",
                                                        "주요 분기점 연령: "
                                                        span { class: "font-bold text-amber-500", "{decision_ages}" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // ── 5.4 운명 크래시 & 인생 부하 진단 (Fuzzer & Load Balancer) ──
                            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                                // 1) Destiny Fuzzer (운명 취약점 퍼징)
                                div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5 space-y-4 shadow-xl",
                                    div { class: "flex justify-between items-center",
                                        h3 { class: "text-sm font-semibold text-slate-400 uppercase tracking-widest", "{t(locale, TK::SajuFuzzerTitle)}" }
                                        span { class: "text-xs font-bold text-rose-400 bg-rose-950/20 px-2.5 py-0.5 rounded border border-rose-900/30",
                                            "{crashes_lbl}"
                                        }
                                    }
                                    p { class: "text-xs text-slate-400 leading-relaxed",
                                        "{t(locale, TK::SajuFuzzerDesc)}"
                                    }
                                    if let Some(fuzz) = &data.vulnerability_report {
                                        div { class: "space-y-3.5 max-h-[300px] overflow-y-auto pr-1",
                                            {fuzz.critical_vectors.iter().map(|vuln| {
                                                let score_val = vuln.crash_score;
                                                let energy_lbl = t(locale, TK::SajuFuzzerEnergyLevel).replace("{:.1}", &format!("{:.1}", score_val));
                                                let vuln_type = vuln.vulnerability_type.clone();
                                                let major_gz = format!("{}{}", vuln.vector.major.hanja(), vuln.vector.major.hangul());
                                                let yearly_gz = format!("{}{}", vuln.vector.yearly.hanja(), vuln.vector.yearly.hangul());
                                                let vector_desc = t(locale, TK::SajuFuzzerVector).replacen("{}", &major_gz, 1).replacen("{}", &yearly_gz, 1);
                                                let tags_list = vuln.tags.join(", ");
                                                let tags_desc = t(locale, TK::SajuFuzzerTags).replace("{}", &tags_list);
                                                rsx! {
                                                    div { class: "p-3.5 rounded-xl bg-slate-950/40 border border-slate-855 hover:border-red-900/40 transition-colors flex flex-col gap-1.5 shadow-inner",
                                                        div { class: "flex justify-between items-center flex-wrap gap-2",
                                                            span { class: "text-xs font-mono font-extrabold text-rose-400", "⚠️ {vuln_type}" }
                                                            span { class: "text-xs font-bold text-rose-500", "{energy_lbl}" }
                                                        }
                                                        p { class: "text-xs text-slate-300 font-serif", "{vector_desc}" }
                                                        if !tags_list.is_empty() {
                                                            p { class: "text-[10px] text-slate-500 font-mono", "{tags_desc}" }
                                                        }
                                                    }
                                                }
                                            })}
                                        }
                                    } else {
                                        p { class: "text-slate-500 text-xs py-4", "{t(locale, TK::SajuFuzzerNoCrash)}" }
                                    }
                                }

                                // 2) Karma Load Balancer (인생 부하 분산 진단)
                                div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5 space-y-4 shadow-xl",
                                    h3 { class: "text-sm font-semibold text-slate-400 uppercase tracking-widest", "{t(locale, TK::SajuLoadTitle)}" }
                                    p { class: "text-xs text-slate-400 leading-relaxed",
                                        "{t(locale, TK::SajuLoadDesc)}"
                                    }
                                    if !data.load_diagnostics.is_empty() {
                                        div { class: "space-y-3.5 max-h-[300px] overflow-y-auto pr-1",
                                            {data.load_diagnostics.iter().map(|diag| {
                                                let (badge_cls, status_lbl) = match diag.status {
                                                    eon_saju::engine::load_balancer::TrafficStatus::Idle => ("text-emerald-400 bg-emerald-950/20 border-emerald-900/30", match locale {
                                                        Locale::Ko => "평온 (Idle)",
                                                        Locale::En => "Idle",
                                                        Locale::Zh => "平稳 (Idle)",
                                                        Locale::Ru => "Покой (Idle)",
                                                    }),
                                                    eon_saju::engine::load_balancer::TrafficStatus::Normal => ("text-blue-400 bg-blue-950/20 border-blue-900/30", match locale {
                                                        Locale::Ko => "보통 (Normal)",
                                                        Locale::En => "Normal",
                                                        Locale::Zh => "正常 (Normal)",
                                                        Locale::Ru => "Нормально (Normal)",
                                                    }),
                                                    eon_saju::engine::load_balancer::TrafficStatus::Overloaded => ("text-amber-400 bg-amber-950/20 border-amber-900/30", match locale {
                                                        Locale::Ko => "오버로드 (Overload)",
                                                        Locale::En => "Overload",
                                                        Locale::Zh => "过载 (Overload)",
                                                        Locale::Ru => "Перегрузка (Overload)",
                                                    }),
                                                    eon_saju::engine::load_balancer::TrafficStatus::SystemDown => ("text-rose-400 bg-rose-950/20 border-rose-900/30", match locale {
                                                        Locale::Ko => "다운 (System Down)",
                                                        Locale::En => "System Down",
                                                        Locale::Zh => "系统故障 (System Down)",
                                                        Locale::Ru => "Системный сбой (System Down)",
                                                    }),
                                                };
                                                let age_val = diag.age;
                                                let reason_desc = diag.reason.clone();
                                                let strategy_desc = diag.strategy.clone();
                                                rsx! {
                                                    div { class: "p-3.5 rounded-xl bg-slate-950/40 border border-slate-850 hover:border-slate-750 transition-colors flex flex-col gap-1.5 shadow-inner",
                                                        div { class: "flex justify-between items-center flex-wrap gap-2",
                                                            span { class: "text-xs font-bold text-slate-300 font-mono", "{format_age(locale, age_val as i32)}" }
                                                            span { class: "text-[10px] font-bold px-2 py-0.5 rounded border {badge_cls}", "{status_lbl}" }
                                                        }
                                                        p { class: "text-xs font-semibold text-amber-400", "{reason_desc}" }
                                                        p { class: "text-[11px] text-slate-400 leading-relaxed", "{strategy_desc}" }
                                                    }
                                                }
                                            })}
                                        }
                                    } else {
                                        p { class: "text-slate-500 text-xs py-4", "{t(locale, TK::SajuLoadNoEvent)}" }
                                    }
                                }
                            }

                            // ── 6. 신살 상세 해설 (Spirit Markers Detail) ─────────
                            if !data.report.spirit_markers.mapped_markers.is_empty() {
                                div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5 space-y-4",
                                    h3 { class: "text-sm font-semibold text-slate-400 uppercase tracking-widest", "{t(locale, TK::SajuSpiritDetailTitle)}" }
                                    div { class: "space-y-3",
                                        {data.report.spirit_markers.mapped_markers.iter().map(|m| {
                                            let lvl_cls = match m.level {
                                                InterpretationLevel::Auspicious => "text-emerald-400 bg-emerald-950/30 border-emerald-800/40",
                                                InterpretationLevel::Caution => "text-rose-400 bg-rose-950/30 border-rose-800/40",
                                                InterpretationLevel::Neutral => "text-slate-400 bg-slate-850 border-slate-800",
                                            };
                                            let pos_name = match m.position {
                                                eon_saju::analysis::spirit_markers::PillarPosition::Year => t(locale, TK::SajuYearPillar),
                                                eon_saju::analysis::spirit_markers::PillarPosition::Month => t(locale, TK::SajuMonthPillar),
                                                eon_saju::analysis::spirit_markers::PillarPosition::Day => t(locale, TK::SajuDayPillar),
                                                eon_saju::analysis::spirit_markers::PillarPosition::Hour => t(locale, TK::SajuHourPillar),
                                            };
                                            let part = if m.is_stem { t(locale, TK::SajuPillarStem) } else { t(locale, TK::SajuPillarBranch) };
                                            let marker_label = format!("{} ({})", m.marker.hangul(), m.marker.hanja());
                                            let pos_part = format!("{} {}", pos_name, part);
                                            rsx! {
                                                div { class: "p-4 rounded-xl bg-slate-800/20 border border-slate-800/60 space-y-2 hover:border-slate-700 transition-colors",
                                                    div { class: "flex items-center justify-between gap-2 flex-wrap",
                                                        div { class: "flex items-center gap-2",
                                                            span { class: "text-base font-bold text-slate-200", "{marker_label}" }
                                                            span { class: "text-xs px-2.5 py-0.5 rounded border font-bold {lvl_cls}",
                                                                {match m.level {
                                                                    InterpretationLevel::Auspicious => t(locale, TK::SajuLevelAuspicious),
                                                                    InterpretationLevel::Caution => t(locale, TK::SajuLevelCaution),
                                                                    InterpretationLevel::Neutral => t(locale, TK::LabelNeutral),
                                                                }}
                                                            }
                                                        }
                                                        span { class: "text-[10px] font-bold text-amber-400/80 bg-amber-950/20 px-2.5 py-0.5 rounded border border-amber-900/30 font-mono",
                                                            "{pos_part}"
                                                        }
                                                    }
                                                    p { class: "text-sm text-slate-300 font-semibold", "{m.summary}" }
                                                    p { class: "text-xs text-slate-400 leading-relaxed", "{m.description}" }
                                                    if !m.reasons.is_empty() {
                                                        div { class: "flex items-center gap-1.5 flex-wrap pt-1",
                                                            span { class: "text-[10px] text-slate-500 font-bold", "{t(locale, TK::SajuSpiritRequirement)}" }
                                                            {m.reasons.iter().map(|r| rsx! {
                                                                span { class: "text-[10px] px-2 py-0.5 bg-slate-800/80 border border-slate-700/40 text-slate-400 rounded-md font-mono", "{r}" }
                                                            })}
                                                        }
                                                    }
                                                }
                                            }
                                        })}
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
}

fn element_card_style(el_name: &str) -> (&'static str, &'static str, &'static str) {
    match el_name {
        "목" => ("text-emerald-400", "bg-emerald-950/20 border-emerald-800/30", "🌿"),
        "화" => ("text-rose-400", "bg-rose-950/20 border-rose-800/30", "🔥"),
        "토" => ("text-yellow-400", "bg-amber-950/20 border-amber-900/30", "⛰️"),
        "금" => ("text-slate-300", "bg-slate-800/40 border-slate-700/30", "⚙️"),
        "수" => ("text-blue-400", "bg-blue-950/20 border-blue-800/30", "💧"),
        _ => ("text-slate-400", "bg-slate-900/40 border-slate-800/30", "◆"),
    }
}

fn element_style(hangul: &str) -> (&'static str, &'static str) {
    match hangul {
        "목" => ("text-emerald-400", "🌿"),
        "화" => ("text-red-400", "🔥"),
        "토" => ("text-yellow-400", "⛰️"),
        "금" => ("text-slate-300", "⚙️"),
        "수" => ("text-blue-400", "💧"),
        _ => ("text-slate-400", "◆"),
    }
}

#[component]
fn PillarCard(
    title: &'static str,
    stem_god: String,
    stem_hanja: String,
    stem_hangul: String,
    stem_element: String,
    branch_god: String,
    branch_hanja: String,
    branch_hangul: String,
    branch_element: String,
    twelve_stage: String,
    shinsals: Vec<String>,
    jijanggans: Vec<JijangganDisplayItem>,
) -> Element {
    let state = use_context::<AnalysisState>();
    let locale = *state.locale.read();
    let (s_text_color, s_bg_color, s_icon) = element_card_style(&stem_element);
    let (b_text_color, b_bg_color, b_icon) = element_card_style(&branch_element);

    rsx! {
        div { class: "flex flex-col gap-2.5 p-4 rounded-2xl bg-slate-950/40 border border-slate-800 hover:scale-[1.02] hover:border-slate-700 transition-all duration-200 shadow-xl",
            // Pillar Title
            div { class: "text-center text-xs font-bold text-slate-400 tracking-widest pb-2 border-b border-slate-800",
                "{title}"
            }

            // Heavenly Stem (천간)
            div { class: "flex flex-col items-center p-3.5 rounded-xl {s_bg_color} border flex-1 justify-center relative overflow-hidden",
                span { class: "text-xs font-semibold text-slate-500 mb-1.5", "{stem_god}" }
                span { class: "text-5xl font-extrabold font-serif {s_text_color} leading-none tracking-tight", "{stem_hanja}" }
                span { class: "text-base text-slate-300 font-bold mt-1.5", "{stem_hangul}" }
                span { class: "text-[10px] text-slate-500 font-mono mt-1", "{s_icon} {stem_element}" }
            }

            // Earthly Branch (지지)
            div { class: "flex flex-col items-center p-3.5 rounded-xl {b_bg_color} border flex-1 justify-center relative overflow-hidden",
                span { class: "text-[10px] text-slate-500 font-mono mb-1", "{b_icon} {branch_element}" }
                span { class: "text-5xl font-extrabold font-serif {b_text_color} leading-none tracking-tight", "{branch_hanja}" }
                span { class: "text-base text-slate-300 font-bold mt-1.5", "{branch_hangul}" }
                span { class: "text-xs font-semibold text-slate-500 mt-1.5", "{branch_god}" }
            }

            // 지장간 리스트 (Jijanggan List)
            div { class: "bg-slate-900/65 border border-slate-800/80 rounded-xl p-2.5 space-y-1.5 mt-0.5 shadow-inner",
                p { class: "text-[9px] font-bold text-slate-500 uppercase tracking-wider text-center border-b border-slate-800/60 pb-1", 
                    "{t(locale, TK::SajuHiddenStemsTitle)}" 
                }
                div { class: "space-y-1.5",
                    {jijanggans.iter().map(|item| {
                        let (el_color, _) = element_style(item.stem.element().hangul());
                        let (card_style, badge_style) = if item.is_main {
                            ("border border-amber-500/50 bg-amber-950/20 rounded-md p-1", "bg-amber-500/20 text-amber-300 border border-amber-500/30 text-[8px] px-1.5 py-0.2 rounded font-extrabold")
                        } else if item.is_projected {
                            ("border border-indigo-500/40 bg-indigo-950/15 rounded-md p-1", "bg-indigo-500/20 text-indigo-300 border border-indigo-500/30 text-[8px] px-1.5 py-0.2 rounded font-extrabold")
                        } else {
                            ("", "")
                        };
                        rsx! {
                            div { class: "flex items-center justify-between text-[11px] {card_style}",
                                div { class: "flex items-center gap-1.5",
                                    span { class: "text-[8px] text-slate-500 font-mono", "{t(locale, item.type_key)}" }
                                    span { class: "font-serif font-bold text-slate-300", "{item.stem.hanja()}({item.stem.hangul()})" }
                                    span { class: "text-[9px] {el_color} font-medium", "{item.ten_god.hangul()}" }
                                }
                                div { class: "flex items-center gap-1",
                                    if item.is_projected {
                                        span { class: "{badge_style}", 
                                            if item.is_main { "{t(locale, TK::SajuProjLevelMain)}" } else { "{t(locale, TK::SajuProjLevelSub)}" }
                                        }
                                    }
                                    span { class: "font-mono text-[9px] text-slate-400 font-bold", "{item.ratio}%" }
                                }
                            }
                        }
                    })}
                }
            }

            // Twelve Stage
            div { class: "text-center py-2 px-3 rounded-xl bg-slate-900 border border-slate-800 text-xs font-extrabold text-amber-400/90 shadow-inner",
                "⚡ {twelve_stage}"
            }

            // Shinsal list
            if !shinsals.is_empty() {
                div { class: "flex flex-col gap-1 mt-1.5",
                    {shinsals.iter().map(|s| {
                        let is_ausp = s.contains("귀인") || s.contains("록") || s.contains("덕");
                        let bg_cls = if is_ausp {
                            "bg-emerald-950/30 text-emerald-400 border-emerald-800/40"
                        } else {
                            "bg-indigo-950/30 text-indigo-400 border-indigo-800/40"
                        };
                        rsx! {
                            span { class: "text-[10px] py-1 px-2.5 rounded-full border text-center font-bold tracking-tight truncate {bg_cls}",
                                "✦ {s}"
                            }
                        }
                    })}
                }
            }
        }
    }
}

#[component]
fn DeukBadge(label: &'static str, acquired: bool) -> Element {
    let (bg, text) = if acquired {
        ("bg-emerald-900/40 border-emerald-700/50 text-emerald-300", "○")
    } else {
        ("bg-slate-800/40 border-slate-700/50 text-slate-500", "✗")
    };
    rsx! {
        div { class: "flex items-center gap-1.5 px-2 py-1 rounded-lg border {bg}",
            span { class: "font-mono text-xs", "{text}" }
            span { class: "text-xs font-medium", "{label}" }
        }
    }
}

#[component]
fn MetaRow(label: &'static str, value: String) -> Element {
    rsx! {
        div { class: "flex justify-between items-start gap-2",
            span { class: "text-slate-500 shrink-0", "{label}" }
            span { class: "text-slate-300 text-right font-mono text-xs", "{value}" }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
struct JijangganItem {
    stem: HeavenlyStem,
    ratio: u8,
    type_key: TK,
}

#[derive(Clone, PartialEq)]
struct JijangganDisplayItem {
    stem: HeavenlyStem,
    ratio: u8,
    type_key: TK,
    ten_god: TenGod,
    is_projected: bool,
    is_main: bool,
}

#[allow(dead_code)]
fn get_branch_from_hanja(hanja: &str) -> Option<EarthlyBranch> {
    match hanja {
        "子" => Some(EarthlyBranch::Zi),
        "丑" => Some(EarthlyBranch::Chou),
        "寅" => Some(EarthlyBranch::Yin),
        "卯" => Some(EarthlyBranch::Mao),
        "辰" => Some(EarthlyBranch::Chen),
        "巳" => Some(EarthlyBranch::Si),
        "午" => Some(EarthlyBranch::Wu),
        "未" => Some(EarthlyBranch::Wei),
        "申" => Some(EarthlyBranch::Shen),
        "酉" => Some(EarthlyBranch::You),
        "戌" => Some(EarthlyBranch::Xu),
        "亥" => Some(EarthlyBranch::Hai),
        _ => None,
    }
}

fn get_jijanggan_items(branch: EarthlyBranch) -> Vec<JijangganItem> {
    use eon_saju::core::stem::HeavenlyStem::*;
    match branch {
        EarthlyBranch::Zi => vec![
            JijangganItem { stem: Ren, ratio: 33, type_key: TK::SajuJijangganYeogi },
            JijangganItem { stem: Gui, ratio: 67, type_key: TK::SajuJijangganJeonggi },
        ],
        EarthlyBranch::Chou => vec![
            JijangganItem { stem: Gui, ratio: 30, type_key: TK::SajuJijangganYeogi },
            JijangganItem { stem: Xin, ratio: 10, type_key: TK::SajuJijangganJunggi },
            JijangganItem { stem: Ji, ratio: 60, type_key: TK::SajuJijangganJeonggi },
        ],
        EarthlyBranch::Yin => vec![
            JijangganItem { stem: Wu, ratio: 23, type_key: TK::SajuJijangganYeogi },
            JijangganItem { stem: Bing, ratio: 23, type_key: TK::SajuJijangganJunggi },
            JijangganItem { stem: Jia, ratio: 54, type_key: TK::SajuJijangganJeonggi },
        ],
        EarthlyBranch::Mao => vec![
            JijangganItem { stem: Jia, ratio: 33, type_key: TK::SajuJijangganYeogi },
            JijangganItem { stem: Yi, ratio: 67, type_key: TK::SajuJijangganJeonggi },
        ],
        EarthlyBranch::Chen => vec![
            JijangganItem { stem: Yi, ratio: 30, type_key: TK::SajuJijangganYeogi },
            JijangganItem { stem: Gui, ratio: 10, type_key: TK::SajuJijangganJunggi },
            JijangganItem { stem: Wu, ratio: 60, type_key: TK::SajuJijangganJeonggi },
        ],
        EarthlyBranch::Si => vec![
            JijangganItem { stem: Wu, ratio: 23, type_key: TK::SajuJijangganYeogi },
            JijangganItem { stem: Geng, ratio: 23, type_key: TK::SajuJijangganJunggi },
            JijangganItem { stem: Bing, ratio: 54, type_key: TK::SajuJijangganJeonggi },
        ],
        EarthlyBranch::Wu => vec![
            JijangganItem { stem: Bing, ratio: 33, type_key: TK::SajuJijangganYeogi },
            JijangganItem { stem: Ji, ratio: 30, type_key: TK::SajuJijangganJunggi },
            JijangganItem { stem: Ding, ratio: 37, type_key: TK::SajuJijangganJeonggi },
        ],
        EarthlyBranch::Wei => vec![
            JijangganItem { stem: Ding, ratio: 30, type_key: TK::SajuJijangganYeogi },
            JijangganItem { stem: Yi, ratio: 10, type_key: TK::SajuJijangganJunggi },
            JijangganItem { stem: Ji, ratio: 60, type_key: TK::SajuJijangganJeonggi },
        ],
        EarthlyBranch::Shen => vec![
            JijangganItem { stem: Wu, ratio: 23, type_key: TK::SajuJijangganYeogi },
            JijangganItem { stem: Ren, ratio: 23, type_key: TK::SajuJijangganJunggi },
            JijangganItem { stem: Geng, ratio: 54, type_key: TK::SajuJijangganJeonggi },
        ],
        EarthlyBranch::You => vec![
            JijangganItem { stem: Geng, ratio: 33, type_key: TK::SajuJijangganYeogi },
            JijangganItem { stem: Xin, ratio: 67, type_key: TK::SajuJijangganJeonggi },
        ],
        EarthlyBranch::Xu => vec![
            JijangganItem { stem: Xin, ratio: 30, type_key: TK::SajuJijangganYeogi },
            JijangganItem { stem: Ding, ratio: 10, type_key: TK::SajuJijangganJunggi },
            JijangganItem { stem: Wu, ratio: 60, type_key: TK::SajuJijangganJeonggi },
        ],
        EarthlyBranch::Hai => vec![
            JijangganItem { stem: Wu, ratio: 23, type_key: TK::SajuJijangganYeogi },
            JijangganItem { stem: Jia, ratio: 23, type_key: TK::SajuJijangganJunggi },
            JijangganItem { stem: Ren, ratio: 54, type_key: TK::SajuJijangganJeonggi },
        ],
    }
}
