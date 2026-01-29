use eon_saju::{FourPillars, SajuInput};
use eon_core::Gender;

fn main() {
    // 김성주: 2004-11-27 22:00
    let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
    let pillars = FourPillars::calculate(&input).unwrap();
    
    println!("【사주 팔자】: {}", pillars.hangul());
    
    let analysis = pillars.strength();
    println!("{}", analysis);
}
