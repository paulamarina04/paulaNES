use super::instruction_set::Instruction;
use super::instruction_set::AddrMode;
use std::num::Wrapping;

impl super::CPU {
    pub(crate) fn execute_instruction(&mut self, instruction : Instruction) {
        match instruction  {
            //access
            Instruction::LDA(addr_mode) => {
                let val = self.get_addressed_value(addr_mode);
                self.A = val;
                self.update_nz_flags(val);
            },
            Instruction::STA(addr_mode) => {
                let (addr_hi, addr_lo) = self.get_addressed_address(addr_mode);
                let data = self.A;
                self.bus.write(addr_hi, addr_lo, data);
            }
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
                let val2 = self.get_addressed_value(addr_mode);
                let carry = if self.C { 0x01 } else { 0x00 };
                let result = Wrapping(val1) + Wrapping(val2) + Wrapping(carry);
                let result: u8 = result.0;
                self.A = result;
                // flag bitwise shenanigans
                self.C = val1 as u16 + val2 as u16 + carry as u16 > 0xFF;
                self.V = (result ^ val1) & (result ^ val2) & 0b10000000 == 0b10000000;
                self.update_nz_flags(result);
            },
            Instruction::SBC(addr_mode) => {
                let val1 = self.A;
                let val2 = self.get_addressed_value(addr_mode);
                let val2 = !val2; // only change from ADC
                let carry = if self.C { 0x01 } else { 0x00 };
                let result = Wrapping(val1) + Wrapping(val2) + Wrapping(carry);
                let result: u8 = result.0;
                self.A = result;
                // flag bitwise shenanigans
                self.C = val1 as u16 + val2 as u16 + carry as u16 > 0xFF;
                self.V = (result ^ val1) & (result ^ val2) & 0b10000000 == 0b10000000;
                self.update_nz_flags(result);
            },
            Instruction::INX => {
                let result = self.X + 1;
                self.X = result;
                self.update_nz_flags(result);
            },
            Instruction::DEX => {
                let result = self.X - 1;
                self.X = result;
                self.update_nz_flags(result);
            },
            Instruction::INY => {
                let result = self.Y + 1;
                self.Y = result;
                self.update_nz_flags(result);
            },
            Instruction::DEY => {
                let result = self.Y - 1;
                self.Y = result;
                self.update_nz_flags(result);
            },
            //biwise
            Instruction::AND(addr_mode) => {
                let val1 = self.A;
                let val2 = self.get_addressed_value(addr_mode);
                let result = val1 & val2;
                self.A = result;
                self.update_nz_flags(result);
            },
            Instruction::ORA(addr_mode) => {
                let val1 = self.A;
                let val2 = self.get_addressed_value(addr_mode);
                let result = val1 | val2;
                self.A = result;
                self.update_nz_flags(result);
            },
            Instruction::XOR(addr_mode) => {
                let val1 = self.A;
                let val2 = self.get_addressed_value(addr_mode);
                let result = val1 ^ val2;
                self.A = result;
                self.update_nz_flags(result);
            },
            /*Instruction::BIT(addr_mode) => {
                let val1 = self.A;
                let val2 = get_addressed_val(addr_mode);
                let result = val1 & val2;
                update_nz_flags(self, result);
            },*/
            //jump
            Instruction::JMP(addr_mode) => {
                let (val_hi, val_lo) = self.get_addressed_address(addr_mode);
                self.PC_hi = val_hi;
                self.PC_lo = val_lo;
            },
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

    fn get_addressed_value(&self, addr_mode: AddrMode) -> u8 {
        return match addr_mode {
            AddrMode::Immediate(op) => { 
                op
            },
            _ => {
                let (hi, lo) = self.get_addressed_address(addr_mode);
                self.bus.read(hi, lo)
            }
        };
    }

    fn get_addressed_address(&self, addr_mode: AddrMode) -> (u8, u8) {
        return match addr_mode {
            AddrMode::Immediate(_) => {
                panic!("Attempted to fetch an address using an immediate addressing mode");
            },
            AddrMode::Absolute(hi,lo ) => {
                (hi, lo)
            },
            AddrMode::AbsoluteIndexedX(hi,lo ) => {
                let indexed_lo = (Wrapping(lo) + Wrapping(self.X)).0;
                let mut indexed_hi = hi;
                if lo > indexed_lo { 
                    // page crossed
                    indexed_hi = (Wrapping(hi) + Wrapping(1)).0;
                }
                (indexed_hi, indexed_lo)
            },
            AddrMode::AbsoluteIndexedY(hi,lo ) => {
                let indexed_lo = (Wrapping(lo) + Wrapping(self.Y)).0;
                let mut indexed_hi = hi;
                if lo > indexed_lo { 
                    // page crossed
                    indexed_hi = (Wrapping(hi) + Wrapping(1)).0;
                }
                (indexed_hi, indexed_lo)
            },
            AddrMode::ZeroPage(lo) => {
                (0x00, lo)
            }
        };
    }

    fn update_nz_flags(&mut self, result: u8) {
        self.Z = result == 0x00;
        self.N = result & 0x80 == 0x80;
    }
}

#[cfg(test)]
mod test_addressing_modes {
    use crate::cpu::CPU;
    use crate::cpu::instruction_set::AddrMode;

    // addressed values

    #[test]
    fn test_immediate_val() {
        let cpu = CPU::new();
        let val = 0xFF;
        let addr_mode = AddrMode::Immediate(val);
        assert_eq!(0xFF, cpu.get_addressed_value(addr_mode));
    }

