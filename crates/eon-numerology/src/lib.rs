//! Numerology engine (Pythagorean and Chaldean letter systems).
//!
//! Everything here is pure and deterministic: identical inputs always give identical results.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Letter-to-number table used for the name based numbers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NumerologySystem {
    /// A=1 .. I=9, J=1 .. (default).
    #[default]
    Pythagorean,
    /// Chaldean table: values 1..=8 only (9 is sacred and never assigned to a letter).
    Chaldean,
}

impl NumerologySystem {
    pub fn label(self) -> &'static str {
        match self {
            NumerologySystem::Pythagorean => "Pythagorean",
            NumerologySystem::Chaldean => "Chaldean",
        }
    }

    /// Highest number a letter can have in this system.
    pub fn max_letter_value(self) -> u8 {
        match self {
            NumerologySystem::Pythagorean => 9,
            NumerologySystem::Chaldean => 8,
        }
    }

    /// Value of a Latin letter in this system (0 for anything that is not A-Z).
    pub fn letter_value(self, ch: char) -> u8 {
        match self {
            NumerologySystem::Pythagorean => letter_value(ch),
            NumerologySystem::Chaldean => chaldean_letter_value(ch),
        }
    }
}

impl fmt::Display for NumerologySystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            NumerologySystem::Pythagorean => "pythagorean",
            NumerologySystem::Chaldean => "chaldean",
        })
    }
}

impl FromStr for NumerologySystem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "pythagorean" => Ok(NumerologySystem::Pythagorean),
            "chaldean" => Ok(NumerologySystem::Chaldean),
            other => Err(format!(
                "Invalid numerology system '{other}'. Expected pythagorean or chaldean"
            )),
        }
    }
}

/// Date a Personal Year / Month / Day is calculated for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TargetDate {
    pub year: u32,
    /// Calendar month 1..=12; needed for the Personal Month.
    pub month: Option<u32>,
    /// Calendar day 1..=31; needed (together with `month`) for the Personal Day.
    pub day: Option<u32>,
}

impl TargetDate {
    /// Only the year is known: Personal Year, no Personal Month / Day.
    pub fn year_only(year: u32) -> Self {
        Self {
            year,
            month: None,
            day: None,
        }
    }

    pub fn full(year: u32, month: u32, day: u32) -> Self {
        Self {
            year,
            month: Some(month),
            day: Some(day),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NumerologyCoreNumbers {
    /// Life Path Number
    pub life_path: u8,
    /// Expression / Destiny Number
    pub expression: Option<u8>,
    /// Soul Urge / Heart's Desire Number
    pub soul_urge: Option<u8>,
    /// Personality Number
    pub personality: Option<u8>,
    /// Birthday Number
    pub birthday: u8,
    /// Maturity Number: Life Path + Expression (needs a name)
    #[serde(default)]
    pub maturity: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PinnaclePeriod {
    pub stage: u8,
    pub pinnacle_number: u8,
    pub challenge_number: u8,
    pub start_age: u32,
    pub end_age: u32,
    pub description_ko: String,
    pub description_en: String,
}

/// A karmic debt number (13, 14, 16 or 19) found in an unreduced intermediate sum.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct KarmicDebt {
    /// 13, 14, 16 or 19
    pub debt_number: u8,
    /// The single digit it reduces to (4, 5, 7, 1)
    pub reduces_to: u8,
    /// Where it was found: lifePath, birthday, expression, soulUrge or personality
    pub source: String,
    pub meaning_en: String,
    pub meaning_ko: String,
}

/// A letter of the name with its number.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NameLetter {
    pub letter: char,
    pub value: u8,
}

/// Numbers that only depend on the letters of the name.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NameInsights {
    /// How often each number 1..=9 occurs among the letters (index 0 = number 1).
    pub number_counts: [u32; 9],
    /// Hidden Passion: the number(s) occurring most often in the name (ties are all listed).
    pub hidden_passions: Vec<u8>,
    /// Balance Number: the initials of every name word added up and reduced to a single digit.
    pub balance: u8,
    /// First letter of the first name.
    pub cornerstone: NameLetter,
    /// Last letter of the first name.
    pub capstone: NameLetter,
    /// Karmic Lessons: the numbers (1..=9, 1..=8 in Chaldean) that no letter of the name carries.
    pub missing_numbers: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NumerologyResult {
    /// Letter system used for the name numbers.
    #[serde(default)]
    pub system: NumerologySystem,
    pub core: NumerologyCoreNumbers,
    pub pinnacles: Vec<PinnaclePeriod>,
    pub personal_year: u8,
    /// Personal Month for the target date (None when only a target year was given).
    #[serde(default)]
    pub personal_month: Option<u8>,
    /// Personal Day for the target date (None when only a target year was given).
    #[serde(default)]
    pub personal_day: Option<u8>,
    pub is_master_life_path: bool,
    /// Karmic debts found in Life Path, Birthday and the name numbers.
    #[serde(default)]
    pub karmic_debts: Vec<KarmicDebt>,
    /// Name based extras (hidden passion, balance, cornerstone/capstone, missing numbers).
    #[serde(default)]
    pub name_insights: Option<NameInsights>,
    pub summary_ko: String,
    pub summary_en: String,
}

/// How well one pair of numbers (e.g. the two Life Paths) fits together.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NumberPairScore {
    pub number_a: u8,
    pub number_b: u8,
    /// 0..=100 heuristic score
    pub score: u8,
    /// same-number, same-group, complementary-groups or contrasting-groups
    pub relation: String,
    pub description_en: String,
    pub description_ko: String,
}

/// Two-person numerology compatibility (a heuristic, see [`compare_numerology`]).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NumerologyCompatibility {
    pub system: NumerologySystem,
    pub life_path: NumberPairScore,
    pub expression: Option<NumberPairScore>,
    pub soul_urge: Option<NumberPairScore>,
    /// Weighted total, 0..=100
    pub total_score: f64,
    pub methodology: String,
    pub summary_en: String,
    pub summary_ko: String,
}

