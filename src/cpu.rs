#[allow(dead_code,non_snake_case)]
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

mod instruction_set;
mod execution;