use std::fs;

#[derive(Debug, PartialEq)]
enum Op {
    Add,
    Mult,
    Save,
    Read,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}
impl Op {
    fn from_digits(code: i32) -> Self {
        match code {
            1 => Op::Add,
            2 => Op::Mult,
            3 => Op::Save,
            4 => Op::Read,
            5 => Op::JumpIfTrue,
            6 => Op::JumpIfFalse,
            7 => Op::LessThan,
            8 => Op::Equals,
            99 => Op::Halt,
            _ => {
                todo!()
            }
        }
    }
    fn writes_to_program(&self) -> bool {
        matches!(
            self,
            Op::Add | Op::Mult | Op::Save | Op::LessThan | Op::Equals
        )
    }
    fn number_of_parameters(&self) -> usize {
        match self {
            Op::Add => 3,
            Op::Mult => 3,
            Op::Save => 1,
            Op::Read => 1,
            Op::Halt => 0,
            Op::JumpIfTrue => 2,
            Op::JumpIfFalse => 2,
            Op::LessThan => 3,
            Op::Equals => 3,
        }
    }
    fn execute(&self, program_state: &mut ProgramState, parameters: &Vec<i32>) -> bool {
        if parameters.len() != self.number_of_parameters() {
            todo!();
        }
        println!("{:?} {:?}", self, parameters);
        match self {
            Op::Add => {
                program_state.program[parameters[2] as usize] = parameters[0] + parameters[1];
            }
            Op::Mult => {
                program_state.program[parameters[2] as usize] = parameters[0] * parameters[1];
            }
            Op::Halt => {
                program_state.running = false;
            }
            Op::Save => {
                program_state.program[parameters[0] as usize] = program_state.inputs.pop().unwrap();
            }
            Op::Read => {
                program_state.outputs.push(parameters[0]);
            }
            Op::JumpIfTrue => {
                if parameters[0] != 0 {
                    program_state.head = parameters[1] as usize;
                }
            }
            Op::JumpIfFalse => {
                if parameters[0] == 0 {
                    program_state.head = parameters[1] as usize;
                }
            }
            Op::LessThan => {
                program_state.program[parameters[2] as usize] =
                    i32::from(parameters[0] < parameters[1]);
            }
            Op::Equals => {
                program_state.program[parameters[2] as usize] =
                    i32::from(parameters[0] == parameters[1]);
            }
        }
        true
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
        println!("{:?}", self);
        let mut parameters: Vec<i32> = Vec::new();
        if self.op.number_of_parameters() > 0 {
            for parameter_index in 0..self.op.number_of_parameters() {
                let parameter = program_state.program[program_state.head + parameter_index + 1];
                match self.param_modes[parameter_index] {
                    ParamType::Position => {
                        parameters.push(program_state.program[parameter as usize])
                    }
                    ParamType::Immediate => parameters.push(parameter),
                }
            }
            // parameters an instruction writes to are never in immediate mode!
            if self.op.writes_to_program() {
                parameters[self.op.number_of_parameters() - 1] =
                    program_state.program[program_state.head + self.op.number_of_parameters()];
            }
        }
        println!("{:?}", parameters);
        self.op.execute(program_state, &parameters)
    }
}
#[derive(Debug)]
struct ProgramState {
    program: Vec<i32>,
    head: usize,
    running: bool,
    inputs: Vec<i32>,
    outputs: Vec<i32>,
}
impl ProgramState {
    fn update(&mut self) -> bool {
        while self.running {
            let current_op = OpCode::parse(&self.program[self.head]);
            let current_head = self.head;
            current_op.execute(self);
            // Only advance if an instruction didn't already modify head
            if current_head == self.head {
                self.head += current_op.get_instruction_size();
            } else {
                println!("Already moved.")
            }
        }
        false
    }
}
fn evaluate_program(program: Vec<i32>, inputs: Vec<i32>) -> ProgramState {
    let mut program_state = ProgramState {
        program,
        head: 0,
        running: true,
        inputs,
        outputs: Vec::<i32>::new(),
    };
    program_state.update();
    program_state
}
pub fn day5(file_path: String) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let numbers: Vec<i32> = contents
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();
    println!(
        "{:?}",
        evaluate_program(numbers, vec![1]).outputs.pop().unwrap()
    );
    let numbers: Vec<i32> = contents
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();
    println!("{:?}", evaluate_program(numbers, vec![5]));
}

