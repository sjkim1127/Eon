//! 자미두수 도메인 타입 정의
//!
//! 좌표계: 12궁은 寅宮=0으로 시작, 반시계 방향(逆時針) 증가
//! [0=寅, 1=卯, 2=辰, 3=巳, 4=午, 5=未, 6=申, 7=酉, 8=戌, 9=亥, 10=子, 11=丑]

use serde::{Deserialize, Serialize};

// ============================================================
// 12궁(宮) 인덱스
// ============================================================

/// 자미두수 좌표계: 寅=0, 반시계 방향
/// `fix_index(i)` = `((i % 12) + 12) % 12`
pub type PalaceIndex = usize;

/// 12 궁위(宮位) 이름
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PalaceName {
    /// 命宮 (명궁) — 자아/성격
    Ming,
    /// 兄弟宮 (형제궁)
    Xiongdi,
    /// 夫妻宮 (부처궁)
    Fuqi,
    /// 子女宮 (자녀궁)
    Zinv,
    /// 財帛宮 (재백궁)
    Caibo,
    /// 疾厄宮 (질액궁)
    Jie,
    /// 遷移宮 (천이궁)
    Qianyi,
    /// 奴僕宮 (노복궁)
    Nupao,
    /// 官祿宮 (관록궁)
    Guanlu,
    /// 田宅宮 (전택궁)
    Tianzhai,
    /// 福德宮 (복덕궁)
    Fude,
    /// 父母宮 (부모궁)
    Fumu,
}

impl PalaceName {
    pub const ALL: [PalaceName; 12] = [
        Self::Ming, Self::Xiongdi, Self::Fuqi, Self::Zinv,
        Self::Caibo, Self::Jie, Self::Qianyi, Self::Nupao,
        Self::Guanlu, Self::Tianzhai, Self::Fude, Self::Fumu,
    ];

    pub fn hanja(&self) -> &'static str {
        match self {
            Self::Ming     => "命宮",
            Self::Xiongdi  => "兄弟宮",
            Self::Fuqi     => "夫妻宮",
            Self::Zinv     => "子女宮",
            Self::Caibo    => "財帛宮",
            Self::Jie      => "疾厄宮",
            Self::Qianyi   => "遷移宮",
            Self::Nupao    => "奴僕宮",
            Self::Guanlu   => "官祿宮",
            Self::Tianzhai => "田宅宮",
            Self::Fude     => "福德宮",
            Self::Fumu     => "父母宮",
        }
    }

    pub fn korean(&self) -> &'static str {
        match self {
            Self::Ming     => "명궁",
            Self::Xiongdi  => "형제궁",
            Self::Fuqi     => "부처궁",
            Self::Zinv     => "자녀궁",
            Self::Caibo    => "재백궁",
            Self::Jie      => "질액궁",
            Self::Qianyi   => "천이궁",
            Self::Nupao    => "노복궁",
            Self::Guanlu   => "관록궁",
            Self::Tianzhai => "전택궁",
            Self::Fude     => "복덕궁",
            Self::Fumu     => "부모궁",
        }
    }
}

// ============================================================
// 별(星) 열거형
// ============================================================

