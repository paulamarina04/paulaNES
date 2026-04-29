mod instruction_set;
mod execution;
#[allow(non_snake_case)]
struct CPU {
    A: u8,
    X: u8,
    Y: u8,
    PC_hi: u8,
    PC_lo: u8,
    S: u8, 
    // processor flags
    N: bool,
    V: bool,
    D: bool,
    I: bool,
    Z: bool,
    C: bool 
}

impl CPU {
    fn new() -> Self {
        let ret= Self {
            A: 0x00,
            X: 0x00,
            Y: 0x00,
            PC_hi: 0x00,
            PC_lo: 0x00,
            S: 0x00, 
            // processor flags
            N: false,
            V: false,
            D: false,
            I: false,
            Z: false,
            C: false, 
        };
        return ret;
    }
}