/// Reduces a number down to a single digit (1..=9) or Master Number (11, 22, 33)
pub fn reduce_number(mut num: u32, preserve_master: bool) -> u8 {
    while num > 9 {
        if preserve_master && (num == 11 || num == 22 || num == 33) {
            return num as u8;
        }
        let sum: u32 = num.to_string().chars().filter_map(|c| c.to_digit(10)).sum();
        num = sum;
    }
    num as u8
}

/// Converts a letter to Pythagorean Numerology number (1..=9)
pub fn letter_value(ch: char) -> u8 {
    match ch.to_ascii_uppercase() {
        'A' | 'J' | 'S' => 1,
        'B' | 'K' | 'T' => 2,
        'C' | 'L' | 'U' => 3,
        'D' | 'M' | 'V' => 4,
        'E' | 'N' | 'W' => 5,
        'F' | 'O' | 'X' => 6,
        'G' | 'P' | 'Y' => 7,
        'H' | 'Q' | 'Z' => 8,
        'I' | 'R' => 9,
        _ => 0,
    }
}

/// Converts a letter to its Chaldean number (1..=8; 9 is never assigned).
pub fn chaldean_letter_value(ch: char) -> u8 {
    match ch.to_ascii_uppercase() {
        'A' | 'I' | 'J' | 'Q' | 'Y' => 1,
        'B' | 'K' | 'R' => 2,
        'C' | 'G' | 'L' | 'S' => 3,
        'D' | 'M' | 'T' => 4,
        'E' | 'H' | 'N' | 'X' => 5,
        'U'..='W' => 6,
        'O' | 'Z' => 7,
        'F' | 'P' => 8,
        _ => 0,
    }
}

pub fn is_vowel(ch: char) -> bool {
    matches!(ch.to_ascii_uppercase(), 'A' | 'E' | 'I' | 'O' | 'U')
}

const KARMIC_DEBT_NUMBERS: [u32; 4] = [13, 14, 16, 19];

/// Looks for a karmic debt number (13, 14, 16, 19) in `num` and in every intermediate sum
/// produced while reducing it (e.g. 19 -> 10 -> 1 contains the debt 19).
fn find_karmic_debt(mut num: u32, preserve_master: bool) -> Option<u8> {
    loop {
        if KARMIC_DEBT_NUMBERS.contains(&num) {
            return Some(num as u8);
        }
        if num <= 9 || (preserve_master && matches!(num, 11 | 22 | 33)) {
            return None;
        }
        num = num.to_string().chars().filter_map(|c| c.to_digit(10)).sum();
    }
}

fn karmic_debt_meaning(debt: u8) -> (&'static str, &'static str) {
    match debt {
        13 => (
            "Karmic debt 13/4: past misuse of time and effort (laziness, shortcuts). Lesson: steady, disciplined work and patience.",
            "카르마 부채 13/4: 시간과 노력을 소홀히 한 과거의 업. 과제는 꾸준함, 규율, 인내로 일을 끝까지 해내는 것.",
        ),
        14 => (
            "Karmic debt 14/5: past abuse of freedom (excess, indulgence). Lesson: moderation, commitment and responsible use of freedom.",
            "카르마 부채 14/5: 자유를 남용하고 과도하게 탐닉한 과거의 업. 과제는 절제, 책임 있는 자유, 변화 속의 중심 잡기.",
        ),
        16 => (
            "Karmic debt 16/7: past ego, pride or misuse of love. Lesson: humility, rebuilding after upheaval and trust in a higher order.",
            "카르마 부채 16/7: 자만, 오만, 사랑의 오용이라는 과거의 업. 과제는 겸손, 무너진 뒤의 재건, 더 큰 질서에 대한 신뢰.",
        ),
        19 => (
            "Karmic debt 19/1: past abuse of power or selfishness. Lesson: independence with consideration, accepting help and using power for others.",
            "카르마 부채 19/1: 힘의 남용과 이기심이라는 과거의 업. 과제는 배려하는 독립심, 도움을 받아들이는 법, 힘을 타인을 위해 쓰는 것.",
        ),
        _ => ("", ""),
    }
}

fn karmic_debt(debt: u8, source: &str) -> KarmicDebt {
    let (en, ko) = karmic_debt_meaning(debt);
    KarmicDebt {
        debt_number: debt,
        reduces_to: reduce_number(debt as u32, false),
        source: source.to_string(),
        meaning_en: en.to_string(),
        meaning_ko: ko.to_string(),
    }
}

