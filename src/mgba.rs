// SPDX-FileCopyrightText: 2020 Tim Crawford <crawfxrd@gmail>
// SPDX-License-Identifier: MPL-2.0

use crate::register::{ReadWrite, Register, WriteOnly};

const MGBA_DEBUG_FLAGS: Register<u16, WriteOnly> = Register::new(0x04FF_F700);
const MGBA_DEBUG_ENABLE: Register<u16, ReadWrite> = Register::new(0x04FF_F780);
const MGBA_DEBUG_STRING: *mut u8 = 0x04FF_F600 as *mut u8;

pub enum Level {
    Fatal = 0,
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
}

pub fn enable() -> bool {
    MGBA_DEBUG_ENABLE.write(0xC0DE);
    enabled()
}

fn enabled() -> bool {
    MGBA_DEBUG_ENABLE.read() == 0x1DEA
}

pub fn log(level: Level, msg: &str) {
    for (i, &b) in msg.as_bytes().iter().enumerate() {
        // mGBA reserves 0x100 bytes for the debug string
        if i >= 0x100 {
            break;
        }

        unsafe {
            MGBA_DEBUG_STRING.add(i).write_volatile(b);
        }
    }

    flush(level);
}

fn flush(level: Level) {
    MGBA_DEBUG_FLAGS.write(0x0100 | level as u16);
}
