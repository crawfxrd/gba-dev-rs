// SPDX-License-Identifier: MPL-2.0
// SPDX-FileCopyrightText: 2022 Tim Crawford <crawfxrd@gmail.com>

use core::ops;

use crate::register::{ReadOnly, Register};

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct Keys(u16);

impl Keys {
    pub const A: Self = Self(1 << 0);
    pub const B: Self = Self(1 << 1);
    pub const SELECT: Self = Self(1 << 2);
    pub const START: Self = Self(1 << 3);
    pub const RIGHT: Self = Self(1 << 4);
    pub const LEFT: Self = Self(1 << 5);
    pub const UP: Self = Self(1 << 6);
    pub const DOWN: Self = Self(1 << 7);
    pub const R: Self = Self(1 << 8);
    pub const L: Self = Self(1 << 9);

    const KEYINPUT: Register<Self, ReadOnly, 0x0400_0130> = unsafe { Register::new() };
    const BIT_MASK: u16 = 0b0000_0011_1111_1111;
}

impl const From<u16> for Keys {
    fn from(value: u16) -> Self {
        Self(value & Keys::BIT_MASK)
    }
}

impl const From<Keys> for u16 {
    fn from(value: Keys) -> Self {
        value.0
    }
}

impl const ops::BitAnd for Keys {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl const ops::BitOr for Keys {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl const ops::BitXor for Keys {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl const ops::Not for Keys {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0 & Keys::BIT_MASK)
    }
}

// NOTE: Inverts KEYINPUT register to treat pressed keys as logic high.

#[derive(Debug, Default)]
pub struct Input {
    previous: Keys,
    current: Keys,
}

impl Input {
    pub fn new() -> Self {
        Self::default()
    }

    /// Update the state of the key inputs.
    /// Should be called once per game loop.
    pub fn poll(&mut self) {
        self.previous = self.current;
        self.current = !Keys::KEYINPUT.read();
    }

    /// Keys that are currently pressed, regardless of previous state.
    pub fn keys_down(&self) -> Keys {
        self.current
    }

    /// Keys that were previously released and now are pressed.
    pub fn keys_pressed(&self) -> Keys {
        !self.previous & self.current
    }

    /// Keys that were previously pressed and now are released.
    pub fn keys_released(&self) -> Keys {
        self.previous & !self.current
    }

    /// Keys that were previsouly pressed and are still pressed.
    pub fn keys_held(&self) -> Keys {
        !self.previous & !self.current
    }
}
