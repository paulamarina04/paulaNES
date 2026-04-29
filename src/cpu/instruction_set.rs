pub enum Instruction {
    //access
    LDA(u8, AddressingMode),
    //other
    NOP
}

pub enum AddressingMode {
    Immediate
}