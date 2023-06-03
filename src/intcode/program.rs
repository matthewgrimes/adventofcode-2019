use crate::intcode::opcode::OpCode;
#[derive(Debug)]
pub struct ProgramState {
    pub program: Vec<i32>,
    pub head: usize,
    pub running: bool,
    pub inputs: Vec<i32>,
    pub outputs: Vec<i32>,
}
impl ProgramState {
    pub fn update(&mut self) -> bool {
        while self.running {
            let current_op = OpCode::parse(&self.program[self.head]);
            let current_head = self.head;
            current_op.execute(self);
            // Only advance if an instruction didn't already modify head
            if current_head == self.head {
                self.head += current_op.get_instruction_size();
            }
        }
        false
    }
}
