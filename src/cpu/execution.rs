use crate::cpu::instruction_set::AddressAddrMode;

use super::instruction_set::Instruction;
use super::instruction_set::ValueAddrMode;
use std::num::Wrapping;
use std::ptr::addr_of;

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

    fn get_addressed_value(&self, addr_mode: ValueAddrMode) -> u8 {
        return match addr_mode {
            ValueAddrMode::Immediate(op) => { 
                op
            }
        }
    }

    fn get_addressed_address(&self, addr_mode: AddressAddrMode) -> (u8, u8) {
        return match addr_mode {
            AddressAddrMode::Absolute(hi,lo ) => {
                (hi, lo)
            }
        };
    }

    fn update_nz_flags(&mut self, result: u8) {
        self.Z = result == 0x00;
        self.N = result & 0x80 == 0x80;
    }
}