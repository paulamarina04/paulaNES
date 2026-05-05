pub enum Instruction {
    //access
    LDA(AddrMode),
    //transfer
    TAX, TXA, TAY, TYA, TXS, TSX,
    //arithmetic
    ADC(AddrMode), SBC(AddrMode),
    //flags
    CLC, SEC, CLI, SEI, CLD, SED, CLV,
    //other
    NOP
}

pub enum AddrMode {
    Immediate(u8)
}