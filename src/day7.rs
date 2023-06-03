use std::fs;

use crate::intcode::program::ProgramState;

pub fn day7(file_path: String) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let numbers: Vec<i32> = contents
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();
    println!("{:?}", numbers);
}

fn calculate_signal(program: Vec<i32>, settings: Vec<i32>) -> i32 {
    let mut previous_output = 0;
    let mut amplifier = ProgramState {
        program: program.clone(),
        head: 0,
        running: true,
        inputs: vec![previous_output, settings[0]],
        outputs: vec![],
    };
    amplifier.update();
    for setting in settings.iter().take(5).skip(1) {
        previous_output = amplifier.outputs.pop().unwrap();
        amplifier = ProgramState {
            program: program.clone(),
            head: 0,
            running: true,
            inputs: vec![previous_output, *setting],
            outputs: vec![],
        };
        amplifier.update();
    }
    amplifier.outputs.pop().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_1() {
        let input = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        assert_eq!(
            crate::day7::calculate_signal(input, vec![4, 3, 2, 1, 0]),
            43210
        )
    }
    #[test]
    fn test_example_2() {
        let input = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        assert_eq!(
            crate::day7::calculate_signal(input, vec![0, 1, 2, 3, 4]),
            54321
        )
    }
    #[test]
    fn test_example_3() {
        let input = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        assert_eq!(
            crate::day7::calculate_signal(input, vec![1, 0, 4, 3, 2]),
            65210
        )
    }
}
