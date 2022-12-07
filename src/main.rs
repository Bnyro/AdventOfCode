pub mod solutions;
pub mod util;

fn main() {
    println!("1) 1: {}", solutions::day_1::part_one());
    println!("1) 2: {}", solutions::day_1::part_two());

    println!("2) 1: {}", solutions::day_2::part_one());
    println!("2) 2: {}", solutions::day_2::part_two());

    println!("3) 1: {}", solutions::day_3::part_one());
    println!("3) 2: {}", solutions::day_3::part_two());

    println!("4) 1: {}", solutions::day_4::part_one());
    println!("4) 2: {}", solutions::day_4::part_two());
}
