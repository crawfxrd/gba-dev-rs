// SPDX-FileCopyrightText: 2019 Tim Crawford <crawfxrd@gmail>
// SPDX-License-Identifier: MPL-2.0

use core::marker::PhantomData;
use core::ptr;

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
        unsafe { ptr::read_volatile(self.address) }
    }
}

impl<WIDTH> Register<WIDTH, WriteOnly> {
    pub fn write(&self, value: WIDTH) {
        unsafe {
            ptr::write_volatile(self.address, value);
        }
    }
}

impl<WIDTH> Register<WIDTH, ReadWrite> {
    pub fn read(&self) -> WIDTH {
        unsafe { ptr::read_volatile(self.address) }
    }

    pub fn write(&self, value: WIDTH) {
        unsafe {
            ptr::write_volatile(self.address, value);
        }
    }
}
