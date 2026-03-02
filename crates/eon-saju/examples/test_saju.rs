use eon_core::{BirthInfo, Gender, Location};
use eon_saju::core::pillars::{FourPillars, SajuInput};

fn main() {
    let gender = Gender::Male;
    let location = Location::new("출생지", 37.32, 126.83, 135.0);
    // 2004-11-27 22:00
    let birth_info = BirthInfo::solar(2004, 11, 27, 22, 0)
        .with_timezone("Asia/Seoul")
        .with_location(location)
        .with_true_solar_time(true)
        .with_gender(gender);

    let (cy, cm, cd, ch, cmin) = birth_info.corrected_datetime();
    println!("Corrected: {}-{}-{} {}:{}", cy, cm, cd, ch, cmin);

    let input = SajuInput::new_solar_at(cy, cm, cd, ch, cmin, 126.83, 37.32).with_gender(gender);
    match FourPillars::calculate(&input) {
        Ok(pillars) => println!("Success: {:?}", pillars),
        Err(e) => println!("Error: {}", e),
    }
}