/// Calculates Full Pythagorean Numerology Analysis (Personal Year only; see
/// [`calculate_numerology_ext`] for Personal Month/Day and the Chaldean system).
pub fn calculate_numerology(
    year: u32,
    month: u32,
    day: u32,
    name_latin: Option<&str>,
    target_year: u32,
) -> NumerologyResult {
    calculate_numerology_ext(
        year,
        month,
        day,
        name_latin,
        TargetDate::year_only(target_year),
        NumerologySystem::Pythagorean,
    )
}

/// Full numerology analysis for a letter system and a target date.
pub fn calculate_numerology_ext(
    year: u32,
    month: u32,
    day: u32,
    name_latin: Option<&str>,
    target: TargetDate,
    system: NumerologySystem,
) -> NumerologyResult {
    let target_year = target.year;
    let mut karmic_debts: Vec<KarmicDebt> = Vec::new();

    // 1. Life Path Number
    let m_red = reduce_number(month, true) as u32;
    let d_red = reduce_number(day, true) as u32;
    let y_red = reduce_number(year, true) as u32;
    let life_path_sum = m_red + d_red + y_red;
    let life_path = reduce_number(life_path_sum, true);
    let is_master_life_path = matches!(life_path, 11 | 22 | 33);
    if let Some(d) = find_karmic_debt(life_path_sum, true) {
        karmic_debts.push(karmic_debt(d, "lifePath"));
    }

    // 2. Birthday Number
    let birthday = reduce_number(day, true);
    if let Some(d) = find_karmic_debt(day, true) {
        karmic_debts.push(karmic_debt(d, "birthday"));
    }

    // 3. Name Based Numbers (Expression, Soul Urge, Personality, Maturity, ...)
    let mut expr_sum = 0u32;
    let mut soul_sum = 0u32;
    let mut pers_sum = 0u32;
    let mut counts = [0u32; 9];
    if let Some(name) = name_latin {
        for ch in name.chars() {
            let val = system.letter_value(ch);
            if val > 0 {
                expr_sum += val as u32;
                counts[val as usize - 1] += 1;
                if is_vowel(ch) {
                    soul_sum += val as u32;
                } else {
                    pers_sum += val as u32;
                }
            }
        }
    }
    let has_name = expr_sum > 0;
    let reduce_opt = |sum: u32| (sum > 0).then(|| reduce_number(sum, true));
    let expression = reduce_opt(expr_sum);
    let soul_urge = reduce_opt(soul_sum);
    let personality = reduce_opt(pers_sum);
    for (sum, source) in [
        (expr_sum, "expression"),
        (soul_sum, "soulUrge"),
        (pers_sum, "personality"),
    ] {
        if let Some(d) = find_karmic_debt(sum, true) {
            karmic_debts.push(karmic_debt(d, source));
        }
    }
    let maturity = expression.map(|e| reduce_number(life_path as u32 + e as u32, true));
    let name_insights = if has_name {
        name_latin.and_then(|n| name_insights(n, system, counts))
    } else {
        None
    };

    // 4. 4 Pinnacles & Challenges
    // The cycle calculations use the reduced birth-date components. This is
    // the convention used by the Pythagorean pinnacle/challenge references;
    // using raw 28/1990 values produces different, non-canonical challenges.
    let m_base = reduce_number(month, false) as u32;
    let d_base = reduce_number(day, false) as u32;
    let y_base = reduce_number(year, false) as u32;

    // Pinnacle 1: reduced month + reduced day
    let p1 = reduce_number(m_base + d_base, true);
    let c1 = reduce_number(m_base.abs_diff(d_base), false);

    // Pinnacle 2: reduced day + reduced year
    let p2 = reduce_number(d_base + y_base, true);
    let c2 = reduce_number(d_base.abs_diff(y_base), false);

    // Pinnacle 3: P1 + P2
    let p3 = reduce_number(p1 as u32 + p2 as u32, true);
    let c3 = reduce_number((c1 as i32 - c2 as i32).unsigned_abs(), false);

    // Pinnacle 4: reduced month + reduced year
    let p4 = reduce_number(m_base + y_base, true);
    let c4 = reduce_number(m_base.abs_diff(y_base), false);

    // Base pinnacle transition age: 36 - Life Path (single digit base)
    let lp_base = reduce_number(life_path as u32, false) as u32;
    let age1 = 36 - lp_base;
    let age2 = age1 + 9;
    let age3 = age2 + 9;

    let pinnacles = vec![
        PinnaclePeriod {
            stage: 1,
            pinnacle_number: p1,
            challenge_number: c1,
            start_age: 0,
            end_age: age1,
            description_ko: format!(
                "Pinnacle 1: Foundation Era (Pinnacle {}, Challenge {})",
                p1, c1
            ),
            description_en: format!(
                "Pinnacle 1: Foundation Era (Pinnacle {}, Challenge {})",
                p1, c1
            ),
        },
        PinnaclePeriod {
            stage: 2,
            pinnacle_number: p2,
            challenge_number: c2,
            start_age: age1,
            end_age: age2,
            description_ko: format!(
                "Pinnacle 2: Expansion Era (Pinnacle {}, Challenge {})",
                p2, c2
            ),
            description_en: format!(
                "Pinnacle 2: Expansion Era (Pinnacle {}, Challenge {})",
                p2, c2
            ),
        },
        PinnaclePeriod {
            stage: 3,
            pinnacle_number: p3,
            challenge_number: c3,
            start_age: age2,
            end_age: age3,
            description_ko: format!(
                "Pinnacle 3: Maturation & Achievement Era (Pinnacle {}, Challenge {})",
                p3, c3
            ),
            description_en: format!(
                "Pinnacle 3: Maturation & Achievement Era (Pinnacle {}, Challenge {})",
                p3, c3
            ),
        },
        PinnaclePeriod {
            stage: 4,
            pinnacle_number: p4,
            challenge_number: c4,
            start_age: age3,
            end_age: 100,
            description_ko: format!(
                "Pinnacle 4: Completion & Wisdom Era (Pinnacle {}, Challenge {})",
                p4, c4
            ),
            description_en: format!(
                "Pinnacle 4: Completion & Wisdom Era (Pinnacle {}, Challenge {})",
                p4, c4
            ),
        },
    ];

    // 5. Personal Year / Month / Day Number
    // Cycle numbers are reduced to a single digit (master numbers are not preserved).
    let personal_year = reduce_number(month + day + target_year, false);
    let personal_month = target
        .month
        .map(|m| reduce_number(personal_year as u32 + m, false));
    let personal_day = match (personal_month, target.month, target.day) {
        (Some(pm), Some(_), Some(d)) => Some(reduce_number(pm as u32 + d, false)),
        _ => None,
    };

    let opt = |v: Option<u8>| {
        v.map(|v| v.to_string())
            .unwrap_or_else(|| "N/A".to_string())
    };
    let debts_en = if karmic_debts.is_empty() {
        String::new()
    } else {
        format!(
            ", Karmic Debt: {}",
            karmic_debts
                .iter()
                .map(|k| format!("{}/{} ({})", k.debt_number, k.reduces_to, k.source))
                .collect::<Vec<_>>()
                .join(" ")
        )
    };
    let cycle_en = match (personal_month, personal_day) {
        (Some(m), Some(d)) => format!(", Personal Month: {m}, Personal Day: {d}"),
        (Some(m), None) => format!(", Personal Month: {m}"),
        _ => String::new(),
    };
    let summary_en = format!(
        "Life Path: {}, Expression: {}, Soul Urge: {}, Maturity: {}, Personal Year: {}{}{} ({} Numerology analysis complete)",
        life_path,
        opt(expression),
        opt(soul_urge),
        opt(maturity),
        personal_year,
        cycle_en,
        debts_en,
        system.label()
    );
    let system_ko = match system {
        NumerologySystem::Pythagorean => "피타고라스",
        NumerologySystem::Chaldean => "칼데안",
    };
    let cycle_ko = match (personal_month, personal_day) {
        (Some(m), Some(d)) => format!(", 개인 월수: {m}, 개인 일수: {d}"),
        (Some(m), None) => format!(", 개인 월수: {m}"),
        _ => String::new(),
    };
    let debts_ko = if karmic_debts.is_empty() {
        String::new()
    } else {
        format!(
            ", 카르마 부채: {}",
            karmic_debts
                .iter()
                .map(|k| format!("{}/{}({})", k.debt_number, k.reduces_to, k.source))
                .collect::<Vec<_>>()
                .join(" ")
        )
    };
    let summary_ko = format!(
        "생명수(Life Path): {}, 표현수: {}, 영혼수: {}, 성숙수: {}, 개인 연수: {}{}{} ({} 수비학 분석 완료)",
        life_path,
        opt(expression),
        opt(soul_urge),
        opt(maturity),
        personal_year,
        cycle_ko,
        debts_ko,
        system_ko
    );

    NumerologyResult {
        system,
        core: NumerologyCoreNumbers {
            life_path,
            expression,
            soul_urge,
            personality,
            birthday,
            maturity,
        },
        pinnacles,
        personal_year,
        personal_month,
        personal_day,
        is_master_life_path,
        karmic_debts,
        name_insights,
        summary_ko,
        summary_en,
    }
}

