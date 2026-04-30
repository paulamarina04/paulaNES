use super::instruction_set::Instruction;
use super::instruction_set::AddressingMode;

impl super::CPU {
    fn execute_instruction(&mut self, instruction : Instruction) {
        match instruction  {
            //access
            Instruction::LDA(op, addr_mode) => {
                let val = get_addressed_val(op, addr_mode);
                self.A = op;
            },
            //transfer
            Instruction::TAX => {
                let val = self.A;
                self.X = val;
            },
            Instruction::TXA => {
                let val = self.X;
                self.A = val;
            },
            Instruction::TAY => {
                let val = self.A;
                self.Y = val;
            },
            Instruction::TYA => {
                let val = self.Y;
                self.A = val;
            },
            Instruction::TXS => {
                let val = self.X;
                self.S = val;
            },
            Instruction::TSX => {
                let val = self.S;
                self.X = val;
            },
            //other
            Instruction::NOP => {}
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

    // access instructions
    
    #[test]
    fn test_LDA() {
        let mut cpu: CPU = CPU::new();
        let val: u8 = 0xFF;
        let addr_mode = AddressingMode::Immediate;
        cpu.execute_instruction(Instruction::LDA(val, addr_mode));
        assert_eq!(val, cpu.A, "fail in LDA immediate");
    }

    // transfer instructions

    #[test]
    fn test_tax() {
        let mut cpu: CPU = CPU::new();
        cpu.A = 0xFF;
        let instruction = Instruction::TAX;
        cpu.execute_instruction(instruction);
        assert_eq!(cpu.X, cpu.A, "fail in TAX");
    }

    #[test]
    fn test_txa() {
        let mut cpu: CPU = CPU::new();
        cpu.X = 0xFF;
        let instruction = Instruction::TXA;
        cpu.execute_instruction(instruction);
        assert_eq!(cpu.X, cpu.A, "fail in TXA");
    }

    #[test]
    fn test_tay() {
        let mut cpu: CPU = CPU::new();
        cpu.A = 0xFF;
        let instruction = Instruction::TAY;
        cpu.execute_instruction(instruction);
        assert_eq!(cpu.Y, cpu.A, "fail in TAY");
    }

    #[test]
    fn test_tya() {
        let mut cpu: CPU = CPU::new();
        cpu.Y = 0xFF;
        let instruction = Instruction::TYA;
        cpu.execute_instruction(instruction);
        assert_eq!(cpu.Y, cpu.A, "fail in TYA");
    }

    #[test]
    fn test_txs() {
        let mut cpu: CPU = CPU::new();
        cpu.X = 0xFF;
        let instruction = Instruction::TXS;
        cpu.execute_instruction(instruction);
        assert_eq!(cpu.X, cpu.S, "fail in TXS");
    }

    #[test]
    fn test_tsx() {
        let mut cpu: CPU = CPU::new();
        cpu.S = 0xFF;
        let instruction = Instruction::TSX;
        cpu.execute_instruction(instruction);
        assert_eq!(cpu.X, cpu.S, "fail in TSX");
    }
}

