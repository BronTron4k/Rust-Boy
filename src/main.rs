use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;

struct Cpu {
    u8: reg_a, // Accumlator
    u8: reg_f, // Flags TODO: Enum type

    // General Purpose
    u8: reg_b,
    u8: reg_c,
    u8: reg_d,
    u8: reg_e,
    u8: reg_h,
    u8: reg_l,

    u16: reg_sp, // Stack Pointer
    u16: reg_pc, // Program Counter
}

impl Cpu {
    fn new() -> Self {
        Cpu {
            reg_a: 0,
            reg_f: 0,
            reg_b: 0,
            reg_c: 0,
            reg_d: 0,
            reg_e: 0,
            reg_h: 0,
            reg_l: 0,
            reg_sp: 0,
            reg_pc: 0,
        }
    }
}

fn main() {
    let boot_file_name = env::args().nth(1).unwrap();
    let rom_file_name = env::args().nth(2).unwrap();

    let bootloader = read_bin(boot_file_name);
    let rom = read_bin(rom_file_name);

    let mut cpu = Cpu::new();
}

fn read_bin<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = fs::File::open(path).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf
}
