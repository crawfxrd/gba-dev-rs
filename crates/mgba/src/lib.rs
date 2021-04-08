// SPDX-License-Identifier: MPL-2.0
// SPDX-FileCopyrightText: 2021 Tim Crawford <crawfxrd@gmail.com>

//! # mGBA logger
//!
//! Provides a mechanism for GBA games to send logs to [mGBA]. This is done by
//! writing to mGBA-specific [MMIO addresses][mmio], which causes the emulator
//! to output messages to the host machine.
//!
//! Be aware than the static strings will end up in the resulting GBA binary,
//! taking up valuable ROM space.
//!
//! ### Usage
//!
//! Call [`enable()`] at the start of your binary to enable logging in mGBA.
//! Then use the macros to log at whatever level you need.
//!
//! ```edition2018
//! pub extern "C" main() -> ! {
//!     mgba::enable();
//!     mgba::info!("Logging to mGBA from a GBA ROM");
//! }
//! ```
//!
//! [mGBA]: https://mgba.io/
//! [mmio]: https://github.com/mgba-emu/mgba/blob/0.8.4/include/mgba/internal/gba/io.h#L157-L159
//! [`enable()`]: ./fn.enable.html

#![no_std]

use core::fmt;

mod macros;

/// The memory-mapped address for setting the log level and sending the log.
const MGBA_DEBUG_FLAGS: *mut u16 = 0x04FF_F700 as *mut u16;

/// The memory-mapped address for enabling mGBA logging.
///
/// To enable logging, write the value 0xC0DE.
/// To check if logging is enabled, read and check for the value 0x1DEA.
const MGBA_DEBUG_ENABLE: *mut u16 = 0x04FF_F780 as *mut u16;

/// The memory-mapped address where logs are written.
const MGBA_DEBUG_STRING: *mut u8 = 0x04FF_F600 as *mut u8;

/// The maximum number of bytes a single log can be. Hard-coded in mGBA.
const MGBA_DEBUG_STRING_LEN: usize = 0x100;

/// Value to trigger mGBA to read and print the debug string.
const MGBA_DEBUG_SEND: u16 = 1 << 8;

/// mGBA-defined log levels.
pub enum Level {
    /// This will cause mGBA to display a dialog window with log message.
    Fatal = 0,
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
}

/// Enables logging in mGBA.
///
/// Returns true if mGBA logging is enabled.
///
/// ### Examples
///
/// ```edition2018
/// # pub extern "C" fn main() -> ! {
/// if mgba::enable() {
///     mgba::info!("mGBA logging enabled");
/// }
/// # }
/// ```
pub fn enable() -> bool {
    unsafe {
        MGBA_DEBUG_ENABLE.write_volatile(0xC0DE);
    }
    enabled()
}

/// Checks if mGBA logging is enabled.
fn enabled() -> bool {
    unsafe { MGBA_DEBUG_ENABLE.read_volatile() == 0x1DEA }
}

/// Sends the log to mGBA to output on the host.
///
/// This function should not be called directly. Use the macros instead.
#[doc(hidden)]
pub fn log(level: Level, args: fmt::Arguments) {
    let mut b = Buffer::new();
    let _ = fmt::write(&mut b, args);
    flush(level);
}

fn flush(level: Level) {
    unsafe {
        MGBA_DEBUG_FLAGS.write_volatile(MGBA_DEBUG_SEND | level as u16);
    }
}

struct Buffer {
    offset: usize,
}

impl Buffer {
    fn new() -> Self {
        Self { offset: 0 }
    }
}

impl fmt::Write for Buffer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for &b in s.as_bytes() {
            if self.offset >= MGBA_DEBUG_STRING_LEN {
                return Err(fmt::Error);
            }

            unsafe {
                MGBA_DEBUG_STRING.add(self.offset).write_volatile(b);
            }

            self.offset += 1;
        }

        Ok(())
    }
}
