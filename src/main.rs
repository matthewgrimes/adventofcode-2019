pub mod day1;
use crate::day1::day1;
pub mod day2;
use crate::day2::day2;
pub mod day3;
use crate::day3::day3;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = &args[1];
    let file_path = (&args[2]).to_string();

    match day.parse().unwrap() {
        1 => day1(file_path),
        2 => day2(file_path),
        3 => day3(file_path),
        i32::MIN..=0_i32 | 2_i32..=i32::MAX => todo!()
    }
}
