use serde::{Deserialize, Serialize};
use crate::core::ganzi::GanZi;
use crate::core::pillars::FourPillars;
use crate::core::stem::HeavenlyStem;
use crate::core::branch::EarthlyBranch;

/// 태원·명궁·신궁 등 보조 기둥 분석 결과
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SupplementaryPillars {
    /// 태원 (胎元): 잉태 시점의 기운
    pub taewon: GanZi,
    /// 명궁 (命宮): 정신적 지향점과 운의 바탕
    pub myeonggung: GanZi,
    /// 신궁 (身宮): 후천적 신체 환경과 실제적 삶의 태도
    pub shingung: GanZi,
    /// 메타 정보
    pub meta: SupplementaryPillarsMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SupplementaryPillarsMeta {
    /// 계산 공식 버전
    pub formula_version: String,
    /// 공식 명칭 (학파 구분용)
    pub formula_name: String,
    /// 시간에 민감한지 여부 (명궁/신궁은 시간 필수)
    pub hour_sensitive: bool,
}

impl SupplementaryPillars {
    /// 기준 FourPillars로부터 보조 기둥 계산
    pub fn calculate(pillars: &FourPillars) -> Self {
        Self::calculate_partial(&pillars.year, &pillars.month, &pillars.day, &pillars.hour)
    }

    /// 원국 간지들로부터 직접 보조 기둥 계산 (FourPillars 생성 과정에서 사용)
    pub fn calculate_partial(year: &GanZi, month: &GanZi, _day: &GanZi, hour: &GanZi) -> Self {
        let taewon = Self::calculate_taewon(month);
        let myeonggung = Self::calculate_myeonggung(year, month, hour);
        let shingung = Self::calculate_shingung(year, month, hour);

        Self {
            taewon,
            myeonggung,
            shingung,
            meta: SupplementaryPillarsMeta {
                formula_version: "1.0".to_string(),
                formula_name: "Traditional (Yin-centric)".to_string(),
                hour_sensitive: true,
            },
        }
    }

    /// 태원(胎元) 계산
    /// 공식: 월주 천간 + 1, 월주 지지 + 3
    fn calculate_taewon(month_ganzi: &GanZi) -> GanZi {
        let stem_idx = (month_ganzi.stem.index() as i32 + 1) % 10;
        let branch_idx = (month_ganzi.branch.index() as i32 + 3) % 12;

        GanZi::new(
            HeavenlyStem::from_index(stem_idx),
            EarthlyBranch::from_index(branch_idx),
        )
    }

    /// 명궁(命宮) 계산
    fn calculate_myeonggung(year: &GanZi, month: &GanZi, hour: &GanZi) -> GanZi {
        let m_idx = month.branch.index() as i32;
        let h_idx = hour.branch.index() as i32;

        let m_val = if m_idx >= 2 { m_idx } else { m_idx + 12 };
        let h_val = if h_idx >= 2 { h_idx } else { h_idx + 12 };

        let mut mg_branch_val = 14 - (m_val + h_val) % 12;
        if mg_branch_val <= 0 { mg_branch_val += 12; }
        if mg_branch_val > 12 { mg_branch_val -= 12; }

        let mg_branch_idx = (mg_branch_val % 12) as i32;
        let branch = EarthlyBranch::from_index(mg_branch_idx);

        let year_stem = year.stem;
        let start_stem_idx = (year_stem.index() as i32 % 5) * 2 + 2;
        let mg_stem_idx = (start_stem_idx + (mg_branch_idx - 2).rem_euclid(12)) % 10;

        GanZi::new(
            HeavenlyStem::from_index(mg_stem_idx),
            branch,
        )
    }

    /// 신궁(身宮) 계산
    fn calculate_shingung(year: &GanZi, month: &GanZi, hour: &GanZi) -> GanZi {
        let m_idx = month.branch.index() as i32;
        let h_idx = hour.branch.index() as i32;

        let m_val = if m_idx >= 2 { m_idx } else { m_idx + 12 };
        let h_val = if h_idx >= 2 { h_idx } else { h_idx + 12 };
        
        let mut sg_branch_val = (m_val + h_val - 2) % 12;
        if sg_branch_val <= 0 { sg_branch_val += 12; }
        if sg_branch_val > 12 { sg_branch_val -= 12; }
        
        let sg_branch_idx = (sg_branch_val % 12) as i32;
        let branch = EarthlyBranch::from_index(sg_branch_idx);

        let year_stem = year.stem;
        let start_stem_idx = (year_stem.index() as i32 % 5) * 2 + 2;
        let sg_stem_idx = (start_stem_idx + (sg_branch_idx - 2).rem_euclid(12)) % 10;

        GanZi::new(
            HeavenlyStem::from_index(sg_stem_idx),
            branch,
        )
    }
}

impl Default for SupplementaryPillars {
    fn default() -> Self {
        Self {
            taewon: GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Zi),
            myeonggung: GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Zi),
            shingung: GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Zi),
            meta: SupplementaryPillarsMeta {
                formula_version: "0.0".to_string(),
                formula_name: "None".to_string(),
                hour_sensitive: false,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::stem::HeavenlyStem::{self as S};
    use crate::core::branch::EarthlyBranch::{self as B};
    use crate::core::pillars::SajuInput;
    use chrono::Utc;

    fn create_mock_pillars(y: (S, B), m: (S, B), d: (S, B), h: (S, B)) -> FourPillars {
        let year_pillar = GanZi::new(y.0, y.1);
        let month_pillar = GanZi::new(m.0, m.1);
        let day_pillar = GanZi::new(d.0, d.1);
        let hour_pillar = GanZi::new(h.0, h.1);
        
        FourPillars {
            year: year_pillar,
            month: month_pillar,
            day: day_pillar,
            hour: hour_pillar,
            birth_time: Utc::now(),
            gender: eon_core::Gender::Male,
            raw_input: SajuInput::new_solar(2000, 1, 1, 12, 0),
            supplementary_pillars: SupplementaryPillars::calculate_partial(&year_pillar, &month_pillar, &day_pillar, &hour_pillar),
        }
    }

    #[test]
    fn test_supplementary_pillars_fixtures() {
        // Case 1: 2004-11-27 22:00 (甲申年 乙亥月 庚戌日 丁亥時)
        // 월주 乙亥 -> 태원: (乙+1=丙, 亥+3=寅) -> 丙寅
        // 명궁/신궁: 월지(亥:9), 시지(亥:9)
        // 명궁: 14 - (9+9)%12 = 14-6 = 8 -> 戌 (명궁지)
        // 신궁: (9+9-2)%12 = 4 -> 巳 (신궁지)
        let p1 = create_mock_pillars(
            (S::Jia, B::Shen), 
            (S::Yi, B::Hai), 
            (S::Geng, B::Xu), 
            (S::Ding, B::Hai)
        );
        let res1 = &p1.supplementary_pillars;
        assert_eq!(res1.taewon.stem, S::Bing);
        assert_eq!(res1.taewon.branch, B::Yin);
        assert_eq!(res1.myeonggung.branch, B::Xu);
        assert_eq!(res1.shingung.branch, B::Si);

        // Case 2: 1988-05-15 10:30 (戊辰年 丁巳月 庚午일 辛巳시)
        // 월주 丁巳 -> 태원: (丁+1=戊, 巳+3=申) -> 戊申
        // 명궁/신궁: 월지(巳:3), 시지(巳:3)
        // 명궁: 14 - (3+3)%12 = 8 -> 戌
        // 신궁: (3+3-2)%12 = 4 -> 巳
        let p2 = create_mock_pillars(
            (S::Wu, B::Chen),
            (S::Ding, B::Si),
            (S::Geng, B::Wu),
            (S::Xin, B::Si)
        );
        let res2 = &p2.supplementary_pillars;
        assert_eq!(res2.taewon.branch, B::Shen);
        assert_eq!(res2.myeonggung.branch, B::Xu);
        assert_eq!(res2.shingung.branch, B::Si);

        // Case 3: 2024-02-10 01:30 (甲辰年 丙寅月 甲辰日 乙丑時) - 인월(寅月) & 축시(丑時)
        // 월지(寅:0), 시지(丑:11)
        // 명궁: 14 - (0+11)%12 = 14-11 = 3 -> 巳
        // 신궁: (0+11-2)%12 = 9 -> 亥
        let p3 = create_mock_pillars(
            (S::Jia, B::Chen),
            (S::Bing, B::Yin),
            (S::Jia, B::Chen),
            (S::Yi, B::Chou)
        );
        let res3 = &p3.supplementary_pillars;
        assert_eq!(res3.myeonggung.branch, B::Si);
        assert_eq!(res3.shingung.branch, B::Hai);

        // Case 4: 1995-12-25 15:30 (乙亥年 戊子月 辛亥日 丙申時) - 자월(子月) & 신시(申시)
        // 월지(子:10), 시지(申:6)
        // 명궁: 14 - (10+6)%12 = 14-4 = 10 -> 子
        // 신궁: (10+6-2)%12 = 2 -> 寅
        let p4 = create_mock_pillars(
            (S::Yi, B::Hai),
            (S::Wu, B::Zi),
            (S::Xin, B::Hai),
            (S::Bing, B::Shen)
        );
        let res4 = &p4.supplementary_pillars;
        assert_eq!(res4.myeonggung.branch, B::Zi);
        assert_eq!(res4.shingung.branch, B::Yin);

        // Case 5: 1970-01-01 00:30 (己酉年 丙子月 壬子日 庚子時) - 자월(子月) & 자시(子時)
        // 월지(子:10), 시지(子:10)
        // 명궁: 14 - (10+10)%12 = 14-8 = 6 -> 申
        // 신궁: (10+10-2)%12 = 6 -> 申
        let p5 = create_mock_pillars(
            (S::Ji, B::You),
            (S::Bing, B::Zi),
            (S::Ren, B::Zi),
            (S::Geng, B::Zi)
        );
        let res5 = &p5.supplementary_pillars;
        assert_eq!(res5.myeonggung.branch, B::Shen);
        assert_eq!(res5.shingung.branch, B::Shen);

        // Case 6: 2010-08-15 14:30 (庚寅年 甲申月 丁未日 丁未時) - 신월(申月) & 미시(未時)
        // 월지(申:6), 시지(未:5)
        // 명궁: 14 - (6+5)%12 = 14-11 = 3 -> 巳
        // 신궁: (6+5-2)%12 = 9 -> 亥
        let p6 = create_mock_pillars(
            (S::Geng, B::Yin),
            (S::Jia, B::Shen),
            (S::Ding, B::Wei),
            (S::Ding, B::Wei)
        );
        let res6 = &p6.supplementary_pillars;
        assert_eq!(res6.myeonggung.branch, B::Si);
        assert_eq!(res6.shingung.branch, B::Hai);

        // Case 7: 1999-03-20 09:30 (己卯年 丁卯月 丙午일 癸巳시) - 묘월(卯月) & 사시(巳時)
        // 월지(卯:1), 시지(巳:3)
        // 명궁: 14 - (1+3)%12 = 10 -> 子
        // 신궁: (1+3-2)%12 = 2 -> 寅
        let p7 = create_mock_pillars(
            (S::Ji, B::Mao),
            (S::Ding, B::Mao),
            (S::Bing, B::Wu),
            (S::Gui, B::Si)
        );
        let res7 = &p7.supplementary_pillars;
        assert_eq!(res7.myeonggung.branch, B::Zi);
        assert_eq!(res7.shingung.branch, B::Yin);

        // Case 8: 2025-05-20 18:30 (乙巳年 辛巳月 己丑일 癸酉시) - 사월(巳月) & 유시(酉時)
        // 월지(巳:3), 시지(酉:7)
        // 명궁: 14 - (3+7)%12 = 4 -> 巳
        // 신궁: (3+7-2)%12 = 8 -> 戌
        let p8 = create_mock_pillars(
            (S::Yi, B::Si),
            (S::Xin, B::Si),
            (S::Ji, B::Chou),
            (S::Gui, B::You)
        );
        let res8 = &p8.supplementary_pillars;
        assert_eq!(res8.myeonggung.branch, B::Si);
        assert_eq!(res8.shingung.branch, B::Xu);
    }
}
