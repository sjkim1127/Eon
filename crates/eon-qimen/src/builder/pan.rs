use crate::core::elements::{Deity, Door, Palace, Star};
use crate::core::types::{PalaceState, QimenPan};
use eon_saju::core::ganzi::GanZi;
use eon_saju::core::stem::HeavenlyStem;

const EARTH_PLATE_ORDER: [HeavenlyStem; 9] = [
    HeavenlyStem::Wu,   // Wu
    HeavenlyStem::Ji,   // Ji
    HeavenlyStem::Geng, // Geng
    HeavenlyStem::Xin,  // Xin
    HeavenlyStem::Ren,  // Ren
    HeavenlyStem::Gui,  // Gui
    HeavenlyStem::Ding, // Ding
    HeavenlyStem::Bing, // Bing
    HeavenlyStem::Yi,   // Yi
];

const LUOSHU_SEQUENCE: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

/// Construct Earth Plate 9-palace stem distribution given Yin/Yang Ju and Ju number (1~9).
pub fn build_earth_plate(is_yin_ju: bool, ju_number: u8) -> [Option<HeavenlyStem>; 9] {
    // Indices 0~8 map to Palaces 1~9.
    let mut earth_stems = [None; 9];

    // Starting palace number for Wu (1~9)
    let start_palace = ju_number;

    for (i, &stem) in EARTH_PLATE_ORDER.iter().enumerate() {
        // Calculate palace number
        let current_palace_num = if !is_yin_ju {
            // Yang Ju: forward (+)
            let mut num = start_palace as i32 + i as i32;
            while num > 9 {
                num -= 9;
            }
            num as u8
        } else {
            // Yin Ju: backward (-)
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
        0 => HeavenlyStem::Wu,   // Jia-Zi Xun
        10 => HeavenlyStem::Ji,  // Jia-Xu Xun
        8 => HeavenlyStem::Geng, // Jia-Shen Xun
        6 => HeavenlyStem::Xin,  // Jia-Wu Xun
        4 => HeavenlyStem::Ren,  // Jia-Chen Xun
        2 => HeavenlyStem::Gui,  // Jia-Yin Xun
        _ => unreachable!(),
    }
}

pub fn get_base_star(palace: u8) -> Star {
    match palace {
        1 => Star::Peng,
        2 => Star::Rui,
        3 => Star::Chong,
        4 => Star::Fu,
        5 => Star::Qin, // Center Palace 5 (Tian Qin, entrusted to Kun 2)
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
        5 => Door::Si, // Center Palace entrusted (Kun 2)
        6 => Door::Kai,
        7 => Door::Jing2,
        8 => Door::Sheng,
        9 => Door::Jing,
        _ => unreachable!(),
    }
}

/// Luo Shu Bagua ring order (1-8-3-4-9-2-7-6)
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

    // 1. Earth Plate distribution
    let earth_stems = build_earth_plate(is_yin_ju, ju_number);
    for i in 0..9 {
        palaces[i].earth_stem = earth_stems[i];
    }

    // 2. Locate Value Chief (Zhi Fu) and Value Envoy (Zhi Shi)
    let xun_shou = get_xun_shou(hour_pillar);

    // Locate palace containing Xun Shou stem on the Earth Plate
    let mut xun_shou_palace = 1;
    for i in 0..9 {
        if palaces[i].earth_stem == Some(xun_shou) {
            xun_shou_palace = (i + 1) as u8;
            break;
        }
    }

    // If in Center Palace 5, entrust to Kun Palace 2
    let base_palace_for_chief = if xun_shou_palace == 5 {
        2
    } else {
        xun_shou_palace
    };

    // When Center Palace entrusts to Kun 2, Zhi Fu still takes Center Palace Tian Qin;
    // Zhi Shi takes the Si Men of Kun 2.
    let value_chief_star = if xun_shou_palace == 5 {
        Star::Qin
    } else {
        get_base_star(base_palace_for_chief)
    };
    let value_envoy_door = get_base_door(xun_shou_palace);

    // 3. Heaven Plate and Star Plate distribution
    // Zhi Fu (Star) moves to the palace where the Hour Stem resides on the Earth Plate.
    let hour_stem = hour_pillar.stem;
    let target_palace_for_chief = if hour_stem == HeavenlyStem::Jia {
        // If hour stem is Jia, it hides under Xun Shou.
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

    // Outer Ring Index
    // RING_PALACES = [1, 8, 3, 4, 9, 2, 7, 6]
    let base_ring_idx = RING_PALACES
        .iter()
        .position(|&x| x == base_palace_for_chief)
        .unwrap_or(0);
    let target_ring_idx = RING_PALACES
        .iter()
        .position(|&x| x == target_palace_for_chief)
        .unwrap_or(0);

    // Star rotation distance
    let star_rotation = (target_ring_idx as i32 - base_ring_idx as i32).rem_euclid(8) as usize;

    for i in 0..8 {
        let original_palace = RING_PALACES[i];
        let original_star = get_base_star(original_palace);

        // Stems on Earth Plate rotate to Heaven Plate
        let original_earth_stem = palaces[(original_palace - 1) as usize].earth_stem;

        let new_ring_idx = (i + star_rotation) % 8;
        let new_palace = RING_PALACES[new_ring_idx];

        palaces[(new_palace - 1) as usize].star = Some(original_star);
        palaces[(new_palace - 1) as usize].heaven_stem = original_earth_stem;
    }

    // Center Palace (5) Heaven Plate star/stem remains with Tian Qin
    palaces[4].star = Some(Star::Qin);
    palaces[4].heaven_stem = palaces[4].earth_stem;

    // 4. Human Plate (Eight Doors) distribution
    // Zhi Shi (Door) steps according to Hour Branch.
    let xun_branch_idx = match get_xun_shou(hour_pillar) {
        HeavenlyStem::Wu => 0,   // Zi
        HeavenlyStem::Ji => 10,  // Xu
        HeavenlyStem::Geng => 8, // Shen
        HeavenlyStem::Xin => 6,  // Wu
        HeavenlyStem::Ren => 4,  // Chen
        HeavenlyStem::Gui => 2,  // Yin
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

    // 5. Deity Plate (Eight Deities) distribution
    // Zhi Fu deity aligns with the palace containing Heaven Plate Zhi Fu (Star).
    // Yang Ju rotates clockwise; Yin Ju rotates counterclockwise.
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
