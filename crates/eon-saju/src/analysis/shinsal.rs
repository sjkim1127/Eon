//! 신살(神殺, Shinsal) 분석
//!
//! 사주의 길흉화복을 판단하는 보조적 도구인 신살을 분석합니다.
//! 12신살, 원진살, 귀문관살 등을 포함합니다.

use crate::core::branch::EarthlyBranch;
use crate::core::pillars::FourPillars;
use serde::{Deserialize, Serialize};

// ============================================
// 12신살 (Twelve Divine Spirits)
// ============================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TwelveShinsal {
    /// 지살(地殺) - 이동, 변동, 시작 (생지)
    Jisal,
    /// 년살(年殺) - 도화살, 인기, 유흥 (목욕)
    Yeonsal,
    /// 월살(月殺) - 고초, 장애 (관대)
    Wolsal,
    /// 망신살(亡身殺) - 실수, 망신 (건록)
    Mangshinsal,
    /// 장성살(將星殺) - 권위, 주도 (제왕)
    Jangseongsal,
    /// 반안살(攀鞍殺) - 출세, 안락 (쇠)
    Banansal,
    /// 역마살(驛馬殺) - 이동, 분주 (병)
    Yeokmasal,
    /// 육해살(六害殺) - 병고, 액난 (사)
    Yukhaesal,
    /// 화개살(華蓋殺) - 예술, 종교, 고독 (묘)
    Hwagaesal,
    /// 겁살(劫殺) - 강탈, 재해 (절)
    Geopsal,
    /// 재살(災殺) - 수옥살, 감금 (태)
    Jaesal,
    /// 천살(天殺) - 천재지변, 불가항력 (양)
    Cheonsal,
    /// 백호살(白虎殺) - 혈광지사, 급작스러운 사고
    Baekhosal,
    /// 괴강살(魁罡殺) - 강렬한 카리스마, 극단적 흥망
    Goegangsal,
    /// 양인살(羊刃殺) - 강한 고집, 칼을 든 형상
    Yanginsal,
    /// 귀록(歸祿) - 말년의 복록
    Gwirok,
    /// 공망(空亡) - 비어 있음, 실속 없음
    Gongmang,
}

impl TwelveShinsal {
    pub const fn hangul(&self) -> &'static str {
        match self {
            Self::Jisal => "지살",
            Self::Yeonsal => "년살(도화)",
            Self::Wolsal => "월살",
            Self::Mangshinsal => "망신살",
            Self::Jangseongsal => "장성살",
            Self::Banansal => "반안살",
            Self::Yeokmasal => "역마살",
            Self::Yukhaesal => "육해살",
            Self::Hwagaesal => "화개살",
            Self::Geopsal => "겁살",
            Self::Jaesal => "재살",
            Self::Cheonsal => "천살",
            Self::Baekhosal => "백호살",
            Self::Goegangsal => "괴강살",
            Self::Yanginsal => "양인살",
            Self::Gwirok => "귀록",
            Self::Gongmang => "공망",
        }
    }

    pub const fn description(&self) -> &'static str {
        match self {
            Self::Jisal => "스스로 움직여 변화를 꾀함",
            Self::Yeonsal => "인기와 매력, 타인의 시선",
            Self::Wolsal => "고초 속에 얻는 실익",
            Self::Mangshinsal => "치부가 드러나거나 망신",
            Self::Jangseongsal => "권위와 강한 주도권",
            Self::Banansal => "안락한 안장 위에 오름",
            Self::Yeokmasal => "타의나 환경에 의한 이동",
            Self::Yukhaesal => "여섯 가지 해로움, 지체",
            Self::Hwagaesal => "화려함을 덮음, 예술과 종교",
            Self::Geopsal => "빼앗기거나 강한 경쟁",
            Self::Jaesal => "재앙이나 갇히는 기운",
            Self::Cheonsal => "하늘의 뜻, 감당하기 어려운 일",
            Self::Baekhosal => "강한 에너지와 급작스러운 사고 주의",
            Self::Goegangsal => "총명하고 결단력이 강함",
            Self::Yanginsal => "극강한 고집과 추진력",
            Self::Gwirok => "말년의 복록과 안정",
            Self::Gongmang => "비어 있어 채워지지 않는 허망함",
        }
    }

    /// 기준 지지와 대상 지지 간의 12신살 계산
    /// 보통 기준은 일지(Day Branch) 또는 년지(Year Branch)를 사용함.
    pub fn calculate(criteria: EarthlyBranch, target: EarthlyBranch) -> Self {
        use EarthlyBranch::*;

        // 삼합 국(Frame)의 첫 글자(생지) 찾기
        // 인오술(火) -> 인
        // 신자진(水) -> 신
        // 사유축(金) -> 사
        // 해묘미(木) -> 해
        let start_branch = match criteria {
            Yin | Wu | Xu => Yin,
            Shen | Zi | Chen => Shen,
            Si | You | Chou => Si,
            Hai | Mao | Wei => Hai,
        };

        // 생지와의 거리 계산 (순행)
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
// 길신 (Auspicious Spirits)
// ============================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Gilsin {
    /// 문창귀인(文昌貴人) - 학문, 지혜, 예술
    MunchangGwiin,
    /// 천덕귀인(天德貴人) - 하늘의 덕, 흉이 길로 변함
    CheondeokGwiin,
    /// 월덕귀인(月德貴人) - 달의 덕, 재앙을 물리침
    WoldeokGwiin,
    /// 암록(暗祿) - 숨은 복록, 생각지 못한 도움
    Amrok,
    /// 천을귀인(天乙貴人) - 최고의 길신
    CheoneulGwiin,
}

