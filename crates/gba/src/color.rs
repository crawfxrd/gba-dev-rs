// SPDX-License-Identifier: MPL-2.0
// SPDX-FileCopyrightText: 2021 Tim Crawford <crawfxrd@gmail.com>

/// A 15-bit color, with each RGB component represented as 5 bits.
#[derive(Clone, Copy, PartialEq)]
pub struct Color(u16);

#[rustfmt::skip]
impl Color {
    pub const BLACK: Color = Color::new(0, 0, 0);
    pub const WHITE: Color = Color::new(0x1F, 0x1F, 0x1F);

    pub const BLUE: Color = Color::new(0, 0, 0x1F);
    pub const GREEN: Color = Color::new(0, 0x1F, 0);
    pub const RED: Color = Color::new(0x1F, 0, 0);

    pub const CYAN: Color = Color::new(0, 0x1F, 0x1F);
    pub const MAGENTA: Color = Color::new(0x1F, 0, 0x1F);
    pub const YELLOW: Color = Color::new(0x1F, 0x1F, 0);

    pub const fn new(red: u16, green: u16, blue: u16) -> Self {
        Self((red & 0x1F) | ((green & 0x1F) << 5) | ((blue & 0x1F) << 10))
    }

    /// The red component of a color.
    pub const fn red(&self) -> u16 {
        self.0 & 0x1F
    }

    /// The green component of a color.
    pub const fn green(&self) -> u16 {
        (self.0 >> 5) & 0x1F
    }

    /// The blue component of a color.
    pub const fn blue(&self) -> u16 {
        (self.0 >> 10) & 0x1F
    }
}

impl From<Color> for u16 {
    fn from(color: Color) -> Self {
        color.0
    }
}

impl From<u16> for Color {
    fn from(value: u16) -> Self {
        Self { 0: value }
    }
}
