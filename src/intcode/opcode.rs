use crate::intcode::program::ProgramState;
#[derive(Debug, PartialEq)]
pub enum Op {
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
pub struct OpCode {
    op: Op,
    param_modes: [ParamType; 3],
}
impl OpCode {
    pub fn parse(code: &i32) -> Self {
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
    pub fn get_instruction_size(&self) -> usize {
        1 + self.op.number_of_parameters()
    }
    pub fn execute(&self, program_state: &mut ProgramState) -> bool {
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
        self.op.execute(program_state, &parameters)
    }
}
