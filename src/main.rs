// SPDX-License-Identifier: MPL-2.0

/*
 * Copyright (c) 2019 Tim Crawford <crawfxrd@gmail.com>
 */

#![feature(asm)]
#![feature(core_intrinsics)]
#![no_std]
#![no_main]
#![allow(dead_code)]
#![deny(warnings)]

mod register;

use core::panic::PanicInfo;
use core::ptr;
use register::{ReadOnly, ReadWrite, Register};

type IrqHandler = unsafe extern "C" fn();
const IRQ_HANDLER: Register<IrqHandler, ReadWrite> = Register::new(0x0300_7FFC);

extern "C" {
    pub fn master_isr();
}

const DISPCNT: Register<u16, ReadWrite> = Register::new(0x0400_0000);
const DISPSTAT: Register<u16, ReadWrite> = Register::new(0x0400_0004);
const VCOUNT: Register<u16, ReadOnly> = Register::new(0x0400_0006);

const IE: Register<u16, ReadWrite> = Register::new(0x0400_0200);
const IME: Register<u16, ReadWrite> = Register::new(0x0400_0208);

#[derive(PartialEq)]
struct Color(u16);

impl Color {
    pub const fn new(red: u32, green: u32, blue: u32) -> Self {
        Self(((red & 0x1F) | ((green & 0x1F) << 5) | ((blue & 0x1F) << 10)) as u16)
    }
}

const BLACK: Color = Color::new(0, 0, 0);
const WHITE: Color = Color::new(0x1F, 0x1F, 0x1F);
const RED: Color = Color::new(0x1F, 0, 0);
const GREEN: Color = Color::new(0, 0x1F, 0);
const BLUE: Color = Color::new(0, 0, 0x1F);
const MAGENTA: Color = Color::new(0x1F, 0, 0x1F);
const CYAN: Color = Color::new(0, 0x1F, 0x1F);
const YELLOW: Color = Color::new(0x1F, 0x1F, 0);

const DISPLAY_WIDTH: u32 = 240;
const DISPLAY_HEIGHT: u32 = 160;
const MODE3: u16 = 0x3;
const ENABLE_BG2: u16 = 1 << 10;

fn vsync() {
    unsafe {
        asm!("svc 0x05" ::: "r0", "r1");
    }
}

fn draw_pixel(x: u32, y: u32, color: &Color) {
    unsafe {
        let addr = (0x0600_0000 as *mut u16).offset((x + y * DISPLAY_WIDTH) as isize);
        ptr::write_volatile(addr, color.0);
    }
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    DISPCNT.write(MODE3 | ENABLE_BG2);

    draw_pixel(104, 80, &MAGENTA);
    draw_pixel(120, 80, &CYAN);
    draw_pixel(136, 80, &YELLOW);

    IRQ_HANDLER.write(master_isr);

    // Enable VBLANK interrupt
    DISPSTAT.write(DISPSTAT.read() | (1 << 3));
    IE.write(1);
    IME.write(1);

    let mut x = 0;
    let mut y = 0;
    let mut color = WHITE;
    loop {
        vsync();

        draw_pixel(x, y, &color);

        x += 1;
        if x >= DISPLAY_WIDTH {
            x = 0;
            y += 1;
        }
        if y >= DISPLAY_HEIGHT {
            y = 0;
            color = if color == BLACK { WHITE } else { BLACK };
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { core::intrinsics::abort() }
}
