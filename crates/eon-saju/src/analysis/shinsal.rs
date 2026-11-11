//! Shinsal (神殺, Symbolic Stars) Analysis
//!
//! Analyzes symbolic stars used as complementary indicators for fortune and timing.
//! Includes 12 Shinsal, Wonjin, Gwimun, etc.

use crate::core::branch::EarthlyBranch;
use crate::core::pillars::FourPillars;
use serde::{Deserialize, Serialize};

// ============================================
// 12 (Twelve Divine Spirits)
// ============================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TwelveShinsal {
    /// (地殺) - , ,  ()
    Jisal,
    /// (年殺) - , ,  ()
    Yeonsal,
    /// (月殺) - ,  ()
    Wolsal,
    /// (亡身殺) - ,  ()
    Mangshinsal,
    /// (將星殺) - ,  ()
    Jangseongsal,
    /// (攀鞍殺) - ,  ()
    Banansal,
    /// (驛馬殺) - ,  ()
    Yeokmasal,
    /// (六害殺) - ,  ()
    Yukhaesal,
    /// (華蓋殺) - , ,  ()
    Hwagaesal,
    /// (劫殺) - ,  ()
    Geopsal,
    /// (災殺) - ,  ()
    Jaesal,
    /// (天殺) - ,  ()
    Cheonsal,
    /// (白虎殺) - ,  
    Baekhosal,
    /// (魁罡殺) -  ,  
    Goegangsal,
    /// (羊刃殺) -  ,   
    Yanginsal,
    /// (歸祿) -  
    Gwirok,
    /// (空亡) -  ,  
    Gongmang,
}

impl TwelveShinsal {
    pub const fn hangul(&self) -> &'static str {
        match self {
            Self::Jisal => "JiSal (Ground Star / Mobility)",
            Self::Yeonsal => "YeonSal (Peach Blossom / Charm)",
            Self::Wolsal => "WolSal (Month Star / Hindrance)",
            Self::Mangshinsal => "MangShinSal (Exposure / Loss of Face)",
            Self::Jangseongsal => "JangSeongSal (General Star / Command)",
            Self::Banansal => "BanAnSal (Saddle Star / Comfort)",
            Self::Yeokmasal => "YeokMaSal (Post Horse / Movement)",
            Self::Yukhaesal => "YukHaeSal (Six Harms / Obstacle)",
            Self::Hwagaesal => "HwaGaeSal (Elegance Cover / Arts & Solitude)",
            Self::Geopsal => "GeopSal (Robbery / Fierce Competition)",
            Self::Jaesal => "JaeSal (Calamity Star / Confinement)",
            Self::Cheonsal => "CheonSal (Heaven Calamity / Force Majeure)",
            Self::Baekhosal => "BaekHoSal (White Tiger / Sudden Shock)",
            Self::Goegangsal => "KuiGangSal (Leader Star / Charisma)",
            Self::Yanginsal => "YangInSal (Yang Blade / Drive)",
            Self::Gwirok => "GwiRok (Returning Prosperity)",
            Self::Gongmang => "GongMang (Void / Emptiness)",
        }
    }

    pub const fn description(&self) -> &'static str {
        match self {
            Self::Jisal => "Self-initiated movement seeking proactive transformation",
            Self::Yeonsal => "Popularity, magnetic charisma, and public attraction",
            Self::Wolsal => "Hard-won gains earned through persevering hardships",
            Self::Mangshinsal => "Public exposure, embarrassment, or vulnerability",
            Self::Jangseongsal => "Authority, command, and strong independent leadership",
            Self::Banansal => "Mounting a comfortable saddle; career promotion and ease",
            Self::Yeokmasal => "Dynamic movement driven by circumstances or travel",
            Self::Yukhaesal => "Six potential impediments, delays, or lingering bottlenecks",
            Self::Hwagaesal => "Covering splendor; spiritual, philosophical, and artistic depth",
            Self::Geopsal => "Intense rivalry, aggressive competition, or resource seizure",
            Self::Jaesal => "Calamity, imprisonment, or legal and systemic constraints",
            Self::Cheonsal => "Providential decree, unexpected force majeure",
            Self::Baekhosal => "High explosive energy; caution regarding sudden accidents",
            Self::Goegangsal => "Brilliant intellect and uncompromising decisive will",
            Self::Yanginsal => "Extreme drive, fierce determination, and bold execution",
            Self::Gwirok => "Late-life prosperity, fulfillment, and lasting stability",
            Self::Gongmang => "Emptiness, ungrounded potential, or delayed fulfillment",
        }
    }

    ///      12
    ///   (Day Branch)  (Year Branch) .
    pub fn calculate(criteria: EarthlyBranch, target: EarthlyBranch) -> Self {
        use EarthlyBranch::*;

        //  (Frame)  ()
        // (火) ->
        // (水) ->
        // (金) ->
        // (木) ->
        let start_branch = match criteria {
            Yin | Wu | Xu => Yin,
            Shen | Zi | Chen => Shen,
            Si | You | Chou => Si,
            Hai | Mao | Wei => Hai,
        };

        //    ()
        let diff = (target.index() as i32 - start_branch.index() as i32).rem_euclid(12);

        match diff {
            0 => Self::Jisal,
            1 => Self::Yeonsal,
            2 => Self::Wolsal,
            3 => Self::Mangshinsal,
            4 => Self::Jangseongsal,
            5 => Self::Banansal,
            6 => Self::Yeokmasal,
            7 => Self::Yukhaesal,
            8 => Self::Hwagaesal,
            9 => Self::Geopsal,
            10 => Self::Jaesal,
            11 => Self::Cheonsal,
            _ => unreachable!(),
        }
    }
}

