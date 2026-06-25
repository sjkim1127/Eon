use crate::store::{AnalysisState, TaskStatus};
use crate::i18n::{t, TK, Locale, translate_planet, translate_planet_str, translate_avastha, rasi_name,
    translate_saju_stem, translate_saju_branch, translate_saju_element, translate_saju_ten_god,
    translate_saju_spirit_marker_name, translate_saju_void_desc, translate_saju_ganzi, translate_saju_tag_str,
    translate_saju_load_balancer, translate_spirit_desc, translate_saju_reason, translate_saju_relation_str,
    translate_saju_twelve_stage_str};
use eon_service::dto::{SajuAnalysisOutput, VedicAnalysisOutput, TransitAnalysisOutput, TierResult, IChingAnalysisOutput, WesternAnalysisOutput};
use eon_vedic::planets::VedicPlanet;
use crate::i18n::iching_db::{get_hexagram_info, get_yao_name, get_yao_description};
use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(inline_js = r#"
export function copy_to_clipboard(text) {
    if (navigator.clipboard && navigator.clipboard.writeText) {
        navigator.clipboard.writeText(text).catch(err => {
            console.error("Failed to copy using navigator.clipboard: ", err);
            fallback_copy(text);
        });
    } else {
        fallback_copy(text);
    }
}

function fallback_copy(text) {
    const textArea = document.createElement("textarea");
    textArea.value = text;
    textArea.style.position = "fixed";
    textArea.style.left = "-999999px";
    textArea.style.top = "-999999px";
    document.body.appendChild(textArea);
    textArea.focus();
    textArea.select();
    try {
        document.execCommand('copy');
    } catch (err) {
        console.error('Fallback copy failed', err);
    }
    document.body.removeChild(textArea);
}
"#)]
extern "C" {
    pub fn copy_to_clipboard(text: &str);
}

const NAKSHATRA_NAMES: &[&str] = &[
    "", "Ashwini", "Bharani", "Krittika", "Rohini", "Mrigashira",
    "Ardra", "Punarvasu", "Pushya", "Ashlesha", "Magha",
    "Purva Phalguni", "Uttara Phalguni", "Hasta", "Chitra",
    "Swati", "Vishakha", "Anuradha", "Jyeshtha", "Mula",
    "Purva Ashadha", "Uttara Ashadha", "Shravana", "Dhanishtha",
    "Shatabhisha", "Purva Bhadrapada", "Uttara Bhadrapada", "Revati",
];

fn nakshatra_name(n: u8) -> &'static str {
    if n == 0 || n > 27 { return "—" }
    NAKSHATRA_NAMES[n as usize]
}

fn format_baladi(av: &eon_vedic::analysis::avasthas::BaladiAvastha) -> &'static str {
    use eon_vedic::analysis::avasthas::BaladiAvastha::*;
    match av {
        Bala => "Bala (Infant)",
        Kumara => "Kumara (Youth)",
        Yuva => "Yuva (Adult)",
        Vriddha => "Vriddha (Old)",
        Mrita => "Mrita (Dead)",
    }
}

fn format_jagradadi(av: &eon_vedic::analysis::avasthas::JagradadiAvastha) -> &'static str {
    use eon_vedic::analysis::avasthas::JagradadiAvastha::*;
    match av {
        Jagrat => "Jagrat (Awake)",
        Swapna => "Swapna (Dream)",
        Sushupti => "Sushupti (Sleep)",
    }
}

fn format_deeptaadi(av: &eon_vedic::analysis::avasthas::DeeptaadiAvastha) -> &'static str {
    use eon_vedic::analysis::avasthas::DeeptaadiAvastha::*;
    match av {
        Deepta => "Deepta (Exalted)",
        Svastha => "Svastha (Own Sign)",
        Mudita => "Mudita (Great Friend)",
        Shanta => "Shanta (Friend)",
        Deena => "Deena (Neutral)",
        Dukhita => "Dukhita (Enemy)",
        Vikala => "Vikala (Great Enemy)",
        Khala => "Khala (Debilitated)",
        Kopita => "Kopita (Combust)",
    }
}

fn format_karaka_role(role: &eon_vedic::analysis::jaimini::JaiminiKarakaRole) -> &'static str {
    use eon_vedic::analysis::jaimini::JaiminiKarakaRole::*;
    match role {
        Atmakaraka => "Atmakaraka (AK)",
        Amatyakaraka => "Amatyakaraka (AmK)",
        Bhratrukaraka => "Bhratrukaraka (BK)",
        Matrukaraka => "Matrukaraka (MK)",
        Pitrikaraka => "Pitrikaraka (PiK)",
        Putrakaraka => "Putrakaraka (PK)",
        Gnatikaraka => "Gnatikaraka (GK)",
        Darakaraka => "Darakaraka (DK)",
    }
}

