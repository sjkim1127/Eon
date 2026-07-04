//! 신살(神煞, Spirit Markers) 분석
//!
//! 사주에서 특정 조합으로 나타나는 길신(吉神)과 흉살(凶煞)을 분석합니다.
//!
//! ## 주요 신살
//!
//! ### 길신 (吉神)
//! - 천을귀인, 문창귀인, 태극귀인, 월덕귀인, 정록 등
//!
//! ### 흉살 (凶煞)
//! - 역마살, 화개살, 괴강살, 도화살, 고신살 등

use crate::analysis::shinsal::{EvilSpirit, Gilsin, TwelveShinsal};
use crate::analysis::supplementary_pillars::InterpretationLevel;
use crate::core::branch::EarthlyBranch;
use crate::core::ganzi::GanZi;
use crate::core::pillars::FourPillars;
use crate::core::stem::HeavenlyStem;
use serde::{Deserialize, Serialize};

/// 신살 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SpiritMarker {
    // === 길신 (吉神) ===
    /// 천을귀인(天乙貴人) - 귀인의 도움
    Tianyi,
    /// 문창귀인(文昌貴人) - 학문, 시험운
    Wenchang,
    /// 태극귀인(太極貴人) - 영적 보호
    Taiji,
    /// 월덕귀인(月德貴人) - 월간의 덕
    Yuede,
    /// 천덕귀인(天德貴人) - 천간의 덕
    Tiande,
    /// 정록(正祿) - 녹성, 재물운
    Zhenglu,
    /// 금여록(金輿祿) - 귀하고 화려한 운
    Jinyu,
    /// 암록(暗祿) - 숨겨진 재물운
    Anlu,
    /// 학당귀인(學堂貴人) - 학업 성취
    Xuetang,
    /// 천의성(天醫星) - 의료, 치유 능력
    TianyiMedical,
    /// 천문성(天文星) - 학문, 예술
    Tianwen,

    // === 흉살 (凶煞) ===
    /// 역마살(驛馬煞) - 이동, 변화
    Yima,
    /// 화개살(華蓋煞) - 고독, 예술성
    Huagai,
    /// 괴강살(魁罡煞) - 강한 성격
    Kuigang,
    /// 도화살(桃花煞) - 이성 관계
    Taohua,
    /// 홍염살(紅艶煞) - 색정, 매력
    Hongyan,
    /// 고신살(孤辰煞) - 고독
    Guchen,
    /// 과숙살(寡宿煞) - 독거
    Guasu,
    /// 현침살(懸針煞) - 날카로움
    Xuanzhen,
    /// 백호살(白虎煞) - 흉험
    Baihu,
    /// 망신살(亡身煞) - 손실
    Wangshen,
    /// 겁살(劫煞) - 겁탈
    Jiesha,
    /// 원진살(怨嗔煞) - 원한
    Yuanzhen,
    /// 양인살(羊刃煞) - 강력한 칼을 쥔 기세
    Yangin,
    /// 천라살(天羅煞) - 하늘의 그물
    Tianluo,
    /// 지망살(地網煞) - 땅의 그물
    Jimang,

    // === 12신살 (추가분) ===
    /// 재살(災煞) - 수옥살, 재난
    Jaesha,
    /// 천살(天煞) - 하늘의 재앙
    Cheonsha,
    /// 지살(地煞) - 땅의 변화, 이동
    Jisha,
    /// 년살/도화살(年煞) - 인기, 화려함
    Nyeonsha,
    /// 월살(月煞) - 고초살, 달빛 아래 고독
    Wolsha,
    /// 장성살(將星살) - 권위, 우두머리
    Jangseong,
    /// 반안살(潘鞍煞) - 말 안장, 편안함
    Banan,
    /// 육해살(六害煞) - 여섯 가지 장애
    Yukhae,
}