/// Hidden passion, balance, cornerstone/capstone and missing numbers of a name.
/// `counts[i]` is how often the number `i + 1` occurs among the letters.
fn name_insights(name: &str, system: NumerologySystem, counts: [u32; 9]) -> Option<NameInsights> {
    // Words are separated by whitespace; a word without any A-Z letter is skipped.
    let words: Vec<Vec<(char, u8)>> = name
        .split_whitespace()
        .map(|w| {
            w.chars()
                .filter_map(|c| {
                    let v = system.letter_value(c);
                    (v > 0).then(|| (c.to_ascii_uppercase(), v))
                })
                .collect::<Vec<_>>()
        })
        .filter(|w| !w.is_empty())
        .collect();
    let first = words.first()?;

    let max_count = counts.iter().copied().max().unwrap_or(0);
    let hidden_passions = if max_count == 0 {
        Vec::new()
    } else {
        (1..=9u8)
            .filter(|n| counts[*n as usize - 1] == max_count)
            .collect()
    };
    let missing_numbers = (1..=system.max_letter_value())
        .filter(|n| counts[*n as usize - 1] == 0)
        .collect();
    let initials: u32 = words.iter().map(|w| w[0].1 as u32).sum();
    let (cl, cv) = first[0];
    let (pl, pv) = first[first.len() - 1];

    Some(NameInsights {
        number_counts: counts,
        hidden_passions,
        balance: reduce_number(initials, false),
        cornerstone: NameLetter {
            letter: cl,
            value: cv,
        },
        capstone: NameLetter {
            letter: pl,
            value: pv,
        },
        missing_numbers,
    })
}