impl Gilsin {
    pub const fn hangul(&self) -> &'static str {
        match self {
            Self::CheoneulGwiin => "천을귀인",
            Self::MunchangGwiin => "문창귀인",
            Self::CheondeokGwiin => "천덕귀인",
            Self::WoldeokGwiin => "월덕귀인",
            Self::Amrok => "암록",
        }
    }

    /// 일간 기준 천을귀인 지지들 반환
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
    /// 원진살(元嗔殺) - 불화, 증오
    Wonjin,
    /// 귀문관살(鬼門關殺) - 예민, 영감
    Gwimun,
    /// 상문살(喪門殺) - 상례, 슬픔
    Sangmunsal,
    /// 조객살(弔客殺) - 문상, 우환
    Jogaeksal,
    /// 고란살(孤鸞殺) - 고독
    Goeunsal,
    /// 과숙살(寡宿殺) - 홀로 됨
    Gwasuksal,
}

impl EvilSpirit {
    pub const fn hangul(&self) -> &'static str {
        match self {
            Self::Wonjin => "원진살",
            Self::Gwimun => "귀문관살",
            Self::Sangmunsal => "상문살",
            Self::Jogaeksal => "조객살",
            Self::Goeunsal => "고란살",
            Self::Gwasuksal => "과숙살",
        }
    }

    pub const fn description(&self) -> &'static str {
        match self {
            Self::Wonjin => "까닭 없이 서로 미워함",
            Self::Gwimun => "신경이 예민하고 직관이 강함",
            Self::Sangmunsal => "상가집에 갈 일이 생기거나 슬픔",
            Self::Jogaeksal => "먼 친척의 우환이나 슬픔",
            Self::Goeunsal => "외로움, 배우자와의 소외",
            Self::Gwasuksal => "여자의 고독, 홀로 됨",
        }
    }

    /// 원진살 확인
    pub fn check_wonjin(b1: EarthlyBranch, b2: EarthlyBranch) -> Option<Self> {
        use EarthlyBranch::*;
        match (b1, b2) {
            (Zi, Wei) | (Wei, Zi) => Some(Self::Wonjin), // 자미 원진
            (Chou, Wu) | (Wu, Chou) => Some(Self::Wonjin), // 축오 원진
            (Yin, You) | (You, Yin) => Some(Self::Wonjin), // 인유 원진
            (Mao, Shen) | (Shen, Mao) => Some(Self::Wonjin), // 묘신 원진
            (Chen, Hai) | (Hai, Chen) => Some(Self::Wonjin), // 진해 원진
            (Si, Xu) | (Xu, Si) => Some(Self::Wonjin),   // 사술 원진
            _ => None,
        }
    }

    /// 귀문관살 확인
    pub fn check_gwimun(b1: EarthlyBranch, b2: EarthlyBranch) -> Option<Self> {
        use EarthlyBranch::*;
        match (b1, b2) {
            (Zi, You) | (You, Zi) => Some(Self::Gwimun), // 자유 귀문
            (Chou, Wu) | (Wu, Chou) => Some(Self::Gwimun), // 축오 귀문
            (Yin, Wei) | (Wei, Yin) => Some(Self::Gwimun), // 인미 귀문
            (Mao, Shen) | (Shen, Mao) => Some(Self::Gwimun), // 묘신 귀문
            (Chen, Hai) | (Hai, Chen) => Some(Self::Gwimun), // 진해 귀문
            (Si, Xu) | (Xu, Si) => Some(Self::Gwimun),   // 사술 귀문
            _ => None,
        }
    }
}

