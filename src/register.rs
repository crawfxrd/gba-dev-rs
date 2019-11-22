// SPDX-License-Identifier: MPL-2.0

/*
 * Copyright (c) 2019 Tim Crawford <crawfxrd@gmail.com>
 */

use core::marker::PhantomData;
use core::ptr;

pub struct ReadOnly;
pub struct ReadWrite;
pub struct WriteOnly;

pub struct Register<T, MODE> {
    address: *mut T,
    mode: PhantomData<MODE>,
}

impl<T, MODE> Register<T, MODE> {
    pub const fn new(address: u32) -> Self {
        Self {
            address: address as *mut T,
            mode: PhantomData,
        }
    }
}

impl<T> Register<T, ReadOnly> {
    pub fn read(&self) -> T {
        unsafe { ptr::read_volatile(self.address) }
    }
}

impl<T> Register<T, WriteOnly> {
    pub fn write(&self, value: T) {
        unsafe {
            ptr::write_volatile(self.address, value);
        }
    }
}

impl<T> Register<T, ReadWrite> {
    pub fn read(&self) -> T {
        unsafe { ptr::read_volatile(self.address) }
    }

    pub fn write(&self, value: T) {
        unsafe {
            ptr::write_volatile(self.address, value);
        }
    }
}