// Format inner Saju content (without global header/birth info)
fn format_saju_inner(data: &SajuAnalysisOutput, locale: Locale) -> String {
    let mut s = String::new();
    let rep = &data.report;

    // Four Pillars
    let (pillars_title, pillar_col, hour_col, day_col, month_col, year_col) = match locale {
        Locale::Ko => ("사주 명식 (四柱八字)", "구분", "시주 (Hour)", "일주 (Day)", "월주 (Month)", "연주 (Year)"),
        Locale::En => ("Four Pillars of Destiny", "Category", "Hour Pillar", "Day Pillar", "Month Pillar", "Year Pillar"),
        Locale::Zh => ("八字命盘", "类别", "时柱", "日柱", "月柱", "年柱"),
        Locale::Ru => ("Четыре Столпа Судьбы", "Категория", "Столп Часа", "Столп Дня", "Столп Месяца", "Столп Года"),
    };
    
    let (stem_row, branch_row, god_stem_row, god_branch_row, stage_row, nayin_row) = match locale {
        Locale::Ko => ("천간 (Heavenly Stem)", "지지 (Earthly Branch)", "천간 십성", "지지 십성", "12운성 (Twelve Stage)", "납음오행 (Nayin)"),
        Locale::En => ("Heavenly Stem", "Earthly Branch", "Stem Ten-God", "Branch Ten-God", "Twelve Stage", "Nayin"),
        Locale::Zh => ("天干", "地支", "天干十神", "地支十神", "十二运星", "纳音五行"),
        Locale::Ru => ("Небесный ствол", "Земная ветвь", "Божество Ствола", "Божество Ветви", "12 стадий судьбы", "Найин"),
    };

    s.push_str(&format!("### {}\n\n", pillars_title));
    s.push_str(&format!("| {} | {} | {} | {} | {} |\n", pillar_col, hour_col, day_col, month_col, year_col));
    s.push_str("| --- | --- | --- | --- | --- |\n");
    s.push_str(&format!("| **{}** | {}({}) | {}({}) | {}({}) | {}({}) |\n", 
        stem_row, rep.pillars.hour.stem.hangul(), rep.pillars.hour.stem.hanja(),
        rep.pillars.day.stem.hangul(), rep.pillars.day.stem.hanja(),
        rep.pillars.month.stem.hangul(), rep.pillars.month.stem.hanja(),
        rep.pillars.year.stem.hangul(), rep.pillars.year.stem.hanja()
    ));
    s.push_str(&format!("| **{}** | {} | {} | {} | {} |\n", 
        god_stem_row, rep.ten_gods.hour_stem.hangul(), rep.ten_gods.day_stem.hangul(),
        rep.ten_gods.month_stem.hangul(), rep.ten_gods.year_stem.hangul()
    ));
    s.push_str(&format!("| **{}** | {}({}) | {}({}) | {}({}) | {}({}) |\n", 
        branch_row, rep.pillars.hour.branch.hangul(), rep.pillars.hour.branch.hanja(),
        rep.pillars.day.branch.hangul(), rep.pillars.day.branch.hanja(),
        rep.pillars.month.branch.hangul(), rep.pillars.month.branch.hanja(),
        rep.pillars.year.branch.hangul(), rep.pillars.year.branch.hanja()
    ));
    s.push_str(&format!("| **{}** | {} | {} | {} | {} |\n", 
        god_branch_row, rep.ten_gods.hour_branch.hangul(), rep.ten_gods.day_branch.hangul(),
        rep.ten_gods.month_branch.hangul(), rep.ten_gods.year_branch.hangul()
    ));
    
    let stages = rep.pillars.twelve_stages();
    s.push_str(&format!("| **{}** | {} | {} | {} | {} |\n", 
        stage_row,
        crate::i18n::translate_saju_twelve_stage(locale, stages.hour_stage),
        crate::i18n::translate_saju_twelve_stage(locale, stages.day_stage),
        crate::i18n::translate_saju_twelve_stage(locale, stages.month_stage),
        crate::i18n::translate_saju_twelve_stage(locale, stages.year_stage)
    ));
    s.push_str(&format!("| **{}** | {} | {} | {} | {} |\n\n", 
        nayin_row,
        crate::i18n::translate_saju_nayin(locale, rep.pillars.hour.nayin()),
        crate::i18n::translate_saju_nayin(locale, rep.pillars.day.nayin()),
        crate::i18n::translate_saju_nayin(locale, rep.pillars.month.nayin()),
        crate::i18n::translate_saju_nayin(locale, rep.pillars.year.nayin())
    ));

       // Day Master & Strength
    let (dm_title, dm_lbl, type_lbl, score_lbl, yong_lbl, hee_lbl, deuk_ryeong_lbl, deuk_ji_lbl, deuk_si_lbl, deuk_se_lbl) = match locale {
        Locale::Ko => ("일간 세력 및 용희신", "일간", "신강/신약 유형", "세력 점수", "용신 (用神)", "희신 (喜神)", "득령", "득지", "득시", "득세"),
        Locale::En => ("Day Master Strength & Yongshin/Heeshin", "Day Master", "Strength Type", "Strength Score", "Yongshin (Deity)", "Heeshin (Assistant)", "Deuk-Ryeong", "Deuk-Ji", "Deuk-Si", "Deuk-Se"),
        Locale::Zh => ("日主强弱与用喜神", "日主", "强弱类型", "能量分数", "用神", "喜神", "得令", "得地", "得时", "得势"),
        Locale::Ru => ("Сила Дневного Доминанта и Полезные Божества", "Дневной Доминант", "Тип силы", "Балл силы", "Полезный Дух", "Благоприятный Дух", "Дэ-Рён", "Дэ-Джи", "Дэ-Ши", "Дэ-Сэ"),
    };

    let str_type_str = match rep.strength.strength_type {
        eon_saju::analysis::strength::StrengthType::Strong => match locale {
            Locale::Ko => "신강 (Strong)",
            Locale::En => "Strong",
            Locale::Zh => "身强",
            Locale::Ru => "Сильный",
        },
        eon_saju::analysis::strength::StrengthType::Weak => match locale {
            Locale::Ko => "신약 (Weak)",
            Locale::En => "Weak",
            Locale::Zh => "身弱",
            Locale::Ru => "Слабый",
        },
        eon_saju::analysis::strength::StrengthType::Balanced => match locale {
            Locale::Ko => "중화 (Balanced)",
            Locale::En => "Balanced",
            Locale::Zh => "中和",
            Locale::Ru => "Нейтральный",
        },
    };

    let dr_val = if rep.strength.deuk_ryeong.acquired { "Yes" } else { "No" };
    let dj_val = if rep.strength.deuk_ji.acquired { "Yes" } else { "No" };
    let di_val = if rep.strength.deuk_si.acquired { "Yes" } else { "No" };
    let ds_val = if rep.strength.deuk_se.acquired { "Yes" } else { "No" };

    s.push_str(&format!("### {}\n\n", dm_title));
    s.push_str(&format!("- **{}**: {}({})\n", dm_lbl, rep.strength.day_master.hangul(), rep.strength.day_master.hanja()));
    s.push_str(&format!("- **{}**: {}\n", type_lbl, str_type_str));
    s.push_str(&format!("- **{}**: {:.1} / 100\n", score_lbl, rep.strength.strength_score));
    s.push_str(&format!("- **{}/{}/{}/{}**: {} / {} / {} / {}\n", deuk_ryeong_lbl, deuk_ji_lbl, deuk_si_lbl, deuk_se_lbl, dr_val, dj_val, di_val, ds_val));
    s.push_str(&format!("- **{}**: {}\n", yong_lbl, rep.yongshin.primary.hangul()));
    s.push_str(&format!("- **{}**: {}\n\n", hee_lbl, rep.yongshin.assistant.hangul()));

    // Detailed Yongshin Recommendations
    let (rec_yong_title, rec_el, rec_sum, rec_desc, rec_reason) = match locale {
        Locale::Ko => ("용신 추천 상세 (Yongshin Recommendations)", "오행", "요약", "상세 설명", "판단 근거"),
        Locale::En => ("Yongshin Detailed Recommendations", "Element", "Summary", "Description", "Reasons"),
        Locale::Zh => ("用神推荐详细", "五行", "摘要", "详细说明", "依据"),
        Locale::Ru => ("Подробные рекомендации Полезного Духа", "Элемент", "Резюме", "Описание", "Причины"),
    };
    s.push_str(&format!("#### {}\n\n", rec_yong_title));
    s.push_str(&format!("| {} | {} | {} | {} |\n", rec_el, rec_sum, rec_desc, rec_reason));
    s.push_str("| --- | --- | --- | --- |\n");
    for rec in &rep.yongshin.recommendations {
        let reasons_str = rec.reasons.join(", ");
        s.push_str(&format!("| {}({}) | **{}** | {} | {} |\n", rec.element.hangul(), rec.element.hanja(), rec.summary, rec.description, reasons_str));
    }
    s.push_str("\n");

    // Diagnostics
    let (diag_title, struct_lbl, struct_sum_lbl, struct_desc_lbl, struct_reasons_lbl, projected_stem_lbl, unpacker_lbl, bottleneck_lbl, throughput_lbl, entropy_lbl, entropy_score_lbl, entropy_desc_lbl, complexity_lbl, grade_lbl, maintenance_entropy_lbl, decision_nodes_lbl, crisis_lbl) = match locale {
        Locale::Ko => ("오행 흐름 및 구조 진단", "격국", "격국 요약", "격국 해설", "격국 근거", "투출 천간", "오행 해결사", "오행 정체 구간", "전체 에너지 유동 효율", "인생 엔트로피 난이도", "엔트로피 점수", "엔트로피 해석", "순환 복잡도", "안정성 등급", "유지보수 지수 (파란만장함)", "주요 분기점 연령", "운이 꺾이는 시기"),
        Locale::En => ("Qi Flow & Structural Diagnostics", "Structure Type", "Structure Summary", "Structure Description", "Structure Reasons", "Projected Stem", "Qi Unpacker", "Qi Bottleneck", "System Throughput", "Destiny Entropy Level", "Entropy Score", "Entropy Interpretation", "Cyclomatic Complexity", "Stability Grade", "Maintenance Entropy", "Decision Node Ages", "Crisis Period Count"),
        Locale::Zh => ("五行流通与结构诊断", "格局", "格局摘要", "格局解析", "格局依据", "透出天干", "五行通关", "五行阻滞", "整体能量效率", "人生熵级复杂度", "熵得分", "熵解析", "结构复杂度", "稳定度评级", "维护熵 (波折度)", "主要决策点年龄", "危机时期数量"),
        Locale::Ru => ("Поток Ци 및 Диагностика Структуры", "Тип структуры", "Резюме структуры", "Описание структуры", "Обоснование структуры", "Проявленный ствол", "Разрешитель Ци", "Затор Ци", "Общая эффективность потока", "Уровень энтропии судьбы", "Балл энтропии", "Толкование энтропии", "Сложность структуры", "Класс стабильности", "Индекс стабильности (Maintenance Entropy)", "Возраст ключевых развилок", "Периоды спада"),
    };

    let structure_str = format!("{:?}", rep.structure.structure);
    let proj_stem_str = rep.structure.projected_stem.map(|s| format!("{} (위치: {})", s.hangul(), rep.structure.projection_path.clone().unwrap_or_default())).unwrap_or_else(|| "—".to_string());
    
    let unpacker_str = data.entropy.unpacker_element.map(|el| el.hangul().to_string()).unwrap_or_else(|| "—".to_string());
    let bottleneck_str = data.qi_topology.bottleneck.map(|el| el.hangul().to_string()).unwrap_or_else(|| "—".to_string());
    
    let ent_lvl_str = format!("{:?}", data.entropy.level);
    
    let (comp_val, stab_val, maint_entropy_str, dec_nodes_str) = if let Some(comp) = &data.complexity {
        let nodes_str = comp.decision_nodes.iter().map(|age| format!("{}세", age)).collect::<Vec<_>>().join(", ");
        (comp.cyclomatic_complexity.to_string(), comp.stability_grade.clone(), format!("{:.2}", comp.entropy), nodes_str)
    } else {
        ("—".to_string(), "—".to_string(), "—".to_string(), "—".to_string())
    };

    s.push_str(&format!("### {}\n\n", diag_title));
    s.push_str(&format!("- **{}**: {}\n", struct_lbl, structure_str));
    s.push_str(&format!("- **{}**: {}\n", struct_sum_lbl, rep.structure.summary));
    s.push_str(&format!("- **{}**: {}\n", struct_desc_lbl, rep.structure.description));
    s.push_str(&format!("- **{}**: {}\n", struct_reasons_lbl, rep.structure.reasons.join(", ")));
    s.push_str(&format!("- **{}**: {}\n", projected_stem_lbl, proj_stem_str));
    s.push_str(&format!("- **{}**: {}\n", unpacker_lbl, unpacker_str));
    s.push_str(&format!("- **{}**: {}\n", bottleneck_lbl, bottleneck_str));
    s.push_str(&format!("- **{}**: {:.1}%\n", throughput_lbl, data.qi_topology.throughput * 100.0));
    s.push_str(&format!("- **{}**: {}\n", entropy_lbl, ent_lvl_str));
    s.push_str(&format!("- **{}**: {:.3} / 2.322\n", entropy_score_lbl, data.entropy.score));
    s.push_str(&format!("- **{}**: {}\n", entropy_desc_lbl, data.entropy.description));
    s.push_str(&format!("- **{}**: {}\n", complexity_lbl, comp_val));
    s.push_str(&format!("- **{}**: {}\n", grade_lbl, stab_val));
    s.push_str(&format!("- **{}**: {}\n", maintenance_entropy_lbl, maint_entropy_str));
    s.push_str(&format!("- **{}**: {}\n", decision_nodes_lbl, dec_nodes_str));
    s.push_str(&format!("- **{}**: {} times detected\n\n", crisis_lbl, data.crash_count));

    // Topology Nodes Detail Table
    let (top_title, top_node, top_cap, top_out) = match locale {
        Locale::Ko => ("오행 네트워크 유동 노드 상세", "오행", "대역폭 (Capacity)", "출력량 (Output)"),
        Locale::En => ("Qi Network Node Capacity Details", "Element", "Bandwidth (Capacity)", "Output"),
        Locale::Zh => ("五行网络流通节点详细", "五行", "大带宽 (Capacity)", "输出量 (Output)"),
        Locale::Ru => ("Подробные показатели пропускной способности узлов Ци", "Элемент", "Пропускная способность (Capacity)", "Выход (Output)"),
    };
    s.push_str(&format!("#### {}\n\n", top_title));
    s.push_str(&format!("| {} | {} | {} |\n", top_node, top_cap, top_out));
    s.push_str("| --- | --- | --- |\n");
    for node in &data.qi_topology.nodes {
        s.push_str(&format!("| {}({}) | {:.1} | {:.1} |\n", translate_saju_element(locale, node.element), node.element.hanja(), node.capacity, node.output));
    }
    s.push_str("\n");

    // Power Analysis (오행 및 십성 점수)
    let (power_title, dominant_el_lbl, dominant_tg_lbl, percentage_col, score_col) = match locale {
        Locale::Ko => ("오행 및 십성 세력 상세 분석 (Power Analysis)", "대표 오행", "대표 십성", "비율", "점수"),
        Locale::En => ("Element & Ten God Power Analysis", "Dominant Element", "Dominant Ten God", "Percentage", "Score"),
        Locale::Zh => ("五行与十神力量详细分析", "代表五行", "代表十神", "比例", "分数"),
        Locale::Ru => ("Подробный анализ сил Первоэлементов и Божеств", "Доминирующий элемент", "Доминирующее Божество", "Процент", "Балл"),
    };
    s.push_str(&format!("### {}\n\n", power_title));
    s.push_str(&format!("- **{}**: {}\n", dominant_el_lbl, translate_saju_element(locale, rep.power.dominant_element)));
    s.push_str(&format!("- **{}**: {}\n\n", dominant_tg_lbl, translate_saju_ten_god(locale, rep.power.dominant_ten_god)));

    // Elements Table
    let el_col = match locale {
        Locale::Ko => "오행 (Element)",
        Locale::En => "Element",
        Locale::Zh => "五行",
        Locale::Ru => "Первоэлемент",
    };
    s.push_str(&format!("| {} | {} | {} |\n", el_col, percentage_col, score_col));
    s.push_str("| --- | --- | --- |\n");
    for &(el, pct, score) in &rep.power.element_scores {
        s.push_str(&format!("| {}({}) | {:.1}% | {:.1} |\n", translate_saju_element(locale, el), el.hanja(), pct, score));
    }
    s.push_str("\n");

    // Ten Gods Table
    let tg_col = match locale {
        Locale::Ko => "십성 (Ten God)",
        Locale::En => "Ten God",
        Locale::Zh => "十神",
        Locale::Ru => "Божество",
    };
    s.push_str(&format!("| {} | {} | {} |\n", tg_col, percentage_col, score_col));
    s.push_str("| --- | --- | --- |\n");
    for &(tg, pct, score) in &rep.power.ten_god_scores {
        s.push_str(&format!("| {} | {:.1}% | {:.1} |\n", translate_saju_ten_god(locale, tg), pct, score));
    }
    s.push_str("\n");

    // Lints
    let (lints_title, lint_code_col, lint_msg_col, lint_adv_col) = match locale {
        Locale::Ko => ("사주 진단 및 조언 (Diagnostics & Lints)", "코드", "진단 및 메시지", "조언"),
        Locale::En => ("Saju Diagnostics & Advice", "Code", "Message", "Advice"),
        Locale::Zh => ("八字诊断与建议", "代码", "诊断信息", "建议"),
        Locale::Ru => ("Диагностика и советы Бацзы", "Код", "Сообщение", "Совет"),
    };
    s.push_str(&format!("### {}\n\n", lints_title));
    if !data.lints.is_empty() {
        s.push_str(&format!("| {} | {} | {} |\n", lint_code_col, lint_msg_col, lint_adv_col));
        s.push_str("| --- | --- | --- |\n");
        for lint in &data.lints {
            let sev_str = match lint.severity {
                eon_saju::engine::linter::LintSeverity::Error => "🚨 Error",
                eon_saju::engine::linter::LintSeverity::Warning => "⚠️ Warning",
                eon_saju::engine::linter::LintSeverity::Info => "ℹ️ Info",
            };
            s.push_str(&format!("| **{} [{}]** | {} | {} |\n", lint.code, sev_str, lint.message, lint.advice));
        }
    } else {
        s.push_str("—\n");
    }
    s.push_str("\n");

    // Voids
    let (voids_title, void_xun_lbl, void_br_lbl) = match locale {
        Locale::Ko => ("공망 분석 (Void / Emptiness)", "순중 (Xun)", "공망 지지"),
        Locale::En => ("Void Analysis (Emptiness)", "Xun Group", "Void Branches"),
        Locale::Zh => ("旬空分析 (Void)", "旬群", "空亡地支"),
        Locale::Ru => ("Анализ Пустоты (Void)", "Группа Сюнь", "Пустые Земные Ветви"),
    };
    s.push_str(&format!("### {}\n\n", voids_title));
    s.push_str(&format!("- **{}**: {}\n", void_xun_lbl, rep.voids.xun_group));
    s.push_str(&format!("- **{}**: {}({}), {}({})\n", 
        void_br_lbl, 
        translate_saju_branch(locale, rep.voids.void_branches[0]), rep.voids.void_branches[0].hanja(),
        translate_saju_branch(locale, rep.voids.void_branches[1]), rep.voids.void_branches[1].hanja()
    ));
    if !rep.voids.mapped_voids.is_empty() {
        let void_details_title = match locale {
            Locale::Ko => "세부 공망 분석",
            Locale::En => "Void Details",
            Locale::Zh => "详细空亡分析",
            Locale::Ru => "Подробный анализ Пустоты",
        };
        s.push_str(&format!("\n**{}**:\n", void_details_title));
        for mv in &rep.voids.mapped_voids {
            let pos_str = match mv.position.as_str() {
                "연주" | "Year" => t(locale, TK::SajuYearPillar),
                "월주" | "Month" => t(locale, TK::SajuMonthPillar),
                "일주" | "Day" => t(locale, TK::SajuDayPillar),
                "시주" | "Hour" => t(locale, TK::SajuHourPillar),
                _ => &mv.position,
            };
            let label_tg = translate_saju_ten_god(locale, mv.ten_god);
            let void_summary = match locale {
                Locale::Ko => format!("{}에 위치한 {} 공망", pos_str, label_tg),
                Locale::Zh => format!("位于{}的{}空亡", pos_str, label_tg),
                Locale::En => format!("{} Void located in {}", label_tg, pos_str),
                Locale::Ru => format!("Пустота {} в {}", label_tg, pos_str),
            };
            let trans_desc = translate_saju_void_desc(locale, &mv.position);
            let void_desc = if trans_desc.is_empty() { &mv.description } else { trans_desc };
            s.push_str(&format!("- **{} {}({}) [{}]**: {} - *{}*\n", 
                pos_str, translate_saju_branch(locale, mv.branch), mv.branch.hanja(), label_tg, void_summary, void_desc));
        }
    }
    s.push_str("\n");

    // Relationships (합충형해)
    let (rel_title, rel_type_col, rel_name_col, rel_pos_col, rel_desc_col) = match locale {
        Locale::Ko => ("합충형해 분석 (Harmony & Clashes)", "종류", "관계명", "위치", "해석"),
        Locale::En => ("Harmony & Clashes Analysis", "Type", "Name", "Positions", "Description"),
        Locale::Zh => ("合冲刑害分析", "种类", "关系名", "位置", "解析"),
        Locale::Ru => ("Анализ Слияний и Столкновений", "Тип", "Название", "Столпы", "Описание"),
    };
    s.push_str(&format!("### {}\n\n", rel_title));
    if !rep.relationships.mapped_relationships.is_empty() {
        s.push_str(&format!("| {} | {} | {} | {} |\n", rel_type_col, rel_name_col, rel_pos_col, rel_desc_col));
        s.push_str("| --- | --- | --- | --- |\n");
        for rel in &rep.relationships.mapped_relationships {
            let rel_type_trans = match rel.relation_type.as_str() {
                "합" | "Combo" | "Harmony" => match locale {
                    Locale::Ko => "합",
                    Locale::Zh => "合",
                    Locale::En => "Harmony",
                    Locale::Ru => "Слияние",
                },
                "충" | "Clash" => match locale {
                    Locale::Ko => "충",
                    Locale::Zh => "冲",
                    Locale::En => "Clash",
                    Locale::Ru => "Столкновение",
                },
                "형" | "Punishment" => match locale {
                    Locale::Ko => "형",
                    Locale::Zh => "刑",
                    Locale::En => "Punishment",
                    Locale::Ru => "Наказание",
                },
                "해" | "Harm" => match locale {
                    Locale::Ko => "해",
                    Locale::Zh => "害",
                    Locale::En => "Harm",
                    Locale::Ru => "Вред",
                },
                "파" | "Destruction" => match locale {
                    Locale::Ko => "파",
                    Locale::Zh => "破",
                    Locale::En => "Destruction",
                    Locale::Ru => "Разрушение",
                },
                "원진" | "Resentment" => match locale {
                    Locale::Ko => "원진",
                    Locale::Zh => "怨嗔",
                    Locale::En => "Resentment",
                    Locale::Ru => "Вражда",
                },
                _ => &rel.relation_type,
            };
            let pos_str = rel.positions.iter().map(|p| translate_saju_tag_str(locale, p)).collect::<Vec<_>>().join("-");
            let summary_trans = translate_saju_tag_str(locale, &rel.summary);
            let desc_trans = translate_saju_tag_str(locale, &rel.description);
            s.push_str(&format!("| {} | **{}** | {} | {} ({}) |\n", 
                rel_type_trans, translate_saju_relation_str(locale, &rel.name), pos_str, summary_trans, desc_trans));
        }
    } else {
        s.push_str("—\n");
    }
    s.push_str("\n");

    // Spirit Markers (신살 상세)
    let (spirit_title, spirit_pos_col, spirit_name_col, spirit_desc_col) = match locale {
        Locale::Ko => ("신살 상세 해설 (Spirit Markers Detail)", "기둥 위치", "신살명", "해석 및 설명"),
        Locale::En => ("Spirit Markers Detailed Descriptions", "Pillar Position", "Spirit Name", "Interpretation"),
        Locale::Zh => ("神煞详细解析", "柱位", "神煞名", "说明"),
        Locale::Ru => ("Подробный анализ Символических Звезд", "Положение", "Название", "Описание"),
    };
    s.push_str(&format!("### {}\n\n", spirit_title));
    if !rep.spirit_markers.mapped_markers.is_empty() {
        s.push_str(&format!("| {} | {} | {} |\n", spirit_pos_col, spirit_name_col, spirit_desc_col));
        s.push_str("| --- | --- | --- |\n");
        for m in &rep.spirit_markers.mapped_markers {
            let pos_str = match m.position {
                eon_saju::analysis::spirit_markers::PillarPosition::Year => t(locale, TK::SajuYearPillar),
                eon_saju::analysis::spirit_markers::PillarPosition::Month => t(locale, TK::SajuMonthPillar),
                eon_saju::analysis::spirit_markers::PillarPosition::Day => t(locale, TK::SajuDayPillar),
                eon_saju::analysis::spirit_markers::PillarPosition::Hour => t(locale, TK::SajuHourPillar),
            };
            let marker_label = match locale {
                Locale::Ko => format!("{} ({})", m.marker.hangul(), m.marker.hanja()),
                Locale::Zh => m.marker.hanja().to_string(),
                _ => translate_saju_spirit_marker_name(locale, m.marker).to_string(),
            };
            let marker_summary = translate_saju_spirit_marker_name(locale, m.marker);
            let marker_desc = translate_spirit_desc(locale, m.marker, m.position, &m.description);
            s.push_str(&format!("| {} | **{}** | {} - *{}* |\n", pos_str, marker_label, marker_summary, marker_desc));
        }
    } else {
        s.push_str("—\n");
    }
    s.push_str("\n");

    // ── Phase 5: Aux Shinsals (보조 신살) ───────────────────────────────────
    let (aux_title, aux_name_col, aux_basis_col, aux_result_col) = match locale {
        Locale::Ko => ("보조 신살 목록 (Auxiliary Spirit Markers)", "신살명", "기준", "결과 지지"),
        Locale::En => ("Auxiliary Spirit Markers", "Marker Name", "Basis", "Result Branch"),
        Locale::Zh => ("辅助神煞列表", "神煞名", "基准", "结果地支"),
        Locale::Ru => ("Вспомогательные Символические Звезды", "Название", "Основа", "Результат"),
    };
    if !rep.spirit_markers.aux_shinsals.is_empty() {
        s.push_str(&format!("#### {}\n\n", aux_title));
        s.push_str(&format!("| {} | {} | {} |\n", aux_name_col, aux_basis_col, aux_result_col));
        s.push_str("| --- | --- | --- |\n");
        for (name, basis, result) in &rep.spirit_markers.aux_shinsals {
            s.push_str(&format!("| {} | {} | {} |\n", 
                translate_saju_tag_str(locale, name), 
                translate_saju_tag_str(locale, basis), 
                translate_saju_tag_str(locale, result)
            ));
        }
        s.push_str("\n");
    }

    // Vulnerability/Fuzzer & Load Diagnostics
    let (fuzzer_title, load_title) = match locale {
        Locale::Ko => ("운명 크래시 분석 (Destiny Fuzzer)", "인생 부하 진단 (Karma Load Balancer)"),
        Locale::En => ("Destiny Fuzzer (Vulnerability Analysis)", "Karma Load Balancer Diagnostics"),
        Locale::Zh => ("命运漏洞分析 (Fuzzer)", "人生负荷均衡诊断"),
        Locale::Ru => ("Анализ Уязвимостей Судьбы", "Диагностика Кармической Нагрузки"),
    };
    s.push_str(&format!("### {}\n\n", fuzzer_title));
    if let Some(fuzz) = &data.vulnerability_report {
        let total_crashes_lbl = match locale {
            Locale::Ko => "발견된 총 치명적 운세 조합(Crash) 수",
            Locale::En => "Total Critical Crashes Detected",
            Locale::Zh => "发现的致命运势组合数 (Total Crashes)",
            Locale::Ru => "Общее количество обнаруженных сбоев (Crashes)",
        };
        s.push_str(&format!("- **{}**: {}\n\n", total_crashes_lbl, fuzz.total_crashes));
        for vuln in &fuzz.critical_vectors {
            let vuln_type = translate_saju_tag_str(locale, &vuln.vulnerability_type);
            let major_gz = match locale {
                Locale::Ko => format!("{}({})", vuln.vector.major.hanja(), vuln.vector.major.hangul()),
                Locale::Zh => vuln.vector.major.hanja().to_string(),
                _ => translate_saju_ganzi(locale, &vuln.vector.major),
            };
            let yearly_gz = match locale {
                Locale::Ko => format!("{}({})", vuln.vector.yearly.hanja(), vuln.vector.yearly.hangul()),
                Locale::Zh => vuln.vector.yearly.hanja().to_string(),
                _ => translate_saju_ganzi(locale, &vuln.vector.yearly),
            };
            let tags_translated: Vec<String> = vuln.tags.iter().map(|tag| translate_saju_tag_str(locale, tag)).collect();
            let tags_list = tags_translated.join(", ");
            s.push_str(&format!("- **[⚠️ {}]** (Score: {:.1}): Major: {}, Yearly: {} (Tags: {})\n", 
                vuln_type, vuln.crash_score, major_gz, yearly_gz, tags_list));
        }
    } else {
        s.push_str("—\n");
    }
    s.push_str("\n");

    s.push_str(&format!("### {}\n\n", load_title));
    if !data.load_diagnostics.is_empty() {
        for diag in &data.load_diagnostics {
            let status_str = match diag.status {
                eon_saju::engine::load_balancer::TrafficStatus::Idle => match locale {
                    Locale::Ko => "평온 (Idle)",
                    Locale::En => "Idle",
                    Locale::Zh => "平稳 (Idle)",
                    Locale::Ru => "Покой (Idle)",
                },
                eon_saju::engine::load_balancer::TrafficStatus::Normal => match locale {
                    Locale::Ko => "보통 (Normal)",
                    Locale::En => "Normal",
                    Locale::Zh => "正常 (Normal)",
                    Locale::Ru => "Нормально (Normal)",
                },
                eon_saju::engine::load_balancer::TrafficStatus::Overloaded => match locale {
                    Locale::Ko => "오버로드 (Overload)",
                    Locale::En => "Overload",
                    Locale::Zh => "过载 (Overload)",
                    Locale::Ru => "Перегрузка (Overload)",
                },
                eon_saju::engine::load_balancer::TrafficStatus::SystemDown => match locale {
                    Locale::Ko => "다운 (System Down)",
                    Locale::En => "System Down",
                    Locale::Zh => "系统故障 (System Down)",
                    Locale::Ru => "Системный сбой (System Down)",
                },
            };
            let age_str = crate::i18n::format_age(locale, diag.age as i32);
            let (reason_desc, strategy_desc) = translate_saju_load_balancer(locale, &diag.reason, &diag.strategy);
            s.push_str(&format!("- **{} [{}]**: *{}* -> Advice: {}\n", age_str, status_str, reason_desc, strategy_desc));
        }
    } else {
        s.push_str("—\n");
    }
    s.push_str("\n");

    // Great Luck
    let (luck_title, age_lbl) = match locale {
        Locale::Ko => ("대운 흐름 (Major Luck)", "나이"),
        Locale::En => ("Major Luck Cycles", "Age"),
        Locale::Zh => ("大运起伏", "年龄"),
        Locale::Ru => ("Столпы Удачи", "Возраст"),
    };

    s.push_str(&format!("### {}\n\n", luck_title));
    if let Some(ml) = &rep.major_luck {
        for cycle in &ml.cycles {
            let cycle_ganzi_str = match locale {
                Locale::Ko => format!("{}{}", cycle.ganzi.stem.hangul(), cycle.ganzi.branch.hangul()),
                _ => translate_saju_ganzi(locale, &cycle.ganzi),
            };
            let luck_suffix = match locale {
                Locale::Ko => " 대운",
                Locale::En => " Major Luck",
                Locale::Zh => " 大运",
                Locale::Ru => " Столп Удачи",
            };
            let stem_god_str = translate_saju_ten_god(locale, cycle.stem_god);
            let branch_god_str = translate_saju_ten_god(locale, cycle.branch_god);
            s.push_str(&format!("- **{} {} ~ {}**: {}{} ({}/{})\n", 
                age_lbl, cycle.start_age, cycle.end_age, 
                cycle_ganzi_str, luck_suffix,
                stem_god_str, branch_god_str
            ));
        }
    } else {
        s.push_str("—\n");
    }
    s.push_str("\n");

    // Supplementary Pillars
    let (supp_title, taewon_lbl, myeonggung_lbl, shingung_lbl, supp_pillar_col, supp_level_col, supp_desc_col, supp_reason_col) = match locale {
        Locale::Ko => ("보조 기둥 분석 (Supplementary Pillars)", "태원 (胎元)", "명궁 (命宮)", "신궁 (身宮)", "기둥", "구분", "해석", "판단 근거"),
        Locale::En => ("Supplementary Pillars Analysis", "Taewon (Conception)", "Myeonggung (Ascendant)", "Shingung (Body)", "Pillar", "Level", "Interpretation", "Reasons"),
        Locale::Zh => ("辅助柱位分析", "胎元", "命宫", "身宫", "柱位", "等级", "解析", "依据"),
        Locale::Ru => ("Вспомогательные Столпы", "Тэвон (Зачатие)", "Мёнгун (Асцендент)", "Шингун (Тело)", "Столп", "Уровень", "Толкование", "Причины"),
    };

    s.push_str(&format!("### {}\n\n", supp_title));
    let format_supp_pillar = |loc, stem: eon_saju::core::stem::HeavenlyStem, branch: eon_saju::core::branch::EarthlyBranch| {
        match loc {
            Locale::Ko => format!("{}{}({}{})", translate_saju_stem(loc, stem), translate_saju_branch(loc, branch), stem.hanja(), branch.hanja()),
            Locale::Zh => format!("{}{}", stem.hanja(), branch.hanja()),
            _ => format!("{}-{}({}{})", translate_saju_stem(loc, stem), translate_saju_branch(loc, branch), stem.hanja(), branch.hanja()),
        }
    };
    s.push_str(&format!("- **{}**: {}\n", taewon_lbl, format_supp_pillar(locale, rep.supplementary_pillars.taewon.stem, rep.supplementary_pillars.taewon.branch)));
    s.push_str(&format!("- **{}**: {}\n", myeonggung_lbl, format_supp_pillar(locale, rep.supplementary_pillars.myeonggung.stem, rep.supplementary_pillars.myeonggung.branch)));
    s.push_str(&format!("- **{}**: {}\n\n", shingung_lbl, format_supp_pillar(locale, rep.supplementary_pillars.shingung.stem, rep.supplementary_pillars.shingung.branch)));

    if !rep.supplementary_pillars.interpretations.is_empty() {
        s.push_str(&format!("| {} | {} | {} | {} |\n", supp_pillar_col, supp_level_col, supp_desc_col, supp_reason_col));
        s.push_str("| --- | --- | --- | --- |\n");
        for interp in &rep.supplementary_pillars.interpretations {
            let lvl_str = match interp.level {
                eon_saju::analysis::supplementary_pillars::InterpretationLevel::Auspicious => match locale {
                    Locale::Ko => "🟢 길(吉)",
                    Locale::Zh => "🟢 吉",
                    Locale::En => "🟢 Auspicious",
                    Locale::Ru => "🟢 Благоприятно",
                },
                eon_saju::analysis::supplementary_pillars::InterpretationLevel::Caution => match locale {
                    Locale::Ko => "🔴 흉(凶)",
                    Locale::Zh => "🔴 凶",
                    Locale::En => "🔴 Caution",
                    Locale::Ru => "🔴 Предупреждение",
                },
                eon_saju::analysis::supplementary_pillars::InterpretationLevel::Neutral => match locale {
                    Locale::Ko => "⚪ 평(平)",
                    Locale::Zh => "⚪ 平",
                    Locale::En => "⚪ Neutral",
                    Locale::Ru => "⚪ Нейтрально",
                },
            };
            let pillar_name_trans = match interp.pillar_name.as_str() {
                "태원" => taewon_lbl,
                "명궁" => myeonggung_lbl,
                "신궁" => shingung_lbl,
                _ => &interp.pillar_name,
            };
            let summary_trans = translate_saju_tag_str(locale, &interp.summary);
            let desc_trans = translate_saju_tag_str(locale, &interp.description);
            let reasons_trans: Vec<String> = interp.reasons.iter().map(|r| translate_saju_reason(locale, r)).collect();
            let reasons_str = reasons_trans.join(", ");
            s.push_str(&format!("| {} | {} | **{}** - {} | {} |\n", pillar_name_trans, lvl_str, summary_trans, desc_trans, reasons_str));
        }
        s.push_str("\n");
    }

    // Golden Time
    let (golden_title, golden_range_lbl, golden_score_lbl) = match locale {
        Locale::Ko => ("인생 골든 타임 분석 (Golden Time)", "전성기 구간", "평균 점수"),
        Locale::En => ("Life Golden Time Analysis", "Golden Age Range", "Average Score"),
        Locale::Zh => ("人生黄金时期分析", "黄金年龄段", "平均分数"),
        Locale::Ru => ("Золотой период жизни (Golden Time)", "Золотой возраст", "Средний балл"),
    };
    if let Some(gt) = &rep.golden_time {
        let age_range_str = match locale {
            Locale::Ko => format!("{}세 ~ {}세", gt.start_age, gt.end_age),
            Locale::Zh => format!("{}岁 ~ {}岁", gt.start_age, gt.end_age),
            Locale::Ru => format!("{} ~ {} лет", gt.start_age, gt.end_age),
            _ => format!("Age {} ~ {}", gt.start_age, gt.end_age),
        };
        let golden_desc = match locale {
            Locale::Ko => format!("{}세부터 {}세까지 가장 운의 밀도가 높은 골든 타임입니다.", gt.start_age, gt.end_age),
            Locale::En => format!("The period from age {} to {} is the golden time with the highest density of luck.", gt.start_age, gt.end_age),
            Locale::Zh => format!("{}岁到{}岁是好运密度最高的黄金时期。", gt.start_age, gt.end_age),
            Locale::Ru => format!("Период с {} по {} лет — это золотое время с наибольшей плотностью удачи.", gt.start_age, gt.end_age),
        };
        s.push_str(&format!("### {}\n\n", golden_title));
        s.push_str(&format!("- **{}**: {}\n", golden_range_lbl, age_range_str));
        s.push_str(&format!("- **{}**: {:.2} / 100\n", golden_score_lbl, gt.average_score));
        s.push_str(&format!("- **{}**\n\n", golden_desc));
    }

    // VM Summary
    let vm_title = match locale {
        Locale::Ko => "인생 가상머신(VM) 분석 요약",
        Locale::En => "Life VM Analysis Summary",
        Locale::Zh => "人生虚拟机 (VM) 分析摘要",
        Locale::Ru => "Сводка анализа виртуальной машины (VM) жизни",
    };
    if let Some(ref vm_sum) = rep.vm_summary {
        s.push_str(&format!("### {}\n\n", vm_title));
        s.push_str(&format!("{}\n\n", vm_sum));
    }

    // 10년 대운 주기 (Major Luck Cycles)
    if let Some(ref ml) = rep.major_luck {
        let (ml_title, ml_dir, ml_age, ml_cycle, ml_ganzi, ml_gods, ml_start_date) = match locale {
            Locale::Ko => ("10년 대운 주기 (10-Year Major Luck Cycles)", "대운 순행/역행", "대운수 (시작 나이)", "대운 주기", "간지 (GanZi)", "십성 (Ten Gods)", "대운 시작 날짜 (초정밀 교운기)"),
            Locale::En => ("10-Year Major Luck Cycles", "Direction", "Luck Number (Start Age)", "Cycle", "GanZi", "Ten Gods", "Precise Start Date"),
            Locale::Zh => ("十年大运周期", "大运顺逆", "大运数 (起运年龄)", "大运区间", "干支", "十神", "起运日期"),
            Locale::Ru => ("10-летние периоды столпов удачи", "Направление удачи", "Число удачи (Возраст)", "Период", "Гань-Чжи", "Божества", "Точная дата начала"),
        };

        s.push_str(&format!("### {}\n\n", ml_title));
        let dir_str = match ml.direction {
            eon_saju::analysis::major_luck::LuckDirection::Forward => match locale {
                Locale::Ko => "순행 (Forward)",
                Locale::En => "Forward",
                Locale::Zh => "顺行",
                Locale::Ru => "Прямое направление",
            },
            eon_saju::analysis::major_luck::LuckDirection::Reverse => match locale {
                Locale::Ko => "역행 (Reverse)",
                Locale::En => "Reverse",
                Locale::Zh => "逆行",
                Locale::Ru => "Обратное направление",
            },
        };
        s.push_str(&format!("- **{}**: {}\n", ml_dir, dir_str));
        let ml_age_val = match locale {
            Locale::Ko => format!("{}세 ({}개월 {}일)", ml.start_age, ml.start_months, ml.start_days),
            Locale::Zh => format!("{}岁 ({}个月 {}天)", ml.start_age, ml.start_months, ml.start_days),
            Locale::Ru => format!("{} лет ({} мес. {} дн.)", ml.start_age, ml.start_months, ml.start_days),
            _ => format!("Age {} ({} months {} days)", ml.start_age, ml.start_months, ml.start_days),
        };
        s.push_str(&format!("- **{}**: {}\n\n", ml_age, ml_age_val));

        s.push_str(&format!("| {} | {} | {} | {} |\n", ml_cycle, ml_ganzi, ml_gods, ml_start_date));
        s.push_str("| --- | --- | --- | --- |\n");
        for cycle in &ml.cycles {
            let cycle_str = match locale {
                Locale::Ko => format!("{}세 ~ {}세", cycle.start_age, cycle.end_age),
                Locale::Zh => format!("{}岁 ~ {}岁", cycle.start_age, cycle.end_age),
                Locale::Ru => format!("{} ~ {} лет", cycle.start_age, cycle.end_age),
                _ => format!("Age {} ~ {}", cycle.start_age, cycle.end_age),
            };
            let ganzi_str = match locale {
                Locale::Ko => format!("{}({})", cycle.ganzi.hangul(), cycle.ganzi.hanja()),
                Locale::Zh => cycle.ganzi.hanja().to_string(),
                _ => format!("{}({})", translate_saju_ganzi(locale, &cycle.ganzi), cycle.ganzi.hanja()),
            };
            let gods_str = format!("{}/{}", 
                translate_saju_ten_god(locale, cycle.stem_god),
                translate_saju_ten_god(locale, cycle.branch_god)
            );
            let date_str = cycle.start_date.format("%Y-%m-%d").to_string();
            s.push_str(&format!("| {} | {} | {} | {} |\n", cycle_str, ganzi_str, gods_str, date_str));
        }
        s.push_str("\n");
    }

    // 100-Year Life Path Simulation Table
    let (sim_title, sim_age, sim_year, sim_seun, sim_daeun, sim_score, sim_wealth, sim_career, sim_academic, sim_health, sim_vol, sim_trans, sim_tags) = match locale {
        Locale::Ko => ("100년 인생 시뮬레이션 상세 경로 (100-Year Life Path Simulation)", "나이", "연도", "세운", "대운", "종합 점수", "재물운", "직업운", "학업운", "건강운", "변동성", "교운기", "주요 특징"),
        Locale::En => ("100-Year Life Path Simulation Detail", "Age", "Year", "Seun", "Daeun", "Score", "Wealth", "Career", "Academic", "Health", "Volatility", "Transition?", "Tags"),
        Locale::Zh => ("100年人生模拟详细路径", "年龄", "年份", "岁运", "大运", "综合得分", "财运", "事业运", "学业运", "健康运", "波动性", "交运期?", "主要特征"),
        Locale::Ru => ("Подробный путь 100-летней симуляции жизни", "Возраст", "Год", "Сеун", "Дэун", "Балл", "Богатство", "Карьера", "Учеба", "Здоровье", "Волатильность", "Смена управителя?", "Метки"),
    };

    s.push_str(&format!("### {}\n\n", sim_title));
    s.push_str(&format!("| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n", 
        sim_age, sim_year, sim_seun, sim_daeun, sim_score, sim_wealth, sim_career, sim_academic, sim_health, sim_vol, sim_trans, sim_tags));
    s.push_str("| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |\n");

    for score in &rep.timeline {
        let frame_opt = rep.simulation_frames.iter().find(|f| f.age == score.age);
        let seun_str = frame_opt.map(|f| {
            match locale {
                Locale::Ko => format!("{}{}", f.ganzi.stem.hangul(), f.ganzi.branch.hangul()),
                Locale::Zh => f.ganzi.hanja().to_string(),
                _ => translate_saju_ganzi(locale, &f.ganzi),
            }
        }).unwrap_or_else(|| "—".to_string());

        let daeun_str = frame_opt.map(|f| {
            match locale {
                Locale::Ko => format!("{}{}", f.major_ganzi.stem.hangul(), f.major_ganzi.branch.hangul()),
                Locale::Zh => f.major_ganzi.hanja().to_string(),
                _ => translate_saju_ganzi(locale, &f.major_ganzi),
            }
        }).unwrap_or_else(|| "—".to_string());

        let tags_str = frame_opt.map(|f| {
            let translated_tags: Vec<String> = f.tags_as_strings().iter().map(|tag| translate_saju_tag_str(locale, tag)).collect();
            translated_tags.join(", ")
        }).unwrap_or_else(|| "—".to_string());

        let transition_str = if score.is_transition_period {
            match locale {
                Locale::Ko => "Yes",
                Locale::Zh => "是",
                Locale::Ru => "Да",
                _ => "Yes",
            }
        } else {
            match locale {
                Locale::Ko => "No",
                Locale::Zh => "否",
                Locale::Ru => "Нет",
                _ => "No",
            }
        };
        
        s.push_str(&format!("| {} | {} | {} | {} | {:.1} | {:.1} | {:.1} | {:.1} | {:.1} | {:.1} | {} | {} |\n",
            score.age, score.year, seun_str, daeun_str, score.total_score, score.wealth_score, score.career_score, score.academic_score, score.health_score, score.volatility_index, transition_str, tags_str));
    }
    s.push_str("\n");

    s
}