// ============================================
// 신살 분석 결과
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShinsalAnalysis {
    /// 12신살 (일지 기준)
    pub twelve_shinsal_day: Vec<(String, TwelveShinsal)>,
    /// 12신살 (년지 기준)
    pub twelve_shinsal_year: Vec<(String, TwelveShinsal)>,
    /// 원진/귀문 등 특수 신살
    pub special_shinsals: Vec<(String, String, EvilSpirit)>,
    /// 길신 (천을귀인 등)
    pub gilsin: Vec<(String, Gilsin)>,
}

impl ShinsalAnalysis {
    pub fn from_pillars(pillars: &FourPillars) -> Self {
        let day_stem = pillars.day_master();
        let branches = [
            ("년지", pillars.year.branch),
            ("월지", pillars.month.branch),
            ("일지", pillars.day.branch),
            ("시지", pillars.hour.branch),
        ];

        // 1. 12신살 (일지 기준)
        let mut twelve_shinsal_day = Vec::new();
        for (name, branch) in &branches {
            // 일지는 자기 자신이므로 제외하거나 장성살이나 지살이 됨. 포함시킴.
            let shinsal = TwelveShinsal::calculate(pillars.day.branch, *branch);
            twelve_shinsal_day.push((name.to_string(), shinsal));
        }

        // 2. 12신살 (년지 기준)
        let mut twelve_shinsal_year = Vec::new();
        for (name, branch) in &branches {
            let shinsal = TwelveShinsal::calculate(pillars.year.branch, *branch);
            twelve_shinsal_year.push((name.to_string(), shinsal));
        }

        // 3. 원진/귀문 등 흉살 (모든 지지 조합 체크)
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

        // 4. 길신 (천을귀인 등)
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

    /// 특정 운(대운/세운 등)에 대한 신살 분석
    pub fn calculate_for_luck(
        luck_ganzi: crate::core::ganzi::GanZi,
        pillars: &FourPillars,
    ) -> Vec<String> {
        let mut results = Vec::new();
        let day_master = pillars.day_master();

        // 1. 12신살 (일지 기준)
        let s_day = TwelveShinsal::calculate(pillars.day.branch, luck_ganzi.branch);
        results.push(format!("(일) {}", s_day.hangul()));

        // 2. 12신살 (년지 기준)
        let s_year = TwelveShinsal::calculate(pillars.year.branch, luck_ganzi.branch);
        results.push(format!("(년) {}", s_year.hangul()));

        // 3. 천을귀인 체크
        let cheoneul = Gilsin::cheoneul_branches(day_master);
        if cheoneul.contains(&luck_ganzi.branch) {
            results.push("천을귀인".to_string());
        }

        // 4. 백호/괴강 등 특수 지지
        use crate::core::branch::EarthlyBranch as B;
        use crate::core::stem::HeavenlyStem as S;

        // 백호살 (Luck Ganzi 자체가 백호인 경우)
        match (luck_ganzi.stem, luck_ganzi.branch) {
            (S::Jia, B::Chen)
            | (S::Yi, B::Wei)
            | (S::Bing, B::Xu)
            | (S::Ding, B::Chou)
            | (S::Wu, B::Chen)
            | (S::Ren, B::Xu)
            | (S::Gui, B::Chou) => {
                results.push("백호살".to_string());
            }
            _ => {}
        }

        // 괴강살
        match (luck_ganzi.stem, luck_ganzi.branch) {
            (S::Wu, B::Xu)
            | (S::Wu, B::Chen)
            | (S::Geng, B::Xu)
            | (S::Geng, B::Chen)
            | (S::Ren, B::Xu)
            | (S::Ren, B::Chen) => {
                results.push("괴강살".to_string());
            }
            _ => {}
        }

        results
    }
}

impl std::fmt::Display for ShinsalAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "【신살(神殺) 분석】")?;
        writeln!(f, "─────────────────────────────────")?;

        writeln!(f, "[12신살 - 일지 기준]")?;
        for (pos, shinsal) in &self.twelve_shinsal_day {
            write!(f, "  {}: {} ", pos, shinsal.hangul())?;
        }
        writeln!(f)?;

        writeln!(f, "\n[12신살 - 년지 기준]")?;
        for (pos, shinsal) in &self.twelve_shinsal_year {
            write!(f, "  {}: {} ", pos, shinsal.hangul())?;
        }
        writeln!(f)?;

        if !self.special_shinsals.is_empty() {
            writeln!(f, "\n[특수 신살 (원진/귀문)]")?;
            for (p1, p2, spirit) in &self.special_shinsals {
                writeln!(f, "  {} - {}: {}", p1, p2, spirit.hangul())?;
            }
        }

        if !self.gilsin.is_empty() {
            writeln!(f, "\n[길신 (Auspicious Spirits)]")?;
            for (pos, spirit) in &self.gilsin {
                writeln!(f, "  {}: {}", pos, spirit.hangul())?;
            }
        }

        Ok(())
    }
}

