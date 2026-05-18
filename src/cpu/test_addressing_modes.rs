use crate::cpu::CPU;
use crate::cpu::instruction_set::AddrMode;

// panic scenarios

#[test]
#[should_panic(expected = "Attempted to fetch an address using an immediate addressing mode")]
fn test_immediate_address() {
    let cpu = CPU::new();
    let val = 0xFF;
    let addr_mode = AddrMode::Immediate(val);
    cpu.get_addressed_address(addr_mode);
}

#[test]
#[should_panic(expected = "Attempted to fetch a value using a plain indirect addressing mode")]
fn test_indirect_value() {
    let cpu = CPU::new();
    let hi = 0x00;
    let lo = 0x00;
    let addr_mode = AddrMode::Indirect(hi, lo);
    cpu.get_addressed_value(addr_mode);
}

// addressed values

#[test]
fn test_immediate_val() {
    let cpu = CPU::new();
    let val = 0xFF;
    let addr_mode = AddrMode::Immediate(val);
    assert_eq!(0xFF, cpu.get_addressed_value(addr_mode));
}

#[test]
fn test_absolute_val() {
    let mut cpu = CPU::new();
    let hi = 0x80;
    let lo = 0x01;
    let val = 0xFF;
    cpu.bus.write(hi, lo, val);
    let addr_mode = AddrMode::Absolute(hi, lo);
    assert_eq!(0xFF, cpu.get_addressed_value(addr_mode));
}

#[test]
fn test_absolute_indexed_x_value() {
    let mut cpu = CPU::new();
    // no page cross
    let hi = 0x80;
    let lo = 0x01;
    let val = 0xFF;
    cpu.X = 0x80;
    cpu.bus.write(hi, 0x81, val);
    let addr_mode = AddrMode::AbsoluteIndexedX(hi, lo);
    assert_eq!(0xFF, cpu.get_addressed_value(addr_mode));
    // page crossed
    let hi = 0x80;
    let lo = 0x80;
    let val = 0xFE;
    cpu.X = 0xFF;
    cpu.bus.write(0x81, 0x7F, val);
    let addr_mode = AddrMode::AbsoluteIndexedX(hi, lo);
    assert_eq!(0xFE, cpu.get_addressed_value(addr_mode));
    // page wrap
    let hi = 0xFF;
    let lo = 0xFF;
    let val = 0xFD;
    cpu.X = 0x01;
    cpu.bus.write(0x00, 0x00, val);
    let addr_mode = AddrMode::AbsoluteIndexedX(hi, lo);
    assert_eq!(0xFD, cpu.get_addressed_value(addr_mode));
}

#[test]
fn test_absolute_indexed_y_value() {
    let mut cpu = CPU::new();
    // no page cross
    let hi = 0x80;
    let lo = 0x01;
    let val = 0xFF;
    cpu.Y = 0x80;
    cpu.bus.write(hi, 0x81, val);
    let addr_mode = AddrMode::AbsoluteIndexedY(hi, lo);
    assert_eq!(0xFF, cpu.get_addressed_value(addr_mode));
    // page crossed
    let hi = 0x80;
    let lo = 0x80;
    let val = 0xFE;
    cpu.Y = 0xFF;
    cpu.bus.write(0x81, 0x7F, val);
    let addr_mode = AddrMode::AbsoluteIndexedY(hi, lo);
    assert_eq!(0xFE, cpu.get_addressed_value(addr_mode));
    // page wrap
    let hi = 0xFF;
    let lo = 0xFF;
    let val = 0xFD;
    cpu.Y = 0x01;
    cpu.bus.write(0x00, 0x00, val);
    let addr_mode = AddrMode::AbsoluteIndexedY(hi, lo);
    assert_eq!(0xFD, cpu.get_addressed_value(addr_mode));
}

#[test]
fn test_zero_page_value() {
    let mut cpu = CPU::new();
    let hi = 0x00;
    let lo = 0x80;
    let val = 0xFF;
    cpu.bus.write(hi, lo, val);
    let addr_mode = AddrMode::ZeroPage(lo);
    assert_eq!(0xFF, cpu.get_addressed_value(addr_mode));
}

#[test]
fn test_zero_page_indexed_x_value() {
    let mut cpu = CPU::new();
    // no page wrap
    let hi = 0x00;
    let lo = 0x80;
    let val = 0xFD;
    cpu.X = 0x7F;
    cpu.bus.write(hi, 0xFF, val);
    let addr_mode = AddrMode::ZeroPageIndexedX(lo);
    assert_eq!(0xFD, cpu.get_addressed_value(addr_mode));
    // page wrap
    let hi = 0x00;
    let lo = 0xFF;
    let val = 0xFC;
    cpu.X = 0x01;
    cpu.bus.write(hi, 0x00, val);
    let addr_mode = AddrMode::ZeroPageIndexedX(lo);
    assert_eq!(0xFC, cpu.get_addressed_value(addr_mode));
}

