use std::cmp::max;
use std::fs;

fn calculate_total_fuel(base_weight: i32) -> i32 {
    let mut fuel_to_add = calculate_added_fuel(&base_weight);
    let mut total_weight = fuel_to_add;
    while fuel_to_add > 0 {
        fuel_to_add = calculate_added_fuel(&fuel_to_add);
        total_weight += fuel_to_add;
    }
    total_weight
}
fn calculate_added_fuel(base_weight: &i32) -> i32 {
    max(base_weight / 3 - 2, 0)
}

pub fn day1(file_path: String) {
    let numbers: Vec<i32>;

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    numbers = contents
        .split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();
    println!(
        "{:?}",
        numbers
            .iter()
            .map(|s| calculate_total_fuel(*s))
            .sum::<i32>()
    );
}
