//! 자미두수 잡성(雜星) 배치 모듈
//!
//! 록존, 경양, 타라, 천마, 화성, 영성, 지공, 지겁 및 기타 약 30개의 중소형 잡성들의 위치를 계산합니다.

use crate::palace::fix_index;
use crate::types::{PalaceIndex, ZwdsStar};
use eon_saju::core::branch::EarthlyBranch;
use eon_saju::core::stem::HeavenlyStem;

/// 록존(禄存), 경양(擎羊), 타라(陀羅)의 ZWDS 지지 인덱스 계산
pub fn place_lucun_qingyang_tuoluo(
    year_stem: HeavenlyStem,
) -> (PalaceIndex, PalaceIndex, PalaceIndex) {
    let lucun = match year_stem {
        HeavenlyStem::Jia => 0,  // 寅
        HeavenlyStem::Yi => 1,   // 卯
        HeavenlyStem::Bing => 3, // 巳
        HeavenlyStem::Ding => 4, // 午
        HeavenlyStem::Wu => 3,   // 巳
        HeavenlyStem::Ji => 4,   // 午
        HeavenlyStem::Geng => 6, // 申
        HeavenlyStem::Xin => 7,  // 酉
        HeavenlyStem::Ren => 9,  // 亥
        HeavenlyStem::Gui => 10, // 子
    };
    let qingyang = fix_index(lucun as i32 + 1);
    let tuoluo = fix_index(lucun as i32 - 1);
    (lucun, qingyang, tuoluo)
}

/// 천마(天馬)의 ZWDS 지지 인덱스 계산
pub fn place_tianma(year_branch_std_idx: usize) -> PalaceIndex {
    let branch = EarthlyBranch::from_index(year_branch_std_idx as i32);
    match branch {
        EarthlyBranch::Yin | EarthlyBranch::Wu | EarthlyBranch::Xu => 6, // 申
        EarthlyBranch::Shen | EarthlyBranch::Zi | EarthlyBranch::Chen => 0, // 寅
        EarthlyBranch::Si | EarthlyBranch::You | EarthlyBranch::Chou => 9, // 亥
        EarthlyBranch::Hai | EarthlyBranch::Mao | EarthlyBranch::Wei => 3, // 巳
    }
}

/// 화성(火星)과 영성(鈴星)의 ZWDS 지지 인덱스 계산
pub fn place_huoxing_lingxing(
    year_branch_std_idx: usize,
    time_branch_std_idx: usize,
) -> (PalaceIndex, PalaceIndex) {
    let branch = EarthlyBranch::from_index(year_branch_std_idx as i32);
    let t = time_branch_std_idx as i32;

    let (huo_start, ling_start) = match branch {
        EarthlyBranch::Yin | EarthlyBranch::Wu | EarthlyBranch::Xu => (11, 1), // 화=丑(11), 영=卯(1)
        EarthlyBranch::Shen | EarthlyBranch::Zi | EarthlyBranch::Chen => (0, 8), // 화=寅(0), 영=戌(8)
        EarthlyBranch::Si | EarthlyBranch::You | EarthlyBranch::Chou => (1, 8), // 화=卯(1), 영=戌(8)
        EarthlyBranch::Hai | EarthlyBranch::Mao | EarthlyBranch::Wei => (7, 8), // 화=酉(7), 영=戌(8)
    };

    let huoxing = fix_index(huo_start + t);
    let lingxing = fix_index(ling_start + t);

    (huoxing, lingxing)
}

/// 지겁(地劫)과 지공(地空)의 ZWDS 지지 인덱스 계산
pub fn place_dijie_dikong(time_branch_std_idx: usize) -> (PalaceIndex, PalaceIndex) {
    let t = time_branch_std_idx as i32;
    let dijie = fix_index(9 + t); // 亥궁(9)에서 순행
    let dikong = fix_index(9 - t); // 亥궁(9)에서 역행
    (dijie, dikong)
}

