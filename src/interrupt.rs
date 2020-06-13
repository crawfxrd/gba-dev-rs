// SPDX-License-Identifier: MPL-2.0

/*
 * Copyright 2019 Tim Crawford <crawfxrd@gmail.com>
 */

use crate::register::{ReadWrite, Register};

const IRQ_HANDLER: Register<IrqHandler, ReadWrite> = Register::new(0x0300_7FFC);
const DISPSTAT: Register<u16, ReadWrite> = Register::new(0x0400_0004);
const IE: Register<u16, ReadWrite> = Register::new(0x0400_0200);
const IME: Register<u16, ReadWrite> = Register::new(0x0400_0208);

pub type IrqHandler = unsafe extern "C" fn();

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

pub fn init(isr: IrqHandler) {
    IRQ_HANDLER.write(isr);
    IME.write(1);
}

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
