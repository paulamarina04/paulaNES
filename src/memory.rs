use std::collections::HashMap;

// IMPORTANT: the following implementation is a placeholder stub 
// it should eventually be replaced by an actual implementation of the NES main memory


pub struct SystemBus {
    mem: HashMap<(u8, u8), u8>
}

impl SystemBus {
    pub fn new() -> Self {
        if cfg!(not(test)) { panic!(); }

        return Self { 
            mem: HashMap::new()
        };
    }

    pub fn read(&self, addr_hi: u8, addr_lo: u8) -> u8 {
        if cfg!(not(test)) { panic!(); }

        let address = (addr_hi, addr_lo);
        let ret = self.mem.get(&address);
        return match ret {
            Some(val) => { *val },
            None => { 0x00 }
        };
    }

    pub fn write(&mut self, addr_hi: u8, addr_lo: u8, data: u8) {
        if cfg!(not(test)) { panic!(); }

        let address = (addr_hi, addr_lo);
        self.mem.insert(address, data);
    }
}

/*#[test]
fn test_mem() {
    let mut mm = SystemBus::new();
    mm.write(0x00, 0x00, 0xFF);
    let mut a = mm.read(0x00, 0x00);
    a = 0x10;
    assert_eq!(0xFF, mm.read(0x00, 0x00))
} */