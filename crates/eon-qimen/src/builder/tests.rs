use crate::builder::pan::*;
use eon_saju::core::branch::EarthlyBranch;
use eon_saju::core::ganzi::GanZi;
use eon_saju::core::stem::HeavenlyStem;

#[test]
fn test_xun_shou() {
    let jiazi = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Zi);
    assert_eq!(get_xun_shou(jiazi), HeavenlyStem::Wu);

    let jiayin = GanZi::new(HeavenlyStem::Jia, EarthlyBranch::Yin);
    assert_eq!(get_xun_shou(jiayin), HeavenlyStem::Gui);

    let xinwei = GanZi::new(HeavenlyStem::Xin, EarthlyBranch::Wei); // 갑자순 (미=7, 신=7 => 7-7 = 0)
    assert_eq!(get_xun_shou(xinwei), HeavenlyStem::Wu);
}