/// 음력 월 기준 잡성 배치
pub fn place_month_stars(lunar_month: u32) -> Vec<(ZwdsStar, PalaceIndex)> {
    let m = lunar_month as i32;
    let mut stars = Vec::new();

    // 천형(天刑): 酉궁(7)에서 순행
    stars.push((ZwdsStar::TianXing, fix_index(7 + m - 1)));
    // 천요(天姚): 丑궁(11)에서 순행
    stars.push((ZwdsStar::TianYao, fix_index(11 + m - 1)));

    // 해신(解神): 1,2월->申(6), 3,4월->酉(7)...
    let jieshen = fix_index(((lunar_month - 1) / 2) as i32 + 6);
    stars.push((ZwdsStar::JieShen, jieshen));

    // 음살(陰煞): 1월->寅(0), 2월->子(10)...
    let yinsha = fix_index(-2 * (((lunar_month - 1) % 6) as i32));
    stars.push((ZwdsStar::YinSha, yinsha));

    // 천월(天月): 룩업 테이블
    let tianyue_table = [8, 3, 2, 0, 5, 7, 6, 5, 9, 4, 0, 7];
    let tianyue = tianyue_table[(lunar_month as usize - 1) % 12];
    stars.push((ZwdsStar::TianYueStar, tianyue));

    // 천무(天巫): 1,5,9월->巳(3)...
    let tianwu = match (lunar_month - 1) % 4 {
        0 => 3, // 巳
        1 => 6, // 申
        2 => 0, // 寅
        3 => 9, // 亥
        _ => unreachable!(),
    };
    stars.push((ZwdsStar::TianWu, tianwu));

    stars
}

/// 출생시 기준 잡성 배치
pub fn place_hour_stars(time_branch_std_idx: usize) -> Vec<(ZwdsStar, PalaceIndex)> {
    let t = time_branch_std_idx as i32;
    let mut stars = Vec::new();

    // 대보(台輔): 午궁(4)에서 순행
    stars.push((ZwdsStar::TaiFu, fix_index(4 + t)));
    // 봉고(封誥): 寅궁(0)에서 순행
    stars.push((ZwdsStar::FengGao, fix_index(t)));

    stars
}

/// 음력 일 기준 잡성 배치
pub fn place_day_stars(
    lunar_day: u32,
    zuofu_idx: PalaceIndex,
    youbi_idx: PalaceIndex,
    wenchang_idx: PalaceIndex,
    wenqu_idx: PalaceIndex,
) -> Vec<(ZwdsStar, PalaceIndex)> {
    let d = lunar_day as i32;
    let mut stars = Vec::new();

    // 삼태(三台): 좌보에서 순행
    stars.push((ZwdsStar::SanTai, fix_index(zuofu_idx as i32 + d - 1)));
    // 팔좌(八座): 우필에서 역행
    stars.push((ZwdsStar::BaZuo, fix_index(youbi_idx as i32 - d + 1)));
    // 은광(恩光): 문창에서 순행
    stars.push((ZwdsStar::EnGuang, fix_index(wenchang_idx as i32 + d - 2)));
    // 천귀(天貴): 문곡에서 순행
    stars.push((ZwdsStar::TianGui, fix_index(wenqu_idx as i32 + d - 2)));

    stars
}