// ============================================
//  (Auspicious Spirits)
// ============================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Gilsin {
    /// (文昌貴人) - , ,
    MunchangGwiin,
    /// (天德貴人) -  ,   
    CheondeokGwiin,
    /// (月德貴人) -  ,  
    WoldeokGwiin,
    /// (暗祿) -  ,   
    Amrok,
    /// (天乙貴人) -  
    CheoneulGwiin,
}

impl Gilsin {
    pub const fn hangul(&self) -> &'static str {
        match self {
            Self::CheoneulGwiin => "Tianyi Noble",
            Self::MunchangGwiin => "Munchang Noble",
            Self::CheondeokGwiin => "Cheondeok Noble",
            Self::WoldeokGwiin => "Woldeok Noble",
            Self::Amrok => "Amrok (Hidden Prosperity)",
        }
    }

    /// Returns Tianyi Noble branches for a given Day Master.
    pub fn cheoneul_branches(day_stem: crate::core::stem::HeavenlyStem) -> Vec<EarthlyBranch> {
        use crate::core::stem::HeavenlyStem as S;
        use EarthlyBranch as B;
        match day_stem {
            S::Jia | S::Wu | S::Geng => vec![B::Chou, B::Wei],
            S::Yi | S::Ji => vec![B::Zi, B::Shen],
            S::Bing | S::Ding => vec![B::Hai, B::You],
            S::Xin => vec![B::Yin, B::Wu],
            S::Ren | S::Gui => vec![B::Si, B::Mao],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EvilSpirit {
    /// (元嗔殺) - ,
    Wonjin,
    /// (鬼門關殺) - ,
    Gwimun,
    /// (喪門殺) - ,
    Sangmunsal,
    /// (弔客殺) - ,
    Jogaeksal,
    /// (孤鸞殺) -
    Goeunsal,
    /// (寡宿殺) -  
    Gwasuksal,
}

impl EvilSpirit {
    pub const fn hangul(&self) -> &'static str {
        match self {
            Self::Wonjin => "Wonjin (Enmity / Friction)",
            Self::Gwimun => "Gwimun (Ghost Gate / Acute Sensitivity)",
            Self::Sangmunsal => "SangmunSal (Funeral Gate)",
            Self::Jogaeksal => "JoGaekSal (Condolence Guest)",
            Self::Goeunsal => "GoranSal (Solitary Bird)",
            Self::Gwasuksal => "GwasukSal (Widowhood / Solitude)",
        }
    }

    pub const fn description(&self) -> &'static str {
        match self {
            Self::Wonjin => "Mutual emotional friction and irrational enmity",
            Self::Gwimun => "Hypersensitive intuition, acute discernment, psychological depth",
            Self::Sangmunsal => "Occasion of mourning or solemn reflection",
            Self::Jogaeksal => "Distress or loss concerning distant acquaintances",
            Self::Goeunsal => "Inner loneliness or emotional detachment in intimate partnership",
            Self::Gwasuksal => "Solitude, independence, standing alone",
        }
    }

    /// Checks for Wonjin (Animode / Bitter Friction).
    pub fn check_wonjin(b1: EarthlyBranch, b2: EarthlyBranch) -> Option<Self> {
        use EarthlyBranch::*;
        match (b1, b2) {
            (Zi, Wei) | (Wei, Zi) => Some(Self::Wonjin), // Zi-Wei Wonjin
            (Chou, Wu) | (Wu, Chou) => Some(Self::Wonjin), // Chou-Wu Wonjin
            (Yin, You) | (You, Yin) => Some(Self::Wonjin), // Yin-You Wonjin
            (Mao, Shen) | (Shen, Mao) => Some(Self::Wonjin), // Mao-Shen Wonjin
            (Chen, Hai) | (Hai, Chen) => Some(Self::Wonjin), // Chen-Hai Wonjin
            (Si, Xu) | (Xu, Si) => Some(Self::Wonjin),   // Si-Xu Wonjin
            _ => None,
        }
    }

    /// Checks for Gwimun (Ghost Gate / Intuition Spike).
    pub fn check_gwimun(b1: EarthlyBranch, b2: EarthlyBranch) -> Option<Self> {
        use EarthlyBranch::*;
        match (b1, b2) {
            (Zi, You) | (You, Zi) => Some(Self::Gwimun), // Zi-You Gwimun
            (Chou, Wu) | (Wu, Chou) => Some(Self::Gwimun), // Chou-Wu Gwimun
            (Yin, Wei) | (Wei, Yin) => Some(Self::Gwimun), // Yin-Wei Gwimun
            (Mao, Shen) | (Shen, Mao) => Some(Self::Gwimun), // Mao-Shen Gwimun
            (Chen, Hai) | (Hai, Chen) => Some(Self::Gwimun), // Chen-Hai Gwimun
            (Si, Xu) | (Xu, Si) => Some(Self::Gwimun),   // Si-Xu Gwimun
            _ => None,
        }
    }
}

