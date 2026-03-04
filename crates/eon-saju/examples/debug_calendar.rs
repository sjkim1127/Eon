use eon_saju::core::calendar::*;

fn main() {
    println!(
        "Res 1: {:?}",
        get_month_branch_index(2024, 2, 3, 12, 0, 540)
    );
    println!(
        "Res 2: {:?}",
        get_month_branch_index(2024, 2, 4, 18, 0, 540)
    );
}