impl SpiritMarker {
    /// 한글 이름
    pub const fn hangul(&self) -> &'static str {
        match self {
            Self::Tianyi => "천을귀인",
            Self::Wenchang => "문창귀인",
            Self::Taiji => "태극귀인",
            Self::Yuede => "월덕귀인",
            Self::Tiande => "천덕귀인",
            Self::Zhenglu => "정록",
            Self::Jinyu => "금여록",
            Self::Anlu => "암록",
            Self::Xuetang => "학당귀인",
            Self::TianyiMedical => "천의성",
            Self::Tianwen => "천문성",
            Self::Yima => "역마살",
            Self::Huagai => "화개살",
            Self::Kuigang => "괴강살",
            Self::Taohua => "도화살",
            Self::Hongyan => "홍염살",
            Self::Guchen => "고신살",
            Self::Guasu => "과숙살",
            Self::Xuanzhen => "현침살",
            Self::Baihu => "백호살",
            Self::Wangshen => "망신살",
            Self::Jiesha => "겁살",
            Self::Yuanzhen => "원진살",
            Self::Yangin => "양인살",
            Self::Tianluo => "천라살",
            Self::Jimang => "지망살",
            Self::Jaesha => "재살",
            Self::Cheonsha => "천살",
            Self::Jisha => "지살",
            Self::Nyeonsha => "년살",
            Self::Wolsha => "월살",
            Self::Jangseong => "장성살",
            Self::Banan => "반안살",
            Self::Yukhae => "육해살",
        }
    }

    /// 한자 이름
    pub const fn hanja(&self) -> &'static str {
        match self {
            Self::Tianyi => "天乙貴人",
            Self::Wenchang => "文昌貴人",
            Self::Taiji => "太極貴人",
            Self::Yuede => "月德貴人",
            Self::Tiande => "天德貴人",
            Self::Zhenglu => "正祿",
            Self::Jinyu => "金輿祿",
            Self::Anlu => "暗祿",
            Self::Xuetang => "學堂貴人",
            Self::TianyiMedical => "天醫星",
            Self::Tianwen => "天文星",
            Self::Yima => "驛馬煞",
            Self::Huagai => "華蓋煞",
            Self::Kuigang => "魁罡煞",
            Self::Taohua => "桃花煞",
            Self::Hongyan => "紅艶煞",
            Self::Guchen => "孤辰煞",
            Self::Guasu => "寡宿煞",
            Self::Xuanzhen => "懸針煞",
            Self::Baihu => "白虎煞",
            Self::Wangshen => "亡身煞",
            Self::Jiesha => "劫煞",
            Self::Yuanzhen => "怨嗔煞",
            Self::Yangin => "羊刃煞",
            Self::Tianluo => "天羅煞",
            Self::Jimang => "地網煞",
            Self::Jaesha => "災煞",
            Self::Cheonsha => "天煞",
            Self::Jisha => "地煞",
            Self::Nyeonsha => "年煞",
            Self::Wolsha => "月煞",
            Self::Jangseong => "將星煞",
            Self::Banan => "潘鞍煞",
            Self::Yukhae => "六害煞",
        }
    }

    /// 길신 여부
    pub const fn is_auspicious(&self) -> bool {
        matches!(
            self,
            Self::Tianyi
                | Self::Wenchang
                | Self::Taiji
                | Self::Yuede
                | Self::Tiande
                | Self::Zhenglu
                | Self::Jinyu
                | Self::Anlu
                | Self::Xuetang
                | Self::TianyiMedical
                | Self::Tianwen
        )
    }

    /// 설명
    pub const fn description(&self) -> &'static str {
        match self {
            Self::Tianyi => "귀인의 도움을 받아 어려움을 극복",
            Self::Wenchang => "학문과 시험에 유리, 문서 관련 길함",
            Self::Taiji => "영적 보호, 종교/철학에 재능",
            Self::Yuede => "월간의 덕으로 재난 회피",
            Self::Tiande => "천간의 덕으로 흉을 피함",
            Self::Zhenglu => "정당한 재물운, 녹봉",
            Self::Jinyu => "귀하고 화려한 운명",
            Self::Anlu => "숨겨진 재물운",
            Self::Xuetang => "학업 성취에 유리",
            Self::TianyiMedical => "의료, 치유 분야에 재능",
            Self::Tianwen => "학문과 예술에 특별한 재능",
            Self::Yima => "이동수, 변화가 많음",
            Self::Huagai => "예술적 감각, 고독한 경향",
            Self::Kuigang => "강한 성격, 리더십",
            Self::Taohua => "이성에게 인기, 연애운",
            Self::Hongyan => "매력적, 색정 주의",
            Self::Guchen => "외로움, 독립적",
            Self::Guasu => "배우자운 불리",
            Self::Xuanzhen => "날카로운 성격",
            Self::Baihu => "사고, 부상 주의",
            Self::Wangshen => "실수나 치부의 노출, 그러나 화려한 변신",
            Self::Jiesha => "외부에 의한 강압적 변화나 손실",
            Self::Yuanzhen => "대인관계 갈등 및 불화",
            Self::Yangin => "강력한 기세와 고집, 수술수 및 급격한 재물 손실 우려",
            Self::Tianluo => "하늘의 그물에 갇힌 듯한 답답함, 정신적 탐구 및 종교/예술 적합",
            Self::Jimang => "땅의 그물에 걸린 듯한 제약, 현실적 한계 극복 및 기술 분야 적합",
            Self::Jaesha => "재난과 사고, 혹은 타인에 의한 구속",
            Self::Cheonsha => "하늘의 뜻에 따른 거부할 수 없는 변화",
            Self::Jisha => "지리적 이동과 활동 영역의 변화",
            Self::Nyeonsha => "대중의 인기와 화려한 매력, 도화살",
            Self::Wolsha => "불우한 환경 속에서 피어나는 결실",
            Self::Jangseong => "자신의 분야에서 정점에 오르는 우두머리 기질",
            Self::Banan => "명예로운 위치와 안정된 보상",
            Self::Yukhae => "질병이나 장애, 혹은 매우 민첩한 대처",
        }
    }
}

impl std::fmt::Display for SpiritMarker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hangul())
    }
}

/// 신살 발견 위치
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PillarPosition {
    Year,
    Month,
    Day,
    Hour,
}

impl PillarPosition {
    pub const fn hangul(&self) -> &'static str {
        match self {
            Self::Year => "년주",
            Self::Month => "월주",
            Self::Day => "일주",
            Self::Hour => "시주",
        }
    }
}

/// 발견된 신살 정보
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoundMarker {
    pub marker: SpiritMarker,
    pub position: PillarPosition,
    pub is_stem: bool, // true=천간, false=지지
}

impl std::fmt::Display for FoundMarker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let part = if self.is_stem { "천간" } else { "지지" };
        write!(f, "{} {} - {}", self.position.hangul(), part, self.marker)
    }
}

/// 신살 상세 정보 (Explainable Spirit Marker)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpiritMarkerDetail {
    pub marker: SpiritMarker,
    pub position: PillarPosition,
    pub is_stem: bool,
    pub level: InterpretationLevel,
    pub summary: String,
    pub description: String,
    pub reasons: Vec<String>,
}

/// 신살 분석 결과
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpiritMarkerAnalysis {
    /// 발견된 모든 신살 상세 정보 (고도의 설명 가능성 포함)
    pub mapped_markers: Vec<SpiritMarkerDetail>,
    /// 발견된 모든 신살 (레거시 호환용)
    pub markers: Vec<FoundMarker>,
    /// 길신 목록
    pub auspicious: Vec<SpiritMarker>,
    /// 흉살 목록
    pub inauspicious: Vec<SpiritMarker>,
    /// 보조 기둥 관련 신살 [기둥명, 기준명, 신살명]
    pub aux_shinsals: Vec<(String, String, String)>,
}

