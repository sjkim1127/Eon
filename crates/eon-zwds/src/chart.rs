//! 자미두수 성반(星盤) 구축 모듈
//!
//! 출생 정보(`BirthInfo`)로부터 음력 변환, 명궁/신궁 계산, 12궁 세부 정보 조립, 대한 목록 산출, 명주/신주 지정을 거쳐 최종 `ZwdsChart`를 완성합니다.

use eon_core::birth::BirthInfo;
use eon_core::birth::Gender;
use eon_saju::core::stem::HeavenlyStem;
use eon_saju::core::branch::EarthlyBranch;

use crate::types::{ZwdsChart, PalaceData, StarInPalace, ZwdsStar, LunarBirthInfo};
use crate::error::ZwdsError;
use crate::calendar::solar_to_lunar_birth;
use crate::palace::{
    get_soul_and_body_index, get_palace_stem, get_five_elements, get_palace_name, zwds_idx_to_std_idx
};
use crate::stars::place_all_stars;
use crate::transformations::get_sihua_stars;
use crate::decadal::calculate_daxian;

/// 출생 정보(`BirthInfo`)를 받아 자미두수 성반(`ZwdsChart`)을 빌드합니다.
pub fn build_chart(birth: &BirthInfo) -> Result<ZwdsChart, ZwdsError> {
    // 1. 양력 생년월일시 -> 음력 변환
    let lunar = solar_to_lunar_birth(
        birth.year,
        birth.month,
        birth.day,
        birth.hour,
    )?;

    build_chart_from_lunar(&lunar, birth)
}

