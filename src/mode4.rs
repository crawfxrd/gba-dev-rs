// SPDX-FileCopyrightText: 2020 Tim Crawford <crawfxrd@gmail>
// SPDX-License-Identifier: MPL-2.0

use crate::register::{ReadWrite, Register};

pub const VRAM: *mut u16 = 0x0600_0000 as *mut u16;

const DISPCNT: Register<u16, ReadWrite> = Register::new(0x0400_0000);
const MODE4: u16 = 0x4;
const SELECT_FRAME: u16 = 1 << 4;
const ENABLE_BG2: u16 = 1 << 10;

pub struct Mode4 {
    pub vram: *mut u16,
}

impl Mode4 {
    pub const WIDTH: u32 = 240;
    pub const HEIGHT: u32 = 160;
    pub const FRAME_SIZE: usize = 0xA000;

    pub fn new() -> Self {
        DISPCNT.write(MODE4 | ENABLE_BG2);
        Self { vram: VRAM }
    }

    pub fn vflip(&mut self) {
        self.vram = (self.vram as usize ^ Self::FRAME_SIZE) as *mut u16;
        DISPCNT.write(DISPCNT.read() ^ SELECT_FRAME);
    }

    // Set the pixel at (x, y) to the color of the given palette index
    pub fn draw_index(&self, x: u32, y: u32, color: u8) {
        if x >= Self::WIDTH || y >= Self::HEIGHT {
            // TODO: Handle better
            panic!();
        }

        // In mode 4, each pixel is a byte, representing the palette index of
        // the color. However, VRAM must be accessed with u16 or u32.
        let pos = x + y * Self::WIDTH;

        unsafe {
            // So first determine offset by converting u8 to u16.
            let addr = self.vram.offset((pos / 2) as isize);

            // Then set the correct byte of the u16 while preserving the other.
            let prev = addr.read_volatile();
            let value = if (pos & 1) == 1 {
                (prev & 0x00FF) | ((color as u16) << 8)
            } else {
                (prev & 0xFF00) | (color as u16)
            };

            addr.write_volatile(value);
        }
    }
}