impl SpiritMarkerAnalysis {
    /// 사주에서 신살 분석
    pub fn from_pillars(pillars: &FourPillars) -> Self {
        let mut markers = Vec::new();

        let branches = [
            (pillars.year.branch, PillarPosition::Year),
            (pillars.month.branch, PillarPosition::Month),
            (pillars.day.branch, PillarPosition::Day),
            (pillars.hour.branch, PillarPosition::Hour),
        ];

        let stems = [
            (pillars.year.stem, PillarPosition::Year),
            (pillars.month.stem, PillarPosition::Month),
            (pillars.day.stem, PillarPosition::Day),
            (pillars.hour.stem, PillarPosition::Hour),
        ];

        let day_branch = pillars.day.branch;
        let day_stem = pillars.day.stem;
        let year_branch = pillars.year.branch;
        let month_branch = pillars.month.branch;

        // === 천을귀인 (天乙貴人) ===
        let tianyi_branches = Self::get_tianyi_branches(day_stem);
        for (branch, pos) in &branches {
            if tianyi_branches.contains(branch) {
                markers.push(FoundMarker {
                    marker: SpiritMarker::Tianyi,
                    position: *pos,
                    is_stem: false,
                });
            }
        }

        // === 12신살 (년지 기준) ===
        let year_sindal = Self::get_12_sindal_map(year_branch);
        for (target_branch, marker) in year_sindal {
            for (curr_branch, pos) in &branches {
                if *curr_branch == target_branch {
                    markers.push(FoundMarker {
                        marker,
                        position: *pos,
                        is_stem: false,
                    });
                }
            }
        }

        // === 12신살 (일지 기준) ===
        let day_sindal = Self::get_12_sindal_map(day_branch);
        for (target_branch, marker) in day_sindal {
            for (curr_branch, pos) in &branches {
                if *curr_branch == target_branch {
                    // 년지 기준과 중복되지 않는 경우에만 추가 (또는 중복 허용 정책에 따라)
                    if !markers
                        .iter()
                        .any(|m| m.marker == marker && m.position == *pos)
                    {
                        markers.push(FoundMarker {
                            marker,
                            position: *pos,
                            is_stem: false,
                        });
                    }
                }
            }
        }

        // === 괴강살 (魁罡煞) ===
        // 일주 기준이 기본이나 년주에 있어도 작용함
        for (ganzi, pos) in &[
            (pillars.year, PillarPosition::Year),
            (pillars.day, PillarPosition::Day),
        ] {
            if Self::is_kuigang(*ganzi) {
                markers.push(FoundMarker {
                    marker: SpiritMarker::Kuigang,
                    position: *pos,
                    is_stem: false,
                });
            }
        }

        // === 문창귀인 (文昌貴人) ===
        let wenchang_branch = Self::get_wenchang_branch(day_stem);
        for (branch, pos) in &branches {
            if *branch == wenchang_branch {
                markers.push(FoundMarker {
                    marker: SpiritMarker::Wenchang,
                    position: *pos,
                    is_stem: false,
                });
            }
        }

        // === 태극귀인 (太極貴人) ===
        let taiji_branches = Self::get_taiji_branches(day_stem);
        for (branch, pos) in &branches {
            if taiji_branches.contains(branch) {
                markers.push(FoundMarker {
                    marker: SpiritMarker::Taiji,
                    position: *pos,
                    is_stem: false,
                });
            }
        }

        // === 월덕귀인 (月德貴人) ===
        let yuede_stem = Self::get_yuede_stem(month_branch);
        for (stem, pos) in &stems {
            if *stem == yuede_stem {
                markers.push(FoundMarker {
                    marker: SpiritMarker::Yuede,
                    position: *pos,
                    is_stem: true,
                });
            }
        }

        // === 천덕귀인 (天德貴人) ===
        if let Some(tiande_stem) = Self::get_tiande_stem(month_branch) {
            for (stem, pos) in &stems {
                if *stem == tiande_stem {
                    markers.push(FoundMarker {
                        marker: SpiritMarker::Tiande,
                        position: *pos,
                        is_stem: true,
                    });
                }
            }
        }

        // === 정록 (正祿) ===
        let zhenglu_branch = Self::get_zhenglu_branch(day_stem);
        for (branch, pos) in &branches {
            if *branch == zhenglu_branch {
                markers.push(FoundMarker {
                    marker: SpiritMarker::Zhenglu,
                    position: *pos,
                    is_stem: false,
                });
            }
        }

        // === 고신살 (孤辰煞) ===
        let guchen_branch = Self::get_guchen_branch(year_branch);
        for (branch, pos) in &branches {
            if *branch == guchen_branch {
                markers.push(FoundMarker {
                    marker: SpiritMarker::Guchen,
                    position: *pos,
                    is_stem: false,
                });
            }
        }

        // === 과숙살 (寡宿煞) ===
        let guasu_branch = Self::get_guasu_branch(year_branch);
        for (branch, pos) in &branches {
            if *branch == guasu_branch {
                markers.push(FoundMarker {
                    marker: SpiritMarker::Guasu,
                    position: *pos,
                    is_stem: false,
                });
            }
        }

        // === 홍염살 (紅艶煞) ===
        if let Some(hongyan_branch) = Self::get_hongyan_branch(day_stem) {
            for (branch, pos) in &branches {
                if *branch == hongyan_branch {
                    markers.push(FoundMarker {
                        marker: SpiritMarker::Hongyan,
                        position: *pos,
                        is_stem: false,
                    });
                }
            }
        }

        // === 현침살 (懸針煞) ===
        // 甲, 辛, 卯, 午, 申 글자에 세로획이 관통하는 형태
        for (stem, pos) in &stems {
            if Self::is_xuanzhen_stem(*stem) {
                markers.push(FoundMarker {
                    marker: SpiritMarker::Xuanzhen,
                    position: *pos,
                    is_stem: true,
                });
            }
        }
        for (branch, pos) in &branches {
            if Self::is_xuanzhen_branch(*branch) {
                markers.push(FoundMarker {
                    marker: SpiritMarker::Xuanzhen,
                    position: *pos,
                    is_stem: false,
                });
            }
        }

        // === 백호살 (白虎煞) ===
        // 甲辰, 乙未, 丙戌, 丁丑, 戊辰, 壬戌, 癸丑
        let pillars_ganzi = [
            (pillars.year, PillarPosition::Year),
            (pillars.month, PillarPosition::Month),
            (pillars.day, PillarPosition::Day),
            (pillars.hour, PillarPosition::Hour),
        ];
        for (ganzi, pos) in &pillars_ganzi {
            if Self::is_baihu(*ganzi) {
                markers.push(FoundMarker {
                    marker: SpiritMarker::Baihu,
                    position: *pos,
                    is_stem: false,
                });
            }
        }

        // === 천의성 (天醫星) ===
        let tianyi_branch = Self::get_tianyi_medical_branch(month_branch);
        for (branch, pos) in &branches {
            if *branch == tianyi_branch {
                markers.push(FoundMarker {
                    marker: SpiritMarker::TianyiMedical,
                    position: *pos,
                    is_stem: false,
                });
            }
        }

        // === 천문성 (天文星) ===
        // 亥가 있으면 천문성
        for (branch, pos) in &branches {
            if *branch == EarthlyBranch::Hai {
                markers.push(FoundMarker {
                    marker: SpiritMarker::Tianwen,
                    position: *pos,
                    is_stem: false,
                });
            }
        }

        // === 금여록 (金輿祿) ===
        if let Some(jinyu_branch) = Self::get_jinyu_branch(day_stem) {
            for (branch, pos) in &branches {
                if *branch == jinyu_branch {
                    markers.push(FoundMarker {
                        marker: SpiritMarker::Jinyu,
                        position: *pos,
                        is_stem: false,
                    });
                }
            }
        }

        // === 망신살 (亡身煞) ===
        let wangshen_branch = Self::get_wangshen_branch(year_branch);
        for (branch, pos) in &branches {
            if *branch == wangshen_branch {
                markers.push(FoundMarker {
                    marker: SpiritMarker::Wangshen,
                    position: *pos,
                    is_stem: false,
                });
            }
        }

        // === 겁살 (劫煞) ===
        let jiesha_branch = Self::get_jiesha_branch(year_branch);
        for (branch, pos) in &branches {
            if *branch == jiesha_branch {
                markers.push(FoundMarker {
                    marker: SpiritMarker::Jiesha,
                    position: *pos,
                    is_stem: false,
                });
            }
        }

        // === 암록 (暗祿) ===
        if let Some(anlu_branch) = Self::get_anlu_branch(day_stem) {
            for (branch, pos) in &branches {
                if *branch == anlu_branch {
                    markers.push(FoundMarker {
                        marker: SpiritMarker::Anlu,
                        position: *pos,
                        is_stem: false,
                    });
                }
            }
        }

        // === 학당귀인 (學堂貴인) ===
        if let Some(xuetang_branch) = Self::get_xuetang_branch(day_stem) {
            for (branch, pos) in &branches {
                if *branch == xuetang_branch {
                    markers.push(FoundMarker {
                        marker: SpiritMarker::Xuetang,
                        position: *pos,
                        is_stem: false,
                    });
                }
            }
        }

        // === 원진살 (怨嗔煞) ===
        for (branch, pos) in &branches {
            if Self::is_yuanzhen(year_branch, *branch) {
                markers.push(FoundMarker {
                    marker: SpiritMarker::Yuanzhen,
                    position: *pos,
                    is_stem: false,
                });
            }
        }

        // === 양인살 (羊刃煞) ===
        let yangin_branch = Self::get_yangin_branch(day_stem);
        for (branch, pos) in &branches {
            if *branch == yangin_branch {
                markers.push(FoundMarker {
                    marker: SpiritMarker::Yangin,
                    position: *pos,
                    is_stem: false,
                });
            }
        }

        // === 천라살 (天羅煞) ===
        let has_xu = branches.iter().any(|(b, _)| *b == EarthlyBranch::Xu);
        let has_hai = branches.iter().any(|(b, _)| *b == EarthlyBranch::Hai);
        if has_xu && has_hai {
            for (branch, pos) in &branches {
                if *branch == EarthlyBranch::Xu || *branch == EarthlyBranch::Hai {
                    markers.push(FoundMarker {
                        marker: SpiritMarker::Tianluo,
                        position: *pos,
                        is_stem: false,
                    });
                }
            }
        }

        // === 지망살 (地網煞) ===
        let has_chen = branches.iter().any(|(b, _)| *b == EarthlyBranch::Chen);
        let has_si = branches.iter().any(|(b, _)| *b == EarthlyBranch::Si);
        if has_chen && has_si {
            for (branch, pos) in &branches {
                if *branch == EarthlyBranch::Chen || *branch == EarthlyBranch::Si {
                    markers.push(FoundMarker {
                        marker: SpiritMarker::Jimang,
                        position: *pos,
                        is_stem: false,
                    });
                }
            }
        }

        // 길신/흉살 분류 (중복 제거)
        let mut auspicious: Vec<_> = markers
            .iter()
            .filter(|m| m.marker.is_auspicious())
            .map(|m| m.marker)
            .collect();
        auspicious.sort_by_key(|m| m.hangul());
        auspicious.dedup();

        let mut inauspicious: Vec<_> = markers
            .iter()
            .filter(|m| !m.marker.is_auspicious())
            .map(|m| m.marker)
            .collect();
        inauspicious.sort_by_key(|m| m.hangul());
        inauspicious.dedup();

        // === 맥락 분석을 위한 정보 수집 (용신/희신 등) ===
        let yongshin = pillars.yongshin();
        let primary_el = yongshin.primary;
        let assistant_el = yongshin.assistant;

        // === 상세 설명 모델(mapped_markers) 생성 ===
        let mapped_markers = markers.iter()
            .map(|m| {
                let pillar_ganzi = match m.position {
                    PillarPosition::Year => pillars.year,
                    PillarPosition::Month => pillars.month,
                    PillarPosition::Day => pillars.day,
                    PillarPosition::Hour => pillars.hour,
                };
                let element = if m.is_stem { pillar_ganzi.stem.element() } else { pillar_ganzi.branch.element() };

                let is_yong_hee = element == primary_el || element == assistant_el;

                let mut level = if m.marker.is_auspicious() {
                    if is_yong_hee { InterpretationLevel::Auspicious } else { InterpretationLevel::Neutral }
                } else {
                    if is_yong_hee { InterpretationLevel::Neutral } else { InterpretationLevel::Caution }
                };

                let mut reasons = vec![format!("{} {}", m.position.hangul(), if m.is_stem { "천간" } else { "지지" })];
                reasons.push(format!("오행: {}", element.hangul()));

                if is_yong_hee {
                    reasons.push("용/희신 적용".to_string());
                } else {
                    reasons.push("기신/한신 영향".to_string());
                }

                // 특수 조건 추가 (기준점 명시)
                if m.marker == SpiritMarker::Tianyi {
                    reasons.push("일간 기준 최고의 길신".to_string());
                } else if m.marker == SpiritMarker::Kuigang {
                    reasons.push("우두머리 별의 기운".to_string());
                }

                let mut description = m.marker.description().to_string();

                // 상황별 문구 보정
                if m.marker.is_auspicious() && is_yong_hee {
                    description = format!("{} (용신/희신에 해당하여 그 작용력이 더욱 강력하고 순수하게 나타납니다.)", description);
                } else if !m.marker.is_auspicious() && is_yong_hee {
                    description = format!("{} (불리한 살성이나, 용신/희신의 기운 위에 있어 그 흉함이 크게 억제되거나 오히려 추진력으로 승화됩니다.)", description);
                    level = InterpretationLevel::Neutral; // 흉살이라도 용신이면 중립으로 승격
                } else if m.marker.is_auspicious() && !is_yong_hee {
                    description = format!("{} (길한 신살이지만, 기운이 비협조적이라 실제 체감되는 도움은 다소 제한적일 수 있습니다.)", description);
                }

                // 위치별 뉘앙스 추가
                let pos_context = match m.position {
                    PillarPosition::Year => "어린 시절이나 조상의 음덕, 사회적 배경에 영향을 줍니다.",
                    PillarPosition::Month => "직업적 환경이나 부모 형제, 사회 활동의 중심에서 작용합니다.",
                    PillarPosition::Day => "나의 내면적인 기질이나 배우자와의 관계에 깊이 관여합니다.",
                    PillarPosition::Hour => "자식운이나 노년의 삶, 비밀스러운 내실을 상징하는 영역에서 나타납니다.",
                };
                description = format!("{} \n\n* {}", description, pos_context);

                SpiritMarkerDetail {
                    marker: m.marker,
                    position: m.position,
                    is_stem: m.is_stem,
                    level,
                    summary: m.marker.hangul().to_string(),
                    description,
                    reasons,
                }
            })
            .collect();

        // === 보조 기둥(태원/명궁/신궁) 신살 분석 ===
        let mut aux_shinsals = Vec::new();
        let sp = &pillars.supplementary_pillars;
        let aux_pillars = [
            ("태원", sp.taewon),
            ("명궁", sp.myeonggung),
            ("신궁", sp.shingung),
        ];

        // tianyi_branches is already in scope from the beginning of the method

        for (name, aux_gz) in &aux_pillars {
            // 1. 일지 기준 12신살
            let marker_day = TwelveShinsal::calculate(day_branch, aux_gz.branch);
            aux_shinsals.push((
                name.to_string(),
                "일지기준".to_string(),
                marker_day.hangul().to_string(),
            ));

            // 2. 년지 기준 12신살
            let marker_year = TwelveShinsal::calculate(year_branch, aux_gz.branch);
            aux_shinsals.push((
                name.to_string(),
                "년지기준".to_string(),
                marker_year.hangul().to_string(),
            ));

            // 3. 천을귀인 체크
            let cheoneul = Gilsin::cheoneul_branches(day_stem);
            if cheoneul.contains(&aux_gz.branch) {
                aux_shinsals.push((
                    name.to_string(),
                    "일간기준".to_string(),
                    "천을귀인".to_string(),
                ));
            }

            // 4. 원진/귀문 체크 (일지 기준)
            if let Some(w) = EvilSpirit::check_wonjin(day_branch, aux_gz.branch) {
                aux_shinsals.push((
                    name.to_string(),
                    "일지기준".to_string(),
                    w.hangul().to_string(),
                ));
            }
        }

        Self {
            mapped_markers,
            markers,
            auspicious,
            inauspicious,
            aux_shinsals,
        }
    }

