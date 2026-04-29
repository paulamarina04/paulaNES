#[allow(dead_code,non_upper_case_globals)]
mod int_registers {
    static mut A: u8 = 0x00;
    static mut X: u8 = 0x00;
    static mut Y: u8 = 0x00;
    static mut PC_hi: u8 = 0x00;
    static mut PC_lo: u8 = 0x00;
    static mut SP: u8 = 0x00; // more commonly known as S
    // processor flags
    static mut N: bool = false;
    static mut V: bool = false;
    static mut D: bool = false;
    static mut I: bool = false;
    static mut Z: bool = false;
    static mut C: bool = false;
}

mod instruction_set;