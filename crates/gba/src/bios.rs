// SPDX-License-Identifier: MPL-2.0
// SPDX-FileCopyrightText: 2021 Tim Crawford <crawfxrd@gmail.com>

/// Reset the device.
#[inline]
pub unsafe fn reset() {
    asm!("svc 0x00");
}

/// Reset the specified MMIO and RAM regions.
#[inline]
pub unsafe fn reset_ram(flags: usize) {
    asm!("svc 0x01",
        in("r0") flags
    );
}

/// Stop CPU execution until any enabled interrupt occurs.
#[inline]
pub fn halt() {
    unsafe {
        asm!("svc 0x02");
    }
}

/// Put the GBA into a very low power state.
#[inline]
pub fn stop() {
    unsafe {
        asm!("svc 0x03");
    }
}

/// Stop CPU execution until any specified interrupts to occur.
#[inline]
pub fn wait(clear: bool, interrupt_flags: usize, extra_flags: usize) {
    unsafe {
        asm!("svc 0x04",
            in("r0") clear as usize,
            in("r1") interrupt_flags,
            in("r2") extra_flags
        );
    }
}

/// Stop CPU execution until the vertical blanking interval.
#[inline]
pub fn vblank() {
    unsafe {
        asm!("svc 0x05",
            // Clobbers
            out("r0") _, out("r1") _
        );
    }
}