// ============================================
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShinsalAnalysis {
    /// 12 ( )
    pub twelve_shinsal_day: Vec<(String, TwelveShinsal)>,
    /// 12 ( )
    pub twelve_shinsal_year: Vec<(String, TwelveShinsal)>,
    /// /   
    pub special_shinsals: Vec<(String, String, EvilSpirit)>,
    ///  ( )
    pub gilsin: Vec<(String, Gilsin)>,
}

impl ShinsalAnalysis {
    pub fn from_pillars(pillars: &FourPillars) -> Self {
        let day_stem = pillars.day_master();
        let branches = [
            ("year_branch", pillars.year.branch),
            ("month_branch", pillars.month.branch),
            ("day_branch", pillars.day.branch),
            ("hour_branch", pillars.hour.branch),
        ];

        // 1. 12 ( )
        let mut twelve_shinsal_day = Vec::new();
        for (name, branch) in &branches {
            //       . .
            let shinsal = TwelveShinsal::calculate(pillars.day.branch, *branch);
            twelve_shinsal_day.push((name.to_string(), shinsal));
        }

        // 2. 12 ( )
        let mut twelve_shinsal_year = Vec::new();
        for (name, branch) in &branches {
            let shinsal = TwelveShinsal::calculate(pillars.year.branch, *branch);
            twelve_shinsal_year.push((name.to_string(), shinsal));
        }

        // 3. /   (   )
        let mut special_shinsals = Vec::new();
        for i in 0..branches.len() {
            for j in (i + 1)..branches.len() {
                let (n1, b1) = branches[i];
                let (n2, b2) = branches[j];

                if let Some(w) = EvilSpirit::check_wonjin(b1, b2) {
                    special_shinsals.push((n1.to_string(), n2.to_string(), w));
                }
                if let Some(g) = EvilSpirit::check_gwimun(b1, b2) {
                    special_shinsals.push((n1.to_string(), n2.to_string(), g));
                }
            }
        }

        // 4.  ( )
        let mut gilsin = Vec::new();
        let cheoneul = Gilsin::cheoneul_branches(day_stem);
        for (name, b) in &branches {
            if cheoneul.contains(b) {
                gilsin.push((name.to_string(), Gilsin::CheoneulGwiin));
            }
        }

        Self {
            twelve_shinsal_day,
            twelve_shinsal_year,
            special_shinsals,
            gilsin,
        }
    }

