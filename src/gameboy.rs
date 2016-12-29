use super::cpu;
use super::interconnect;

pub struct GameBoy {
    cpu: cpu::Cpu,
}

impl GameBoy {
    pub fn new(boot_rom: Vec<u8>) -> Self {
        GameBoy {
            cpu: cpu::Cpu::new(interconnect::Interconnect::new(load_boot_rom(boot_rom))),
        }
    }

    pub fn run(&mut self) {
       self.cpu.run();
    }

}

fn load_boot_rom(boot_rom_data: Vec<u8>) -> [u8; interconnect::BOOT_ROM_SIZE] {
    let mut boot_rom = [0; interconnect::BOOT_ROM_SIZE];

    for (idx, op) in boot_rom_data.iter().enumerate() {
        boot_rom[idx] = *op;
    }

    boot_rom
}