    // === 신살 계산 헬퍼 함수들 ===

    fn _get_yima_branch(day_branch: EarthlyBranch) -> EarthlyBranch {
        match day_branch {
            EarthlyBranch::Yin | EarthlyBranch::Wu | EarthlyBranch::Xu => EarthlyBranch::Shen,
            EarthlyBranch::Shen | EarthlyBranch::Zi | EarthlyBranch::Chen => EarthlyBranch::Yin,
            EarthlyBranch::Si | EarthlyBranch::You | EarthlyBranch::Chou => EarthlyBranch::Hai,
            EarthlyBranch::Hai | EarthlyBranch::Mao | EarthlyBranch::Wei => EarthlyBranch::Si,
        }
    }

    fn _get_huagai_branch(day_branch: EarthlyBranch) -> EarthlyBranch {
        match day_branch {
            EarthlyBranch::Yin | EarthlyBranch::Wu | EarthlyBranch::Xu => EarthlyBranch::Xu,
            EarthlyBranch::Shen | EarthlyBranch::Zi | EarthlyBranch::Chen => EarthlyBranch::Chen,
            EarthlyBranch::Si | EarthlyBranch::You | EarthlyBranch::Chou => EarthlyBranch::Chou,
            EarthlyBranch::Hai | EarthlyBranch::Mao | EarthlyBranch::Wei => EarthlyBranch::Wei,
        }
    }

