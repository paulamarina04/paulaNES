
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
        assert_eq!(false, cpu.Z);
        assert_eq!(true, cpu.N);
    }

    #[test]
    fn test_sta() {
        let mut cpu: CPU = CPU::new();
        let addr_hi = 0x00;
        let addr_lo = 0x00;
        cpu.A = 0xFF;
        let addr_mode = AddrMode::Absolute(addr_hi, addr_lo);
        let instruction = Instruction::STA(addr_mode);
        cpu.execute_instruction(instruction);
        assert_eq!(0xFF, cpu.bus.read(addr_hi, addr_lo));
    }
    
    #[test]
    fn test_ldx() {
        let mut cpu: CPU = CPU::new();
        let val: u8 = 0xFF;
        let addr_mode = AddrMode::Immediate(val);
        cpu.execute_instruction(Instruction::LDX(addr_mode));
        assert_eq!(val, cpu.X);
        assert_eq!(false, cpu.Z);
        assert_eq!(true, cpu.N);
    }

    #[test]
    fn test_stx() {
        let mut cpu: CPU = CPU::new();
        let addr_hi = 0x00;
        let addr_lo = 0x00;
        cpu.X = 0xFF;
        let addr_mode = AddrMode::Absolute(addr_hi, addr_lo);
        let instruction = Instruction::STX(addr_mode);
        cpu.execute_instruction(instruction);
        assert_eq!(0xFF, cpu.bus.read(addr_hi, addr_lo));
    }
    
    #[test]
    fn test_ldy() {
        let mut cpu: CPU = CPU::new();
        let val: u8 = 0xFF;
        let addr_mode = AddrMode::Immediate(val);
        cpu.execute_instruction(Instruction::LDY(addr_mode));
        assert_eq!(val, cpu.Y);
        assert_eq!(false, cpu.Z);
        assert_eq!(true, cpu.N);
    }

    #[test]
    fn test_sty() {
        let mut cpu: CPU = CPU::new();
        let addr_hi = 0x00;
        let addr_lo = 0x00;
        cpu.Y = 0xFF;
        let addr_mode = AddrMode::Absolute(addr_hi, addr_lo);
        let instruction = Instruction::STY(addr_mode);
        cpu.execute_instruction(instruction);
        assert_eq!(0xFF, cpu.bus.read(addr_hi, addr_lo));
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

    #[test]
    fn test_sbc() { // C and V flag behaviour is kinda unintuitive here
        let mut cpu = CPU::new();
        // 2 - 1 with carry
        cpu.A = 0x02;
        let op = 0x01;
        cpu.C = true;
        let instruction = Instruction::SBC(AddrMode::Immediate(op));
        cpu.execute_instruction(instruction);
        assert_eq!(0x01, cpu.A);
        assert_eq!(true, cpu.C);
        assert_eq!(false, cpu.Z);
        assert_eq!(false, cpu.V);
        assert_eq!(false, cpu.N);
        // 2 - 1 no carry
        cpu.A = 0x02;
        let op = 0x01;
        cpu.C = false;
        let instruction = Instruction::SBC(AddrMode::Immediate(op));
        cpu.execute_instruction(instruction);
        assert_eq!(0x00, cpu.A);
        assert_eq!(true, cpu.C);
        assert_eq!(true, cpu.Z);
        assert_eq!(false, cpu.V);
        assert_eq!(false, cpu.N);
        // 1 - 2 with carry
        cpu.A = 0x01;
        let op = 0x02;
        cpu.C = true;
        let instruction = Instruction::SBC(AddrMode::Immediate(op));
        cpu.execute_instruction(instruction);
        assert_eq!(0xFF, cpu.A);
        assert_eq!(false, cpu.C);
        assert_eq!(false, cpu.Z);
        assert_eq!(false, cpu.V);
        assert_eq!(true, cpu.N);
        // 1 - (-1) with carry
        cpu.A = 0x01;
        let op = 0xFF;
        cpu.C = true;
        let instruction = Instruction::SBC(AddrMode::Immediate(op));
        cpu.execute_instruction(instruction);
        assert_eq!(0x02, cpu.A);
        assert_eq!(false, cpu.C);
        assert_eq!(false, cpu.Z);
        assert_eq!(false, cpu.V);
        assert_eq!(false, cpu.N);
        // (-1) - 1 with carry
        cpu.A = 0xFF;
        let op = 0x01;
        cpu.C = true;
        let instruction = Instruction::SBC(AddrMode::Immediate(op));
        cpu.execute_instruction(instruction);
        assert_eq!(0xFE, cpu.A);
        assert_eq!(true, cpu.C);
        assert_eq!(false, cpu.Z);
        assert_eq!(false, cpu.V);
        assert_eq!(true, cpu.N);
    }

    #[test]
    fn test_inc() {
        let mut cpu = CPU::new();
        // no overflow
        let val = 0x10;
        cpu.bus.write(0x00, 0x80, val);
        let addr_mode = AddrMode::ZeroPage(0x80);
        let instruction = Instruction::INC(addr_mode);
        cpu.execute_instruction(instruction);
        assert_eq!(0x11, cpu.bus.read(0x00, 0x80));
        assert_eq!(false, cpu.Z);
        assert_eq!(false, cpu.N);
        // overflow
        let val = 0xFF;
        cpu.C = false;
        cpu.V = false;
        cpu.bus.write(0x00, 0x80, val);
        let addr_mode = AddrMode::ZeroPage(0x80);
        let instruction = Instruction::INC(addr_mode);
        cpu.execute_instruction(instruction);
        assert_eq!(0x00, cpu.bus.read(0x00, 0x80));
        assert_eq!(true, cpu.Z);
        assert_eq!(false, cpu.N);
        assert_eq!(false, cpu.C);
        assert_eq!(false, cpu.V);
    }

    #[test]
    fn test_inx() {
        let mut cpu = CPU::new();
        cpu.X = 0x00;
        let instruction = Instruction::INX;
        cpu.execute_instruction(instruction);
        assert_eq!(0x01, cpu.X);
        assert_eq!(false, cpu.Z);
        assert_eq!(false, cpu.N);
    }
    #[test]
    fn test_dex() {
        let mut cpu = CPU::new();
        cpu.X = 0x01;
        let instruction = Instruction::DEX;
        cpu.execute_instruction(instruction);
        assert_eq!(0x00, cpu.X);
        assert_eq!(true, cpu.Z);
        assert_eq!(false, cpu.N);
    }
    #[test]
    fn test_iny() {
        let mut cpu = CPU::new();
        cpu.Y = 0x00;
        let instruction = Instruction::INY;
        cpu.execute_instruction(instruction);
        assert_eq!(0x01, cpu.Y);
        assert_eq!(false, cpu.Z);
        assert_eq!(false, cpu.N);
    }
    #[test]
    fn test_dey() {
        let mut cpu = CPU::new();
        cpu.Y = 0x01;
        let instruction = Instruction::DEY;
        cpu.execute_instruction(instruction);
        assert_eq!(0x00, cpu.Y);
        assert_eq!(true, cpu.Z);
        assert_eq!(false, cpu.N);
    }


    // bitwise instructions

    #[test]
    fn test_and() {
        let mut cpu: CPU = CPU::new();
        let val1: u8 = 0b11001100;
        let val2: u8 = 0b10101010;
        cpu.A = val1;
        let instruction = Instruction::AND(AddrMode::Immediate(val2));
        cpu.execute_instruction(instruction);
        assert_eq!(0b10001000, cpu.A);
        assert_eq!(false, cpu.Z);
        assert_eq!(true, cpu.N);
    }

    #[test]
    fn test_ora() {
        let mut cpu: CPU = CPU::new();
        let val1: u8 = 0b11001100;
        let val2: u8 = 0b10101010;
        cpu.A = val1;
        let instruction = Instruction::ORA(AddrMode::Immediate(val2));
        cpu.execute_instruction(instruction);
        assert_eq!(0b11101110, cpu.A);
        assert_eq!(false, cpu.Z);
        assert_eq!(true, cpu.N);
    }

    #[test]
    fn test_xor() {
        let mut cpu: CPU = CPU::new();
        let val1: u8 = 0b11001100;
        let val2: u8 = 0b10101010;
        cpu.A = val1;
        let instruction = Instruction::XOR(AddrMode::Immediate(val2));
        cpu.execute_instruction(instruction);
        assert_eq!(0b01100110, cpu.A);
        assert_eq!(false, cpu.Z);
        assert_eq!(false, cpu.N);
    }

    #[test]
    fn test_bit() {
        let mut cpu: CPU = CPU::new();
        // bit 7
        let val1: u8 = 0xFF;
        let val2: u8 = 0x80;
        cpu.A = val1;
        let instruction = Instruction::BIT(AddrMode::Immediate(val2));
        cpu.execute_instruction(instruction);
        assert_eq!(0xFF, cpu.A);
        assert_eq!(false, cpu.Z);
        assert_eq!(true, cpu.N);
        assert_eq!(false, cpu.V);
        // bit 6
        let val1: u8 = 0xFF;
        let val2: u8 = 0x40;
        cpu.A = val1;
        let instruction = Instruction::BIT(AddrMode::Immediate(val2));
        cpu.execute_instruction(instruction);
        assert_eq!(0xFF, cpu.A);
        assert_eq!(false, cpu.Z);
        assert_eq!(false, cpu.N);
        assert_eq!(true, cpu.V);
        // bits 5-0
        let val1: u8 = 0xFF;
        let val2: u8 = 0x3F;
        cpu.A = val1;
        let instruction = Instruction::BIT(AddrMode::Immediate(val2));
        cpu.execute_instruction(instruction);
        assert_eq!(0xFF, cpu.A);
        assert_eq!(false, cpu.Z);
        assert_eq!(false, cpu.N);
        assert_eq!(false, cpu.V);
        // bit not set
        cpu.A = 0x7F;
        let val2: u8 = 0x80;
        let instruction = Instruction::BIT(AddrMode::Immediate(val2));
        cpu.execute_instruction(instruction);
        assert_eq!(true, cpu.Z);
        assert_eq!(false, cpu.N);
        assert_eq!(false, cpu.V);
    }


    // compare instructions

    #[test]
    fn test_cmp() {
        let mut cpu = CPU::new();
        // A == val
        cpu.A = 0xFF;
        let val = 0xFF;
        let addr_mode = AddrMode::Immediate(val);
        let instruction = Instruction::CMP(addr_mode);
        cpu.execute_instruction(instruction);
        assert_eq!(true, cpu.Z);
        assert_eq!(true, cpu.C);
        assert_eq!(false, cpu.N);
        // A > val (signed & unsigned)
        cpu.A = 0x02;
        let val = 0x01;
        let addr_mode = AddrMode::Immediate(val);
        let instruction = Instruction::CMP(addr_mode);
        cpu.execute_instruction(instruction);
        assert_eq!(false, cpu.Z);
        assert_eq!(true, cpu.C);
        assert_eq!(false, cpu.N);
        // A > val (A < val for signed)
        cpu.A = 0xFF;
        let val = 0x01;
        let addr_mode = AddrMode::Immediate(val);
        let instruction = Instruction::CMP(addr_mode);
        cpu.execute_instruction(instruction);
        assert_eq!(false, cpu.Z);
        assert_eq!(true, cpu.C);
        assert_eq!(true, cpu.N);
        // A < val (signed & unsigned)
        cpu.A = 0x01;
        let val = 0x02;
        let addr_mode = AddrMode::Immediate(val);
        let instruction = Instruction::CMP(addr_mode);
        cpu.execute_instruction(instruction);
        assert_eq!(false, cpu.Z);
        assert_eq!(false, cpu.C);
        assert_eq!(true, cpu.N);
        // A < val (A > val for signed)
        cpu.A = 0x01;
        let val = 0xFF;
        let addr_mode = AddrMode::Immediate(val);
        let instruction = Instruction::CMP(addr_mode);
        cpu.execute_instruction(instruction);
        assert_eq!(false, cpu.Z);
        assert_eq!(false, cpu.C);
        assert_eq!(false, cpu.N);
    }

    #[test]
    fn test_cpx() {
        let mut cpu = CPU::new();
        // X == val
        cpu.X = 0xFF;
        let val = 0xFF;
        let addr_mode = AddrMode::Immediate(val);
        let instruction = Instruction::CPX(addr_mode);
        cpu.execute_instruction(instruction);
        assert_eq!(true, cpu.Z);
        assert_eq!(true, cpu.C);
        assert_eq!(false, cpu.N);
        // X > val (signed & unsigned)
        cpu.X = 0x02;
        let val = 0x01;
        let addr_mode = AddrMode::Immediate(val);
        let instruction = Instruction::CPX(addr_mode);
        cpu.execute_instruction(instruction);
        assert_eq!(false, cpu.Z);
        assert_eq!(true, cpu.C);
        assert_eq!(false, cpu.N);
        // X > val (X < val for signed)
        cpu.X = 0xFF;
        let val = 0x01;
        let addr_mode = AddrMode::Immediate(val);
        let instruction = Instruction::CPX(addr_mode);
        cpu.execute_instruction(instruction);
        assert_eq!(false, cpu.Z);
        assert_eq!(true, cpu.C);
        assert_eq!(true, cpu.N);
        // A < val (signed & unsigned)
        cpu.X = 0x01;
        let val = 0x02;
        let addr_mode = AddrMode::Immediate(val);
        let instruction = Instruction::CPX(addr_mode);
        cpu.execute_instruction(instruction);
        assert_eq!(false, cpu.Z);
        assert_eq!(false, cpu.C);
        assert_eq!(true, cpu.N);
        // X < val (X > val for signed)
        cpu.X = 0x01;
        let val = 0xFF;
        let addr_mode = AddrMode::Immediate(val);
        let instruction = Instruction::CPX(addr_mode);
        cpu.execute_instruction(instruction);
        assert_eq!(false, cpu.Z);
        assert_eq!(false, cpu.C);
        assert_eq!(false, cpu.N);
    }

    #[test]
    fn test_cpy() {
        let mut cpu = CPU::new();
        // Y == val
        cpu.Y = 0xFF;
        let val = 0xFF;
        let addr_mode = AddrMode::Immediate(val);
        let instruction = Instruction::CPY(addr_mode);
        cpu.execute_instruction(instruction);
        assert_eq!(true, cpu.Z);
        assert_eq!(true, cpu.C);
        assert_eq!(false, cpu.N);
        // Y > val (signed & unsigned)
        cpu.Y = 0x02;
        let val = 0x01;
        let addr_mode = AddrMode::Immediate(val);
        let instruction = Instruction::CPY(addr_mode);
        cpu.execute_instruction(instruction);
        assert_eq!(false, cpu.Z);
        assert_eq!(true, cpu.C);
        assert_eq!(false, cpu.N);
        // Y > val (Y < val for signed)
        cpu.Y = 0xFF;
        let val = 0x01;
        let addr_mode = AddrMode::Immediate(val);
        let instruction = Instruction::CPY(addr_mode);
        cpu.execute_instruction(instruction);
        assert_eq!(false, cpu.Z);
        assert_eq!(true, cpu.C);
        assert_eq!(true, cpu.N);
        // Y < val (signed & unsigned)
        cpu.Y = 0x01;
        let val = 0x02;
        let addr_mode = AddrMode::Immediate(val);
        let instruction = Instruction::CPY(addr_mode);
        cpu.execute_instruction(instruction);
        assert_eq!(false, cpu.Z);
        assert_eq!(false, cpu.C);
        assert_eq!(true, cpu.N);
        // A < val (A > val for signed)
        cpu.Y = 0x01;
        let val = 0xFF;
        let addr_mode = AddrMode::Immediate(val);
        let instruction = Instruction::CPY(addr_mode);
        cpu.execute_instruction(instruction);
        assert_eq!(false, cpu.Z);
        assert_eq!(false, cpu.C);
        assert_eq!(false, cpu.N);
    }


    // branch instructions

    #[test]
    fn test_bcc() {
        let mut cpu = CPU::new();
        // no branch taken
        cpu.C = true;
        cpu.PC_lo = 0x80;
        cpu.PC_hi = 0x80;
        let offset = 0x0F;
        let instruction = Instruction::BCC(offset);
        cpu.execute_instruction(instruction);
        assert_eq!(0x80, cpu.PC_lo);
        assert_eq!(0x80, cpu.PC_hi);
        // forward branch, no page cross
        cpu.C = false;
        cpu.PC_lo = 0x80;
        cpu.PC_hi = 0x80;
        let offset = 16;
        let instruction = Instruction::BCC(offset);
        cpu.execute_instruction(instruction);
        assert_eq!(0x92, cpu.PC_lo);
        assert_eq!(0x80, cpu.PC_hi);
        // backward branch, no page cross
        cpu.C = false;
        cpu.PC_lo = 0x80;
        cpu.PC_hi = 0x80;
        let offset = -16;
        let instruction = Instruction::BCC(offset);
        cpu.execute_instruction(instruction);
        assert_eq!(0x72, cpu.PC_lo);
        assert_eq!(0x80, cpu.PC_hi);
        // forward branch, forward page cross
        cpu.C = false;
        cpu.PC_lo = 0xF0;
        cpu.PC_hi = 0x80;
        let offset = 16;
        let instruction = Instruction::BCC(offset);
        cpu.execute_instruction(instruction);
        assert_eq!(0x02, cpu.PC_lo);
        assert_eq!(0x81, cpu.PC_hi);
        // backward branch, backward page cross
        cpu.C = false;
        cpu.PC_lo = 0x00;
        cpu.PC_hi = 0x80;
        let offset = -16;
        let instruction = Instruction::BCC(offset);
        cpu.execute_instruction(instruction);
        assert_eq!(0xF2, cpu.PC_lo);
        assert_eq!(0x7F, cpu.PC_hi);
        // -1 offset, forward page cross
        cpu.C = false;
        cpu.PC_lo = 0xFF;
        cpu.PC_hi = 0x80;
        let offset = -1;
        let instruction = Instruction::BCC(offset);
        cpu.execute_instruction(instruction);
        assert_eq!(0x00, cpu.PC_lo);
        assert_eq!(0x81, cpu.PC_hi);
        // max offset, no page cross
        cpu.C = false;
        cpu.PC_lo = 0x00;
        cpu.PC_hi = 0x80;
        let offset = 127;
        let instruction = Instruction::BCC(offset);
        cpu.execute_instruction(instruction);
        assert_eq!(0x81, cpu.PC_lo);
        assert_eq!(0x80, cpu.PC_hi);
        // max offset, forward page cross
        cpu.C = false;
        cpu.PC_lo = 0x80;
        cpu.PC_hi = 0x80;
        let offset = 127;
        let instruction = Instruction::BCC(offset);
        cpu.execute_instruction(instruction);
        assert_eq!(0x01, cpu.PC_lo);
        assert_eq!(0x81, cpu.PC_hi);
        // min offset, no page cross
        cpu.C = false;
        cpu.PC_lo = 0x80;
        cpu.PC_hi = 0x80;
        let offset = -128;
        let instruction = Instruction::BCC(offset);
        cpu.execute_instruction(instruction);
        assert_eq!(0x02, cpu.PC_lo);
        assert_eq!(0x80, cpu.PC_hi);
        // min offset, backward page cross
        cpu.C = false;
        cpu.PC_lo = 0x70;
        cpu.PC_hi = 0x80;
        let offset = -128;
        let instruction = Instruction::BCC(offset);
        cpu.execute_instruction(instruction);
        assert_eq!(0xF2, cpu.PC_lo);
        assert_eq!(0x7F, cpu.PC_hi);
        // forward bank wrap
        cpu.C = false;
        cpu.PC_lo = 0xFE;
        cpu.PC_hi = 0xFF;
        let offset = 0;
        let instruction = Instruction::BCC(offset);
        cpu.execute_instruction(instruction);
        assert_eq!(0x00, cpu.PC_lo);
        assert_eq!(0x00, cpu.PC_hi);
        // backward bank wrap
        cpu.C = false;
        cpu.PC_lo = 0x00;
        cpu.PC_hi = 0x00;
        let offset = -3;
        let instruction = Instruction::BCC(offset);
        cpu.execute_instruction(instruction);
        assert_eq!(0xFF, cpu.PC_lo);
        assert_eq!(0xFF, cpu.PC_hi);
    }

    #[test]
    fn test_bcs() {
        let mut cpu = CPU::new();
        // no branch taken
        cpu.C = false;
        cpu.PC_lo = 0x80;
        cpu.PC_hi = 0x80;
        let offset = 0x0F;
        let instruction = Instruction::BCS(offset);
        cpu.execute_instruction(instruction);
        assert_eq!(0x80, cpu.PC_lo);
        assert_eq!(0x80, cpu.PC_hi);
        // branch taken
        cpu.C = true;
        cpu.PC_lo = 0x80;
        cpu.PC_hi = 0x80;
        let offset = 16;
        let instruction = Instruction::BCS(offset);
        cpu.execute_instruction(instruction);
        assert_eq!(0x92, cpu.PC_lo);
        assert_eq!(0x80, cpu.PC_hi);
    }

    #[test]
    fn test_bne() {
        let mut cpu = CPU::new();
        // no branch taken
        cpu.Z = true;
        cpu.PC_lo = 0x80;
        cpu.PC_hi = 0x80;
        let offset = 0x0F;
        let instruction = Instruction::BNE(offset);
        cpu.execute_instruction(instruction);
        assert_eq!(0x80, cpu.PC_lo);
        assert_eq!(0x80, cpu.PC_hi);
        // branch taken
        cpu.Z = false;
        cpu.PC_lo = 0x80;
        cpu.PC_hi = 0x80;
        let offset = 16;
        let instruction = Instruction::BNE(offset);
        cpu.execute_instruction(instruction);
        assert_eq!(0x92, cpu.PC_lo);
        assert_eq!(0x80, cpu.PC_hi);
    }

    #[test]
    fn test_beq() {
        let mut cpu = CPU::new();
        // no branch taken
        cpu.Z = false;
        cpu.PC_lo = 0x80;
        cpu.PC_hi = 0x80;
        let offset = 0x0F;
        let instruction = Instruction::BEQ(offset);
        cpu.execute_instruction(instruction);
        assert_eq!(0x80, cpu.PC_lo);
        assert_eq!(0x80, cpu.PC_hi);
        // branch taken
        cpu.Z = true;
        cpu.PC_lo = 0x80;
        cpu.PC_hi = 0x80;
        let offset = 16;
        let instruction = Instruction::BEQ(offset);
        cpu.execute_instruction(instruction);
        assert_eq!(0x92, cpu.PC_lo);
        assert_eq!(0x80, cpu.PC_hi);
    }

    #[test]
    fn test_bpl() {
        let mut cpu = CPU::new();
        // no branch taken
        cpu.N = true;
        cpu.PC_lo = 0x80;
        cpu.PC_hi = 0x80;
        let offset = 0x0F;
        let instruction = Instruction::BPL(offset);
        cpu.execute_instruction(instruction);
        assert_eq!(0x80, cpu.PC_lo);
        assert_eq!(0x80, cpu.PC_hi);
        // branch taken
        cpu.N = false;
        cpu.PC_lo = 0x80;
        cpu.PC_hi = 0x80;
        let offset = 16;
        let instruction = Instruction::BPL(offset);
        cpu.execute_instruction(instruction);
        assert_eq!(0x92, cpu.PC_lo);
        assert_eq!(0x80, cpu.PC_hi);
    }

    #[test]
    fn test_bmi() {
        let mut cpu = CPU::new();
        // no branch taken
        cpu.N = false;
        cpu.PC_lo = 0x80;
        cpu.PC_hi = 0x80;
        let offset = 0x0F;
        let instruction = Instruction::BMI(offset);
        cpu.execute_instruction(instruction);
        assert_eq!(0x80, cpu.PC_lo);
        assert_eq!(0x80, cpu.PC_hi);
        // branch taken
        cpu.N = true;
        cpu.PC_lo = 0x80;
        cpu.PC_hi = 0x80;
        let offset = 16;
        let instruction = Instruction::BMI(offset);
        cpu.execute_instruction(instruction);
        assert_eq!(0x92, cpu.PC_lo);
        assert_eq!(0x80, cpu.PC_hi);
    }

    #[test]
    fn test_bvc() {
        let mut cpu = CPU::new();
        // no branch taken
        cpu.V = true;
        cpu.PC_lo = 0x80;
        cpu.PC_hi = 0x80;
        let offset = 0x0F;
        let instruction = Instruction::BVC(offset);
        cpu.execute_instruction(instruction);
        assert_eq!(0x80, cpu.PC_lo);
        assert_eq!(0x80, cpu.PC_hi);
        // branch taken
        cpu.V = false;
        cpu.PC_lo = 0x80;
        cpu.PC_hi = 0x80;
        let offset = 16;
        let instruction = Instruction::BVC(offset);
        cpu.execute_instruction(instruction);
        assert_eq!(0x92, cpu.PC_lo);
        assert_eq!(0x80, cpu.PC_hi);
    }

    #[test]
    fn test_bvs() {
        let mut cpu = CPU::new();
        // no branch taken
        cpu.V = false;
        cpu.PC_lo = 0x80;
        cpu.PC_hi = 0x80;
        let offset = 0x0F;
        let instruction = Instruction::BVS(offset);
        cpu.execute_instruction(instruction);
        assert_eq!(0x80, cpu.PC_lo);
        assert_eq!(0x80, cpu.PC_hi);
        // branch taken
        cpu.V = true;
        cpu.PC_lo = 0x80;
        cpu.PC_hi = 0x80;
        let offset = 16;
        let instruction = Instruction::BVS(offset);
        cpu.execute_instruction(instruction);
        assert_eq!(0x92, cpu.PC_lo);
        assert_eq!(0x80, cpu.PC_hi);
    }


    // jump instructions

    #[test]
    fn test_jmp() {
        let mut cpu = CPU::new();
        cpu.PC_hi = 0x00;
        cpu.PC_lo = 0x00;
        let val_hi = 0xFF;
        let val_lo = 0xFF;
        let addr_mode = AddrMode::Absolute(val_hi, val_lo);
        let instruction = Instruction::JMP(addr_mode);
        cpu.execute_instruction(instruction);
        assert_eq!(0xFF, cpu.PC_hi);
        assert_eq!(0xFF, cpu.PC_lo);
    }

    #[test]
    fn test_jsr() {
        let mut cpu = CPU::new();
        // no page cross
        cpu.S = 0xFF;
        cpu.PC_lo = 0x80;
        cpu.PC_hi = 0x80;
        let val_hi = 0xFF;
        let val_lo = 0xFF;
        let addr_mode = AddrMode::Absolute(val_hi, val_lo);
        let instruction = Instruction::JSR(addr_mode);
        cpu.execute_instruction(instruction);
        assert_eq!(0xFF, cpu.PC_hi);
        assert_eq!(0xFF, cpu.PC_lo);
        assert_eq!(0x82, cpu.bus.read(0x01, 0xFE));
        assert_eq!(0x80, cpu.bus.read(0x01, 0xFF));
        // return address page cross
        cpu.S = 0xFF;
        cpu.PC_lo = 0xFF;
        cpu.PC_hi = 0x80;
        let val_hi = 0xFF;
        let val_lo = 0xFF;
        let addr_mode = AddrMode::Absolute(val_hi, val_lo);
        let instruction = Instruction::JSR(addr_mode);
        cpu.execute_instruction(instruction);
        assert_eq!(0xFF, cpu.PC_hi);
        assert_eq!(0xFF, cpu.PC_lo);
        assert_eq!(0x01, cpu.bus.read(0x01, 0xFE));
        assert_eq!(0x81, cpu.bus.read(0x01, 0xFF));
    }

    #[test]
    fn test_rts() {
        let mut cpu = CPU::new();
        // no page cross
        cpu.S = 0xFD;
        cpu.PC_lo = 0x00;
        cpu.PC_hi = 0x00;
        let val_lo = 0x01;
        let val_hi = 0x80;
        cpu.bus.write(0x01, 0xFE, val_lo);
        cpu.bus.write(0x01, 0xFF, val_hi);
        let instruction = Instruction::RTS;
        cpu.execute_instruction(instruction);
        assert_eq!(0x02, cpu.PC_lo);
        assert_eq!(0x80, cpu.PC_hi);
        assert_eq!(0xFF, cpu.S);
        // page cross when incrementing return address
        cpu.S = 0xFD;
        cpu.PC_lo = 0x00;
        cpu.PC_hi = 0x00;
        let val_lo = 0xFF;
        let val_hi = 0x80;
        cpu.bus.write(0x01, 0xFE, val_lo);
        cpu.bus.write(0x01, 0xFF, val_hi);
        let instruction = Instruction::RTS;
        cpu.execute_instruction(instruction);
        assert_eq!(0x00, cpu.PC_lo);
        assert_eq!(0x81, cpu.PC_hi);
        assert_eq!(0xFF, cpu.S);
    }

    #[test]
    fn test_brk() {
        let mut cpu = CPU::new();
        // all flags clear, no page cross
        cpu.N = false;
        cpu.V = false;
        cpu.D = false;
        cpu.I = false;
        cpu.Z = false;
        cpu.C = false;
        cpu.S = 0xFF;
        cpu.PC_lo = 0x0F;
        cpu.PC_hi = 0x0F;
        let val_lo = 0x01;
        let val_hi = 0x80;
        cpu.bus.write(0xFF, 0xFE, val_lo);
        cpu.bus.write(0xFF, 0xFF, val_hi);
        let instruction = Instruction::BRK;
        cpu.execute_instruction(instruction);
        assert_eq!(0x01, cpu.PC_lo);
        assert_eq!(0x80, cpu.PC_hi);
        assert_eq!(0xFC, cpu.S);
        assert_eq!(0b00110000, cpu.bus.read(0x01, 0xFD));
        assert_eq!(0x11, cpu.bus.read(0x01, 0xFE));
        assert_eq!(0x0F, cpu.bus.read(0x01, 0xFF));
        // all flags set, page cross on ret addr
        cpu.N = true;
        cpu.V = true;
        cpu.D = true;
        cpu.I = true;
        cpu.Z = true;
        cpu.C = true;
        cpu.S = 0xFF;
        cpu.PC_lo = 0xFF;
        cpu.PC_hi = 0x0F;
        let val_lo = 0x01;
        let val_hi = 0x80;
        cpu.bus.write(0xFF, 0xFE, val_lo);
        cpu.bus.write(0xFF, 0xFF, val_hi);
        let instruction = Instruction::BRK;
        cpu.execute_instruction(instruction);
        assert_eq!(0x01, cpu.PC_lo);
        assert_eq!(0x80, cpu.PC_hi);
        assert_eq!(0xFC, cpu.S);
        assert_eq!(0xFF, cpu.bus.read(0x01, 0xFD));
        assert_eq!(0x01, cpu.bus.read(0x01, 0xFE));
        assert_eq!(0x10, cpu.bus.read(0x01, 0xFF));
    }

    #[test]
    fn test_rti() {
        let mut cpu = CPU::new();
        // all flags set
        cpu.S = 0xFC;
        cpu.PC_lo = 0x0F;
        cpu.PC_hi = 0x0F;
        let val_lo = 0x01;
        let val_hi = 0x80;
        let val_flags = 0xFF;
        cpu.bus.write(0x01, 0xFD, val_flags);
        cpu.bus.write(0x01, 0xFE, val_lo);
        cpu.bus.write(0x01, 0xFF, val_hi);
        let instruction = Instruction::RTI;
        cpu.execute_instruction(instruction);
        assert_eq!(0x01, cpu.PC_lo);
        assert_eq!(0x80, cpu.PC_hi);
        assert_eq!(0xFF, cpu.S);
        assert_eq!(true, cpu.N);
        assert_eq!(true, cpu.V);
        assert_eq!(true, cpu.D);
        assert_eq!(true, cpu.I);
        assert_eq!(true, cpu.Z);
        assert_eq!(true, cpu.C);
        // all flags clear
        cpu.S = 0xFC;
        cpu.PC_lo = 0x0F;
        cpu.PC_hi = 0x0F;
        let val_lo = 0x01;
        let val_hi = 0x80;
        let val_flags = 0b00110000;
        cpu.bus.write(0x01, 0xFD, val_flags);
        cpu.bus.write(0x01, 0xFE, val_lo);
        cpu.bus.write(0x01, 0xFF, val_hi);
        let instruction = Instruction::RTI;
        cpu.execute_instruction(instruction);
        assert_eq!(0x01, cpu.PC_lo);
        assert_eq!(0x80, cpu.PC_hi);
        assert_eq!(0xFF, cpu.S);
        assert_eq!(false, cpu.N);
        assert_eq!(false, cpu.V);
        assert_eq!(false, cpu.D);
        assert_eq!(false, cpu.I);
        assert_eq!(false, cpu.Z);
        assert_eq!(false, cpu.C);
    }

    // stack intructions

    #[test]
    fn test_pha() {
        let mut cpu = CPU::new();
        cpu.A = 0xFF;
        cpu.S = 0x80;
        let instruction = Instruction::PHA;
        cpu.execute_instruction(instruction);
        assert_eq!(0xFF, cpu.bus.read(0x01, 0x80));
        assert_eq!(0x7F, cpu.S);
        // stack overflow
        cpu.A = 0xFE;
        cpu.S = 0x00;
        let instruction = Instruction::PHA;
        cpu.execute_instruction(instruction);
        assert_eq!(0xFE, cpu.bus.read(0x01, 0x00));
        assert_eq!(0xFF, cpu.S);
    }

    #[test]
    fn test_pla() {
        let mut cpu = CPU::new();
        cpu.A = 0x00;
        cpu.S = 0x80;
        let val = 0xFF;
        cpu.bus.write(0x01, 0x81, val);
        let instruction = Instruction::PLA;
        cpu.execute_instruction(instruction);
        assert_eq!(0xFF, cpu.A);
        assert_eq!(0x81, cpu.S);
        assert_eq!(false, cpu.Z);
        assert_eq!(true, cpu.N);
        // stack underflow
        cpu.S = 0xFF;
        let val = 0xFE;
        cpu.bus.write(0x01, 0x00, val);
        let instruction = Instruction::PLA;
        cpu.execute_instruction(instruction);
        assert_eq!(0xFE, cpu.A);
        assert_eq!(0x00, cpu.S);
        assert_eq!(false, cpu.Z);
        assert_eq!(true, cpu.N);
    }

    #[test]
    fn test_php() {
        let mut cpu = CPU::new();
        // n bit
        cpu.N = true;
        cpu.V = false;
        cpu.D = false;
        cpu.I = false;
        cpu.Z = false;
        cpu.C = false;
        cpu.S = 0x80;
        let instruction = Instruction::PHP;
        cpu.execute_instruction(instruction);
        assert_eq!(0b10110000, cpu.bus.read(0x01, 0x80));
        assert_eq!(0x7F, cpu.S);
        // v bit
        cpu.N = false;
        cpu.V = true;
        cpu.D = false;
        cpu.I = false;
        cpu.Z = false;
        cpu.C = false;
        let instruction = Instruction::PHP;
        cpu.execute_instruction(instruction);
        assert_eq!(0b01110000, cpu.bus.read(0x01, 0x7F));
        assert_eq!(0x7E, cpu.S);
        // d bit
        cpu.N = false;
        cpu.V = false;
        cpu.D = true;
        cpu.I = false;
        cpu.Z = false;
        cpu.C = false;
        let instruction = Instruction::PHP;
        cpu.execute_instruction(instruction);
        assert_eq!(0b00111000, cpu.bus.read(0x01, 0x7E));
        assert_eq!(0x7D, cpu.S);
        // i bit
        cpu.N = false;
        cpu.V = false;
        cpu.D = false;
        cpu.I = true;
        cpu.Z = false;
        cpu.C = false;
        let instruction = Instruction::PHP;
        cpu.execute_instruction(instruction);
        assert_eq!(0b00110100, cpu.bus.read(0x01, 0x7D));
        assert_eq!(0x7C, cpu.S);
        // z bit
        cpu.N = false;
        cpu.V = false;
        cpu.D = false;
        cpu.I = false;
        cpu.Z = true;
        cpu.C = false;
        let instruction = Instruction::PHP;
        cpu.execute_instruction(instruction);
        assert_eq!(0b00110010, cpu.bus.read(0x01, 0x7C));
        assert_eq!(0x7B, cpu.S);
        // c bit
        cpu.N = false;
        cpu.V = false;
        cpu.D = false;
        cpu.I = false;
        cpu.Z = false;
        cpu.C = true;
        let instruction = Instruction::PHP;
        cpu.execute_instruction(instruction);
        assert_eq!(0b00110001, cpu.bus.read(0x01, 0x7B));
        assert_eq!(0x7A, cpu.S);
        // all bits set
        cpu.N = true;
        cpu.V = true;
        cpu.D = true;
        cpu.I = true;
        cpu.Z = true;
        cpu.C = true;
        let instruction = Instruction::PHP;
        cpu.execute_instruction(instruction);
        assert_eq!(0b11111111, cpu.bus.read(0x01, 0x7A));
        assert_eq!(0x79, cpu.S);
    }

    #[test]
    fn test_plp() {
        let mut cpu = CPU::new();
        // n bit
        cpu.S = 0x80;
        let val = 0b10110000;
        cpu.bus.write(0x01, 0x81, val);
        let instruction = Instruction::PLP;
        cpu.execute_instruction(instruction);
        assert_eq!(true, cpu.N);
        assert_eq!(false, cpu.V);
        assert_eq!(false, cpu.D);
        assert_eq!(false, cpu.I);
        assert_eq!(false, cpu.Z);
        assert_eq!(false, cpu.C);
        assert_eq!(0x81, cpu.S);
        // v bit
        let val = 0b01110000;
        cpu.bus.write(0x01, 0x82, val);
        let instruction = Instruction::PLP;
        cpu.execute_instruction(instruction);
        assert_eq!(false, cpu.N);
        assert_eq!(true, cpu.V);
        assert_eq!(false, cpu.D);
        assert_eq!(false, cpu.I);
        assert_eq!(false, cpu.Z);
        assert_eq!(false, cpu.C);
        assert_eq!(0x82, cpu.S);
        // d bit
        let val = 0b00111000;
        cpu.bus.write(0x01, 0x83, val);
        let instruction = Instruction::PLP;
        cpu.execute_instruction(instruction);
        assert_eq!(false, cpu.N);
        assert_eq!(false, cpu.V);
        assert_eq!(true, cpu.D);
        assert_eq!(false, cpu.I);
        assert_eq!(false, cpu.Z);
        assert_eq!(false, cpu.C);
        assert_eq!(0x83, cpu.S);
        // i bit
        let val = 0b00110100;
        cpu.bus.write(0x01, 0x84, val);
        let instruction = Instruction::PLP;
        cpu.execute_instruction(instruction);
        assert_eq!(false, cpu.N);
        assert_eq!(false, cpu.V);
        assert_eq!(false, cpu.D);
        assert_eq!(true, cpu.I);
        assert_eq!(false, cpu.Z);
        assert_eq!(false, cpu.C);
        assert_eq!(0x84, cpu.S);
        // z bit
        let val = 0b00110010;
        cpu.bus.write(0x01, 0x85, val);
        let instruction = Instruction::PLP;
        cpu.execute_instruction(instruction);
        assert_eq!(false, cpu.N);
        assert_eq!(false, cpu.V);
        assert_eq!(false, cpu.D);
        assert_eq!(false, cpu.I);
        assert_eq!(true, cpu.Z);
        assert_eq!(false, cpu.C);
        assert_eq!(0x85, cpu.S);
        // c bit
        let val = 0b00110001;
        cpu.bus.write(0x01, 0x86, val);
        let instruction = Instruction::PLP;
        cpu.execute_instruction(instruction);
        assert_eq!(false, cpu.N);
        assert_eq!(false, cpu.V);
        assert_eq!(false, cpu.D);
        assert_eq!(false, cpu.I);
        assert_eq!(false, cpu.Z);
        assert_eq!(true, cpu.C);
        assert_eq!(0x86, cpu.S);
        // all bits set
        let val = 0b11111111;
        cpu.bus.write(0x01, 0x87, val);
        let instruction = Instruction::PLP;
        cpu.execute_instruction(instruction);
        assert_eq!(true, cpu.N);
        assert_eq!(true, cpu.V);
        assert_eq!(true, cpu.D);
        assert_eq!(true, cpu.I);
        assert_eq!(true, cpu.Z);
        assert_eq!(true, cpu.C);
        assert_eq!(0x87, cpu.S);
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