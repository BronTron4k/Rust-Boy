use super::mmu;

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

    pub mmu: mmu::Mmu,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            reg_a: 1,
            reg_f: RegFlag::default(),

            reg_b: 0,
            reg_c: 0,
            reg_d: 0,
            reg_e: 0,
            reg_h: 0,
            reg_l: 0,
            reg_sp: 0,
            reg_pc: 0,

            mmu: mmu::Mmu::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.mmu.read_byte(self.reg_pc);
            println!("reg_pc {:#06x}: {:#04x}", self.reg_pc, opcode);

            self.reg_pc += 1;


            match opcode {
                0x0C => { // INC C
                    self.reg_f.half_carry = (self.reg_c & 0xF0) + 1 == 0x10;
                    self.reg_c = self.reg_c.wrapping_add(1);

                    self.reg_f.zero = self.reg_c == 0;
                    self.reg_f.add_sub = true;
                },
                0x0E => { // LD C, d8
                    self.reg_c = self.load_imm_u8();
                },
                0x20 => { // JR NZ, r8
                    if !self.reg_f.zero {
                        self.jump_return()
                    } else {
                        self.reg_pc += 1;
                    }
                },
                0x21 => { // LD HL, d16
                    let imm = self.load_imm_u16();
                    self.set_hl(imm);
                },
                0x31 => { // LD SP, d16
                    self.reg_sp = self.load_imm_u16();
                },
                0x32 => { // LD (HL-), A
                    let idx = self.hl();
                    self.set_hl(idx - 1);
                    self.mmu.write_byte(idx, self.reg_a);
                },
                0x3E => { // LD A, d8
                    self.reg_a = self.load_imm_u8();
                },
                0xAF => { // XOR A
                    self.reg_a ^= self.reg_a;

                    self.reg_f.zero = self.reg_a == 0;
                    self.reg_f.carry = false;
                    self.reg_f.add_sub = false;
                    self.reg_f.half_carry = false;
                },
                0xCB => { // CB Insruction
                    self.execute_cb();
                },
                0xE2 => { // Ld (C), A
                    self.mmu.write_byte(0xFF00 + self.reg_c as u16, self.reg_a);
                }
                _ => panic!("Unimpletemented instruction: {:#04X}", opcode)
            }
        }
    }

    fn execute_cb(&mut self) {
        let opcode = self.mmu.read_byte(self.reg_pc);
        println!("reg_pc {:#06x}: CB Instruction -> {:#04x}", self.reg_pc, opcode);

        self.reg_pc += 1;

        match opcode {
            0x7C => { // BIT 7, H
                self.reg_f.zero = self.reg_h >> 7 == 0;
                self.reg_f.add_sub = false;
                self.reg_f.half_carry = true;
            }
            _ => panic!("Unimplemented CB Instruction: {:#04x}", opcode)
        }
    }

    fn load_imm_u8(&mut self) -> u8 {
        let imm = self.mmu.read_byte(self.reg_pc);
        self.reg_pc +=1;
        imm
    }

    fn load_imm_u16(&mut self) -> u16 {
        (self.load_imm_u8() as u16) | (self.load_imm_u8() as u16) << 8
    }

    fn bc(&self) -> u16 {
        (self.reg_b as u16) << 8 | (self.reg_c as u16)
    }

    fn set_bc(&mut self, value: u16) {
        self.reg_b = (value >> 8) as u8;
        self.reg_c = (value & 0xFF) as u8;
    }

    fn de(&self) -> u16 {
        (self.reg_d as u16) << 8 | (self.reg_e as u16)
    }

    fn set_de(&mut self, value: u16) {
        self.reg_d = (value >> 8) as u8;
        self.reg_e = (value & 0xFF) as u8;
    }

    fn hl(&self) -> u16 {
        (self.reg_h as u16) << 8 | (self.reg_l as u16)
    }

    fn set_hl(&mut self, value: u16) {
        self.reg_h = (value >> 8) as u8;
        self.reg_l = (value & 0xFF) as u8;
    }

    fn jump_return(&mut self) {
        let imm = self.load_imm_u8();
        let jump_dist = unsigned_to_signed(imm);

        if jump_dist > 0 {
            self.reg_pc += jump_dist as u16;
        } else {
            self.reg_pc -= signed_to_unsigned(jump_dist) as u16;
        }
    }
}

fn unsigned_to_signed(val: u8) -> i16 {
    (val as i8) as i16
}

fn signed_to_unsigned(val: i16) -> u8 {
    !(val as u8) + 1
}

#[derive(Default)]
struct RegFlag {
    carry: bool,
    half_carry: bool,
    add_sub: bool,
    zero: bool,
}
