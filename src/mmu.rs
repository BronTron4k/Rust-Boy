use super::apu;
use super::gpu;

pub const ROM_BANK_SIZE: usize = 0x4000;
pub const WRAM_SIZE: usize = 0x2000;
pub const HIRAM_SIZE: usize = 0xFF;

pub struct Mmu {
    rom: [u8; 2 * ROM_BANK_SIZE],
    wram: [u8; WRAM_SIZE],
    hram: [u8; HIRAM_SIZE],
    gpu: gpu::Gpu,
    apu: apu::Apu,
}

impl Mmu {
    pub fn new() -> Self {
        Mmu {
            rom: [0; 2 * ROM_BANK_SIZE],
            wram: [0; WRAM_SIZE],
            hram: [0; HIRAM_SIZE],
            gpu: gpu::Gpu::new(),
            apu: apu::Apu::new(),
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr >> 8 {
            0x00...0x7F => self.rom[addr as usize],
            0x80...0xFE => self.hram[addr as usize],
            0xFF if addr < 0xFF80 => {
                match addr & 0xFF {
                    0x40...0x4B => self.gpu.read_byte(addr),
                    _ => panic!("Unimplemented read at IO address: {:#X}", addr)
                }
            },
            0xFF if addr > 0xFF7F && addr < 0xFFFF => {
                self.hram[(addr & 0xFF) as usize]
            },
            _ => panic!("Unimplemented read address: {:#x}", addr)
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        match addr >> 8 {
            0x00...0x7F => self.rom[addr as usize] = value,
            0x80...0x9F => self.gpu.write_byte(addr, value),
            0xFF if addr < 0xFF80 => {
                match addr & 0xFF {
                    0x10...0x3F => self.apu.write_byte(addr, value),
                    0x40...0x4B => self.gpu.write_byte(addr, value),
                    _ => panic!("Unimplemented write at IO address: {:#x}: {:#x}", addr, value)
                }
            },
            0xFF if addr > 0xFF7F && addr < 0xFFFF => {
                self.hram[(addr & 0xFF) as usize] = value;
            },
            _ => panic!("Unimplmented write address: {:#x}", addr)
        }
    }
}

