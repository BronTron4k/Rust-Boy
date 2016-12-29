use super::interconnect;

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

    interconnect: interconnect::Interconnect,
}

impl Cpu {
    pub fn new(interconnect: interconnect::Interconnect) -> Self {
        Cpu {
            reg_a: 0,
            reg_f: RegFlag::default(),

            reg_b: 0,
            reg_c: 0,
            reg_d: 0,
            reg_e: 0,
            reg_h: 0,
            reg_l: 0,
            reg_sp: 0,
            reg_pc: 0,

            interconnect: interconnect,
        }
    }

    pub fn run(&mut self) {
    }
}

#[derive(Default)]
struct RegFlag {
    carry: bool,
    half_carry: bool,
    add_sub: bool,
    zero: bool,
}