/// 자미두수 전체 성계(星系)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ZwdsStar {
    // ── 14 主星 ──────────────────────────────────────────
    /// 紫微 (자미)
    ZiWei,
    /// 天機 (천기)
    TianJi,
    /// 太陽 (태양)
    TaiYang,
    /// 武曲 (무곡)
    WuQu,
    /// 天同 (천동)
    TianTong,
    /// 廉貞 (렴정)
    LianZhen,
    /// 天府 (천부)
    TianFu,
    /// 太陰 (태음)
    TaiYin,
    /// 貪狼 (탐랑)
    TanLang,
    /// 巨門 (거문)
    JuMen,
    /// 天相 (천상)
    TianXiang,
    /// 天梁 (천량)
    TianLiang,
    /// 七殺 (칠살)
    QiSha,
    /// 破軍 (파군)
    PoJun,

    // ── 6 輔星 A ─────────────────────────────────────────
    /// 文昌 (문창)
    WenChang,
    /// 文曲 (문곡)
    WenQu,
    /// 左輔 (좌보)
    ZuoFu,
    /// 右弼 (우필)
    YouBi,
    /// 天魁 (천괴)
    TianKui,
    /// 天鉞 (천월)
    TianYue,

    // ── 祿馬三方 ──────────────────────────────────────────
    /// 禄存 (록존)
    LuCun,
    /// 擎羊 (경양)
    QingYang,
    /// 陀羅 (타라)
    TuoLuo,
    /// 天馬 (천마)
    TianMa,

    // ── 火鈴 ─────────────────────────────────────────────
    /// 火星 (화성)
    HuoXing,
    /// 鈴星 (영성)
    LingXing,

    // ── 空劫 ─────────────────────────────────────────────
    /// 地劫 (지겁)
    DiJie,
    /// 地空 (지공)
    DiKong,

    // ── 中諸星 (중잡성, ~24성) ───────────────────────────
    /// 紅鸞 (홍란)
    HongLuan,
    /// 天喜 (천희)
    TianXi,
    /// 天刑 (천형)
    TianXing,
    /// 天姚 (천요)
    TianYao,
    /// 解神 (해신)
    JieShen,
    /// 天巫 (천무)
    TianWu,
    /// 天月 (천월성, TianYue와 다름)
    TianYueStar,
    /// 陰煞 (음살)
    YinSha,
    /// 台輔 (대보)
    TaiFu,
    /// 封誥 (봉고)
    FengGao,
    /// 三台 (삼태)
    SanTai,
    /// 八座 (팔좌)
    BaZuo,
    /// 恩光 (은광)
    EnGuang,
    /// 天貴 (천귀)
    TianGui,
    /// 天才 (천재)
    TianCai,
    /// 天壽 (천수)
    TianShou,
    /// 龍池 (용지)
    LongChi,
    /// 鳳閣 (봉각)
    FengGe,
    /// 天哭 (천곡)
    TianKu,
    /// 天虛 (천허)
    TianXu,
    /// 華蓋 (화개)
    HuaGai,
    /// 咸池 (함지)
    XianChi,
    /// 孤辰 (고신)
    GuChen,
    /// 寡宿 (과숙)
    GuaSu,
    /// 天空 (천공)
    TianKong,
    /// 劫殺 (겁살)
    JieSha,
    /// 天廚 (천주)
    TianChu,
    /// 天官 (천관)
    TianGuan,
    /// 天福 (천복)
    TianFu2, // 天府와 이름 충돌 방지
}

