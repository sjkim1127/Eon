use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NumerologyCoreNumbers {
    /// 생애여정수 (Life Path Number)
    pub life_path: u8,
    /// 표현수 (Expression / Destiny Number)
    pub expression: u8,
    /// 영혼수 (Soul Urge / Heart's Desire Number)
    pub soul_urge: u8,
    /// 인격수 (Personality Number)
    pub personality: u8,
    /// 생일수 (Birthday Number)
    pub birthday: u8,
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
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NumerologyResult {
    pub core: NumerologyCoreNumbers,
    pub pinnacles: Vec<PinnaclePeriod>,
    pub personal_year: u8,
    pub is_master_life_path: bool,
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

pub fn is_vowel(ch: char) -> bool {
    matches!(ch.to_ascii_uppercase(), 'A' | 'E' | 'I' | 'O' | 'U')
}

/// Calculates Full Pythagorean Numerology Analysis
pub fn calculate_numerology(
    year: u32,
    month: u32,
    day: u32,
    name_latin: Option<&str>,
    target_year: u32,
) -> NumerologyResult {
    // 1. Life Path Number
    let m_red = reduce_number(month, true) as u32;
    let d_red = reduce_number(day, true) as u32;
    let y_red = reduce_number(year, true) as u32;
    let life_path = reduce_number(m_red + d_red + y_red, true);
    let is_master_life_path = matches!(life_path, 11 | 22 | 33);

    // 2. Birthday Number
    let birthday = reduce_number(day, true);

    // 3. Name Based Numbers (Expression, Soul Urge, Personality)
    let (expression, soul_urge, personality) = if let Some(name) = name_latin {
        let mut expr_sum = 0u32;
        let mut soul_sum = 0u32;
        let mut pers_sum = 0u32;

        for ch in name.chars() {
            let val = letter_value(ch);
            if val > 0 {
                expr_sum += val as u32;
                if is_vowel(ch) {
                    soul_sum += val as u32;
                } else {
                    pers_sum += val as u32;
                }
            }
        }
        (
            reduce_number(expr_sum, true),
            reduce_number(soul_sum, true),
            reduce_number(pers_sum, true),
        )
    } else {
        // Fallback proxy from birth values if latin name not provided
        (
            reduce_number(d_red + y_red, true),
            reduce_number(m_red, true),
            reduce_number(y_red, true),
        )
    };

    // 4. 4 Pinnacles & Challenges
    // Pinnacle 1: Month + Day
    let p1 = reduce_number(month + day, true);
    let c1 = reduce_number((month as i32 - day as i32).unsigned_abs(), false);

    // Pinnacle 2: Day + Year
    let p2 = reduce_number(day + year, true);
    let c2 = reduce_number((day as i32 - year as i32).unsigned_abs(), false);

    // Pinnacle 3: P1 + P2
    let p3 = reduce_number(p1 as u32 + p2 as u32, true);
    let c3 = reduce_number((c1 as i32 - c2 as i32).unsigned_abs(), false);

    // Pinnacle 4: Month + Year
    let p4 = reduce_number(month + year, true);
    let c4 = reduce_number((month as i32 - year as i32).unsigned_abs(), false);

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
            description_ko: format!("제1정점: 기반 형성기 (정점수 {}, 과제수 {})", p1, c1),
        },
        PinnaclePeriod {
            stage: 2,
            pinnacle_number: p2,
            challenge_number: c2,
            start_age: age1,
            end_age: age2,
            description_ko: format!("제2정점: 개인적 도약기 (정점수 {}, 과제수 {})", p2, c2),
        },
        PinnaclePeriod {
            stage: 3,
            pinnacle_number: p3,
            challenge_number: c3,
            start_age: age2,
            end_age: age3,
            description_ko: format!(
                "제3정점: 성숙과 사회적 결실기 (정점수 {}, 과제수 {})",
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
                "제4정점: 완성과 지혜의 통달기 (정점수 {}, 과제수 {})",
                p4, c4
            ),
        },
    ];

    // 5. Personal Year Number
    let personal_year = reduce_number(month + day + target_year, false);

    let summary_ko = format!(
        "생애여정수: {}, 표현수: {}, 영혼수: {}, 개인 연운수: {} (피타고라스 수비 진동 분석 완료)",
        life_path, expression, soul_urge, personal_year
    );

    NumerologyResult {
        core: NumerologyCoreNumbers {
            life_path,
            expression,
            soul_urge,
            personality,
            birthday,
        },
        pinnacles,
        personal_year,
        is_master_life_path,
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
        assert_eq!(res.pinnacles.len(), 4);
        assert!(res.personal_year >= 1 && res.personal_year <= 9);
    }
}
