// SPDX-License-Identifier: MPL-2.0
// SPDX-FileCopyrightText: 2021 Tim Crawford <crawfxrd@gmail.com>

#![no_std]
#![feature(asm)]
#![deny(warnings)]
#![allow(dead_code)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::missing_safety_doc)]

pub mod color;
pub mod input;
pub mod interrupt;
pub mod mode4;
pub mod register;

#[inline]
pub fn vsync() {
    unsafe {
        asm!("svc 0x05",
            // Clobbers
            out("r0") _, out("r1") _
        );
    }
}

#[inline]
pub fn stop() {
    unsafe {
        asm!("svc 0x03");
    }
}
