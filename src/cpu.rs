use super::memory::SystemBus;

mod instruction_set;
mod execution;

#[cfg(test)]
mod test_instructions;
#[cfg(test)]
mod test_addressing_modes;


#[allow(non_snake_case)]
struct CPU {
    // internal registers
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
    C: bool,
    // system bus
    bus: SystemBus
}

impl CPU {
    fn new() -> Self {
        return Self {
            // internal registers
            A: 0x00,
            X: 0x00,
            Y: 0x00,
            PC_hi: 0x00,
            PC_lo: 0x00,
            S: 0xFF, 
            // processor flags
            N: false,
            V: false,
            D: false,
            I: false,
            Z: false,
            C: false, 
            // system bus
            bus: SystemBus::new()
        };
    }
}