impl ZwdsStar {
    /// 한자 표기
    pub fn hanja(&self) -> &'static str {
        match self {
            Self::ZiWei    => "紫微",
            Self::TianJi   => "天機",
            Self::TaiYang  => "太陽",
            Self::WuQu     => "武曲",
            Self::TianTong => "天同",
            Self::LianZhen => "廉貞",
            Self::TianFu   => "天府",
            Self::TaiYin   => "太陰",
            Self::TanLang  => "貪狼",
            Self::JuMen    => "巨門",
            Self::TianXiang => "天相",
            Self::TianLiang => "天梁",
            Self::QiSha    => "七殺",
            Self::PoJun    => "破軍",
            Self::WenChang => "文昌",
            Self::WenQu    => "文曲",
            Self::ZuoFu    => "左輔",
            Self::YouBi    => "右弼",
            Self::TianKui  => "天魁",
            Self::TianYue  => "天鉞",
            Self::LuCun    => "禄存",
            Self::QingYang => "擎羊",
            Self::TuoLuo   => "陀羅",
            Self::TianMa   => "天馬",
            Self::HuoXing  => "火星",
            Self::LingXing => "鈴星",
            Self::DiJie    => "地劫",
            Self::DiKong   => "地空",
            Self::HongLuan => "紅鸞",
            Self::TianXi   => "天喜",
            Self::TianXing => "天刑",
            Self::TianYao  => "天姚",
            Self::JieShen  => "解神",
            Self::TianWu   => "天巫",
            Self::TianYueStar => "天月",
            Self::YinSha   => "陰煞",
            Self::TaiFu    => "台輔",
            Self::FengGao  => "封誥",
            Self::SanTai   => "三台",
            Self::BaZuo    => "八座",
            Self::EnGuang  => "恩光",
            Self::TianGui  => "天貴",
            Self::TianCai  => "天才",
            Self::TianShou => "天壽",
            Self::LongChi  => "龍池",
            Self::FengGe   => "鳳閣",
            Self::TianKu   => "天哭",
            Self::TianXu   => "天虛",
            Self::HuaGai   => "華蓋",
            Self::XianChi  => "咸池",
            Self::GuChen   => "孤辰",
            Self::GuaSu    => "寡宿",
            Self::TianKong => "天空",
            Self::JieSha   => "劫殺",
            Self::TianChu  => "天廚",
            Self::TianGuan => "天官",
            Self::TianFu2  => "天福",
        }
    }

    /// 한글 표기
    pub fn korean(&self) -> &'static str {
        match self {
            Self::ZiWei    => "자미",
            Self::TianJi   => "천기",
            Self::TaiYang  => "태양",
            Self::WuQu     => "무곡",
            Self::TianTong => "천동",
            Self::LianZhen => "렴정",
            Self::TianFu   => "천부",
            Self::TaiYin   => "태음",
            Self::TanLang  => "탐랑",
            Self::JuMen    => "거문",
            Self::TianXiang => "천상",
            Self::TianLiang => "천량",
            Self::QiSha    => "칠살",
            Self::PoJun    => "파군",
            Self::WenChang => "문창",
            Self::WenQu    => "문곡",
            Self::ZuoFu    => "좌보",
            Self::YouBi    => "우필",
            Self::TianKui  => "천괴",
            Self::TianYue  => "천월",
            Self::LuCun    => "록존",
            Self::QingYang => "경양",
            Self::TuoLuo   => "타라",
            Self::TianMa   => "천마",
            Self::HuoXing  => "화성",
            Self::LingXing => "영성",
            Self::DiJie    => "지겁",
            Self::DiKong   => "지공",
            Self::HongLuan => "홍란",
            Self::TianXi   => "천희",
            Self::TianXing => "천형",
            Self::TianYao  => "천요",
            Self::JieShen  => "해신",
            Self::TianWu   => "천무",
            Self::TianYueStar => "천월성",
            Self::YinSha   => "음살",
            Self::TaiFu    => "대보",
            Self::FengGao  => "봉고",
            Self::SanTai   => "삼태",
            Self::BaZuo    => "팔좌",
            Self::EnGuang  => "은광",
            Self::TianGui  => "천귀",
            Self::TianCai  => "천재",
            Self::TianShou => "천수",
            Self::LongChi  => "용지",
            Self::FengGe   => "봉각",
            Self::TianKu   => "천곡",
            Self::TianXu   => "천허",
            Self::HuaGai   => "화개",
            Self::XianChi  => "함지",
            Self::GuChen   => "고신",
            Self::GuaSu    => "과숙",
            Self::TianKong => "천공",
            Self::JieSha   => "겁살",
            Self::TianChu  => "천주",
            Self::TianGuan => "천관",
            Self::TianFu2  => "천복",
        }
    }

    /// 주성 여부
    pub fn is_main_star(&self) -> bool {
        matches!(self,
            Self::ZiWei | Self::TianJi | Self::TaiYang | Self::WuQu |
            Self::TianTong | Self::LianZhen | Self::TianFu | Self::TaiYin |
            Self::TanLang | Self::JuMen | Self::TianXiang | Self::TianLiang |
            Self::QiSha | Self::PoJun
        )
    }
}

// ============================================================
// 오행국(五行局)
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FiveElementsClass {
    /// 水二局 — 대한 2세 시작
    Water2,
    /// 木三局 — 대한 3세 시작
    Wood3,
    /// 金四局 — 대한 4세 시작
    Metal4,
    /// 土五局 — 대한 5세 시작
    Earth5,
    /// 火六局 — 대한 6세 시작
    Fire6,
}

impl FiveElementsClass {
    /// 대한 시작 나이
    pub fn starting_age(&self) -> u8 {
        match self {
            Self::Water2 => 2,
            Self::Wood3  => 3,
            Self::Metal4 => 4,
            Self::Earth5 => 5,
            Self::Fire6  => 6,
        }
    }

    /// 오행국 숫자 값
    pub fn value(&self) -> u32 {
        match self {
            Self::Water2 => 2,
            Self::Wood3  => 3,
            Self::Metal4 => 4,
            Self::Earth5 => 5,
            Self::Fire6  => 6,
        }
    }

    pub fn korean(&self) -> &'static str {
        match self {
            Self::Water2 => "수이국(水二局)",
            Self::Wood3  => "목삼국(木三局)",
            Self::Metal4 => "금사국(金四局)",
            Self::Earth5 => "토오국(土五局)",
            Self::Fire6  => "화육국(火六局)",
        }
    }
}

