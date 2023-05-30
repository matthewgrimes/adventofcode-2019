use std::fs;

#[derive(Debug)]
enum Op {
    Add,
    Mult,
    //    SAVE,
    //    READ,
    Halt,
}
impl Op {
    fn from_digits(code: i32) -> Self {
        match code {
            1 => Op::Add,
            2 => Op::Mult,
            99 => Op::Halt,
            _ => {
                todo!()
            }
        }
    }
    fn number_of_parameters(&self) -> usize {
        match self {
            Op::Add => 3,
            Op::Mult => 3,
            //Op::SAVE => 1,
            //Op::READ => 1,
            Op::Halt => 0,
        }
    }
    fn execute(&self, program_state: &mut ProgramState, parameters: &Vec<i32>) -> bool {
        if parameters.len() != self.number_of_parameters() {
            todo!();
        }
        match self {
            Op::Add => {
                program_state.program[parameters[2] as usize] = parameters[0] + parameters[1];
                true
            }
            Op::Mult => {
                program_state.program[parameters[2] as usize] = parameters[0] * parameters[1];
                true
            }
            Op::Halt => false,
        }
    }
}
#[derive(Debug)]
enum ParamType {
    Position,
    Immediate,
}
impl ParamType {
    fn from_digit(digit: i32) -> Self {
        match digit {
            0 => ParamType::Position,
            1 => ParamType::Immediate,
            _ => todo!(),
        }
    }
}
#[derive(Debug)]
struct OpCode {
    op: Op,
    param_modes: [ParamType; 3],
}
impl OpCode {
    fn parse(code: &i32) -> Self {
        let digits: Vec<i32> = format!("{:0>5}", code.to_string())
            .chars()
            .map(|x| x.to_digit(10).unwrap() as i32)
            .collect();
        OpCode {
            op: Op::from_digits(digits[3] * 10 + digits[4]),
            param_modes: [
                ParamType::from_digit(digits[2]),
                ParamType::from_digit(digits[1]),
                ParamType::from_digit(digits[0]),
            ],
        }
    }
    fn get_instruction_size(&self) -> usize {
        1 + self.op.number_of_parameters()
    }
    fn execute(&self, program_state: &mut ProgramState) -> bool {
        let mut parameters: Vec<i32> = Vec::new();
        //self.op = Op::from_digits(program_state.program[program_state.head]);
        if self.op.number_of_parameters() > 0 {
            for parameter_index in 0..self.op.number_of_parameters() - 1 {
                let parameter = program_state.program[program_state.head + parameter_index + 1];
                match self.param_modes[parameter_index] {
                    ParamType::Position => {
                        parameters.push(program_state.program[parameter as usize])
                    }
                    ParamType::Immediate => parameters.push(parameter),
                }
            }
            parameters
                .push(program_state.program[program_state.head + self.op.number_of_parameters()]);
        }
        self.op.execute(program_state, &parameters)
    }
}
#[derive(Debug)]
struct ProgramState {
    program: Vec<i32>,
    head: usize,
}
impl ProgramState {
    fn update(&mut self) -> bool {
        let mut continue_program = true;
        while continue_program {
            let current_op = OpCode::parse(&self.program[self.head]);
            continue_program = current_op.execute(self);
            self.head += current_op.get_instruction_size();
        }
        false
    }
}
fn evaluate_program(program: Vec<i32>) -> Vec<i32> {
    let mut program_state = ProgramState { program, head: 0 };
    program_state.update();
    program_state.program
}
pub fn day5(file_path: String) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let numbers: Vec<i32> = contents
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();
    println!("{:?}", evaluate_program(numbers));
}

#[cfg(test)]
mod tests {
    use crate::day5::evaluate_program;
    #[test]
    fn test_1() {
        let result = evaluate_program(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
        assert_eq!(result, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
    }
    #[test]
    fn test_2() {
        let result = evaluate_program(vec![1, 0, 0, 0, 99]);
        assert_eq!(result, vec![2, 0, 0, 0, 99]);
    }
    #[test]
    fn test_3() {
        let result = evaluate_program(vec![2, 3, 0, 3, 99]);
        assert_eq!(result, vec![2, 3, 0, 6, 99]);
    }
    #[test]
    fn test_4() {
        let result = evaluate_program(vec![2, 4, 4, 5, 99, 0]);
        assert_eq!(result, vec![2, 4, 4, 5, 99, 9801]);
    }
    #[test]
    fn test_5() {
        let result = evaluate_program(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        assert_eq!(result, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
    #[test]
    fn test_full_day2_part1() {
        let input = vec![
            1, 12, 2, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 13, 19, 1, 9, 19, 23, 1, 6, 23,
            27, 2, 27, 9, 31, 2, 6, 31, 35, 1, 5, 35, 39, 1, 10, 39, 43, 1, 43, 13, 47, 1, 47, 9,
            51, 1, 51, 9, 55, 1, 55, 9, 59, 2, 9, 59, 63, 2, 9, 63, 67, 1, 5, 67, 71, 2, 13, 71,
            75, 1, 6, 75, 79, 1, 10, 79, 83, 2, 6, 83, 87, 1, 87, 5, 91, 1, 91, 9, 95, 1, 95, 10,
            99, 2, 9, 99, 103, 1, 5, 103, 107, 1, 5, 107, 111, 2, 111, 10, 115, 1, 6, 115, 119, 2,
            10, 119, 123, 1, 6, 123, 127, 1, 127, 5, 131, 2, 9, 131, 135, 1, 5, 135, 139, 1, 139,
            10, 143, 1, 143, 2, 147, 1, 147, 5, 0, 99, 2, 0, 14, 0,
        ];
        let result = evaluate_program(input);
        assert_eq!(result[0], 5305097);
    }
    #[test]
    fn test_full_day2_part2() {
        let mut input = vec![
            1, 12, 2, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 13, 19, 1, 9, 19, 23, 1, 6, 23,
            27, 2, 27, 9, 31, 2, 6, 31, 35, 1, 5, 35, 39, 1, 10, 39, 43, 1, 43, 13, 47, 1, 47, 9,
            51, 1, 51, 9, 55, 1, 55, 9, 59, 2, 9, 59, 63, 2, 9, 63, 67, 1, 5, 67, 71, 2, 13, 71,
            75, 1, 6, 75, 79, 1, 10, 79, 83, 2, 6, 83, 87, 1, 87, 5, 91, 1, 91, 9, 95, 1, 95, 10,
            99, 2, 9, 99, 103, 1, 5, 103, 107, 1, 5, 107, 111, 2, 111, 10, 115, 1, 6, 115, 119, 2,
            10, 119, 123, 1, 6, 123, 127, 1, 127, 5, 131, 2, 9, 131, 135, 1, 5, 135, 139, 1, 139,
            10, 143, 1, 143, 2, 147, 1, 147, 5, 0, 99, 2, 0, 14, 0,
        ];
        input[1] = 49;
        input[2] = 25;
        let result = evaluate_program(input);
        assert_eq!(result[0], 19690720);
    }
    #[test]
    fn test_day5_example() {
        let input = vec![1002, 4, 3, 4, 33];
        let result = evaluate_program(input);
        assert_eq!(result, vec![1002, 4, 3, 4, 99]);
    }
}
