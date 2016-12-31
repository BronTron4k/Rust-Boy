pub const ROM_BANK_SIZE: usize = 0x4000;
pub const WRAM_SIZE: usize = 0x2000;
pub const VRAM_SIZE: usize = 0x2000;

pub struct Mmu {
    rom: [u8; 2 * ROM_BANK_SIZE],
    wram: [u8; WRAM_SIZE],
    gpu: Gpu,
    apu: Apu,
}

impl Mmu {
    pub fn new() -> Self {
        Mmu {
            rom: [0; 2 * ROM_BANK_SIZE],
            wram: [0; WRAM_SIZE],
            gpu: Gpu::new(),
            apu: Apu::new(),
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr >> 8 {
            0x00...0x7F => self.rom[addr as usize],
            _ => panic!("Unimplemented read address: {:#x}", addr)
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        match addr >> 8 {
            0x00...0x7F => self.rom[addr as usize] = value,
            0x80...0x9F => self.gpu.vram[(addr as usize) & 0xFF] = value,
            0xFF => {
                let addr = (addr & 0xFF) as u8;
                match addr {
                    0x10...0x3F => {
                        self.apu.write_byte(addr, value);
                    }
                    _ => panic!("Unimplemented IO address: {:#x}: {:#x}", addr, value)
                }
            }
            _ => panic!("Unimplmented write address: {:#x}", addr)
        }
    }
}

struct Gpu {
    vram: [u8; VRAM_SIZE],
}

impl Gpu {
    fn new() -> Self {
        Gpu {
            vram: [0; VRAM_SIZE],
        }
    }
}

struct Apu {
    enabled: bool,
    chan1: Chan1,
}

impl Apu {
    fn new() -> Self {
        Apu {
            enabled: false,
            chan1: Chan1,
        }
    }

    fn write_byte(&mut self, addr: u8, value: u8) {
        match addr {
            0x26 => {
                if value == 0x80 {
                    self.enabled = value >> 7 == 1;
                    println!("Control Master: {:?}", self.enabled)
                }
            },
            _ => panic!("Unimpletemented Apu write at address: {:#x}: {:#x}",addr, value)
        }
    }
}

struct chan1 {
    wave_duty:
}