// Format inner Vedic content (without global header/birth info)
fn format_vedic_inner(data: &VedicAnalysisOutput, locale: Locale) -> String {
    let mut s = String::new();

    // Planetary Positions
    let (planets_title, planet_col, degree_col, rasi_col, house_col, nak_col, pada_col, lord_col, retro_col, combust_col) = match locale {
        Locale::Ko => ("행성 배치 정보 (Planetary Positions)", "행성", "도수", "사인 (Sign)", "하우스", "나크샤트라", "파다(Pada)", "지배성", "역행", "태비"),
        Locale::En => ("Planetary Positions", "Planet", "Degree", "Sign", "House", "Nakshatra", "Pada", "Sign Lord", "Retro", "Combust"),
        Locale::Zh => ("星体位置", "星体", "度数", "星座", "宫位", "星宿", "步 (Pada)", "定位星", "逆行", "焦伤"),
        Locale::Ru => ("Положения планет", "Планета", "Градус", "Знак", "Дом", "Накшатра", "Пада", "Управитель", "Ретро", "Сожж"),
    };

    s.push_str(&format!("### {}\n\n", planets_title));
    s.push_str(&format!("| {} | {} | {} | {} | {} | {} | {} | {} | {} |\n", 
        planet_col, degree_col, rasi_col, house_col, nak_col, pada_col, retro_col, combust_col, lord_col));
    s.push_str("| --- | --- | --- | --- | --- | --- | --- | --- | --- |\n");

    let format_pos = |pos: &eon_vedic::core::chart::VedicPosition, is_asc: bool| -> String {
        let p_name = if is_asc {
            translate_planet(locale, VedicPlanet::Ascendant)
        } else {
            translate_planet(locale, pos.planet)
        };
        
        let deg_within_sign = pos.sidereal_deg % 30.0;
        let deg_floor = deg_within_sign.floor() as i32;
        let min_val = ((deg_within_sign - deg_floor as f64) * 60.0).round() as i32;
        let deg_str = format!("{:02}° {:02}'", deg_floor, min_val);
        
        let r_name = crate::i18n::rasi_name(locale, pos.rasi);
        let nak_name_val = nakshatra_name(pos.nakshatra);
        let retro_str = if pos.is_retrograde { "Yes" } else { "No" };
        let combust_str = if pos.is_combust { "Yes" } else { "No" };
        
        let lord = match pos.rasi {
            1 | 8 => VedicPlanet::Mars,
            2 | 7 => VedicPlanet::Venus,
            3 | 6 => VedicPlanet::Mercury,
            4 => VedicPlanet::Moon,
            5 => VedicPlanet::Sun,
            9 | 12 => VedicPlanet::Jupiter,
            10 | 11 => VedicPlanet::Saturn,
            _ => VedicPlanet::Sun,
        };
        let lord_name = translate_planet(locale, lord);
        
        format!("| **{}** | {} | {} | {} | {} | {} | {} | {} | {} |\n", 
            p_name, deg_str, r_name, pos.house_index, nak_name_val, pos.pada, retro_str, combust_str, lord_name)
    };

    // Add Lagna first
    s.push_str(&format_pos(&data.chart.ascendant, true));
    
    // Add planets
    for p in &data.chart.planets {
        s.push_str(&format_pos(p, false));
    }
    s.push_str("\n");

    // Metrics Summary
    let (metrics_title, strength_lbl, dasha_lbl, nak_lbl, sade_sati_lbl) = match locale {
        Locale::Ko => ("베딕 종합 분석 결과", "종합 세력 점수", "현재 대운 초점", "나크샤트라 요약", "사데 사티 (Sade Sati)"),
        Locale::En => ("Vedic Analysis Summary", "Overall Strength Score", "Current Dasha Focus", "Nakshatra Summary", "Sade Sati Phase"),
        Locale::Zh => ("吠陀分析概要", "综合能量分数", "当前运势焦点", "星宿扼要", "萨德萨提 (Sade Sati)"),
        Locale::Ru => ("Ведический аналитический обзор", "Общий балл силы", "Текущий фокус Даши", "Сводка Накшатры", "Саде Сати"),
    };

    let ss_phase_str = match data.report.sade_sati {
        eon_vedic::analysis::gochara::SadeSatiPhase::Rising => match locale {
            Locale::Ko => "상승기 (Rising) - 토성이 달 기준 12하우스 진입",
            Locale::En => "Rising - Saturn in 12th from Moon",
            Locale::Zh => "上升期 (Rising) - 土星进入月亮前一宫",
            Locale::Ru => "Начало (Rising) - Сатурн во 12-м доме от Луны",
        },
        eon_vedic::analysis::gochara::SadeSatiPhase::Peak => match locale {
            Locale::Ko => "절정기 (Peak) - 토성이 달과 합",
            Locale::En => "Peak - Saturn conjunct natal Moon",
            Locale::Zh => "巅峰期 (Peak) - 土星与月亮同宫",
            Locale::Ru => "Пик (Peak) - Сатурн в соединении с Луной",
        },
        eon_vedic::analysis::gochara::SadeSatiPhase::Setting => match locale {
            Locale::Ko => "하강기 (Setting) - 토성이 달 기준 2하우스 진입",
            Locale::En => "Setting - Saturn in 2nd from Moon",
            Locale::Zh => "衰退期 (Setting) - 土星进入月亮后一宫",
            Locale::Ru => "Завершение (Setting) - Сатурн во 2-м доме от Луны",
        },
        eon_vedic::analysis::gochara::SadeSatiPhase::None => match locale {
            Locale::Ko => "해당 없음 (None)",
            Locale::En => "None active",
            Locale::Zh => "无",
            Locale::Ru => "Неактивен",
        },
    };

    s.push_str(&format!("### {}\n\n", metrics_title));
    s.push_str(&format!("- **{}**: {:.2}\n", strength_lbl, data.report.overall_strength_score));
    s.push_str(&format!("- **{}**: {}\n", dasha_lbl, data.report.dasha_focus));
    s.push_str(&format!("- **{}**: {}\n", nak_lbl, data.report.nakshatra_info));
    let ayanamsa_lbl = match locale {
        Locale::Ko => "아야남사 (Ayanamsa)",
        Locale::En => "Ayanamsa",
        Locale::Zh => "岁差 (Ayanamsa)",
        Locale::Ru => "Айанамша",
    };
    let deg_floor = data.chart.ayanamsa.floor() as i32;
    let min_val = ((data.chart.ayanamsa - deg_floor as f64) * 60.0).round() as i32;
    s.push_str(&format!("- **{}**: {}\n", sade_sati_lbl, ss_phase_str));
    s.push_str(&format!("- **{}**: {}° {:02}' (Lahiri)\n\n", ayanamsa_lbl, deg_floor, min_val));

    // Ashtakavarga Points (SAV)
    s.push_str("### 3.3 아슈타카바르가 SAV 점수 (Sarvashtakavarga Points)\n\n");
    s.push_str("| House (하우스) | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 |\n");
    s.push_str("| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |\n");
    s.push_str(&format!("| **SAV** | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n\n", 
        data.chart.sav.points[0], data.chart.sav.points[1], data.chart.sav.points[2], data.chart.sav.points[3],
        data.chart.sav.points[4], data.chart.sav.points[5], data.chart.sav.points[6], data.chart.sav.points[7],
        data.chart.sav.points[8], data.chart.sav.points[9], data.chart.sav.points[10], data.chart.sav.points[11]
    ));

    // Jaimini Karakas & Special Lagnas
    s.push_str("### 3.4 Jaimini Chara Karakas & Special Lagnas\n\n");
    s.push_str("**Chara Karaka Assignments**:\n");
    for k in &data.chart.karakas {
        s.push_str(&format!("- **{}**: {} ({:.2}°)\n", 
            format_karaka_role(&k.role), translate_planet(locale, k.planet), k.degree_in_rasi));
    }
    s.push_str("\n**Arudha Padas**:\n");
    for ap in &data.chart.arudha_padas {
        s.push_str(&format!("- **{}**: House {} ({})\n", ap.name, ap.house, rasi_name(locale, ap.rasi)));
    }
    s.push_str("\n**Special Lagnas**:\n");
    for sl in &data.chart.special_lagnas {
        s.push_str(&format!("- **{}**: {:.2}° ({})\n", sl.name, sl.longitude, rasi_name(locale, sl.rasi)));
    }
    s.push_str("\n");

    // Planetary Avasthas (행성 상태)
    s.push_str("### 3.5 행성 상태 분석 (Planetary Avasthas)\n\n");
    s.push_str("| Planet (행성) | Baladi Avastha | Jagradadi Avastha | Deeptaadi Avastha | Lajjitadi Avastha |\n");
    s.push_str("| --- | --- | --- | --- | --- |\n");
    for av in &data.chart.avasthas {
        s.push_str(&format!("| **{}** | {} | {} | {} | {} |\n", 
            translate_planet(locale, av.planet),
            format_baladi(&av.baladi),
            format_jagradadi(&av.jagradadi),
            format_deeptaadi(&av.deeptaadi),
            translate_avastha(locale, &av.lajjitadi)
        ));
    }
    s.push_str("\n");

    // Panchanga Details (5대 판창가)
    s.push_str("### 3.6 5대 판창가 상세 (Panchanga Details)\n\n");
    let pan = &data.chart.panchanga;
    s.push_str(&format!("- **Vara (요일)**: {} (Lord: {})\n", pan.vara, translate_planet(locale, pan.day_lord)));
    s.push_str(&format!("- **Tithi (음력 날짜)**: {} (Tithi #{})\n", pan.tithi_name, pan.tithi));
    s.push_str(&format!("- **Nakshatra (나크샤트라)**: {} (Nakshatra #{})\n", nakshatra_name(pan.nakshatra), pan.nakshatra));
    s.push_str(&format!("- **Yoga (요가)**: Yoga #{}\n", pan.yoga));
    s.push_str(&format!("- **Karana (카라나)**: {} (Karana #{})\n", pan.karana_name, pan.karana));
    s.push_str(&format!("- **Yogi Planet (요기 행성)**: {} (Point: {:.2}°)\n", translate_planet(locale, pan.yogi_planet), pan.yogi_point));
    s.push_str(&format!("- **Avayogi Planet (아바요기 행성)**: {}\n", translate_planet(locale, pan.avayogi_planet)));
    
    let dagdha_names: Vec<String> = pan.dagdha_rashis.iter().map(|&r| rasi_name(locale, r).to_string()).collect();
    s.push_str(&format!("- **Dagdha Rashis (연소된 사인)**: {}\n", dagdha_names.join(", ")));
    s.push_str(&format!("- **Sunrise/Sunset (일출/일몰)**: {} / {} ({})\n", 
        pan.sunrise.format("%H:%M:%S"), pan.sunset.format("%H:%M:%S"),
        if pan.is_day_birth { "Day Birth ☀️" } else { "Night Birth 🌙" }
    ));
    let (rahu_start, rahu_end) = pan.rahu_kalam;
    let (yama_start, yama_end) = pan.yamaganda;
    let (guli_start, guli_end) = pan.gulika;
    s.push_str(&format!("- **Rahu Kalam (라후 칼람)**: {} ~ {}\n", rahu_start.format("%H:%M"), rahu_end.format("%H:%M")));
    s.push_str(&format!("- **Yamaganda (야마간다)**: {} ~ {}\n", yama_start.format("%H:%M"), yama_end.format("%H:%M")));
    s.push_str(&format!("- **Gulika (굴리카)**: {} ~ {}\n\n", guli_start.format("%H:%M"), guli_end.format("%H:%M")));

    // KP System cusps/significators
    if let Some(kp) = &data.kp_analysis {
        s.push_str("### 3.7 KP System unequal 하우스 및 지표성 (KP Significators)\n\n");
        s.push_str("**KP House Cusps**:\n");
        s.push_str("| Cusp | Longitude | Sign Lord | Star Lord | Sub Lord |\n");
        s.push_str("| --- | --- | --- | --- | --- |\n");
        for c in &kp.cusps {
            s.push_str(&format!("| {} | {:.2}° | {} | {} | {} |\n", 
                c.name, c.longitude, 
                translate_planet(locale, c.sign_lord), translate_planet(locale, c.star_lord), translate_planet(locale, c.sub_lord)));
        }
        s.push_str("\n");

        s.push_str("**KP Planet Points**:\n");
        s.push_str("| Planet | Longitude | Sign Lord | Star Lord | Sub Lord |\n");
        s.push_str("| --- | --- | --- | --- | --- |\n");
        for p in &kp.planets {
            s.push_str(&format!("| {} | {:.2}° | {} | {} | {} |\n", 
                translate_planet_str(locale, &p.name), p.longitude, 
                translate_planet(locale, p.sign_lord), translate_planet(locale, p.star_lord), translate_planet(locale, p.sub_lord)));
        }
        s.push_str("\n");

        s.push_str("**KP Significators (지표성)**:\n");
        s.push_str("| Planet | Occupied House | Owned Houses | Level 1 (Star Occ) | Level 2 (Occ) | Level 3 (Star Own) | Level 4 (Own) |\n");
        s.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for sig in &kp.significators {
            let owned_str = sig.owned_houses.iter().map(|h| h.to_string()).collect::<Vec<_>>().join(", ");
            let lvl1 = sig.level1.iter().map(|h| h.to_string()).collect::<Vec<_>>().join(", ");
            let lvl2 = sig.level2.iter().map(|h| h.to_string()).collect::<Vec<_>>().join(", ");
            let lvl3 = sig.level3.iter().map(|h| h.to_string()).collect::<Vec<_>>().join(", ");
            let lvl4 = sig.level4.iter().map(|h| h.to_string()).collect::<Vec<_>>().join(", ");
            s.push_str(&format!("| **{}** | {} | [{}] | [{}] | [{}] | [{}] | [{}] |\n", 
                translate_planet(locale, sig.planet), sig.occupied_house, owned_str, lvl1, lvl2, lvl3, lvl4));
        }
        s.push_str("\n");
    }

    // Yogas
    let (yogas_title, yoga_name_col, yoga_desc_col) = match locale {
        Locale::Ko => ("요가 (Yogas)", "요가명", "영향 및 설명"),
        Locale::En => ("Planetary Yogas", "Yoga Name", "Effects & Description"),
        Locale::Zh => ("吉凶组合 (Yogas)", "组合名称", "影响与说明"),
        Locale::Ru => ("Планетарные Йоги", "Название Йоги", "Влияние и описание"),
    };

    s.push_str(&format!("### {}\n\n", yogas_title));
    if !data.report.yogas.is_empty() {
        s.push_str(&format!("| {} | {} |\n", yoga_name_col, yoga_desc_col));
        s.push_str("| --- | --- |\n");
        for y in &data.report.yogas {
            s.push_str(&format!("| **{}** | {} |\n", y.name, y.description));
        }
    } else {
        s.push_str("—\n");
    }
    s.push_str("\n");

    // ── Phase 3: Vimshottari Dasha with Antar Dasha (hierarchical) ──────────
    let (dasha_title, dasha_years_lbl, antar_lbl) = match locale {
        Locale::Ko => ("빔쇼타리 다샤 흐름 (Vimshottari Dasha + 소운)", "년", "소운"),
        Locale::En => ("Vimshottari Dasha Timeline (with Antar Dasha)", "years", "Antar"),
        Locale::Zh => ("维姆绍塔里大运 (含小运)", "年", "小运"),
        Locale::Ru => ("Вимшоттари Даша (с Антар Дашой)", "лет", "Антар"),
    };

    s.push_str(&format!("### {}\n\n", dasha_title));
    for period in &data.report.dasha_timeline {
        let start_fmt = period.start_time.format("%Y-%m-%d").to_string();
        let end_fmt = period.end_time.format("%Y-%m-%d").to_string();
        let p_name = translate_planet(locale, period.lord);
        let duration_years = (period.end_time - period.start_time).num_days() as f64 / 365.25;
        s.push_str(&format!("- **{} ({} ~ {})**: {:.1} {}\n", p_name, start_fmt, end_fmt, duration_years, dasha_years_lbl));
        // Antar Dasha (소운) — up to 9 sub-periods
        for antar in period.sub_dashas.iter().take(9) {
            let a_start = antar.start_time.format("%Y-%m-%d").to_string();
            let a_end = antar.end_time.format("%Y-%m-%d").to_string();
            let a_name = translate_planet(locale, antar.lord);
            let a_dur = (antar.end_time - antar.start_time).num_days() as f64 / 365.25;
            s.push_str(&format!("  - *{} {}: {} ~ {}* ({:.2} {})\n", antar_lbl, a_name, a_start, a_end, a_dur, dasha_years_lbl));
        }
    }
    s.push_str("\n");

    // House Strengths & Ratings
    let (house_rating_title, house_col, score_col, rating_col, meaning_col, desc_col, reasons_col) = match locale {
        Locale::Ko => ("하우스별 강도 및 해석 (House Ratings)", "하우스", "총 점수", "평가 등급", "의미", "세부 설명", "판단 요인"),
        Locale::En => ("House Strengths & Ratings", "House", "Total Score", "Rating", "Key Meaning", "Description", "Factors"),
        Locale::Zh => ("各宫位力量与评估", "宫位", "总分", "评分", "核心意义", "详细说明", "判定因素"),
        Locale::Ru => ("Сила и рейтинг домов", "Дом", "Общий балл", "Рейтинг", "Ключевое значение", "Описание", "Факторы"),
    };

    s.push_str(&format!("### {}\n\n", house_rating_title));
    s.push_str(&format!("| {} | {} | {} | {} | {} | {} |\n", house_col, score_col, rating_col, meaning_col, desc_col, reasons_col));
    s.push_str("| --- | --- | --- | --- | --- | --- |\n");
    for h in &data.report.house_summary {
        let reasons_str = h.reasons.join(", ");
        s.push_str(&format!("| House {} | {:.1} | **{}** | {} | {} | {} |\n", 
            h.house, h.total_score, h.rating, h.summary, h.description, reasons_str));
    }
    s.push_str("\n");

    // Gochara planetary transits
    let (gochara_title, g_planet, g_rasi, g_house, g_status, g_blocked, g_murti, g_desc) = match locale {
        Locale::Ko => ("고차라 실시간 행성 통과 분석 (Gochara Transits)", "행성", "현재 사인", "달 기준 하우스", "길흉 여부", "Vedha 장애", "Murt Nirnaya (형태)", "상세 해설"),
        Locale::En => ("Gochara Planetary Transits Analysis", "Planet", "Current Sign", "House from Moon", "Status", "Vedha Obstruction", "Murti Type", "Description"),
        Locale::Zh => ("过境星体分析 (Gochara)", "星体", "当前星座", "月亮起算宫", "吉凶", "星曜阻碍 (Vedha)", "穆尔蒂类型", "详细解析"),
        Locale::Ru => ("Транзиты планет (Gochara)", "Планета", "Текущий знак", "Дом от Луны", "Статус", "Препятствие Ведха", "Тип Мурти", "Описание"),
    };

    s.push_str(&format!("### {}\n\n", gochara_title));
    s.push_str(&format!("| {} | {} | {} | {} | {} | {} | {} |\n", g_planet, g_rasi, g_house, g_status, g_blocked, g_murti, g_desc));
    s.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
    for t_pos in &data.gochara.transits {
        let p_name = translate_planet(locale, t_pos.planet);
        let r_name = rasi_name(locale, t_pos.current_rasi);
        let status_str = if t_pos.is_benefic_transit { "🟢 Benefic (吉)" } else { "🔴 Malefic (凶)" };
        let blocked_str = if t_pos.is_blocked { "Yes (Blocked)" } else { "No" };
        let murti_str = match t_pos.murti {
            eon_vedic::analysis::gochara::MurtiType::Gold => "Suvarna (Gold 🥇)",
            eon_vedic::analysis::gochara::MurtiType::Silver => "Rajata (Silver 🥈)",
            eon_vedic::analysis::gochara::MurtiType::Copper => "Tamra (Copper 🥉)",
            eon_vedic::analysis::gochara::MurtiType::Iron => "Loha (Iron ⛓️)",
            eon_vedic::analysis::gochara::MurtiType::Unknown => "Unknown",
        };
        s.push_str(&format!("| {} | {} | House {} | {} | {} | {} | **{}** - {} |\n", 
            p_name, r_name, t_pos.house_from_moon, status_str, blocked_str, murti_str, t_pos.summary, t_pos.description));
    }
    s.push_str("\n");

    // Varga Nakshatra Reports (23 charts)
    let (varga_sec_title, v_planet, v_pos, v_house, v_nak, v_lord, v_deity, v_purpose) = match locale {
        Locale::Ko => ("분할차트(Varga) 상세 분석 (Varga Charts)", "행성", "도수 / 상태", "사인 / 하우스", "나크샤트라 (파다)", "나크/파다 지배성", "수호신 (Deity)", "인생 지향점 (Purpose)"),
        Locale::En => ("Varga Charts Detailed Analysis", "Planet", "Position / State", "Sign / House", "Nakshatra (Pada)", "Nak / Pada Lord", "Deity", "Life Purpose"),
        Locale::Zh => ("分盘详细分析 (Vargas)", "星体", "度数 / 状态", "星座 / 宫位", "星宿 (步)", "星宿/步主星", "主神 (Deity)", "人生目标 (Purpose)"),
        Locale::Ru => ("Подробный анализ дробных карт (Varga)", "Планета", "Положение / Состояние", "Знак / Дом", "Накшатра (Пада)", "Управитель Нак/Пада", "Божество", "Жизненная цель"),
    };

    s.push_str(&format!("### {}\n\n", varga_sec_title));
    let varga_keys = vec![
        "rasi", "hora", "drekkana", "chaturthamsha", "panchamsa", "shashtamsa", "saptamsa", "ashtamsa", "navamsa",
        "dasamsa", "rudramsa", "dwadasamsa", "shodashamsa", "vimsamsa", "chaturvimshamsa", "saptavimsamsa", "trimsamsa",
        "khavedamsa", "akshavedamsa", "shashtyamsa", "navanavamsa", "ashtottaramsa", "dwadasdwadasamsa",
    ];

    for key in &varga_keys {
        if let Some(rep) = data.varga_nakshatra_reports.reports.get(*key) {
            let lagna_sign = rasi_name(locale, rep.lagna_rasi);
            s.push_str(&format!("#### {} (Lagna: {})\n\n", rep.varga_label, lagna_sign));
            s.push_str(&format!("| {} | {} | {} | {} | {} | {} | {} |\n", 
                v_planet, v_pos, v_house, v_nak, v_lord, v_deity, v_purpose));
            s.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
            for row in &rep.rows {
                let p_trans = translate_planet_str(locale, &row.planet);
                let sign_n = rasi_name(locale, row.sign);
                
                let mut state_suffix = String::new();
                if row.is_retrograde {
                    state_suffix.push_str(" [R]");
                }
                if row.is_combust {
                    state_suffix.push_str(" [C]");
                }
                
                s.push_str(&format!("| **{}** | {}{} | {} (House {}) | {} (Pada {}) | {} / {} | {} | {} |\n",
                    p_trans, row.position_str, state_suffix, sign_n, row.house, row.nakshatra_name, row.pada, row.nakshatra_lord, row.pada_lord, row.deity, row.purpose));
            }
            s.push_str("\n");
        }
    }

    // Varga Integrated Planet Interpretations
    let (varga_interp_title, vi_planet, vi_vargo, vi_pushkar, vi_d9, vi_d10, vi_d60, vi_summary) = match locale {
        Locale::Ko => ("분할차트 통합 행성 해석 (Varga Interpretations)", "행성", "Vargottama", "Pushkar Navamsa", "D9 사인", "D10 사인", "D60 사인", "해석 요약"),
        Locale::En => ("Varga Integrated Planet Interpretations", "Planet", "Vargottama", "Pushkar Navamsa", "D9 Sign", "D10 Sign", "D60 Sign", "Interpretation Summary"),
        Locale::Zh => ("分盘综合星体解析", "星体", "九分得雄 (Vargottama)", "普什卡星区 (Pushkar)", "D9 星座", "D10 星座", "D60 星座", "解析摘要"),
        Locale::Ru => ("Интегрированное толкование планет по дробным картам", "Планета", "Варготтама", "Пушкар Навамша", "D9 знак", "D10 знак", "D60 знак", "Резюме толкования"),
    };

    s.push_str(&format!("### {}\n\n", varga_interp_title));
    s.push_str(&format!("| {} | {} | {} | {} | {} | {} | {} |\n", 
        vi_planet, vi_vargo, vi_pushkar, vi_d9, vi_d10, vi_d60, vi_summary));
    s.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
    for vi in &data.report.varga_interpretations {
        let p_name = translate_planet(locale, vi.planet);
        let vargo_str = if vi.is_vargottama { "Yes (🟢)" } else { "No" };
        let pushkar_str = if vi.is_pushkar_navamsa { "Yes (🌟)" } else { "No" };
        let d9_sign = rasi_name(locale, vi.d9_rasi);
        let d10_sign = rasi_name(locale, vi.d10_rasi);
        let d60_sign = rasi_name(locale, vi.d60_rasi);
        
        s.push_str(&format!("| {} | {} | {} | {} | {} | {} | **{}** - {} |\n", 
            p_name, vargo_str, pushkar_str, d9_sign, d10_sign, d60_sign, vi.summary, vi.description));
    }
    s.push_str("\n");

    // Marriage & Career text analysis
    let (marriage_title, career_title) = match locale {
        Locale::Ko => ("D9 삼수/배우자 분석 (Navamsa Marriage Analysis)", "D10 직업/성공 분석 (Dasamsa Career Analysis)"),
        Locale::En => ("D9 Navamsa Marriage & Relationship Analysis", "D10 Dasamsa Career & Visibility Analysis"),
        Locale::Zh => ("D9 九分盘婚姻与配偶分析", "D10 十分盘事业与社会成就分析"),
        Locale::Ru => ("D9 Навамша: Анализ брака и партнерства", "D10 Дашамша: Анализ карьеры и успеха"),
    };

    s.push_str(&format!("### {}\n\n", marriage_title));
    s.push_str(&format!("{}\n\n", data.report.d9_marriage_analysis));

    s.push_str(&format!("### {}\n\n", career_title));
    s.push_str(&format!("{}\n\n", data.report.d10_career_analysis));

    // Tajika Annual Report
    let (tajika_title, t_lord_lbl, t_muntha_lbl, t_saham_lbl, t_saham_name, t_saham_pos, t_saham_rasi, t_harsha_lbl, t_harsha_score) = match locale {
        Locale::Ko => ("타지카 연간 점성학 분석 (Tajika Varshaphala)", "올해의 주성 (Year Lord)", "문타 사인 (Muntha Rasi)", "사함 (Saham) 지표", "지표명", "도수", "사인", "하르샤 발라 (Harsha Bala 강도)", "강도 점수 (0~4)"),
        Locale::En => ("Tajika Annual Astrological Analysis (Varshaphala)", "Year Lord", "Muntha Sign (Progressed)", "Saham Points", "Point Name", "Longitude", "Sign", "Harsha Bala Strengths", "Strength Score (0~4)"),
        Locale::Zh => ("塔吉卡年运分析 (Varshaphala)", "年度主宰星 (Year Lord)", "Muntha 星座", "萨罕 (Saham) 指标", "指标名称", "经度", "星座", "哈尔沙力量 (Harsha Bala)", "力量得分 (0~4)"),
        Locale::Ru => ("Годовой анализ Таджика (Varshaphala)", "Управитель Года", "Мунтха Знак", "Точки Сахам", "Название точки", "Долгота", "Знак", "Сила Харша Бала", "Балл силы (0~4)"),
    };

    if let Some(tajika) = &data.tajika_report {
        s.push_str(&format!("### {}\n\n", tajika_title));
        let yl_str = tajika.year_lord.map(|p| translate_planet(locale, p)).unwrap_or("—");
        let muntha_str = rasi_name(locale, tajika.muntha_rasi);
        s.push_str(&format!("- **{}**: {}\n", t_lord_lbl, yl_str));
        s.push_str(&format!("- **{}**: {}\n\n", t_muntha_lbl, muntha_str));
        
        s.push_str(&format!("#### {}\n\n", t_saham_lbl));
        s.push_str(&format!("| {} | {} | {} |\n", t_saham_name, t_saham_pos, t_saham_rasi));
        s.push_str("| --- | --- | --- |\n");
        for saham in &tajika.sahams {
            let deg_within_sign = saham.longitude % 30.0;
            let deg_floor = deg_within_sign.floor() as i32;
            let min_val = ((deg_within_sign - deg_floor as f64) * 60.0).round() as i32;
            let pos_str = format!("{:02}° {:02}'", deg_floor, min_val);
            let s_rasi = rasi_name(locale, saham.rasi);
            s.push_str(&format!("| {} | {} | {} |\n", saham.name, pos_str, s_rasi));
        }
        s.push_str("\n");
        
        s.push_str(&format!("#### {}\n\n", t_harsha_lbl));
        s.push_str(&format!("| {} | {} |\n", t_saham_name, t_harsha_score));
        s.push_str("| --- | --- |\n");
        for &(p, score) in &tajika.harsha_bala_summary {
            let p_name = translate_planet(locale, p);
            s.push_str(&format!("| {} | {} / 4 |\n", p_name, score));
        }
        s.push_str("\n");
        
        if !tajika.summary.is_empty() {
            s.push_str(&format!("**{}**:\n{}\n\n", t_lord_lbl, tajika.summary));
        }
    }

    // Annual Chart Planetary Positions
    let (ann_chart_title, ann_planet, ann_deg, ann_rasi, ann_house, ann_nak, ann_pada, ann_lord, ann_retro, ann_combust) = match locale {
        Locale::Ko => ("연간 분할차트 행성 배치 (Annual Chart Positions)", "행성", "도수", "사인 (Sign)", "하우스", "나크샤트라", "파다(Pada)", "지배성", "역행", "태비"),
        Locale::En => ("Annual Chart Planetary Positions", "Planet", "Degree", "Sign", "House", "Nakshatra", "Pada", "Sign Lord", "Retro", "Combust"),
        Locale::Zh => ("年运星体位置", "星体", "度数", "星座", "宫位", "星宿", "步 (Pada)", "定位星", "逆行", "焦伤"),
        Locale::Ru => ("Положения планет годовой карты", "Планета", "Градус", "Знак", "Дом", "Накшатра", "Пада", "Управитель", "Ретро", "Сожж"),
    };

    if let Some(ann) = &data.annual_chart {
        s.push_str(&format!("### {}\n\n", ann_chart_title));
        s.push_str(&format!("| {} | {} | {} | {} | {} | {} | {} | {} | {} |\n", 
            ann_planet, ann_deg, ann_rasi, ann_house, ann_nak, ann_pada, ann_retro, ann_combust, ann_lord));
        s.push_str("| --- | --- | --- | --- | --- | --- | --- | --- | --- |\n");
        
        let format_ann_pos = |pos: &eon_vedic::core::chart::VedicPosition, is_asc: bool| -> String {
            let p_name = if is_asc {
                translate_planet(locale, VedicPlanet::Ascendant)
            } else {
                translate_planet(locale, pos.planet)
            };
            
            let deg_within_sign = pos.sidereal_deg % 30.0;
            let deg_floor = deg_within_sign.floor() as i32;
            let min_val = ((deg_within_sign - deg_floor as f64) * 60.0).round() as i32;
            let deg_str = format!("{:02}° {:02}'", deg_floor, min_val);
            
            let r_name = rasi_name(locale, pos.rasi);
            let nak_name_val = nakshatra_name(pos.nakshatra);
            let retro_str = if pos.is_retrograde { "Yes" } else { "No" };
            let combust_str = if pos.is_combust { "Yes" } else { "No" };
            
            let lord = match pos.rasi {
                1 | 8 => VedicPlanet::Mars,
                2 | 7 => VedicPlanet::Venus,
                3 | 6 => VedicPlanet::Mercury,
                4 => VedicPlanet::Moon,
                5 => VedicPlanet::Sun,
                9 | 12 => VedicPlanet::Jupiter,
                10 | 11 => VedicPlanet::Saturn,
                _ => VedicPlanet::Sun,
            };
            let lord_name = translate_planet(locale, lord);
            
            format!("| **{}** | {} | {} | {} | {} | {} | {} | {} | {} |\n", 
                p_name, deg_str, r_name, pos.house_index, nak_name_val, pos.pada, retro_str, combust_str, lord_name)
        };
        
        s.push_str(&format_ann_pos(&ann.ascendant, true));
        for p in &ann.planets {
            s.push_str(&format_ann_pos(p, false));
        }
        s.push_str("\n");
    }

    // Yogini Dasha
    let (yogini_title, y_years_lbl) = match locale {
        Locale::Ko => ("요기니 다샤 흐름 (Yogini Dasha)", "년"),
        Locale::En => ("Yogini Dasha Timeline", "years"),
        Locale::Zh => ("约吉尼大运", "年"),
        Locale::Ru => ("Временные периоды Йогини Даша", "лет"),
    };

    if !data.report.yogini_timeline.is_empty() {
        s.push_str(&format!("### {}\n\n", yogini_title));
        for period in &data.report.yogini_timeline {
            let start_fmt = period.start_time.format("%Y-%m-%d").to_string();
            let end_fmt = period.end_time.format("%Y-%m-%d").to_string();
            let p_name = translate_planet(locale, period.lord);
            let duration_years = (period.end_time - period.start_time).num_days() as f64 / 365.25;
            
            let yogini_name = match period.lord {
                VedicPlanet::Moon => "Mangala (망갈라)",
                VedicPlanet::Sun => "Pingala (핑갈라)",
                VedicPlanet::Jupiter => "Dhanya (다냐)",
                VedicPlanet::Mars => "Bhramari (브라마리)",
                VedicPlanet::Mercury => "Bhadrika (바드리카)",
                VedicPlanet::Saturn => "Ulka (울카)",
                VedicPlanet::Venus => "Siddha (시다)",
                VedicPlanet::Rahu => "Sankata (산카타)",
                _ => "",
            };
            
            let label = if yogini_name.is_empty() {
                p_name.to_string()
            } else {
                format!("{} [{}]", yogini_name, p_name)
            };
            
            s.push_str(&format!("- **{} ({} ~ {})**: {:.1} {}\n", label, start_fmt, end_fmt, duration_years, y_years_lbl));
        }
        s.push_str("\n");
    }

    // Jaimini Chara Dasha
    let (chara_title, c_years_lbl) = match locale {
        Locale::Ko => ("제이머니 차라 다샤 흐름 (Jaimini Chara Dasha)", "년"),
        Locale::En => ("Jaimini Chara Dasha Timeline", "years"),
        Locale::Zh => ("吉米尼查拉大运", "年"),
        Locale::Ru => ("Временные периоды Джаймини Чара Даша", "лет"),
    };

    if !data.report.chara_dasha_timeline.is_empty() {
        s.push_str(&format!("### {}\n\n", chara_title));
        for period in &data.report.chara_dasha_timeline {
            let start_fmt = period.start_time.format("%Y-%m-%d").to_string();
            let end_fmt = period.end_time.format("%Y-%m-%d").to_string();
            let r_name = rasi_name(locale, period.rasi);
            let duration_years = (period.end_time - period.start_time).num_days() as f64 / 365.25;
            s.push_str(&format!("- **{} ({} ~ {})**: {:.1} {}\n", r_name, start_fmt, end_fmt, duration_years, c_years_lbl));
        }
        s.push_str("\n");
    }

    // Kala Chakra Dasha
    let (kala_title, k_years_lbl) = match locale {
        Locale::Ko => ("칼라차크라 다샤 흐름 (Kala Chakra Dasha)", "년"),
        Locale::En => ("Kala Chakra Dasha Timeline", "years"),
        Locale::Zh => ("时轮大运 (Kala Chakra Dasha)", "年"),
        Locale::Ru => ("Временные периоды Калачакра Даша", "лет"),
    };

    if !data.report.kalachakra_timeline.is_empty() {
        s.push_str(&format!("### {}\n\n", kala_title));
        for period in &data.report.kalachakra_timeline {
            let start_fmt = period.start_time.format("%Y-%m-%d").to_string();
            let end_fmt = period.end_time.format("%Y-%m-%d").to_string();
            let r_name = rasi_name(locale, period.rasi);
            s.push_str(&format!("- **{} ({} ~ {})**: {} {}\n", r_name, start_fmt, end_fmt, period.duration_years, k_years_lbl));
        }
        s.push_str("\n");
    }

    // Bhinnashtakavarga (BAV) Points Detail
    let (bav_title, bav_planet_col, bav_type_col) = match locale {
        Locale::Ko => ("행성별 아슈타카바르가 상세 (Bhinnashtakavarga - BAV)", "행성", "구분"),
        Locale::En => ("Planetary Ashtakavarga Details (Bhinnashtakavarga - BAV)", "Planet", "Type"),
        Locale::Zh => ("各星体阿슈타카바르가 (BAV)", "星体", "类型"),
        Locale::Ru => ("Планетарный Аштакаварга (BAV)", "Планета", "Тип"),
    };

    s.push_str(&format!("### {}\n\n", bav_title));
    s.push_str(&format!("| {} | {} | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | Sodya Pinda |\n", bav_planet_col, bav_type_col));
    s.push_str("| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |\n");
    for bav in &data.chart.bav {
        let p_name = translate_planet(locale, bav.planet);
        
        // Raw points
        s.push_str(&format!("| {} | Raw | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n", 
            p_name, bav.points[0], bav.points[1], bav.points[2], bav.points[3], bav.points[4], bav.points[5], bav.points[6], bav.points[7], bav.points[8], bav.points[9], bav.points[10], bav.points[11], bav.sodya_pinda));
        
        // Trikona points
        s.push_str(&format!("| | Trikona Shodhana | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | |\n", 
            bav.trikona_points[0], bav.trikona_points[1], bav.trikona_points[2], bav.trikona_points[3], bav.trikona_points[4], bav.trikona_points[5], bav.trikona_points[6], bav.trikona_points[7], bav.trikona_points[8], bav.trikona_points[9], bav.trikona_points[10], bav.trikona_points[11]));

        // Shodhana points
        s.push_str(&format!("| | Ekadhipatya Shodhana | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | |\n", 
            bav.shodhana_points[0], bav.shodhana_points[1], bav.shodhana_points[2], bav.shodhana_points[3], bav.shodhana_points[4], bav.shodhana_points[5], bav.shodhana_points[6], bav.shodhana_points[7], bav.shodhana_points[8], bav.shodhana_points[9], bav.shodhana_points[10], bav.shodhana_points[11]));
    }
    s.push_str("\n");

    // Vimshopaka Scores
    let (vim_title, v_planet_col, v_shadvarga_col, v_shodasha_col) = match locale {
        Locale::Ko => ("빔쇼파카 강도 분석 (Vimshopaka Bala)", "행성", "Shadvarga 점수 (6분할)", "Shodashavarga 점수 (16분할)"),
        Locale::En => ("Vimshopaka Strength Analysis (Vimshopaka Bala)", "Planet", "Shadvarga Score (Out of 20)", "Shodashavarga Score (Out of 20)"),
        Locale::Zh => ("Vimshopaka 力量分析", "星体", "Shadvarga 得分 (20分制)", "Shodashavarga 得分 (20分制)"),
        Locale::Ru => ("Анализ силы Вимшопака (Vimshopaka Bala)", "Планета", "Балл Шадварга (из 20)", "Балл Шодашаварга (из 20)"),
    };

    s.push_str(&format!("### {}\n\n", vim_title));
    s.push_str(&format!("| {} | {} | {} |\n", v_planet_col, v_shadvarga_col, v_shodasha_col));
    s.push_str("| --- | --- | --- |\n");
    for &(p, ref score) in &data.chart.vimshopaka_scores {
        let p_name = translate_planet(locale, p);
        s.push_str(&format!("| {} | {:.2} / 20 | {:.2} / 20 |\n", p_name, score.shadvarga_score, score.shodashavarga_score));
    }
    s.push_str("\n");

    // Planetary Aspects
    let (aspect_title, aspect_planet_col, aspect_houses_col) = match locale {
        Locale::Ko => ("행성별 애스펙트 (Planetary Aspects)", "영향을 주는 행성", "영향을 받는 하우스 목록"),
        Locale::En => ("Planetary Aspects (Drishti)", "Aspecting Planet", "Aspected Houses"),
        Locale::Zh => ("星体相位 (Drishti)", "相位星体", "受相位影响的宫位"),
        Locale::Ru => ("Планетарные Аспекты (Drishti)", "Аспектирующая планета", "Аспектируемые дома"),
    };

    s.push_str(&format!("### {}\n\n", aspect_title));
    s.push_str(&format!("| {} | {} |\n", aspect_planet_col, aspect_houses_col));
    s.push_str("| --- | --- |\n");
    for rel in &data.chart.aspects {
        let p_name = translate_planet(locale, rel.aspecting_planet);
        let houses_str = rel.aspected_houses.iter().map(|h| format!("House {}", h)).collect::<Vec<_>>().join(", ");
        s.push_str(&format!("| {} | {} |\n", p_name, houses_str));
    }
    s.push_str("\n");

    // ── Phase 2: Primary Karakas + Arudha/Upapada Lagna ────────────────────
    let (karaka_title, ak_lbl, amk_lbl, bk_lbl, mk_lbl, pk_lbl, gk_lbl, dk_lbl, al_lbl, ul_lbl) = match locale {
        Locale::Ko => ("자이미니 핵심 카라카 (Primary Karakas)", "아트마카라카 (AK - 자아)", "아마트야카라카 (AmK - 직업)", "브라트루카라카 (BK - 형제)", "마트루카라카 (MK - 어머니)", "푸트라카라카 (PK - 자녀)", "그나티카라카 (GK - 친족)", "다라카라카 (DK - 배우자)", "아루다 라그나 (AL)", "우파파다 라그나 (UL)"),
        Locale::En => ("Jaimini Primary Karakas", "Atmakaraka (AK - Self)", "Amatyakaraka (AmK - Career)", "Bhratrukaraka (BK - Siblings)", "Matrukaraka (MK - Mother)", "Putrakaraka (PK - Children)", "Gnatikaraka (GK - Relatives)", "Darakaraka (DK - Spouse)", "Arudha Lagna (AL)", "Upapada Lagna (UL)"),
        Locale::Zh => ("贾米尼核心功能星 (Primary Karakas)", "命主星 (AK - 自我)", "业主星 (AmK - 事业)", "兄弟星 (BK - 兄弟)", "母星 (MK - 母亲)", "子女星 (PK - 子女)", "亲族星 (GK - 亲属)", "配偶星 (DK - 配偶)", "月亮升华点 (AL)", "婚姻升华点 (UL)"),
        Locale::Ru => ("Основные Каракас Джаймини", "Атмакарака (AK — Я)", "Аматьякарака (AmK — Карьера)", "Братрукарака (BK — Братья)", "Матрукарака (MK — Мать)", "Путракарака (PK — Дети)", "Гнатикарака (GK — Родня)", "Даракарака (DK — Супруг)", "Аруда Лагна (AL)", "Упапада Лагна (UL)"),
    };
    s.push_str(&format!("### {}\n\n", karaka_title));
    s.push_str(&format!("- **{}**: {}\n", ak_lbl, translate_planet(locale, data.report.primary_karakas.atmakaraka)));
    s.push_str(&format!("- **{}**: {}\n", amk_lbl, translate_planet(locale, data.report.primary_karakas.amatyakaraka)));
    if let Some(bk) = data.report.primary_karakas.bhratrukaraka {
        s.push_str(&format!("- **{}**: {}\n", bk_lbl, translate_planet(locale, bk)));
    }
    if let Some(mk) = data.report.primary_karakas.matrukaraka {
        s.push_str(&format!("- **{}**: {}\n", mk_lbl, translate_planet(locale, mk)));
    }
    if let Some(pk) = data.report.primary_karakas.putrakaraka {
        s.push_str(&format!("- **{}**: {}\n", pk_lbl, translate_planet(locale, pk)));
    }
    if let Some(gk) = data.report.primary_karakas.gnatikaraka {
        s.push_str(&format!("- **{}**: {}\n", gk_lbl, translate_planet(locale, gk)));
    }
    s.push_str(&format!("- **{}**: {}\n", dk_lbl, translate_planet(locale, data.report.primary_karakas.darakaraka)));
    if data.report.arudha_lagna > 0 {
        s.push_str(&format!("- **{}**: {} ({})\n", al_lbl, data.report.arudha_lagna, rasi_name(locale, data.report.arudha_lagna)));
    }
    if data.report.upapada_lagna > 0 {
        s.push_str(&format!("- **{}**: {} ({})\n", ul_lbl, data.report.upapada_lagna, rasi_name(locale, data.report.upapada_lagna)));
    }
    if !data.report.special_lagnas_summary.is_empty() {
        let (spl_lbl, sl_lbl, il_lbl, hl_lbl, gl_lbl, pl_lbl) = match locale {
            Locale::Ko => ("특수 라그나 (Special Lagnas)", "Shri Lagna (SL - 번영)", "Indu Lagna (IL - 부)", "Hora Lagna (HL - 자산)", "Ghati Lagna (GL - 지위)", "Pranapada Lagna (PL - 생명)"),
            Locale::En => ("Special Lagnas", "Shri Lagna (SL - Prosperity)", "Indu Lagna (IL - Wealth)", "Hora Lagna (HL - Assets)", "Ghati Lagna (GL - Power)", "Pranapada Lagna (PL - Vitality)"),
            Locale::Zh => ("特殊命宫 (Special Lagnas)", "Shri Lagna (SL - 繁荣)", "Indu Lagna (IL - 财富)", "Hora Lagna (HL - 资产)", "Ghati Lagna (GL - 地位)", "Pranapada Lagna (PL - 生命)"),
            Locale::Ru => ("Специальные Лагны", "Шри Лагна (SL - Процветание)", "Инду Лагна (IL - Богатство)", "Хора Лагна (HL - Активы)", "Гхати Лагна (GL - Влияние)", "Пранапада Лагна (PL - Жизнь)"),
        };
        s.push_str(&format!("- **{}**:\n", spl_lbl));
        for (name, rasi) in &data.report.special_lagnas_summary {
            let label = match name.as_str() {
                "Shri Lagna" => sl_lbl,
                "Indu Lagna" => il_lbl,
                "Hora Lagna" => hl_lbl,
                "Ghati Lagna" => gl_lbl,
                "Pranapada Lagna" => pl_lbl,
                _ => name.as_str(),
            };
            s.push_str(&format!("  - {}: {}\n", label, rasi_name(locale, *rasi)));
        }
    }
    s.push_str("\n");

    // ── Phase 1: Shadbala 6-Factor Planetary Strength Table ─────────────────
    let (shadbala_title, sb_planet, sb_sthana, sb_dig, sb_kala, sb_chesta, sb_naisargika, sb_drik, sb_total, sb_status, sb_ishta, sb_kashta) = match locale {
        Locale::Ko => (
            "샤드발라 행성 강도 6대 요인 상세 (Shadbala)",
            "행성", "위치 강도 (Sthana)", "방향 강도 (Dig)", "시간 강도 (Kala)",
            "운동 강도 (Chesta)", "본질 강도 (Naisargika)", "상호 강도 (Drik)",
            "총점", "상태", "이쉬타 팔라 (길조)", "카쉬타 팔라 (흉조)",
        ),
        Locale::En => (
            "Shadbala Planetary Strength — 6 Factors",
            "Planet", "Sthana Bala", "Dig Bala", "Kala Bala",
            "Chesta Bala", "Naisargika Bala", "Drik Bala",
            "Total", "Status", "Ishta Phala (Auspicious)", "Kashta Phala (Inauspicious)",
        ),
        Locale::Zh => (
            "沙德拔拉行星力量六大要素",
            "星体", "位置力 (Sthana)", "方向力 (Dig)", "时间力 (Kala)",
            "运动力 (Chesta)", "本质力 (Naisargika)", "相互力 (Drik)",
            "总分", "状态", "吉祥指数 (Ishta)", "凶兆指数 (Kashta)",
        ),
        Locale::Ru => (
            "Шадбала — 6 факторов силы планет",
            "Планета", "Позиц. сила (Sthana)", "Направл. (Dig)", "Времен. (Kala)",
            "Движен. (Chesta)", "Нат. сила (Naisargika)", "Аспект. (Drik)",
            "Итого", "Статус", "Иштха Пхала (Удача)", "Каштха Пхала (Трудности)",
        ),
    };
    s.push_str(&format!("### {}\n\n", shadbala_title));
    s.push_str(&format!("| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
        sb_planet, sb_sthana, sb_dig, sb_kala, sb_chesta, sb_naisargika, sb_drik, sb_total, sb_status, sb_ishta, sb_kashta));
    s.push_str("| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |\n");
    for planet_pos in &data.chart.planets {
        let strength = eon_vedic::analysis::strength::StrengthEngine::calculate(planet_pos, &data.chart);
        let p_name = translate_planet(locale, planet_pos.planet);
        s.push_str(&format!("| **{}** | {:.1} | {:.1} | {:.1} | {:.1} | {:.1} | {:.1} | **{:.1}** | {} | {:.1} | {:.1} |\n",
            p_name,
            strength.sthana_bala,
            strength.dig_bala,
            strength.kala_bala,
            strength.chesta_bala,
            strength.naisargika_bala,
            strength.drik_bala,
            strength.total_score,
            strength.status,
            strength.ishta_phala,
            strength.kashta_phala,
        ));
    }
    s.push_str("\n");

    s
}

