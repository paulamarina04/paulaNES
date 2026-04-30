pub enum Instruction {
    //access
    LDA(u8, AddressingMode),
    //transfer
    TAX,
    TXA,
    TAY,
    TYA,
    TXS,
    TSX,
    //flags
    CLC,
    SEC,
    CLI,
    SEI,
    CLD,
    SED,
    CLV,
    //other
    NOP
}

pub enum AddressingMode {
    Immediate
}