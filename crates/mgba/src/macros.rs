// SPDX-License-Identifier: MPL-2.0
// SPDX-FileCopyrightText: 2021 Tim Crawford <crawfxrd@gmail.com>

/// Sends a log to mGBA at the specified level.
///
/// [`enable()`] must be called before mGBA will output logs.
///
/// For ease of use, the level-specific macros should be used instead.
///
/// [`enable()`]: ./fn.enable.html
#[doc(hidden)]
#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)*) => (
        $crate::log($level, format_args!($($arg)*))
    )
}

/// Sends a DEBUG log to mGBA.
///
/// [`enable()`] must be called before mGBA will output logs.
///
/// [`enable()`]: ./fn.enable.html
///
/// ### Examples
///
/// ```rust
/// # pub extern "C" fn main() -> ! {
/// mgba::enable();
/// let x = 42;
/// mgba::debug!("x: {}", x);
/// # }
/// ```
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => (
        $crate::log!($crate::Level::Debug, $($arg)*)
    )
}

/// Sends an INFO log to mGBA.
///
/// [`enable()`] must be called before mGBA will output logs.
///
/// [`enable()`]: ./fn.enable.html
///
/// ### Examples
///
/// ```rust
/// # pub extern "C" fn main() -> ! {
/// mgba::enable();
/// let index = 4;
/// mgba::info!("Moving to map {}", index);
/// # }
/// ```
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => (
        $crate::log!($crate::Level::Info, $($arg)*)
    )
}

/// Sends a WARN log to mGBA.
///
/// [`enable()`] must be called before mGBA will output logs.
///
/// [`enable()`]: ./fn.enable.html
///
/// ### Examples
///
/// ```rust
/// # pub extern "C" fn main() -> ! {
/// mgba::enable();
/// let (x, y) = (1400, 80);
/// mgba::warn!("({}, {}) is outside the map, resetting to default", x, y);
/// # }
/// ```
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => (
        $crate::log!($crate::Level::Warn, $($arg)*)
    )
}

/// Sends an ERROR  log to mGBA.
///
/// [`enable()`] must be called before mGBA will output logs.
///
/// [`enable()`]: ./fn.enable.html
///
/// ### Examples
///
/// ```rust
/// # pub extern "C" fn main() -> ! {
/// mgba::enable();
/// let (x, y) = (0x16, 0x06);
/// mgba::error!("Expected value {:#X}, found {:#X}", x, y);
/// # }
/// ```
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => (
        $crate::log!($crate::Level::Error, $($arg)*)
    )
}

/// Sends a FATAL log to mGBA.
///
/// In addition to logging the message, mGBA will open a dialog window with
/// the log message.
///
/// [`enable()`] must be called before mGBA will output logs.
///
/// [`enable()`]: ./fn.enable.html
///
/// ### Examples
///
/// ```rust
/// # pub extern "C" fn main() -> ! {
/// mgba::enable();
/// let errno = 3;
/// mgba::fatal!("Unrecoverable error: {}", errno);
/// # }
/// ```
#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => (
        $crate::log!($crate::Level::Fatal, $($arg)*)
    )
}
