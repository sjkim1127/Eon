use chrono::{DateTime, Utc};
use eon_astro::AstroEngine;
use eon_saju::core::ganzi::GanZi;

/// QiMen DunJia Yin/Yang Ju classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JuType {
    Yang, // Yang Ju (Winter Solstice to Summer Solstice)
    Yin,  // Yin Ju (Summer Solstice to Winter Solstice)
}

/// 24 solar terms to Ju number mapping.
/// Array of Ju numbers for Upper, Middle, and Lower Yuan (size 3).
#[derive(Debug, Clone, Copy)]
pub struct TermJu(pub u8, pub u8, pub u8);

/// QiMen DunJia Ju table for 24 solar terms (starting from Lichun: 0=Lichun, 1=Yushui, ...).
pub const JU_TABLE: [(JuType, TermJu); 24] = [
    // --- Yang Ju ---
    // 0: Lichun
    (JuType::Yang, TermJu(8, 5, 2)),
    // 1: Yushui
    (JuType::Yang, TermJu(9, 6, 3)),
    // 2: Jingzhe
    (JuType::Yang, TermJu(1, 7, 4)),
    // 3: Chunfen
    (JuType::Yang, TermJu(3, 9, 6)),
    // 4: Qingming
    (JuType::Yang, TermJu(4, 1, 7)),
    // 5: Guyu
    (JuType::Yang, TermJu(5, 2, 8)),
    // 6: Lixia
    (JuType::Yang, TermJu(4, 1, 7)),
    // 7: Xiaoman
    (JuType::Yang, TermJu(5, 2, 8)),
    // 8: Mangzhong
    (JuType::Yang, TermJu(6, 3, 9)),
    // 9: Xiazhi - Yin Ju begins here
    (JuType::Yin, TermJu(9, 3, 6)),
    // 10: Xiaoshu
    (JuType::Yin, TermJu(8, 2, 5)),
    // 11: Dashu
    (JuType::Yin, TermJu(7, 1, 4)),
    // 12: Liqiu
    (JuType::Yin, TermJu(2, 5, 8)),
    // 13: Chushu
    (JuType::Yin, TermJu(1, 4, 7)),
    // 14: Bailu
    (JuType::Yin, TermJu(9, 3, 6)),
    // 15: Qiufen
    (JuType::Yin, TermJu(7, 1, 4)),
    // 16: Hanlu
    (JuType::Yin, TermJu(6, 9, 3)),
    // 17: Shuangjiang
    (JuType::Yin, TermJu(5, 8, 2)),
    // 18: Lidong
    (JuType::Yin, TermJu(6, 9, 3)),
    // 19: Xiaoxue
    (JuType::Yin, TermJu(5, 8, 2)),
    // 20: Daxue
    (JuType::Yin, TermJu(4, 7, 1)),
    // 21: Dongzhi - Yang Ju resumes here
    (JuType::Yang, TermJu(1, 7, 4)),
    // 22: Xiaohan
    (JuType::Yang, TermJu(2, 8, 5)),
    // 23: Dahan
    (JuType::Yang, TermJu(3, 9, 6)),
];

/// Compute the Yuan (Upper=0, Middle=1, Lower=2) for a given Day Pillar using Chai Bu method.
pub fn get_yuan_index(day_pillar: GanZi) -> usize {
    // In Chai Bu method, the Yuan changes based on the Earthly Branch of the Jia or Ji day.
    // Find the Fu Tou (leader stem: Jia or Ji).
    let stem_idx = day_pillar.stem.index();

    // Fu Tou offset calculation
    let offset = stem_idx % 5;

    // Fu Tou Earthly Branch index
    let branch_idx = (day_pillar.branch.index() as i32 - offset as i32).rem_euclid(12) as u8;

    // Yuan determined by branch index:
    // Zi(0), Wu(6), Mao(3), You(9) -> Upper Yuan (0)
    // Yin(2), Shen(8), Si(5), Hai(11) -> Middle Yuan (1)
    // Chen(4), Xu(10), Chou(1), Wei(7) -> Lower Yuan (2)
    match branch_idx {
        0 | 3 | 6 | 9 => 0,  // Upper Yuan
        2 | 5 | 8 | 11 => 1, // Middle Yuan
        1 | 4 | 7 | 10 => 2, // Lower Yuan
        _ => unreachable!(),
    }
}

/// Compute 24 solar term index (0: Lichun, 1: Yushui, ... 23: Dahan) from UTC time.
pub fn get_24_solar_term_index(dt: DateTime<Utc>) -> Result<u8, eon_astro::AstroError> {
    let engine = AstroEngine::new();
    let sun_long = engine.get_sun_longitude(dt)?;
    let adjusted = (sun_long - 315.0 + 360.0) % 360.0;
    Ok((adjusted / 15.0).floor() as u8 % 24)
}

/// Calculate Ju number and Yin/Yang classification using Chai Bu method.
pub fn calculate_ju(
    dt: DateTime<Utc>,
    day_pillar: GanZi,
) -> Result<(bool, u8), eon_astro::AstroError> {
    let term_idx = get_24_solar_term_index(dt)?;
    let yuan_idx = get_yuan_index(day_pillar);

    let (ju_type, term_ju) = JU_TABLE[term_idx as usize];
    let is_yin_ju = ju_type == JuType::Yin;

    let ju_number = match yuan_idx {
        0 => term_ju.0,
        1 => term_ju.1,
        2 => term_ju.2,
        _ => unreachable!(),
    };

    Ok((is_yin_ju, ju_number))
}
