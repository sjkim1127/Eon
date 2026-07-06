use chrono::{DateTime, Utc};
use eon_astro::AstroEngine;
use eon_saju::core::ganzi::GanZi;

/// 기문둔갑 음양둔 (Yin/Yang Ju) 구분
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JuType {
    Yang, // 양둔 (동지~하지 전)
    Yin,  // 음둔 (하지~동지 전)
}

/// 24절기와 국수 (Ju Number) 매핑 정보
/// 상원, 중원, 하원의 국수 배열 (크기 3)
#[derive(Debug, Clone, Copy)]
pub struct TermJu(pub u8, pub u8, pub u8);

/// 24절기에 대한 기문둔갑 국수 테이블 (순서는 입춘부터 시작: 0=입춘, 1=우수, ...)
pub const JU_TABLE: [(JuType, TermJu); 24] = [
    // --- 양둔 (Yang Ju) ---
    // 0: 입춘 (Lichun)
    (JuType::Yang, TermJu(8, 5, 2)),
    // 1: 우수 (Yushui)
    (JuType::Yang, TermJu(9, 6, 3)),
    // 2: 경칩 (Jingzhe)
    (JuType::Yang, TermJu(1, 7, 4)),
    // 3: 춘분 (Chunfen)
    (JuType::Yang, TermJu(3, 9, 6)),
    // 4: 청명 (Qingming)
    (JuType::Yang, TermJu(4, 1, 7)),
    // 5: 곡우 (Guyu)
    (JuType::Yang, TermJu(5, 2, 8)),
    // 6: 입하 (Lixia)
    (JuType::Yang, TermJu(4, 1, 7)),
    // 7: 소만 (Xiaoman)
    (JuType::Yang, TermJu(5, 2, 8)),
    // 8: 망종 (Mangzhong)
    (JuType::Yang, TermJu(6, 3, 9)),
    // 9: 하지 (Xiazhi) - 여기서부터 음둔
    (JuType::Yin, TermJu(9, 3, 6)),
    // 10: 소서 (Xiaoshu)
    (JuType::Yin, TermJu(8, 2, 5)),
    // 11: 대서 (Dashu)
    (JuType::Yin, TermJu(7, 1, 4)),
    // 12: 입추 (Liqiu)
    (JuType::Yin, TermJu(2, 5, 8)),
    // 13: 처서 (Chushu)
    (JuType::Yin, TermJu(1, 4, 7)),
    // 14: 백로 (Bailu)
    (JuType::Yin, TermJu(9, 3, 6)),
    // 15: 추분 (Qiufen)
    (JuType::Yin, TermJu(7, 1, 4)),
    // 16: 한로 (Hanlu)
    (JuType::Yin, TermJu(6, 9, 3)),
    // 17: 상강 (Shuangjiang)
    (JuType::Yin, TermJu(5, 8, 2)),
    // 18: 입동 (Lidong)
    (JuType::Yin, TermJu(6, 9, 3)),
    // 19: 소설 (Xiaoxue)
    (JuType::Yin, TermJu(5, 8, 2)),
    // 20: 대설 (Daxue)
    (JuType::Yin, TermJu(4, 7, 1)),
    // 21: 동지 (Dongzhi) - 여기서부터 다시 양둔
    (JuType::Yang, TermJu(1, 7, 4)),
    // 22: 소한 (Xiaohan)
    (JuType::Yang, TermJu(2, 8, 5)),
    // 23: 대한 (Dahan)
    (JuType::Yang, TermJu(3, 9, 6)),
];

/// 차보법(Chai Bu)으로 해당 일진(Day Pillar)의 원(Yuan)을 구한다.
/// 반환값: 0 (상원), 1 (중원), 2 (하원)
pub fn get_yuan_index(day_pillar: GanZi) -> usize {
    // 기문둔갑 차보법에서는 甲, 己 일의 지지에 따라 원(상,중,하)이 바뀐다.
    // 甲, 己 일까지 며칠 뒤/앞인지 계산하여 부두(符头)를 찾는다.
    // 甲(0), 己(5). 현재 천간이 무엇이든 간에, 최근의 甲 또는 己일이 부두가 된다.
    let stem_idx = day_pillar.stem.index();

    // 부두까지의 거리는 현재 천간이 갑(0)~무(4)이면 stem_idx만큼 빼면 甲일.
    // 기(5)~계(9)이면 stem_idx - 5만큼 빼면 己일.
    let offset = stem_idx % 5;

    // 부두의 지지 인덱스를 구한다.
    let branch_idx = (day_pillar.branch.index() as i32 - offset as i32).rem_euclid(12) as u8;

    // 부두의 지지에 따라 원이 결정됨
    // 子(0), 午(6), 卯(3), 酉(9) -> 상원 (0)
    // 寅(2), 申(8), 巳(5), 亥(11) -> 중원 (1)
    // 辰(4), 戌(10), 丑(1), 未(7) -> 하원 (2)
    match branch_idx {
        0 | 3 | 6 | 9 => 0,  // 상원
        2 | 5 | 8 | 11 => 1, // 중원
        1 | 4 | 7 | 10 => 2, // 하원
        _ => unreachable!(),
    }
}

/// UTC 시각을 받아 24절기 인덱스를 반환 (0: 입춘, 1: 우수, ... 23: 대한)
pub fn get_24_solar_term_index(dt: DateTime<Utc>) -> Result<u8, eon_astro::AstroError> {
    let engine = AstroEngine::new();
    let sun_long = engine.get_sun_longitude(dt)?;
    let adjusted = (sun_long - 315.0 + 360.0) % 360.0;
    Ok((adjusted / 15.0).floor() as u8 % 24)
}

/// 차보법 기준 국수와 음양둔 산출
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