/// Weights of the pair scores in [`compare_numerology`].
const LIFE_PATH_WEIGHT: u32 = 50;
const EXPRESSION_WEIGHT: u32 = 30;
const SOUL_URGE_WEIGHT: u32 = 20;

/// Temperament group of a number: 0 = {1,5,7} independent seekers, 1 = {2,4,8} builders,
/// 2 = {3,6,9} expressive humanitarians. Master numbers use their base digit (11 -> 2, ...).
fn number_group(n: u8) -> u8 {
    match reduce_number(n as u32, false) {
        1 | 5 | 7 => 0,
        2 | 4 | 8 => 1,
        _ => 2,
    }
}

fn pair_score(a: u8, b: u8) -> NumberPairScore {
    let (ba, bb) = (
        reduce_number(a as u32, false),
        reduce_number(b as u32, false),
    );
    let (ga, gb) = (number_group(a), number_group(b));
    let (score, relation, en, ko) = if ba == bb {
        (
            85,
            "same-number",
            "Same vibration: deep mutual understanding, but shared blind spots too.",
            "같은 진동수: 서로를 깊이 이해하지만 약점도 같이 공유합니다.",
        )
    } else if ga == gb {
        (
            75,
            "same-group",
            "Same temperament group (1-5-7, 2-4-8 or 3-6-9): natural rapport and similar rhythm.",
            "같은 기질 그룹(1-5-7, 2-4-8, 3-6-9): 자연스러운 공감대와 비슷한 리듬.",
        )
    } else if (ga.min(gb), ga.max(gb)) == (0, 2) {
        (
            65,
            "complementary-groups",
            "Seekers (1-5-7) and expressives (3-6-9): lively, stimulating pairing that needs shared direction.",
            "탐구형(1-5-7)과 표현형(3-6-9): 활기와 자극이 있으나 공통의 방향이 필요합니다.",
        )
    } else if (ga.min(gb), ga.max(gb)) == (1, 2) {
        (
            60,
            "complementary-groups",
            "Builders (2-4-8) and expressives (3-6-9): warmth meets stability, with differing priorities.",
            "구축형(2-4-8)과 표현형(3-6-9): 따뜻함과 안정감이 만나지만 우선순위가 다릅니다.",
        )
    } else {
        (
            50,
            "contrasting-groups",
            "Seekers (1-5-7) and builders (2-4-8): freedom versus security; needs deliberate compromise.",
            "탐구형(1-5-7)과 구축형(2-4-8): 자유와 안정의 대비로 의식적인 타협이 필요합니다.",
        )
    };
    NumberPairScore {
        number_a: a,
        number_b: b,
        score,
        relation: relation.to_string(),
        description_en: en.to_string(),
        description_ko: ko.to_string(),
    }
}