// ============================================================
// 사화(四化)
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SiHuaType {
    /// 化祿 (화록) — 재물·복록
    HuaLu,
    /// 化權 (화권) — 권위·리더십
    HuaQuan,
    /// 化科 (화과) — 명예·학문
    HuaKe,
    /// 化忌 (화기) — 장애·시련
    HuaJi,
}

impl SiHuaType {
    pub fn korean(&self) -> &'static str {
        match self {
            Self::HuaLu   => "화록(化祿)",
            Self::HuaQuan => "화권(化權)",
            Self::HuaKe   => "화과(化科)",
            Self::HuaJi   => "화기(化忌)",
        }
    }
    pub fn emoji(&self) -> &'static str {
        match self {
            Self::HuaLu   => "祿",
            Self::HuaQuan => "權",
            Self::HuaKe   => "科",
            Self::HuaJi   => "忌",
        }
    }
}

// ============================================================
// 각 궁(宮)의 데이터
// ============================================================

/// 자미두수 별의 밝기 (廟旺利陷)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ZwdsBrightness {
    /// 廟 (Temple) — 가장 밝고 길함
    Miao,
    /// 旺 (Bright) — 매우 밝고 길함
    Wang,
    /// 得 (Gain) — 얻음, 길함
    De,
    /// 利 (Benefit) — 이로움, 보통 이상
    Li,
    /// 平 (Peace) — 평범함
    Ping,
    /// 不 (Not) — 밝지 않음, 不得地
    Bu,
    /// 陷 (Fallen) — 낙함, 가장 어둡고 흉함
    Xian,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StarInPalace {
    pub star: ZwdsStar,
    /// 사화 태그 (해당 별에 사화가 붙는 경우)
    pub si_hua: Option<SiHuaType>,
    /// 별의 밝기 (廟旺利陷)
    pub brightness: Option<ZwdsBrightness>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PalaceData {
    /// 궁 인덱스 (0=寅宮)
    pub index: PalaceIndex,
    /// 궁 이름
    pub name: PalaceName,
    /// 천간(天干) — 궁의 천간
    pub heavenly_stem: String,
    /// 지지(地支) — 궁의 지지
    pub earthly_branch: String,
    /// 이 궁에 배치된 별들
    pub stars: Vec<StarInPalace>,
    /// 대한(大限) 연령 범위 (이 궁이 몇 세 대한인지)
    pub daxian_range: Option<(u8, u8)>,
    /// 유년(流年) 해당 여부
    pub is_current_liu_nian: bool,
}

// ============================================================
// 음력 데이터
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LunarBirthInfo {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub is_leap_month: bool,
    /// 연간(年干) 인덱스 (0=甲…9=癸)
    pub year_stem_idx: usize,
    /// 연지(年支) 인덱스 (0=子…11=亥, 표준 子=0 기준)
    pub year_branch_idx: usize,
    /// 시지(時支) 인덱스 (0=子…11=亥)
    pub time_branch_idx: usize,
}

// ============================================================
// 운세 분석 데이터 (대한 / 유년 / 성반)
// ============================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DaXian {
    pub index: u32,
    pub age_start: u32,
    pub age_end: u32,
    pub palace_idx: PalaceIndex,
    pub stem_hanja: String,
    pub branch_hanja: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LiuNian {
    pub year: i32,
    pub palace_idx: PalaceIndex,
    pub stem_hanja: String,
    pub branch_hanja: String,
    pub si_hua: [ZwdsStar; 4],
    pub liu_lu: PalaceIndex,
    pub liu_yang: PalaceIndex,
    pub liu_tuo: PalaceIndex,
    pub liu_chang: PalaceIndex,
    pub liu_qu: PalaceIndex,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyPattern {
    pub name_hanja: String,
    pub name_korean: String,
    pub is_auspicious: bool,
    pub description_korean: String,
    pub description_english: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlyingSiHua {
    pub from_palace: PalaceName,
    pub to_palace: PalaceName,
    pub sihua_type: SiHuaType,
    pub star: ZwdsStar,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZwdsChart {
    pub palaces: [PalaceData; 12],
    pub soul_idx: PalaceIndex,
    pub body_idx: PalaceIndex,
    pub soul_master: ZwdsStar,
    pub body_master: ZwdsStar,
    pub five_elements: FiveElementsClass,
    pub daxian: Vec<DaXian>,
    pub destiny_patterns: Vec<DestinyPattern>,
    pub flying_sihua: Vec<FlyingSiHua>,
}

