use crate::core::elements::{Deity, Door, Palace, Star};
use crate::core::types::{PalaceState, QimenPan};
use eon_saju::core::ganzi::GanZi;
use eon_saju::core::stem::HeavenlyStem;

const EARTH_PLATE_ORDER: [HeavenlyStem; 9] = [
    HeavenlyStem::Wu,   // 무
    HeavenlyStem::Ji,   // 기
    HeavenlyStem::Geng, // 경
    HeavenlyStem::Xin,  // 신
    HeavenlyStem::Ren,  // 임
    HeavenlyStem::Gui,  // 계
    HeavenlyStem::Ding, // 정
    HeavenlyStem::Bing, // 병
    HeavenlyStem::Yi,   // 을
];

const LUOSHU_SEQUENCE: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

/// 양둔/음둔과 국수(1~9)를 받아 지반(Earth Plate) 9궁의 간(Stem) 배치를 반환한다.
pub fn build_earth_plate(is_yin_ju: bool, ju_number: u8) -> [Option<HeavenlyStem>; 9] {
    // 인덱스는 0~8이며, 이는 1궁~9궁에 매핑된다.
    let mut earth_stems = [None; 9];

    // 무(戊)가 배치될 시작 궁 번호 (1~9)
    let start_palace = ju_number;

    for (i, &stem) in EARTH_PLATE_ORDER.iter().enumerate() {
        // 궁의 번호 계산
        let current_palace_num = if !is_yin_ju {
            // 양둔: 순행 (+)
            let mut num = start_palace as i32 + i as i32;
            while num > 9 {
                num -= 9;
            }
            num as u8
        } else {
            // 음둔: 역행 (-)
            let mut num = start_palace as i32 - i as i32;
            while num < 1 {
                num += 9;
            }
            num as u8
        };

        earth_stems[(current_palace_num - 1) as usize] = Some(stem);
    }

    earth_stems
}

pub fn get_xun_shou(pillar: GanZi) -> HeavenlyStem {
    let diff = (pillar.branch.index() as i32 - pillar.stem.index() as i32).rem_euclid(12);
    match diff {
        0 => HeavenlyStem::Wu,   // 갑자순
        10 => HeavenlyStem::Ji,  // 갑술순
        8 => HeavenlyStem::Geng, // 갑신순
        6 => HeavenlyStem::Xin,  // 갑오순
        4 => HeavenlyStem::Ren,  // 갑진순
        2 => HeavenlyStem::Gui,  // 갑인순
        _ => unreachable!(),
    }
}

pub fn get_base_star(palace: u8) -> Star {
    match palace {
        1 => Star::Peng,
        2 => Star::Rui,
        3 => Star::Chong,
        4 => Star::Fu,
        5 => Star::Rui, // 중궁 기궁 (곤2궁)
        6 => Star::Xin,
        7 => Star::Zhu,
        8 => Star::Ren,
        9 => Star::Ying,
        _ => unreachable!(),
    }
}

pub fn get_base_door(palace: u8) -> Door {
    match palace {
        1 => Door::Xiu,
        2 => Door::Si,
        3 => Door::Shang,
        4 => Door::Du,
        5 => Door::Si, // 중궁 기궁 (곤2궁)
        6 => Door::Kai,
        7 => Door::Jing2,
        8 => Door::Sheng,
        9 => Door::Jing,
        _ => unreachable!(),
    }
}

/// 낙서구궁 외곽(Bagua ring)의 순환 순서 (1-8-3-4-9-2-7-6)
pub const RING_PALACES: [u8; 8] = [1, 8, 3, 4, 9, 2, 7, 6];

