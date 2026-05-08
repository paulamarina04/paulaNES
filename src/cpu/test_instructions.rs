
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

    /*#[test]
    fn test_bit() {
        let mut cpu: CPU = CPU::new();
        let val1: u8 = 0b11001100;
        let val2: u8 = 0b10000000;
        cpu.A = val1;
        let instruction = Instruction::BIT(AddrMode::Immediate(val2));
        cpu.execute_instruction(instruction);
        assert_eq!(0b11001100, cpu.A);
        assert_eq!(false, cpu.Z);
        assert_eq!(true, cpu.N);
    }*/



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