// Global birth details header
fn format_global_header(form: &crate::store::FormState, locale: Locale) -> String {
    let mut s = String::new();

    let (birth_title, date_lbl, lunar_lbl, gender_lbl, loc_lbl, male_txt, female_txt, yes_txt, no_txt) = match locale {
        Locale::Ko => ("1. 기본 출생 정보", "출생일시", "음력 여부", "성별", "출생 위치", "남성", "여성", "예", "아니오"),
        Locale::En => ("1. Basic Birth Information", "Date & Time", "Is Lunar", "Gender", "Birth Location", "Male", "Female", "Yes", "No"),
        Locale::Zh => ("1. 基本出生信息", "出生时间", "是否阴历", "性别", "出生地点", "男性", "女性", "是", "否"),
        Locale::Ru => ("1. Основная информация о рождении", "Дата и время", "По лунному календарю", "Пол", "Место рождения", "Мужской", "Женский", "Да", "Нет"),
    };

    s.push_str(&format!("## {}\n\n", birth_title));
    s.push_str(&format!("- **{}**: {}-{:02}-{:02} {:02}:{:02}\n", date_lbl, form.year, form.month, form.day, form.hour, form.minute));
    s.push_str(&format!("- **{}**: {}\n", lunar_lbl, if form.is_lunar { format!("{} (윤달: {})", yes_txt, if form.is_leap_month { yes_txt } else { no_txt }) } else { no_txt.to_string() }));
    s.push_str(&format!("- **{}**: {}\n", gender_lbl, if form.is_male { male_txt } else { female_txt }));
    s.push_str(&format!("- **{}**: {:.4}°N, {:.4}°E\n\n", loc_lbl, form.lat, form.lon));

    s
}

