use dioxus::prelude::*;
use crate::store::{AnalysisState, TaskStatus};
use eon_service::dto::{SajuAnalysisInput, AnalysisInput};
use eon_service::facade;
use eon_saju::analysis::strength::StrengthType;
use eon_saju::analysis::supplementary_pillars::InterpretationLevel;
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
                    "사주 명식 (四柱 命式)"
                }
                button {
                    class: "px-5 py-2.5 bg-gradient-to-r from-amber-600 to-orange-600 hover:from-amber-500 hover:to-orange-500 rounded-xl font-semibold text-white shadow-lg shadow-amber-900/30 transition-all duration-200 active:scale-95",
                    onclick: run_analysis,
                    "🔮 분석 실행"
                }
            }

            match &state.saju.read().status {
                TaskStatus::Idle => rsx! {
                    div { class: "flex flex-col items-center justify-center py-20 gap-3 text-slate-500",
                        span { class: "text-5xl", "🌌" }
                        p { class: "text-lg font-medium", "출생 정보를 입력하고 분석을 실행하세요." }
                    }
                },
                TaskStatus::Loading => rsx! {
                    div { class: "flex flex-col items-center justify-center py-20 gap-3",
                        div { class: "w-12 h-12 rounded-full border-4 border-amber-500/30 border-t-amber-400 animate-spin" }
                        p { class: "text-amber-400 font-medium animate-pulse", "사주 계산 중..." }
                    }
                },
                TaskStatus::Error(e) => rsx! {
                    div { class: "p-4 rounded-xl bg-red-900/20 border border-red-800/50 text-red-400", "오류: {e}" }
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
                            let decision_ages = comp.decision_nodes.iter().map(|&a| format!("{}세", a)).collect::<Vec<_>>().join(", ");
                            (comp_label, entropy_str, decision_ages)
                        });

                        rsx! {
                            // ── 1. 사주 원국 (천간/지지/십성/12운성/신살) ─────────
                            div { class: "grid grid-cols-4 gap-3.5",
                                PillarCard {
                                    title: "시주 (Hour)",
                                    stem_god: data.report.ten_gods.hour_stem.hangul().to_string(),
                                    stem_hanja: data.report.pillars.hour.stem.hanja().to_string(),
                                    stem_hangul: data.report.pillars.hour.stem.hangul().to_string(),
                                    stem_element: data.report.pillars.hour.stem.element().hangul().to_string(),
                                    branch_god: data.report.ten_gods.hour_branch.hangul().to_string(),
                                    branch_hanja: data.report.pillars.hour.branch.hanja().to_string(),
                                    branch_hangul: data.report.pillars.hour.branch.hangul().to_string(),
                                    branch_element: data.report.pillars.hour.branch.element().hangul().to_string(),
                                    twelve_stage: twelve_stages.hour_stage.hangul().to_string(),
                                    shinsals: shinsals_for(eon_saju::analysis::spirit_markers::PillarPosition::Hour)
                                }
                                PillarCard {
                                    title: "일주 (Day)",
                                    stem_god: "일간 (본인)".to_string(),
                                    stem_hanja: data.report.pillars.day.stem.hanja().to_string(),
                                    stem_hangul: data.report.pillars.day.stem.hangul().to_string(),
                                    stem_element: data.report.pillars.day.stem.element().hangul().to_string(),
                                    branch_god: data.report.ten_gods.day_branch.hangul().to_string(),
                                    branch_hanja: data.report.pillars.day.branch.hanja().to_string(),
                                    branch_hangul: data.report.pillars.day.branch.hangul().to_string(),
                                    branch_element: data.report.pillars.day.branch.element().hangul().to_string(),
                                    twelve_stage: twelve_stages.day_stage.hangul().to_string(),
                                    shinsals: shinsals_for(eon_saju::analysis::spirit_markers::PillarPosition::Day)
                                }
                                PillarCard {
                                    title: "월주 (Month)",
                                    stem_god: data.report.ten_gods.month_stem.hangul().to_string(),
                                    stem_hanja: data.report.pillars.month.stem.hanja().to_string(),
                                    stem_hangul: data.report.pillars.month.stem.hangul().to_string(),
                                    stem_element: data.report.pillars.month.stem.element().hangul().to_string(),
                                    branch_god: data.report.ten_gods.month_branch.hangul().to_string(),
                                    branch_hanja: data.report.pillars.month.branch.hanja().to_string(),
                                    branch_hangul: data.report.pillars.month.branch.hangul().to_string(),
                                    branch_element: data.report.pillars.month.branch.element().hangul().to_string(),
                                    twelve_stage: twelve_stages.month_stage.hangul().to_string(),
                                    shinsals: shinsals_for(eon_saju::analysis::spirit_markers::PillarPosition::Month)
                                }
                                PillarCard {
                                    title: "연주 (Year)",
                                    stem_god: data.report.ten_gods.year_stem.hangul().to_string(),
                                    stem_hanja: data.report.pillars.year.stem.hanja().to_string(),
                                    stem_hangul: data.report.pillars.year.stem.hangul().to_string(),
                                    stem_element: data.report.pillars.year.stem.element().hangul().to_string(),
                                    branch_god: data.report.ten_gods.year_branch.hangul().to_string(),
                                    branch_hanja: data.report.pillars.year.branch.hanja().to_string(),
                                    branch_hangul: data.report.pillars.year.branch.hangul().to_string(),
                                    branch_element: data.report.pillars.year.branch.element().hangul().to_string(),
                                    twelve_stage: twelve_stages.year_stage.hangul().to_string(),
                                    shinsals: shinsals_for(eon_saju::analysis::spirit_markers::PillarPosition::Year)
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
                                            rsx! {
                                                span { class: "text-3xl", "{icon}" }
                                                div {
                                                    div { class: "inline-block px-3 py-1 rounded-full border text-sm font-bold {badge_color}",
                                                        "{st.hangul()} ({st.hanja()})"
                                                    }
                                                    p { class: "text-xs text-slate-500 mt-1",
                                                        "조건 {data.report.strength.acquired_count}/4 충족 | 점수 {data.report.strength.strength_score:.1}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    // 득령/득지/득시/득세
                                    div { class: "grid grid-cols-2 gap-1.5 text-xs",
                                        DeukBadge { label: "득령", acquired: data.report.strength.deuk_ryeong.acquired }
                                        DeukBadge { label: "득지", acquired: data.report.strength.deuk_ji.acquired }
                                        DeukBadge { label: "득시", acquired: data.report.strength.deuk_si.acquired }
                                        DeukBadge { label: "득세", acquired: data.report.strength.deuk_se.acquired }
                                    }
                                }

                                // 용신
                                div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5 flex flex-col gap-3",
                                    h3 { class: "text-sm font-semibold text-slate-400 uppercase tracking-widest", "용신 (用神)" }
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
                                                        p { class: "text-xs text-slate-400", "제1용신" }
                                                        p { class: "font-bold {p_color}", "{primary_el.hangul()} ({primary_el.hanja()})" }
                                                    }
                                                }
                                                div { class: "flex items-center gap-2",
                                                    div { class: "flex-1",
                                                        p { class: "text-xs text-slate-400", "희신" }
                                                        p { class: "font-semibold {a_color}", "{assist_el.hangul()} ({assist_el.hanja()})" }
                                                    }
                                                }
                                                if !yn.recommendations.is_empty() {
                                                    p { class: "text-xs text-slate-500 mt-1 leading-relaxed",
                                                        "{yn.recommendations[0].summary}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                // 분석 메타
                                div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5 flex flex-col gap-3",
                                    h3 { class: "text-sm font-semibold text-slate-400 uppercase tracking-widest", "분석 정보" }
                                    div { class: "space-y-2 text-sm",
                                        MetaRow { label: "입력 시각", value: data.meta.input_time.clone() }
                                        MetaRow { label: "교정 시각", value: data.meta.corrected_time.clone() }
                                        MetaRow { label: "타임존", value: data.meta.analysis_timezone.clone() }
                                        MetaRow { label: "DST", value: if data.meta.is_dst { "적용됨".to_string() } else { "해당없음".to_string() } }
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
                            if let Some(ml) = &data.report.major_luck {
                                div { class: "bg-slate-900 border border-slate-800 rounded-2xl overflow-hidden",
                                    div { class: "bg-slate-800/50 border-b border-slate-800 px-5 py-3 flex items-center justify-between",
                                        h3 { class: "font-semibold text-slate-200", "대운 (大運) — {ml.direction}" }
                                        span { class: "text-xs text-slate-400", "만 {ml.start_age}세 교운" }
                                    }
                                    div { class: "p-4 overflow-x-auto",
                                        div { class: "flex gap-2 min-w-max",
                                            {ml.cycles.iter().map(|cycle| rsx! {
                                                div { class: "flex flex-col items-center gap-1 p-3 rounded-xl bg-slate-800/50 border border-slate-700/50 hover:border-amber-700/60 transition-colors min-w-[80px]",
                                                    span { class: "text-xs text-slate-400 font-mono", "만 {cycle.start_age}세~" }
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
                                h3 { class: "text-sm font-semibold text-slate-400 uppercase tracking-widest", "보조 기둥 및 신살 (Supplementary Pillars)" }
                                div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                                    {
                                        let sp = &data.report.supplementary_pillars;
                                        let aux_pillars = vec![
                                            ("태원 (胎元)", &sp.taewon, "taewon"),
                                            ("명궁 (命宮)", &sp.myeonggung, "myeonggung"),
                                            ("신궁 (身宮)", &sp.shingung, "shingung"),
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
                                h3 { class: "text-sm font-semibold text-slate-400 uppercase tracking-widest", "공망 분석 (Void / Emptiness)" }
                                div { class: "p-4 rounded-xl bg-slate-800/40 border border-slate-800 flex flex-col gap-3 shadow-inner",
                                    div { class: "flex flex-wrap gap-x-6 gap-y-2 text-sm",
                                        p { class: "text-slate-400 font-bold",
                                            "순중 공망 (Xun Group): "
                                            span { class: "text-amber-400", {data.report.voids.xun_group.clone()} }
                                        }
                                        p { class: "text-slate-400 font-bold",
                                            "공망 지지 (Void Branches): "
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
                                                "공망 십성 (Void Ten Gods): "
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
                                                let label_pos = format!("{} 공망", void_dt.position);
                                                rsx! {
                                                    div { class: "p-3 rounded-lg bg-slate-900/40 border border-slate-800/60 space-y-1 text-xs hover:border-slate-750 transition-colors",
                                                        div { class: "flex justify-between items-center gap-2 flex-wrap",
                                                            div { class: "flex items-center gap-1.5",
                                                                span { class: "font-serif text-slate-200 font-bold text-sm", "{label_branch}" }
                                                                span { class: "text-amber-400/90 font-semibold", "{label_tg}" }
                                                                span { class: "px-2 py-0.5 rounded border text-[9px] font-bold {lvl_cls}",
                                                                    {match void_dt.level {
                                                                        eon_saju::analysis::supplementary_pillars::InterpretationLevel::Auspicious => "길조(吉)",
                                                                        eon_saju::analysis::supplementary_pillars::InterpretationLevel::Caution => "영향(凶)",
                                                                        eon_saju::analysis::supplementary_pillars::InterpretationLevel::Neutral => "보통",
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
                                        h3 { class: "text-sm font-semibold text-slate-400 uppercase tracking-widest", "운명 취약점 퍼징 리포트 (Destiny Fuzzer)" }
                                        span { class: "text-xs font-bold text-rose-400 bg-rose-950/20 px-2.5 py-0.5 rounded border border-rose-900/30",
                                            "크래시: {data.crash_count}회 검출"
                                        }
                                    }
                                    p { class: "text-xs text-slate-400 leading-relaxed",
                                        "사주 가상머신(VM)에서 현재 월지 환경 하에 60갑자 세운을 대입·퍼징하여, 시스템 에너지가 극도로 저하되는 임계점(Crash)을 찾아낸 진단입니다."
                                    }
                                    if let Some(fuzz) = &data.vulnerability_report {
                                        div { class: "space-y-3.5 max-h-[300px] overflow-y-auto pr-1",
                                            {fuzz.critical_vectors.iter().map(|vuln| {
                                                let score_val = vuln.crash_score;
                                                let energy_lbl = format!("에너지 레벨: {:.1}", score_val);
                                                let vuln_type = vuln.vulnerability_type.clone();
                                                let major_gz = format!("{}{}", vuln.vector.major.hanja(), vuln.vector.major.hangul());
                                                let yearly_gz = format!("{}{}", vuln.vector.yearly.hanja(), vuln.vector.yearly.hangul());
                                                let vector_desc = format!("대운: {} | 세운: {}", major_gz, yearly_gz);
                                                let tags_list = vuln.tags.join(", ");
                                                rsx! {
                                                    div { class: "p-3.5 rounded-xl bg-slate-950/40 border border-slate-850 hover:border-red-900/40 transition-colors flex flex-col gap-1.5 shadow-inner",
                                                        div { class: "flex justify-between items-center flex-wrap gap-2",
                                                            span { class: "text-xs font-mono font-extrabold text-rose-400", "⚠️ {vuln_type}" }
                                                            span { class: "text-xs font-bold text-rose-500", "{energy_lbl}" }
                                                        }
                                                        p { class: "text-xs text-slate-300 font-serif", "{vector_desc}" }
                                                        if !tags_list.is_empty() {
                                                            p { class: "text-[10px] text-slate-500 font-mono", "결정적 요소: {tags_list}" }
                                                        }
                                                    }
                                                }
                                            })}
                                        }
                                    } else {
                                        p { class: "text-slate-500 text-xs py-4", "크래시 데이터 분석 결과 없음" }
                                    }
                                }

                                // 2) Karma Load Balancer (인생 부하 분산 진단)
                                div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5 space-y-4 shadow-xl",
                                    h3 { class: "text-sm font-semibold text-slate-400 uppercase tracking-widest", "인생 부하 분산 진단 (Karma Load Balancer)" }
                                    p { class: "text-xs text-slate-400 leading-relaxed",
                                        "일생의 흐름 중 급등 또는 급락하여 시스템 오버헤드가 과다 발생하는 시점을 감지하고 이에 맞는 행동 전략을 제시합니다."
                                    }
                                    if !data.load_diagnostics.is_empty() {
                                        div { class: "space-y-3.5 max-h-[300px] overflow-y-auto pr-1",
                                            {data.load_diagnostics.iter().map(|diag| {
                                                let (badge_cls, status_lbl) = match diag.status {
                                                    eon_saju::engine::load_balancer::TrafficStatus::Idle => ("text-emerald-400 bg-emerald-950/20 border-emerald-900/30", "평온 (Idle)"),
                                                    eon_saju::engine::load_balancer::TrafficStatus::Normal => ("text-blue-400 bg-blue-950/20 border-blue-900/30", "보통 (Normal)"),
                                                    eon_saju::engine::load_balancer::TrafficStatus::Overloaded => ("text-amber-400 bg-amber-950/20 border-amber-900/30", "오버로드 (Overload)"),
                                                    eon_saju::engine::load_balancer::TrafficStatus::SystemDown => ("text-rose-400 bg-rose-950/20 border-rose-900/30", "다운 (System Down)"),
                                                };
                                                let age_val = diag.age;
                                                let reason_desc = diag.reason.clone();
                                                let strategy_desc = diag.strategy.clone();
                                                rsx! {
                                                    div { class: "p-3.5 rounded-xl bg-slate-950/40 border border-slate-850 hover:border-slate-750 transition-colors flex flex-col gap-1.5 shadow-inner",
                                                        div { class: "flex justify-between items-center flex-wrap gap-2",
                                                            span { class: "text-xs font-bold text-slate-300 font-mono", "만 {age_val}세" }
                                                            span { class: "text-[10px] font-bold px-2 py-0.5 rounded border {badge_cls}", "{status_lbl}" }
                                                        }
                                                        p { class: "text-xs font-semibold text-amber-400", "{reason_desc}" }
                                                        p { class: "text-[11px] text-slate-400 leading-relaxed", "{strategy_desc}" }
                                                    }
                                                }
                                            })}
                                        }
                                    } else {
                                        p { class: "text-slate-500 text-xs py-4", "안정적인 흐름으로 감지된 시스템 이벤트가 없습니다." }
                                    }
                                }
                            }

                            // ── 6. 신살 상세 해설 (Spirit Markers Detail) ─────────
                            if !data.report.spirit_markers.mapped_markers.is_empty() {
                                div { class: "bg-slate-900 border border-slate-800 rounded-2xl p-5 space-y-4",
                                    h3 { class: "text-sm font-semibold text-slate-400 uppercase tracking-widest", "신살 상세 해설 (Spirit Markers Detail)" }
                                    div { class: "space-y-3",
                                        {data.report.spirit_markers.mapped_markers.iter().map(|m| {
                                            let lvl_cls = match m.level {
                                                InterpretationLevel::Auspicious => "text-emerald-400 bg-emerald-950/30 border-emerald-800/40",
                                                InterpretationLevel::Caution => "text-rose-400 bg-rose-950/30 border-rose-800/40",
                                                InterpretationLevel::Neutral => "text-slate-400 bg-slate-850 border-slate-800",
                                            };
                                            let pos_name = match m.position {
                                                eon_saju::analysis::spirit_markers::PillarPosition::Year => "년주 (Year)",
                                                eon_saju::analysis::spirit_markers::PillarPosition::Month => "월주 (Month)",
                                                eon_saju::analysis::spirit_markers::PillarPosition::Day => "일주 (Day)",
                                                eon_saju::analysis::spirit_markers::PillarPosition::Hour => "시주 (Hour)",
                                            };
                                            let part = if m.is_stem { "천간" } else { "지지" };
                                            let marker_label = format!("{} ({})", m.marker.hangul(), m.marker.hanja());
                                            let pos_part = format!("{} {}", pos_name, part);
                                            rsx! {
                                                div { class: "p-4 rounded-xl bg-slate-800/20 border border-slate-800/60 space-y-2 hover:border-slate-700 transition-colors",
                                                    div { class: "flex items-center justify-between gap-2 flex-wrap",
                                                        div { class: "flex items-center gap-2",
                                                            span { class: "text-base font-bold text-slate-200", "{marker_label}" }
                                                            span { class: "text-xs px-2.5 py-0.5 rounded border font-bold {lvl_cls}",
                                                                {match m.level {
                                                                    InterpretationLevel::Auspicious => "길조(吉)",
                                                                    InterpretationLevel::Caution => "주의(凶)",
                                                                    InterpretationLevel::Neutral => "보통",
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
                                                            span { class: "text-[10px] text-slate-500 font-bold", "성립 요건:" }
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
) -> Element {
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
