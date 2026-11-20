//! Classical lookup tables of 奇門遁甲: palace branches and elements, the auspice of doors, stars
//! and deities, and the stem-over-stem formations (格局).
//!
//! Sources are the common classical texts (《煙波釣叟歌》, 《奇門遁甲統宗》) as summarised in modern
//! manuals. Schools disagree on the details, so every choice below is documented in
//! `docs/CALCULATION.md` section 7a and the scores are heuristics, not verdicts.

use crate::core::{Deity, Door, Palace, Star};
use eon_saju::core::branch::EarthlyBranch;
use eon_saju::core::element::Element;
use eon_saju::core::stem::HeavenlyStem;
use serde::{Deserialize, Serialize};

/// Earthly branches that sit in a palace (後天八卦 / Later Heaven arrangement).
pub fn palace_branches(palace: Palace) -> &'static [EarthlyBranch] {
    use EarthlyBranch::*;
    match palace {
        Palace::Kan1 => &[Zi],
        Palace::Gen8 => &[Chou, Yin],
        Palace::Zhen3 => &[Mao],
        Palace::Xun4 => &[Chen, Si],
        Palace::Li9 => &[Wu],
        Palace::Kun2 => &[Wei, Shen],
        Palace::Dui7 => &[You],
        Palace::Qian6 => &[Xu, Hai],
        Palace::Center5 => &[],
    }
}

/// The palace a branch belongs to (never the centre).
pub fn palace_of_branch(branch: EarthlyBranch) -> Palace {
    use EarthlyBranch::*;
    match branch {
        Zi => Palace::Kan1,
        Chou | Yin => Palace::Gen8,
        Mao => Palace::Zhen3,
        Chen | Si => Palace::Xun4,
        Wu => Palace::Li9,
        Wei | Shen => Palace::Kun2,
        You => Palace::Dui7,
        Xu | Hai => Palace::Qian6,
    }
}

pub fn palace_element(palace: Palace) -> Element {
    match palace {
        Palace::Kan1 => Element::Water,
        Palace::Kun2 | Palace::Center5 | Palace::Gen8 => Element::Earth,
        Palace::Zhen3 | Palace::Xun4 => Element::Wood,
        Palace::Qian6 | Palace::Dui7 => Element::Metal,
        Palace::Li9 => Element::Fire,
    }
}

pub fn door_element(door: Door) -> Element {
    match door {
        Door::Xiu => Element::Water,
        Door::Sheng | Door::Si => Element::Earth,
        Door::Shang | Door::Du => Element::Wood,
        Door::Jing => Element::Fire,
        Door::Jing2 | Door::Kai => Element::Metal,
    }
}

/// Auspice of a door: the three good doors 開休生 (+2), the three bad doors 傷驚死 (-2, death -3);
/// 景 and 杜 are "middling" (0).
pub fn door_score(door: Door) -> i32 {
    match door {
        Door::Kai | Door::Xiu | Door::Sheng => 2,
        Door::Jing | Door::Du => 0,
        Door::Shang | Door::Jing2 => -2,
        Door::Si => -3,
    }
}

/// Auspice of a star: 輔 心 (+2), 禽 任 沖 (+1), 英 柱 (-1), 蓬 芮 (-2).
pub fn star_score(star: Star) -> i32 {
    match star {
        Star::Fu | Star::Xin => 2,
        Star::Qin | Star::Ren | Star::Chong => 1,
        Star::Ying | Star::Zhu => -1,
        Star::Peng | Star::Rui => -2,
    }
}

/// Auspice of a deity: 值符 (+2); 九天 九地 太陰 六合 (+1); 螣蛇 玄武 (-1); 白虎 (-2).
pub fn deity_score(deity: Deity) -> i32 {
    match deity {
        Deity::ZhiFu => 2,
        Deity::JiuTian | Deity::JiuDi | Deity::TaiYin | Deity::LiuHe => 1,
        Deity::TengShe | Deity::XuanWu => -1,
        Deity::BaiHu => -2,
    }
}

