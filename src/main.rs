pub mod day1;
pub(crate) mod utils;
use crate::day1::day1;
pub mod day2;
use crate::day2::day2;
pub mod day3;
use crate::day3::day3;
pub mod day4;
use crate::day4::day4;
pub mod day5;
use crate::day5::day5;
pub mod day6;
use crate::day6::day6;
pub mod day7;
use crate::day7::day7;
pub mod intcode;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = &args[1];
    let file_path = format!("inputs/day{}-input.txt", day);

    match day.parse().unwrap() {
        1 => day1(file_path),
        2 => day2(file_path),
        3 => day3(file_path),
        4 => day4(file_path),
        5 => day5(file_path),
        6 => day6(file_path),
        7 => day7(file_path),
        i32::MIN..=0_i32 | 8_i32..=i32::MAX => todo!(),
    };
}
