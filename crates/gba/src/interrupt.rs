// SPDX-License-Identifier: MPL-2.0
// SPDX-FileCopyrightText: 2021 Tim Crawford <crawfxrd@gmail.com>

use crate::register::{ReadWrite, Register};

const IRQ_HANDLER: Register<IrqHandler, ReadWrite, 0x0300_7FFC> = unsafe { Register::new() };
const DISPSTAT: Register<u16, ReadWrite, 0x0400_0004> = unsafe { Register::new() };
const IE: Register<u16, ReadWrite, 0x0400_0200> = unsafe { Register::new() };
const IF: Register<u16, ReadWrite, 0x0400_0202> = unsafe { Register::new() };
const IME: Register<u16, ReadWrite, 0x0400_0208> = unsafe { Register::new() };

pub type IrqHandler = unsafe extern "C" fn();

/// Signature for the default master ISR.
extern "C" {
    pub fn master_isr();
}

#[rustfmt::skip]
pub enum Irq {
    VBlank,
    //HBlank,
    //VCount,
    //Timer0,
    //Timer1,
    //Timer2,
    //Timer3,
    //Serial,
    //DMA1,
    //DMA2,
    //DMA3,
    //DMA4,
    //Keypad,
    //GamePak,
}

/// Sets the master ISR and enables interrupt handling.
///
/// * `isr`: Function to use as the master ISR
pub fn init(isr: IrqHandler) {
    IRQ_HANDLER.write(isr);
    IME.write(1);
}

/// Disables and clears all interrupts.
pub fn reset() {
    IME.write(0);
    IE.write(0);
    IF.write(0xFF);
}

/// Enables handling of the specified IRQ type.
pub fn enable(irq: Irq) {
    let ime = IME.read();
    IME.write(0);

    match irq {
        Irq::VBlank => {
            DISPSTAT.write(DISPSTAT.read() | (1 << 3));
            IE.write(1);
        }
    }

    IME.write(ime);
}