/// 출생년 지지 기준 잡성 배치
pub fn place_year_branch_stars(year_branch_std_idx: usize) -> Vec<(ZwdsStar, PalaceIndex)> {
    // ZWDS 년지 지지 인덱스 (0=寅)
    let y_zwds = ((year_branch_std_idx + 10) % 12) as i32;
    let mut stars = Vec::new();

    // 홍란(紅鸞): 卯궁(1)에서 역행
    let hongluan = fix_index(1 - y_zwds);
    stars.push((ZwdsStar::HongLuan, hongluan));
    // 천희(天喜): 홍란 대궁
    stars.push((ZwdsStar::TianXi, fix_index(hongluan as i32 + 6)));

    // 화개(華蓋) & 함지(咸池) & 겁살(劫殺)
    let branch = EarthlyBranch::from_index(year_branch_std_idx as i32);
    let (huagai, xianchi, jiesha) = match branch {
        EarthlyBranch::Yin | EarthlyBranch::Wu | EarthlyBranch::Xu => (8, 1, 9), // 戌, 卯, 亥
        EarthlyBranch::Shen | EarthlyBranch::Zi | EarthlyBranch::Chen => (2, 7, 3), // 辰, 酉, 巳
        EarthlyBranch::Si | EarthlyBranch::You | EarthlyBranch::Chou => (11, 4, 0), // 丑, 午, 寅
        EarthlyBranch::Hai | EarthlyBranch::Mao | EarthlyBranch::Wei => (5, 10, 6), // 未, 子, 申
    };
    stars.push((ZwdsStar::HuaGai, huagai));
    stars.push((ZwdsStar::XianChi, xianchi));
    stars.push((ZwdsStar::JieSha, jiesha));

    // 고신(孤辰) & 과숙(寡宿)
    let (guchen, guasu) = match branch {
        EarthlyBranch::Yin | EarthlyBranch::Mao | EarthlyBranch::Chen => (3, 11), // 巳, 丑
        EarthlyBranch::Si | EarthlyBranch::Wu | EarthlyBranch::Wei => (6, 2),     // 申, 辰
        EarthlyBranch::Shen | EarthlyBranch::You | EarthlyBranch::Xu => (9, 5),   // 亥, 未
        EarthlyBranch::Hai | EarthlyBranch::Zi | EarthlyBranch::Chou => (0, 8),   // 寅, 戌
    };
    stars.push((ZwdsStar::GuChen, guchen));
    stars.push((ZwdsStar::GuaSu, guasu));

    // 용지(龍池): 辰궁(2)에서 순행
    stars.push((ZwdsStar::LongChi, fix_index(2 + y_zwds)));
    // 봉각(鳳閣): 戌궁(8)에서 역행
    stars.push((ZwdsStar::FengGe, fix_index(8 - y_zwds)));

    // 천곡(天哭): 午궁(4)에서 역행
    stars.push((ZwdsStar::TianKu, fix_index(4 - y_zwds)));
    // 천허(天虛): 午궁(4)에서 순행
    stars.push((ZwdsStar::TianXu, fix_index(4 + y_zwds)));

    // 천공(天空): 卯궁(1)에서 순행 (년지 ZWDS 인덱스 + 1)
    stars.push((ZwdsStar::TianKong, fix_index(y_zwds + 1)));

    stars
}

/// 출생년 천간 기준 잡성 배치
pub fn place_year_stem_stars(
    year_stem: HeavenlyStem,
    year_branch_std_idx: usize,
    soul_idx: PalaceIndex,
    body_idx: PalaceIndex,
) -> Vec<(ZwdsStar, PalaceIndex)> {
    let y_zwds = ((year_branch_std_idx + 10) % 12) as i32;
    let mut stars = Vec::new();

    // 천재(天才): 명궁에서 년지 인덱스만큼 순행
    stars.push((ZwdsStar::TianCai, fix_index(soul_idx as i32 + y_zwds)));
    // 천수(天壽): 신궁에서 년지 인덱스만큼 순행
    stars.push((ZwdsStar::TianShou, fix_index(body_idx as i32 + y_zwds)));

    let s_idx = year_stem.index() as usize;

    // 천관(天官)
    let tianguan_table = [5, 2, 3, 0, 1, 7, 9, 7, 8, 4];
    stars.push((ZwdsStar::TianGuan, tianguan_table[s_idx]));

    // 천복(天福)
    let tianfu_table = [7, 6, 10, 9, 1, 0, 4, 3, 4, 3];
    stars.push((ZwdsStar::TianFu2, tianfu_table[s_idx]));

    // 천주(天廚)
    let tianchu_table = [3, 4, 10, 3, 4, 6, 0, 4, 7, 9];
    stars.push((ZwdsStar::TianChu, tianchu_table[s_idx]));

    stars
}
