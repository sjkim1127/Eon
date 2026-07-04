use eon_core::Gender;
use eon_saju::core::pillars::FourPillars;
use eon_saju::core::pillars::SajuInput;
use eon_saju::engine::emulator::LifePathEmulator;

fn main() {
    let input = SajuInput::new_solar(1990, 5, 10, 12, 0).with_gender(Gender::Male);
    let pillars = FourPillars::calculate(&input).unwrap();
    let emulator = LifePathEmulator::new(pillars.clone(), Gender::Male, 1990);
    match emulator.emulate() {
        Ok(r) => println!("Success! Timeline: {}", r.timeline.len()),
        Err(e) => println!("Error! {:?}", e),
    }
}
