pub enum Instruction {
    //access
    LDA(AddrMode8), STA(AddrMode16),
    //transfer
    TAX, TXA, TAY, TYA, TXS, TSX,
    //arithmetic
    ADC(AddrMode8), SBC(AddrMode8), INX, DEX, INY, DEY,
    //bitwise
    AND(AddrMode8), ORA(AddrMode8), XOR(AddrMode8), /*BIT(AddrMode),*/
    // jump
    JMP(AddrMode16),
    //flags
    CLC, SEC, CLI, SEI, CLD, SED, CLV,
    //other
    NOP
}

pub enum AddrMode8 {
    Immediate(u8)
}

pub enum AddrMode16 {
    Absolute(u8, u8)
}