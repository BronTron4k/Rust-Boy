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
    pulse_a: PulseA,
}

impl Apu {
    fn new() -> Self {
        Apu {
            enabled: false,
            pulse_a: PulseA::new(),
        }
    }

    fn write_byte(&mut self, addr: u8, value: u8) {
        match addr {
            0x11 => {
                self.pulse_a.sound_length = 64 - (value & 0x3F);

                match value >> 6 {
                    0 => self.pulse_a.wave_duty = WaveDuty::HalfQuarter,
                    1 => self.pulse_a.wave_duty = WaveDuty::Quarter,
                    2 => self.pulse_a.wave_duty = WaveDuty::Half,
                    3 => self.pulse_a.wave_duty = WaveDuty::ThreeQuarters,
                    _ => panic!()
                }
            },
            0x12 => {
                self.pulse_a.envelope.count = value & 0x7;
                self.pulse_a.envelope.direction = match (value & 0x8) >> 3 {
                    0 => Direction::Decreasing,
                    1 => Direction::Increasing,
                    _ => panic!("{:#x}", (value & 0x8) >> 3)
                };
            }
            0x25 => {
            },
            0x26 => self.enabled = value >> 7 == 1,
            _ => panic!("Unimpletemented Apu write at address: {:#x}: {:#x}",addr, value)
        }
    }
}

#[derive(Debug)]
struct PulseA {
    wave_duty: WaveDuty,
    sound_length: u8,
    envelope: Envelope
}

struct PulseB {
    wave_duty: WaveDuty,
    sound_length: u8,
    envelope: Envelope
}

impl PulseA {
    fn new() -> Self {
        PulseA {
            wave_duty: WaveDuty::Half,
            sound_length: 0,
            envelope: Envelope::new()
        }
    }
}

impl PulseB {
    fn new() -> Self {
        PulseB {
            wave_duty: WaveDuty::Half,
            sound_length: 0,
            envelope: Envelope::new()
        }
    }
}

#[derive(Debug)]
enum WaveDuty {
    HalfQuarter,
    Quarter,
    Half,
    ThreeQuarters,
}

#[derive(Debug)]
struct Envelope {
    volume: u8,
    direction: Direction,
    count: u8,
}

impl Envelope {
    fn new () -> Self {
        Envelope {
            volume: 0,
            direction: Direction::Increasing,
            count: 0,
        }
    }
}

#[derive(Debug)]
enum Direction {
    Increasing,
    Decreasing,
}
