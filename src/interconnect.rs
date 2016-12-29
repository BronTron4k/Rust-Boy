pub const BOOT_ROM_SIZE: usize = 256;
const RAM_SIZE: usize = 8192;

pub struct Interconnect {
    boot_rom: [u8; BOOT_ROM_SIZE],
    wram: [u8; RAM_SIZE],
}

impl Interconnect {
    pub fn new(boot_rom:  [u8; BOOT_ROM_SIZE]) -> Self {

        Interconnect {
            boot_rom: boot_rom,
            wram: [0; RAM_SIZE],
        }
    }
}
