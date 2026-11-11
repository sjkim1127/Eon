//! Spirit Markers (神煞) Analysis
//!
//! Analyzes auspicious spirits and inauspicious stars occurring in specific pillar combinations.
//!
//! ##  
//!
//! ###  (吉神)
//! - , , , ,  
//!
//! ###  (凶煞)
//! - , , , ,  

pub use crate::analysis::shinsal::{calculate_samjae, SamjaeStage};
use crate::analysis::shinsal::{EvilSpirit, Gilsin, TwelveShinsal};
use crate::analysis::supplementary_pillars::InterpretationLevel;
use crate::core::branch::EarthlyBranch;
use crate::core::ganzi::GanZi;
use crate::core::pillars::FourPillars;
use crate::core::stem::HeavenlyStem;
use serde::{Deserialize, Serialize};

/// Spirit Marker Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SpiritMarker {
    // ===  (吉神) ===
    /// (天乙貴人) -  
    Tianyi,
    /// (文昌貴人) - ,
    Wenchang,
    /// (太極貴人) -  
    Taiji,
    /// (月德貴人) -  
    Yuede,
    /// (天德貴人) -  
    Tiande,
    /// (正祿) - ,
    Zhenglu,
    /// (金輿祿) -   
    Jinyu,
    /// (暗祿) -  
    Anlu,
    /// (學堂貴人) -  
    Xuetang,
    /// (天醫星) - ,  
    TianyiMedical,
    /// (天文星) - ,
    Tianwen,

    // ===  (凶煞) ===
    /// (驛馬煞) - ,
    Yima,
    /// (華蓋煞) - ,
    Huagai,
    /// (魁罡煞) -  
    Kuigang,
    /// (桃花煞) -  
    Taohua,
    /// (紅艶煞) - ,
    Hongyan,
    /// (孤辰煞) -
    Guchen,
    /// (寡宿煞) -
    Guasu,
    /// (懸針煞) -
    Xuanzhen,
    /// (白虎煞) -
    Baihu,
    /// (亡身煞) -
    Wangshen,
    /// (劫煞) -
    Jiesha,
    /// (怨嗔煞) -
    Yuanzhen,
    /// (羊刃煞) -    
    Yangin,
    /// (天羅煞) -  
    Tianluo,
    /// (地網煞) -  
    Jimang,

    // === 12 () ===
    /// (災煞) - ,
    Jaesha,
    /// (天煞) -  
    Cheonsha,
    /// (地煞) -  ,
    Jisha,
    /// /(年煞) - ,
    Nyeonsha,
    /// (月煞) - ,   
    Wolsha,
    /// (將星) - ,
    Jangseong,
    /// (潘鞍煞) -  ,
    Banan,
    /// (六害煞) -   
    Yukhae,
}

