// SPDX-FileCopyrightText: 2019 Tim Crawford <crawfxrd@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use core::marker::PhantomData;

pub struct ReadOnly;
pub struct ReadWrite;
pub struct WriteOnly;

pub struct Register<WIDTH, MODE> {
    address: *mut WIDTH,
    mode: PhantomData<MODE>,
}

impl<WIDTH, MODE> Register<WIDTH, MODE> {
    pub const fn new(address: u32) -> Self {
        Self {
            address: address as *mut WIDTH,
            mode: PhantomData,
        }
    }
}

impl<WIDTH> Register<WIDTH, ReadOnly> {
    pub fn read(&self) -> WIDTH {
        unsafe { self.address.read_volatile() }
    }
}

impl<WIDTH> Register<WIDTH, WriteOnly> {
    pub fn write(&self, value: WIDTH) {
        unsafe {
            self.address.write_volatile(value);
        }
    }
}

impl<WIDTH> Register<WIDTH, ReadWrite> {
    pub fn read(&self) -> WIDTH {
        unsafe { self.address.read_volatile() }
    }

    pub fn write(&self, value: WIDTH) {
        unsafe {
            self.address.write_volatile(value);
        }
    }
}
