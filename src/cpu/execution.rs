use super::instruction_set::Instruction;
use super::instruction_set::AddressingMode;

impl super::CPU {
    fn execute_instruction(&mut self, instruction : Instruction) {
        match instruction  {
            Instruction::NOP => {},
            Instruction::LDA(op, addr_mode) => {
                let val = get_addressed_val(op, addr_mode);
                self.A = op;
            }
        }
    }  
}

fn get_addressed_val(op: u8, addr_mode: AddressingMode) -> u8 {
    let ret: u8;
    match addr_mode {
        AddressingMode::Immediate => { 
            ret = op;
        }
    }
    return ret;
}


#[cfg(test)]
mod test {
    use crate::cpu::CPU;
    use crate::cpu::instruction_set::*;
    
    #[test]
    fn test_instructions() {
        let mut cpu: CPU = CPU::new();

        let val: u8 = 0xFF;
        let addr_mode = AddressingMode::Immediate;
        cpu.execute_instruction(Instruction::LDA(val, addr_mode));
        assert_eq!(val, cpu.A, "fail in LDA immediate");
    }
}

