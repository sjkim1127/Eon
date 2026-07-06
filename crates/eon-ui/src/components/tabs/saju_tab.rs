use crate::components::shared::birth_form::BirthForm;
use crate::i18n::{
    format_age, format_age_from, format_age_shift, format_strength_summary, t,
    translate_aux_shinsal, translate_saju_branch, translate_saju_element, translate_saju_ganzi,
    translate_saju_load_balancer, translate_saju_reason, translate_saju_relation_str,
    translate_saju_spirit_marker_name, translate_saju_stem, translate_saju_stem_combination,
    translate_saju_strength_type, translate_saju_structure, translate_saju_structure_desc,
    translate_saju_structure_summary, translate_saju_tag_str, translate_saju_ten_god,
    translate_saju_void_desc, translate_saju_yongshin_type, translate_spirit_desc, Locale, TK,
};
use crate::store::{AnalysisState, TaskStatus};
use dioxus::prelude::*;
use eon_saju::analysis::strength::StrengthType;
use eon_saju::analysis::supplementary_pillars::InterpretationLevel;
use eon_saju::core::branch::EarthlyBranch;
use eon_saju::core::stem::HeavenlyStem;
use eon_saju::core::ten_gods::TenGod;
use eon_service::dto::SajuAnalysisInput;
use eon_service::facade;

#[derive(Clone, Copy, PartialEq, Eq)]
enum SubTab {
    Summary,
    Themes,
    Transit,
    Remedies,
}

