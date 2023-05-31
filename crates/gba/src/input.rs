// SPDX-License-Identifier: MPL-2.0
// SPDX-FileCopyrightText: 2022 Tim Crawford <crawfxrd@gmail.com>

//! Input handling for hardware keys.
//!
//! Note that GBA hardware uses inverted logic for keys. That is,
//!
//! - 0 indicated a key is pressed.
//! - 1 indicated a key is released.

use core::{fmt, ops};

use crate::register::{ReadOnly, Register};

/// A bit field that represents the raw state of hardware keys.
///
/// Bits 0-9 are used to represent each key's state.
/// Bits 10-15 are not used.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(transparent)]
pub struct Keys(u16);

impl Keys {
    /// A button
    pub const A: Self = Self(1 << 0);
    /// B button
    pub const B: Self = Self(1 << 1);
    /// Select button
    pub const SELECT: Self = Self(1 << 2);
    /// Start button
    pub const START: Self = Self(1 << 3);
    /// Control-pad right
    pub const RIGHT: Self = Self(1 << 4);
    /// Control-pad left
    pub const LEFT: Self = Self(1 << 5);
    /// Control-pad up
    pub const UP: Self = Self(1 << 6);
    /// Control-pad down
    pub const DOWN: Self = Self(1 << 7);
    /// Right shoulder button
    pub const R: Self = Self(1 << 8);
    /// Left shoulder button
    pub const L: Self = Self(1 << 9);

    /// The memory-mapped register of input keys state.
    const KEYINPUT: Register<Self, ReadOnly, 0x0400_0130> = unsafe { Register::new() };
    /// The mask of used bits from the register.
    const MASK: u16 = 0b0000_0011_1111_1111;
}

impl Keys {
    /// Returns the current state of all hardware keys.
    pub fn get() -> Self {
        Self::KEYINPUT.read() & Self(Self::MASK)
    }
}

impl Default for Keys {
    /// Returns the default value of all keys released.
    fn default() -> Self {
        Self(Self::MASK)
    }
}

impl From<u16> for Keys {
    fn from(value: u16) -> Self {
        Self(value & Keys::MASK)
    }
}

impl From<Keys> for u16 {
    fn from(value: Keys) -> Self {
        value.0
    }
}

impl ops::BitAnd for Keys {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl ops::BitOr for Keys {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl ops::BitXor for Keys {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl ops::Not for Keys {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0 & Keys::MASK)
    }
}

impl fmt::Binary for Keys {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Binary::fmt(&self.0, f)
    }
}

impl fmt::LowerHex for Keys {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::LowerHex::fmt(&self.0, f)
    }
}

impl fmt::UpperHex for Keys {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.0, f)
    }
}

/// Tracks basic state and provides convenience functions for key operations.
#[derive(Debug, Default)]
pub struct Input {
    previous: Keys,
    current: Keys,
}

impl Input {
    pub fn new() -> Self {
        Self::default()
    }

    /// Updates the state of the keys.
    /// Should be called once per game loop.
    pub fn update(&mut self) {
        self.previous = self.current;
        self.current = !Keys::get();
    }

    /// Returns the keys that are currently pressed.
    ///
    /// Bits set to 1 indicate that a key is pressed.
    pub fn keys_pressed(&self) -> Keys {
        self.current
    }

    /// Checks if `keys` are currently pressed.
    pub fn pressed(&self, keys: Keys) -> bool {
        (self.keys_pressed() & keys) == keys
    }

    /// Returns the keys that were previously released and are now pressed.
    ///
    /// Bits set to 1 indicate that a key was just pressed.
    pub fn keys_just_pressed(&self) -> Keys {
        !self.previous & self.current
    }

    /// Checks if `keys` were just pressed.
    pub fn just_pressed(&self, keys: Keys) -> bool {
        (self.keys_just_pressed() & keys) == keys
    }

    /// Returns the keys that were previously pressed and are now released.
    ///
    /// Bits set to 1 indicate that a key was just released.
    pub fn keys_just_released(&self) -> Keys {
        self.previous & !self.current
    }

    /// Checks if `keys` were just released.
    pub fn just_released(&self, keys: Keys) -> bool {
        (self.keys_just_released() & keys) == keys
    }
}
