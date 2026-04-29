use super::instruction_set::Instruction;

impl super::CPU {
    fn _execute_instruction(&mut self, instruction : Instruction) {
        match instruction  {
            Instruction::NOP => {},
            Instruction::LDA(op) => {
                self.A = op;
            }
        }
    }  
}