#[component]
pub fn SajuTab() -> Element {
    let state = use_context::<AnalysisState>();
    let locale = *state.locale.read();

    // Reactive trigger for manual analysis runs
    let mut analysis_trigger = use_signal(|| 0);

    // Sub-tab state
    let mut active_sub_tab = use_signal(|| SubTab::Summary);

    // Auto-run or manually triggered analysis when form or trigger changes
    let state_cloned = state.clone();
    use_effect(move || {
        let form = state_cloned.form.read().clone();
        let _trig = *analysis_trigger.read();

        if form.year > 0 {
            let mut state = state_cloned.clone();
            spawn(async move {
                state.saju.write().status = TaskStatus::Loading;
                let input = SajuAnalysisInput::new(
                    form.to_analysis_input(),
                    form.is_male,
                    form.use_night_rat_hour,
                    Some(false),
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
        }
    });

    rsx! {
        div { class: "space-y-6 animate-in fade-in slide-in-from-bottom-4 duration-700",
            BirthForm {}

            div { class: "flex justify-between items-center",
                h2 { class: "text-2xl font-bold bg-gradient-to-r from-amber-200 to-orange-400 bg-clip-text text-transparent",
                    "{t(locale, TK::SectionSajuChart)}"
                }
                button {
                    class: "p-2.5 bg-slate-800 hover:bg-slate-700 active:bg-slate-600 border border-slate-700/50 rounded-xl text-slate-300 hover:text-white transition-all cursor-pointer flex items-center justify-center active:scale-95",
                    onclick: move |_| {
                        let current = *analysis_trigger.peek();
                        analysis_trigger.set(current + 1);
                    },
                    title: "{t(locale, TK::BtnCalculate)}",
                    svg {
                        class: "w-5 h-5",
                        fill: "none",
                        stroke: "currentColor",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M4 4v5h.582m15.356 2A8.001 8.001 0 1121.21 7.89M21 3v5h-5"
                        }
                    }
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
                                .cloned()
                                .collect::<Vec<eon_saju::analysis::spirit_markers::SpiritMarkerDetail>>()
                        };
                        let unpacker_info = data.entropy.unpacker_element.map(|unpacker| {
                            let label = match locale {
                                Locale::Ko => format!("{}({})", unpacker.hangul(), unpacker.hanja()),
                                Locale::Zh => unpacker.hanja().to_string(),
                                _ => translate_saju_element(locale, unpacker).to_string(),
                            };
                            (element_style(unpacker.hangul()).0, label)
                        });
                        let bottleneck_info = data.qi_topology.bottleneck.map(|btn| {
                            let label = match locale {
                                Locale::Ko => format!("{}({})", btn.hangul(), btn.hanja()),
                                Locale::Zh => btn.hanja().to_string(),
                                _ => translate_saju_element(locale, btn).to_string(),
                            };
                            (element_style(btn.hangul()).0, label)
                        });
                        let complexity_info = data.complexity.as_ref().map(|comp| {
                            let comp_label = format!("M = {} ({})", comp.cyclomatic_complexity, comp.stability_grade);
                            let entropy_str = format!("{:.2}", comp.entropy);
                            let decision_ages = comp.decision_nodes.iter().map(|&a| format_age(locale, a as i32)).collect::<Vec<_>>().join(", ");
                            (comp_label, entropy_str, decision_ages)
                        });

                        let major_luck_info = data.report.major_luck.as_ref().map(|ml| {
                            let dir_str = match locale {
                                Locale::Ko => if ml.direction == eon_saju::LuckDirection::Forward { t(locale, TK::SajuDirectionForward) } else { t(locale, TK::SajuDirectionReverse) },
                                Locale::En => if ml.direction == eon_saju::LuckDirection::Forward { "Direct" } else { "Reverse" },
                                Locale::Zh => if ml.direction == eon_saju::LuckDirection::Forward { "顺行" } else { "逆行" },
                                Locale::Ru => if ml.direction == eon_saju::LuckDirection::Forward { "Прямо" } else { "Обратно" },
                            };
                            (ml, dir_str)
                        });

                        let crashes_lbl = t(locale, TK::SajuFuzzerCrashes).replace("{}", &data.crash_count.to_string());

                        let day_master = data.report.pillars.day.stem;
                        let stems = [data.report.pillars.hour.stem,
                            data.report.pillars.day.stem,
                            data.report.pillars.month.stem,
                            data.report.pillars.year.stem];

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
                            (data.report.pillars.hour.branch, false, TK::SajuHourBranch, TK::SajuHourPillar),
                            (data.report.pillars.day.branch, false, TK::SajuDayBranch, TK::SajuDayPillar),
                            (data.report.pillars.month.branch, true, TK::SajuMonthBranch, TK::SajuMonthPillar),
                            (data.report.pillars.year.branch, false, TK::SajuYearBranch, TK::SajuYearPillar),
                        ];

                        let target_stems = vec![
                            (data.report.pillars.hour.stem, TK::SajuHourStem, TK::SajuHourPillar),
                            (data.report.pillars.day.stem, TK::SajuDayStem, TK::SajuDayPillar),
                            (data.report.pillars.month.stem, TK::SajuMonthStem, TK::SajuMonthPillar),
                            (data.report.pillars.year.stem, TK::SajuYearStem, TK::SajuYearPillar),
                        ];

                        for &(branch, is_month, b_pos_tk, b_pillar_tk) in &branch_positions {
                            for j_item in get_jijanggan_items(branch) {
                                for &(t_stem, t_pos_tk, _t_pillar_tk) in &target_stems {
                                    if j_item.stem == t_stem {
                                        let ten_god = TenGod::from_stems(day_master, j_item.stem);
                                        projected_instances.push((
                                            b_pos_tk,
                                            branch,
                                            j_item.stem,
                                            j_item.ratio,
                                            j_item.type_key,
                                            ten_god,
                                            t_pos_tk,
                                            is_month,
                                        ));
                                    }
                                }
                            }
                        }

                        let no_proj_msg = match locale {
                            Locale::Ko => {t(locale, TK::SajuNoProjectedJijanggan)},
                            Locale::En => "No hidden stems are projected to the heavenly stems.",
                            Locale::Zh => "无地支藏干透出至天干。",
                            Locale::Ru => "Нет скрытых небесных стволов, проецирующихся на небесные стволы.",
                        };

                        let struct_dt = &data.report.structure;
                        let struct_lbl = match locale {
                            Locale::Ko => format!("{}({})", struct_dt.structure.hangul(), struct_dt.structure.hanja()),
                            Locale::Zh => struct_dt.structure.hanja().to_string(),
                            _ => translate_saju_structure(locale, struct_dt.structure).to_string(),
                        };
                        let struct_decision_lbl = t(locale, TK::SajuStructDecision).replace("{}", &struct_lbl);
                        let proj_stem_lbl = struct_dt.projected_stem.map(|s| match locale {
                            Locale::Ko => format!("{}({})", s.hanja(), s.hangul()),
                            Locale::Zh => s.hanja().to_string(),
                            _ => translate_saju_stem(locale, s).to_string(),
                        });
                        let path_translated = struct_dt.projection_path.as_ref().map(|p| {
                            match locale {
                                Locale::Ko => match p.as_str() {
                                    "시간" | "Hour" | "Hour Stem" | "시주" => "시간",
                                    "일간" | "Day" | "Day Stem" | "일주" => "일간",
                                    "월간" | "Month" | "Month Stem" | "월주" => "월간",
                                    "연간" | "Year" | "Year Stem" | "연주" => "연간",
                                    _ => p.as_str(),
                                },
                                Locale::Zh => match p.as_str() {
                                    "시간" | "Hour" | "Hour Stem" | "시주" => "时干",
                                    "일간" | "Day" | "Day Stem" | "일주" => "日干",
                                    "월간" | "Month" | "Month Stem" | "월주" => "月干",
                                    "연간" | "Year" | "Year Stem" | "연주" => "年干",
                                    _ => p.as_str(),
                                },
                                Locale::En => match p.as_str() {
                                    "시간" | "Hour" | "Hour Stem" | "시주" => "Hour Stem",
                                    "일간" | "Day" | "Day Stem" | "일주" => "Day Stem",
                                    "월간" | "Month" | "Month Stem" | "월주" => "Month Stem",
                                    "연간" | "Year" | "Year Stem" | "연주" => "Year Stem",
                                    _ => p.as_str(),
                                },
                                Locale::Ru => match p.as_str() {
                                    "시간" | "Hour" | "Hour Stem" | "시주" => "Небесный ствол часа",
                                    "일간" | "Day" | "Day Stem" | "일주" => "Небесный ствол дня",
                                    "월간" | "Month" | "Month Stem" | "월주" => "Небесный ствол месяца",
                                    "연간" | "Year" | "Year Stem" | "연주" => "Небесный ствол года",
                                    _ => p.as_str(),
                                },
                            }
                        });

                        rsx! {
                            div { class: "w-full max-w-7xl mx-auto space-y-6",
                                div { class: "flex flex-wrap gap-2 border-b border-slate-800 pb-2 mb-4",
                                    button {
                                        class: if *active_sub_tab.read() == SubTab::Summary { "px-4 py-2 bg-slate-800 text-slate-100 rounded-t font-semibold" } else { "px-4 py-2 text-slate-400 hover:text-slate-200" },
                                        onclick: move |_| { active_sub_tab.set(SubTab::Summary); },
                                        "{t(locale, TK::SajuTabSummary)}"
                                    }
                                    button {
                                        class: if *active_sub_tab.read() == SubTab::Themes { "px-4 py-2 bg-slate-800 text-amber-300 rounded-t font-semibold" } else { "px-4 py-2 text-slate-400 hover:text-slate-200" },
                                        onclick: move |_| { active_sub_tab.set(SubTab::Themes); },
                                        "{t(locale, TK::SajuTabThemes)}"
                                    }
                                    button {
                                        class: if *active_sub_tab.read() == SubTab::Transit { "px-4 py-2 bg-slate-800 text-blue-300 rounded-t font-semibold" } else { "px-4 py-2 text-slate-400 hover:text-slate-200" },
                                        onclick: move |_| { active_sub_tab.set(SubTab::Transit); },
                                        "{t(locale, TK::SajuTabTransit)}"
                                    }
                                    button {
                                        class: if *active_sub_tab.read() == SubTab::Remedies { "px-4 py-2 bg-slate-800 text-rose-300 rounded-t font-semibold" } else { "px-4 py-2 text-slate-400 hover:text-slate-200" },
                                        onclick: move |_| { active_sub_tab.set(SubTab::Remedies); },
                                        "{t(locale, TK::SajuTabRemedies)}"
                                    }
                                }

                                if *active_sub_tab.read() == SubTab::Summary {
                                    div { class: "space-y-8 animate-fade-in",
                            // ── 1. 사주 원국 (천간/지지/십성/12운성/신살) ─────────
                            div { class: "grid grid-cols-4 gap-3.5",
                                PillarCard {
                                    title: t(locale, TK::SajuHourPillar),
                                    stem_god: Some(data.report.ten_gods.hour_stem),
                                    stem: data.report.pillars.hour.stem,
                                    branch_god: data.report.ten_gods.hour_branch,
                                    branch: data.report.pillars.hour.branch,
                                    twelve_stage: crate::i18n::translate_saju_twelve_stage(locale, twelve_stages.hour_stage).to_string(),
                                    nayin: crate::i18n::translate_saju_nayin(locale, data.report.pillars.hour.nayin()),
                                    shinsals: shinsals_for(eon_saju::analysis::spirit_markers::PillarPosition::Hour),
                                    jijanggans: hour_jijanggans
                                }
                                PillarCard {
                                    title: t(locale, TK::SajuDayPillar),
                                    stem_god: None,
                                    stem: data.report.pillars.day.stem,
                                    branch_god: data.report.ten_gods.day_branch,
                                    branch: data.report.pillars.day.branch,
                                    twelve_stage: crate::i18n::translate_saju_twelve_stage(locale, twelve_stages.day_stage).to_string(),
                                    nayin: crate::i18n::translate_saju_nayin(locale, data.report.pillars.day.nayin()),
                                    shinsals: shinsals_for(eon_saju::analysis::spirit_markers::PillarPosition::Day),
                                    jijanggans: day_jijanggans
                                }
                                PillarCard {
                                    title: t(locale, TK::SajuMonthPillar),
                                    stem_god: Some(data.report.ten_gods.month_stem),
                                    stem: data.report.pillars.month.stem,
                                    branch_god: data.report.ten_gods.month_branch,
                                    branch: data.report.pillars.month.branch,
                                    twelve_stage: crate::i18n::translate_saju_twelve_stage(locale, twelve_stages.month_stage).to_string(),
                                    nayin: crate::i18n::translate_saju_nayin(locale, data.report.pillars.month.nayin()),
                                    shinsals: shinsals_for(eon_saju::analysis::spirit_markers::PillarPosition::Month),
                                    jijanggans: month_jijanggans
                                }
                                PillarCard {
                                    title: t(locale, TK::SajuYearPillar),
                                    stem_god: Some(data.report.ten_gods.year_stem),
                                    stem: data.report.pillars.year.stem,
                                    branch_god: data.report.ten_gods.year_branch,
                                    branch: data.report.pillars.year.branch,
                                    twelve_stage: crate::i18n::translate_saju_twelve_stage(locale, twelve_stages.year_stage).to_string(),
                                    nayin: crate::i18n::translate_saju_nayin(locale, data.report.pillars.year.nayin()),
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
                                        {projected_instances.iter().map(|&(b_pos_tk, _branch, stem, ratio, _type_key, ten_god, t_pos_tk, is_main)| {
                                            let (el_color, _el_bg, el_icon) = element_card_style(stem.element().hangul());
                                            let proj_type_lbl = if is_main { t(locale, TK::SajuProjLevelMain) } else { t(locale, TK::SajuProjLevelSub) };
                                            let badge_cls = if is_main {
                                                "bg-amber-500/20 text-amber-300 border-amber-500/30"
                                            } else {
                                                "bg-indigo-500/20 text-indigo-300 border-indigo-500/30"
                                            };
                                            let pos_branch_name = t(locale, b_pos_tk);
                                            let pos_stem_name = t(locale, t_pos_tk);
                                            
                                            let stem_trans = translate_saju_stem(locale, stem);
                                            let ten_god_trans = translate_saju_ten_god(locale, ten_god);
                                            let stage_desc = match locale {
                                                Locale::Ko => format!(
                                                    "{}의 지장간 {}({}) [{}]이 {}로 투출하여 외부로 강하게 발현됩니다. (가중치: {}%)",
                                                    pos_branch_name, stem.hanja(), stem.hangul(), ten_god.hangul(), pos_stem_name, ratio
                                                ),
                                                Locale::En => format!(
                                                    "Hidden stem {}({}) [{}] in {} is projected to {}, manifesting strongly. (Weight: {}%)",
                                                    stem.hanja(), stem_trans, ten_god_trans, pos_branch_name, pos_stem_name, ratio
                                                ),
                                                Locale::Zh => format!(
                                                    "{}藏干{}({})[{}]透出至{}，外部作用力显著增强。（权重：{}%）",
                                                    pos_branch_name, stem.hanja(), stem_trans, ten_god_trans, pos_stem_name, ratio
                                                ),
                                                Locale::Ru => format!(
                                                    "Скрытый ствол {}({}) [{}] из {} проецируется на {}, сильно проявляясь вовне. (Вес: {}%)",
                                                    stem.hanja(), stem_trans, ten_god_trans, pos_branch_name, pos_stem_name, ratio
                                                ),
                                            };
                                            rsx! {
                                                div { class: "p-4 rounded-xl bg-slate-850/40 border border-slate-800/80 hover:border-slate-700 transition-colors flex gap-3 shadow-inner",
                                                    span { class: "text-2xl shrink-0 mt-0.5", "{el_icon}" }
                                                    div { class: "space-y-1.5 flex-1",
                                                        div { class: "flex items-center justify-between flex-wrap gap-2",
                                                            div { class: "flex items-center gap-1.5",
                                                                span { class: "font-serif font-extrabold text-sm {el_color}", "{stem.hanja()}({stem_trans})" }
                                                                span { class: "text-xs font-semibold text-slate-350", "{ten_god_trans}" }
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
                                    h3 { class: "text-sm font-semibold text-slate-400 uppercase tracking-widest",
                                        {match locale {
                                            Locale::Ko => {t(locale, TK::SajuStrengthTitle)},
                                            Locale::Zh => "身强/身弱",
                                            Locale::En => "Strength (Strong/Weak)",
                                            Locale::Ru => "Сила карты (Сильная/Слабая)",
                                        }}
                                    }
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
                                            let st_label = match locale {
                                                Locale::Ko => format!("{} ({})", st.hangul(), st.hanja()),
                                                Locale::Zh => st.hanja().to_string(),
                                                _ => translate_saju_strength_type(locale, st).to_string(),
                                            };
                                            rsx! {
                                                span { class: "text-3xl", "{icon}" }
                                                div {
                                                    div { class: "inline-block px-3 py-1 rounded-full border text-sm font-bold {badge_color}",
                                                        "{st_label}"
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
                                        let primary_el_lbl = match locale {
                                            Locale::Ko => format!("{} ({})", primary_el.hangul(), primary_el.hanja()),
                                            Locale::Zh => primary_el.hanja().to_string(),
                                            _ => translate_saju_element(locale, primary_el).to_string(),
                                        };
                                        let assist_el_lbl = match locale {
                                            Locale::Ko => format!("{} ({})", assist_el.hangul(), assist_el.hanja()),
                                            Locale::Zh => assist_el.hanja().to_string(),
                                            _ => translate_saju_element(locale, assist_el).to_string(),
                                        };
                                        rsx! {
                                            div { class: "space-y-2",
                                                div { class: "flex items-center gap-2",
                                                    span { class: "text-lg", "{p_icon}" }
                                                    div { class: "flex-1",
                                                        p { class: "text-xs text-slate-400", "{t(locale, TK::SajuPrimaryYongShen)}" }
                                                        p { class: "font-bold {p_color}", "{primary_el_lbl}" }
                                                    }
                                                }
                                                div { class: "flex items-center gap-2",
                                                    div { class: "flex-1",
                                                        p { class: "text-xs text-slate-400", "{t(locale, TK::SajuHeeShen)}" }
                                                        p { class: "font-semibold {a_color}", "{assist_el_lbl}" }
                                                    }
                                                }
                                                if !yn.recommendations.is_empty() {
                                                    div { class: "mt-3 pt-3 border-t border-slate-800 space-y-2.5",
                                                        p { class: "text-[10px] font-bold text-slate-500 uppercase tracking-wider", "{t(locale, TK::SajuYongShenDetail)}" }
                                                        div { class: "space-y-2",
                                                            {yn.recommendations.iter().map(|rec| {
                                                                let (el_color, el_icon) = element_style(rec.element.hangul());
                                                                let type_name = translate_saju_yongshin_type(locale, rec.yongshin_type);
                                                                let rec_el_lbl = match locale {
                                                                    Locale::Ko => format!("{}({})", rec.element.hangul(), rec.element.hanja()),
                                                                    Locale::Zh => rec.element.hanja().to_string(),
                                                                    _ => translate_saju_element(locale, rec.element).to_string(),
                                                                };
                                                                let summary_lbl = translate_saju_reason(locale, &rec.summary);
                                                                let desc_lbl = translate_saju_reason(locale, &rec.description);
                                                                rsx! {
                                                                    div { class: "p-2.5 rounded-lg bg-slate-900/50 border border-slate-800/80 space-y-1 text-xs",
                                                                        div { class: "flex justify-between items-center",
                                                                            span { class: "text-[10px] font-bold text-slate-400", "{type_name}" }
                                                                            span { class: "font-bold text-xs {el_color}", "{el_icon} {rec_el_lbl}" }
                                                                        }
                                                                        p { class: "text-xs text-slate-300 font-semibold", "{summary_lbl}" }
                                                                        p { class: "text-[11px] text-slate-400 leading-relaxed", "{desc_lbl}" }
                                                                        if !rec.reasons.is_empty() {
                                                                            div { class: "flex flex-wrap gap-1 mt-1",
                                                                                {rec.reasons.iter().map(|r| rsx! {
                                                                                    span { class: "text-[9px] px-1 bg-slate-950/60 border border-slate-855 rounded text-slate-550", "{translate_saju_reason(locale, r)}" }
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
                                        h3 { class: "text-sm font-semibold text-slate-400 uppercase tracking-widest",
                                            {match locale {
                                                Locale::Ko => {t(locale, TK::SajuFiveElementsPower)},
                                                Locale::Zh => "五行详细强度 (Weighted Five Elements)",
                                                Locale::En => "Weighted Five Elements Analysis",
                                                Locale::Ru => "Взвешенный анализ пяти стихий",
                                            }}
                                        }
                                        span { class: "text-xs font-bold text-amber-400 bg-amber-950/20 px-2.5 py-0.5 rounded border border-amber-900/30",
                                            {match locale {
                                                Locale::Ko => format!("대표: {}", data.report.power.dominant_element.hangul()),
                                                Locale::Zh => format!("代表: {}", data.report.power.dominant_element.hanja()),
                                                _ => format!("Dominant: {}", translate_saju_element(locale, data.report.power.dominant_element)),
                                            }}
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
                                                let label = match locale {
                                                    Locale::Ko => format!("{}({})", el.hangul(), el.hanja()),
                                                    Locale::Zh => el.hanja().to_string(),
                                                    _ => translate_saju_element(locale, el).to_string(),
                                                };
                                                let score_str = match locale {
                                                    Locale::Ko => format!("{:.1}점", score),
                                                    Locale::Zh => format!("{:.1}分", score),
                                                    _ => format!("{:.1} pts", score),
                                                };
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
                                        h3 { class: "text-sm font-semibold text-slate-400 uppercase tracking-widest",
                                            {match locale {
                                                Locale::Ko => {t(locale, TK::SajuTenGodsPower)},
                                                Locale::Zh => "十神强度 (Ten Gods Power)",
                                                Locale::En => "Ten Gods Power Analysis",
                                                Locale::Ru => "Анализ силы Десяти Божеств",
                                            }}
                                        }
                                        span { class: "text-xs font-bold text-indigo-400 bg-indigo-950/20 px-2.5 py-0.5 rounded border border-indigo-900/30",
                                            {match locale {
                                                Locale::Ko => format!("대표: {}", data.report.power.dominant_ten_god.hangul()),
                                                Locale::Zh => format!("代表: {}", data.report.power.dominant_ten_god.hanja()),
                                                _ => format!("Dominant: {}", translate_saju_ten_god(locale, data.report.power.dominant_ten_god)),
                                            }}
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
                                                let label = match locale {
                                                    Locale::Ko => format!("{}({})", tg.hangul(), tg.hanja()),
                                                    Locale::Zh => tg.hanja().to_string(),
                                                    _ => translate_saju_ten_god(locale, tg).to_string(),
                                                };
                                                let score_str = match locale {
                                                    Locale::Ko => format!("{:.1}점", score),
                                                    Locale::Zh => format!("{:.1}分", score),
                                                    _ => format!("{:.1} pts", score),
                                                };
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
                                                    span { class: "text-sm text-slate-300", "{translate_saju_ganzi(locale, &cycle.ganzi)}" }
                                                    div { class: "flex gap-1 flex-wrap justify-center",
                                                        span { class: "text-xs px-1.5 py-0.5 rounded bg-slate-700 text-amber-400", "{translate_saju_ten_god(locale, cycle.stem_god)}" }
                                                        span { class: "text-xs px-1.5 py-0.5 rounded bg-slate-700 text-blue-400", "{translate_saju_ten_god(locale, cycle.branch_god)}" }
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
                                                .map(|s| translate_aux_shinsal(locale, &s.2))
                                                .collect();
                                            rsx! {
                                                div { class: "p-4 rounded-xl bg-slate-800/40 border border-slate-800 flex flex-col gap-2 shadow-inner",
                                                    p { class: "text-xs font-bold text-slate-500", "{label}" }
                                                    div { class: "flex items-center gap-2",
                                                        span { class: "text-3xl font-extrabold font-serif {s_color}", "{gz.stem.hanja()}" }
                                                        span { class: "text-3xl font-extrabold font-serif {b_color}", "{gz.branch.hanja()}" }
                                                        span { class: "text-sm text-slate-300 font-bold", "{translate_saju_ganzi(locale, gz)}" }
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
                                                    translate_saju_branch(locale, data.report.voids.void_branches[0]),
                                                    data.report.voids.void_branches[1].hanja(),
                                                    translate_saju_branch(locale, data.report.voids.void_branches[1])
                                                )}
                                            }
                                        }
                                        if !data.report.voids.void_ten_gods.is_empty() {
                                            p { class: "text-slate-400 font-bold",
                                                "{t(locale, TK::SajuVoidTenGods)}: "
                                                span { class: "text-indigo-400",
                                                    {data.report.voids.void_ten_gods.iter().map(|tg| translate_saju_ten_god(locale, *tg)).collect::<Vec<_>>().join(", ")}
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
                                                                    InterpretationLevel::Danger => "text-red-400 bg-red-950/40 border-red-500 animate-pulse",
                                                };
                                                let label_branch = format!("{}({})", void_dt.branch.hanja(), translate_saju_branch(locale, void_dt.branch));
                                                let label_tg = translate_saju_ten_god(locale, void_dt.ten_god).to_string();
                                                let pos_str = match void_dt.position.to_string().as_str() {
                                                    "연주" | "Year" => t(locale, TK::SajuYearPillar),
                                                    "월주" | "Month" => t(locale, TK::SajuMonthPillar),
                                                    "일주" | "Day" => t(locale, TK::SajuDayPillar),
                                                    "시주" | "Hour" => t(locale, TK::SajuHourPillar),
                                                    _ => &void_dt.position.to_string(),
                                                };
                                                let label_pos = format!("{} {}", pos_str, t(locale, TK::SajuVoidSuffix));
                                                let void_summary = match locale {
                                                    Locale::Ko => format!("{}에 위치한 {} 공망", pos_str, label_tg),
                                                    Locale::Zh => format!("位于{}的{}空亡", pos_str, label_tg),
                                                    Locale::En => format!("{} Void located in {}", label_tg, pos_str),
                                                    Locale::Ru => format!("Пустота {} в {}", label_tg, pos_str),
                                                };
                                                let trans_desc = translate_saju_void_desc(locale, &void_dt.position.to_string());
                                                let void_desc = if trans_desc.is_empty() { &void_dt.description } else { trans_desc };
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
                                                                        InterpretationLevel::Danger => t(locale, TK::SajuLevelDanger),
                                                                    }}
                                                                }
                                                            }
                                                            span { class: "text-[10px] text-slate-500 font-mono", "{label_pos}" }
                                                        }
                                                        p { class: "text-slate-300 font-semibold", "{void_summary}" }
                                                        p { class: "text-slate-400 leading-relaxed", "{void_desc}" }
                                                    }
                                                }
                                            })}
                                        }
                                    }
                                }
                            }

                            // ── 5.1.2 격국 분석 (Structure / Pattern) ─────────────
                            {
                                let struct_type = &data.report.structure.structure;
                                let struct_type_str = format!("{:?}", struct_type);
                                let is_golden = struct_type_str.contains("Jong") || struct_type_str == "HwaGi" || struct_type_str == "SpecialTransformation";
                                let is_crimson = struct_type_str == "YangIn" || struct_type_str == "JianLu";

                                let container_class = if is_golden {
                                    "bg-gradient-to-r from-amber-950/40 to-yellow-900/20 border border-amber-500/50 rounded-2xl p-5 space-y-4 shadow-[0_0_15px_rgba(245,158,11,0.2)]"
                                } else if is_crimson {
                                    "bg-gradient-to-r from-red-950/40 to-rose-900/20 border border-red-500/50 rounded-2xl p-5 space-y-4 shadow-[0_0_15px_rgba(225,29,72,0.2)]"
                                } else {
                                    "bg-slate-900 border border-slate-800 rounded-2xl p-5 space-y-4 shadow-xl"
                                };

                                let title_class = if is_golden { "text-sm font-semibold text-amber-200 uppercase tracking-widest" }
                                                  else if is_crimson { "text-sm font-semibold text-rose-200 uppercase tracking-widest" }
                                                  else { "text-sm font-semibold text-slate-200 uppercase tracking-widest" };

                                let badge_class = if is_golden { "text-xs font-bold text-amber-900 bg-amber-400 px-3 py-1 rounded border border-amber-500/80 shadow-[0_0_8px_rgba(251,191,36,0.5)]" }
                                                  else if is_crimson { "text-xs font-bold text-rose-900 bg-rose-400 px-3 py-1 rounded border border-rose-500/80 shadow-[0_0_8px_rgba(2fb,113,133,0.5)]" }
                                                  else { "text-xs font-bold text-amber-400 bg-amber-950/20 px-3 py-1 rounded border border-amber-900/30" };

                                rsx! {
                                    div { class: "{container_class}",
                                        div { class: "flex items-center justify-between border-b border-slate-800/60 pb-3 flex-wrap gap-2",
                                            div { class: "space-y-0.5",
                                                h3 { class: "{title_class}", "{t(locale, TK::SajuStructTitle)}" }
                                                p { class: "text-xs text-slate-500", "{t(locale, TK::SajuStructDesc)}" }
                                            }
                                            span { class: "{badge_class}",
                                                "{struct_decision_lbl}"
                                            }
                                        }
                                div { class: "p-4 rounded-xl bg-slate-800/40 border border-slate-800 space-y-3.5 shadow-inner",
                                    if let Some(stem_lbl) = &proj_stem_lbl {
                                        div { class: "flex items-center gap-4 text-xs font-mono text-slate-400 flex-wrap",
                                            div { "{t(locale, TK::SajuProjectedStem)}: "
                                                span { class: "text-indigo-400 font-bold text-sm ml-1", "{stem_lbl}" }
                                            }
                                            if let Some(path) = &path_translated {
                                                div { "{t(locale, TK::SajuProjectionPath)}: "
                                                    span { class: "text-indigo-400 font-bold text-sm ml-1", "{path}" }
                                                }
                                            }
                                        }
                                    }
                                    div { class: "space-y-1.5",
                                        p { class: "text-sm text-slate-200 font-bold", "{translate_saju_structure_summary(locale, data.report.structure.structure, data.report.structure.projection_path.as_deref())}" }
                                        p { class: "text-xs text-slate-400 leading-relaxed", "{translate_saju_structure_desc(locale, data.report.structure.structure, data.report.structure.projected_stem.is_some())}" }
                                    }
                                    if !data.report.structure.reasons.is_empty() {
                                        div { class: "border-t border-slate-800/80 pt-3 space-y-2",
                                            p { class: "text-[10px] font-bold text-slate-500 uppercase tracking-wider", "{t(locale, TK::SajuStructReasons)}" }
                                            div { class: "flex flex-wrap gap-2",
                                                {
                                                    let reason_class = if is_golden {
                                                        "px-2 py-0.5 bg-amber-950/20 border border-amber-500/30 text-[10px] text-amber-300 rounded-md shadow-sm"
                                                    } else if is_crimson {
                                                        "px-2 py-0.5 bg-rose-950/20 border border-rose-500/30 text-[10px] text-rose-300 rounded-md shadow-sm"
                                                    } else {
                                                        "px-2 py-0.5 bg-slate-900 border border-slate-800/80 text-[10px] text-slate-400 rounded-md"
                                                    };

                                                    data.report.structure.reasons.iter().map(move |reason| rsx! {
                                                        span { class: "{reason_class}", "{translate_saju_reason(locale, reason)}" }
                                                    })
                                                }
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
                                    h3 { class: "text-sm font-semibold text-slate-400 uppercase tracking-widest",
                                        {match locale {
                                            Locale::Ko => {t(locale, TK::SajuHarmonyClashTitle)},
                                            Locale::Zh => "合冲刑害分析 (Harmony & Clashes)",
                                            Locale::En => "Harmony & Clashes Analysis",
                                            Locale::Ru => "Анализ отношений (Harmony & Clashes)",
                                        }}
                                    }
                                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-3.5",
                                        {data.report.relationships.mapped_relationships.iter().map(|rel| {
                                            let lvl_cls = match rel.level {
                                                eon_saju::analysis::supplementary_pillars::InterpretationLevel::Auspicious => "text-emerald-400 bg-emerald-950/30 border-emerald-800/40",
                                                eon_saju::analysis::supplementary_pillars::InterpretationLevel::Caution => "text-rose-400 bg-rose-950/30 border-rose-800/40",
                                                eon_saju::analysis::supplementary_pillars::InterpretationLevel::Neutral => "text-slate-400 bg-slate-850 border-slate-800",
                                                InterpretationLevel::Danger => "text-red-400 bg-red-950/40 border-red-500 animate-pulse",
                                            };
                                            let pos_str = rel.positions.iter().map(|p| translate_saju_tag_str(locale, p)).collect::<Vec<_>>().join("-");
                                            let rel_name = translate_saju_relation_str(locale, &rel.name);
                                            let trans_label = rel.transformed_element.map(|el| match locale {
                                                Locale::Ko => format!("{}({})", el.hangul(), el.hanja()),
                                                Locale::Zh => el.hanja().to_string(),
                                                _ => translate_saju_element(locale, el).to_string(),
                                            });
                                            rsx! {
                                                div { class: "p-4 rounded-xl bg-slate-800/20 border border-slate-800/60 space-y-2 hover:border-slate-700 transition-colors",
                                                    div { class: "flex items-center justify-between gap-2 flex-wrap",
                                                        div { class: "flex items-center gap-2",
                                                            span { class: "text-base font-bold text-slate-200", "{rel_name}" }
                                                            span { class: "text-xs px-2 py-0.5 rounded border font-bold {lvl_cls}",
                                                                {match rel.level {
                                                                    eon_saju::analysis::supplementary_pillars::InterpretationLevel::Auspicious => match locale {
                                                                        Locale::Ko => "조화(吉)",
                                                                        Locale::Zh => "和谐(吉)",
                                                                        Locale::En => "Harmony (Auspicious)",
                                                                        Locale::Ru => "Гармония (Благоприятно)",
                                                                    },
                                                                    eon_saju::analysis::supplementary_pillars::InterpretationLevel::Caution => match locale {
                                                                        Locale::Ko => "대립(凶)",
                                                                        Locale::Zh => "冲突(凶)",
                                                                        Locale::En => "Clash (Caution)",
                                                                        Locale::Ru => "Конфликт (Предупреждение)",
                                                                    },
                                                                    eon_saju::analysis::supplementary_pillars::InterpretationLevel::Neutral => match locale {
                                                                        Locale::Ko => "작용",
                                                                        Locale::Zh => "作用",
                                                                        Locale::En => "Influence",
                                                                        Locale::Ru => "Влияние",
                                                                    },
                                                                    eon_saju::analysis::supplementary_pillars::InterpretationLevel::Danger => match locale {
                                                                        Locale::Ko => "파탈(發動)",
                                                                        Locale::Zh => "破局(发动)",
                                                                        Locale::En => "Danger (Triggered)",
                                                                        Locale::Ru => "Опасность (Срабатывание)",
                                                                    },
                                                                }}
                                                             }
                                                        }
                                                        span { class: "text-[10px] font-mono font-bold text-indigo-400/80 bg-indigo-950/20 px-2 py-0.5 rounded border border-indigo-900/30",
                                                            "{pos_str}"
                                                        }
                                                    }
                                                    p { class: "text-sm text-slate-300 font-semibold", "{translate_saju_tag_str(locale, &rel.summary)}" }
                                                    p { class: "text-xs text-slate-400 leading-relaxed", "{translate_saju_tag_str(locale, &rel.description)}" }
                                                    if let Some(trans_el_str) = trans_label {
                                                        div { class: "pt-1 flex items-center gap-1 text-[10px]",
                                                            span { class: "text-slate-500 font-bold", "{t(locale, TK::SajuLabelTransformedElement)}:" }
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
                                            h3 { class: "text-sm font-semibold text-slate-200 uppercase tracking-widest", "{t(locale, TK::SajuHiddenHarmonyTitle)}" }
                                            p { class: "text-xs text-slate-500", "{t(locale, TK::SajuHiddenHarmonyDesc)}" }
                                        }
                                    }
                                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-5",
                                        // 암합 (지장간끼리의 비밀스런 합)
                                        if !data.report.relationships.am_combinations.is_empty() {
                                            div { class: "space-y-3",
                                                h4 { class: "text-xs font-bold text-amber-400/90 tracking-wider flex items-center gap-1.5",
                                                    span { "🔒" }
                                                    span { "{t(locale, TK::SajuAmHarmonyTitle)}" }
                                                }
                                                div { class: "space-y-2",
                                                    {data.report.relationships.am_combinations.iter().map(|(am, p1, p2)| {
                                                        let b1_lbl = format!("{}({})", am.branches.0.hanja(), translate_saju_branch(locale, am.branches.0));
                                                        let b2_lbl = format!("{}({})", am.branches.1.hanja(), translate_saju_branch(locale, am.branches.1));
                                                        let comb_lbl = format!("{}({})", am.combination.hanja(), translate_saju_stem_combination(locale, am.combination));
                                                        let trans_el = am.combination.transformed_element();
                                                        let trans_lbl = match locale {
                                                            Locale::Ko => format!("{}({})", trans_el.hangul(), trans_el.hanja()),
                                                            Locale::Zh => trans_el.hanja().to_string(),
                                                            _ => translate_saju_element(locale, trans_el).to_string(),
                                                        };
                                                        let p1_translated = match p1.as_str() {
                                                            "연주" | "Year" => t(locale, TK::SajuYearPillar),
                                                            "월주" | "Month" => t(locale, TK::SajuMonthPillar),
                                                            "일주" | "Day" => t(locale, TK::SajuDayPillar),
                                                            "시주" | "Hour" => t(locale, TK::SajuHourPillar),
                                                            _ => p1.as_str(),
                                                        };
                                                        let p2_translated = match p2.as_str() {
                                                            "연주" | "Year" => t(locale, TK::SajuYearPillar),
                                                            "월주" | "Month" => t(locale, TK::SajuMonthPillar),
                                                            "일주" | "Day" => t(locale, TK::SajuDayPillar),
                                                            "시주" | "Hour" => t(locale, TK::SajuHourPillar),
                                                            _ => p2.as_str(),
                                                        };
                                                        rsx! {
                                                            div { class: "p-3 rounded-xl bg-slate-800/20 border border-slate-800 space-y-1.5 hover:border-slate-750 transition-colors",
                                                                div { class: "flex items-center justify-between",
                                                                    span { class: "text-sm font-bold text-slate-300", "{b1_lbl} ⇄ {b2_lbl}" }
                                                                    span { class: "text-[10px] font-mono font-bold text-indigo-400 bg-indigo-950/20 px-2 py-0.5 rounded border border-indigo-900/30", "{p1_translated} ⇄ {p2_translated}" }
                                                                }
                                                                div { class: "flex flex-wrap gap-x-3 gap-y-1 items-center text-xs",
                                                                    div { class: "flex items-center gap-1",
                                                                        span { class: "text-slate-500", "{t(locale, TK::SajuLabelHarmony)}:" }
                                                                        span { class: "text-amber-400 font-bold", "{comb_lbl}" }
                                                                    }
                                                                    div { class: "flex items-center gap-1",
                                                                        span { class: "text-slate-500", "{t(locale, TK::SajuLabelTransformedElement)}:" }
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
                                                    span { "{t(locale, TK::SajuMyungAmHarmonyTitle)}" }
                                                }
                                                div { class: "space-y-2",
                                                    {data.report.relationships.myung_am_combinations.iter().map(|(ma, p1, p2)| {
                                                        let stem_lbl = format!("{}({})", ma.stem.hanja(), translate_saju_stem(locale, ma.stem));
                                                        let branch_lbl = format!("{}({})", ma.branch.hanja(), translate_saju_branch(locale, ma.branch));
                                                        let comb_lbl = format!("{}({})", ma.combination.hanja(), translate_saju_stem_combination(locale, ma.combination));
                                                        let trans_el = ma.combination.transformed_element();
                                                        let trans_lbl = match locale {
                                                            Locale::Ko => format!("{}({})", trans_el.hangul(), trans_el.hanja()),
                                                            Locale::Zh => trans_el.hanja().to_string(),
                                                            _ => translate_saju_element(locale, trans_el).to_string(),
                                                        };
                                                        let p1_translated = match p1.as_str() {
                                                            "연주" | "Year" => t(locale, TK::SajuYearPillar),
                                                            "월주" | "Month" => t(locale, TK::SajuMonthPillar),
                                                            "일주" | "Day" => t(locale, TK::SajuDayPillar),
                                                            "시주" | "Hour" => t(locale, TK::SajuHourPillar),
                                                            _ => p1.as_str(),
                                                        };
                                                        let p2_translated = match p2.as_str() {
                                                            "연주" | "Year" => t(locale, TK::SajuYearPillar),
                                                            "월주" | "Month" => t(locale, TK::SajuMonthPillar),
                                                            "일주" | "Day" => t(locale, TK::SajuDayPillar),
                                                            "시주" | "Hour" => t(locale, TK::SajuHourPillar),
                                                            _ => p2.as_str(),
                                                        };
                                                        rsx! {
                                                            div { class: "p-3 rounded-xl bg-slate-800/20 border border-slate-800 space-y-1.5 hover:border-slate-750 transition-colors",
                                                                div { class: "flex items-center justify-between",
                                                                    span { class: "text-sm font-bold text-slate-350", "{stem_lbl} ({t(locale, TK::SajuPillarStem)}) ⇄ {branch_lbl} ({t(locale, TK::SajuPillarBranch)})" }
                                                                    span { class: "text-[10px] font-mono font-bold text-indigo-400 bg-indigo-950/20 px-2 py-0.5 rounded border border-indigo-900/30", "{p1_translated} ⇄ {p2_translated}" }
                                                                }
                                                                div { class: "flex flex-wrap gap-x-3 gap-y-1 items-center text-xs",
                                                                    div { class: "flex items-center gap-1",
                                                                        span { class: "text-slate-500", "{t(locale, TK::SajuLabelHarmony)}:" }
                                                                        span { class: "text-amber-400 font-bold", "{comb_lbl}" }
                                                                    }
                                                                    div { class: "flex items-center gap-1",
                                                                        span { class: "text-slate-500", "{t(locale, TK::SajuLabelTransformedElement)}:" }
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
                                h3 { class: "text-sm font-semibold text-slate-400 uppercase tracking-widest",
                                    {match locale {
                                        Locale::Ko => {t(locale, TK::SajuSystemEngTitle)},
                                        Locale::Zh => "系统工程诊断 (System Engineering & Topology)",
                                        Locale::En => "System Engineering Diagnostics & Topology",
                                        Locale::Ru => "Системная инженерная диагностика и топология",
                                    }}
                                }
                                div { class: "grid grid-cols-1 md:grid-cols-3 gap-4.5",
                                    // 1) Entropy & Obfuscation
                                    div { class: "p-4 rounded-xl bg-slate-850/50 border border-slate-800 space-y-3 flex flex-col justify-between shadow-inner",
                                        div { class: "space-y-1.5",
                                            p { class: "text-xs font-bold text-slate-500 uppercase tracking-wider",
                                                {match locale {
                                                    Locale::Ko => "운명 난독화 및 엔트로피",
                                                    Locale::Zh => "命运混淆与熵",
                                                    Locale::En => "Destiny Obfuscation & Entropy",
                                                    Locale::Ru => "Обфускация судьбы и энтропия",
                                                }}
                                            }
                                            p { class: "text-lg font-bold text-indigo-300",
                                                {match &data.entropy.level {
                                                    eon_saju::engine::entropy::ObfuscationLevel::Plaintext => match locale {
                                                        Locale::Ko => "단순형 (Plaintext)",
                                                        Locale::Zh => "简单型 (Plaintext)",
                                                        Locale::En => "Plaintext (Plain)",
                                                        Locale::Ru => "Простая форма (Plaintext)",
                                                    },
                                                    eon_saju::engine::entropy::ObfuscationLevel::Standard => match locale {
                                                        Locale::Ko => "보통형 (Standard)",
                                                        Locale::Zh => "标准型 (Standard)",
                                                        Locale::En => "Standard (Normal)",
                                                        Locale::Ru => "Стандартная форма (Standard)",
                                                    },
                                                    eon_saju::engine::entropy::ObfuscationLevel::Packed => match locale {
                                                        Locale::Ko => "복합형 (Packed)",
                                                        Locale::Zh => "复合型 (Packed)",
                                                        Locale::En => "Packed (Complex)",
                                                        Locale::Ru => "Сложная форма (Packed)",
                                                    },
                                                    eon_saju::engine::entropy::ObfuscationLevel::Encrypted => match locale {
                                                        Locale::Ko => "복잡형 (Encrypted)",
                                                        Locale::Zh => "加密型 (Encrypted)",
                                                        Locale::En => "Encrypted (Secure)",
                                                        Locale::Ru => "Зашифрованная форма (Encrypted)",
                                                    },
                                                }}
                                            }
                                            p { class: "text-xs text-slate-400 leading-relaxed", "{translate_saju_tag_str(locale, &data.entropy.description)}" }
                                        }
                                        div { class: "border-t border-slate-800/80 pt-2 flex flex-col gap-1.5 text-xs text-slate-500",
                                            p {
                                                "Shannon Entropy: "
                                                span { class: "font-mono font-bold text-slate-300", {format!("{:.3} / 2.322", data.entropy.score)} }
                                            }
                                            if let Some((style, label)) = &unpacker_info {
                                                p {
                                                    {match locale {
                                                        Locale::Ko => "디컴파일 오행 (Unpacker): ",
                                                        Locale::Zh => "反编译五行 (Unpacker): ",
                                                        Locale::En => "Decompiled Element (Unpacker): ",
                                                        Locale::Ru => "Распакованный элемент (Unpacker): ",
                                                    }}
                                                    span { class: "font-bold {style}", "{label}" }
                                                }
                                            }
                                        }
                                    }

                                    // 2) Qi Network Flow Topology
                                    div { class: "p-4 rounded-xl bg-slate-850/50 border border-slate-800 space-y-3 flex flex-col justify-between shadow-inner",
                                        div { class: "space-y-1.5",
                                            p { class: "text-xs font-bold text-slate-500 uppercase tracking-wider",
                                                {match locale {
                                                    Locale::Ko => "에너지 네트워크 토폴로지",
                                                    Locale::Zh => "能量网络拓扑",
                                                    Locale::En => "Energy Network Flow Topology",
                                                    Locale::Ru => "Топология энергетической сети",
                                                }}
                                            }
                                            p { class: "text-lg font-bold text-emerald-400",
                                                {match locale {
                                                    Locale::Ko => format!("유동 효율: {:.1}%", data.qi_topology.throughput * 100.0),
                                                    Locale::Zh => format!("流通效率: {:.1}%", data.qi_topology.throughput * 100.0),
                                                    Locale::En => format!("Flow Efficiency: {:.1}%", data.qi_topology.throughput * 100.0),
                                                    Locale::Ru => format!("Эффективность потока: {:.1}%", data.qi_topology.throughput * 100.0),
                                                }}
                                            }
                                            div { class: "space-y-1 mt-1 text-[11px] font-mono",
                                                {data.qi_topology.nodes.iter().map(|node| {
                                                    let (txt_color, _) = element_style(node.element.hangul());
                                                    let capacity_pct = if node.capacity > 0.0 { (node.output / node.capacity * 100.0).min(100.0) } else { 0.0 };
                                                    let node_el_hangul = translate_saju_element(locale, node.element);
                                                    let node_label = match locale {
                                                        Locale::Ko => format!("대역폭: {:.0} | 출력: {:.1} ({:.0}%)", node.capacity, node.output, capacity_pct),
                                                        Locale::Zh => format!("带宽: {:.0} | 输出: {:.1} ({:.0}%)", node.capacity, node.output, capacity_pct),
                                                        Locale::En => format!("Bandwidth: {:.0} | Output: {:.1} ({:.0}%)", node.capacity, node.output, capacity_pct),
                                                        Locale::Ru => format!("Пропускная способность: {:.0} | Выход: {:.1} ({:.0}%)", node.capacity, node.output, capacity_pct),
                                                    };
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
                                                    {match locale {
                                                        Locale::Ko => "흐름 정체 구간 (Bottleneck): ",
                                                        Locale::Zh => "流量瓶颈区间 (Bottleneck): ",
                                                        Locale::En => "Flow Bottleneck: ",
                                                        Locale::Ru => "Узкое место потока (Bottleneck): ",
                                                    }}
                                                    span { class: "font-bold {style}", "{label}" }
                                                }
                                            } else {
                                                p {
                                                    {match locale {
                                                        Locale::Ko => "흐름 정체 구간 (Bottleneck): 없음",
                                                        Locale::Zh => "流量瓶颈区间 (Bottleneck): 无",
                                                        Locale::En => "Flow Bottleneck: None",
                                                        Locale::Ru => "Узкое место потока (Bottleneck): Нет",
                                                    }}
                                                }
                                            }
                                        }
                                    }

                                    // 3) Cyclomatic Complexity (VM execution)
                                    div { class: "p-4 rounded-xl bg-slate-850/50 border border-slate-800 space-y-3 flex flex-col justify-between shadow-inner",
                                        div { class: "space-y-1.5",
                                            p { class: "text-xs font-bold text-slate-500 uppercase tracking-wider",
                                                {match locale {
                                                    Locale::Ko => "가상머신(VM) 순환 복잡도",
                                                    Locale::Zh => "虚拟机 (VM) 循环复杂度",
                                                    Locale::En => "Virtual Machine (VM) Cyclomatic Complexity",
                                                    Locale::Ru => "Цикломатическая сложность виртуальной машины (VM)",
                                                }}
                                            }
                                            if let Some((comp_label, _, _)) = &complexity_info {
                                                p { class: "text-lg font-bold text-amber-400", "{comp_label}" }
                                                p { class: "text-xs text-slate-400 leading-relaxed",
                                                    {match locale {
                                                        Locale::Ko => "인생 시뮬레이션 상 감지된 주요 결정 분기점(Decision Points) 개수와 시스템 안정성 등급입니다.",
                                                        Locale::Zh => "生命模拟中检测到的主要决策分支（Decision Points）数量与系统稳定性等级。",
                                                        Locale::En => "The number of major decision points detected during life simulation and the system stability rating.",
                                                        Locale::Ru => "Количество основных точек принятия решений, обнаруженных при симуляции жизни, и рейтинг стабильности системы.",
                                                    }}
                                                }
                                            } else {
                                                p { class: "text-slate-500 text-xs",
                                                    {match locale {
                                                        Locale::Ko => "VM 시뮬레이션 복잡도 분석 결과 없음",
                                                        Locale::Zh => "无虚拟机模拟复杂度分析结果",
                                                        Locale::En => "No VM simulation complexity analysis result",
                                                        Locale::Ru => "Результаты анализа сложности симуляции VM отсутствуют",
                                                    }}
                                                }
                                            }
                                        }
                                        div { class: "border-t border-slate-800/80 pt-2 flex flex-col gap-1 text-xs text-slate-500",
                                            if let Some((_, entropy_str, decision_ages)) = &complexity_info {
                                                p {
                                                    {match locale {
                                                        Locale::Ko => "유지보수 엔트로피: ",
                                                        Locale::Zh => "维护熵值: ",
                                                        Locale::En => "Maintenance Entropy: ",
                                                        Locale::Ru => "Энтропия обслуживания: ",
                                                    }}
                                                    span { class: "font-mono font-bold text-slate-300", "{entropy_str}" }
                                                }
                                                if !decision_ages.is_empty() {
                                                    p { class: "truncate",
                                                        {match locale {
                                                            Locale::Ko => "주요 분기점 연령: ",
                                                            Locale::Zh => "主要决策点年龄: ",
                                                            Locale::En => "Decision Point Ages: ",
                                                            Locale::Ru => "Возраст точек принятия решений: ",
                                                        }}
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
                                                let vuln_type = translate_saju_tag_str(locale, &vuln.vulnerability_type);
                                                let major_gz = match locale {
                                                    Locale::Ko => format!("{}({})", vuln.vector.major.hanja(), vuln.vector.major.hangul()),
                                                    Locale::Zh => vuln.vector.major.hanja().to_string(),
                                                    _ => translate_saju_ganzi(locale, &vuln.vector.major).to_string(),
                                                };
                                                let yearly_gz = match locale {
                                                    Locale::Ko => format!("{}({})", vuln.vector.yearly.hanja(), vuln.vector.yearly.hangul()),
                                                    Locale::Zh => vuln.vector.yearly.hanja().to_string(),
                                                    _ => translate_saju_ganzi(locale, &vuln.vector.yearly).to_string(),
                                                };
                                                let vector_desc = t(locale, TK::SajuFuzzerVector).replacen("{}", &major_gz, 1).replacen("{}", &yearly_gz, 1);
                                                let tags_translated: Vec<String> = vuln.tags.iter().map(|tag| translate_saju_tag_str(locale, tag).to_string()).collect();
                                                let tags_list = tags_translated.join(", ");
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
                                                let (reason_desc, strategy_desc) = translate_saju_load_balancer(locale, &diag.reason, &diag.strategy);
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
                                                InterpretationLevel::Danger => "text-red-400 bg-red-950/40 border-red-500 animate-pulse shadow-[0_0_10px_rgba(239,68,68,0.3)]",
                                                InterpretationLevel::Neutral => "text-slate-400 bg-slate-850 border-slate-800",
                                            };
                                            let pos_name = match m.position {
                                                eon_saju::analysis::spirit_markers::PillarPosition::Year => t(locale, TK::SajuYearPillar),
                                                eon_saju::analysis::spirit_markers::PillarPosition::Month => t(locale, TK::SajuMonthPillar),
                                                eon_saju::analysis::spirit_markers::PillarPosition::Day => t(locale, TK::SajuDayPillar),
                                                eon_saju::analysis::spirit_markers::PillarPosition::Hour => t(locale, TK::SajuHourPillar),
                                            };
                                            let part = if m.is_stem { t(locale, TK::SajuPillarStem) } else { t(locale, TK::SajuPillarBranch) };

                                            let clash_title_cls = if m.is_clashed { "line-through opacity-60 decoration-red-500/80 decoration-2" } else { "" };
                                            let combine_title_cls = if m.is_combined { "drop-shadow-[0_0_8px_rgba(96,165,250,0.8)]" } else { "" };

                                            let marker_label = match locale {
                                                Locale::Ko => format!("{} ({})", m.marker.hangul(), m.marker.hanja()),
                                                Locale::Zh => m.marker.hanja().to_string(),
                                                _ => translate_saju_spirit_marker_name(locale, m.marker).to_string(),
                                            };
                                            let marker_summary = translate_saju_spirit_marker_name(locale, m.marker); // We might want to use m.summary for KO to show "(파극됨)" etc, but keeping translation logic for now

                                            // Handle the localized summary manually to inject the "(파극됨)" or "(발동됨)" status from m.summary if it's Korean
                                            let localized_summary = if locale == Locale::Ko { m.summary.clone() } else { marker_summary.to_string() };

                                            let marker_desc = translate_spirit_desc(locale, m.marker, m.position, &m.description);
                                            let pos_part = format!("{} {}", pos_name, part);

                                            let box_bg_cls = if m.is_clashed { "bg-red-950/10 border-red-900/30" } else if m.is_combined { "bg-blue-950/10 border-blue-900/30" } else { "bg-slate-800/20 border-slate-800/60" };

                                            rsx! {
                                                div { class: "p-4 rounded-xl space-y-2 hover:border-slate-700 transition-colors border {box_bg_cls}",
                                                    div { class: "flex items-center justify-between gap-2 flex-wrap",
                                                        div { class: "flex items-center gap-2",
                                                            span { class: "text-base font-bold text-slate-200 {clash_title_cls} {combine_title_cls}", "{marker_label}" }
                                                            span { class: "text-xs px-2.5 py-0.5 rounded border font-bold {lvl_cls}",
                                                                {match m.level {
                                                                    InterpretationLevel::Auspicious => t(locale, TK::SajuLevelAuspicious),
                                                                    InterpretationLevel::Caution => t(locale, TK::SajuLevelCaution),
                                                                    InterpretationLevel::Danger => t(locale, TK::SajuLevelDanger),
                                                                    InterpretationLevel::Neutral => t(locale, TK::LabelNeutral),
                                                                }}
                                                            }
                                                        }
                                                        span { class: "text-[10px] font-bold text-amber-400/80 bg-amber-950/20 px-2.5 py-0.5 rounded border border-amber-900/30 font-mono",
                                                            "{pos_part}"
                                                        }
                                                    }
                                                    p { class: "text-sm text-slate-300 font-semibold {clash_title_cls} {combine_title_cls}", "{localized_summary}" }
                                                    p { class: "text-xs text-slate-400 leading-relaxed", "{marker_desc}" }
                                                    if !m.reasons.is_empty() {
                                                        div { class: "flex items-center gap-1.5 flex-wrap pt-1",
                                                            span { class: "text-[10px] text-slate-500 font-bold", "{t(locale, TK::SajuSpiritRequirement)}" }
                                                            {m.reasons.iter().map(|r| rsx! {
                                                                span { class: "text-[10px] px-2 py-0.5 bg-slate-800/80 border border-slate-700/40 text-slate-400 rounded-md font-mono", "{translate_saju_reason(locale, r)}" }
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
                        // =================== Summary End ===================
                        }

                        if *active_sub_tab.read() == SubTab::Themes {
                            if let Some(themes) = &data.report.themes {
                                div { class: "w-full space-y-6 animate-fade-in",
                                    h2 { class: "text-xl font-bold text-slate-100", "{t(locale, TK::SajuTabThemes)}" }
                                    // Career
                                    div { class: "p-5 rounded-2xl bg-slate-900 border border-slate-800 space-y-3",
                                        h3 { class: "text-lg font-bold text-amber-400", "{t(locale, TK::SajuThemeCareer)}" }
                                        p { class: "text-slate-300", "{themes.career.summary}" }
                                        p { class: "text-sm text-slate-400 mt-2", "{themes.career.recommendation}" }
                                    }
                                    // Wealth
                                    div { class: "p-5 rounded-2xl bg-slate-900 border border-slate-800 space-y-3",
                                        h3 { class: "text-lg font-bold text-emerald-400", "{t(locale, TK::SajuThemeWealth)}" }
                                        p { class: "text-slate-300", "{themes.wealth.summary}" }
                                        p { class: "text-sm text-slate-400 mt-2", "{themes.wealth.flow}" }
                                    }
                                    // Romance
                                    div { class: "p-5 rounded-2xl bg-slate-900 border border-slate-800 space-y-3",
                                        h3 { class: "text-lg font-bold text-pink-400", "{t(locale, TK::SajuThemeRomance)}" }
                                        p { class: "text-slate-300", "{themes.romance.summary}" }
                                        p { class: "text-sm text-slate-400 mt-2", "{themes.romance.advice}" }
                                    }
                                    // Health
                                    div { class: "p-5 rounded-2xl bg-slate-900 border border-slate-800 space-y-3",
                                        h3 { class: "text-lg font-bold text-blue-400", "{t(locale, TK::SajuThemeHealth)}" }
                                        p { class: "text-slate-300", "{themes.health.summary}" }
                                        div { class: "flex gap-2 text-xs",
                                            for organ in &themes.health.vulnerable_organs {
                                                span { class: "px-2 py-1 rounded bg-blue-950/40 text-blue-300 border border-blue-900/50", "{organ}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        if *active_sub_tab.read() == SubTab::Transit {
                            div { class: "w-full space-y-6 animate-fade-in",
                                h2 { class: "text-xl font-bold text-slate-100", "{t(locale, TK::SajuTabTransit)}" }
                                div { class: "p-6 rounded-3xl bg-slate-900 border border-slate-800",
                                    div { class: "relative border-l-2 border-slate-700/50 ml-4 pl-6 space-y-8",
                                        {data.report.simulation_frames.iter().map(|frame| {
                                            let age = frame.age;
                                            let ganzi = &frame.ganzi;
                                            let m_ganzi = &frame.major_ganzi;
                                            let score = frame.score;
                                            let tags = frame.tags_as_strings();

                                            let (dot_color, score_color) = if score >= 70.0 {
                                                ("bg-emerald-500 shadow-[0_0_10px_rgba(16,185,129,0.5)]", "text-emerald-400")
                                            } else if score < 40.0 {
                                                ("bg-rose-500 shadow-[0_0_10px_rgba(244,63,94,0.5)]", "text-rose-400")
                                            } else {
                                                ("bg-slate-500", "text-slate-400")
                                            };

                                            let stem_god = TenGod::from_stems(day_master, ganzi.stem);
                                            let branch_god = TenGod::from_stem_and_branch(day_master, ganzi.branch);
                                            let stem_god_str = translate_saju_ten_god(locale, stem_god);
                                            let branch_god_str = translate_saju_ten_god(locale, branch_god);
                                            let ganzi_str = translate_saju_ganzi(locale, ganzi);
                                            let m_ganzi_str = translate_saju_ganzi(locale, m_ganzi);

                                            rsx! {
                                                div { class: "relative group",
                                                    div { class: "absolute -left-[31px] top-4 w-3 h-3 rounded-full border-2 border-slate-900 {dot_color} group-hover:scale-150 transition-transform" }

                                                    div { class: "flex flex-col md:flex-row md:items-center gap-4 bg-slate-800/30 p-5 rounded-2xl border border-slate-800/60 hover:bg-slate-800/80 hover:border-slate-700 transition-all shadow-sm",
                                                        div { class: "flex-shrink-0 w-24",
                                                            div { class: "text-2xl font-black text-slate-200 tracking-tight", "{age}세" }
                                                            div { class: "text-xs text-slate-500 mt-1 font-medium", "대운: {m_ganzi_str}" }
                                                        }

                                                        div { class: "flex-1 flex gap-4 md:gap-6",
                                                            div { class: "text-center",
                                                                div { class: "text-[10px] uppercase tracking-wider text-slate-400 mb-1 font-semibold", "{stem_god_str}" }
                                                                div { class: "text-2xl font-bold {element_style(ganzi.stem.hangul()).0}", "{ganzi.stem.hanja()}" }
                                                            }
                                                            div { class: "text-center",
                                                                div { class: "text-[10px] uppercase tracking-wider text-slate-400 mb-1 font-semibold", "{branch_god_str}" }
                                                                div { class: "text-2xl font-bold {element_style(ganzi.branch.hangul()).0}", "{ganzi.branch.hanja()}" }
                                                            }
                                                            div { class: "self-center text-lg font-medium text-slate-300 ml-2",
                                                                "{ganzi_str}년"
                                                            }
                                                        }

                                                        div { class: "flex-1 flex flex-col md:items-end gap-2",
                                                            div { class: "text-2xl font-black tracking-tighter {score_color}", "{score as i32}점" }
                                                            div { class: "flex flex-wrap gap-1.5 justify-end",
                                                                {tags.iter().map(|tag| rsx! {
                                                                    span { class: "px-2 py-0.5 rounded-md text-[11px] font-medium bg-slate-950/60 border border-slate-700/50 text-slate-300 shadow-sm",
                                                                        "{translate_saju_tag_str(locale, tag)}"
                                                                    }
                                                                })}
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        })}
                                    }
                                }
                            }
                        }

                        if *active_sub_tab.read() == SubTab::Remedies {
                            if let Some(remedies) = &data.report.remedies {
                                div { class: "w-full space-y-6 animate-fade-in",
                                    h2 { class: "text-xl font-bold text-slate-100", "{t(locale, TK::SajuTabRemedies)}" }
                                    div { class: "grid grid-cols-2 gap-4",
                                        div { class: "p-5 rounded-xl bg-slate-800/40 border border-slate-700/50",
                                            h3 { class: "text-sm font-semibold text-slate-400 mb-2", "{t(locale, TK::SajuRemediesColors)}" }
                                            div { class: "flex gap-2",
                                                for color in &remedies.lucky_colors {
                                                    span { class: "px-3 py-1 bg-slate-900 rounded border border-slate-700 text-sm font-medium", "{color}" }
                                                }
                                            }
                                        }
                                        div { class: "p-5 rounded-xl bg-slate-800/40 border border-slate-700/50",
                                            h3 { class: "text-sm font-semibold text-slate-400 mb-2", "{t(locale, TK::SajuRemediesNumbers)}" }
                                            div { class: "flex gap-2",
                                                for num in &remedies.lucky_numbers {
                                                    span { class: "px-3 py-1 bg-slate-900 rounded border border-slate-700 text-sm font-bold text-amber-400", "{num}" }
                                                }
                                            }
                                        }
                                    }
                                    div { class: "p-5 rounded-xl bg-slate-800/40 border border-slate-700/50 space-y-3",
                                        h3 { class: "text-sm font-semibold text-slate-400", "{t(locale, TK::SajuRemediesAdvice)}" }
                                        p { class: "text-slate-300 leading-relaxed", "{remedies.lifestyle_advice}" }
                                        div { class: "mt-4 p-3 rounded bg-red-950/20 border border-red-900/30 text-red-300 text-sm",
                                            "주의: {remedies.warning}"
                                        }
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
}

fn element_card_style(el_name: &str) -> (&'static str, &'static str, &'static str) {
    match el_name {
        "목" => (
            "text-emerald-400",
            "bg-emerald-950/20 border-emerald-800/30",
            "🌿",
        ),
        "화" => ("text-rose-400", "bg-rose-950/20 border-rose-800/30", "🔥"),
        "토" => (
            "text-yellow-400",
            "bg-amber-950/20 border-amber-900/30",
            "⛰️",
        ),
        "금" => (
            "text-slate-300",
            "bg-slate-800/40 border-slate-700/30",
            "⚙️",
        ),
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
    stem_god: Option<TenGod>,
    stem: HeavenlyStem,
    branch_god: TenGod,
    branch: EarthlyBranch,
    twelve_stage: String,
    nayin: String,
    shinsals: Vec<eon_saju::analysis::spirit_markers::SpiritMarkerDetail>,
    jijanggans: Vec<JijangganDisplayItem>,
) -> Element {
    let state = use_context::<AnalysisState>();
    let locale = *state.locale.read();
    let (s_text_color, s_bg_color, s_icon) = element_card_style(stem.element().hangul());
    let (b_text_color, b_bg_color, b_icon) = element_card_style(branch.element().hangul());

    let stem_god_str = stem_god
        .map(|g| translate_saju_ten_god(locale, g))
        .unwrap_or_else(|| t(locale, TK::SajuDayMaster));
    let stem_hanja = stem.hanja();
    let stem_hangul = translate_saju_stem(locale, stem);
    let stem_element = translate_saju_element(locale, stem.element());

    let branch_god_str = translate_saju_ten_god(locale, branch_god);
    let branch_hanja = branch.hanja();
    let branch_hangul = translate_saju_branch(locale, branch);
    let branch_element = translate_saju_element(locale, branch.element());

    rsx! {
        div { class: "flex flex-col gap-2.5 p-4 rounded-2xl bg-slate-950/40 border border-slate-800 hover:scale-[1.02] hover:border-slate-700 transition-all duration-200 shadow-xl",
            // Pillar Title
            div { class: "text-center text-xs font-bold text-slate-400 tracking-widest pb-2 border-b border-slate-800",
                "{title}"
            }

            // Heavenly Stem (천간)
            div { class: "flex flex-col items-center p-3.5 rounded-xl {s_bg_color} border flex-1 justify-center relative overflow-hidden",
                span { class: "text-xs font-semibold text-slate-500 mb-1.5", "{stem_god_str}" }
                span { class: "text-5xl font-extrabold font-serif {s_text_color} leading-none tracking-tight", "{stem_hanja}" }
                span { class: "text-base text-slate-300 font-bold mt-1.5", "{stem_hangul}" }
                span { class: "text-[10px] text-slate-500 font-mono mt-1", "{s_icon} {stem_element}" }
            }

            // Earthly Branch (지지)
            div { class: "flex flex-col items-center p-3.5 rounded-xl {b_bg_color} border flex-1 justify-center relative overflow-hidden",
                span { class: "text-[10px] text-slate-500 font-mono mb-1", "{b_icon} {branch_element}" }
                span { class: "text-5xl font-extrabold font-serif {b_text_color} leading-none tracking-tight", "{branch_hanja}" }
                span { class: "text-base text-slate-300 font-bold mt-1.5", "{branch_hangul}" }
                span { class: "text-xs font-semibold text-slate-500 mt-1.5", "{branch_god_str}" }
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
                                    span { class: "font-serif font-bold text-slate-300", "{item.stem.hanja()}({translate_saju_stem(locale, item.stem)})" }
                                    span { class: "text-[9px] {el_color} font-medium", "{translate_saju_ten_god(locale, item.ten_god)}" }
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

            // Nayin (납음오행)
            div { class: "text-center py-1.5 px-3 rounded-xl bg-slate-900/60 border border-slate-800/80 text-[10px] font-bold text-slate-400 shadow-inner",
                "☯ {nayin}"
            }

            // Twelve Stage
            div { class: "text-center py-2 px-3 rounded-xl bg-slate-900 border border-slate-800 text-xs font-extrabold text-amber-400/90 shadow-inner",
                "⚡ {twelve_stage}"
            }

            // Shinsal list
            if !shinsals.is_empty() {
                div { class: "flex flex-col gap-1 mt-1.5",
                    {shinsals.iter().map(|s| {
                        let label = translate_saju_spirit_marker_name(locale, s.marker);

                        let bg_cls = match s.level {
                            InterpretationLevel::Auspicious => "bg-emerald-950/30 text-emerald-400 border-emerald-800/40",
                            InterpretationLevel::Caution => "bg-rose-950/30 text-rose-400 border-rose-800/40",
                            InterpretationLevel::Danger => "bg-red-950/50 text-red-400 border-red-500 shadow-[0_0_8px_rgba(239,68,68,0.5)] animate-pulse",
                            InterpretationLevel::Neutral => "bg-slate-800 text-slate-400 border-slate-700",
                        };

                        let clash_cls = if s.is_clashed { "line-through opacity-70 decoration-red-500/70" } else { "" };
                        let combine_cls = if s.is_combined { "ring-1 ring-blue-400/50 shadow-[0_0_10px_rgba(96,165,250,0.3)]" } else { "" };

                        rsx! {
                            span { class: "text-[10px] py-1 px-2.5 rounded-full border text-center font-bold tracking-tight truncate transition-all {bg_cls} {clash_cls} {combine_cls}",
                                "✦ {label}"
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
        (
            "bg-emerald-900/40 border-emerald-700/50 text-emerald-300",
            "○",
        )
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
            JijangganItem {
                stem: Ren,
                ratio: 33,
                type_key: TK::SajuJijangganYeogi,
            },
            JijangganItem {
                stem: Gui,
                ratio: 67,
                type_key: TK::SajuJijangganJeonggi,
            },
        ],
        EarthlyBranch::Chou => vec![
            JijangganItem {
                stem: Gui,
                ratio: 30,
                type_key: TK::SajuJijangganYeogi,
            },
            JijangganItem {
                stem: Xin,
                ratio: 10,
                type_key: TK::SajuJijangganJunggi,
            },
            JijangganItem {
                stem: Ji,
                ratio: 60,
                type_key: TK::SajuJijangganJeonggi,
            },
        ],
        EarthlyBranch::Yin => vec![
            JijangganItem {
                stem: Wu,
                ratio: 23,
                type_key: TK::SajuJijangganYeogi,
            },
            JijangganItem {
                stem: Bing,
                ratio: 23,
                type_key: TK::SajuJijangganJunggi,
            },
            JijangganItem {
                stem: Jia,
                ratio: 54,
                type_key: TK::SajuJijangganJeonggi,
            },
        ],
        EarthlyBranch::Mao => vec![
            JijangganItem {
                stem: Jia,
                ratio: 33,
                type_key: TK::SajuJijangganYeogi,
            },
            JijangganItem {
                stem: Yi,
                ratio: 67,
                type_key: TK::SajuJijangganJeonggi,
            },
        ],
        EarthlyBranch::Chen => vec![
            JijangganItem {
                stem: Yi,
                ratio: 30,
                type_key: TK::SajuJijangganYeogi,
            },
            JijangganItem {
                stem: Gui,
                ratio: 10,
                type_key: TK::SajuJijangganJunggi,
            },
            JijangganItem {
                stem: Wu,
                ratio: 60,
                type_key: TK::SajuJijangganJeonggi,
            },
        ],
        EarthlyBranch::Si => vec![
            JijangganItem {
                stem: Wu,
                ratio: 23,
                type_key: TK::SajuJijangganYeogi,
            },
            JijangganItem {
                stem: Geng,
                ratio: 23,
                type_key: TK::SajuJijangganJunggi,
            },
            JijangganItem {
                stem: Bing,
                ratio: 54,
                type_key: TK::SajuJijangganJeonggi,
            },
        ],
        EarthlyBranch::Wu => vec![
            JijangganItem {
                stem: Bing,
                ratio: 33,
                type_key: TK::SajuJijangganYeogi,
            },
            JijangganItem {
                stem: Ji,
                ratio: 30,
                type_key: TK::SajuJijangganJunggi,
            },
            JijangganItem {
                stem: Ding,
                ratio: 37,
                type_key: TK::SajuJijangganJeonggi,
            },
        ],
        EarthlyBranch::Wei => vec![
            JijangganItem {
                stem: Ding,
                ratio: 30,
                type_key: TK::SajuJijangganYeogi,
            },
            JijangganItem {
                stem: Yi,
                ratio: 10,
                type_key: TK::SajuJijangganJunggi,
            },
            JijangganItem {
                stem: Ji,
                ratio: 60,
                type_key: TK::SajuJijangganJeonggi,
            },
        ],
        EarthlyBranch::Shen => vec![
            JijangganItem {
                stem: Wu,
                ratio: 23,
                type_key: TK::SajuJijangganYeogi,
            },
            JijangganItem {
                stem: Ren,
                ratio: 23,
                type_key: TK::SajuJijangganJunggi,
            },
            JijangganItem {
                stem: Geng,
                ratio: 54,
                type_key: TK::SajuJijangganJeonggi,
            },
        ],
        EarthlyBranch::You => vec![
            JijangganItem {
                stem: Geng,
                ratio: 33,
                type_key: TK::SajuJijangganYeogi,
            },
            JijangganItem {
                stem: Xin,
                ratio: 67,
                type_key: TK::SajuJijangganJeonggi,
            },
        ],
        EarthlyBranch::Xu => vec![
            JijangganItem {
                stem: Xin,
                ratio: 30,
                type_key: TK::SajuJijangganYeogi,
            },
            JijangganItem {
                stem: Ding,
                ratio: 10,
                type_key: TK::SajuJijangganJunggi,
            },
            JijangganItem {
                stem: Wu,
                ratio: 60,
                type_key: TK::SajuJijangganJeonggi,
            },
        ],
        EarthlyBranch::Hai => vec![
            JijangganItem {
                stem: Wu,
                ratio: 23,
                type_key: TK::SajuJijangganYeogi,
            },
            JijangganItem {
                stem: Jia,
                ratio: 23,
                type_key: TK::SajuJijangganJunggi,
            },
            JijangganItem {
                stem: Ren,
                ratio: 54,
                type_key: TK::SajuJijangganJeonggi,
            },
        ],
    }
}
