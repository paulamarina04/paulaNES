use super::instruction_set::Instruction;
use super::CPU;

impl CPU {
    fn execute_instruction(&mut self, instruction : Instruction) {
        match instruction  {
            Instruction::NOP => {},
            Instruction::LDA(op) => {
                self.A = op;
            }
        }
    }  
}


#[cfg(test)]
mod test {
    use crate::cpu::CPU;
    use crate::cpu::instruction_set::Instruction;
    
    #[test]
    fn test_instructions() {
        let mut cpu: CPU = CPU::new();

        let val: u8 = 0xFF;
        cpu.execute_instruction(Instruction::LDA(val));
        assert_eq!(val, cpu.A, "fail in LDA immediate");
    }
}