/// 변환 완료된 음력 정보로부터 자미두수 성반을 빌드합니다.
pub fn build_chart_from_lunar(lunar: &LunarBirthInfo, birth: &BirthInfo) -> Result<ZwdsChart, ZwdsError> {
    let year_stem = HeavenlyStem::from_index(lunar.year_stem_idx as i32);

    // 2. 명궁(soul) 및 신궁(body) 인덱스 계산
    let (soul_idx, body_idx) = get_soul_and_body_index(lunar.month, lunar.time_branch_idx);

    // 3. 명궁 천간 구하기 및 오행국 결정
    let soul_stem = get_palace_stem(year_stem, soul_idx);
    let five_elements = get_five_elements(soul_stem, soul_idx);

    // 4. 모든 별 배치 계산 (HashMap<ZwdsStar, PalaceIndex>)
    let star_positions = place_all_stars(lunar, five_elements.value(), soul_idx, body_idx);

    // 5. 출생년 사화 대상 별 조회
    let sihua_stars = get_sihua_stars(year_stem);

    // 6. 대한 목록 생성
    let daxian_list = calculate_daxian(
        birth.gender.unwrap_or(Gender::Male),
        year_stem,
        soul_idx,
        five_elements,
    );

    // 7. 12궁 데이터 조립
    // PalaceIndex는 0=寅宫 ~ 11=丑宫
    let mut palaces_vec = Vec::new();
    for p_idx in 0..12 {
        let p_name = get_palace_name(soul_idx, p_idx);
        let p_stem = get_palace_stem(year_stem, p_idx);
        let branch_std = zwds_idx_to_std_idx(p_idx);
        let p_branch = EarthlyBranch::from_index(branch_std as i32);

        // 이 궁에 배치된 별들 및 사화 태그 매핑
        let mut stars_in_palace = Vec::new();
        for (&star, &pos) in star_positions.iter() {
            if pos == p_idx {
                // 사화 확인
                let si_hua = if star == sihua_stars[0] {
                    Some(crate::types::SiHuaType::HuaLu)
                } else if star == sihua_stars[1] {
                    Some(crate::types::SiHuaType::HuaQuan)
                } else if star == sihua_stars[2] {
                    Some(crate::types::SiHuaType::HuaKe)
                } else if star == sihua_stars[3] {
                    Some(crate::types::SiHuaType::HuaJi)
                } else {
                    None
                };

                stars_in_palace.push(StarInPalace { star, si_hua });
            }
        }

        // 별 정렬 (주성이 먼저 나오도록)
        stars_in_palace.sort_by_key(|s| !s.star.is_main_star());

        // 대한 나이 범위 매핑
        let daxian_range = daxian_list.iter()
            .find(|d| d.palace_idx == p_idx)
            .map(|d| (d.age_start as u8, d.age_end as u8));

        palaces_vec.push(PalaceData {
            index: p_idx,
            name: p_name,
            heavenly_stem: p_stem.hanja().to_string(),
            earthly_branch: p_branch.hanja().to_string(),
            stars: stars_in_palace,
            daxian_range,
            is_current_liu_nian: false, // 기본값 false, 서비스 레이어에서 연도 매핑 시 업데이트
        });
    }

    let palaces: [PalaceData; 12] = palaces_vec.try_into()
        .map_err(|_| ZwdsError::Internal("12궁 데이터 생성 실패".to_string()))?;

    // 8. 명주(命主) / 신주(身主) 결정
    // 명궁의 표준 지지에 따라 명주 결정
    let soul_branch_std_idx = zwds_idx_to_std_idx(soul_idx);
    let soul_master = match soul_branch_std_idx {
        0 => ZwdsStar::TanLang,  // 子
        1 => ZwdsStar::JuMen,    // 丑
        2 => ZwdsStar::LuCun,    // 寅
        3 => ZwdsStar::WenQu,    // 卯
        4 => ZwdsStar::LianZhen, // 辰
        5 => ZwdsStar::WuQu,     // 巳
        6 => ZwdsStar::PoJun,    // 午
        7 => ZwdsStar::WuQu,     // 未
        8 => ZwdsStar::LianZhen, // 申
        9 => ZwdsStar::WenQu,    // 酉
        10 => ZwdsStar::LuCun,   // 戌
        11 => ZwdsStar::JuMen,   // 亥
        _ => unreachable!(),
    };

    // 태어난 년의 표준 지지에 따라 신주 결정
    let body_master = match lunar.year_branch_idx {
        0 => ZwdsStar::HuoXing,    // 子
        1 => ZwdsStar::TianXiang,  // 丑
        2 => ZwdsStar::TianLiang,  // 寅
        3 => ZwdsStar::TianTong,   // 卯
        4 => ZwdsStar::WenChang,   // 辰
        5 => ZwdsStar::TianJi,     // 巳
        6 => ZwdsStar::HuoXing,    // 午
        7 => ZwdsStar::TianXiang,  // 未
        8 => ZwdsStar::TianLiang,  // 申
        9 => ZwdsStar::TianTong,   // 酉
        10 => ZwdsStar::WenChang,  // 戌
        11 => ZwdsStar::TianJi,    // 亥
        _ => unreachable!(),
    };

    Ok(ZwdsChart {
        palaces,
        soul_idx,
        body_idx,
        soul_master,
        body_master,
        five_elements,
        daxian: daxian_list,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use eon_core::birth::Gender;

    #[test]
    fn test_build_chart() {
        let birth = BirthInfo::solar(2004, 11, 27, 22, 0)
            .with_gender(Gender::Male)
            .with_location(eon_core::Location::seoul())
            .with_korea_timezone();

        let chart = build_chart(&birth).unwrap();
        // 2004-11-27 22:00 -> 음력 10월 16일 亥시생.
        // 명궁은 子(10), 신궁은 戌(8).
        assert_eq!(chart.soul_idx, 10);
        assert_eq!(chart.body_idx, 8);

        // 명주: 명궁 지지가 子(0) -> 貪狼
        assert_eq!(chart.soul_master, ZwdsStar::TanLang);

        // 2004년 甲申년생 -> 년지 申(8) -> 신주: 天梁
        assert_eq!(chart.body_master, ZwdsStar::TianLiang);
    }
}
