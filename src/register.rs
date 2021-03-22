// SPDX-FileCopyrightText: 2021 Tim Crawford <crawfxrd@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use core::marker::PhantomData;

pub struct ReadOnly;
pub struct ReadWrite;
pub struct WriteOnly;

pub struct Register<WIDTH, MODE, const ADDRESS: u32> {
    width: PhantomData<WIDTH>,
    mode: PhantomData<MODE>,
}

impl<WIDTH, MODE, const ADDRESS: u32> Register<WIDTH, MODE, ADDRESS> {
    pub const unsafe fn new() -> Self {
        Self {
            width: PhantomData,
            mode: PhantomData,
        }
    }
}

impl<WIDTH, const ADDRESS: u32> Register<WIDTH, ReadOnly, ADDRESS> {
    pub fn read(&self) -> WIDTH {
        unsafe { (ADDRESS as *mut WIDTH).read_volatile() }
    }
}

impl<WIDTH, const ADDRESS: u32> Register<WIDTH, WriteOnly, ADDRESS> {
    pub fn write(&self, value: WIDTH) {
        unsafe {
            (ADDRESS as *mut WIDTH).write_volatile(value);
        }
    }
}

impl<WIDTH, const ADDRESS: u32> Register<WIDTH, ReadWrite, ADDRESS> {
    pub fn read(&self) -> WIDTH {
        unsafe { (ADDRESS as *mut WIDTH).read_volatile() }
    }

    pub fn write(&self, value: WIDTH) {
        unsafe {
            (ADDRESS as *mut WIDTH).write_volatile(value);
        }
    }
}
