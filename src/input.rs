// SPDX-License-Identifier: MPL-2.0

/*
 * Copyright 2019 Tim Crawford <crawfxrd@gmail.com>
 */

use crate::register::{ReadOnly, Register};

#[allow(clippy::identity_op)]
#[derive(Clone, Copy)]
pub enum Key {
    A = 1 << 0,
    B = 1 << 1,
    Select = 1 << 2,
    Start = 1 << 3,
    Right = 1 << 4,
    Left = 1 << 5,
    Up = 1 << 6,
    Down = 1 << 7,
    R = 1 << 8,
    L = 1 << 9,
}

pub struct Input {
    previous: u16,
    current: u16,
}

impl Input {
    const KEYINPUT: Register<u16, ReadOnly> = Register::new(0x0400_0130);
    const KEY_MASK: u16 = 0b0000_0011_1111_1111;

    pub const fn new() -> Self {
        Self {
            previous: 0,
            current: 0,
        }
    }

    pub fn poll(&mut self) {
        self.previous = self.current;
        self.current = !Input::KEYINPUT.read() & Input::KEY_MASK;
    }

    pub fn key_is_down(&self, key: Key) -> bool {
        (self.current & key as u16) != 0
    }

    pub fn key_is_up(&self, key: Key) -> bool {
        (!self.current & key as u16) != 0
    }

    pub fn key_was_down(&self, key: Key) -> bool {
        (self.previous & key as u16) != 0
    }

    pub fn key_was_up(&self, key: Key) -> bool {
        (!self.previous & key as u16) != 0
    }

    pub fn key_down(&self, key: Key) -> bool {
        self.key_was_up(key) && self.key_is_down(key)
    }

    pub fn key_up(&self, key: Key) -> bool {
        self.key_was_down(key) && self.key_is_up(key)
    }
}
