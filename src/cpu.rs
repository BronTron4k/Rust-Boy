#[derive(Debug, Default)]
pub struct Cpu {
    reg_a: u8, // Accumlator
    reg_f: RegFlag, // Flags

    // General Purpose
    reg_b: u8,
    reg_c: u8,
    reg_d: u8,
    reg_e: u8,
    reg_h: u8,
    reg_l: u8,

    reg_sp: u16, // Stack Pointer
    reg_pc: u16, // Program Counter
}

impl Cpu {
    pub fn new() -> Self {
        Cpu::default()
    }
}

#[derive(Debug, Default)]
struct RegFlag {
    Carry: bool,
    HalfCarry: bool,
    AddSub: bool,
    Zero: bool,
}
