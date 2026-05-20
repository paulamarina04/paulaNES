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
            // jump
            Instruction::JMP(addr_mode) => {
                let (val_hi, val_lo) = self.get_addressed_address(addr_mode);
                self.PC_hi = val_hi;
                self.PC_lo = val_lo;
            },
            // stack
            Instruction::PHA => {
                let val = self.A;
                let addr_lo = self.S;
                let addr_hi = 0x01;
                self.bus.write(addr_hi, addr_lo, val);
                self.S = (Wrapping(self.S) - Wrapping(1)).0;
            },
            Instruction::PLA => {
                self.S = (Wrapping(self.S) + Wrapping(1)).0;
                let addr_lo = self.S;
                let addr_hi = 0x01;
                let popped_val = self.bus.read(addr_hi, addr_lo);
                self.A = popped_val;
                self.update_nz_flags(popped_val);            
            },
            Instruction::PHP => {
                let n_bit = if self.N { 0x80 } else { 0x00 };
                let v_bit = if self.V { 0x40 } else { 0x00 };
                let extra_and_b_bits = 0x30;
                let d_bit = if self.D { 0x08 } else { 0x00 };
                let i_bit = if self.I { 0x04 } else { 0x00 };
                let z_bit = if self.Z { 0x02 } else { 0x00 };
                let c_bit = if self.C { 0x01 } else { 0x00 };
                let pushed_val = n_bit | v_bit | extra_and_b_bits | d_bit | i_bit | z_bit | c_bit;

                let addr_lo = self.S;
                let addr_hi = 0x01;
                self.bus.write(addr_hi, addr_lo, pushed_val);
                self.S = (Wrapping(self.S) - Wrapping(1)).0;
            },
            Instruction::PLP => {
                self.S = (Wrapping(self.S) + Wrapping(1)).0;
                let addr_lo = self.S;
                let addr_hi = 0x01;
                let popped_val = self.bus.read(addr_hi, addr_lo);

                let n_bit = popped_val & 0x80;
                let v_bit = popped_val & 0x40;
                let d_bit = popped_val & 0x08;
                let i_bit = popped_val & 0x04;
                let z_bit = popped_val & 0x02;
                let c_bit = popped_val & 0x01; 
                self.N = n_bit != 0x00;  
                self.V = v_bit != 0x00;  
                self.D = d_bit != 0x00;  
                self.I = i_bit != 0x00;  
                self.Z = z_bit != 0x00;  
                self.C = c_bit != 0x00;       
            },
            // flags
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

    pub(crate) fn get_addressed_value(&self, addr_mode: AddrMode) -> u8 {
        return match addr_mode {
            AddrMode::Indirect(..) => {
                panic!("Attempted to fetch a value using a plain indirect addressing mode");
            }
            AddrMode::Immediate(op) => { 
                op
            },
            _ => {
                let (hi, lo) = self.get_addressed_address(addr_mode);
                self.bus.read(hi, lo)
            }
        };
    }

    pub(crate) fn get_addressed_address(&self, addr_mode: AddrMode) -> (u8, u8) {
        return match addr_mode {
            AddrMode::Immediate(..) => {
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
            },
            AddrMode::ZeroPageIndexedX(lo) => {
                let indexed_lo = (Wrapping(lo) + Wrapping(self.X)).0;
                (0x00, indexed_lo)
            },
            AddrMode::ZeroPageIndexedY(lo) => {
                let indexed_lo = (Wrapping(lo) + Wrapping(self.Y)).0;
                (0x00, indexed_lo)
            },
            AddrMode::Indirect(hi, lo) => {
                let indirect_lo = self.bus.read(hi, lo); // little endian; low byte first
                let next_lo = (Wrapping(lo) + Wrapping(1)).0;
                let indirect_hi = self.bus.read(hi, next_lo); // cpu doesnt check for page cross!!!
                (indirect_hi, indirect_lo)
            },
            AddrMode::IndexedIndirect(lo) => {
                let indexed_lo = (Wrapping(lo) + Wrapping(self.X)).0;
                let indexed_lo_next = (Wrapping(indexed_lo) + Wrapping(1)).0;
                let indirect_lo = self.bus.read(0x00, indexed_lo);
                let indirect_hi = self.bus.read(0x00, indexed_lo_next);
                (indirect_hi, indirect_lo) 
            },
            AddrMode::IndirectIndexed(lo) => {
                let lo_next = (Wrapping(lo) + Wrapping(1)).0;
                let indirect_lo = self.bus.read(0x00, lo);
                let indirect_hi = self.bus.read(0x00, lo_next);

                let indexed_lo = (Wrapping(indirect_lo) + Wrapping(self.Y)).0;
                let indexed_hi = if indirect_lo > indexed_lo { 
                    (Wrapping(indirect_hi) + Wrapping(1)).0 // page cross
                } else {
                    indirect_hi
                };

                (indexed_hi, indexed_lo)
            }
        };
    }

    fn update_nz_flags(&mut self, result: u8) {
        self.Z = result == 0x00;
        self.N = result & 0x80 == 0x80;
    }
}
