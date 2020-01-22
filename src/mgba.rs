// SPDX-License-Identifier: MPL-2.0

/*
 * Copyright (c) 2019 Tim Crawford <crawfxrd@gmail.com>
 */

use crate::register::{ReadWrite, Register, WriteOnly};

const MGBA_DEBUG_FLAGS: Register<u16, WriteOnly> = Register::new(0x04FF_F700);
const MGBA_DEBUG_ENABLE: Register<u16, ReadWrite> = Register::new(0x04FF_F780);

pub fn enable() -> bool {
    MGBA_DEBUG_ENABLE.write(0xC0DE);
    MGBA_DEBUG_ENABLE.read() == 0x1DEA
}

pub fn log(msg: &str) {
    let mut mgba_str = 0x04FF_F600 as *mut u8;
    for &b in msg.as_bytes().iter() {
        unsafe {
            mgba_str.write(b);
            mgba_str = mgba_str.offset(1);
        }
    }

    MGBA_DEBUG_FLAGS.write(0x0100 | 3);
}