    pub fn is_kuigang(day: GanZi) -> bool {
        matches!(
            (day.stem, day.branch),
            (HeavenlyStem::Geng, EarthlyBranch::Chen)
                | (HeavenlyStem::Geng, EarthlyBranch::Xu)
                | (HeavenlyStem::Ren, EarthlyBranch::Chen)
                | (HeavenlyStem::Wu, EarthlyBranch::Xu)
        )
    }

    fn _get_taohua_branch(day_branch: EarthlyBranch) -> EarthlyBranch {
        match day_branch {
            EarthlyBranch::Yin | EarthlyBranch::Wu | EarthlyBranch::Xu => EarthlyBranch::Mao,
            EarthlyBranch::Shen | EarthlyBranch::Zi | EarthlyBranch::Chen => EarthlyBranch::You,
            EarthlyBranch::Si | EarthlyBranch::You | EarthlyBranch::Chou => EarthlyBranch::Wu,
            EarthlyBranch::Hai | EarthlyBranch::Mao | EarthlyBranch::Wei => EarthlyBranch::Zi,
        }
    }

    fn get_wenchang_branch(day_stem: HeavenlyStem) -> EarthlyBranch {
        match day_stem {
            HeavenlyStem::Jia => EarthlyBranch::Si,
            HeavenlyStem::Yi => EarthlyBranch::Wu,
            HeavenlyStem::Bing | HeavenlyStem::Wu => EarthlyBranch::Shen,
            HeavenlyStem::Ding | HeavenlyStem::Ji => EarthlyBranch::You,
            HeavenlyStem::Geng => EarthlyBranch::Hai,
            HeavenlyStem::Xin => EarthlyBranch::Zi,
            HeavenlyStem::Ren => EarthlyBranch::Yin,
            HeavenlyStem::Gui => EarthlyBranch::Mao,
        }
    }

    fn get_taiji_branches(day_stem: HeavenlyStem) -> Vec<EarthlyBranch> {
        // 표준 태극귀인 대조표 (일간 기준)
        // 甲乙 → 子·午  丙丁 → 卯·酉  戊己 → 丑·未  庚辛 → 寅·亥  壬癸 → 巳·申
        match day_stem {
            HeavenlyStem::Jia | HeavenlyStem::Yi => vec![EarthlyBranch::Zi, EarthlyBranch::Wu],
            HeavenlyStem::Bing | HeavenlyStem::Ding => vec![EarthlyBranch::Mao, EarthlyBranch::You],
            HeavenlyStem::Wu | HeavenlyStem::Ji => vec![EarthlyBranch::Chou, EarthlyBranch::Wei],
            HeavenlyStem::Geng | HeavenlyStem::Xin => vec![EarthlyBranch::Yin, EarthlyBranch::Hai],
            HeavenlyStem::Ren | HeavenlyStem::Gui => vec![EarthlyBranch::Si, EarthlyBranch::Shen],
        }
    }

