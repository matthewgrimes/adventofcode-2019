use std::fs;

pub fn day4(file_path: String) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let contents_parse: Vec<i32> = contents
        .split('-')
        .map(|x| x.trim().parse().unwrap())
        .collect();
    let range_min = contents_parse[0];
    let range_max = contents_parse[1];
    let mut number_of_matches: usize = 0;
    for candidate in range_min..range_max {
        if check_adjacency_rule(candidate) && check_no_decrease_rule(candidate) {
            number_of_matches += 1;
        }
    }
    println!("{:?}", number_of_matches);
}

fn check_no_decrease_rule(candidate: i32) -> bool {
    let candidate_vec: Vec<u32> = candidate
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect();
    let mut sorted = candidate_vec.clone();
    sorted.sort();
    sorted == candidate_vec
}
fn check_adjacency_rule(candidate: i32) -> bool {
    let candidate_vec: Vec<u32> = candidate
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect();
    for i in 0..candidate_vec.len() - 1 {
        if candidate_vec[i] == candidate_vec[i + 1]
            && (i >= candidate_vec.len() - 2 || candidate_vec[i] != candidate_vec[i + 2])
            && (i == 0 || candidate_vec[i] != candidate_vec[i - 1])
        {
            return true;
        }
    }
    false
}
#[cfg(test)]
mod tests {
    use crate::day4::{check_adjacency_rule, check_no_decrease_rule};
    #[test]
    fn check_112233() {
        assert_eq!(check_no_decrease_rule(112233), true);
        assert_eq!(check_adjacency_rule(112233), true);
    }
    #[test]
    fn check_123444() {
        assert_eq!(check_no_decrease_rule(123444), true);
        assert_eq!(check_adjacency_rule(123444), false);
    }
    #[test]
    fn check_111122() {
        assert_eq!(check_no_decrease_rule(111122), true);
        assert_eq!(check_adjacency_rule(111122), true);
    }
}
