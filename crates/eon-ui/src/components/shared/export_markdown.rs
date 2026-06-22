use crate::store::{AnalysisState, TaskStatus};
use crate::i18n::{t, TK, Locale, translate_planet};
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
        Locale::Ru => "🌌 EON - Отчет по анализу Бацзы",
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
