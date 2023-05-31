// SPDX-License-Identifier: MPL-2.0
// SPDX-FileCopyrightText: 2021 Tim Crawford <crawfxrd@gmail.com>

#![no_std]
#![allow(clippy::missing_safety_doc)]
#![deny(clippy::borrow_as_ptr)]
#![deny(clippy::cast_lossless)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unwrap_used)]

pub use gba_proc_macros::entry;

pub mod bios;
pub mod color;
pub mod input;
pub mod interrupt;
pub mod register;

#[doc(hidden)]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    interrupt::reset();
    loop {
        bios::stop();
    }
}
