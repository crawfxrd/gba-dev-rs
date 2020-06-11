// SPDX-License-Identifier: MPL-2.0

/*
 * Copyright (c) 2020 Tim Crawford <crawfxrd@gmail.com>
 */

#![feature(core_intrinsics)]
#![feature(llvm_asm)]
#![no_std]
#![no_main]
#![deny(warnings)]
#![allow(dead_code)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::missing_safety_doc)]

mod input;
mod interrupt;
mod mgba;
mod register;

use core::ptr;
use input::{Input, Key};
use interrupt::Irq;
use register::{ReadWrite, Register};

const DISPCNT: Register<u16, ReadWrite> = Register::new(0x0400_0000);
const VRAM: *mut u16 = 0x0600_0000 as *mut u16;

#[derive(Clone, Copy, PartialEq)]
struct Color(u16);

impl Color {
    pub const fn new(red: u32, green: u32, blue: u32) -> Self {
        Self(((red & 0x1F) | ((green & 0x1F) << 5) | ((blue & 0x1F) << 10)) as u16)
    }
}

const BLACK: Color = Color::new(0, 0, 0);
const GRAY: Color = Color::new(0x08, 0x08, 0x08);
const LIGHT_GRAY: Color = Color::new(0x1C, 0x1C, 0x1C);
const WHITE: Color = Color::new(0x1F, 0x1F, 0x1F);
const RED: Color = Color::new(0x1F, 0, 0);
const GREEN: Color = Color::new(0, 0x1F, 0);
const BLUE: Color = Color::new(0, 0, 0x1F);
const MAGENTA: Color = Color::new(0x1F, 0, 0x1F);
const CYAN: Color = Color::new(0, 0x1F, 0x1F);
const YELLOW: Color = Color::new(0x1F, 0x1F, 0);
const LIGHT_STEEL_BLUE: Color = Color::new(0x16, 0x18, 0x1B);

const MODE4: u16 = 0x4;
const SELECT_FRAME: u16 = 1 << 4;
const ENABLE_BG2: u16 = 1 << 10;

fn vsync() {
    unsafe {
        llvm_asm!("svc 0x05" ::: "r0", "r1");
    }
}

struct Mode4 {
    vram: *mut u16,
}

impl Mode4 {
    const WIDTH: u32 = 240;
    const HEIGHT: u32 = 160;
    const FRAME_SIZE: usize = 0xA000;

    fn new(dispcnt: u16) -> Self {
        DISPCNT.write(MODE4 | dispcnt);
        Self { vram: VRAM }
    }

    fn vflip(&mut self) {
        self.vram = (self.vram as usize ^ Self::FRAME_SIZE) as *mut u16;
        DISPCNT.write(DISPCNT.read() ^ SELECT_FRAME);
    }

    // Set the pixel at (x, y) to the color of the given palette index
    unsafe fn draw_index(&self, x: u32, y: u32, color: u8) {
        // In mode 4, each pixel is a byte, representing the palette index of
        // the color. However, VRAM must be accessed with u16 or u32.
        let pos = x + y * Self::WIDTH;

        // So first determine offset by converting u8 to u16.
        let addr = self.vram.offset((pos / 2) as isize);

        // Then set the correct byte of the u16 while preserving the other.
        let prev = ptr::read_volatile(addr);
        let value = if (pos & 1) == 1 {
            (prev & 0x00FF) | ((color as u16) << 8)
        } else {
            (prev & 0xFF00) | (color as u16)
        };
        ptr::write_volatile(addr, value);
    }
}

struct Palette;

impl Palette {
    const PALETTE: *mut u16 = 0x0500_0000 as *mut u16;
    fn set(index: u8, color: Color) {
        unsafe {
            ptr::write_volatile(Self::PALETTE.offset(index as isize), color.0);
        }
    }
}

fn set_palette() {
    Palette::set(0, BLACK);
    Palette::set(1, WHITE);
    Palette::set(2, Color::new(0x18, 0x19, 0x19));
    Palette::set(3, Color::new(0x0D, 0x10, 0x10));
    Palette::set(4, Color::new(0x0A, 0x0D, 0x0D));
    Palette::set(5, RED);
    Palette::set(6, GREEN);
    Palette::set(7, BLUE);
    Palette::set(8, MAGENTA);
    Palette::set(9, CYAN);
    Palette::set(10, YELLOW);
    Palette::set(11, LIGHT_STEEL_BLUE);
}

fn draw_copyright_symbol(display: &Mode4) {
    const COPYRIGHT: [u16; 32] = [
        0x0000, 0x0102, 0x0201, 0x0000,
        0x0100, 0x0000, 0x0000, 0x0001,
        0x0002, 0x0104, 0x0301, 0x0200,
        0x0001, 0x0001, 0x0000, 0x0100,
        0x0001, 0x0001, 0x0000, 0x0100,
        0x0002, 0x0104, 0x0301, 0x0200,
        0x0100, 0x0000, 0x0000, 0x0001,
        0x0000, 0x0102, 0x0201, 0x0000,
    ];

    // Offset to put it in the bottom left corner
    let pos = (Mode4::WIDTH * (Mode4::HEIGHT - 8) / 2) as isize;

    for i in (0..32).step_by(4) {
        unsafe {
            display.vram.offset(pos + (i / 4) * 120 + 0).write_volatile(COPYRIGHT[i as usize]);
            display.vram.offset(pos + (i / 4) * 120 + 1).write_volatile(COPYRIGHT[(i + 1) as usize]);
            display.vram.offset(pos + (i / 4) * 120 + 2).write_volatile(COPYRIGHT[(i + 2) as usize]);
            display.vram.offset(pos + (i / 4) * 120 + 3).write_volatile(COPYRIGHT[(i + 3) as usize]);
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

    unsafe fn render(&self, display: &Mode4) {
        display.draw_index(self.x, self.y, self.color);
    }

    fn update(&mut self, input: &Input) {
        if input.key_is_down(Key::Right) {
            if self.x < Mode4::WIDTH - 1 {
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
            if self.y < Mode4::HEIGHT - 1 {
                self.y += 1;
            }
        }

        if input.key_down(Key::Start) {
            self.x = Mode4::WIDTH / 2;
            self.y = Mode4::HEIGHT / 2;
        }

        if input.key_down(Key::A) {
            self.color = 9;
        } else if input.key_down(Key::B) {
            self.color = 10;
        } else if input.key_down(Key::R) {
            self.color = 8;
        } else if input.key_down(Key::L) {
            self.color = 11;
        }
    }
}

// XXX: Is it safe to call Rust from asm?
#[no_mangle]
pub extern "C" fn main() -> ! {
    mgba::enable();
    mgba::log(mgba::Level::Info, "Testing mGBA logging");

    extern "C" {
        fn master_isr();
    }

    interrupt::init(master_isr);
    interrupt::enable(Irq::VBlank);

    let mut display = Mode4::new(ENABLE_BG2);

    set_palette();

    let mut input = Input::new();
    let mut pxl = Pixel::new(Mode4::WIDTH / 2, Mode4::HEIGHT / 2, 9);

    loop {
        vsync();
        input.poll();

        // XXX: Background not redrawn on new frame. Fill current pixel with
        // background color to not "streak" when moving.
        unsafe { display.draw_index(pxl.x, pxl.y, 0); }
        display.vflip();

        draw_copyright_symbol(&display);

        pxl.update(&input);
        unsafe { pxl.render(&display); }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // XXX: Marked safe by rust-lang/rust#72204
    core::intrinsics::abort();
}