    fn get_guchen_branch(day_branch: EarthlyBranch) -> EarthlyBranch {
        match day_branch {
            EarthlyBranch::Yin | EarthlyBranch::Mao | EarthlyBranch::Chen => EarthlyBranch::Si,
            EarthlyBranch::Si | EarthlyBranch::Wu | EarthlyBranch::Wei => EarthlyBranch::Shen,
            EarthlyBranch::Shen | EarthlyBranch::You | EarthlyBranch::Xu => EarthlyBranch::Hai,
            EarthlyBranch::Hai | EarthlyBranch::Zi | EarthlyBranch::Chou => EarthlyBranch::Yin,
        }
    }

    fn get_hongyan_branch(day_stem: HeavenlyStem) -> Option<EarthlyBranch> {
        match day_stem {
            HeavenlyStem::Jia => Some(EarthlyBranch::Wu),
            HeavenlyStem::Yi => Some(EarthlyBranch::Shen),
            HeavenlyStem::Bing => Some(EarthlyBranch::Yin),
            HeavenlyStem::Ding => Some(EarthlyBranch::Wei),
            HeavenlyStem::Wu => Some(EarthlyBranch::Chen),
            HeavenlyStem::Ji => Some(EarthlyBranch::Chen),
            HeavenlyStem::Geng => Some(EarthlyBranch::Xu),
            HeavenlyStem::Xin => Some(EarthlyBranch::You),
            HeavenlyStem::Ren => Some(EarthlyBranch::Zi),
            HeavenlyStem::Gui => Some(EarthlyBranch::Shen),
        }
    }

    fn get_tianyi_medical_branch(month_branch: EarthlyBranch) -> EarthlyBranch {
        // 천의성(天醫星): 월지의 바로 이전(前) 지지
        // 예) 월지 亥(9) → 天醫 戌(8), 월지 子(10) → 天醫 亥(9)
        EarthlyBranch::from_index((month_branch.index() as i32 - 1).rem_euclid(12))
    }

    fn get_jinyu_branch(day_stem: HeavenlyStem) -> Option<EarthlyBranch> {
        match day_stem {
            HeavenlyStem::Jia => Some(EarthlyBranch::Chen),
            HeavenlyStem::Yi => Some(EarthlyBranch::Si),
            HeavenlyStem::Bing | HeavenlyStem::Wu => Some(EarthlyBranch::Wei),
            HeavenlyStem::Ding | HeavenlyStem::Ji => Some(EarthlyBranch::Shen),
            HeavenlyStem::Geng => Some(EarthlyBranch::Xu),
            HeavenlyStem::Xin => Some(EarthlyBranch::Hai),
            HeavenlyStem::Ren => Some(EarthlyBranch::Chou),
            HeavenlyStem::Gui => Some(EarthlyBranch::Yin),
        }
    }

    /// 천을귀인 (天乙貴人) - 일간 기준
    fn get_tianyi_branches(day_stem: HeavenlyStem) -> Vec<EarthlyBranch> {
        match day_stem {
            HeavenlyStem::Jia | HeavenlyStem::Wu => vec![EarthlyBranch::Chou, EarthlyBranch::Wei],
            HeavenlyStem::Yi | HeavenlyStem::Ji => vec![EarthlyBranch::Zi, EarthlyBranch::Shen],
            HeavenlyStem::Bing | HeavenlyStem::Ding => vec![EarthlyBranch::Hai, EarthlyBranch::You],
            HeavenlyStem::Geng | HeavenlyStem::Xin => vec![EarthlyBranch::Chou, EarthlyBranch::Wei],
            HeavenlyStem::Ren | HeavenlyStem::Gui => vec![EarthlyBranch::Mao, EarthlyBranch::Si],
        }
    }

    /// 12신살 전체 리스트 계산 - 기준 지지(년/일)의 삼합 그룹 활용
    pub fn get_12_sindal_map(basis: EarthlyBranch) -> Vec<(EarthlyBranch, SpiritMarker)> {
        use EarthlyBranch::*;
        use SpiritMarker::*;

        // 삼합 그룹별 겁살(Jiesha)의 위치
        let jiesha_start = match basis {
            Yin | Wu | Xu => Hai,
            Shen | Zi | Chen => Si,
            Si | You | Chou => Yin,
            Hai | Mao | Wei => Shen,
        };

        let sindal_order = [
            Jiesha, Jaesha, Cheonsha, Jisha, Nyeonsha, Wolsha, Wangshen, Jangseong, Banan, Yima,
            Yukhae, Huagai,
        ];

        let mut results = Vec::new();
        let start_idx = jiesha_start.index() as i32;
        for (i, marker) in sindal_order.iter().enumerate() {
            let branch = EarthlyBranch::from_index(start_idx + i as i32);
            results.push((branch, *marker));
        }
        results
    }

    /// 월덕귀인 (月德貴人) - 월지 기준으로 천간 결정
    fn get_yuede_stem(month_branch: EarthlyBranch) -> HeavenlyStem {
        match month_branch {
            // 寅午戌月 → 丙
            EarthlyBranch::Yin | EarthlyBranch::Wu | EarthlyBranch::Xu => HeavenlyStem::Bing,
            // 申子辰月 → 壬
            EarthlyBranch::Shen | EarthlyBranch::Zi | EarthlyBranch::Chen => HeavenlyStem::Ren,
            // 亥卯未月 → 甲
            EarthlyBranch::Hai | EarthlyBranch::Mao | EarthlyBranch::Wei => HeavenlyStem::Jia,
            // 巳酉丑月 → 庚
            EarthlyBranch::Si | EarthlyBranch::You | EarthlyBranch::Chou => HeavenlyStem::Geng,
        }
    }