pub fn export_saju_to_markdown(data: &SajuAnalysisOutput, form: &crate::store::FormState, locale: Locale) -> String {
    let mut s = String::new();
    let title = match locale {
        Locale::Ko => "🌌 EON - 사주 명식 분석 보고서",
        Locale::En => "🌌 EON - Saju Analysis Report",
        Locale::Zh => "🌌 EON - 八字命理分析报告",
        Locale::Ru => "🌌 EON - Отчет по 분석 Бацзы",
    };
    s.push_str(&format!("# {}\n\n", title));
    s.push_str(&format_global_header(form, locale));
    s.push_str("## 2. 사주 분석 상세\n\n");
    s.push_str(&format_saju_inner(data, locale));
    s
}

pub fn export_vedic_to_markdown(data: &VedicAnalysisOutput, form: &crate::store::FormState, locale: Locale) -> String {
    let mut s = String::new();
    let title = match locale {
        Locale::Ko => "✨ EON - 베딕 점성학 분석 보고서",
        Locale::En => "✨ EON - Vedic Astrology Analysis Report",
        Locale::Zh => "✨ EON - 吠陀占星分析报告",
        Locale::Ru => "✨ EON - Отчет по Ведической Астрологии",
    };
    s.push_str(&format!("# {}\n\n", title));
    s.push_str(&format_global_header(form, locale));
    s.push_str("## 2. 베딕 분석 상세\n\n");
    s.push_str(&format_vedic_inner(data, locale));
    s
}

pub fn export_combined_to_markdown(
    saju: Option<&SajuAnalysisOutput>,
    vedic: Option<&VedicAnalysisOutput>,
    zwds: Option<&ZwdsAnalysisOutput>,
    tier: Option<&TierResult>,
    transit: Option<&TransitAnalysisOutput>,
    iching: Option<&IChingAnalysisOutput>,
    western: Option<&WesternAnalysisOutput>,
    form: &crate::store::FormState,
    locale: Locale,
) -> String {
    let mut s = String::new();
    let title = match locale {
        Locale::Ko => "🌌✨ EON - 사주, 베딕, 자미두수, 티어, 운세, 주역, 점성학 통합 분석 보고서",
        Locale::En => "🌌✨ EON - Saju, Vedic, ZWDS, Tier, Transit, I Ching & Western Astro Integrated Analysis Report",
        Locale::Zh => "🌌✨ EON - 八字、吠陀、紫微斗数、阶级、运势、周易与占星整合分析报告",
        Locale::Ru => "🌌✨ EON - Интегрированный отчет по Бацзы, Ведической Астрологии, ЦВдШ, Уровням, Транзитам, И Цзин и Западной астрологии",
    };
    s.push_str(&format!("# {}\n\n", title));
    s.push_str(&format_global_header(form, locale));
    
    let mut sec_num = 2;

    if let Some(saju_data) = saju {
        let saju_title = match locale {
            Locale::Ko => "사주 분석 상세 결과 (Saju Analysis)",
            Locale::En => "Saju Analysis Details",
            Locale::Zh => "八字分析详细结果",
            Locale::Ru => "Подробные результаты анализа Бацзы",
        };
        s.push_str(&format!("## {}. {}\n\n", sec_num, saju_title));
        s.push_str(&format_saju_inner(saju_data, locale));
        sec_num += 1;
    }
    
    if let Some(vedic_data) = vedic {
        let vedic_title = match locale {
            Locale::Ko => "베딕 분석 상세 결과 (Vedic Analysis)",
            Locale::En => "Vedic Analysis Details",
            Locale::Zh => "吠陀分析详细结果",
            Locale::Ru => "Подробные результаты Ведического анализа",
        };
        s.push_str(&format!("## {}. {}\n\n", sec_num, vedic_title));
        s.push_str(&format_vedic_inner(vedic_data, locale));
        sec_num += 1;
    }

    if let Some(zwds_data) = zwds {
        let zwds_title = match locale {
            Locale::Ko => "자미두수 분석 상세 결과 (Zi Wei Dou Shu)",
            Locale::En => "Zi Wei Dou Shu Details",
            Locale::Zh => "紫微斗数分析详细结果",
            Locale::Ru => "Подробные результаты анализа Цзы Вэй Доу Шу",
        };
        s.push_str(&format!("## {}. {}\n\n", sec_num, zwds_title));
        s.push_str(&format_zwds_inner(zwds_data, locale));
        sec_num += 1;
    }

    if let Some(iching_data) = iching {
        let iching_title = match locale {
            Locale::Ko => "주역 / 하락이수 분석 상세 결과 (I Ching / He Luo Li Shu)",
            Locale::En => "I Ching / He Luo Li Shu Lifetime Analysis Details",
            Locale::Zh => "周易 / 河洛理数 analysis 详细结果",
            Locale::Ru => "Подробные результаты анализа И Cзин / Хэ Ло Ли Шу",
        };
        s.push_str(&format!("## {}. {}\n\n", sec_num, iching_title));
        s.push_str(&format_iching_inner(iching_data, locale));
        sec_num += 1;
    }

    if let Some(western_data) = western {
        let western_title = match locale {
            Locale::Ko => "서양 점성학 분석 상세 결과 (Western Astrology)",
            Locale::En => "Western Astrology Analysis Details",
            Locale::Zh => "西洋占星分析详细结果",
            Locale::Ru => "Подробные результаты Западной астрологии",
        };
        s.push_str(&format!("## {}. {}\n\n", sec_num, western_title));
        s.push_str(&format_western_inner(western_data, locale));
        sec_num += 1;
    }

    if let Some(tier_data) = tier {
        let tier_title = match locale {
            Locale::Ko => "종합 운명 티어 분석 결과 (Destiny Tier)",
            Locale::En => "Integrated Destiny Tier Details",
            Locale::Zh => "综合命运阶级结果",
            Locale::Ru => "Результаты уровня судьбы",
        };
        s.push_str(&format!("## {}. {}\n\n", sec_num, tier_title));
        s.push_str(&format_tier_inner(tier_data, locale));
        sec_num += 1;
    }

    if let Some(transit_data) = transit {
        let transit_title = match locale {
            Locale::Ko => "실시간 운세 분석 결과 (Transit Luck)",
            Locale::En => "Real-time Transit Luck Details",
            Locale::Zh => "实时运势分析结果",
            Locale::Ru => "Результаты транзитной удачи",
        };
        s.push_str(&format!("## {}. {}\n\n", sec_num, transit_title));
        s.push_str(&format_transit_inner(transit_data, locale));
    }
    
    s
}

