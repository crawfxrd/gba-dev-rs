// SPDX-License-Identifier: MPL-2.0
// SPDX-FileCopyrightText: 2021 Tim Crawford <crawfxrd@gmail.com>

#![no_std]
#![no_main]

#[rustfmt::skip]
mod cyberpunk;

use gba::{bios, interrupt};

const DISPCNT: *mut u16 = 0x0400_0000 as *mut u16;
const VRAM: *mut u16 = 0x0600_0000 as *mut u16;

const MODE3: u16 = 0x3;
const ENABLE_BG2: u16 = 1 << 10;

#[no_mangle]
pub extern "C" fn main() -> ! {
    interrupt::init(interrupt::master_isr);
    interrupt::enable(interrupt::Irq::VBlank);

    unsafe {
        DISPCNT.write_volatile(MODE3 | ENABLE_BG2);
    }

    for (i, &color) in cyberpunk::DATA.iter().enumerate() {
        unsafe {
            VRAM.add(i).write_volatile(color);
        }
    }

    loop {
        bios::vblank();
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    interrupt::reset();
    loop {
        bios::halt();
    }
}
