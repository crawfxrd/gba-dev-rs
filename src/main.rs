// SPDX-License-Identifier: MPL-2.0
// SPDX-FileCopyrightText: 2021 Tim Crawford <crawfxrd@gmail.com>

#![no_std]
#![no_main]
#![allow(clippy::collapsible_if)]
#![allow(clippy::missing_safety_doc)]

use untitled::bios;
use untitled::color::Color;
use untitled::input::{Input, Key};
use untitled::interrupt::{self, Irq};
use untitled::mode4::*;

struct Palette;

impl Palette {
    const PALETTE: *mut u16 = 0x0500_0000 as *mut u16;
    fn set(index: u8, color: Color) {
        unsafe {
            Self::PALETTE
                .offset(index as isize)
                .write_volatile(u16::from(color));
        }
    }
}

fn set_palette() {
    Palette::set(0, Color::BLACK);
    Palette::set(1, Color::WHITE);
    Palette::set(2, Color::new(0x18, 0x19, 0x19));
    Palette::set(3, Color::new(0x0D, 0x10, 0x10));
    Palette::set(4, Color::new(0x0A, 0x0D, 0x0D));
    Palette::set(5, Color::RED);
    Palette::set(6, Color::GREEN);
    Palette::set(7, Color::BLUE);
    Palette::set(8, Color::MAGENTA);
    Palette::set(9, Color::CYAN);
    Palette::set(10, Color::YELLOW);
    Palette::set(11, Color::LIGHT_STEEL_BLUE);
}

fn draw_copyright_symbol(display: &Mode4) {
    const COPYRIGHT: [[u16; 4]; 8] = [
        [0x0000, 0x0102, 0x0201, 0x0000],
        [0x0100, 0x0000, 0x0000, 0x0001],
        [0x0002, 0x0104, 0x0301, 0x0200],
        [0x0001, 0x0001, 0x0000, 0x0100],
        [0x0001, 0x0001, 0x0000, 0x0100],
        [0x0002, 0x0104, 0x0301, 0x0200],
        [0x0100, 0x0000, 0x0000, 0x0001],
        [0x0000, 0x0102, 0x0201, 0x0000],
    ];

    // Offset to put it in the bottom left corner
    let pos = (display.width() * (display.height() - 8) / 2) as usize;

    for (i, row) in COPYRIGHT.iter().enumerate() {
        unsafe {
            #[allow(clippy::identity_op)]
            display.write(pos + i * 120 + 0, row[0]);
            display.write(pos + i * 120 + 1, row[1]);
            display.write(pos + i * 120 + 2, row[2]);
            display.write(pos + i * 120 + 3, row[3]);
        }
    }
}

struct Pixel {
    x: u32,
    y: u32,
    color: u8,
}

impl Pixel {
    fn new(x: u32, y: u32, color: u8) -> Self {
        Self { x, y, color }
    }

    fn render(&self, display: &Mode4) {
        display.draw_index(self.x, self.y, self.color);
    }

    fn update(&mut self, display: &Mode4, input: &Input) {
        if input.key_is_down(Key::Right) {
            if self.x < display.width() - 1 {
                self.x += 1;
            }
        }
        if input.key_is_down(Key::Left) {
            if self.x > 0 {
                self.x -= 1;
            }
        }

        if input.key_is_down(Key::Up) {
            if self.y > 0 {
                self.y -= 1;
            }
        }
        if input.key_is_down(Key::Down) {
            if self.y < display.height() - 1 {
                self.y += 1;
            }
        }

        if input.key_down(Key::Start) {
            self.x = display.width() / 2;
            self.y = display.height() / 2;
        }

        if input.key_down(Key::A) {
            mgba::info!("Color = CYAN");
            self.color = 9;
        } else if input.key_down(Key::B) {
            mgba::info!("Color = YELLOW");
            self.color = 10;
        } else if input.key_down(Key::R) {
            mgba::info!("Color = MAGENTA");
            self.color = 8;
        } else if input.key_down(Key::L) {
            mgba::info!("Color = LIGHT_STEEL_BLUE");
            self.color = 11;
        }
    }
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    mgba::enable();
    mgba::info!("Testing mGBA logging");

    extern "C" {
        fn master_isr();
    }

    interrupt::init(master_isr);
    interrupt::enable(Irq::VBlank);

    let mut display = Mode4::new();

    set_palette();

    let mut input = Input::new();
    let mut pxl = Pixel::new(display.width() / 2, display.height() / 2, 9);

    loop {
        bios::vblank();
        input.poll();

        // XXX: Background not redrawn on new frame. Fill current pixel with
        // background color to not "streak" when moving.
        display.draw_index(pxl.x, pxl.y, 0);
        display.vflip();

        draw_copyright_symbol(&display);

        pxl.update(&display, &input);
        pxl.render(&display);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    interrupt::reset();
    loop {
        bios::stop();
    }
}
