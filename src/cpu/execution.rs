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
            Instruction::LDX(addr_mode) => {
                let val = self.get_addressed_value(addr_mode);
                self.X = val;
                self.update_nz_flags(val);
            },
            Instruction::STX(addr_mode) => {
                let (addr_hi, addr_lo) = self.get_addressed_address(addr_mode);
                let data = self.X;
                self.bus.write(addr_hi, addr_lo, data);
            }
            Instruction::LDY(addr_mode) => {
                let val = self.get_addressed_value(addr_mode);
                self.Y = val;
                self.update_nz_flags(val);
            },
            Instruction::STY(addr_mode) => {
                let (addr_hi, addr_lo) = self.get_addressed_address(addr_mode);
                let data = self.Y;
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
            Instruction::INC(addr_mode) => {
                let (addr_hi, addr_lo) = self.get_addressed_address(addr_mode);
                let value = self.bus.read(addr_hi, addr_lo);
                let result = value.wrapping_add(1);
                self.bus.write(addr_hi, addr_lo, value); // RMW shenanigans, og value written first
                self.bus.write(addr_hi, addr_lo, result); 
                self.update_nz_flags(result);
            },
            Instruction::INX => {
                let result = self.X.wrapping_add(1);
                self.X = result;
                self.update_nz_flags(result);
            },
            Instruction::DEX => {
                let result = self.X.wrapping_sub(1);
                self.X = result;
                self.update_nz_flags(result);
            },
            Instruction::INY => {
                let result = self.Y.wrapping_add(1);
                self.Y = result;
                self.update_nz_flags(result);
            },
            Instruction::DEY => {
                let result = self.Y.wrapping_sub(1);
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
            Instruction::BIT(addr_mode) => {
                let val1 = self.A;
                let val2 = self.get_addressed_value(addr_mode);
                let result = val1 & val2;
                self.V = result & 0x40 == 0x40;
                self.update_nz_flags(result);
            },
            // compare
            Instruction::CMP(addr_mode) => {
                let val1 = self.A;
                let val2 = self.get_addressed_value(addr_mode);
                let val2 = (Wrapping(!val2) + Wrapping(1)).0; // 2s complement
                let result = (Wrapping(val1) + Wrapping(val2)).0;
                self.C = val1 as u16 + val2 as u16 > 0xFF;
                self.update_nz_flags(result);
            },
            Instruction::CPX(addr_mode) => {
                let val1 = self.X;
                let val2 = self.get_addressed_value(addr_mode);
                let val2 = (Wrapping(!val2) + Wrapping(1)).0; // 2s complement
                let result = (Wrapping(val1) + Wrapping(val2)).0;
                self.C = val1 as u16 + val2 as u16 > 0xFF;
                self.update_nz_flags(result);
            },
            Instruction::CPY(addr_mode) => {
                let val1 = self.Y;
                let val2 = self.get_addressed_value(addr_mode);
                let val2 = (Wrapping(!val2) + Wrapping(1)).0; // 2s complement
                let result = (Wrapping(val1) + Wrapping(val2)).0;
                self.C = val1 as u16 + val2 as u16 > 0xFF;
                self.update_nz_flags(result);
            },
            // branch
            Instruction::BCC(offset) => {
                if self.C { return; }
                self.branch_with_offset(offset);
            },
            Instruction::BCS(offset) => {
                if !self.C { return; }
                self.branch_with_offset(offset);
            },
            Instruction::BNE(offset) => {
                if self.Z { return; }
                self.branch_with_offset(offset);
            },
            Instruction::BEQ(offset) => {
                if !self.Z { return; }
                self.branch_with_offset(offset);
            },
            Instruction::BPL(offset) => {
                if self.N { return; }
                self.branch_with_offset(offset);
            },
            Instruction::BMI(offset) => {
                if !self.N { return; }
                self.branch_with_offset(offset);
            },
            Instruction::BVC(offset) => {
                if self.V { return; }
                self.branch_with_offset(offset);
            },
            Instruction::BVS(offset) => {
                if !self.V { return; }
                self.branch_with_offset(offset);
            },
            // jump
            Instruction::JMP(addr_mode) => {
                let (val_hi, val_lo) = self.get_addressed_address(addr_mode);
                self.PC_hi = val_hi;
                self.PC_lo = val_lo;
            },
            Instruction::JSR(addr_mode) => {
                let (sub_hi, sub_lo) = self.get_addressed_address(addr_mode);
                let ret_lo = self.PC_lo.wrapping_add(2);
                let ret_hi = if ret_lo < self.PC_lo {
                    self.PC_hi.wrapping_add(1)
                } else {
                    self.PC_hi
                };
                // push ret addr to stack
                self.bus.write(0x01, self.S, ret_hi);
                self.S = self.S.wrapping_sub(1);
                self.bus.write(0x01, self.S, ret_lo);
                self.S = self.S.wrapping_sub(1);
                // jump to subroutine
                self.PC_hi = sub_hi;
                self.PC_lo = sub_lo;
            },
            Instruction::RTS => {
                // pop ret addr from stack
                self.S = self.S.wrapping_add(1);
                let popped_lo = self.bus.read(0x01, self.S);
                self.S = self.S.wrapping_add(1);
                let popped_hi = self.bus.read(0x01, self.S);
                // add 1 to return address, for some reason
                let ret_lo = popped_lo.wrapping_add(1);
                let ret_hi = if ret_lo < popped_lo {
                    popped_hi.wrapping_add(1)
                } else {
                    popped_hi
                };
                // jump to return address
                self.PC_lo = ret_lo;
                self.PC_hi = ret_hi;
            },
            Instruction::BRK => {
                // get interrupt jump addr 
                let int_lo = self.bus.read(0xFF, 0xFE);
                let int_hi = self.bus.read(0xFF, 0xFF);
                // get return address 
                let ret_lo = self.PC_lo.wrapping_add(2);
                let ret_hi = if ret_lo < self.PC_lo {
                    self.PC_hi.wrapping_add(1)
                } else {
                    self.PC_hi
                };
                // get flags value
                let n_bit = if self.N { 0x80 } else { 0x00 };
                let v_bit = if self.V { 0x40 } else { 0x00 };
                let extra_and_b_bits = 0x30; // b bit is set (unlike hardware interrupt)
                let d_bit = if self.D { 0x08 } else { 0x00 };
                let i_bit = if self.I { 0x04 } else { 0x00 };
                let z_bit = if self.Z { 0x02 } else { 0x00 };
                let c_bit = if self.C { 0x01 } else { 0x00 };
                let pushed_flags = n_bit | v_bit | extra_and_b_bits | d_bit | i_bit | z_bit | c_bit;
                // push ret addr and flags val to stack
                self.bus.write(0x01, self.S, ret_hi);
                self.S = self.S.wrapping_sub(1);
                self.bus.write(0x01, self.S, ret_lo);
                self.S = self.S.wrapping_sub(1);
                self.bus.write(0x01, self.S, pushed_flags);
                self.S = self.S.wrapping_sub(1);
                // jump to subroutine
                self.PC_hi = int_hi;
                self.PC_lo = int_lo;
            },
            Instruction::RTI => {
                // pop flags and ret addr from stack
                self.S = self.S.wrapping_add(1);
                let popped_flags = self.bus.read(0x01, self.S);
                self.S = self.S.wrapping_add(1);
                let ret_lo = self.bus.read(0x01, self.S);
                self.S = self.S.wrapping_add(1);
                let ret_hi = self.bus.read(0x01, self.S);
                // restore flags
                let n_bit = popped_flags & 0x80;
                let v_bit = popped_flags & 0x40;
                let d_bit = popped_flags & 0x08;
                let i_bit = popped_flags & 0x04;
                let z_bit = popped_flags & 0x02;
                let c_bit = popped_flags & 0x01; 
                self.N = n_bit != 0x00;  
                self.V = v_bit != 0x00;  
                self.D = d_bit != 0x00;  
                self.I = i_bit != 0x00;  
                self.Z = z_bit != 0x00;  
                self.C = c_bit != 0x00;   
                // jump to return address
                self.PC_lo = ret_lo;
                self.PC_hi = ret_hi;
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

    fn branch_with_offset(&mut self, offset: i8) {
        let total_offset = offset as i16 + 2;
        let offset_pc_lo = (self.PC_lo as i16 + total_offset) as u8; 
        let offset_pc_hi = 
        if total_offset > 0 && offset_pc_lo < self.PC_lo {
            self.PC_hi.wrapping_add(1) // page crossed (positive)
        } else if total_offset < 0 && offset_pc_lo > self.PC_lo {
            self.PC_hi.wrapping_sub(1) // page crossed (negative)
        } else {
            self.PC_hi
        };
        self.PC_lo = offset_pc_lo;
        self.PC_hi = offset_pc_hi;
    }
}
