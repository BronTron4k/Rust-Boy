const TILE_MAP_SIZE: usize = 0x180;
const BG_MAP_SIZE: usize = 0x400;
const SCREEN_HEIGHT: usize = 144;
const SCREEN_WIDTH: usize = 160;
const SCREEN_PIXELS: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

#[derive(Clone, Copy)]
struct Tile {
    data: [u8; 16],
}

pub struct Gpu {
    tile_map: [Tile; TILE_MAP_SIZE],
    bg_map_0: [u8; BG_MAP_SIZE],
    bg_map_1: [u8; BG_MAP_SIZE],
    bg_palette: Palette,

    lcd: Lcd,

    scroll_x: u8,
    scroll_y: u8,

    mode: Mode,

    screen_buffer: [Color; SCREEN_PIXELS],
}

impl Gpu {
    pub fn new() -> Self {
        Gpu {
            tile_map: [Tile {data: [0; 16]}; TILE_MAP_SIZE],
            bg_map_0: [0; BG_MAP_SIZE],
            bg_map_1: [0; BG_MAP_SIZE],
            bg_palette: Palette::new(),

            lcd: Lcd::new(),

            scroll_x: 0,
            scroll_y: 0,

            mode: Mode::AccessOam,

            screen_buffer: [Color::White; SCREEN_PIXELS],
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr >> 8 {
            0xFF => {
                match addr & 0xFF {
                    0x44 => self.lcd.current_line,
                    _ => panic!("Unimplemented GPU Register read at {:04X}", addr),
                }
            },
            _ => panic!("Unimplemented GPU read at {:04X}", addr),
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        match addr >> 8 {
            0x80...0x97 => {
                let rel_addr = addr - 0x8000;
                let tile = &mut self.tile_map[(rel_addr) as usize / 16];
                tile.data[rel_addr as usize % 16] = value;
            },
            0x98...0x9B => self.bg_map_0[(addr - 0x9800) as usize] = value,
            0x9C...0x9F => self.bg_map_0[(addr - 0x9C00) as usize] = value,
            0xFF => {
                match addr & 0xFF {
                    0x40 => { // LCD Control
                        let new_controls = Controls::from_bits_truncate(value);

                        if !new_controls.contains(LCD_ENABLE) &&
                            self.lcd.controls.contains(LCD_ENABLE) {
                            if self.mode != Mode::VBlank {
                                panic!("Cannot turn off LCD outside of VBLANK")
                            } else {
                                self.lcd.current_line = 0;
                            }
                        }

                        if new_controls.contains(LCD_ENABLE) &&
                            !self.lcd.controls.contains(LCD_ENABLE) {
                        }

                        self.lcd.controls = new_controls;
                    },
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

#[derive(Clone, Copy, Debug)]
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
     current_line: u8,
}

impl Lcd {
    fn new() -> Self {
        Lcd {
            controls: Controls::empty(),
            current_line: 0,
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

#[derive(PartialEq)]
enum Mode {
    AccessOam,
    AccessVram,
    HBlank,
    VBlank,
}