#[component]
pub fn ExportWidget() -> Element {
    let state = use_context::<AnalysisState>();
    let locale = *state.locale.read();

    let saju_state = state.saju.read();
    let vedic_state = state.vedic.read();
    let zwds_state = state.zwds.read();
    let tier_state = state.tier.read();
    let transit_state = state.transit.read();
    let iching_state = state.iching.read();
    let western_state = state.western.read();
    let form = state.form.read().clone();

    let has_saju = saju_state.status == TaskStatus::Success && saju_state.data.is_some();
    let has_vedic = vedic_state.status == TaskStatus::Success && vedic_state.data.is_some();
    let has_zwds = zwds_state.status == TaskStatus::Success && zwds_state.data.is_some();
    let has_tier = tier_state.status == TaskStatus::Success && tier_state.data.is_some();
    let has_transit = transit_state.status == TaskStatus::Success && transit_state.data.is_some();
    let has_iching = iching_state.status == TaskStatus::Success && iching_state.data.is_some();
    let has_western = western_state.status == TaskStatus::Success && western_state.data.is_some();

    let saju_data = saju_state.data.clone();
    let vedic_data = vedic_state.data.clone();
    let zwds_data = zwds_state.data.clone();
    let tier_data = tier_state.data.clone();
    let transit_data = transit_state.data.clone();
    let iching_data = iching_state.data.clone();
    let western_data = western_state.data.clone();

    let mut copied_saju = use_signal(|| false);
    let mut copied_vedic = use_signal(|| false);
    let mut copied_zwds = use_signal(|| false);
    let mut copied_tier = use_signal(|| false);
    let mut copied_transit = use_signal(|| false);
    let mut copied_iching = use_signal(|| false);
    let mut copied_western = use_signal(|| false);
    let mut copied_combined = use_signal(|| false);

    let widget_title = match locale {
        Locale::Ko => "분석 결과 내보내기",
        Locale::En => "EXPORT REPORT",
        Locale::Zh => "导出分析报告",
        Locale::Ru => "ЭКСПОРТ ОТЧЕТА",
    };

    let saju_btn_lbl = match locale {
        Locale::Ko => "사주 보고서 복사",
        Locale::En => "Copy Saju Report",
        Locale::Zh => "复制八字报告",
        Locale::Ru => "Копировать отчет Бацзы",
    };
    let vedic_btn_lbl = match locale {
        Locale::Ko => "베딕 보고서 복사",
        Locale::En => "Copy Vedic Report",
        Locale::Zh => "复制吠陀报告",
        Locale::Ru => "Копировать Ведический отчет",
    };
    let zwds_btn_lbl = match locale {
        Locale::Ko => "자미두수 보고서 복사",
        Locale::En => "Copy ZWDS Report",
        Locale::Zh => "复制紫微斗数报告",
        Locale::Ru => "Копировать отчет ЦВдШ",
    };
    let tier_btn_lbl = match locale {
        Locale::Ko => "운명 티어 보고서 복사",
        Locale::En => "Copy Destiny Tier Report",
        Locale::Zh => "复制命运阶级报告",
        Locale::Ru => "Копировать отчет уровня судьбы",
    };
    let transit_btn_lbl = match locale {
        Locale::Ko => "실시간 운세 보고서 복사",
        Locale::En => "Copy Transit Luck Report",
        Locale::Zh => "复制实时运势报告",
        Locale::Ru => "Копировать отчет транзитной удачи",
    };
    let combined_btn_lbl = match locale {
        Locale::Ko => "통합 분석 보고서 복사",
        Locale::En => "Copy Combined Report",
        Locale::Zh => "复制综合分析报告",
        Locale::Ru => "Копировать объединенный отчет",
    };
    let iching_btn_lbl = match locale {
        Locale::Ko => "주역 보고서 복사",
        Locale::En => "Copy I Ching Report",
        Locale::Zh => "复制周易报告",
        Locale::Ru => "Копировать отчет И Цзин",
    };
    let western_btn_lbl = match locale {
        Locale::Ko => "서양 점성학 보고서 복사",
        Locale::En => "Copy Western Astro Report",
        Locale::Zh => "复制西洋占星报告",
        Locale::Ru => "Копировать отчет западной астрологии",
    };

    let form_cloned_saju = form.clone();
    let form_cloned_vedic = form.clone();
    let form_cloned_zwds = form.clone();
    let form_cloned_tier = form.clone();
    let form_cloned_transit = form.clone();
    let form_cloned_iching = form.clone();
    let form_cloned_western = form.clone();
    let form_cloned_comb = form.clone();

    let saju_data_cloned_saju = saju_data.clone();
    let saju_data_cloned_comb = saju_data.clone();

    let vedic_data_cloned_vedic = vedic_data.clone();
    let vedic_data_cloned_comb = vedic_data.clone();

    let zwds_data_cloned_zwds = zwds_data.clone();
    let zwds_data_cloned_comb = zwds_data.clone();

    let tier_data_cloned_tier = tier_data.clone();
    let tier_data_cloned_comb = tier_data.clone();

    let transit_data_cloned_transit = transit_data.clone();
    let transit_data_cloned_comb = transit_data.clone();

    let iching_data_cloned_iching = iching_data.clone();
    let iching_data_cloned_comb = iching_data.clone();
    let western_data_cloned_western = western_data.clone();
    let western_data_cloned_comb = western_data.clone();

    rsx! {
        div { class: "px-4 py-4 border-t border-slate-800/50 flex flex-col gap-2.5",
            p { class: "text-[10px] text-slate-500 uppercase tracking-widest font-semibold mb-1", "{widget_title}" }
            
            // 1. Copy Saju
            button {
                class: if has_saju {
                    "w-full text-xs font-semibold py-2 px-3 rounded-lg border transition-all duration-200 cursor-pointer flex items-center justify-between bg-slate-800/40 border-slate-700/50 text-slate-300 hover:bg-slate-700/50 hover:text-white"
                } else {
                    "w-full text-xs font-semibold py-2 px-3 rounded-lg border flex items-center justify-between bg-slate-900/20 border-slate-800/40 text-slate-600 cursor-not-allowed opacity-40"
                },
                disabled: !has_saju,
                onclick: move |_| {
                    if let Some(ref data) = saju_data_cloned_saju {
                        let txt = export_saju_to_markdown(data, &form_cloned_saju, locale);
                        copy_to_clipboard(&txt);
                        copied_saju.set(true);
                        spawn(async move {
                            gloo_timers::future::TimeoutFuture::new(2000).await;
                            copied_saju.set(false);
                        });
                    }
                },
                span { "📝 {saju_btn_lbl}" }
                if *copied_saju.read() {
                    span { class: "text-[10px] text-emerald-400 font-bold transition-all duration-300 animate-pulse", "{t(locale, TK::MsgCopiedToClipboard)}" }
                } else {
                    span { class: "text-[10px] text-slate-500", "Markdown" }
                }
            }

            // 2. Copy Vedic
            button {
                class: if has_vedic {
                    "w-full text-xs font-semibold py-2 px-3 rounded-lg border transition-all duration-200 cursor-pointer flex items-center justify-between bg-slate-800/40 border-slate-700/50 text-slate-300 hover:bg-slate-700/50 hover:text-white"
                } else {
                    "w-full text-xs font-semibold py-2 px-3 rounded-lg border flex items-center justify-between bg-slate-900/20 border-slate-800/40 text-slate-600 cursor-not-allowed opacity-40"
                },
                disabled: !has_vedic,
                onclick: move |_| {
                    if let Some(ref data) = vedic_data_cloned_vedic {
                        let txt = export_vedic_to_markdown(data, &form_cloned_vedic, locale);
                        copy_to_clipboard(&txt);
                        copied_vedic.set(true);
                        spawn(async move {
                            gloo_timers::future::TimeoutFuture::new(2000).await;
                            copied_vedic.set(false);
                        });
                    }
                },
                span { "✨ {vedic_btn_lbl}" }
                if *copied_vedic.read() {
                    span { class: "text-[10px] text-emerald-400 font-bold transition-all duration-300 animate-pulse", "{t(locale, TK::MsgCopiedToClipboard)}" }
                } else {
                    span { class: "text-[10px] text-slate-500", "Markdown" }
                }
            }

            // 3. Copy ZWDS
            button {
                class: if has_zwds {
                    "w-full text-xs font-semibold py-2 px-3 rounded-lg border transition-all duration-200 cursor-pointer flex items-center justify-between bg-slate-800/40 border-slate-700/50 text-slate-300 hover:bg-slate-700/50 hover:text-white"
                } else {
                    "w-full text-xs font-semibold py-2 px-3 rounded-lg border flex items-center justify-between bg-slate-900/20 border-slate-800/40 text-slate-600 cursor-not-allowed opacity-40"
                },
                disabled: !has_zwds,
                onclick: move |_| {
                    if let Some(ref data) = zwds_data_cloned_zwds {
                        let txt = export_zwds_to_markdown(data, &form_cloned_zwds, locale);
                        copy_to_clipboard(&txt);
                        copied_zwds.set(true);
                        spawn(async move {
                            gloo_timers::future::TimeoutFuture::new(2000).await;
                            copied_zwds.set(false);
                        });
                    }
                },
                span { "🔮 {zwds_btn_lbl}" }
                if *copied_zwds.read() {
                    span { class: "text-[10px] text-emerald-400 font-bold transition-all duration-300 animate-pulse", "{t(locale, TK::MsgCopiedToClipboard)}" }
                } else {
                    span { class: "text-[10px] text-slate-500", "Markdown" }
                }
            }

            // 4. Copy Destiny Tier
            button {
                class: if has_tier {
                    "w-full text-xs font-semibold py-2 px-3 rounded-lg border transition-all duration-200 cursor-pointer flex items-center justify-between bg-slate-800/40 border-slate-700/50 text-slate-300 hover:bg-slate-700/50 hover:text-white"
                } else {
                    "w-full text-xs font-semibold py-2 px-3 rounded-lg border flex items-center justify-between bg-slate-900/20 border-slate-800/40 text-slate-600 cursor-not-allowed opacity-40"
                },
                disabled: !has_tier,
                onclick: move |_| {
                    if let Some(ref data) = tier_data_cloned_tier {
                        let txt = export_tier_to_markdown(data, &form_cloned_tier, locale);
                        copy_to_clipboard(&txt);
                        copied_tier.set(true);
                        spawn(async move {
                            gloo_timers::future::TimeoutFuture::new(2000).await;
                            copied_tier.set(false);
                        });
                    }
                },
                span { "🏆 {tier_btn_lbl}" }
                if *copied_tier.read() {
                    span { class: "text-[10px] text-emerald-400 font-bold transition-all duration-300 animate-pulse", "{t(locale, TK::MsgCopiedToClipboard)}" }
                } else {
                    span { class: "text-[10px] text-slate-500", "Markdown" }
                }
            }

            // 5. Copy Transit Luck
            button {
                class: if has_transit {
                    "w-full text-xs font-semibold py-2 px-3 rounded-lg border transition-all duration-200 cursor-pointer flex items-center justify-between bg-slate-800/40 border-slate-700/50 text-slate-300 hover:bg-slate-700/50 hover:text-white"
                } else {
                    "w-full text-xs font-semibold py-2 px-3 rounded-lg border flex items-center justify-between bg-slate-900/20 border-slate-800/40 text-slate-600 cursor-not-allowed opacity-40"
                },
                disabled: !has_transit,
                onclick: move |_| {
                    if let Some(ref data) = transit_data_cloned_transit {
                        let txt = export_transit_to_markdown(data, &form_cloned_transit, locale);
                        copy_to_clipboard(&txt);
                        copied_transit.set(true);
                        spawn(async move {
                            gloo_timers::future::TimeoutFuture::new(2000).await;
                            copied_transit.set(false);
                        });
                    }
                },
                span { "⏳ {transit_btn_lbl}" }
                if *copied_transit.read() {
                    span { class: "text-[10px] text-emerald-400 font-bold transition-all duration-300 animate-pulse", "{t(locale, TK::MsgCopiedToClipboard)}" }
                } else {
                    span { class: "text-[10px] text-slate-500", "Markdown" }
                }
            }

            // 5.5 Copy I Ching
            button {
                class: if has_iching {
                    "w-full text-xs font-semibold py-2 px-3 rounded-lg border transition-all duration-200 cursor-pointer flex items-center justify-between bg-slate-800/40 border-slate-700/50 text-slate-300 hover:bg-slate-700/50 hover:text-white"
                } else {
                    "w-full text-xs font-semibold py-2 px-3 rounded-lg border flex items-center justify-between bg-slate-900/20 border-slate-800/40 text-slate-600 cursor-not-allowed opacity-40"
                },
                disabled: !has_iching,
                onclick: move |_| {
                    if let Some(ref data) = iching_data_cloned_iching {
                        let txt = export_iching_to_markdown(data, &form_cloned_iching, locale);
                        copy_to_clipboard(&txt);
                        copied_iching.set(true);
                        spawn(async move {
                            gloo_timers::future::TimeoutFuture::new(2000).await;
                            copied_iching.set(false);
                        });
                    }
                },
                span { "☯️ {iching_btn_lbl}" }
                if *copied_iching.read() {
                    span { class: "text-[10px] text-emerald-400 font-bold transition-all duration-300 animate-pulse", "{t(locale, TK::MsgCopiedToClipboard)}" }
                } else {
                    span { class: "text-[10px] text-slate-500", "Markdown" }
                }
            }

            // 5.6 Copy Western Astrology
            button {
                class: if has_western {
                    "w-full text-xs font-semibold py-2 px-3 rounded-lg border transition-all duration-200 cursor-pointer flex items-center justify-between bg-slate-800/40 border-slate-700/50 text-slate-300 hover:bg-slate-700/50 hover:text-white"
                } else {
                    "w-full text-xs font-semibold py-2 px-3 rounded-lg border flex items-center justify-between bg-slate-900/20 border-slate-800/40 text-slate-600 cursor-not-allowed opacity-40"
                },
                disabled: !has_western,
                onclick: move |_| {
                    if let Some(ref data) = western_data_cloned_western {
                        let txt = export_western_to_markdown(data, &form_cloned_western, locale);
                        copy_to_clipboard(&txt);
                        copied_western.set(true);
                        spawn(async move {
                            gloo_timers::future::TimeoutFuture::new(2000).await;
                            copied_western.set(false);
                        });
                    }
                },
                span { "🪐 {western_btn_lbl}" }
                if *copied_western.read() {
                    span { class: "text-[10px] text-emerald-400 font-bold transition-all duration-300 animate-pulse", "{t(locale, TK::MsgCopiedToClipboard)}" }
                } else {
                    span { class: "text-[10px] text-slate-500", "Markdown" }
                }
            }

            // 6. Copy Combined
            button {
                class: if has_saju || has_vedic || has_zwds || has_tier || has_transit || has_iching || has_western {
                    "w-full text-xs font-semibold py-2 px-3 rounded-lg border transition-all duration-200 cursor-pointer flex items-center justify-between bg-gradient-to-r from-violet-900/20 to-indigo-900/20 border-violet-800/40 text-violet-300 hover:from-violet-850/40 hover:to-indigo-850/40 hover:text-white hover:border-violet-600/50"
                } else {
                    "w-full text-xs font-semibold py-2 px-3 rounded-lg border flex items-center justify-between bg-slate-900/20 border-slate-800/40 text-slate-600 cursor-not-allowed opacity-40"
                },
                disabled: !has_saju && !has_vedic && !has_zwds && !has_tier && !has_transit && !has_iching && !has_western,
                onclick: move |_| {
                    let txt = export_combined_to_markdown(
                        saju_data_cloned_comb.as_ref(),
                        vedic_data_cloned_comb.as_ref(),
                        zwds_data_cloned_comb.as_ref(),
                        tier_data_cloned_comb.as_ref(),
                        transit_data_cloned_comb.as_ref(),
                        iching_data_cloned_comb.as_ref(),
                        western_data_cloned_comb.as_ref(),
                        &form_cloned_comb,
                        locale
                    );
                    copy_to_clipboard(&txt);
                    copied_combined.set(true);
                    spawn(async move {
                        gloo_timers::future::TimeoutFuture::new(2000).await;
                        copied_combined.set(false);
                    });
                },
                span { "🌌✨ {combined_btn_lbl}" }
                if *copied_combined.read() {
                    span { class: "text-[10px] text-emerald-400 font-bold transition-all duration-300 animate-pulse", "{t(locale, TK::MsgCopiedToClipboard)}" }
                } else {
                    span { class: "text-[10px] text-violet-400/80", "All" }
                }
            }
        }
    }
}

// ============================================================
// 자미두수 (ZWDS) 마크다운 포맷터
// ============================================================

use eon_service::dto::ZwdsAnalysisOutput;

pub fn format_zwds_inner(data: &ZwdsAnalysisOutput, locale: Locale) -> String {
    let mut s = String::new();
    let chart = &data.chart;
    let ln = &data.current_liu_nian;
    let dx = &data.current_daxian;

    // ── Phase 4: Current Daxian / Liunian Summary ──────────────────────────
    let (cur_title, cur_daxian_lbl, cur_liunian_lbl, _cur_age_lbl, _cur_year_lbl, _cur_palace_lbl, cur_sihua_lbl) = match locale {
        Locale::Ko => ("현재 대한 / 유년 현황", "현재 대한 (大限)", "현재 유년 (流年)", "나이 범위", "유년 연도", "유년 궁위", "유년 사화"),
        Locale::En => ("Current Da-Xian / Liu-Nian Status", "Current Da-Xian (Major Period)", "Current Liu-Nian (Annual Fortune)", "Age Range", "Annual Year", "Annual Palace", "Annual Si-Hua"),
        Locale::Zh => ("当前大限/流年状态", "当前大限", "当前流年", "年龄段", "流年年份", "流年宫位", "流年四化"),
        Locale::Ru => ("Текущее состояние Да-Сянь / Лю-Нянь", "Текущий Да-Сянь", "Текущий Лю-Нянь", "Возрастной диапазон", "Год", "Дворец года", "Четыре трансформации года"),
    };
    s.push_str(&format!("### {}\n\n", cur_title));

    // Current Da-Xian
    let dx_range_str = if locale == Locale::Ko {
        format!("{}세 ~ {}세", dx.age_start, dx.age_end)
    } else if locale == Locale::Zh {
        format!("{}岁 ~ {}岁", dx.age_start, dx.age_end)
    } else if locale == Locale::Ru {
        format!("{} - {} лет", dx.age_start, dx.age_end)
    } else {
        format!("Age {} - {}", dx.age_start, dx.age_end)
    };
    s.push_str(&format!("- **{}**: {}{}  ({}{}  |  {})\n",
        cur_daxian_lbl,
        dx.stem_hanja, dx.branch_hanja,
        crate::i18n::translate_zwds_palace(locale, eon_zwds::palace::get_palace_name(dx.palace_idx, dx.palace_idx)),
        match locale { Locale::Ko => "궁", Locale::Zh => "宫", _ => " Palace" },
        dx_range_str
    ));

    // Current Liu-Nian
    let ln_sihua_str = {
        let stars = [
            ("祿", ln.si_hua[0]),
            ("權", ln.si_hua[1]),
            ("科", ln.si_hua[2]),
            ("忌", ln.si_hua[3]),
        ];
        stars.iter()
            .map(|(sym, star)| format!("{}({})", sym, crate::i18n::translate_zwds_star(locale, *star)))
            .collect::<Vec<_>>()
            .join(", ")
    };
    s.push_str(&format!("- **{}**: {} ({}: {})\n",
        cur_liunian_lbl,
        ln.year,
        cur_sihua_lbl,
        ln_sihua_str
    ));
    s.push_str("\n");

    // Translated metadata labels
    let label_master_soul = match locale {
        Locale::Ko => "명주 (命主)",
        Locale::En => "Soul Master",
        Locale::Zh => "命主",
        Locale::Ru => "Хозяин Судьбы",
    };
    let label_master_body = match locale {
        Locale::Ko => "신주 (身主)",
        Locale::En => "Body Master",
        Locale::Zh => "身主",
        Locale::Ru => "Хозяин Тела",
    };
    let label_elements = match locale {
        Locale::Ko => "오행국 (五行局)",
        Locale::En => "Elements Bureau",
        Locale::Zh => "五行局",
        Locale::Ru => "Дворец Стихий",
    };

    s.push_str(&format!("- **{}**: {}\n", label_master_soul, crate::i18n::translate_zwds_star(locale, chart.soul_master)));
    s.push_str(&format!("- **{}**: {}\n", label_master_body, crate::i18n::translate_zwds_star(locale, chart.body_master)));
    s.push_str(&format!("- **{}**: {}\n\n", label_elements, crate::i18n::translate_five_elements(locale, chart.five_elements)));

    // 감지된 전통 격국
    if !chart.destiny_patterns.is_empty() {
        let label_patterns = match locale {
            Locale::Ko => "### 감지된 전통 격국",
            Locale::En => "### Detected Destiny Patterns",
            Locale::Zh => "### 检测到的传统格局",
            Locale::Ru => "### Обнаруженные традиционные структуры",
        };
        s.push_str(&format!("{}\n\n", label_patterns));
        for pat in chart.destiny_patterns.iter() {
            let pat_name = match locale {
                Locale::Zh => pat.name_hanja.clone(),
                _ => format!("{} ({})", pat.name_korean, pat.name_hanja),
            };
            let type_str = if pat.is_auspicious {
                match locale {
                    Locale::Ko => "길격(吉格)",
                    Locale::Zh => "吉格",
                    Locale::En => "Auspicious",
                    Locale::Ru => "Благоприятный",
                }
            } else {
                match locale {
                    Locale::Ko => "흉격(凶格)",
                    Locale::Zh => "凶格",
                    Locale::En => "Inauspicious",
                    Locale::Ru => "Неблагоприятный",
                }
            };
            let desc = match locale {
                Locale::Ko => &pat.description_korean,
                _ => &pat.description_english,
            };
            s.push_str(&format!("- **{}** [{}] - {}\n", pat_name, type_str, desc));
        }
        s.push_str("\n");
    }

    // 12궁 성반 배치 정보
    let title_palace_info = match locale {
        Locale::Ko => "### 12궁 성반 배치 정보",
        Locale::En => "### 12 Palaces Placement Detail",
        Locale::Zh => "### 12宫位安星详情",
        Locale::Ru => "### Детализация размещения по 12 дворцам",
    };
    s.push_str(&format!("{}\n\n", title_palace_info));

    let header_palace = match locale {
        Locale::Ko => "| 궁위 (Palace) | 궁명 (Name) | 별 배치 (Stars) | 대한 범위 (Da-Xian) | 유년 여부 |",
        Locale::En => "| Palace (Cusp) | Palace Name | Star Placement | Da-Xian Age | Annual Fortune |",
        Locale::Zh => "| 宫位 (Palace) | 宫名 (Name) | 星曜配置 (Stars) | 大限年龄段 | 是否流年 |",
        Locale::Ru => "| Дворец (Зодиак) | Имя Дворца | Расположение Звезд | Да-Сянь (Возраст) | Год |",
    };
    s.push_str(&format!("{}\n", header_palace));
    s.push_str("| --- | --- | --- | --- | --- |\n");

    for palace in chart.palaces.iter() {
        // Collect annual stars for this palace
        let mut annual_stars = Vec::new();
        if palace.index == ln.liu_lu { annual_stars.push(eon_zwds::types::ZwdsStar::LuCun); }
        if palace.index == ln.liu_yang { annual_stars.push(eon_zwds::types::ZwdsStar::QingYang); }
        if palace.index == ln.liu_tuo { annual_stars.push(eon_zwds::types::ZwdsStar::TuoLuo); }
        if palace.index == ln.liu_chang { annual_stars.push(eon_zwds::types::ZwdsStar::WenChang); }
        if palace.index == ln.liu_qu { annual_stars.push(eon_zwds::types::ZwdsStar::WenQu); }

        let mut stars_str_parts = Vec::new();

        // 1. 선천 별 + 사화
        for s_in_p in palace.stars.iter() {
            let star_name = crate::i18n::translate_zwds_star(locale, s_in_p.star);
            let mut parts = star_name.to_string();

            // 별 밝기 추가
            if let Some(brightness) = s_in_p.brightness {
                parts.push_str(&format!(" ({})", crate::i18n::translate_zwds_brightness(locale, brightness)));
            }

            let mut sihua_suffixes = Vec::new();

            // 선천 사화
            if let Some(sh) = s_in_p.si_hua {
                sihua_suffixes.push(sh.emoji().to_string());
            }

            // 유년 사화
            if s_in_p.star == ln.si_hua[0] { sihua_suffixes.push("流祿".to_string()); }
            else if s_in_p.star == ln.si_hua[1] { sihua_suffixes.push("流權".to_string()); }
            else if s_in_p.star == ln.si_hua[2] { sihua_suffixes.push("流科".to_string()); }
            else if s_in_p.star == ln.si_hua[3] { sihua_suffixes.push("流忌".to_string()); }

            if !sihua_suffixes.is_empty() {
                parts.push_str(&format!(" ({})", sihua_suffixes.join("+")));
            }

            stars_str_parts.push(parts);
        }

        // 2. 유년 잡성
        for a_star in annual_stars {
            let star_name = crate::i18n::translate_zwds_star(locale, a_star);
            stars_str_parts.push(format!("流 {}", star_name));
        }

        let stars_str = stars_str_parts.join(", ");

        let daxian_str = palace.daxian_range
            .map(|r| {
                if locale == Locale::Ko { format!("{}세 ~ {}세", r.0, r.1) }
                else if locale == Locale::Zh { format!("{}岁 ~ {}岁", r.0, r.1) }
                else if locale == Locale::Ru { format!("{} - {} лет", r.0, r.1) }
                else { format!("Age {} - {}", r.0, r.1) }
            })
            .unwrap_or_else(|| "—".to_string());

        let liunian_str = if palace.is_current_liu_nian {
            match locale {
                Locale::Ko => "★ 유년 궁",
                Locale::En => "★ Annual Fortune",
                Locale::Zh => "★ 流年宫",
                Locale::Ru => "★ Годовой Дворец",
            }
        } else {
            "—"
        };

        let daxian_name = eon_zwds::palace::get_palace_name(data.current_daxian.palace_idx, palace.index);
        let liunian_name = eon_zwds::palace::get_palace_name(data.current_liu_nian.palace_idx, palace.index);

        let dx_abbr = crate::i18n::translate_zwds_palace_abbr(locale, daxian_name);
        let ln_abbr = crate::i18n::translate_zwds_palace_abbr(locale, liunian_name);

        let dx_prefix = match locale {
            Locale::Ko => "대",
            Locale::Zh => "大",
            Locale::En => "D-",
            Locale::Ru => "Д-",
        };
        let ln_prefix = match locale {
            Locale::Ko => "유",
            Locale::Zh => "流",
            Locale::En => "A-",
            Locale::Ru => "Г-",
        };

        let dynamic_palaces_str = format!(" ({}{}/{}{})", dx_prefix, dx_abbr, ln_prefix, ln_abbr);

        s.push_str(&format!(
            "| {}{} | {}{} | {} | {} | {} |\n",
            palace.heavenly_stem, palace.earthly_branch,
            crate::i18n::translate_zwds_palace(locale, palace.name),
            dynamic_palaces_str,
            stars_str,
            daxian_str,
            liunian_str
        ));
    }
    s.push_str("\n");

    // 대한 리스트
    let title_daxian = match locale {
        Locale::Ko => "### 10년 대한(大限) 주기",
        Locale::En => "### 10-Year Da-Xian Cycles",
        Locale::Zh => "### 十年大限周期",
        Locale::Ru => "### 10-летние периоды Да-Сянь",
    };
    s.push_str(&format!("{}\n\n", title_daxian));

    let header_daxian = match locale {
        Locale::Ko => "| 순서 | 연령 범위 | 궁위 | 간지 |",
        Locale::En => "| Cycle | Age Range | Palace | Stem/Branch |",
        Locale::Zh => "| 序号 | 年龄区间 | 宫位 | 干支 |",
        Locale::Ru => "| Номер | Возрастной Диапазон | Дворец | Ствол/Ветвь |",
    };
    s.push_str(&format!("{}\n", header_daxian));
    s.push_str("| --- | --- | --- | --- |\n");

    for dx in chart.daxian.iter() {
        let cycle_name = if locale == Locale::Ko { format!("{}대운", dx.index + 1) }
            else if locale == Locale::Zh { format!("第 {} 大限", dx.index + 1) }
            else if locale == Locale::Ru { format!("{} Да-Сянь", dx.index + 1) }
            else { format!("Cycle {}", dx.index + 1) };

        let range_str = if locale == Locale::Ko { format!("{}세 ~ {}세", dx.age_start, dx.age_end) }
            else if locale == Locale::Zh { format!("{}岁 ~ {}岁", dx.age_start, dx.age_end) }
            else if locale == Locale::Ru { format!("{} - {} лет", dx.age_start, dx.age_end) }
            else { format!("Age {} - {}", dx.age_start, dx.age_end) };

        let palace_idx_str = if locale == Locale::Ko { format!("{}번 궁", dx.palace_idx) }
            else if locale == Locale::Zh { format!("{}号宫", dx.palace_idx) }
            else if locale == Locale::Ru { format!("Дворец {}", dx.palace_idx) }
            else { format!("Palace {}", dx.palace_idx) };

        s.push_str(&format!(
            "| {} | {} | {} | {}{} |\n",
            cycle_name,
            range_str,
            palace_idx_str,
            dx.stem_hanja,
            dx.branch_hanja
        ));
    }
    s.push_str("\n");

    // ── 궁위별 심층 분석 리딩 (Advanced Palace Destiny Interpretations) ──
    let title_deep_reading = match locale {
        Locale::Ko => "### 12궁위 상세 심층 분석 리딩 (Deep Destiny Readings)",
        Locale::Zh => "### 12宫位深层命运导读 (Deep Readings)",
        Locale::En => "### 12 Palaces Deep Destiny Readings",
        Locale::Ru => "### Подробное толкование по 12 дворцам",
    };
    s.push_str(&format!("{}\n\n", title_deep_reading));
    for palace in chart.palaces.iter() {
        let palace_name_str = crate::i18n::translate_zwds_palace(locale, palace.name);
        s.push_str(&format!("#### {} ({}{})\n\n", palace_name_str, palace.heavenly_stem, palace.earthly_branch));
        let advanced_reading = crate::i18n::zwds_interpret::get_advanced_palace_interpretation(
            locale,
            palace.name,
            &palace.stars,
            &chart.destiny_patterns,
        );
        s.push_str(&advanced_reading);
        s.push_str("\n\n");
    }

    // ── 비성사화 분석표 (Palace Flying Stars Analysis Table) ──
    let title_flying_stars = match locale {
        Locale::Ko => "### 궁간 비성사화 분석표 (Palace Flying Stars)",
        Locale::Zh => "### 宫干飞星四化分析表 (Flying Stars)",
        Locale::En => "### Palace Flying Stars Analysis Table",
        Locale::Ru => "### Таблица Летящих Звезд Дворца",
    };
    s.push_str(&format!("{}\n\n", title_flying_stars));

    let header_flying = match locale {
        Locale::Ko => "| 출발 궁위 (From) | 궁위 천간 | 성계 (Star) | 사화 (Sihua) | 대상 궁위 (To) |",
        Locale::Zh => "| 出发宫位 (From) | 宫干 (Stem) | 星曜 (Star) | 四化 (Sihua) | 目标宫位 (To) |",
        Locale::En => "| From Palace | Stem | Star | Si-Hua | To Palace |",
        Locale::Ru => "| Из дворца | Ствол | Звезда | Си-Хуа | В дворец |",
    };
    s.push_str(&format!("{}\n", header_flying));
    s.push_str("| --- | --- | --- | --- | --- |\n");

    for fs in chart.flying_sihua.iter() {
        let from_palace_data = chart.palaces.iter().find(|p| p.name == fs.from_palace);
        let stem_str = from_palace_data.map(|p| p.heavenly_stem.as_str()).unwrap_or("—");
        
        let from_palace_name = crate::i18n::translate_zwds_palace(locale, fs.from_palace);
        let to_palace_name = crate::i18n::translate_zwds_palace(locale, fs.to_palace);
        let star_name = crate::i18n::translate_zwds_star(locale, fs.star);
        let sihua_emoji = fs.sihua_type.emoji();
        
        s.push_str(&format!(
            "| {} | {} | {} | {} | {} |\n",
            from_palace_name,
            stem_str,
            star_name,
            sihua_emoji,
            to_palace_name
        ));
    }
    s.push_str("\n");

    s
}