impl SpiritMarker {
    /// English name
    pub const fn hangul(&self) -> &'static str {
        match self {
            Self::Tianyi => "Tianyi Noble",
            Self::Wenchang => "Wenchang Noble",
            Self::Taiji => "Taiji Noble",
            Self::Yuede => "Yuede Noble",
            Self::Tiande => "Tiande Noble",
            Self::Zhenglu => "ZhengLu (True Prosperity)",
            Self::Jinyu => "JinYu (Golden Carriage)",
            Self::Anlu => "AnLu (Hidden Prosperity)",
            Self::Xuetang => "XueTang Noble",
            Self::TianyiMedical => "TianYi (Heavenly Physician)",
            Self::Tianwen => "TianWen (Heavenly Intellect)",
            Self::Yima => "YeokMa (Post Horse)",
            Self::Huagai => "HwaGae (Elegance Cover)",
            Self::Kuigang => "KuiGang (Leader Star)",
            Self::Taohua => "TaoHua (Peach Blossom)",
            Self::Hongyan => "HongYan (Red Beauty)",
            Self::Guchen => "GuChen (Solitary Star)",
            Self::Guasu => "GuaSu (Widowhood Star)",
            Self::Xuanzhen => "XuanZhen (Suspended Needle)",
            Self::Baihu => "BaiHu (White Tiger)",
            Self::Wangshen => "WangShen (Exposure Star)",
            Self::Jiesha => "JieSha (Robbery Star)",
            Self::Yuanzhen => "WonJin (Enmity Star)",
            Self::Yangin => "YangIn (Yang Blade)",
            Self::Tianluo => "TianLuo (Heaven Net)",
            Self::Jimang => "DiWang (Earth Web)",
            Self::Jaesha => "JaeSha (Calamity Star)",
            Self::Cheonsha => "CheonSha (Heaven Calamity)",
            Self::Jisha => "JiSha (Ground Star)",
            Self::Nyeonsha => "NyeonSha (Year Star)",
            Self::Wolsha => "WolSha (Month Star)",
            Self::Jangseong => "JangSeong (General Star)",
            Self::Banan => "BanAn (Saddle Star)",
            Self::Yukhae => "YukHae (Six Harms)",
        }
    }

    /// Hanja name
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

    /// Whether the spirit is auspicious
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

    /// Detailed description of the spirit influence
    pub const fn description(&self) -> &'static str {
        match self {
            Self::Tianyi => "Overcoming difficulties through timely assistance from noble helpers",
            Self::Wenchang => "Scholarly achievement, favorable exams, success in documentation",
            Self::Taiji => "Spiritual protection, innate affinity for philosophy and metaphysics",
            Self::Yuede => "Disaster mitigation and harmonious blessings through seasonal grace",
            Self::Tiande => "Averting misfortune through innate heavenly virtue",
            Self::Zhenglu => "Righteous wealth, professional standing, established salary",
            Self::Jinyu => "Noble lifestyle, prestige, prosperous standard of living",
            Self::Anlu => "Hidden reserves of wealth, unexpected financial support",
            Self::Xuetang => "Exceptional intellectual acumen and academic progression",
            Self::TianyiMedical => {
                "Natural diagnostic insight, therapeutic intuition, healing talents"
            }
            Self::Tianwen => "Deep insight into philosophical systems, arts, and abstract theory",
            Self::Yima => "Frequent mobility, international orientation, dynamic shifts",
            Self::Huagai => "Artistic sensibilities, contemplative solitary disposition",
            Self::Kuigang => "Commanding fortitude, executive leadership",
            Self::Taohua => "Popularity, romantic allure, interpersonal charisma",
            Self::Hongyan => "Magnetic charm, passionate relational intensity",
            Self::Guchen => "Inner solitude, fiercely self-reliant disposition",
            Self::Guasu => "Requires conscious effort and patience in intimate bonds",
            Self::Xuanzhen => "Penetrating analytical sharpness, specialized precision",
            Self::Baihu => "Dynamic volatility, requires bodily mindfulness and care",
            Self::Wangshen => {
                "Public exposure, vulnerability turned into transformative performance"
            }
            Self::Jiesha => "External disruption or fierce competitive pressure",
            Self::Yuanzhen => "Interpersonal friction, misaligned expectations",
            Self::Yangin => "Extreme drive and unyielding resolve, requiring emotional containment",
            Self::Tianluo => {
                "Systemic constraints; thrives in spiritual inquiry, law, and philosophical arts"
            }
            Self::Jimang => {
                "Earthly limits; thrives in overcoming practical barriers through technical mastery"
            }
            Self::Jaesha => "Crisis mitigation, legal prudence, overcoming systemic confinement",
            Self::Cheonsha => "Providential shifts beyond personal control",
            Self::Jisha => "Geographic relocation, dynamic shifts in operational base",
            Self::Nyeonsha => "Public admiration and magnetic stage presence",
            Self::Wolsha => "Persevering in adversity, harvesting resilience from solitude",
            Self::Jangseong => "Zenith of professional command and field leadership",
            Self::Banan => "Honorable standing, executive rank, stable prosperity",
            Self::Yukhae => "Alert obstacle resolution, swift contingency management",
        }
    }
}

impl std::fmt::Display for SpiritMarker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hangul())
    }
}

/// Pillar Position
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
            Self::Year => "Year",
            Self::Month => "Month",
            Self::Day => "Day",
            Self::Hour => "Hour",
        }
    }
}

/// Found Spirit Marker
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoundMarker {
    pub marker: SpiritMarker,
    pub position: PillarPosition,
    pub is_stem: bool, // true=stem, false=branch
}

impl std::fmt::Display for FoundMarker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let part = if self.is_stem { "Stem" } else { "Branch" };
        write!(f, "{} {} - {}", self.position.hangul(), part, self.marker)
    }
}

///    (Explainable Spirit Marker)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SpiritMarkerDetail {
    pub marker: SpiritMarker,
    pub position: PillarPosition,
    pub is_stem: bool,
    pub level: InterpretationLevel,
    pub summary: String,
    pub description: String,
    pub reasons: Vec<String>,
    pub is_clashed: bool,
    pub is_combined: bool,
}

