pub enum Instruction {
    //access
    LDA(ValueAddrMode), STA(AddressAddrMode),
    //transfer
    TAX, TXA, TAY, TYA, TXS, TSX,
    //arithmetic
    ADC(ValueAddrMode), SBC(ValueAddrMode), INX, DEX, INY, DEY,
    //bitwise
    AND(ValueAddrMode), ORA(ValueAddrMode), XOR(ValueAddrMode), /*BIT(AddrMode),*/
    // jump
    JMP(AddressAddrMode),
    //flags
    CLC, SEC, CLI, SEI, CLD, SED, CLV,
    //other
    NOP
}

pub enum ValueAddrMode {
    Immediate(u8),
    Absolute(u8, u8)
}

pub enum AddressAddrMode {
    Absolute(u8, u8)
}