fn format_tier_inner(data: &TierResult, locale: Locale) -> String {
    let mut s = String::new();

    let (tier_hdr, domain_hdr, saju_vedic_hdr, str_weak_hdr, components_hdr,
         destiny_tier_lbl, destiny_score_lbl, potential_tier_lbl, potential_score_lbl,
         domain_col, tier_col, saju_score_lbl, vedic_score_lbl,
         strengths_lbl, weaknesses_lbl, component_col, score_col, weight_col, reasons_col,
         growth_gap_lbl, risk_level_lbl, profile_lbl) = match locale {
        Locale::Ko => (
            "종합 운명 티어 분석 (Destiny Tier)",
            "분야별 운명 티어 (Domain Tiers)",
            "사주 및 베딕 세부 결과 (Eastern & Vedic Detailed Results)",
            "선천적 강점 및 주의점 (Strengths & Weaknesses)",
            "운명 구성요소 가중치 세부 정보 (Detailed Components & Weights)",
            "종합 운명 티어", "종합 운명 점수", "잠재력 티어", "잠재력 점수",
            "분야", "티어", "동양 사주 점수", "베딕 점성학 점수",
            "선천적 강점", "주의할 약점/주의점", "구성요소", "점수", "가중치", "판단 근거 / 세부 이유",
            "성장 갭 (Growth Gap)", "위험도 레벨 (Risk Level)", "인생 프로필 유형 (Profile)"
        ),
        Locale::En => (
            "Integrated Destiny Tier Analysis",
            "Domain Tiers",
            "Eastern & Vedic Detailed Results",
            "Inherent Strengths & Weaknesses",
            "Destiny Component Weights & Details",
            "Overall Destiny Tier", "Overall Destiny Score", "Potential Tier", "Potential Score",
            "Domain", "Tier", "Eastern Saju Score", "Vedic Score",
            "Inherent Strengths", "Inherent Weaknesses / Cautions", "Component", "Score", "Weight", "Reasons / Details",
            "Growth Gap", "Risk Level", "Life Profile Type"
        ),
        Locale::Zh => (
            "综合命运阶级分析 (Destiny Tier)",
            "各领域命运阶级 (Domain Tiers)",
            "八字与吠陀详细评分 (Eastern & Vedic Detailed Results)",
            "先天优势与注意事项 (Strengths & Weaknesses)",
            "命运构成要素权重细节 (Detailed Components & Weights)",
            "综合命运阶级", "综合命运分数", "潜能阶级", "潜能分数",
            "领域", "阶级", "东洋八字分数", "吠陀占星分数",
            "先天优势", "需要注意的弱点", "构成要素", "分数", "权重", "判断依据 / 细节原因",
            "成长空间 (Growth Gap)", "风险等级 (Risk Level)", "人生侧影类型 (Profile)"
        ),
        Locale::Ru => (
            "Интегрированный анализ уровня судьбы (Destiny Tier)",
            "Уровни судьбы по сферам (Domain Tiers)",
            "Подробные результаты Бацзы и Ведической астрологии",
            "Врожденные сильные стороны и предостережения",
            "Веса и детали компонентов судьбы",
            "Итоговый уровень судьбы", "Итоговый балл судьбы", "Потенциальный уровень", "Потенциальный балл",
            "Сфера", "Уровень", "Балл восточного Бацзы", "Балл ведической астрологии",
            "Врожденные преимущества", "Слабые стороны / Предостережения", "Компонент", "Балл", "Вес", "Обоснование / Детали",
            "Разрыв роста (Growth Gap)", "Уровень риска (Risk Level)", "Тип жизненного профиля (Profile)"
        ),
    };

    s.push_str(&format!("### {}\n\n", tier_hdr));
    s.push_str(&format!("- **{}**: **{}** ({})\n", destiny_tier_lbl, data.destiny_tier.grade, data.destiny_tier.label));
    s.push_str(&format!("- **{}**: {:.1} / 100.0\n", destiny_score_lbl, data.destiny_score));
    s.push_str(&format!("- **{}**: **{}** ({})\n", potential_tier_lbl, data.potential_tier.grade, data.potential_tier.label));
    s.push_str(&format!("- **{}**: {:.1} / 100.0\n", potential_score_lbl, data.potential_score));
    s.push_str(&format!("- **{}**: {:.1}\n", growth_gap_lbl, data.growth_gap));
    s.push_str(&format!("- **{}**: {}\n", risk_level_lbl, data.risk_level));
    s.push_str(&format!("- **{}**: {}\n\n", profile_lbl, data.profile));
    s.push_str(&format!("> **{}**\n\n", data.destiny_tier.desc));

    // Domain Tiers
    s.push_str(&format!("#### {}\n\n", domain_hdr));
    s.push_str(&format!("| {} | {} |\n", domain_col, tier_col));
    s.push_str("| --- | --- |\n");
    for dt in &data.domain_tiers {
        s.push_str(&format!("| {} | **{}** |\n", dt.domain, dt.tier));
    }
    s.push_str("\n");

    // Saju & Vedic details
    s.push_str(&format!("#### {}\n\n", saju_vedic_hdr));
    s.push_str(&format!("**{} ({:.1})**:\n", saju_score_lbl, data.saju_result.score));
    for hl in &data.saju_result.highlights {
        s.push_str(&format!("- ✓ {}\n", hl));
    }
    s.push_str("\n");
    s.push_str(&format!("**{} ({:.1})**:\n", vedic_score_lbl, data.vedic_result.score));
    for hl in &data.vedic_result.highlights {
        s.push_str(&format!("- ✓ {}\n", hl));
    }
    s.push_str("\n");

    // Strengths & Weaknesses
    s.push_str(&format!("#### {}\n\n", str_weak_hdr));
    s.push_str(&format!("**{}**:\n", strengths_lbl));
    for st in &data.strengths {
        s.push_str(&format!("- • {}\n", st));
    }
    s.push_str("\n");
    s.push_str(&format!("**{}**:\n", weaknesses_lbl));
    for we in &data.weaknesses {
        s.push_str(&format!("- • {}\n", we));
    }
    s.push_str("\n");

    // Detailed Components
    if !data.detailed_components.is_empty() {
        s.push_str(&format!("#### {}\n\n", components_hdr));
        s.push_str(&format!("| {} | {} | {} | {} |\n", component_col, score_col, weight_col, reasons_col));
        s.push_str("| --- | --- | --- | --- |\n");
        for comp in &data.detailed_components {
            let weight_pct = format!("{:.1}%", comp.weight * 100.0);
            let reasons_str = comp.reasons.join(", ");
            s.push_str(&format!("| {} | {:.1} | {} | {} |\n", comp.label, comp.score, weight_pct, reasons_str));
        }
        s.push_str("\n");
    }

    s
}

fn format_transit_inner(data: &TransitAnalysisOutput, locale: Locale) -> String {
    let mut s = String::new();

    let (transit_title, current_age_lbl, yearly_luck_lbl, monthly_luck_lbl, daily_luck_lbl, hourly_luck_lbl,
         pillar_col, god_col, relation_col, special_col, stage_col, year_col, month_col,
         monthly_lucks_title) = match locale {
        Locale::Ko => (
            "실시간 운세 분석 (Transit Luck)", "현재 나이", "세운 (연운)", "월운 (이번 달)", "일운 (오늘)", "시운 (현재 시간대)",
            "간지 (Pillar)", "십성 (Ten-God)", "원국 상호작용 (Natal Relations)", "특이사항 (Special Events)", "12운성", "연도", "월",
            "12개월 월운 상세 (Monthly Luck Details)"
        ),
        Locale::En => (
            "Real-time Transit Luck Analysis", "Current Age", "Annual Luck", "Monthly Luck (This Month)", "Daily Luck (Today)", "Hourly Luck (Current Time)",
            "GanZi (Pillar)", "Ten-God", "Natal Relations", "Special Events", "Twelve Stage", "Year", "Month",
            "12-Month Luck Details"
        ),
        Locale::Zh => (
            "实时运势分析 (Transit Luck)", "当前年龄", "流年运势", "流月运势 (本月)", "流日运势 (今日)", "流时运势 (当前时段)",
            "干支", "十神", "原局作用", "特殊注意事项", "十二运星", "年份", "月份",
            "12个月流月运势详情"
        ),
        Locale::Ru => (
            "Анализ транзитной удачи в реальном времени", "Текущий возраст", "Годовая удача", "Месячная удача", "Дневная удача", "Часовая удача",
            "Столп (Гань-Чжи)", "Десять Божеств", "Взаимодействие с картой", "Особые события", "12 Стадий Судьбы", "Год", "Месяц",
            "Подробности месячной удачи на 12 месяцев"
        ),
    };

    let age_str = match locale {
        Locale::Ko => format!("만 {}세", data.current_age),
        Locale::Zh => format!("{}岁", data.current_age),
        Locale::Ru => format!("{} лет", data.current_age),
        _ => format!("Age {}", data.current_age),
    };

    s.push_str(&format!("### {}\n\n", transit_title));
    s.push_str(&format!("- **{}**: {}\n\n", current_age_lbl, age_str));

    // Helper to translate influence relations
    let translate_inf_list = |loc: Locale, inf_str: &str| -> String {
        if inf_str == "—" || inf_str.is_empty() {
            return "—".to_string();
        }
        inf_str.split(", ")
            .map(|s| translate_saju_tag_str(loc, s.trim()))
            .collect::<Vec<String>>()
            .join(", ")
    };

    // Helper to translate special events
    let translate_spec_list = |loc: Locale, spec_str: &str| -> String {
        if spec_str == "—" || spec_str.is_empty() {
            return "—".to_string();
        }
        spec_str.split(", ")
            .map(|s| translate_saju_tag_str(loc, s.trim()))
            .collect::<Vec<String>>()
            .join(", ")
    };

    // Helper to translate 12 stages
    let translate_stage_val = |loc: Locale, stage_str: &str| -> String {
        if stage_str == "—" || stage_str.is_empty() {
            return "—".to_string();
        }
        let translated = translate_saju_twelve_stage_str(loc, stage_str);
        if translated.is_empty() { stage_str.to_string() } else { translated.to_string() }
    };

    let category_lbl = match locale {
        Locale::Ko => "구분",
        Locale::Zh => "分类",
        Locale::Ru => "Категория",
        _ => "Category",
    };

    // Summary table
    s.push_str(&format!("| {} | {} | {} | {} | {} | {} |\n", category_lbl, pillar_col, god_col, relation_col, stage_col, special_col));
    s.push_str("| --- | --- | --- | --- | --- | --- |\n");

    // Yearly
    let yr = &data.yearly_luck;
    let yr_inf = yr.influence.as_ref().map(|i| i.relations_with_natal.join(", ")).unwrap_or_else(|| "—".to_string());
    let yr_inf_trans = translate_inf_list(locale, &yr_inf);
    let yr_stage = yr.twelve_stage.as_deref().unwrap_or("—");
    let yr_stage_trans = translate_stage_val(locale, yr_stage);
    let yr_spec = if yr.special_events.is_empty() { "—".to_string() } else { yr.special_events.join(", ") };
    let yr_spec_trans = translate_spec_list(locale, &yr_spec);
    let yr_label_formatted = match locale {
        Locale::Ko => format!("{} ({}년)", yearly_luck_lbl, yr.year),
        Locale::Zh => format!("{} ({}年)", yearly_luck_lbl, yr.year),
        _ => format!("{} ({})", yearly_luck_lbl, yr.year),
    };
    let yr_ganzi_str = match locale {
        Locale::Ko => format!("{}({})", yr.ganzi.hanja(), yr.ganzi.hangul()),
        Locale::Zh => yr.ganzi.hanja().to_string(),
        _ => format!("{}({})", translate_saju_ganzi(locale, &yr.ganzi), yr.ganzi.hanja()),
    };
    let yr_gods_str = format!("{} / {}", 
        translate_saju_ten_god(locale, yr.stem_god), 
        translate_saju_ten_god(locale, yr.branch_god)
    );
    s.push_str(&format!("| **{}** | {} | {} | {} | {} | {} |\n",
        yr_label_formatted, yr_ganzi_str, yr_gods_str,
        yr_inf_trans, yr_stage_trans, yr_spec_trans
    ));

    // Monthly
    let mo = &data.monthly_luck;
    let mo_inf = mo.influence.as_ref().map(|i| i.relations_with_natal.join(", ")).unwrap_or_else(|| "—".to_string());
    let mo_inf_trans = translate_inf_list(locale, &mo_inf);
    let mo_stage = mo.twelve_stage.as_deref().unwrap_or("—");
    let mo_stage_trans = translate_stage_val(locale, mo_stage);
    let mo_spec = if mo.special_events.is_empty() { "—".to_string() } else { mo.special_events.join(", ") };
    let mo_spec_trans = translate_spec_list(locale, &mo_spec);
    let mo_label_formatted = match locale {
        Locale::Ko => format!("{} ({}월)", monthly_luck_lbl, mo.month),
        Locale::Zh => format!("{} ({}月)", monthly_luck_lbl, mo.month),
        Locale::Ru => format!("{} ({} мес.)", monthly_luck_lbl, mo.month),
        _ => format!("{} (Month {})", monthly_luck_lbl, mo.month),
    };
    let mo_ganzi_str = match locale {
        Locale::Ko => format!("{}({})", mo.ganzi.hanja(), mo.ganzi.hangul()),
        Locale::Zh => mo.ganzi.hanja().to_string(),
        _ => format!("{}({})", translate_saju_ganzi(locale, &mo.ganzi), mo.ganzi.hanja()),
    };
    let mo_gods_str = format!("{} / {}", 
        translate_saju_ten_god(locale, mo.stem_god), 
        translate_saju_ten_god(locale, mo.branch_god)
    );
    s.push_str(&format!("| **{}** | {} | {} | {} | {} | {} |\n",
        mo_label_formatted, mo_ganzi_str, mo_gods_str,
        mo_inf_trans, mo_stage_trans, mo_spec_trans
    ));

    // Daily
    let dy = &data.daily_luck;
    let dy_inf = dy.influence.as_ref().map(|i| i.relations_with_natal.join(", ")).unwrap_or_else(|| "—".to_string());
    let dy_inf_trans = translate_inf_list(locale, &dy_inf);
    let dy_stage = dy.twelve_stage.as_deref().unwrap_or("—");
    let dy_stage_trans = translate_stage_val(locale, dy_stage);
    let dy_spec = if dy.special_events.is_empty() { "—".to_string() } else { dy.special_events.join(", ") };
    let dy_spec_trans = translate_spec_list(locale, &dy_spec);
    let dy_label_formatted = match locale {
        Locale::Ko => format!("{} ({}일)", daily_luck_lbl, dy.day),
        Locale::Zh => format!("{} ({}日)", daily_luck_lbl, dy.day),
        Locale::Ru => format!("{} ({} дн.)", daily_luck_lbl, dy.day),
        _ => format!("{} (Day {})", daily_luck_lbl, dy.day),
    };
    let dy_ganzi_str = match locale {
        Locale::Ko => format!("{}({})", dy.ganzi.hanja(), dy.ganzi.hangul()),
        Locale::Zh => dy.ganzi.hanja().to_string(),
        _ => format!("{}({})", translate_saju_ganzi(locale, &dy.ganzi), dy.ganzi.hanja()),
    };
    let dy_gods_str = format!("{} / {}", 
        translate_saju_ten_god(locale, dy.stem_god), 
        translate_saju_ten_god(locale, dy.branch_god)
    );
    s.push_str(&format!("| **{}** | {} | {} | {} | {} | {} |\n",
        dy_label_formatted, dy_ganzi_str, dy_gods_str,
        dy_inf_trans, dy_stage_trans, dy_spec_trans
    ));

    // Hourly
    let hr = &data.hourly_luck;
    let hr_inf = hr.influence.as_ref().map(|i| i.relations_with_natal.join(", ")).unwrap_or_else(|| "—".to_string());
    let hr_inf_trans = translate_inf_list(locale, &hr_inf);
    let hr_stage = hr.twelve_stage.as_deref().unwrap_or("—");
    let hr_stage_trans = translate_stage_val(locale, hr_stage);
    let hr_spec = if hr.special_events.is_empty() { "—".to_string() } else { hr.special_events.join(", ") };
    let hr_spec_trans = translate_spec_list(locale, &hr_spec);
    let hr_label_formatted = match locale {
        Locale::Ko => format!("{} ({}시)", hourly_luck_lbl, hr.hour),
        Locale::Zh => format!("{} ({}时)", hourly_luck_lbl, hr.hour),
        Locale::Ru => format!("{} ({} ч.)", hourly_luck_lbl, hr.hour),
        _ => format!("{} (Hour {})", hourly_luck_lbl, hr.hour),
    };
    let hr_ganzi_str = match locale {
        Locale::Ko => format!("{}({})", hr.ganzi.hanja(), hr.ganzi.hangul()),
        Locale::Zh => hr.ganzi.hanja().to_string(),
        _ => format!("{}({})", translate_saju_ganzi(locale, &hr.ganzi), hr.ganzi.hanja()),
    };
    let hr_gods_str = format!("{} / {}", 
        translate_saju_ten_god(locale, hr.stem_god), 
        translate_saju_ten_god(locale, hr.branch_god)
    );
    s.push_str(&format!("| **{}** | {} | {} | {} | {} | {} |\n\n",
        hr_label_formatted, hr_ganzi_str, hr_gods_str,
        hr_inf_trans, hr_stage_trans, hr_spec_trans
    ));

    // 12 Months details
    s.push_str(&format!("#### {}\n\n", monthly_lucks_title));
    s.push_str(&format!("| {} | {} | {} | {} | {} | {} |\n", year_col, month_col, pillar_col, god_col, stage_col, relation_col));
    s.push_str("| --- | --- | --- | --- | --- | --- |\n");
    for m_luck in &data.monthly_lucks {
        let m_inf = m_luck.influence.as_ref().map(|i| i.relations_with_natal.join(", ")).unwrap_or_else(|| "—".to_string());
        let m_inf_trans = translate_inf_list(locale, &m_inf);
        let m_stage = m_luck.twelve_stage.as_deref().unwrap_or("—");
        let m_stage_trans = translate_stage_val(locale, m_stage);
        let m_month_formatted = match locale {
            Locale::Ko => format!("{}월", m_luck.month),
            Locale::Zh => format!("{}月", m_luck.month),
            Locale::Ru => format!("{} мес.", m_luck.month),
            _ => format!("Month {}", m_luck.month),
        };
        let m_ganzi_str = match locale {
            Locale::Ko => format!("{}({})", m_luck.ganzi.hanja(), m_luck.ganzi.hangul()),
            Locale::Zh => m_luck.ganzi.hanja().to_string(),
            _ => format!("{}({})", translate_saju_ganzi(locale, &m_luck.ganzi), m_luck.ganzi.hanja()),
        };
        let m_gods_str = format!("{} / {}", 
            translate_saju_ten_god(locale, m_luck.stem_god), 
            translate_saju_ten_god(locale, m_luck.branch_god)
        );
        s.push_str(&format!("| {} | {} | {} | {} | {} | {} |\n",
            m_luck.year, m_month_formatted, m_ganzi_str, m_gods_str,
            m_stage_trans, m_inf_trans
        ));
    }
    s.push_str("\n");

    s
}

pub fn export_tier_to_markdown(
    data: &TierResult,
    form: &crate::store::FormState,
    locale: Locale,
) -> String {
    let mut s = String::new();
    let title = match locale {
        Locale::Ko => "🏆 EON - 운명 티어 분석 보고서",
        Locale::En => "🏆 EON - Destiny Tier Analysis Report",
        Locale::Zh => "🏆 EON - 命运阶级分析报告",
        Locale::Ru => "🏆 EON - Отчет по уровню судьбы",
    };
    s.push_str(&format!("# {}\n\n", title));
    s.push_str(&format_global_header(form, locale));
    s.push_str("## 2. 운명 티어 분석 상세 결과\n\n");
    s.push_str(&format_tier_inner(data, locale));
    s
}

pub fn export_transit_to_markdown(
    data: &TransitAnalysisOutput,
    form: &crate::store::FormState,
    locale: Locale,
) -> String {
    let mut s = String::new();
    let title = match locale {
        Locale::Ko => "⏳ EON - 실시간 운세 분석 보고서",
        Locale::En => "⏳ EON - Real-time Transit Luck Analysis Report",
        Locale::Zh => "⏳ EON - 实时运势分析报告",
        Locale::Ru => "⏳ EON - Отчет по транзитной удаче",
    };
    s.push_str(&format!("# {}\n\n", title));
    s.push_str(&format_global_header(form, locale));
    s.push_str("## 2. 실시간 운세 분석 상세 결과\n\n");
    s.push_str(&format_transit_inner(data, locale));
    s
}