/// Palace where a heaven-plate stem sits "in its own temple" (三奇升殿): 乙 in 震, 丙 in 離, 丁 in 兌.
pub fn temple_palace(stem: HeavenlyStem) -> Option<Palace> {
    match stem {
        HeavenlyStem::Yi => Some(Palace::Zhen3),
        HeavenlyStem::Bing => Some(Palace::Li9),
        HeavenlyStem::Ding => Some(Palace::Dui7),
        _ => None,
    }
}

/// Palace where a heaven-plate stem is "punished" (六儀擊刑): 戊 in 震, 己 in 坤, 庚 in 艮,
/// 辛 in 離, 壬 and 癸 in 巽.
pub fn punishment_palace(stem: HeavenlyStem) -> Option<Palace> {
    match stem {
        HeavenlyStem::Wu => Some(Palace::Zhen3),
        HeavenlyStem::Ji => Some(Palace::Kun2),
        HeavenlyStem::Geng => Some(Palace::Gen8),
        HeavenlyStem::Xin => Some(Palace::Li9),
        HeavenlyStem::Ren | HeavenlyStem::Gui => Some(Palace::Xun4),
        _ => None,
    }
}

/// Palace where a three-wonder stem enters its tomb (三奇入墓): 乙 in 坤, 丙 in 乾, 丁 in 艮.
pub fn tomb_palace(stem: HeavenlyStem) -> Option<Palace> {
    match stem {
        HeavenlyStem::Yi => Some(Palace::Kun2),
        HeavenlyStem::Bing => Some(Palace::Qian6),
        HeavenlyStem::Ding => Some(Palace::Gen8),
        _ => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatternKind {
    Auspicious,
    Inauspicious,
}

/// A named formation. `id` is stable for programmatic use.
#[derive(Debug, Clone, Copy)]
pub struct PatternInfo {
    pub id: &'static str,
    pub hanja: &'static str,
    pub english: &'static str,
    pub korean: &'static str,
    pub kind: PatternKind,
}

const fn pat(
    id: &'static str,
    hanja: &'static str,
    english: &'static str,
    korean: &'static str,
    kind: PatternKind,
) -> PatternInfo {
    PatternInfo {
        id,
        hanja,
        english,
        korean,
        kind,
    }
}

use PatternKind::{Auspicious as Good, Inauspicious as Bad};

/// Heaven stem over earth stem in the same palace.
pub const STEM_PAIRS: [(HeavenlyStem, HeavenlyStem, PatternInfo); 11] = [
    (
        HeavenlyStem::Wu,
        HeavenlyStem::Bing,
        pat(
            "qing_long_fan_shou",
            "青龍返首",
            "Azure Dragon Returns",
            "청룡반수",
            Good,
        ),
    ),
    (
        HeavenlyStem::Bing,
        HeavenlyStem::Wu,
        pat(
            "fei_niao_die_xue",
            "飛鳥跌穴",
            "Flying Bird Falls Into the Cave",
            "비조질혈",
            Good,
        ),
    ),
    (
        HeavenlyStem::Geng,
        HeavenlyStem::Bing,
        pat(
            "tai_bai_ru_ying",
            "太白入熒",
            "Venus Enters Mars",
            "태백입형",
            Bad,
        ),
    ),
    (
        HeavenlyStem::Bing,
        HeavenlyStem::Geng,
        pat(
            "ying_ru_tai_bai",
            "熒入太白",
            "Mars Enters Venus",
            "형입태백",
            Bad,
        ),
    ),
    (
        HeavenlyStem::Yi,
        HeavenlyStem::Xin,
        pat(
            "qing_long_tao_zou",
            "青龍逃走",
            "Azure Dragon Flees",
            "청룡도주",
            Bad,
        ),
    ),
    (
        HeavenlyStem::Xin,
        HeavenlyStem::Yi,
        pat(
            "bai_hu_chang_kuang",
            "白虎猖狂",
            "White Tiger Runs Wild",
            "백호창광",
            Bad,
        ),
    ),
    (
        HeavenlyStem::Gui,
        HeavenlyStem::Ding,
        pat(
            "teng_she_yao_jiao",
            "螣蛇夭矯",
            "Snake Writhes",
            "등사요교",
            Bad,
        ),
    ),
    (
        HeavenlyStem::Ding,
        HeavenlyStem::Gui,
        pat(
            "zhu_que_tou_jiang",
            "朱雀投江",
            "Vermilion Bird Dives Into the River",
            "주작투강",
            Bad,
        ),
    ),
    (
        HeavenlyStem::Geng,
        HeavenlyStem::Gui,
        pat("da_ge", "大格", "Great Obstruction", "대격", Bad),
    ),
    (
        HeavenlyStem::Geng,
        HeavenlyStem::Ren,
        pat("xiao_ge", "小格", "Small Obstruction", "소격", Bad),
    ),
    (
        HeavenlyStem::Geng,
        HeavenlyStem::Ji,
        pat("xing_ge", "刑格", "Punishing Obstruction", "형격", Bad),
    ),
];

pub const SAN_QI_ENTER_ENVOY: PatternInfo = pat(
    "san_qi_de_shi",
    "三奇得使",
    "Three Wonders Gain the Envoy",
    "삼기득사",
    Good,
);
pub const YU_NV_SHOU_MEN: PatternInfo = pat(
    "yu_nv_shou_men",
    "玉女守門",
    "Jade Maiden Guards the Door",
    "옥녀수문",
    Good,
);
pub const SAN_QI_SHENG_DIAN: PatternInfo = pat(
    "san_qi_sheng_dian",
    "三奇升殿",
    "Wonder Ascends Its Hall",
    "삼기승전",
    Good,
);
pub const SAN_QI_RU_MU: PatternInfo = pat(
    "san_qi_ru_mu",
    "三奇入墓",
    "Wonder Enters the Tomb",
    "삼기입묘",
    Bad,
);
pub const LIU_YI_JI_XING: PatternInfo = pat(
    "liu_yi_ji_xing",
    "六儀擊刑",
    "Six Yi Struck by Punishment",
    "육의격형",
    Bad,
);
pub const TIAN_DUN: PatternInfo = pat("tian_dun", "天遁", "Heaven Escape", "천둔", Good);
pub const DI_DUN: PatternInfo = pat("di_dun", "地遁", "Earth Escape", "지둔", Good);
pub const REN_DUN: PatternInfo = pat("ren_dun", "人遁", "Human Escape", "인둔", Good);
pub const SHEN_DUN: PatternInfo = pat("shen_dun", "神遁", "Spirit Escape", "신둔", Good);
pub const GUI_DUN: PatternInfo = pat("gui_dun", "鬼遁", "Ghost Escape", "귀둔", Good);
pub const NIAN_GE: PatternInfo = pat("nian_ge", "年格", "Year Obstruction", "년격", Bad);
pub const YUE_GE: PatternInfo = pat("yue_ge", "月格", "Month Obstruction", "월격", Bad);
pub const RI_GE: PatternInfo = pat("ri_ge", "日格", "Day Obstruction", "일격", Bad);
pub const SHI_GE: PatternInfo = pat("shi_ge", "時格", "Hour Obstruction", "시격", Bad);
pub const FU_YIN_STAR: PatternInfo = pat(
    "fu_yin_star",
    "星伏吟",
    "Stars Hidden (Fu Yin)",
    "성복음",
    Bad,
);
pub const FAN_YIN_STAR: PatternInfo = pat(
    "fan_yin_star",
    "星反吟",
    "Stars Reversed (Fan Yin)",
    "성반음",
    Bad,
);
pub const FU_YIN_DOOR: PatternInfo = pat(
    "fu_yin_door",
    "門伏吟",
    "Doors Hidden (Fu Yin)",
    "문복음",
    Bad,
);
pub const FAN_YIN_DOOR: PatternInfo = pat(
    "fan_yin_door",
    "門反吟",
    "Doors Reversed (Fan Yin)",
    "문반음",
    Bad,
);
pub const WU_BU_YU_SHI: PatternInfo = pat(
    "wu_bu_yu_shi",
    "五不遇時",
    "Hour of the Five No-Meetings",
    "오불우시",
    Bad,
);
