use super::instruction_set::Instruction;
use super::instruction_set::AddrMode;
use std::num::Wrapping;

impl super::CPU {
    fn execute_instruction(&mut self, instruction : Instruction) {
        match instruction  {
            //access
            Instruction::LDA(addr_mode) => {
                let val = get_addressed_val(addr_mode);
                self.A = val;
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
            //arithmetic
            Instruction::ADC(addr_mode) => {
                let val1 = self.A;
                let val2 = get_addressed_val(addr_mode);
                let carry = if self.C { 0x01 } else { 0x00 };
                let result = Wrapping(val1) + Wrapping(val2) + Wrapping(carry);
                let result: u8 = result.0;
                self.A = result;
                // flag bitwise shenanigans
                self.C = val1 as u16 + val2 as u16 + carry as u16 > 0xFF;
                self.Z = result == 0x00;
                self.V = (result ^ val1) & (result ^ val2) & 0b10000000 == 0b10000000;
                self.N = result & 0b10000000 == 0b10000000;
            }
            //flags
            Instruction::CLC => {
                self.C = false;
            },
            Instruction::SEC => {
                self.C = true;
            },
            Instruction::CLI => {
                self.I = false;
            },
            Instruction::SEI => {
                self.I = true;
            },
            Instruction::CLD => {
                self.D = false;
            },
            Instruction::SED => {
                self.D = true;
            },
            Instruction::CLV => {
                self.V = false;
            },
            //other
            Instruction::NOP => {}
        }
    }  
}

fn get_addressed_val(addr_mode: AddrMode) -> u8 {
    let ret: u8;
    match addr_mode {
        AddrMode::Immediate(op) => { 
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
    fn test_lda() {
        let mut cpu: CPU = CPU::new();
        let val: u8 = 0xFF;
        let addr_mode = AddrMode::Immediate(val);
        cpu.execute_instruction(Instruction::LDA(addr_mode));
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

    // arithmetic instructions

    #[test]
    fn test_adc() {
        let mut cpu: CPU = CPU::new();
        // 0 + 0 no carry
        cpu.A = 0x00;
        let op = 0x00;
        cpu.C = false;
        let instruction = Instruction::ADC(AddrMode::Immediate(op));
        cpu.execute_instruction(instruction);
        assert_eq!(0x00, cpu.A);
        assert_eq!(false, cpu.C);
        assert_eq!(true, cpu.Z);
        assert_eq!(false, cpu.V);
        assert_eq!(false, cpu.N);
        // 0 + 0 with carry
        cpu.A = 0x00;
        let op = 0x00;
        cpu.C = true;
        let instruction = Instruction::ADC(AddrMode::Immediate(op));
        cpu.execute_instruction(instruction);
        assert_eq!(0x01, cpu.A);
        assert_eq!(false, cpu.C);
        assert_eq!(false, cpu.Z);
        assert_eq!(false, cpu.V);
        assert_eq!(false, cpu.N);
        // 0x40 + 0x3F no carry
        cpu.A = 0x40;
        let op = 0x3F;
        cpu.C = false;
        let instruction = Instruction::ADC(AddrMode::Immediate(op));
        cpu.execute_instruction(instruction);
        assert_eq!(0x7F, cpu.A, "0x40 + 0x3F no carry: wrong result");
        assert_eq!(false, cpu.C, "0x40 + 0x3F no carry: wrong carry flag");
        assert_eq!(false, cpu.Z, "0x40 + 0x3F no carry: wrong zero flag");
        assert_eq!(false, cpu.V, "0x40 + 0x3F no carry: wrong overflow flag");
        assert_eq!(false, cpu.N, "0x40 + 0x3F no carry: wrong negative flag");
        // 0x40 + 0x40 no carry
        cpu.A = 0x40;
        let op = 0x40;
        cpu.C = false;
        let instruction = Instruction::ADC(AddrMode::Immediate(op));
        cpu.execute_instruction(instruction);
        assert_eq!(0x80, cpu.A, "0x40 + 0x40 no carry: wrong result");
        assert_eq!(false, cpu.C, "0x40 + 0x40 no carry: wrong carry flag");
        assert_eq!(false, cpu.Z, "0x40 + 0x40 no carry: wrong zero flag");
        assert_eq!(true, cpu.V, "0x40 + 0x40 no carry: wrong overflow flag");
        assert_eq!(true, cpu.N, "0x40 + 0x40 no carry: wrong negative flag");
        // 0x80 + 0x7F no carry
        cpu.A = 0x80;
        let op = 0x7F;
        cpu.C = false;
        let instruction = Instruction::ADC(AddrMode::Immediate(op));
        cpu.execute_instruction(instruction);
        assert_eq!(0xFF, cpu.A, "0x80 + 0x7F no carry: wrong result");
        assert_eq!(false, cpu.C, "0x80 + 0x7F no carry: wrong carry flag");
        assert_eq!(false, cpu.Z, "0x80 + 0x7F no carry: wrong zero flag");
        assert_eq!(false, cpu.V, "0x80 + 0x7F no carry: wrong overflow flag");
        assert_eq!(true, cpu.N, "0x80 + 0x7F no carry: wrong negative flag");
        // 0x80 + 0x80 no carry
        cpu.A = 0x80;
        let op = 0x80;
        cpu.C = false;
        let instruction = Instruction::ADC(AddrMode::Immediate(op));
        cpu.execute_instruction(instruction);
        assert_eq!(0x00, cpu.A, "0x80 + 0x80 no carry: wrong result");
        assert_eq!(true, cpu.C, "0x80 + 0x80 no carry: wrong carry flag");
        assert_eq!(true, cpu.Z, "0x80 + 0x80 no carry: wrong zero flag");
        assert_eq!(true, cpu.V, "0x80 + 0x80 no carry: wrong overflow flag");
        assert_eq!(false, cpu.N, "0x80 + 0x80 no carry: wrong negative flag");
    }

    // flag instructions

    #[test]
    fn test_clc() {
        let mut cpu: CPU = CPU::new();
        cpu.C = true;
        let instruction = Instruction::CLC;
        cpu.execute_instruction(instruction);
        assert_eq!(cpu.C, false);
    }

    #[test]
    fn test_sec() {
        let mut cpu: CPU = CPU::new();
        cpu.C = false;
        let instruction = Instruction::SEC;
        cpu.execute_instruction(instruction);
        assert_eq!(cpu.C, true);
    }

#[test]
    fn test_cli() {
        let mut cpu: CPU = CPU::new();
        cpu.I = true;
        let instruction = Instruction::CLI;
        cpu.execute_instruction(instruction);
        assert_eq!(cpu.I, false);
    }

    #[test]
    fn test_sei() {
        let mut cpu: CPU = CPU::new();
        cpu.I = false;
        let instruction = Instruction::SEI;
        cpu.execute_instruction(instruction);
        assert_eq!(cpu.I, true);
    }

#[test]
    fn test_cld() {
        let mut cpu: CPU = CPU::new();
        cpu.D = true;
        let instruction = Instruction::CLD;
        cpu.execute_instruction(instruction);
        assert_eq!(cpu.D, false);
    }

    #[test]
    fn test_sed() {
        let mut cpu: CPU = CPU::new();
        cpu.D = false;
        let instruction = Instruction::SED;
        cpu.execute_instruction(instruction);
        assert_eq!(cpu.D, true);
    }

#[test]
    fn test_clv() {
        let mut cpu: CPU = CPU::new();
        cpu.V = true;
        let instruction = Instruction::CLV;
        cpu.execute_instruction(instruction);
        assert_eq!(cpu.V, false);
    }


}

