pub struct Apu {
    enabled: bool,
    channel_1: Channel,
    channel_2: Channel,
    pulse_a: PulseA,
    pulse_b: PulseB,
}

impl Apu {
    pub fn new() -> Self {
        Apu {
            enabled: false,
            channel_1: Channel::new(),
            channel_2: Channel::new(),
            pulse_a: PulseA::new(),
            pulse_b: PulseB::new(),
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        match addr & 0xFF {
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
            },
            0x24 => {
                self.channel_1.vin = (value & 0x8) == 0x8;
                self.channel_2.vin = (value & 0x8) == 0x8;

                self.channel_1.volume = value & 0x7;
                self.channel_2.volume = value >> 4 & 0x7;
            },
            0x25 => {
                self.channel_1.voices = Voices::from_bits_truncate(value);
                self.channel_2.voices= Voices::from_bits_truncate(value >> 4);
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

#[derive(Debug)]
struct Channel {
    vin: bool,
    volume: u8,
    voices: Voices
}

impl Channel {
    fn new() -> Self {
        Channel {
            vin: false,
            volume: 0,
            voices: Voices::empty(),
        }
    }
}

bitflags! {
    flags Voices: u8 {
        const PULSE_A = 1 << 0,
        const PULSE_B = 1 << 1,
        const WAVE    = 1 << 2,
        const NOISE   = 1 << 3,
    }
}