    #[test]
    fn test_absolute_val() {
        let mut cpu = CPU::new();
        let hi = 0x80;
        let lo = 0x01;
        let val = 0xFF;
        cpu.bus.write(hi, lo, val);
        let addr_mode = AddrMode::Absolute(hi, lo);
        assert_eq!(0xFF, cpu.get_addressed_value(addr_mode));
    }
    
    #[test]
    fn test_absolute_indexed_x_value() {
        let mut cpu = CPU::new();
        // no page cross
        let hi = 0x80;
        let lo = 0x01;
        let val = 0xFF;
        cpu.X = 0x80;
        cpu.bus.write(hi, 0x81, val);
        let addr_mode = AddrMode::AbsoluteIndexedX(hi, lo);
        assert_eq!(0xFF, cpu.get_addressed_value(addr_mode));
        // page crossed
        let hi = 0x80;
        let lo = 0x80;
        let val = 0xFE;
        cpu.X = 0xFF;
        cpu.bus.write(0x81, 0x7F, val);
        let addr_mode = AddrMode::AbsoluteIndexedX(hi, lo);
        assert_eq!(0xFE, cpu.get_addressed_value(addr_mode));
        // page wrap
        let hi = 0xFF;
        let lo = 0xFF;
        let val = 0xFD;
        cpu.X = 0x01;
        cpu.bus.write(0x00, 0x00, val);
        let addr_mode = AddrMode::AbsoluteIndexedX(hi, lo);
        assert_eq!(0xFD, cpu.get_addressed_value(addr_mode));
    }
    
    #[test]
    fn test_absolute_indexed_y_value() {
        let mut cpu = CPU::new();
        // no page cross
        let hi = 0x80;
        let lo = 0x01;
        let val = 0xFF;
        cpu.Y = 0x80;
        cpu.bus.write(hi, 0x81, val);
        let addr_mode = AddrMode::AbsoluteIndexedY(hi, lo);
        assert_eq!(0xFF, cpu.get_addressed_value(addr_mode));
        // page crossed
        let hi = 0x80;
        let lo = 0x80;
        let val = 0xFE;
        cpu.Y = 0xFF;
        cpu.bus.write(0x81, 0x7F, val);
        let addr_mode = AddrMode::AbsoluteIndexedY(hi, lo);
        assert_eq!(0xFE, cpu.get_addressed_value(addr_mode));
        // page wrap
        let hi = 0xFF;
        let lo = 0xFF;
        let val = 0xFD;
        cpu.Y = 0x01;
        cpu.bus.write(0x00, 0x00, val);
        let addr_mode = AddrMode::AbsoluteIndexedY(hi, lo);
        assert_eq!(0xFD, cpu.get_addressed_value(addr_mode));
    }
    
    #[test]
    fn test_zero_page_value() {
        let mut cpu = CPU::new();
        let hi = 0x00;
        let lo = 0x80;
        let val = 0xFF;
        cpu.bus.write(hi, lo, val);
        let addr_mode = AddrMode::ZeroPage(lo);
        assert_eq!(0xFF, cpu.get_addressed_value(addr_mode));
    }

    // addressed addresses

    #[test]
    fn test_absolute_address() {
        let cpu = CPU::new();
        let hi = 0x80;
        let lo = 0x01;
        let addr_mode = AddrMode::Absolute(hi, lo);
        assert_eq!((0x80, 0x01), cpu.get_addressed_address(addr_mode));
    }

    #[test]
    fn test_absolute_indexed_x_address() {
        let mut cpu = CPU::new();
        // no page cross
        let hi = 0x80;
        let lo = 0x01;
        cpu.X = 0x80;
        let addr_mode = AddrMode::AbsoluteIndexedX(hi, lo);
        assert_eq!((0x80, 0x81), cpu.get_addressed_address(addr_mode));
        // page crossed
        let hi = 0x80;
        let lo = 0x80;
        cpu.X = 0xFF;
        let addr_mode = AddrMode::AbsoluteIndexedX(hi, lo);
        assert_eq!((0x81, 0x7F), cpu.get_addressed_address(addr_mode));
        // page wrap
        let hi = 0xFF;
        let lo = 0xFF;
        cpu.X = 0x01;
        let addr_mode = AddrMode::AbsoluteIndexedX(hi, lo);
        assert_eq!((0x00, 0x00), cpu.get_addressed_address(addr_mode));
    }

    #[test]
    fn test_absolute_indexed_y_address() {
        let mut cpu = CPU::new();
        // no page cross
        let hi = 0x80;
        let lo = 0x01;
        cpu.Y = 0x80;
        let addr_mode = AddrMode::AbsoluteIndexedY(hi, lo);
        assert_eq!((0x80, 0x81), cpu.get_addressed_address(addr_mode));
        // page crossed
        let hi = 0x80;
        let lo = 0x80;
        cpu.Y = 0xFF;
        let addr_mode = AddrMode::AbsoluteIndexedY(hi, lo);
        assert_eq!((0x81, 0x7F), cpu.get_addressed_address(addr_mode));
        // page wrap
        let hi = 0xFF;
        let lo = 0xFF;
        cpu.Y = 0x01;
        let addr_mode = AddrMode::AbsoluteIndexedY(hi, lo);
        assert_eq!((0x00, 0x00), cpu.get_addressed_address(addr_mode));
    }

    #[test]
    fn test_zero_page_address() {
        let cpu = CPU::new();
        let lo = 0x80;
        let addr_mode = AddrMode::ZeroPage(lo);
        assert_eq!((0x0, 0x80), cpu.get_addressed_address(addr_mode));
    }
}