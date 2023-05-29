use std::fs;

fn evaluate_program(mut program: Vec<usize>) -> Vec<usize> {
    let mut position = 0;
    let mut new_value;
    let mut new_value_position;
    let mut terminate_program = false;

    while !terminate_program {
        let opcode = program[position];
        if opcode == 99 {
            terminate_program = true;
            continue;
        }

        new_value_position = program[position + 3];
        if opcode == 1 {
            new_value = program[program[position + 1]] + program[program[position + 2]];
            program[new_value_position] = new_value;
            position += 4;
        } else if opcode == 2 {
            new_value = program[program[position + 1]] * program[program[position + 2]];
            program[new_value_position] = new_value;
            position += 4;
        } else {
            println!("Unknown opcode: {:?}", opcode);
            terminate_program = true;
        }
    }
    program
}
pub fn day5(file_path: String) {
    let mut numbers: Vec<usize>;

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    numbers = contents
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();
    for noun in 0..100 {
        numbers[1] = noun;
        for verb in 0..100 {
            numbers[2] = verb;
            let completed_program = evaluate_program(numbers.clone()); 
            let (result, answer) = (completed_program[1]*100 + completed_program[2], completed_program[0]);
            if result == 19690720 {
                println!("{:?}", (result, answer));
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::day5::{evaluate_program};
    #[test]
    fn test_1() {
    let result = evaluate_program(vec![1,9,10,3,2,3,11,0,99,30,40,50]);
    assert_eq!(result, vec![3500,9,10,70,2,3,11,0,99,30,40,50]);
    }
    #[test]
    fn test_2() {
    let result = evaluate_program(vec![1,0,0,0,99]);
    assert_eq!(result, vec![2,0,0,0,99]);
    }
    #[test]
    fn test_3() {
    let result = evaluate_program(vec![2,3,0,3,99]);
    assert_eq!(result, vec![2,3,0,6,99]);
    }
    #[test]
    fn test_4() {
    let result = evaluate_program(vec![2,4,4,5,99,0]);
    assert_eq!(result, vec![2,4,4,5,99,9801]);
    }
    #[test]
    fn test_5() {
    let result = evaluate_program(vec![1,1,1,4,99,5,6,0,99]);
    assert_eq!(result, vec![30,1,1,4,2,5,6,0,99]);
    }
    #[test]
    fn test_full_day2_part1() {
    let input = vec![1,12,2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,13,19,1,9,19,23,1,6,23,27,2,27,9,31,2,6,31,35,1,5,35,39,1,10,39,43,1,43,13,47,1,47,9,51,1,51,9,55,1,55,9,59,2,9,59,63,2,9,63,67,1,5,67,71,2,13,71,75,1,6,75,79,1,10,79,83,2,6,83,87,1,87,5,91,1,91,9,95,1,95,10,99,2,9,99,103,1,5,103,107,1,5,107,111,2,111,10,115,1,6,115,119,2,10,119,123,1,6,123,127,1,127,5,131,2,9,131,135,1,5,135,139,1,139,10,143,1,143,2,147,1,147,5,0,99,2,0,14,0];
    let result = evaluate_program(input);
    assert_eq!(result[0], 5305097);
    }
    #[test]
    fn test_full_day2_part2() {
    let mut input = vec![1,12,2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,13,19,1,9,19,23,1,6,23,27,2,27,9,31,2,6,31,35,1,5,35,39,1,10,39,43,1,43,13,47,1,47,9,51,1,51,9,55,1,55,9,59,2,9,59,63,2,9,63,67,1,5,67,71,2,13,71,75,1,6,75,79,1,10,79,83,2,6,83,87,1,87,5,91,1,91,9,95,1,95,10,99,2,9,99,103,1,5,103,107,1,5,107,111,2,111,10,115,1,6,115,119,2,10,119,123,1,6,123,127,1,127,5,131,2,9,131,135,1,5,135,139,1,139,10,143,1,143,2,147,1,147,5,0,99,2,0,14,0];
    input[1] = 49;
    input[2] = 25;
    let result = evaluate_program(input);
    assert_eq!(result[0], 19690720);
    }

}
