use eon_saju::{FourPillars, SajuInput};

fn main() {
    // Kim Sung-ju: 2004-11-27 22:00
    let input = SajuInput::new_solar(2004, 11, 27, 22, 0);
    let pillars = FourPillars::calculate(&input).unwrap();

    println!("【Four Pillars】: {}", pillars.hangul());

    let analysis = pillars.strength();
    println!("{}", analysis);
}
