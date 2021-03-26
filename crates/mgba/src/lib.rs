// SPDX-License-Identifier: MPL-2.0
// SPDX-FileCopyrightText: 2021 Tim Crawford <crawfxrd@gmail.com>

#![no_std]
#![deny(warnings)]

use core::fmt;

mod macros;

const MGBA_DEBUG_FLAGS: *mut u16 = 0x04FF_F700 as *mut u16;
const MGBA_DEBUG_ENABLE: *mut u16 = 0x04FF_F780 as *mut u16;
const MGBA_DEBUG_STRING: *mut u8 = 0x04FF_F600 as *mut u8;

pub enum Level {
    Fatal = 0,
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
}

pub fn enable() -> bool {
    unsafe { MGBA_DEBUG_ENABLE.write_volatile(0xC0DE); }
    enabled()
}

pub fn enabled() -> bool {
    unsafe { MGBA_DEBUG_ENABLE.read_volatile() == 0x1DEA }
}

pub fn log(level: Level, args: fmt::Arguments) {
    let mut b = Buffer::new();
    let _ = fmt::write(&mut b, args);
    flush(level);
}

fn flush(level: Level) {
    unsafe { MGBA_DEBUG_FLAGS.write_volatile(0x0100 | level as u16); }
}

struct Buffer {
    offset: usize,
}

impl Buffer {
    fn new() -> Self {
        Self { offset: 0 }
    }
}

impl fmt::Write for Buffer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for &b in s.as_bytes() {
            // mGBA reserves 0x100 bytes for the debug string
            if self.offset >= 0x100 {
                break;
            }

            unsafe {
                MGBA_DEBUG_STRING.add(self.offset).write_volatile(b);
            }

            self.offset += 1;
        }

        Ok(())
    }
}