#[cfg(test)]
mod tests {
    use crate::day5::evaluate_program;
    #[test]
    fn test_1() {
        let result = evaluate_program(
            vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
            Vec::<i32>::new(),
        );
        assert_eq!(
            result.program,
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }
    #[test]
    fn test_2() {
        let result = evaluate_program(vec![1, 0, 0, 0, 99], Vec::<i32>::new());
        assert_eq!(result.program, vec![2, 0, 0, 0, 99]);
    }
    #[test]
    fn test_3() {
        let result = evaluate_program(vec![2, 3, 0, 3, 99], Vec::<i32>::new());
        assert_eq!(result.program, vec![2, 3, 0, 6, 99]);
    }
    #[test]
    fn test_4() {
        let result = evaluate_program(vec![2, 4, 4, 5, 99, 0], Vec::<i32>::new());
        assert_eq!(result.program, vec![2, 4, 4, 5, 99, 9801]);
    }
    #[test]
    fn test_5() {
        let result = evaluate_program(vec![1, 1, 1, 4, 99, 5, 6, 0, 99], Vec::<i32>::new());
        assert_eq!(result.program, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
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
        let result = evaluate_program(input, Vec::<i32>::new());
        assert_eq!(result.program[0], 5305097);
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
        let result = evaluate_program(input, Vec::<i32>::new());
        assert_eq!(result.program[0], 19690720);
    }
    #[test]
    fn test_day5_example() {
        let input = vec![1002, 4, 3, 4, 33];
        let result = evaluate_program(input, Vec::<i32>::new());
        assert_eq!(result.program, vec![1002, 4, 3, 4, 99]);
    }
    #[test]
    fn test_day5_part1() {
        let program = vec![
            3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1102, 89, 49, 225, 1102, 35, 88, 224,
            101, -3080, 224, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 3, 224, 1, 223, 224, 223,
            1101, 25, 33, 224, 1001, 224, -58, 224, 4, 224, 102, 8, 223, 223, 101, 5, 224, 224, 1,
            223, 224, 223, 1102, 78, 23, 225, 1, 165, 169, 224, 101, -80, 224, 224, 4, 224, 102, 8,
            223, 223, 101, 7, 224, 224, 1, 224, 223, 223, 101, 55, 173, 224, 1001, 224, -65, 224,
            4, 224, 1002, 223, 8, 223, 1001, 224, 1, 224, 1, 223, 224, 223, 2, 161, 14, 224, 101,
            -3528, 224, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 7, 224, 1, 224, 223, 223, 1002,
            61, 54, 224, 1001, 224, -4212, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 1, 224, 1,
            223, 224, 223, 1101, 14, 71, 225, 1101, 85, 17, 225, 1102, 72, 50, 225, 1102, 9, 69,
            225, 1102, 71, 53, 225, 1101, 10, 27, 225, 1001, 158, 34, 224, 101, -51, 224, 224, 4,
            224, 102, 8, 223, 223, 101, 6, 224, 224, 1, 223, 224, 223, 102, 9, 154, 224, 101, -639,
            224, 224, 4, 224, 102, 8, 223, 223, 101, 2, 224, 224, 1, 224, 223, 223, 4, 223, 99, 0,
            0, 0, 677, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1105, 0, 99999, 1105, 227, 247, 1105, 1,
            99999, 1005, 227, 99999, 1005, 0, 256, 1105, 1, 99999, 1106, 227, 99999, 1106, 0, 265,
            1105, 1, 99999, 1006, 0, 99999, 1006, 227, 274, 1105, 1, 99999, 1105, 1, 280, 1105, 1,
            99999, 1, 225, 225, 225, 1101, 294, 0, 0, 105, 1, 0, 1105, 1, 99999, 1106, 0, 300,
            1105, 1, 99999, 1, 225, 225, 225, 1101, 314, 0, 0, 106, 0, 0, 1105, 1, 99999, 108, 226,
            226, 224, 102, 2, 223, 223, 1006, 224, 329, 101, 1, 223, 223, 1007, 677, 677, 224,
            1002, 223, 2, 223, 1005, 224, 344, 1001, 223, 1, 223, 8, 226, 677, 224, 1002, 223, 2,
            223, 1006, 224, 359, 1001, 223, 1, 223, 108, 226, 677, 224, 1002, 223, 2, 223, 1005,
            224, 374, 1001, 223, 1, 223, 107, 226, 677, 224, 102, 2, 223, 223, 1006, 224, 389, 101,
            1, 223, 223, 1107, 226, 226, 224, 1002, 223, 2, 223, 1005, 224, 404, 1001, 223, 1, 223,
            1107, 677, 226, 224, 102, 2, 223, 223, 1005, 224, 419, 101, 1, 223, 223, 1007, 226,
            226, 224, 102, 2, 223, 223, 1006, 224, 434, 1001, 223, 1, 223, 1108, 677, 226, 224,
            1002, 223, 2, 223, 1005, 224, 449, 101, 1, 223, 223, 1008, 226, 226, 224, 102, 2, 223,
            223, 1005, 224, 464, 101, 1, 223, 223, 7, 226, 677, 224, 102, 2, 223, 223, 1006, 224,
            479, 101, 1, 223, 223, 1008, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 494, 101, 1,
            223, 223, 1107, 226, 677, 224, 1002, 223, 2, 223, 1005, 224, 509, 1001, 223, 1, 223,
            1108, 226, 226, 224, 1002, 223, 2, 223, 1006, 224, 524, 101, 1, 223, 223, 7, 226, 226,
            224, 102, 2, 223, 223, 1006, 224, 539, 1001, 223, 1, 223, 107, 226, 226, 224, 102, 2,
            223, 223, 1006, 224, 554, 101, 1, 223, 223, 107, 677, 677, 224, 102, 2, 223, 223, 1006,
            224, 569, 101, 1, 223, 223, 1008, 677, 677, 224, 1002, 223, 2, 223, 1006, 224, 584,
            1001, 223, 1, 223, 8, 677, 226, 224, 1002, 223, 2, 223, 1005, 224, 599, 101, 1, 223,
            223, 1108, 226, 677, 224, 1002, 223, 2, 223, 1005, 224, 614, 101, 1, 223, 223, 108,
            677, 677, 224, 102, 2, 223, 223, 1005, 224, 629, 1001, 223, 1, 223, 8, 677, 677, 224,
            1002, 223, 2, 223, 1005, 224, 644, 1001, 223, 1, 223, 7, 677, 226, 224, 102, 2, 223,
            223, 1006, 224, 659, 1001, 223, 1, 223, 1007, 226, 677, 224, 102, 2, 223, 223, 1005,
            224, 674, 101, 1, 223, 223, 4, 223, 99, 226,
        ];
        let result = evaluate_program(program, vec![1]);
        assert_eq!(*result.outputs.last().unwrap(), 7839346);
    }
    #[test]
    fn test_equal_position() {
        for value in 5..10 {
            let input = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
            assert_eq!(
                *evaluate_program(input, vec![value]).outputs.last().unwrap(),
                if value == 8 { 1 } else { 0 }
            );
        }
    }
    #[test]
    fn test_lt_position() {
        for value in 5..10 {
            let input = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
            assert_eq!(
                *evaluate_program(input, vec![value]).outputs.last().unwrap(),
                if value < 8 { 1 } else { 0 }
            );
        }
    }
    #[test]
    fn test_equal_immediate() {
        for value in 5..10 {
            let input = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
            assert_eq!(
                *evaluate_program(input, vec![value]).outputs.last().unwrap(),
                if value == 8 { 1 } else { 0 }
            );
        }
    }
    #[test]
    fn test_lt_immediate() {
        for value in 5..10 {
            let input = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
            assert_eq!(
                *evaluate_program(input, vec![value]).outputs.last().unwrap(),
                if value < 8 { 1 } else { 0 }
            );
        }
    }
    #[test]
    fn test_comparison_full() {
        for value in 5..10 {
            let input = vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ];
            assert_eq!(
                *evaluate_program(input, vec![value]).outputs.last().unwrap(),
                if value < 8 {
                    999
                } else if value == 8 {
                    1000
                } else {
                    1001
                }
            );
        }
    }
    #[test]
    fn test_read_position() {
        let input = vec![4, 5, 99, -1, -1, 27];
        assert_eq!(
            *evaluate_program(input, Vec::<i32>::new())
                .outputs
                .last()
                .unwrap(),
            27
        );
    }
    #[test]
    fn test_read_immediate() {
        let input = vec![104, 5, 99, -1, -1, 27];
        assert_eq!(
            *evaluate_program(input, Vec::<i32>::new())
                .outputs
                .last()
                .unwrap(),
            5
        );
    }
}