    ///  (/ )   
    pub fn calculate_for_luck(
        luck_ganzi: crate::core::ganzi::GanZi,
        pillars: &FourPillars,
    ) -> Vec<String> {
        let mut results = Vec::new();
        let day_master = pillars.day_master();

        // 1. 12 ( )
        let s_day = TwelveShinsal::calculate(pillars.day.branch, luck_ganzi.branch);
        results.push(format!("(Day) {}", s_day.hangul()));

        // 2. 12 ( )
        let s_year = TwelveShinsal::calculate(pillars.year.branch, luck_ganzi.branch);
        results.push(format!("(Year) {}", s_year.hangul()));

        // 3.
        let cheoneul = Gilsin::cheoneul_branches(day_master);
        if cheoneul.contains(&luck_ganzi.branch) {
            results.push("Tianyi Noble".to_string());
        }

        // 4. /
        use crate::core::branch::EarthlyBranch as B;
        use crate::core::stem::HeavenlyStem as S;

        //  (Luck Ganzi   )
        match (luck_ganzi.stem, luck_ganzi.branch) {
            (S::Jia, B::Chen)
            | (S::Yi, B::Wei)
            | (S::Bing, B::Xu)
            | (S::Ding, B::Chou)
            | (S::Wu, B::Chen)
            | (S::Ren, B::Xu)
            | (S::Gui, B::Chou) => {
                results.push("BaekHoSal".to_string());
            }
            _ => {}
        }

        match (luck_ganzi.stem, luck_ganzi.branch) {
            (S::Wu, B::Xu)
            | (S::Wu, B::Chen)
            | (S::Geng, B::Xu)
            | (S::Geng, B::Chen)
            | (S::Ren, B::Xu)
            | (S::Ren, B::Chen) => {
                results.push("KuiGangSal".to_string());
            }
            _ => {}
        }

        results
    }
}

impl std::fmt::Display for ShinsalAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "【Shinsal (神殺) Analysis】")?;
        writeln!(f, "─────────────────────────────────")?;

        writeln!(f, "[12 Shinsal - Day Branch Basis]")?;
        for (pos, shinsal) in &self.twelve_shinsal_day {
            write!(f, "  {}: {} ", pos, shinsal.hangul())?;
        }
        writeln!(f)?;

        writeln!(f, "\n[12 Shinsal - Year Branch Basis]")?;
        for (pos, shinsal) in &self.twelve_shinsal_year {
            write!(f, "  {}: {} ", pos, shinsal.hangul())?;
        }
        writeln!(f)?;

        if !self.special_shinsals.is_empty() {
            writeln!(f, "\n[Special Spirits (Wonjin / Gwimun)]")?;
            for (p1, p2, spirit) in &self.special_shinsals {
                writeln!(f, "  {} - {}: {}", p1, p2, spirit.hangul())?;
            }
        }

        if !self.gilsin.is_empty() {
            writeln!(f, "\n[Auspicious Spirits (Gilsin)]")?;
            for (pos, spirit) in &self.gilsin {
                writeln!(f, "  {}: {}", pos, spirit.hangul())?;
            }
        }

        Ok(())
    }
}

///   (, , )
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SamjaeStage {
    ///  ( -  )
    Entrance,
    ///  ( -  )
    Dwelling,
    ///  ( -  )
    Exit,
}

