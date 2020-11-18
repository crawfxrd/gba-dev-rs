// SPDX-FileCopyrightText: 2020 Tim Crawford <crawfxrd@gmail>
// SPDX-License-Identifier: MPL-2.0

#[derive(Clone, Copy, PartialEq)]
pub struct Color(u16);

impl Color {
    pub const BLACK: Color = Color::new(0, 0, 0);
    pub const BLUE: Color = Color::new(0, 0, 0x1F);
    pub const CYAN: Color = Color::new(0, 0x1F, 0x1F);
    pub const GRAY: Color = Color::new(0x08, 0x08, 0x08);
    pub const GREEN: Color = Color::new(0, 0x1F, 0);
    pub const LIGHT_GRAY: Color = Color::new(0x1C, 0x1C, 0x1C);
    pub const LIGHT_STEEL_BLUE: Color = Color::new(0x16, 0x18, 0x1B);
    pub const MAGENTA: Color = Color::new(0x1F, 0, 0x1F);
    pub const RED: Color = Color::new(0x1F, 0, 0);
    pub const WHITE: Color = Color::new(0x1F, 0x1F, 0x1F);
    pub const YELLOW: Color = Color::new(0x1F, 0x1F, 0);

    pub const fn new(red: u16, green: u16, blue: u16) -> Self {
        Self((red & 0x1F) | ((green & 0x1F) << 5) | ((blue & 0x1F) << 10))
    }

    pub const fn red(&self) -> u16 {
        self.0 & 0x1F
    }

    pub const fn green(&self) -> u16 {
        (self.0 >> 5) & 0x1F
    }

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