#[test]
fn test_zero_page_indexed_y_value() {
    let mut cpu = CPU::new();
    // no page wrap
    let hi = 0x00;
    let lo = 0x80;
    let val = 0xFD;
    cpu.Y = 0x7F;
    cpu.bus.write(hi, 0xFF, val);
    let addr_mode = AddrMode::ZeroPageIndexedY(lo);
    assert_eq!(0xFD, cpu.get_addressed_value(addr_mode));
    // page wrap
    let hi = 0x00;
    let lo = 0xFF;
    let val = 0xFC;
    cpu.Y = 0x01;
    cpu.bus.write(hi, 0x00, val);
    let addr_mode = AddrMode::ZeroPageIndexedY(lo);
    assert_eq!(0xFC, cpu.get_addressed_value(addr_mode));
}

// addressed addresses

#[test]
fn test_absolute_address() {
    let cpu = CPU::new();
    let hi = 0x80;
    let lo = 0x01;
    let addr_mode = AddrMode::Absolute(hi, lo);
    assert_eq!((0x80, 0x01), cpu.get_addressed_address(addr_mode));
}

#[test]
fn test_absolute_indexed_x_address() {
    let mut cpu = CPU::new();
    // no page cross
    let hi = 0x80;
    let lo = 0x01;
    cpu.X = 0x80;
    let addr_mode = AddrMode::AbsoluteIndexedX(hi, lo);
    assert_eq!((0x80, 0x81), cpu.get_addressed_address(addr_mode));
    // page crossed
    let hi = 0x80;
    let lo = 0x80;
    cpu.X = 0xFF;
    let addr_mode = AddrMode::AbsoluteIndexedX(hi, lo);
    assert_eq!((0x81, 0x7F), cpu.get_addressed_address(addr_mode));
    // page wrap
    let hi = 0xFF;
    let lo = 0xFF;
    cpu.X = 0x01;
    let addr_mode = AddrMode::AbsoluteIndexedX(hi, lo);
    assert_eq!((0x00, 0x00), cpu.get_addressed_address(addr_mode));
}

#[test]
fn test_absolute_indexed_y_address() {
    let mut cpu = CPU::new();
    // no page cross
    let hi = 0x80;
    let lo = 0x01;
    cpu.Y = 0x80;
    let addr_mode = AddrMode::AbsoluteIndexedY(hi, lo);
    assert_eq!((0x80, 0x81), cpu.get_addressed_address(addr_mode));
    // page crossed
    let hi = 0x80;
    let lo = 0x80;
    cpu.Y = 0xFF;
    let addr_mode = AddrMode::AbsoluteIndexedY(hi, lo);
    assert_eq!((0x81, 0x7F), cpu.get_addressed_address(addr_mode));
    // page wrap
    let hi = 0xFF;
    let lo = 0xFF;
    cpu.Y = 0x01;
    let addr_mode = AddrMode::AbsoluteIndexedY(hi, lo);
    assert_eq!((0x00, 0x00), cpu.get_addressed_address(addr_mode));
}

#[test]
fn test_zero_page_address() {
    let cpu = CPU::new();
    let lo = 0x80;
    let addr_mode = AddrMode::ZeroPage(lo);
    assert_eq!((0x0, 0x80), cpu.get_addressed_address(addr_mode));
}

#[test]
fn test_zero_page_indexed_x_address() {
    let mut cpu = CPU::new();
    // no page wrap
    let lo = 0x80;
    cpu.X = 0x7F;
    let addr_mode = AddrMode::ZeroPageIndexedX(lo);
    assert_eq!((0x0, 0xFF), cpu.get_addressed_address(addr_mode));
    // page wrap
    let lo = 0xFF;
    cpu.X = 0x01;
    let addr_mode = AddrMode::ZeroPageIndexedX(lo);
    assert_eq!((0x0, 0x00), cpu.get_addressed_address(addr_mode));
}

#[test]
fn test_zero_page_indexed_y_address() {
    let mut cpu = CPU::new();
    // no page wrap
    let lo = 0x80;
    cpu.Y = 0x7F;
    let addr_mode = AddrMode::ZeroPageIndexedY(lo);
    assert_eq!((0x0, 0xFF), cpu.get_addressed_address(addr_mode));
    // page wrap
    let lo = 0xFF;
    cpu.Y = 0x01;
    let addr_mode = AddrMode::ZeroPageIndexedY(lo);
    assert_eq!((0x0, 0x00), cpu.get_addressed_address(addr_mode));
}

#[test]
fn test_indirect_address() {
    let mut cpu = CPU::new();
    // no page cross
    let hi = 0x80;
    let lo = 0x01;
    let val1 = 0xFF; // low byte
    let val2 = 0xFE; // high byte
    cpu.bus.write(hi, lo, val1);
    cpu.bus.write(hi, lo + 1, val2);
    let addr_mode = AddrMode::Indirect(hi, lo);
    assert_eq!((0xFE, 0xFF), cpu.get_addressed_address(addr_mode));
    // page cross bug
    let hi = 0x80;
    let lo = 0xFF;
    let val1 = 0xFD; // low byte
    let val2 = 0xFC; // actual high byte fetched
    let val3 = 0xFB; // expected high byte if not accounting for the bug
    cpu.bus.write(hi, lo, val1);
    cpu.bus.write(0x80, 0x00, val2);
    cpu.bus.write(0x81, 0x00, val3);
    let addr_mode = AddrMode::Indirect(hi, lo);
    assert_eq!((0xFC, 0xFD), cpu.get_addressed_address(addr_mode));
}