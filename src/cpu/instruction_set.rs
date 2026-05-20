pub enum Instruction {
    //access
    LDA(AddrMode), STA(AddrMode),
    //transfer
    TAX, TXA, TAY, TYA, TXS, TSX,
    //arithmetic
    ADC(AddrMode), SBC(AddrMode), INX, DEX, INY, DEY,
    //bitwise
    AND(AddrMode), ORA(AddrMode), XOR(AddrMode), /*BIT(AddrMode),*/
    // jump
    JMP(AddrMode),
    // stack
    PHA,
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