/// Two-person numerology compatibility.
///
/// **This is a heuristic, not a canonical table.** Each pair of core numbers (Life Path,
/// Expression, Soul Urge) is scored from the temperament groups {1,5,7}, {2,4,8}, {3,6,9}
/// (master numbers use their base digit): identical number 85, same group 75, groups
/// {1,5,7}+{3,6,9} 65, {2,4,8}+{3,6,9} 60, {1,5,7}+{2,4,8} 50. The total is the weighted
/// average Life Path 50%, Expression 30%, Soul Urge 20%; when a name is missing for either
/// person, only the available pairs are used and the weights are renormalised.
pub fn compare_numerology(a: &NumerologyResult, b: &NumerologyResult) -> NumerologyCompatibility {
    let life_path = pair_score(a.core.life_path, b.core.life_path);
    let both = |x: Option<u8>, y: Option<u8>| match (x, y) {
        (Some(x), Some(y)) => Some(pair_score(x, y)),
        _ => None,
    };
    let expression = both(a.core.expression, b.core.expression);
    let soul_urge = both(a.core.soul_urge, b.core.soul_urge);

    let mut weighted = LIFE_PATH_WEIGHT * life_path.score as u32;
    let mut weights = LIFE_PATH_WEIGHT;
    if let Some(p) = &expression {
        weighted += EXPRESSION_WEIGHT * p.score as u32;
        weights += EXPRESSION_WEIGHT;
    }
    if let Some(p) = &soul_urge {
        weighted += SOUL_URGE_WEIGHT * p.score as u32;
        weights += SOUL_URGE_WEIGHT;
    }
    let total_score = (weighted as f64 * 10.0 / weights as f64).round() / 10.0;

    let methodology = "Heuristic: pair scores from temperament groups {1,5,7} {2,4,8} {3,6,9} \
(same number 85, same group 75, 1-5-7 + 3-6-9 65, 2-4-8 + 3-6-9 60, 1-5-7 + 2-4-8 50); \
weights Life Path 50 / Expression 30 / Soul Urge 20, renormalised when names are missing."
        .to_string();
    let used = 1 + expression.is_some() as u8 + soul_urge.is_some() as u8;
    let summary_en = format!(
        "Numerology compatibility {:.1}/100 (Life Path {} & {}{}{}; {} pair(s) compared, heuristic score)",
        total_score,
        a.core.life_path,
        b.core.life_path,
        expression
            .as_ref()
            .map(|p| format!(", Expression {} & {}", p.number_a, p.number_b))
            .unwrap_or_default(),
        soul_urge
            .as_ref()
            .map(|p| format!(", Soul Urge {} & {}", p.number_a, p.number_b))
            .unwrap_or_default(),
        used
    );
    let summary_ko = format!(
        "수비학 궁합 {:.1}/100 (생명수 {} & {}{}{}; {}개 쌍 비교, 휴리스틱 점수)",
        total_score,
        a.core.life_path,
        b.core.life_path,
        expression
            .as_ref()
            .map(|p| format!(", 표현수 {} & {}", p.number_a, p.number_b))
            .unwrap_or_default(),
        soul_urge
            .as_ref()
            .map(|p| format!(", 영혼수 {} & {}", p.number_a, p.number_b))
            .unwrap_or_default(),
        used
    );

    NumerologyCompatibility {
        system: a.system,
        life_path,
        expression,
        soul_urge,
        total_score,
        methodology,
        summary_en,
        summary_ko,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reduce_number() {
        assert_eq!(reduce_number(1990, true), 1);
        assert_eq!(reduce_number(11, true), 11);
        assert_eq!(reduce_number(22, true), 22);
        assert_eq!(reduce_number(33, true), 33);
        assert_eq!(reduce_number(11, false), 2);
    }

    #[test]
    fn test_calculate_numerology() {
        let res = calculate_numerology(1990, 5, 15, Some("John Doe"), 2026);
        assert!(res.core.life_path >= 1);
        assert!(res.core.expression.is_some());
        assert!(res.core.soul_urge.is_some());
        assert!(res.core.personality.is_some());
        assert_eq!(res.pinnacles.len(), 4);
        assert!(res.personal_year >= 1 && res.personal_year <= 9);

        let res_no_name = calculate_numerology(1990, 5, 15, None, 2026);
        assert!(res_no_name.core.expression.is_none());
        assert!(res_no_name.core.soul_urge.is_none());
        assert!(res_no_name.core.personality.is_none());
    }

    #[test]
    fn pinnacle_and_challenge_cycles_reduce_date_components_first() {
        let res = calculate_numerology(1990, 5, 28, None, 2026);

        // Month=5, day=28->1, year=1990->1.
        assert_eq!(res.pinnacles[0].pinnacle_number, 6);
        assert_eq!(res.pinnacles[0].challenge_number, 4);
        assert_eq!(res.pinnacles[1].pinnacle_number, 2);
        assert_eq!(res.pinnacles[1].challenge_number, 0);
        assert_eq!(res.pinnacles[2].challenge_number, 4);
        assert_eq!(res.pinnacles[3].pinnacle_number, 6);
        assert_eq!(res.pinnacles[3].challenge_number, 4);
    }

    fn ext(
        y: u32,
        m: u32,
        d: u32,
        name: Option<&str>,
        system: NumerologySystem,
    ) -> NumerologyResult {
        calculate_numerology_ext(y, m, d, name, TargetDate::year_only(2026), system)
    }

    fn debt_sources(res: &NumerologyResult, debt: u8) -> Vec<String> {
        res.karmic_debts
            .iter()
            .filter(|k| k.debt_number == debt)
            .map(|k| k.source.clone())
            .collect()
    }

    #[test]
    fn default_entry_point_is_pythagorean_and_matches_ext() {
        let a = calculate_numerology(1990, 5, 15, Some("John Doe"), 2026);
        let b = ext(1990, 5, 15, Some("John Doe"), NumerologySystem::Pythagorean);
        assert_eq!(a, b);
        assert_eq!(a.system, NumerologySystem::Pythagorean);
        assert_eq!(a.personal_month, None);
        assert_eq!(a.personal_day, None);
    }

    #[test]
    fn known_pythagorean_name_numbers_and_maturity() {
        // JOHN DOE: 1+6+8+5 + 4+6+5 = 35 -> 8; vowels O,O,E = 17 -> 8; consonants 18 -> 9.
        // Life path 1990-05-15: 5 + 6 + 1 = 12 -> 3; maturity 3 + 8 = 11 (master kept).
        let res = ext(1990, 5, 15, Some("John Doe"), NumerologySystem::Pythagorean);
        assert_eq!(res.core.life_path, 3);
        assert_eq!(res.core.expression, Some(8));
        assert_eq!(res.core.soul_urge, Some(8));
        assert_eq!(res.core.personality, Some(9));
        assert_eq!(res.core.maturity, Some(11));
        assert!(res.karmic_debts.is_empty());
    }

    #[test]
    fn karmic_debts_in_life_path_sum() {
        for (y, m, d, debt, reduces_to, lp) in [
            (1994, 4, 4, 13, 4, 4), // 4 + 4 + 5 = 13
            (2002, 5, 5, 14, 5, 5), // 5 + 5 + 4 = 14
            (2000, 7, 7, 16, 7, 7), // 7 + 7 + 2 = 16
            (1990, 9, 9, 19, 1, 1), // 9 + 9 + 1 = 19 -> 10 -> 1
        ] {
            let res = ext(y, m, d, None, NumerologySystem::Pythagorean);
            assert_eq!(res.core.life_path, lp, "{y}-{m}-{d}");
            assert_eq!(debt_sources(&res, debt), vec!["lifePath"], "{y}-{m}-{d}");
            let k = &res.karmic_debts[0];
            assert_eq!(k.reduces_to, reduces_to);
            assert!(!k.meaning_en.is_empty() && !k.meaning_ko.is_empty());
        }
        // No debt for 1990-05-15 (life path sum 12).
        assert!(ext(1990, 5, 15, None, NumerologySystem::Pythagorean)
            .karmic_debts
            .is_empty());
    }

    #[test]
    fn karmic_debt_from_birthday_day() {
        // Day 13 -> birthday number 4 with karmic debt 13; life path sum 1 + 4 + 1 = 6 has none.
        let res = ext(1990, 1, 13, None, NumerologySystem::Pythagorean);
        assert_eq!(res.core.birthday, 4);
        assert_eq!(debt_sources(&res, 13), vec!["birthday"]);
        assert_eq!(res.karmic_debts.len(), 1);
        for day in [14, 16, 19] {
            let r = ext(1990, 1, day, None, NumerologySystem::Pythagorean);
            assert_eq!(debt_sources(&r, day as u8), vec!["birthday"]);
        }
        // Day 28 -> 10 -> 1 is not a karmic debt.
        assert!(ext(1990, 1, 28, None, NumerologySystem::Pythagorean)
            .karmic_debts
            .iter()
            .all(|k| k.source != "birthday"));
    }

    #[test]
    fn karmic_debt_from_name_sums() {
        // DAVE (Pythagorean): 4+1+4+5 = 14 -> 5 (debt 14 in the expression);
        // vowels A,E = 6; consonants D,V = 8.
        let res = ext(1990, 5, 15, Some("Dave"), NumerologySystem::Pythagorean);
        assert_eq!(res.core.expression, Some(5));
        assert_eq!(res.core.soul_urge, Some(6));
        assert_eq!(res.core.personality, Some(8));
        assert_eq!(debt_sources(&res, 14), vec!["expression"]);
        assert_eq!(res.core.maturity, Some(8)); // 3 + 5
    }

    #[test]
    fn chaldean_letter_table() {
        let expected = [
            ('A', 1),
            ('B', 2),
            ('C', 3),
            ('D', 4),
            ('E', 5),
            ('F', 8),
            ('G', 3),
            ('H', 5),
            ('I', 1),
            ('J', 1),
            ('K', 2),
            ('L', 3),
            ('M', 4),
            ('N', 5),
            ('O', 7),
            ('P', 8),
            ('Q', 1),
            ('R', 2),
            ('S', 3),
            ('T', 4),
            ('U', 6),
            ('V', 6),
            ('W', 6),
            ('X', 5),
            ('Y', 1),
            ('Z', 7),
        ];
        for (ch, v) in expected {
            assert_eq!(chaldean_letter_value(ch), v, "{ch}");
            assert_eq!(chaldean_letter_value(ch.to_ascii_lowercase()), v);
        }
        assert_eq!(chaldean_letter_value('1'), 0);
        // Pythagorean table is untouched: F=6, V=4.
        assert_eq!(letter_value('F'), 6);
        assert_eq!(letter_value('V'), 4);
    }

    #[test]
    fn known_chaldean_values_and_karmic_debt_16() {
        // DAVE (Chaldean): 4+1+6+5 = 16 -> 7 (debt 16); vowels 1+5 = 6; consonants 4+6 = 10 -> 1.
        let res = ext(1990, 5, 15, Some("Dave"), NumerologySystem::Chaldean);
        assert_eq!(res.system, NumerologySystem::Chaldean);
        assert_eq!(res.core.expression, Some(7));
        assert_eq!(res.core.soul_urge, Some(6));
        assert_eq!(res.core.personality, Some(1));
        assert_eq!(debt_sources(&res, 16), vec!["expression"]);
        assert_eq!(res.core.maturity, Some(1)); // 3 + 7 = 10 -> 1
                                                // Date based numbers do not depend on the letter system.
        let pyth = ext(1990, 5, 15, Some("Dave"), NumerologySystem::Pythagorean);
        assert_eq!(res.core.life_path, pyth.core.life_path);
        assert_eq!(res.pinnacles, pyth.pinnacles);
        // JOHN DOE (Chaldean): 1+7+5+5 + 4+7+5 = 34 -> 7.
        let jd = ext(1990, 5, 15, Some("John Doe"), NumerologySystem::Chaldean);
        assert_eq!(jd.core.expression, Some(7));
        // Chaldean never assigns a 9, so only 1..=8 can be "missing".
        let ni = jd.name_insights.unwrap();
        assert_eq!(ni.missing_numbers, vec![2, 3, 6, 8]);
    }

    #[test]
    fn name_insights_for_john_doe() {
        let res = ext(
            1990,
            5,
            15,
            Some("john  DOE"),
            NumerologySystem::Pythagorean,
        );
        let ni = res.name_insights.expect("name insights");
        // Letter numbers: J1 O6 H8 N5 | D4 O6 E5
        assert_eq!(ni.number_counts, [1, 0, 0, 1, 2, 2, 0, 1, 0]);
        assert_eq!(ni.hidden_passions, vec![5, 6]); // 5 (N,E) and 6 (O,O) tie
        assert_eq!(ni.balance, 5); // J1 + D4
        assert_eq!(
            ni.cornerstone,
            NameLetter {
                letter: 'J',
                value: 1
            }
        );
        assert_eq!(
            ni.capstone,
            NameLetter {
                letter: 'N',
                value: 5
            }
        );
        assert_eq!(ni.missing_numbers, vec![2, 3, 7, 9]);
    }

    #[test]
    fn balance_uses_every_word_and_reduces_to_single_digit() {
        // M4 + A1 + R9 initials = 14 -> 5
        let res = ext(
            1990,
            5,
            15,
            Some("Mary Ann Reed"),
            NumerologySystem::Pythagorean,
        );
        assert_eq!(res.name_insights.unwrap().balance, 5);
    }

    #[test]
    fn non_latin_or_empty_name_has_no_name_numbers() {
        for name in [Some("김수진"), Some("  "), Some("123"), None] {
            let res = ext(1990, 5, 15, name, NumerologySystem::Pythagorean);
            assert!(res.core.expression.is_none());
            assert!(res.core.maturity.is_none());
            assert!(res.name_insights.is_none());
            assert!(res
                .karmic_debts
                .iter()
                .all(|k| k.source == "lifePath" || k.source == "birthday"));
        }
    }

    #[test]
    fn personal_month_and_day_for_target_date() {
        // 1990-05-28, target 2026-11-20:
        // year 5 + 28 + 2026 = 2059 -> 16 -> 7; month 7 + 11 = 18 -> 9; day 9 + 20 = 29 -> 11 -> 2.
        let res = calculate_numerology_ext(
            1990,
            5,
            28,
            None,
            TargetDate::full(2026, 11, 20),
            NumerologySystem::Pythagorean,
        );
        assert_eq!(res.personal_year, 7);
        assert_eq!(res.personal_month, Some(9));
        assert_eq!(res.personal_day, Some(2));
        assert!(res.summary_en.contains("Personal Month: 9"));
        assert!(res.summary_ko.contains("개인 월수: 9"));

        // A target year alone gives no month/day cycles.
        let y = calculate_numerology(1990, 5, 28, None, 2026);
        assert_eq!(y.personal_year, 7);
        assert_eq!(y.personal_month, None);
        assert_eq!(y.personal_day, None);
    }

    #[test]
    fn compatibility_pairs_and_weights() {
        let dave = ext(1990, 5, 15, Some("Dave"), NumerologySystem::Pythagorean);
        let john = ext(1990, 5, 15, Some("John Doe"), NumerologySystem::Pythagorean);
        let c = compare_numerology(&dave, &john);
        // Life path 3 & 3 -> 85; expression 5 (seekers) & 8 (builders) -> 50;
        // soul urge 6 (expressive) & 8 (builders) -> 60.
        assert_eq!(c.life_path.score, 85);
        assert_eq!(c.life_path.relation, "same-number");
        assert_eq!(c.expression.as_ref().unwrap().score, 50);
        assert_eq!(c.soul_urge.as_ref().unwrap().score, 60);
        // (50*85 + 30*50 + 20*60) / 100 = 69.5
        assert_eq!(c.total_score, 69.5);
        assert!(c.summary_ko.contains("69.5"));
        assert!(c.methodology.contains("Heuristic"));
    }

    #[test]
    fn compatibility_without_names_uses_life_path_only() {
        let a = ext(1990, 5, 15, None, NumerologySystem::Pythagorean); // life path 3
        let b = ext(1994, 4, 4, None, NumerologySystem::Pythagorean); // life path 4
        let c = compare_numerology(&a, &b);
        assert!(c.expression.is_none() && c.soul_urge.is_none());
        assert_eq!(c.life_path.score, 60); // {3,6,9} + {2,4,8}
        assert_eq!(c.total_score, 60.0);
    }

    #[test]
    fn compatibility_groups_and_master_numbers() {
        assert_eq!(pair_score(1, 7).score, 75); // same group {1,5,7}
        assert_eq!(pair_score(1, 3).score, 65); // {1,5,7} + {3,6,9}
        assert_eq!(pair_score(2, 9).score, 60); // {2,4,8} + {3,6,9}
        assert_eq!(pair_score(5, 4).score, 50); // {1,5,7} + {2,4,8}
        assert_eq!(pair_score(11, 2).score, 85); // 11 reduces to 2
        assert_eq!(pair_score(22, 8).score, 75); // 22 -> 4, same group as 8
        assert_eq!(pair_score(3, 7), {
            let mut p = pair_score(7, 3);
            std::mem::swap(&mut p.number_a, &mut p.number_b);
            p
        });
    }

    #[test]
    fn system_parses_and_displays() {
        assert_eq!("Chaldean".parse(), Ok(NumerologySystem::Chaldean));
        assert_eq!(" pythagorean ".parse(), Ok(NumerologySystem::Pythagorean));
        assert!("vedic".parse::<NumerologySystem>().is_err());
        assert_eq!(NumerologySystem::default().to_string(), "pythagorean");
    }
}
