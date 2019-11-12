#![feature(core_intrinsics)]

#![no_std]
#![no_main]
#![deny(warnings)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { core::intrinsics::abort() }
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    let _x = 42;

    unsafe {
        (0x400_0000 as *mut u16).write_volatile(0x0403);
        (0x600_0000 as *mut u16).offset(104 + 80 * 240).write_volatile(0x001F);
        (0x600_0000 as *mut u16).offset(120 + 80 * 240).write_volatile(0x03E0);
        (0x600_0000 as *mut u16).offset(136 + 80 * 240).write_volatile(0x7C00);
    }
    loop {}
}