impl FourPillars {
    /// 신살 분석
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
        // 자(Zi) - 미(Wei) 원진
        assert_eq!(
            EvilSpirit::check_wonjin(EarthlyBranch::Zi, EarthlyBranch::Wei),
            Some(EvilSpirit::Wonjin)
        );
        // 축(Chou) - 오(Wu) 귀문
        assert_eq!(
            EvilSpirit::check_gwimun(EarthlyBranch::Chou, EarthlyBranch::Wu),
            Some(EvilSpirit::Gwimun)
        );
    }

    #[test]
    fn test_twelve_shinsal() {
        // 인오술(火) 생지(寅)
        // 기준: 인(寅), 대상: 오(Wu) -> 장성살
        let s = TwelveShinsal::calculate(EarthlyBranch::Yin, EarthlyBranch::Wu);
        assert_eq!(s, TwelveShinsal::Jangseongsal);

        // 대상: 신(Shen) -> 역마살
        let s2 = TwelveShinsal::calculate(EarthlyBranch::Yin, EarthlyBranch::Shen);
        assert_eq!(s2, TwelveShinsal::Yeokmasal);
    }

    #[test]
    fn test_user_shinsal() {
        // 김성주: 갑신년 을해월 경인일 정해시
        let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
        let pillars = FourPillars::calculate(&input).unwrap();

        // 일지(인) 기준
        // 인오술 화국 -> 생지 인(Jisal)
        // 년지(신): 인신충 -> 역마(Yeokmasal) (인 기준 신은 6칸 차이)
        // 월지(해): 인해합 -> 겁살(Geopsal)?
        // 계산: 인(0) -> 해(9) -> 겁살

        let analysis = pillars.shinsal();
        println!("{}", analysis);

        // 각 위치별 신살 확인
        // 년지(신): 인(0) ~ 신(6) -> 역마살
        let year_shinsal = analysis
            .twelve_shinsal_day
            .iter()
            .find(|(p, _)| p == "년지")
            .unwrap();
        assert_eq!(year_shinsal.1, TwelveShinsal::Yeokmasal);

        // 월지(해): 인(0) ~ 해(9) -> 겁살
        let month_shinsal = analysis
            .twelve_shinsal_day
            .iter()
            .find(|(p, _)| p == "월지")
            .unwrap();
        assert_eq!(month_shinsal.1, TwelveShinsal::Geopsal);
    }
}
