// display (monochrome, 64x32 or 128x64 for SUPER-CHIP)

use minifb::{Key::*, Window, WindowOptions};

const SCALE: usize = 8;
pub const LO_WIDTH: usize = 64;
pub const LO_HEIGHT: usize = 32;
pub const HI_WIDTH: usize = 128;
pub const HI_HEIGHT: usize = 64;
const OFF: u32 = 0;
const ON: u32 = u32::MAX;

pub struct Display {
    pub width: usize,
    pub height: usize,
    buffer: [u32; HI_WIDTH * HI_HEIGHT],
    pub window: Window,
    hires: bool,
    pub just_updated: bool,
    pub just_pressed_key: bool,
}

impl Display {
    pub fn with_fps(fps: usize) -> Display {
        let mut disp = Display {
            width: LO_WIDTH,
            height: LO_HEIGHT,
            buffer: [OFF; HI_WIDTH * HI_HEIGHT],
            window: Window::new(
                "Chip-8",
                HI_WIDTH * SCALE,
                HI_HEIGHT * SCALE,
                WindowOptions::default(),
            )
            .unwrap_or_else(|e| {
                panic!("{e}");
            }),
            hires: false,
            just_updated: true,
            just_pressed_key: false,
        };
        disp.window.set_target_fps(fps);
        disp
    }
    pub fn clear(&mut self) {
        self.buffer.fill(OFF);
    }
    // NOTE: this may or may not work
    pub fn draw_at(&mut self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        let mut i = x + y * HI_WIDTH;
        if self.hires {
            self.buffer[i] = if self.buffer[i] == ON { OFF } else { ON };
        } else {
            i *= 2;
            self.buffer[i] = if self.buffer[i] == ON { OFF } else { ON };
            self.buffer[i + 1] = if self.buffer[i + 1] == ON { OFF } else { ON };
            self.buffer[i + HI_WIDTH] = if self.buffer[i + HI_WIDTH] == ON {
                OFF
            } else {
                ON
            };
            self.buffer[i + HI_WIDTH + 1] = if self.buffer[i + HI_WIDTH + 1] == ON {
                OFF
            } else {
                ON
            };
        }
        // did we "turn off" the pixel?
        // e.g. was there a collision?
        self.buffer[i] == OFF
    }
    pub fn key_pressed(&mut self, key: u8) -> bool {
        self.just_pressed_key = self.window.is_key_down(match key {
            0x1 => Key1,
            0x2 => Key2,
            0x3 => Key3,
            0xC => Key4,
            0x4 => Q,
            0x5 => W,
            0x6 => E,
            0xD => R,
            0x7 => A,
            0x8 => S,
            0x9 => D,
            0xE => F,
            0xA => Z,
            0x0 => X,
            0xB => C,
            0xF => V,
            _ => panic!(),
        });
        self.just_pressed_key
    }
    pub fn scroll_down(&mut self) {
        self.buffer.rotate_right(HI_WIDTH);
        for i in 0..HI_WIDTH {
            self.buffer[i] = OFF;
        }
    }
    pub fn scroll_left(&mut self) {
        for y in 0..HI_HEIGHT {
            for x in 0..(HI_WIDTH - 4) {
                self.buffer[x + y * HI_WIDTH] = self.buffer[x + y * HI_WIDTH + 4]
            }
            for x in (HI_WIDTH - 4)..HI_WIDTH {
                self.buffer[x + y * HI_WIDTH] = OFF;
            }
        }
    }
    pub fn scroll_right(&mut self) {
        for y in 0..HI_HEIGHT {
            for x in (4..HI_WIDTH).rev() {
                self.buffer[x + y * HI_WIDTH] = self.buffer[x + y * HI_WIDTH - 4]
            }
            for x in 0..4 {
                self.buffer[x + y * HI_WIDTH] = OFF;
            }
        }
    }
    pub fn scroll_up(&mut self) {
        self.buffer.rotate_left(HI_WIDTH);
        for i in (HI_WIDTH * (HI_HEIGHT - 1))..(HI_WIDTH * HI_HEIGHT) {
            self.buffer[i] = OFF;
        }
    }
    pub fn set_hires(&mut self) {
        self.hires = true;
        self.width = HI_WIDTH;
        self.height = HI_HEIGHT;
    }
    pub fn set_lores(&mut self) {
        self.hires = false;
        self.width = LO_WIDTH;
        self.height = LO_HEIGHT;
    }
    pub fn update(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, HI_WIDTH, HI_HEIGHT)
            .unwrap();
        self.just_updated = true;
    }
}
