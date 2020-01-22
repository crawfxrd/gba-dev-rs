// SPDX-License-Identifier: MPL-2.0

/*
 * Copyright (c) 2019 Tim Crawford <crawfxrd@gmail.com>
 */

#![feature(asm)]
#![feature(core_intrinsics)]
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

const MODE3: u16 = 0x3;
const ENABLE_BG2: u16 = 1 << 10;

fn vsync() {
    unsafe {
        asm!("svc 0x05" ::: "r0", "r1");
    }
}

struct Mode3;

impl Mode3 {
    const WIDTH: u32 = 240;
    const HEIGHT: u32 = 160;

    pub fn draw_pixel(x: u32, y: u32, color: Color) {
        unsafe {
            let addr = (0x0600_0000 as *mut u16).offset((x + y * Self::WIDTH) as isize);
            ptr::write_volatile(addr, color.0);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    extern "C" {
        fn master_isr();
    }

    interrupt::init(master_isr);
    interrupt::enable(Irq::VBlank);

    DISPCNT.write(MODE3 | ENABLE_BG2);

    let mut input = Input::new();

    let mut x = Mode3::WIDTH >> 1;
    let mut y = Mode3::HEIGHT >> 1;
    let mut color = CYAN;
    loop {
        vsync();
        input.poll();

        Mode3::draw_pixel(x, y, BLACK);

        if input.key_is_down(Key::Right) {
            if x < Mode3::WIDTH - 1 {
                x += 1;
            }
        }
        if input.key_is_down(Key::Left) {
            if x > 0 {
                x -= 1;
            }
        }

        if input.key_is_down(Key::Up) {
            if y > 0 {
                y -= 1;
            }
        }
        if input.key_is_down(Key::Down) {
            if y < Mode3::HEIGHT - 1 {
                y += 1;
            }
        }

        if input.key_down(Key::Start) {
            x = Mode3::WIDTH >> 1;
            y = Mode3::HEIGHT >> 1;
        }

        if input.key_down(Key::A) {
            color = CYAN;
        } else if input.key_down(Key::B) {
            color = YELLOW;
        } else if input.key_down(Key::R) {
            color = MAGENTA;
        } else if input.key_down(Key::L) {
            color = LIGHT_STEEL_BLUE;
        }

        Mode3::draw_pixel(x, y, color);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::intrinsics::abort() }
}
