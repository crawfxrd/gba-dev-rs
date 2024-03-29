// SPDX-License-Identifier: MPL-2.0
// SPDX-FileCopyrightText: 2021 Tim Crawford <crawfxrd@gmail.com>

use gba::register::{ReadWrite, Register};

const VRAM: *mut u16 = 0x0600_0000 as *mut u16;

const DISPCNT: Register<u16, ReadWrite, 0x0400_0000> = unsafe { Register::new() };
const MODE4: u16 = 0x4;
const SELECT_FRAME: u16 = 1 << 4;
const ENABLE_BG2: u16 = 1 << 10;

pub struct Mode4 {
    vram: *mut u16,
}

impl Mode4 {
    const FRAME_SIZE: usize = 0xA000;
    const HEIGHT: usize = 160;
    const WIDTH: usize = 240;

    pub fn new() -> Self {
        DISPCNT.write(MODE4 | ENABLE_BG2);
        Self { vram: VRAM }
    }

    pub const fn height(&self) -> usize {
        Self::HEIGHT
    }

    pub const fn width(&self) -> usize {
        Self::WIDTH
    }

    pub fn vflip(&mut self) {
        self.vram = (self.vram as usize ^ Self::FRAME_SIZE) as *mut u16;
        DISPCNT.write(DISPCNT.read() ^ SELECT_FRAME);
    }

    pub unsafe fn write(&self, offset: usize, value: u16) {
        self.vram.add(offset).write_volatile(value);
    }

    // Set the pixel at (x, y) to the color of the given palette index
    pub fn draw_index(&self, x: usize, y: usize, color: u8) {
        if x >= self.width() || y >= self.height() {
            // No-op if not visible
            return;
        }

        // In mode 4, each pixel is a byte, representing the palette index of
        // the color. However, VRAM must be accessed with u16 or u32.
        let pos = x + y * self.width();

        unsafe {
            // So first determine offset by converting u8 to u16.
            let addr = self.vram.add(pos / 2);

            // Then set the correct byte of the u16 while preserving the other.
            let prev = addr.read_volatile();
            let value = if (pos & 1) == 1 {
                (prev & 0x00FF) | (u16::from(color) << 8)
            } else {
                (prev & 0xFF00) | u16::from(color)
            };

            addr.write_volatile(value);
        }
    }
}

impl Default for Mode4 {
    fn default() -> Self {
        Self::new()
    }
}
