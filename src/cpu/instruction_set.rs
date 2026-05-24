pub enum Instruction {
    //access
    LDA(AddrMode), STA(AddrMode), LDX(AddrMode), STX(AddrMode), LDY(AddrMode), STY(AddrMode), 
    //transfer
    TAX, TXA, TAY, TYA, TXS, TSX,
    //arithmetic
    ADC(AddrMode), SBC(AddrMode), INC(AddrMode), DEC(AddrMode), INX, DEX, INY, DEY,
    //shift
    ASL_A, ASL_MEM(AddrMode),
    //bitwise
    AND(AddrMode), ORA(AddrMode), XOR(AddrMode), BIT(AddrMode),
    // compare
    CMP(AddrMode), CPX(AddrMode), CPY(AddrMode),
    // branch
    BCC(i8), BCS(i8), BEQ(i8), BNE(i8), BPL(i8), BMI(i8), BVC(i8), BVS(i8), 
    // jump
    JMP(AddrMode), JSR(AddrMode), RTS, BRK, RTI,
    // stack
    PHA, PLA, PHP, PLP,
    //flags
    CLC, SEC, CLI, SEI, CLD, SED, CLV,
    //other
    NOP
}

pub enum AddrMode {
    Immediate(u8),
    Absolute(u8, u8),
    AbsoluteIndexedX(u8, u8),
    AbsoluteIndexedY(u8, u8),
    ZeroPage(u8),
    ZeroPageIndexedX(u8),
    ZeroPageIndexedY(u8),
    Indirect(u8, u8),
    IndexedIndirect(u8),
    IndirectIndexed(u8)
}