pub fn build_qimen_pan(
    time: chrono::DateTime<chrono::Utc>,
    year_pillar: GanZi,
    month_pillar: GanZi,
    day_pillar: GanZi,
    hour_pillar: GanZi,
    is_yin_ju: bool,
    ju_number: u8,
) -> QimenPan {
    let mut palaces: Vec<PalaceState> = (1..=9)
        .map(|num| PalaceState {
            palace: Palace::from_u8(num).unwrap(),
            earth_stem: None,
            heaven_stem: None,
            door: None,
            star: None,
            deity: None,
        })
        .collect();

    // 1. 지반(Earth Plate) 포국
    let earth_stems = build_earth_plate(is_yin_ju, ju_number);
    for i in 0..9 {
        palaces[i].earth_stem = earth_stems[i];
    }

    // 2. 직부(Value Chief), 직사(Value Envoy) 찾기
    let xun_shou = get_xun_shou(hour_pillar);

    // 순수(Xun Shou) 천간이 지반에서 어느 궁에 있는지 찾는다.
    let mut xun_shou_palace = 1;
    for i in 0..9 {
        if palaces[i].earth_stem == Some(xun_shou) {
            xun_shou_palace = (i + 1) as u8;
            break;
        }
    }

    // 중궁(5궁)에 있다면 기궁 법칙에 따라 2궁으로 간주.
    let base_palace_for_chief = if xun_shou_palace == 5 {
        2
    } else {
        xun_shou_palace
    };

    let value_chief_star = get_base_star(base_palace_for_chief);
    let value_envoy_door = get_base_door(base_palace_for_chief);

    // 3. 천반(Heaven Plate) 및 구성(Star Plate) 포국
    // 직부(Star)는 시간(Hour Stem)이 지반에 있는 궁으로 이동한다. (단, 시간이 甲이면 순수와 같으므로 직부 궁에 그대로)
    let hour_stem = hour_pillar.stem;
    let target_palace_for_chief = if hour_stem == HeavenlyStem::Jia {
        // 시간이 甲이면 은복되므로 순수와 일치.
        base_palace_for_chief
    } else {
        let mut p = 1;
        for i in 0..9 {
            if palaces[i].earth_stem == Some(hour_stem) {
                p = (i + 1) as u8;
                break;
            }
        }
        if p == 5 {
            2
        } else {
            p
        }
    };

    // 구궁의 외곽(Ring) 인덱스
    // RING_PALACES = [1, 8, 3, 4, 9, 2, 7, 6]
    let base_ring_idx = RING_PALACES
        .iter()
        .position(|&x| x == base_palace_for_chief)
        .unwrap_or(0);
    let target_ring_idx = RING_PALACES
        .iter()
        .position(|&x| x == target_palace_for_chief)
        .unwrap_or(0);

    // 외곽 회전 (얼마나 이동했는지)
    let star_rotation = (target_ring_idx as i32 - base_ring_idx as i32).rem_euclid(8) as usize;

    for i in 0..8 {
        let original_palace = RING_PALACES[i];
        let original_star = get_base_star(original_palace);

        // 지반에 있던 천간이 천반으로 함께 이동
        let original_earth_stem = if original_palace == 2 {
            // 곤2궁은 중궁(5궁)의 지반간도 함께 가져올 수 있으나, 일반적으로는 본궁의 천간을 가져감.
            // 기문둔갑에서 중궁 기궁 시 천반간 처리는 문파마다 다르나,
            // 여기서는 본래 2궁의 천간과 5궁의 천간 중 어떤 것을 취할지에 대해 단순화: 원래 궁의 천간 이동
            palaces[(original_palace - 1) as usize].earth_stem
        } else {
            palaces[(original_palace - 1) as usize].earth_stem
        };

        let new_ring_idx = (i + star_rotation) % 8;
        let new_palace = RING_PALACES[new_ring_idx];

        palaces[(new_palace - 1) as usize].star = Some(original_star);
        palaces[(new_palace - 1) as usize].heaven_stem = original_earth_stem;
    }

    // 중궁(5궁)의 천반성/간은 기본적으로 비워두거나(None) 지반과 동일하게 처리.
    palaces[4].star = Some(Star::Qin);
    palaces[4].heaven_stem = palaces[4].earth_stem;

    // 4. 팔문(Human Plate) 포국
    // 직사(Door)는 시지(Hour Branch)에 맞게 이동.
    // 직사의 원래 위치는 xun_shou_palace (또는 2궁).
    // 여기서 순수(Xun Shou) 지지의 궁에서 출발해, 시지까지 양/음둔 순역 무관하게 양둔은 순행(1->2->3...), 음둔은 역행(9->8->7...)한다는 설도 있으나,
    // 대다수 문파: 직사는 순수 지지(예: 甲子순이면 子) 궁에서 시작, 시간에 도달할 때까지 궁수를 양둔은 +1, 음둔은 -1 씩 추산하여 도달한 궁이 타겟 궁.
    // 1~9궁 순서로 이동 (1,2,3...9) (중궁은 제외하고 1~9로 세거나, 혹은 포함하여 세고 5면 2로).
    // 좀 더 표준적인 방법:
    let xun_branch_idx = match get_xun_shou(hour_pillar) {
        HeavenlyStem::Wu => 0,   // 자
        HeavenlyStem::Ji => 10,  // 술
        HeavenlyStem::Geng => 8, // 신
        HeavenlyStem::Xin => 6,  // 오
        HeavenlyStem::Ren => 4,  // 진
        HeavenlyStem::Gui => 2,  // 인
        _ => 0,
    };
    let hour_branch_idx = hour_pillar.branch.index();
    let steps = (hour_branch_idx as i32 - xun_branch_idx).rem_euclid(12);

    let mut door_target_palace = base_palace_for_chief as i32;
    if !is_yin_ju {
        door_target_palace += steps;
        while door_target_palace > 9 {
            door_target_palace -= 9;
        }
    } else {
        door_target_palace -= steps;
        while door_target_palace < 1 {
            door_target_palace += 9;
        }
    }
    if door_target_palace == 5 {
        door_target_palace = 2;
    }

    let base_door_ring_idx = RING_PALACES
        .iter()
        .position(|&x| x == base_palace_for_chief)
        .unwrap_or(0);
    let target_door_ring_idx = RING_PALACES
        .iter()
        .position(|&x| x == door_target_palace as u8)
        .unwrap_or(0);
    let door_rotation =
        (target_door_ring_idx as i32 - base_door_ring_idx as i32).rem_euclid(8) as usize;

    for i in 0..8 {
        let original_palace = RING_PALACES[i];
        let original_door = get_base_door(original_palace);
        let new_ring_idx = (i + door_rotation) % 8;
        let new_palace = RING_PALACES[new_ring_idx];
        palaces[(new_palace - 1) as usize].door = Some(original_door);
    }

    // 5. 팔신(Deity Plate) 포국
    // 소위 천반 팔신 (가장 흔함): 직부가 천반 직부(Star)가 위치한 궁에 오고,
    // 양둔이면 시계방향(1-8-3-4-9-2-7-6), 음둔이면 반시계방향으로 8신이 배치.
    let deity_order_yang = [
        Deity::ZhiFu,
        Deity::TengShe,
        Deity::TaiYin,
        Deity::LiuHe,
        Deity::BaiHu,
        Deity::XuanWu,
        Deity::JiuDi,
        Deity::JiuTian,
    ];
    let deity_order_yin = [
        Deity::ZhiFu,
        Deity::JiuTian,
        Deity::JiuDi,
        Deity::XuanWu,
        Deity::BaiHu,
        Deity::LiuHe,
        Deity::TaiYin,
        Deity::TengShe,
    ];
    let deities = if !is_yin_ju {
        &deity_order_yang
    } else {
        &deity_order_yin
    };

    for i in 0..8 {
        let new_ring_idx = (target_ring_idx + i) % 8;
        let new_palace = RING_PALACES[new_ring_idx];
        palaces[(new_palace - 1) as usize].deity = Some(deities[i]);
    }

    QimenPan {
        time,
        year_pillar,
        month_pillar,
        day_pillar,
        hour_pillar,
        is_yin_ju,
        ju_number,
        palaces,
        value_chief_star: Some(value_chief_star),
        value_envoy_door: Some(value_envoy_door),
    }
}
