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
    fn master_isr();
}

const DISPCNT: Register<u16, ReadWrite> = Register::new(0x0400_0000);
const DISPSTAT: Register<u16, ReadWrite> = Register::new(0x0400_0004);
const VCOUNT: Register<u16, ReadOnly> = Register::new(0x0400_0006);

const IE: Register<u16, ReadWrite> = Register::new(0x0400_0200);
const IME: Register<u16, ReadWrite> = Register::new(0x0400_0208);

const KEYINPUT: Register<u16, ReadOnly> = Register::new(0x0400_0130);
const KEY_A: u16 = 1 << 0;
const KEY_B: u16 = 1 << 1;
const KEY_SELECT: u16 = 1 << 2;
const KEY_START: u16 = 1 << 3;
const KEY_RIGHT: u16 = 1 << 4;
const KEY_LEFT: u16 = 1 << 5;
const KEY_UP: u16 = 1 << 6;
const KEY_DOWN: u16 = 1 << 7;
const KEY_R: u16 = 1 << 8;
const KEY_L: u16 = 1 << 9;
const KEY_MASK: u16 = 0b0000_0011_1111_1111;

static mut INPUT_PREV: u16 = 0;
static mut INPUT_CURRENT: u16 = 0;

fn key_current_state() -> u16 {
    unsafe { INPUT_CURRENT }
}

fn key_prev_state() -> u16 {
    unsafe { INPUT_PREV }
}

fn key_poll() {
    unsafe {
        INPUT_PREV = INPUT_CURRENT;
        INPUT_CURRENT = !KEYINPUT.read() & KEY_MASK;
    }
}

fn key_is_down(key: u16) -> bool {
    (key_current_state() & key) != 0
}

fn key_is_up(key: u16) -> bool {
    (!key_current_state() & key) != 0
}

fn key_was_down(key: u16) -> bool {
    (key_prev_state() & key) != 0
}

fn key_was_up(key: u16) -> bool {
    (!key_prev_state() & key) != 0
}

fn key_down(key: u16) -> bool {
    key_was_up(key) && key_is_down(key)
}

fn key_up(key: u16) -> bool {
    key_was_down(key) && key_is_up(key)
}

#[derive(PartialEq)]
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
        key_poll();

        if key_down(KEY_A) {
            color = CYAN;
        } else if key_down(KEY_B) {
            color = YELLOW;
        } else if key_down(KEY_START) {
            color = WHITE;
        } else if key_down(KEY_SELECT) {
            color = BLACK;
        } else if key_down(KEY_RIGHT) {
            color = RED;
        } else if key_down(KEY_LEFT) {
            color = GREEN;
        } else if key_down(KEY_UP) {
            color = BLUE;
        } else if key_down(KEY_DOWN) {
            color = MAGENTA;
        } else if key_down(KEY_R) {
            color = LIGHT_GRAY;
        } else if key_down(KEY_L) {
            color = GRAY;
        }

        draw_pixel(x, y, &color);

        x += 1;
        if x >= DISPLAY_WIDTH {
            x = 0;
            y += 1;
        }
        if y >= DISPLAY_HEIGHT {
            y = 0;
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { core::intrinsics::abort() }
}