    /// 천덕귀인 (天德貴人) - 월지 기준으로 천간 결정
    fn get_tiande_stem(month_branch: EarthlyBranch) -> Option<HeavenlyStem> {
        match month_branch {
            EarthlyBranch::Yin => Some(HeavenlyStem::Ding),
            EarthlyBranch::Mao => Some(HeavenlyStem::Xin),
            EarthlyBranch::Chen => Some(HeavenlyStem::Ren),
            EarthlyBranch::Si => Some(HeavenlyStem::Xin),
            EarthlyBranch::Wu => Some(HeavenlyStem::Jia),
            EarthlyBranch::Wei => Some(HeavenlyStem::Gui),
            EarthlyBranch::Shen => Some(HeavenlyStem::Ren),
            EarthlyBranch::You => Some(HeavenlyStem::Geng),
            EarthlyBranch::Xu => Some(HeavenlyStem::Bing),
            EarthlyBranch::Hai => Some(HeavenlyStem::Yi),
            EarthlyBranch::Zi => Some(HeavenlyStem::Gui),
            EarthlyBranch::Chou => Some(HeavenlyStem::Geng),
        }
    }

    /// 정록 (正祿) - 일간 기준
    fn get_zhenglu_branch(day_stem: HeavenlyStem) -> EarthlyBranch {
        match day_stem {
            HeavenlyStem::Jia => EarthlyBranch::Yin,
            HeavenlyStem::Yi => EarthlyBranch::Mao,
            HeavenlyStem::Bing | HeavenlyStem::Wu => EarthlyBranch::Si,
            HeavenlyStem::Ding | HeavenlyStem::Ji => EarthlyBranch::Wu,
            HeavenlyStem::Geng => EarthlyBranch::Shen,
            HeavenlyStem::Xin => EarthlyBranch::You,
            HeavenlyStem::Ren => EarthlyBranch::Hai,
            HeavenlyStem::Gui => EarthlyBranch::Zi,
        }
    }

    /// 과숙살 (寡宿煞) - 년지 기준
    fn get_guasu_branch(year_branch: EarthlyBranch) -> EarthlyBranch {
        match year_branch {
            EarthlyBranch::Yin | EarthlyBranch::Mao | EarthlyBranch::Chen => EarthlyBranch::Chou,
            EarthlyBranch::Si | EarthlyBranch::Wu | EarthlyBranch::Wei => EarthlyBranch::Chen,
            EarthlyBranch::Shen | EarthlyBranch::You | EarthlyBranch::Xu => EarthlyBranch::Wei,
            EarthlyBranch::Hai | EarthlyBranch::Zi | EarthlyBranch::Chou => EarthlyBranch::Xu,
        }
    }

    /// 현침살 (懸針煞) - 甲, 辛
    fn is_xuanzhen_stem(stem: HeavenlyStem) -> bool {
        matches!(stem, HeavenlyStem::Jia | HeavenlyStem::Xin)
    }

    /// 현침살 (懸針煞) - 卯, 午, 申
    fn is_xuanzhen_branch(branch: EarthlyBranch) -> bool {
        matches!(
            branch,
            EarthlyBranch::Mao | EarthlyBranch::Wu | EarthlyBranch::Shen
        )
    }

    /// 백호살 (白虎煞) - 甲辰, 乙未, 丙戌, 丁丑, 戊辰, 壬戌, 癸丑
    pub fn is_baihu(ganzi: GanZi) -> bool {
        matches!(
            (ganzi.stem, ganzi.branch),
            (HeavenlyStem::Jia, EarthlyBranch::Chen)
                | (HeavenlyStem::Yi, EarthlyBranch::Wei)
                | (HeavenlyStem::Bing, EarthlyBranch::Xu)
                | (HeavenlyStem::Ding, EarthlyBranch::Chou)
                | (HeavenlyStem::Wu, EarthlyBranch::Chen)
                | (HeavenlyStem::Ren, EarthlyBranch::Xu)
                | (HeavenlyStem::Gui, EarthlyBranch::Chou)
        )
    }

    /// 망신살 (亡身煞) - 년지 기준
    fn get_wangshen_branch(year_branch: EarthlyBranch) -> EarthlyBranch {
        match year_branch {
            EarthlyBranch::Yin | EarthlyBranch::Wu | EarthlyBranch::Xu => EarthlyBranch::Si,
            EarthlyBranch::Shen | EarthlyBranch::Zi | EarthlyBranch::Chen => EarthlyBranch::Hai,
            EarthlyBranch::Si | EarthlyBranch::You | EarthlyBranch::Chou => EarthlyBranch::Yin,
            EarthlyBranch::Hai | EarthlyBranch::Mao | EarthlyBranch::Wei => EarthlyBranch::Shen,
        }
    }

    /// 겁살 (劫煞) - 년지 기준
    fn get_jiesha_branch(year_branch: EarthlyBranch) -> EarthlyBranch {
        match year_branch {
            EarthlyBranch::Yin | EarthlyBranch::Wu | EarthlyBranch::Xu => EarthlyBranch::Hai,
            EarthlyBranch::Shen | EarthlyBranch::Zi | EarthlyBranch::Chen => EarthlyBranch::Si,
            EarthlyBranch::Si | EarthlyBranch::You | EarthlyBranch::Chou => EarthlyBranch::Yin,
            EarthlyBranch::Hai | EarthlyBranch::Mao | EarthlyBranch::Wei => EarthlyBranch::Shen,
        }
    }

    /// 암록 (暗祿) - 일간 기준
    fn get_anlu_branch(day_stem: HeavenlyStem) -> Option<EarthlyBranch> {
        match day_stem {
            HeavenlyStem::Jia => Some(EarthlyBranch::Hai),
            HeavenlyStem::Yi => Some(EarthlyBranch::Xu),
            HeavenlyStem::Bing | HeavenlyStem::Wu => Some(EarthlyBranch::Shen),
            HeavenlyStem::Ding | HeavenlyStem::Ji => Some(EarthlyBranch::Wei),
            HeavenlyStem::Geng => Some(EarthlyBranch::Si),
            HeavenlyStem::Xin => Some(EarthlyBranch::Chen),
            HeavenlyStem::Ren => Some(EarthlyBranch::Yin),
            HeavenlyStem::Gui => Some(EarthlyBranch::Chou),
        }
    }

    /// 학당귀인 (學堂貴人) - 일간 기준 (장생지)
    fn get_xuetang_branch(day_stem: HeavenlyStem) -> Option<EarthlyBranch> {
        match day_stem {
            HeavenlyStem::Jia => Some(EarthlyBranch::Hai),
            HeavenlyStem::Yi => Some(EarthlyBranch::Wu),
            HeavenlyStem::Bing | HeavenlyStem::Wu => Some(EarthlyBranch::Yin),
            HeavenlyStem::Ding | HeavenlyStem::Ji => Some(EarthlyBranch::You),
            HeavenlyStem::Geng => Some(EarthlyBranch::Si),
            HeavenlyStem::Xin => Some(EarthlyBranch::Zi),
            HeavenlyStem::Ren => Some(EarthlyBranch::Shen),
            HeavenlyStem::Gui => Some(EarthlyBranch::Mao),
        }
    }

