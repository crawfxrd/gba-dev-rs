// SPDX-License-Identifier: MPL-2.0
// SPDX-FileCopyrightText: 2021 Tim Crawford <crawfxrd@gmail.com>

/// A 15-bit color, with each RGB component represented as 5 bits.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct Color(u16);

#[rustfmt::skip]
impl Color {
    pub const BLACK: Self = Self::new(0, 0, 0);
    pub const WHITE: Self = Self::new(0x1F, 0x1F, 0x1F);

    pub const BLUE: Self = Self::new(0, 0, 0x1F);
    pub const GREEN: Self = Self::new(0, 0x1F, 0);
    pub const RED: Self = Self::new(0x1F, 0, 0);

    pub const CYAN: Self = Self::new(0, 0x1F, 0x1F);
    pub const MAGENTA: Self = Self::new(0x1F, 0, 0x1F);
    pub const YELLOW: Self = Self::new(0x1F, 0x1F, 0);

    pub const fn new(red: u16, green: u16, blue: u16) -> Self {
        Self((red & 0x1F) | ((green & 0x1F) << 5) | ((blue & 0x1F) << 10))
    }

    /// The red component of a color.
    pub const fn red(&self) -> Self {
        Self(self.0 & 0x1F)
    }

    /// The green component of a color.
    pub const fn green(&self) -> Self {
        Self((self.0 >> 5) & 0x1F)
    }

    /// The blue component of a color.
    pub const fn blue(&self) -> Self {
        Self((self.0 >> 10) & 0x1F)
    }
}

impl const From<Color> for u16 {
    fn from(color: Color) -> Self {
        color.0
    }
}

impl const From<u16> for Color {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