/// Spirit Marker Analysis Result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpiritMarkerAnalysis {
    ///      (   )
    pub mapped_markers: Vec<SpiritMarkerDetail>,
    ///    ( )
    pub markers: Vec<FoundMarker>,
    /// Auspicious markers
    pub auspicious: Vec<SpiritMarker>,
    /// Inauspicious markers
    pub inauspicious: Vec<SpiritMarker>,
    ///     [, , ]
    pub aux_shinsals: Vec<(String, String, String)>,
}

impl SpiritMarkerAnalysis {
    /// Analyze spirit markers from Four Pillars
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

        // ===  (天乙貴人) ===
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

        // === 12 ( ) ===
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

        // === 12 ( ) ===
        let day_sindal = Self::get_12_sindal_map(day_branch);
        for (target_branch, marker) in day_sindal {
            for (curr_branch, pos) in &branches {
                if *curr_branch == target_branch {
                    //       (    )
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

        // ===  (魁罡煞) ===
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

        // ===  (文昌貴人) ===
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

        // ===  (太極貴人) ===
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

        // ===  (月德貴人) ===
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

        // ===  (天德貴人) ===
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

        // ===  (正祿) ===
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

        // ===  (孤辰煞) ===
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

        // ===  (寡宿煞) ===
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

        // ===  (紅艶煞) ===
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

        // ===  (懸針煞) ===
        // 甲, 辛, 卯, 午, 申
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

        // ===  (白虎煞) ===
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

        // ===  (天醫星) ===
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

        // ===  (天文星) ===
        // 亥
        for (branch, pos) in &branches {
            if *branch == EarthlyBranch::Hai {
                markers.push(FoundMarker {
                    marker: SpiritMarker::Tianwen,
                    position: *pos,
                    is_stem: false,
                });
            }
        }

        // ===  (金輿祿) ===
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

        // ===  (亡身煞) ===
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

        // ===  (劫煞) ===
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

        // ===  (暗祿) ===
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

        // ===  (學堂貴) ===
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

        // ===  (怨嗔煞) ===
        for (branch, pos) in &branches {
            if Self::is_yuanzhen(year_branch, *branch) {
                markers.push(FoundMarker {
                    marker: SpiritMarker::Yuanzhen,
                    position: *pos,
                    is_stem: false,
                });
            }
        }

        // ===  (羊刃煞) ===
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

        // ===  (天羅煞) ===
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

        // ===  (地網煞) ===
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

        // /  ( )
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

        // ===      (/ ) ===
        let yongshin = pillars.yongshin();
        let primary_el = yongshin.primary;
        let assistant_el = yongshin.assistant;

        // ()  (  )
        let rel_analysis =
            crate::analysis::relationships::RelationshipAnalysis::from_pillars(pillars);

        // ===   (mapped_markers)  ===
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

                let mut is_clashed = false;
                let mut is_combined = false;

                if !m.is_stem {
                    let pos_str = match m.position {
                        PillarPosition::Year => "year_branch",
                        PillarPosition::Month => "month_branch",
                        PillarPosition::Day => "day_branch",
                        PillarPosition::Hour => "hour_branch",
                    };
                    for (_, p1, p2) in &rel_analysis.branch_clashes {
                        if p1 == pos_str || p2 == pos_str { is_clashed = true; }
                    }
                    for (_, p1, p2) in &rel_analysis.branch_punishments {
                        if p1 == pos_str || p2 == pos_str { is_clashed = true; } // Punishments also trigger clash/activation
                    }
                    for (_, p1, p2) in &rel_analysis.six_combinations {
                        if p1 == pos_str || p2 == pos_str { is_combined = true; }
                    }
                    for (_, p1, p2) in &rel_analysis.dominant_semi_combinations {
                        if p1 == pos_str || p2 == pos_str { is_combined = true; }
                    }

                    let branch = pillar_ganzi.branch;

                    for triple in &rel_analysis.triple_combinations {
                        use crate::core::branch::EarthlyBranch::*;
                        use crate::analysis::relationships::TripleCombination::*;
                        match triple {
                            YinWuXu if branch == Yin || branch == Wu || branch == Xu => is_combined = true,
                            ShenZiChen if branch == Shen || branch == Zi || branch == Chen => is_combined = true,
                            SiYouChou if branch == Si || branch == You || branch == Chou => is_combined = true,
                            HaiMaoWei if branch == Hai || branch == Mao || branch == Wei => is_combined = true,
                            _ => {}
                        }
                    }
                }

                let mut level = if m.marker.is_auspicious() {
                    if is_yong_hee { InterpretationLevel::Auspicious } else { InterpretationLevel::Neutral }
                } else {
                    if is_yong_hee { InterpretationLevel::Neutral } else { InterpretationLevel::Caution }
                };

                let mut reasons = vec![format!("{} {}", m.position.hangul(), if m.is_stem { "Stem" } else { "Branch" })];
                reasons.push(format!("Element: {}", element.hangul()));

                if is_yong_hee {
                    reasons.push("Aligned with Yong/Hee-shin".to_string());
                } else {
                    reasons.push("Influenced by adverse elements".to_string());
                }

                //    ( )
                if m.marker == SpiritMarker::Tianyi {
                    reasons.push("Supreme noble helper based on Day Master".to_string());
                } else if m.marker == SpiritMarker::Kuigang {
                    reasons.push("Commanding leader star energy".to_string());
                }

                let mut description = m.marker.description().to_string();
                let mut summary = m.marker.hangul().to_string();

                let void_analysis = pillars.void_analysis();
                let is_in_void = !m.is_stem && void_analysis.void_branches.contains(&pillar_ganzi.branch);
                let is_void_dissolved = is_in_void && (is_clashed || is_combined);

                //      (//) 
                if is_in_void && !is_void_dissolved && m.marker.is_auspicious() {
                    level = InterpretationLevel::Neutral; // Nullified by noble void
                    summary.push_str(" (Noble Void)");
                    description = format!("{} (The branch falls under void, significantly halving or nullifying the noble star's beneficial efficacy.)", description);
                    reasons.push("Noble Void: Beneficial power nullified by void".to_string());
                } else if is_in_void && is_void_dissolved {
                    summary.push_str(" (Void Rescued)");
                    reasons.push("Void position rescued by clash or combination".to_string());
                    if m.marker.is_auspicious() {
                        if is_clashed {
                            level = InterpretationLevel::Neutral;
                            summary.push_str(" (Defeated)");
                            description = format!("{} (The auspicious star or branch encounters clash or punishment, causing noble defeat and significantly halving its efficacy.)", description);
                            reasons.push("Loss of noble efficacy due to earthly branch clash/punishment".to_string());
                        } else if is_combined {
                            level = InterpretationLevel::Auspicious;
                            summary.push_str(" (Combo Amplified)");
                            description = format!("{} (The branch forms a combination, amplifying and stabilizing the auspicious star's positive energy.)", description);
                            reasons.push("Efficacy enhanced by branch combination".to_string());
                        }
                    }
                } else if m.marker.is_auspicious() {
                    if is_clashed {
                        level = InterpretationLevel::Neutral; // Level downgraded due to broken noble status
                        summary.push_str(" (Defeated)");
                        description = format!("{} (The auspicious star or branch encounters clash or punishment, causing noble defeat and significantly halving its efficacy.)", description);
                        reasons.push("Loss of noble efficacy due to earthly branch clash/punishment".to_string());
                    } else if is_combined {
                        level = InterpretationLevel::Auspicious;
                        summary.push_str(" (Combo Amplified)");
                        description = format!("{} (The branch forms a combination, amplifying and stabilizing the auspicious star's positive energy.)", description);
                        reasons.push("Efficacy enhanced by branch combination".to_string());
                    } else if is_yong_hee {
                        description = format!("{} (Corresponds to Yongshin/Heeshin, manifesting its beneficial influence more powerfully and purely.)", description);
                    } else if !is_yong_hee {
                        description = format!("{} (Although an auspicious star, uncooperative energy makes perceived benefits somewhat limited.)", description);
                    }
                } else {
                    if is_clashed {
                        level = InterpretationLevel::Danger; // Level escalated due to inauspicious star activation
                        summary.push_str(" (Triggered)");
                        description = format!("{} (The inauspicious star encounters clash or punishment, raising concerns of explosive activation requiring caution.)", description);
                        reasons.push("Fierce activation of inauspicious star from branch clash/punishment".to_string());
                    } else if is_combined {
                        level = InterpretationLevel::Neutral; // Level reduced due to star being tied down
                        summary.push_str(" (Tied Down)");
                        description = format!("{} (The inauspicious star or branch is tied down by combination, subduing its fierce energy and bringing stability.)", description);
                        reasons.push("Hostile energy subdued by combination binding".to_string());
                    } else if is_yong_hee {
                        description = format!("{} (Although an adverse star, resting upon Yongshin/Heeshin significantly restrains its negativity or channels it into driving momentum.)", description);
                        level = InterpretationLevel::Neutral;
                    }
                }

                let pos_context = match m.position {
                    PillarPosition::Year => "Influences childhood foundation, ancestral blessings, and broad societal context.",
                    PillarPosition::Month => "Acts at the core of professional environment, parental bonds, and public career.",
                    PillarPosition::Day => "Deeply governs personal temperament and intimate spousal partnership.",
                    PillarPosition::Hour => "Operates across progeny prospects, elder years, and private internal assets.",
                };
                description = format!("{} 

* {}", description, pos_context);

                SpiritMarkerDetail {
                    marker: m.marker,
                    position: m.position,
                    is_stem: m.is_stem,
                    level,
                    summary,
                    description,
                    reasons,
                    is_clashed,
                    is_combined,
                }
            })
            .collect();

        // ===  (//)   ===
        let mut aux_shinsals = Vec::new();
        let sp = &pillars.supplementary_pillars;
        let aux_pillars = [
            ("Taewon", sp.taewon),
            ("Myeonggung", sp.myeonggung),
            ("Singung", sp.shingung),
        ];

        // tianyi_branches is already in scope from the beginning of the method

        for (name, aux_gz) in &aux_pillars {
            // 1.   12
            let marker_day = TwelveShinsal::calculate(day_branch, aux_gz.branch);
            aux_shinsals.push((
                name.to_string(),
                "day_branch_basis".to_string(),
                marker_day.hangul().to_string(),
            ));

            // 2.   12
            let marker_year = TwelveShinsal::calculate(year_branch, aux_gz.branch);
            aux_shinsals.push((
                name.to_string(),
                "year_branch_basis".to_string(),
                marker_year.hangul().to_string(),
            ));

            // 3.
            let cheoneul = Gilsin::cheoneul_branches(day_stem);
            if cheoneul.contains(&aux_gz.branch) {
                aux_shinsals.push((
                    name.to_string(),
                    "day_stem_basis".to_string(),
                    "Tianyi Noble".to_string(),
                ));
            }

            // 4. /  ( )
            if let Some(w) = EvilSpirit::check_wonjin(day_branch, aux_gz.branch) {
                aux_shinsals.push((
                    name.to_string(),
                    "day_branch_basis".to_string(),
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

    // ===     ===

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
        //    ( )
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
        // (天醫星):   (前)
        // )  亥(9) → 天醫 戌(8),  子(10) → 天醫 亥(9)
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

    ///  (天乙貴人) -  
    fn get_tianyi_branches(day_stem: HeavenlyStem) -> Vec<EarthlyBranch> {
        match day_stem {
            HeavenlyStem::Jia | HeavenlyStem::Wu => vec![EarthlyBranch::Chou, EarthlyBranch::Wei],
            HeavenlyStem::Yi | HeavenlyStem::Ji => vec![EarthlyBranch::Zi, EarthlyBranch::Shen],
            HeavenlyStem::Bing | HeavenlyStem::Ding => vec![EarthlyBranch::Hai, EarthlyBranch::You],
            HeavenlyStem::Geng | HeavenlyStem::Xin => vec![EarthlyBranch::Chou, EarthlyBranch::Wei],
            HeavenlyStem::Ren | HeavenlyStem::Gui => vec![EarthlyBranch::Mao, EarthlyBranch::Si],
        }
    }

    /// 12    -  (/)   
    pub fn get_12_sindal_map(basis: EarthlyBranch) -> Vec<(EarthlyBranch, SpiritMarker)> {
        use EarthlyBranch::*;
        use SpiritMarker::*;

        //   (Jiesha)
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

    ///  (月德貴人) -    
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

    ///  (天德貴人) -    
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

    ///  (正祿) -  
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

    ///  (寡宿煞) -  
    fn get_guasu_branch(year_branch: EarthlyBranch) -> EarthlyBranch {
        match year_branch {
            EarthlyBranch::Yin | EarthlyBranch::Mao | EarthlyBranch::Chen => EarthlyBranch::Chou,
            EarthlyBranch::Si | EarthlyBranch::Wu | EarthlyBranch::Wei => EarthlyBranch::Chen,
            EarthlyBranch::Shen | EarthlyBranch::You | EarthlyBranch::Xu => EarthlyBranch::Wei,
            EarthlyBranch::Hai | EarthlyBranch::Zi | EarthlyBranch::Chou => EarthlyBranch::Xu,
        }
    }

    ///  (懸針煞) - 甲, 辛
    fn is_xuanzhen_stem(stem: HeavenlyStem) -> bool {
        matches!(stem, HeavenlyStem::Jia | HeavenlyStem::Xin)
    }

    ///  (懸針煞) - 卯, 午, 申
    fn is_xuanzhen_branch(branch: EarthlyBranch) -> bool {
        matches!(
            branch,
            EarthlyBranch::Mao | EarthlyBranch::Wu | EarthlyBranch::Shen
        )
    }

    ///  (白虎煞) - 甲辰, 乙未, 丙戌, 丁丑, 戊辰, 壬戌, 癸丑
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

    ///  (亡身煞) -  
    fn get_wangshen_branch(year_branch: EarthlyBranch) -> EarthlyBranch {
        match year_branch {
            EarthlyBranch::Yin | EarthlyBranch::Wu | EarthlyBranch::Xu => EarthlyBranch::Si,
            EarthlyBranch::Shen | EarthlyBranch::Zi | EarthlyBranch::Chen => EarthlyBranch::Hai,
            EarthlyBranch::Si | EarthlyBranch::You | EarthlyBranch::Chou => EarthlyBranch::Yin,
            EarthlyBranch::Hai | EarthlyBranch::Mao | EarthlyBranch::Wei => EarthlyBranch::Shen,
        }
    }

    ///  (劫煞) -  
    fn get_jiesha_branch(year_branch: EarthlyBranch) -> EarthlyBranch {
        match year_branch {
            EarthlyBranch::Yin | EarthlyBranch::Wu | EarthlyBranch::Xu => EarthlyBranch::Hai,
            EarthlyBranch::Shen | EarthlyBranch::Zi | EarthlyBranch::Chen => EarthlyBranch::Si,
            EarthlyBranch::Si | EarthlyBranch::You | EarthlyBranch::Chou => EarthlyBranch::Yin,
            EarthlyBranch::Hai | EarthlyBranch::Mao | EarthlyBranch::Wei => EarthlyBranch::Shen,
        }
    }

    ///  (暗祿) -  
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

    ///  (學堂貴人) -   ()
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

    ///  (怨嗔煞) -    (  )
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

    ///  (羊刃煞) -   (/ )
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
        writeln!(f, "【Spirit Markers Analysis】")?;
        writeln!(f, "─────────────────────────────────")?;

        if !self.auspicious.is_empty() {
            write!(f, "Auspicious: ")?;
            for (i, marker) in self.auspicious.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", marker.hangul())?;
            }
            writeln!(f)?;
        }

        if !self.inauspicious.is_empty() {
            write!(f, "Inauspicious: ")?;
            for (i, marker) in self.inauspicious.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", marker.hangul())?;
            }
            writeln!(f)?;
        }

        writeln!(f, "\nDetails:")?;
        for marker in &self.markers {
            writeln!(f, "  • {}", marker)?;
        }

        Ok(())
    }
}

// ============================================
// FourPillars
// ============================================

impl FourPillars {
    /// Analyze spirit markers
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
        //  : 甲申年 乙亥月 庚戌日 丁亥時
        let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
        let pillars = FourPillars::calculate(&input).unwrap();

        let analysis = pillars.spirit_markers();

        println!("{}", analysis);

        //   (庚戌日)
        assert!(analysis
            .markers
            .iter()
            .any(|m| m.marker == SpiritMarker::Kuigang));
    }

    #[test]
    fn test_kuigang() {
        // 庚戌日 =
        let ganzi = GanZi {
            stem: HeavenlyStem::Geng,
            branch: EarthlyBranch::Xu,
        };
        assert!(SpiritMarkerAnalysis::is_kuigang(ganzi));

        // 庚辰日 =
        let ganzi2 = GanZi {
            stem: HeavenlyStem::Geng,
            branch: EarthlyBranch::Chen,
        };
        assert!(SpiritMarkerAnalysis::is_kuigang(ganzi2));
    }

    #[test]
    fn test_yima() {
        // 戌日 → 申
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
        //  : 甲 -> 卯
        assert_eq!(
            SpiritMarkerAnalysis::get_yangin_branch(HeavenlyStem::Jia),
            EarthlyBranch::Mao
        );

        //  : 庚戌 丁亥  (戌, 亥 )
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

        //  : 辰, 巳
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