    /// 원진살 (怨嗔煞) - 지지 간 관계 (보통 년지 기준)
    fn is_yuanzhen(b1: EarthlyBranch, b2: EarthlyBranch) -> bool {
        use EarthlyBranch::*;
        matches!(
            (b1, b2),
            (Zi, Wei)
                | (Wei, Zi)
                | (Chou, Wu)
                | (Wu, Chou)
                | (Yin, You)
                | (You, Yin)
                | (Mao, Shen)
                | (Shen, Mao)
                | (Chen, Hai)
                | (Hai, Chen)
                | (Si, Xu)
                | (Xu, Si)
        )
    }

    /// 양인살 (羊刃煞) - 일간 기준 (제왕지/음간은 관대지)
    fn get_yangin_branch(day_stem: HeavenlyStem) -> EarthlyBranch {
        match day_stem {
            HeavenlyStem::Jia => EarthlyBranch::Mao,
            HeavenlyStem::Yi => EarthlyBranch::Chen,
            HeavenlyStem::Bing | HeavenlyStem::Wu => EarthlyBranch::Wu,
            HeavenlyStem::Ding | HeavenlyStem::Ji => EarthlyBranch::Wei,
            HeavenlyStem::Geng => EarthlyBranch::You,
            HeavenlyStem::Xin => EarthlyBranch::Xu,
            HeavenlyStem::Ren => EarthlyBranch::Zi,
            HeavenlyStem::Gui => EarthlyBranch::Chou,
        }
    }
}

impl std::fmt::Display for SpiritMarkerAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "【신살 분석】")?;
        writeln!(f, "─────────────────────────────────")?;

        if !self.auspicious.is_empty() {
            write!(f, "길신: ")?;
            for (i, marker) in self.auspicious.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", marker.hangul())?;
            }
            writeln!(f)?;
        }

        if !self.inauspicious.is_empty() {
            write!(f, "흉살: ")?;
            for (i, marker) in self.inauspicious.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", marker.hangul())?;
            }
            writeln!(f)?;
        }

        writeln!(f, "\n상세:")?;
        for marker in &self.markers {
            writeln!(f, "  • {}", marker)?;
        }

        Ok(())
    }
}

// ============================================
// FourPillars 편의 메서드
// ============================================

impl FourPillars {
    /// 신살 분석
    pub fn spirit_markers(&self) -> SpiritMarkerAnalysis {
        SpiritMarkerAnalysis::from_pillars(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::pillars::SajuInput;

    #[test]
    fn test_user_spirit_markers() {
        // 김성주님 사주: 甲申年 乙亥月 庚戌日 丁亥時
        let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
        let pillars = FourPillars::calculate(&input).unwrap();

        let analysis = pillars.spirit_markers();

        println!("{}", analysis);

        // 괴강살 확인 (庚戌日)
        assert!(analysis
            .markers
            .iter()
            .any(|m| m.marker == SpiritMarker::Kuigang));
    }

    #[test]
    fn test_kuigang() {
        // 庚戌日 = 괴강살
        let ganzi = GanZi {
            stem: HeavenlyStem::Geng,
            branch: EarthlyBranch::Xu,
        };
        assert!(SpiritMarkerAnalysis::is_kuigang(ganzi));

        // 庚辰日 = 괴강살
        let ganzi2 = GanZi {
            stem: HeavenlyStem::Geng,
            branch: EarthlyBranch::Chen,
        };
        assert!(SpiritMarkerAnalysis::is_kuigang(ganzi2));
    }

    #[test]
    fn test_yima() {
        // 戌日 → 申이 역마
        assert_eq!(
            SpiritMarkerAnalysis::_get_yima_branch(EarthlyBranch::Xu),
            EarthlyBranch::Shen
        );
    }

    fn make_mock_pillars(year: GanZi, month: GanZi, day: GanZi, hour: GanZi) -> FourPillars {
        FourPillars {
            year,
            month,
            day,
            hour,
            birth_time: chrono::Utc::now(),
            gender: eon_core::Gender::Male,
            raw_input: SajuInput::new_solar(2000, 1, 1, 12, 0),
            supplementary_pillars: Default::default(),
        }
    }

    #[test]
    fn test_new_spirit_markers() {
        // 양인살 검증: 甲일간 -> 卯
        assert_eq!(
            SpiritMarkerAnalysis::get_yangin_branch(HeavenlyStem::Jia),
            EarthlyBranch::Mao
        );

        // 천라살 검증: 庚戌일 丁亥시 사주 (戌, 亥 존재)
        let pillars = make_mock_pillars(
            GanZi {
                stem: HeavenlyStem::Jia,
                branch: EarthlyBranch::Shen,
            },
            GanZi {
                stem: HeavenlyStem::Yi,
                branch: EarthlyBranch::Hai,
            },
            GanZi {
                stem: HeavenlyStem::Geng,
                branch: EarthlyBranch::Xu,
            },
            GanZi {
                stem: HeavenlyStem::Ding,
                branch: EarthlyBranch::Hai,
            },
        );
        let analysis = SpiritMarkerAnalysis::from_pillars(&pillars);
        assert!(analysis
            .markers
            .iter()
            .any(|m| m.marker == SpiritMarker::Tianluo));

        // 지망살 검증: 辰, 巳 존재 사주
        let pillars_jimang = make_mock_pillars(
            GanZi {
                stem: HeavenlyStem::Jia,
                branch: EarthlyBranch::Chen,
            },
            GanZi {
                stem: HeavenlyStem::Yi,
                branch: EarthlyBranch::Si,
            },
            GanZi {
                stem: HeavenlyStem::Geng,
                branch: EarthlyBranch::Wu,
            },
            GanZi {
                stem: HeavenlyStem::Ding,
                branch: EarthlyBranch::Hai,
            },
        );
        let analysis_jimang = SpiritMarkerAnalysis::from_pillars(&pillars_jimang);
        assert!(analysis_jimang
            .markers
            .iter()
            .any(|m| m.marker == SpiritMarker::Jimang));
    }
}
