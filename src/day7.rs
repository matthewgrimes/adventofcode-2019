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
}