pub fn export_zwds_to_markdown(
    data: &ZwdsAnalysisOutput,
    form: &crate::store::FormState,
    locale: Locale,
) -> String {
    let mut s = String::new();
    let title = match locale {
        Locale::Ko => "🔮 EON - 자미두수 분석 보고서",
        Locale::En => "🔮 EON - Zi Wei Dou Shu Analysis Report",
        Locale::Zh => "🔮 EON - 紫微斗数分析报告",
        Locale::Ru => "🔮 EON - Отчет по Цзы Вэй Доу Шу",
    };
    s.push_str(&format!("# {}\n\n", title));
    s.push_str(&format_global_header(form, locale));
    s.push_str("## 2. 자미두수 분석 상세\n\n");
    s.push_str(&format_zwds_inner(data, locale));
    s
}

pub fn export_compatibility_to_markdown(
    compat: &eon_service::dto::VedicCompatibilityOutput,
    male_form: &crate::store::FormState,
    female_year: i32,
    female_month: u32,
    female_day: u32,
    female_hour: u32,
    female_minute: u32,
    female_lat: f64,
    female_lon: f64,
    locale: Locale,
) -> String {
    let mut s = String::new();
    let title = match locale {
        Locale::Ko => "✨ EON - 베딕 점성학 궁합(Compatibility) 분석 보고서",
        Locale::En => "✨ EON - Vedic Astrology Compatibility Analysis Report",
        Locale::Zh => "✨ EON - 吠陀占星八字合婚分析报告",
        Locale::Ru => "✨ EON - Отчет о совместимости в Ведической Астрологии",
    };
    s.push_str(&format!("# {}\n\n", title));

    let (lbl_birth_info, lbl_male, lbl_female, lbl_date, lbl_loc) = match locale {
        Locale::Ko => ("1. 출생 정보 (Birth Info)", "남성 (Male)", "여성 (Female)", "일시", "좌표"),
        Locale::En => ("1. Birth Information", "Male Partner", "Female Partner", "Date/Time", "Coords"),
        Locale::Zh => ("1. 出生信息", "男方", "女方", "日期/时间", "经纬度"),
        Locale::Ru => ("1. Информация о рождении", "Мужчина", "Женщина", "Дата/Время", "Координаты"),
    };

    s.push_str(&format!("## {}\n\n", lbl_birth_info));
    s.push_str(&format!("### {}\n", lbl_male));
    s.push_str(&format!("- **{}**: {}-{:02}-{:02} {:02}:{:02}\n", lbl_date, male_form.year, male_form.month, male_form.day, male_form.hour, male_form.minute));
    s.push_str(&format!("- **{}**: {:.4}°N, {:.4}°E\n\n", lbl_loc, male_form.lat, male_form.lon));

    s.push_str(&format!("### {}\n", lbl_female));
    s.push_str(&format!("- **{}**: {}-{:02}-{:02} {:02}:{:02}\n", lbl_date, female_year, female_month, female_day, female_hour, female_minute));
    s.push_str(&format!("- **{}**: {:.4}°N, {:.4}°E\n\n", lbl_loc, female_lat, female_lon));

    let (lbl_summary, lbl_score, lbl_status, lbl_exp) = match locale {
        Locale::Ko => ("2. 궁합 분석 요약 (Compatibility Summary)", "획득 점수", "최종 판단", "총평"),
        Locale::En => ("2. Compatibility Summary", "Total Guna Score", "Final Determination", "Explanation"),
        Locale::Zh => ("2. 合婚分析摘要", "获得分数", "最终判定", "解析"),
        Locale::Ru => ("2. Сводка совместимости", "Общий балл Гуна", "Итоговое решение", "Объяснение"),
    };

    s.push_str(&format!("## {}\n\n", lbl_summary));
    s.push_str(&format!("- **{}**: {:.1} / 36.0\n", lbl_score, compat.report.total_score));

    let status_str = if compat.report.is_compatible {
        match locale {
            Locale::Ko => "🟢 길 (조화로움)",
            Locale::En => "🟢 Compatible (Harmonious)",
            Locale::Zh => "🟢 合婚 (和谐)",
            Locale::Ru => "🟢 Совместимы (Гармония)",
        }
    } else {
        match locale {
            Locale::Ko => "🔴 흉 (불협화음 / 주의 필요)",
            Locale::En => "🔴 Incompatible (Requires Attention)",
            Locale::Zh => "🔴 不合 (需要注意)",
            Locale::Ru => "🔴 Несовместимы (Требуется внимание)",
        }
    };
    s.push_str(&format!("- **{}**: {}\n", lbl_status, status_str));

    let (lbl_mangal, lbl_male_mangal, lbl_female_mangal, lbl_mangal_cxl, txt_detected, txt_not_detected, txt_cancelled, txt_not_cancelled) = match locale {
        Locale::Ko => ("### 망갈 도샤 (Mangal Dosha / 화성 살성)", "남성 망갈 도샤", "여성 망갈 도샤", "상쇄 여부", "감지됨 (🚨)", "감지되지 않음 (✅)", "상쇄됨 (✅)", "상쇄되지 않음 (🚨)"),
        Locale::En => ("### Mangal Dosha (Martial Affliction)", "Male Mangal Dosha", "Female Mangal Dosha", "Cancellation Status", "Detected (🚨)", "Not Detected (✅)", "Cancelled (✅)", "Not Cancelled (🚨)"),
        Locale::Zh => ("### 火星煞 (Mangal Dosha)", "男方火星煞", "女方火星煞", "是否化解", "检测到 (🚨)", "未检测到 (✅)", "已化解 (✅)", "未化解 (🚨)"),
        Locale::Ru => ("### Мангалик Доша (Влияние Марса)", "Мангал Доша у мужчины", "Мангал Доша у женщины", "Статус отмены", "Обнаружено (🚨)", "Не обнаружено (✅)", "Отменено (✅)", "Не отменено (🚨)"),
    };
    s.push_str(&format!("{}\n\n", lbl_mangal));
    s.push_str(&format!("- **{}**: {}\n", lbl_male_mangal, if compat.report.male_mangal_dosha { txt_detected } else { txt_not_detected }));
    s.push_str(&format!("- **{}**: {}\n", lbl_female_mangal, if compat.report.female_mangal_dosha { txt_detected } else { txt_not_detected }));
    s.push_str(&format!("- **{}**: {}\n\n", lbl_mangal_cxl, if compat.report.mangal_dosha_cancelled { txt_cancelled } else { txt_not_cancelled }));

    s.push_str(&format!("- **{}**: {}\n\n", lbl_exp, compat.report.explanation));

    let (lbl_koota_title, col_koota_name, col_earned, col_max, col_desc) = match locale {
        Locale::Ko => ("3. 아슈타쿠타 8대 세부 항목 (Ashtakoota Breakdown)", "항목", "획득 점수", "만점", "설명"),
        Locale::En => ("3. Ashtakoota 8 Kootas Breakdown", "Koota Name", "Earned Score", "Max Score", "Description"),
        Locale::Zh => ("3. 八大项目评分 (Ashtakoota)", "项目名称", "获得分数", "最高分数", "解析"),
        Locale::Ru => ("3. Детализация 8 кут Аштакуты", "Название Куты", "Полученный балл", "Макс. балл", "Описание"),
    };

    s.push_str(&format!("## {}\n\n", lbl_koota_title));
    s.push_str(&format!("| {} | {} | {} | {} |\n", col_koota_name, col_earned, col_max, col_desc));
    s.push_str("| --- | --- | --- | --- |\n");

    for k in &compat.report.kootas {
        s.push_str(&format!("| **{}** | {:.1} | {:.1} | {} |\n", k.name, k.earned_points, k.max_points, k.description));
    }
    s.push_str("\n");

    s
}

pub fn format_iching_inner(data: &IChingAnalysisOutput, locale: Locale) -> String {
    let mut s = String::new();
    let res = &data.result;
    
    let pre_natal_hex = get_hexagram_info(res.pre_natal_hexagram);
    let post_natal_hex = get_hexagram_info(res.post_natal_hexagram);
    let yuan_dang = res.yuan_dang_yao;

    let (pre_name, pre_desc) = match locale {
        Locale::Ko => (pre_natal_hex.name, pre_natal_hex.desc_ko),
        Locale::En => (pre_natal_hex.name_en, pre_natal_hex.desc_en),
        Locale::Zh => (pre_natal_hex.name_zh, pre_natal_hex.desc_zh),
        Locale::Ru => (pre_natal_hex.name_ru, pre_natal_hex.desc_ru),
    };
    let (post_name, post_desc) = match locale {
        Locale::Ko => (post_natal_hex.name, post_natal_hex.desc_ko),
        Locale::En => (post_natal_hex.name_en, post_natal_hex.desc_en),
        Locale::Zh => (post_natal_hex.name_zh, post_natal_hex.desc_zh),
        Locale::Ru => (post_natal_hex.name_ru, post_natal_hex.desc_ru),
    };

    let (lbl_overview, lbl_pre_title, lbl_post_title, lbl_yd_title) = match locale {
        Locale::Ko => ("### ☯️ 선천괘 및 후천괘 개요 (Overview)", "선천괘 (先天卦 - Innate)", "후천괘 (後天卦 - Acquired)", "평생의 원당효 (Yuan Dang Yao)"),
        Locale::En => ("### ☯️ Hexagrams Overview", "Pre-Natal Hexagram (Innate)", "Post-Natal Hexagram (Acquired)", "Lifetime Yuan Dang Yao"),
        Locale::Zh => ("### ☯️ 卦象概要", "先天卦 (Innate)", "后天卦 (Acquired)", "终身元当爻"),
        Locale::Ru => ("### ☯️ Обзор гексаграмм", "Врожденная гексаграмма (Innate)", "Приобретенная гексаграмма (Acquired)", "Пожизненный Юань Дан Яо"),
    };

    s.push_str(&format!("{}\n\n", lbl_overview));
    s.push_str(&format!("- **{}**: **{} (n.{})** - {}\n", lbl_pre_title, pre_name, res.pre_natal_hexagram, pre_desc));
    s.push_str(&format!("- **{}**: **{} (n.{})** - {}\n", lbl_post_title, post_name, res.post_natal_hexagram, post_desc));
    s.push_str(&format!("- **{}**: **{} {}**\n\n", lbl_yd_title, yuan_dang, match locale {
        Locale::Ko => "효",
        Locale::En => "Yao",
        Locale::Zh => "爻",
        Locale::Ru => "Яо",
    }));

    let (col_age, col_phase, col_hex, col_yao, col_yd, lbl_prenatal, lbl_postnatal, lbl_yes, lbl_no) = match locale {
        Locale::Ko => ("연령대", "시기", "괘상", "효", "원당효 여부", "선천괘", "후천괘", "예 (★)", "아니오"),
        Locale::En => ("Age Range", "Phase", "Hexagram", "Yao Line", "Yuan Dang?", "Innate", "Acquired", "Yes (★)", "No"),
        Locale::Zh => ("年龄段", "时期", "卦象", "爻", "是否元当爻", "先天", "后天", "是 (★)", "否"),
        Locale::Ru => ("Возраст", "Период", "Гексаграмма", "Линия", "Юань Дан?", "Врожд.", "Приобр.", "Да (★)", "Нет"),
    };

    let lbl_timeline = match locale {
        Locale::Ko => "### 📅 하락이수 평생 대운 타임라인",
        Locale::En => "### 📅 He Luo Li Shu Lifetime Cycles",
        Locale::Zh => "### 📅 河洛理数终身大运时间线",
        Locale::Ru => "### 📅 Пожизненные циклы Хэ Ло Ли Шу",
    };
    s.push_str(&format!("{}\n\n", lbl_timeline));

    s.push_str(&format!("| {} | {} | {} | {} | {} |\n", col_age, col_phase, col_hex, col_yao, col_yd));
    s.push_str("| --- | --- | --- | --- | --- |\n");
    for cycle in &res.lifetime_cycles {
        let c_hex = get_hexagram_info(cycle.hexagram_index);
        let c_name = match locale {
            Locale::Ko => c_hex.name,
            Locale::En => c_hex.name_en,
            Locale::Zh => c_hex.name_zh,
            Locale::Ru => c_hex.name_ru,
        };
        let cycle_is_yang = (cycle.end_age - cycle.start_age + 1) == 9;
        let line_name = get_yao_name(cycle.line_index, cycle_is_yang, locale);
        let phase_str = if cycle.is_pre_natal { lbl_prenatal } else { lbl_postnatal };
        let is_yd = cycle.is_pre_natal && cycle.line_index == res.yuan_dang_yao;
        let yd_str = if is_yd { lbl_yes } else { lbl_no };
        s.push_str(&format!("| {} - {} | {} | {} (n.{}) | {} | {} |\n",
            cycle.start_age, cycle.end_age, phase_str, c_name, cycle.hexagram_index, line_name, yd_str));
    }
    s.push_str("\n");

    let lbl_detail_title = match locale {
        Locale::Ko => "### 🔍 대운 세부 해설 (Detailed Interpretation)",
        Locale::En => "### 🔍 Detailed Cycle Interpretations",
        Locale::Zh => "### 🔍 大运详细解析",
        Locale::Ru => "### 🔍 Подробное толкование циклов",
    };
    s.push_str(&format!("{}\n\n", lbl_detail_title));

    for cycle in &res.lifetime_cycles {
        let c_hex = get_hexagram_info(cycle.hexagram_index);
        let c_name = match locale {
            Locale::Ko => c_hex.name,
            Locale::En => c_hex.name_en,
            Locale::Zh => c_hex.name_zh,
            Locale::Ru => c_hex.name_ru,
        };
        let c_hanja = c_hex.hanja;
        let c_desc = match locale {
            Locale::Ko => c_hex.desc_ko,
            Locale::En => c_hex.desc_en,
            Locale::Zh => c_hex.desc_zh,
            Locale::Ru => c_hex.desc_ru,
        };
        let cycle_is_yang = (cycle.end_age - cycle.start_age + 1) == 9;
        let line_name = get_yao_name(cycle.line_index, cycle_is_yang, locale);
        let phase_str = if cycle.is_pre_natal { lbl_prenatal } else { lbl_postnatal };
        let is_yd = cycle.is_pre_natal && cycle.line_index == res.yuan_dang_yao;
        let yao_desc = get_yao_description(cycle.line_index, cycle_is_yang, is_yd, locale);

        s.push_str(&format!("#### 📅 [{} - {}] {} / {}\n\n", cycle.start_age, cycle.end_age, phase_str, c_name));
        s.push_str(&format!("- **{}**: {} ({})\n", col_hex, c_name, c_hanja));
        s.push_str(&format!("- **{}**: {}\n", col_yao, line_name));
        s.push_str(&format!("- **{}**: {}\n\n", match locale {
            Locale::Ko => "괘 설명",
            Locale::En => "Hexagram Description",
            Locale::Zh => "卦象说明",
            Locale::Ru => "Описание гексаграммы",
        }, c_desc));
        s.push_str(&format!("##### ✦ {}\n\n", match locale {
            Locale::Ko => "효사 해설",
            Locale::En => "Yao Line Interpretation",
            Locale::Zh => "爻辞解析",
            Locale::Ru => "Толкование линии",
        }));
        s.push_str(&format!("{}\n\n", yao_desc.trim()));
        s.push_str("---\n\n");
    }

    s
}

pub fn export_iching_to_markdown(
    data: &IChingAnalysisOutput,
    form: &crate::store::FormState,
    locale: Locale,
) -> String {
    let mut s = String::new();
    let title = match locale {
        Locale::Ko => "☯️ EON - 주역 / 하락이수 평생 괘상 분석 보고서",
        Locale::En => "☯️ EON - I Ching / He Luo Li Shu Lifetime Analysis Report",
        Locale::Zh => "☯️ EON - 周易 / 河洛理数终身卦象分析报告",
        Locale::Ru => "☯️ EON - Отчет по анализу гексаграмм И Цзин / Хэ Ло Ли Шу",
    };
    s.push_str(&format!("# {}\n\n", title));
    s.push_str(&format_global_header(form, locale));
    s.push_str("## 2. 주역 / 하락이수 분석 상세\n\n");
    s.push_str(&format_iching_inner(data, locale));
    s
}



pub fn export_western_to_markdown(
    data: &WesternAnalysisOutput,
    form: &crate::store::FormState,
    locale: Locale,
) -> String {
    let mut s = String::new();
    let title = match locale {
        Locale::Ko => "🪐 EON - 서양 점성학 천체 차트 분석 보고서",
        Locale::En => "🪐 EON - Western Astrology Natal Chart Analysis Report",
        Locale::Zh => "🪐 EON - 西洋占星天体星盘 analysis 报告",
        Locale::Ru => "🪐 EON - Отчет по анализу натальной карты западной астрологии",
    };
    s.push_str(&format!("# {}\n\n", title));
    s.push_str(&format_global_header(form, locale));
    s.push_str("## 2. 서양 점성학 분석 상세\n\n");
    s.push_str(&format_western_inner(data, locale));
    s
}

pub fn format_western_inner(data: &WesternAnalysisOutput, locale: Locale) -> String {
    let mut s = String::new();
    let res = &data.result;
    
    // ASC sign and degree
    let asc_sign_idx = (res.ascendant / 30.0).floor() as usize;
    let (_, asc_sign_name) = crate::components::tabs::western_tab::get_sign_emoji_and_name(asc_sign_idx, locale);
    let asc_deg = res.ascendant % 30.0;
    let asc_deg_str = format!("{:.0}° {:.0}'", asc_deg.floor(), (asc_deg.fract() * 60.0).round());
    
    let (_, cr_name) = crate::components::tabs::western_tab::get_planet_emoji_and_name(&res.chart_ruler, locale);
    
    let (lbl_overview, lbl_asc, lbl_cr, lbl_dom_el, lbl_dom_mo) = match locale {
        Locale::Ko => ("### 🪐 핵심 천체 지표 요약", "상승궁 (Ascendant)", "상승성주 (Chart Ruler)", "우세 원소 (Dominant Element)", "우세 양태 (Dominant Modality)"),
        _ => ("### 🪐 Key Celestial Indicators", "Ascendant (ASC)", "Chart Ruler", "Dominant Element", "Dominant Modality"),
    };
    
    s.push_str(&format!("{}\n\n", lbl_overview));
    s.push_str(&format!("- **{}**: {} ({})\n", lbl_asc, asc_sign_name, asc_deg_str));
    s.push_str(&format!("- **{}**: {}\n", lbl_cr, cr_name));
    s.push_str(&format!("- **{}**: {}\n", lbl_dom_el, res.dominant_element));
    s.push_str(&format!("- **{}**: {}\n\n", lbl_dom_mo, res.dominant_modality));

    // Elements & Modalities distribution
    let (lbl_dist, lbl_el, lbl_mo) = match locale {
        Locale::Ko => ("### 📊 성향 지표 분포", "4대 원소 (Elements) 분포", "3대 양태 (Modalities) 분포"),
        _ => ("### 📊 Temperament Distributions", "4 Elements Distribution", "3 Modalities Distribution"),
    };
    s.push_str(&format!("{}\n\n", lbl_dist));
    s.push_str(&format!("#### {}\n", lbl_el));
    s.push_str(&format!("- **Fire (불)**: {:.1}%\n", res.elements.fire));
    s.push_str(&format!("- **Earth (흙)**: {:.1}%\n", res.elements.earth));
    s.push_str(&format!("- **Air (공기)**: {:.1}%\n", res.elements.air));
    s.push_str(&format!("- **Water (물)**: {:.1}%\n\n", res.elements.water));

    s.push_str(&format!("#### {}\n", lbl_mo));
    s.push_str(&format!("- **Cardinal (활동)**: {:.1}%\n", res.modalities.cardinal));
    s.push_str(&format!("- **Fixed (고정)**: {:.1}%\n", res.modalities.fixed));
    s.push_str(&format!("- **Mutable (변통)**: {:.1}%\n\n", res.modalities.mutable));

    // Planet positions table
    let (lbl_planets, col_planet, col_sign, col_deg, col_house) = match locale {
        Locale::Ko => ("### 🪐 행성별 배치 상세 (Planet Positions)", "행성", "사인 (Sign)", "도수 (Degree)", "하우스 (House)"),
        _ => ("### 🪐 Planet Positions", "Planet", "Zodiac Sign", "Degree", "House"),
    };
    s.push_str(&format!("{}\n\n", lbl_planets));
    s.push_str(&format!("| {} | {} | {} | {} |\n", col_planet, col_sign, col_deg, col_house));
    s.push_str("| --- | --- | --- | --- |\n");
    for p in &res.planets {
        let (p_emoji, p_name) = crate::components::tabs::western_tab::get_planet_emoji_and_name(&p.name, locale);
        let (s_emoji, s_name) = crate::components::tabs::western_tab::get_sign_emoji_and_name(p.sign_index, locale);
        let p_deg = p.degree_in_sign;
        let p_deg_str = format!("{:.0}° {:.0}'", p_deg.floor(), (p_deg.fract() * 60.0).round());
        let rx = if p.is_retrograde { " ℞" } else { "" };
        s.push_str(&format!("| {} {} | {} {} | {}{} | 제 {} 하우스 |\n", p_emoji, p_name, s_emoji, s_name, p_deg_str, rx, p.house_number));
    }
    s.push_str("\n");

    // House cusps table
    let (lbl_cusps, col_cusp) = match locale {
        Locale::Ko => ("### 🏠 하우스 경계 좌표 (House Cusps)", "하우스"),
        _ => ("### 🏠 House Cusps", "House"),
    };
    s.push_str(&format!("{}\n\n", lbl_cusps));
    s.push_str(&format!("| {} | {} | {} |\n", col_cusp, col_sign, col_deg));
    s.push_str("| --- | --- | --- |\n");
    for h in &res.houses {
        let (s_emoji, s_name) = crate::components::tabs::western_tab::get_sign_emoji_and_name(h.sign_index, locale);
        let h_deg = h.degree_in_sign;
        let h_deg_str = format!("{:.0}° {:.0}'", h_deg.floor(), (h_deg.fract() * 60.0).round());
        s.push_str(&format!("| 제 {} 하우스 | {} {} | {} |\n", h.house_number, s_emoji, s_name, h_deg_str));
    }
    s.push_str("\n");

    // Aspects table
    let (lbl_aspects, col_body_a, col_body_b, col_aspect, col_orb) = match locale {
        Locale::Ko => ("### 📐 행성 간 아스펙트 (Aspects)", "천체 A", "천체 B", "관계 (Aspect)", "오차 (Orb)"),
        _ => ("### 📐 Planetary Aspects", "Celestial A", "Celestial B", "Aspect Type", "Orb"),
    };
    s.push_str(&format!("{}\n\n", lbl_aspects));
    s.push_str(&format!("| {} | {} | {} | {} |\n", col_body_a, col_body_b, col_aspect, col_orb));
    s.push_str("| --- | --- | --- | --- |\n");
    for asp in &res.aspects {
        let (_, b_a_name) = crate::components::tabs::western_tab::get_planet_emoji_and_name(&asp.body_a_name, locale);
        let (_, b_b_name) = crate::components::tabs::western_tab::get_planet_emoji_and_name(&asp.body_b_name, locale);
        let (asp_emoji, asp_name) = crate::components::tabs::western_tab::get_aspect_emoji_and_name(asp.aspect_type, locale);
        s.push_str(&format!("| {} | {} | {} {} | {:.2}° |\n", b_a_name, b_b_name, asp_emoji, asp_name, asp.orb));
    }
    s.push_str("\n");

    s
}

