use crate::store::{AnalysisState, TaskStatus};
use crate::i18n::{t, TK, Locale, translate_planet, translate_planet_str, translate_avastha, rasi_name};
use eon_service::dto::{SajuAnalysisOutput, VedicAnalysisOutput};
use eon_vedic::planets::VedicPlanet;
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
    
    let (stem_row, branch_row, god_stem_row, god_branch_row, stage_row) = match locale {
        Locale::Ko => ("천간 (Heavenly Stem)", "지지 (Earthly Branch)", "천간 십성", "지지 십성", "12운성 (Twelve Stage)"),
        Locale::En => ("Heavenly Stem", "Earthly Branch", "Stem Ten-God", "Branch Ten-God", "Twelve Stage"),
        Locale::Zh => ("天干", "地支", "天干十神", "地支十神", "十二运星"),
        Locale::Ru => ("Небесный ствол", "Земная ветвь", "Божество Ствола", "Божество Ветви", "12 стадий судьбы"),
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
    s.push_str(&format!("| **{}** | {} | {} | {} | {} |\n\n", 
        stage_row, stages.hour_stage.hangul(), stages.day_stage.hangul(),
        stages.month_stage.hangul(), stages.year_stage.hangul()
    ));

    // Day Master & Strength
    let (dm_title, dm_lbl, type_lbl, score_lbl, yong_lbl, hee_lbl, deuk_ryeong_lbl, deuk_ji_lbl, deuk_se_lbl) = match locale {
        Locale::Ko => ("일간 세력 및 용희신", "일간", "신강/신약 유형", "세력 점수", "용신 (用神)", "희신 (喜神)", "득령", "득지", "득세"),
        Locale::En => ("Day Master Strength & Yongshin/Heeshin", "Day Master", "Strength Type", "Strength Score", "Yongshin (Deity)", "Heeshin (Assistant)", "Deuk-Ryeong", "Deuk-Ji", "Deuk-Se"),
        Locale::Zh => ("日主强弱与用喜神", "日主", "强弱类型", "能量分数", "用神", "喜神", "得令", "得地", "得势"),
        Locale::Ru => ("Сила Дневного Доминанта и Полезные Божества", "Дневной Доминант", "Тип силы", "Балл силы", "Полезный Дух", "Благоприятный Дух", "Дэ-Рён", "Дэ-Джи", "Дэ-Сэ"),
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
    let ds_val = if rep.strength.deuk_se.acquired { "Yes" } else { "No" };

    s.push_str(&format!("### {}\n\n", dm_title));
    s.push_str(&format!("- **{}**: {}({})\n", dm_lbl, rep.strength.day_master.hangul(), rep.strength.day_master.hanja()));
    s.push_str(&format!("- **{}**: {}\n", type_lbl, str_type_str));
    s.push_str(&format!("- **{}**: {:.1} / 100\n", score_lbl, rep.strength.strength_score));
    s.push_str(&format!("- **{}/{}/{}**: {} / {} / {}\n", deuk_ryeong_lbl, deuk_ji_lbl, deuk_se_lbl, dr_val, dj_val, ds_val));
    s.push_str(&format!("- **{}**: {}\n", yong_lbl, rep.yongshin.primary.hangul()));
    s.push_str(&format!("- **{}**: {}\n\n", hee_lbl, rep.yongshin.assistant.hangul()));

    // Diagnostics
    let (diag_title, struct_lbl, unpacker_lbl, bottleneck_lbl, complexity_lbl, grade_lbl, crisis_lbl) = match locale {
        Locale::Ko => ("오행 흐름 및 구조 진단", "격국", "오행 해결사", "오행 정체", "구조 복잡도", "안정성 등급", "운이 꺾이는 시기"),
        Locale::En => ("Qi Flow & Structural Diagnostics", "Structure", "Qi Unpacker", "Qi Bottleneck", "Structural Complexity", "Stability Grade", "Crisis Period Count"),
        Locale::Zh => ("五行流通与结构诊断", "格局", "五行通关", "五行阻滞", "结构复杂度", "稳定度评级", "危机时期数量"),
        Locale::Ru => ("Поток Ци и Диагностика Структуры", "Структура", "Разрешитель Ци", "Затор Ци", "Сложность структуры", "Класс стабильности", "Периоды спада"),
    };

    let structure_str = format!("{:?}", rep.structure.structure);
    let unpacker_str = data.entropy.unpacker_element.map(|el| el.hangul().to_string()).unwrap_or_else(|| "—".to_string());
    let bottleneck_str = data.qi_topology.bottleneck.map(|el| el.hangul().to_string()).unwrap_or_else(|| "—".to_string());
    let (comp_val, stab_val) = if let Some(comp) = &data.complexity {
        (comp.cyclomatic_complexity.to_string(), comp.stability_grade.clone())
    } else {
        ("—".to_string(), "—".to_string())
    };

    s.push_str(&format!("### {}\n\n", diag_title));
    s.push_str(&format!("- **{}**: {}\n", struct_lbl, structure_str));
    s.push_str(&format!("- **{}**: {}\n", unpacker_lbl, unpacker_str));
    s.push_str(&format!("- **{}**: {}\n", bottleneck_lbl, bottleneck_str));
    s.push_str(&format!("- **{}**: {}\n", complexity_lbl, comp_val));
    s.push_str(&format!("- **{}**: {}\n", grade_lbl, stab_val));
    s.push_str(&format!("- **{}**: {} times detected\n\n", crisis_lbl, data.crash_count));

    // Power Analysis (오행 및 십성 점수)
    let (power_title, dominant_el_lbl, dominant_tg_lbl, percentage_col, score_col) = match locale {
        Locale::Ko => ("오행 및 십성 세력 상세 분석 (Power Analysis)", "대표 오행", "대표 십성", "비율", "점수"),
        Locale::En => ("Element & Ten God Power Analysis", "Dominant Element", "Dominant Ten God", "Percentage", "Score"),
        Locale::Zh => ("五行与十神力量详细分析", "代表五行", "代表十神", "比例", "分数"),
        Locale::Ru => ("Подробный анализ сил Первоэлементов и Божеств", "Доминирующий элемент", "Доминирующее Божество", "Процент", "Балл"),
    };
    s.push_str(&format!("### {}\n\n", power_title));
    s.push_str(&format!("- **{}**: {}\n", dominant_el_lbl, rep.power.dominant_element.hangul()));
    s.push_str(&format!("- **{}**: {}\n\n", dominant_tg_lbl, rep.power.dominant_ten_god.hangul()));

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
        s.push_str(&format!("| {}({}) | {:.1}% | {:.1} |\n", el.hangul(), el.hanja(), pct, score));
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
        s.push_str(&format!("| {} | {:.1}% | {:.1} |\n", tg.hangul(), pct, score));
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
        rep.voids.void_branches[0].hangul(), rep.voids.void_branches[0].hanja(),
        rep.voids.void_branches[1].hangul(), rep.voids.void_branches[1].hanja()
    ));
    if !rep.voids.mapped_voids.is_empty() {
        s.push_str("\n**세부 공망 분석 (Void Details)**:\n");
        for mv in &rep.voids.mapped_voids {
            s.push_str(&format!("- **{} {}({}) [{}]**: {} - *{}*\n", 
                mv.position, mv.branch.hangul(), mv.branch.hanja(), mv.ten_god.hangul(), mv.summary, mv.description));
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
            s.push_str(&format!("| {} | **{}** | {} | {} ({}) |\n", 
                rel.relation_type, rel.name, rel.positions.join("-"), rel.summary, rel.description));
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
            s.push_str(&format!("| {} | **{}** | {} - *{}* |\n", pos_str, m.marker.hangul(), m.summary, m.description));
        }
    } else {
        s.push_str("—\n");
    }
    s.push_str("\n");

    // Vulnerability/Fuzzer & Load Diagnostics
    let (fuzzer_title, load_title) = match locale {
        Locale::Ko => ("운명 크래시 분석 (Destiny Fuzzer)", "인생 부하 진단 (Karma Load Balancer)"),
        Locale::En => ("Destiny Fuzzer (Vulnerability Analysis)", "Karma Load Balancer Diagnostics"),
        Locale::Zh => ("命运漏洞分析 (Fuzzer)", "人生负荷均衡诊断"),
        Locale::Ru => ("Анализ Уязвимостей Судьбы", "Диагностика Кармической Нагрузки"),
    };
    s.push_str(&format!("### {}\n\n", fuzzer_title));
    if let Some(fuzz) = &data.vulnerability_report {
        for vuln in &fuzz.critical_vectors {
            let major_gz = format!("{}{}", vuln.vector.major.hanja(), vuln.vector.major.hangul());
            let yearly_gz = format!("{}{}", vuln.vector.yearly.hanja(), vuln.vector.yearly.hangul());
            s.push_str(&format!("- **[⚠️ {}]** (Score: {:.1}): Major: {}, Yearly: {} (Tags: {})\n", 
                vuln.vulnerability_type, vuln.crash_score, major_gz, yearly_gz, vuln.tags.join(", ")));
        }
    } else {
        s.push_str("—\n");
    }
    s.push_str("\n");

    s.push_str(&format!("### {}\n\n", load_title));
    if !data.load_diagnostics.is_empty() {
        for diag in &data.load_diagnostics {
            let status_str = match diag.status {
                eon_saju::engine::load_balancer::TrafficStatus::Idle => "Idle",
                eon_saju::engine::load_balancer::TrafficStatus::Normal => "Normal",
                eon_saju::engine::load_balancer::TrafficStatus::Overloaded => "Overloaded",
                eon_saju::engine::load_balancer::TrafficStatus::SystemDown => "SystemDown",
            };
            s.push_str(&format!("- **Age {} [{}]**: *{}* -> Advice: {}\n", diag.age, status_str, diag.reason, diag.strategy));
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
            s.push_str(&format!("- **{} {} ~ {}**: {}{} 대운 ({}/{})\n", 
                age_lbl, cycle.start_age, cycle.end_age, 
                cycle.ganzi.stem.hangul(), cycle.ganzi.branch.hangul(),
                cycle.stem_god.hangul(), cycle.branch_god.hangul()
            ));
        }
    } else {
        s.push_str("—\n");
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
    s.push_str(&format!("- **{}**: {}\n\n", sade_sati_lbl, ss_phase_str));

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
    s.push_str(&format!("- **Sunrise/Sunset (일출/일몰)**: {} / {} ({})\n\n", 
        pan.sunrise.format("%H:%M:%S"), pan.sunset.format("%H:%M:%S"),
        if pan.is_day_birth { "Day Birth ☀️" } else { "Night Birth 🌙" }
    ));

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

    // Vimshottari Dasha
    let (dasha_title, dasha_years_lbl) = match locale {
        Locale::Ko => ("빔쇼타리 다샤 흐름 (Vimshottari Dasha)", "년"),
        Locale::En => ("Vimshottari Dasha Timeline", "years"),
        Locale::Zh => ("维姆绍塔里大运", "年"),
        Locale::Ru => ("Временные периоды Вимшоттари Даша", "лет"),
    };

    s.push_str(&format!("### {}\n\n", dasha_title));
    for period in &data.report.dasha_timeline {
        let start_fmt = period.start_time.format("%Y-%m-%d").to_string();
        let end_fmt = period.end_time.format("%Y-%m-%d").to_string();
        let p_name = translate_planet(locale, period.lord);
        let duration_years = (period.end_time - period.start_time).num_days() as f64 / 365.25;
        s.push_str(&format!("- **{} ({} ~ {})**: {:.1} {}\n", p_name, start_fmt, end_fmt, duration_years, dasha_years_lbl));
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
    form: &crate::store::FormState,
    locale: Locale,
) -> String {
    let mut s = String::new();
    let title = match locale {
        Locale::Ko => "🌌✨ EON - 사주 및 베딕 통합 분석 보고서",
        Locale::En => "🌌✨ EON - Saju & Vedic Integrated Analysis Report",
        Locale::Zh => "🌌✨ EON - 八字与吠陀占星整合分析报告",
        Locale::Ru => "🌌✨ EON - Интегрированный отчет по Бацзы и Ведической Астрологии",
    };
    s.push_str(&format!("# {}\n\n", title));
    s.push_str(&format_global_header(form, locale));
    
    if let Some(saju_data) = saju {
        let saju_title = match locale {
            Locale::Ko => "2. 사주 분석 상세 결과 (Saju Analysis)",
            Locale::En => "2. Saju Analysis Details",
            Locale::Zh => "2. 八字分析详细结果",
            Locale::Ru => "2. Подробные результаты анализа Бацзы",
        };
        s.push_str(&format!("## {}\n\n", saju_title));
        s.push_str(&format_saju_inner(saju_data, locale));
    }
    
    if let Some(vedic_data) = vedic {
        let vedic_title = match locale {
            Locale::Ko => "3. 베딕 분석 상세 결과 (Vedic Analysis)",
            Locale::En => "3. Vedic Analysis Details",
            Locale::Zh => "3. 吠陀分析详细结果",
            Locale::Ru => "3. Подробные результаты Ведического анализа",
        };
        s.push_str(&format!("## {}\n\n", vedic_title));
        s.push_str(&format_vedic_inner(vedic_data, locale));
    }
    
    s
}

#[component]
pub fn ExportWidget() -> Element {
    let state = use_context::<AnalysisState>();
    let locale = *state.locale.read();

    let saju_state = state.saju.read();
    let vedic_state = state.vedic.read();
    let form = state.form.read().clone();

    let has_saju = saju_state.status == TaskStatus::Success && saju_state.data.is_some();
    let has_vedic = vedic_state.status == TaskStatus::Success && vedic_state.data.is_some();

    let saju_data = saju_state.data.clone();
    let vedic_data = vedic_state.data.clone();

    let mut copied_saju = use_signal(|| false);
    let mut copied_vedic = use_signal(|| false);
    let mut copied_combined = use_signal(|| false);

    let widget_title = match locale {
        Locale::Ko => "분석 결과 내보내기",
        Locale::En => "EXPORT REPORT",
        Locale::Zh => "导出分析报告",
        Locale::Ru => "ЭКСПОРТ ОТЧЕТА",
    };

    let form_cloned_saju = form.clone();
    let form_cloned_vedic = form.clone();
    let form_cloned_comb = form.clone();

    let saju_data_cloned_saju = saju_data.clone();
    let saju_data_cloned_comb = saju_data.clone();

    let vedic_data_cloned_vedic = vedic_data.clone();
    let vedic_data_cloned_comb = vedic_data.clone();

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
                span { "📝 {t(locale, TK::BtnCopySajuMarkdown)}" }
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
                span { "✨ {t(locale, TK::BtnCopyVedicMarkdown)}" }
                if *copied_vedic.read() {
                    span { class: "text-[10px] text-emerald-400 font-bold transition-all duration-300 animate-pulse", "{t(locale, TK::MsgCopiedToClipboard)}" }
                } else {
                    span { class: "text-[10px] text-slate-500", "Markdown" }
                }
            }

            // 3. Copy Combined
            button {
                class: if has_saju || has_vedic {
                    "w-full text-xs font-semibold py-2 px-3 rounded-lg border transition-all duration-200 cursor-pointer flex items-center justify-between bg-gradient-to-r from-violet-900/20 to-indigo-900/20 border-violet-800/40 text-violet-300 hover:from-violet-850/40 hover:to-indigo-850/40 hover:text-white hover:border-violet-600/50"
                } else {
                    "w-full text-xs font-semibold py-2 px-3 rounded-lg border flex items-center justify-between bg-slate-900/20 border-slate-800/40 text-slate-600 cursor-not-allowed opacity-40"
                },
                disabled: !has_saju && !has_vedic,
                onclick: move |_| {
                    let txt = export_combined_to_markdown(saju_data_cloned_comb.as_ref(), vedic_data_cloned_comb.as_ref(), &form_cloned_comb, locale);
                    copy_to_clipboard(&txt);
                    copied_combined.set(true);
                    spawn(async move {
                        gloo_timers::future::TimeoutFuture::new(2000).await;
                        copied_combined.set(false);
                    });
                },
                span { "🌌✨ {t(locale, TK::BtnCopyCombinedMarkdown)}" }
                if *copied_combined.read() {
                    span { class: "text-[10px] text-emerald-400 font-bold transition-all duration-300 animate-pulse", "{t(locale, TK::MsgCopiedToClipboard)}" }
                } else {
                    span { class: "text-[10px] text-violet-400/80", "All" }
                }
            }
        }
    }
}
