pub const TILE_MAP_SIZE: usize = 0x1000;
pub const BG_MAP_SIZE: usize = 0x400;

pub struct Gpu {
    tile_map_0: [u8; TILE_MAP_SIZE],
    tile_map_1: [u8; TILE_MAP_SIZE],
    bg_map_0: [u8; BG_MAP_SIZE],
    bg_map_1: [u8; BG_MAP_SIZE],
    bg_palette: Palette,

    lcd: Lcd,

    scroll_x: u8,
    scroll_y: u8,

    ly: u8,
}

impl Gpu {
    pub fn new() -> Self {
        Gpu {
            tile_map_0: [0; TILE_MAP_SIZE],
            tile_map_1: [0; TILE_MAP_SIZE],
            bg_map_0: [0; BG_MAP_SIZE],
            bg_map_1: [0; BG_MAP_SIZE],
            bg_palette: Palette::new(),

            lcd: Lcd::new(),

            scroll_x: 0,
            scroll_y: 0,
            ly: 0,
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr >> 8 {
            0xFF => {
                match addr & 0xFF {
                    0x44 => self.ly,
                    _ => panic!("Unimplemented GPU Register read at {:04X}", addr),
                }
            },
            _ => panic!("Unimplemented GPU read at {:04X}", addr),
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        match addr >> 8 {
            0x80...0x87 => self.tile_map_0[(addr-0x8000) as usize] = value,
            0x88...0x8F => {
                self.tile_map_0[(addr-0x8000) as usize] = value;
                self.tile_map_1[(addr-0x8800) as usize] = value;
            },
            0x90...0x97 => self.tile_map_1[(addr-0x8800) as usize] = value,
            0x98...0x9B => self.bg_map_0[(addr - 0x9800) as usize] = value,
            0x9C...0x9F => self.bg_map_0[(addr - 0x9C00) as usize] = value,
            0xFF => {
                match addr & 0xFF {
                    0x40 => { // LCD Control
                        self.lcd.controls = Controls::from_bits_truncate(value);
                    }
                    0x42 => self.scroll_y = value,
                    0x47 => {
                        self.bg_palette.color_0 = Color::from_u8((value >> 0) & 0x3);
                        self.bg_palette.color_1 = Color::from_u8((value >> 2) & 0x3);
                        self.bg_palette.color_2 = Color::from_u8((value >> 4) & 0x3);
                        self.bg_palette.color_3 = Color::from_u8((value >> 6) & 0x3);
                    },
                    _ => panic!("Unimpletmented GPU Register write at {:#04X}: {:#04X}", addr, value)
                }
            }
            _ => panic!("Unimplemented GPU write at {:#04X}", addr)
        }
    }
}

#[derive(Debug)]
struct Palette {
    color_0: Color,
    color_1: Color,
    color_2: Color,
    color_3: Color,
}

impl Palette {
    fn new() -> Self {
        Palette {
            color_0: Color::White,
            color_1: Color::White,
            color_2: Color::White,
            color_3: Color::White,
        }
    }
}

#[derive(Debug)]
enum Color {
    White,
    LightGray,
    DarkGray,
    Black,
}

impl Color {
    fn from_u8(value: u8) -> Color {
        match value {
            0x00 => Color::White,
            0x01 => Color::LightGray,
            0x02 => Color::DarkGray,
            0x03 => Color::Black,
            _ => panic!("Unrecoginized Color: {:#X}", value)
        }
    }
}

struct Lcd {
     controls: Controls,
}

impl Lcd {
    fn new() -> Self {
        Lcd {
            controls: Controls::empty(),
        }
    }
}

bitflags! {
    flags Controls: u8 {
        const LCD_ENABLE        = 1 << 7,
        const WINDOW_MAP_SELECT = 1 << 6,
        const WINDOW_ENABLE     = 1 << 5,
        const BG_WINDOW_SELECT  = 1 << 4,
        const BG_MAP_SELECT     = 1 << 3,
        const SPRITE_SIZE       = 1 << 2,
        const SPRINT_ENABLE     = 1 << 1,
        const BG_ENABLE         = 1 << 0,
    }
}
