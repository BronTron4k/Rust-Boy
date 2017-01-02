use super::cpu;
use super::mmu;



pub struct GameBoy {
    cpu: cpu::Cpu,
}

impl GameBoy {
    pub fn new() -> Self {
        GameBoy {
            cpu: cpu::Cpu::new(),
        }
    }

    pub fn power_on(&mut self, boot_rom: Vec<u8>, game_rom: Vec<u8>) {
        self.load_rom(game_rom);
        self.load_rom(boot_rom);

        self.run()
    }

    fn run(&mut self) {
        self.cpu.run();
    }

    fn load_rom(&mut self, rom_data: Vec<u8>) {
        for (idx, op) in rom_data.iter().enumerate().take(0x2000) {
            self.cpu.mmu.write_byte(idx as u16, *op)
        }
    }
}

