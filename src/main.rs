// SPDX-License-Identifier: MPL-2.0

/*
 * Copyright (c) 2019 Tim Crawford <crawfxrd@gmail.com>
 */

#![feature(core_intrinsics)]

#![no_std]
#![no_main]
#![allow(dead_code)]
#![deny(warnings)]

use core::panic::PanicInfo;

struct Color(u16);

impl Color {
    pub const fn new(red: u32, green: u32, blue: u32) -> Color {
        Color(((red & 0x1F) | ((green & 0x1F) << 5) | ((blue & 0x1F) << 10)) as u16)
    }
}

const RED: Color = Color::new(0x1F, 0, 0);
const GREEN: Color = Color::new(0, 0x1F, 0);
const BLUE: Color = Color::new(0, 0, 0x1F);

const MODE3: u16 = 0x3;
const ENABLE_BG2: u16 = 1 << 10;

fn dispcnt(val: u16) {
    unsafe {
        (0x400_0000 as *mut u16).write_volatile(val);
    }
}

fn draw_pixel(x: u32, y: u32, color: Color) {
    unsafe {
        (0x600_0000 as *mut u16)
            .offset((x + y * 240) as isize)
            .write_volatile(color.0);
    }
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    let _x = 42;

    dispcnt(MODE3 | ENABLE_BG2);
    draw_pixel(104, 80, RED);
    draw_pixel(120, 80, GREEN);
    draw_pixel(136, 80, BLUE);

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { core::intrinsics::abort() }
}