impl SamjaeStage {
    pub const fn hangul(&self) -> &'static str {
        match self {
            Self::Entrance => "Entrance Samjae (Year 1)",
            Self::Dwelling => "Dwelling Samjae (Year 2)",
            Self::Exit => "Exit Samjae (Year 3)",
        }
    }

    pub const fn description(&self) -> &'static str {
        match self {
            Self::Entrance => {
                "First year of Samjae; exercise prudence during initial shifts and changes."
            }
            Self::Dwelling => {
                "Second year of Samjae; avoid rash decisions in career or residential transitions."
            }
            Self::Exit => "Final year of Samjae; concludes transition cycle with renewed clarity.",
        }
    }
}

///     3(三災)   
pub fn calculate_samjae(
    year_branch: EarthlyBranch,
    transit_year_branch: EarthlyBranch,
) -> Option<SamjaeStage> {
    use EarthlyBranch::*;
    match year_branch {
        Shen | Zi | Chen => match transit_year_branch {
            Yin => Some(SamjaeStage::Entrance),
            Mao => Some(SamjaeStage::Dwelling),
            Chen => Some(SamjaeStage::Exit),
            _ => None,
        },
        Yin | Wu | Xu => match transit_year_branch {
            Shen => Some(SamjaeStage::Entrance),
            You => Some(SamjaeStage::Dwelling),
            Xu => Some(SamjaeStage::Exit),
            _ => None,
        },
        Si | You | Chou => match transit_year_branch {
            Hai => Some(SamjaeStage::Entrance),
            Zi => Some(SamjaeStage::Dwelling),
            Chou => Some(SamjaeStage::Exit),
            _ => None,
        },
        Hai | Mao | Wei => match transit_year_branch {
            Si => Some(SamjaeStage::Entrance),
            Wu => Some(SamjaeStage::Dwelling),
            Wei => Some(SamjaeStage::Exit),
            _ => None,
        },
    }
}

impl FourPillars {
    /// Shinsal Analysis
    pub fn shinsal(&self) -> ShinsalAnalysis {
        ShinsalAnalysis::from_pillars(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::pillars::SajuInput;

    #[test]
    fn test_wonjin_gwimun() {
        // (Zi) - (Wei)
        assert_eq!(
            EvilSpirit::check_wonjin(EarthlyBranch::Zi, EarthlyBranch::Wei),
            Some(EvilSpirit::Wonjin)
        );
        // (Chou) - (Wu)
        assert_eq!(
            EvilSpirit::check_gwimun(EarthlyBranch::Chou, EarthlyBranch::Wu),
            Some(EvilSpirit::Gwimun)
        );
    }

    #[test]
    fn test_twelve_shinsal() {
        // (火) (寅)
        // : (寅), : (Wu) ->
        let s = TwelveShinsal::calculate(EarthlyBranch::Yin, EarthlyBranch::Wu);
        assert_eq!(s, TwelveShinsal::Jangseongsal);

        // : (Shen) ->
        let s2 = TwelveShinsal::calculate(EarthlyBranch::Yin, EarthlyBranch::Shen);
        assert_eq!(s2, TwelveShinsal::Yeokmasal);
    }

    #[test]
    fn test_user_shinsal() {
        // :
        let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
        let pillars = FourPillars::calculate(&input).unwrap();

        // ()
        //   ->  (Jisal)
        // ():  -> (Yeokmasal) (   6 )
        // ():  -> (Geopsal)?
        // : (0) -> (9) ->

        let analysis = pillars.shinsal();
        println!("{}", analysis);

        // (): (0) ~ (6) ->
        let year_shinsal = analysis
            .twelve_shinsal_day
            .iter()
            .find(|(p, _)| p == "year_branch")
            .unwrap();
        assert_eq!(year_shinsal.1, TwelveShinsal::Yeokmasal);

        // (): (0) ~ (9) ->
        let month_shinsal = analysis
            .twelve_shinsal_day
            .iter()
            .find(|(p, _)| p == "month_branch")
            .unwrap();
        assert_eq!(month_shinsal.1, TwelveShinsal::Geopsal);
    }
}
