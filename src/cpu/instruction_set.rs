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
    //other
    NOP
}

pub enum AddressingMode {
    Immediate
}