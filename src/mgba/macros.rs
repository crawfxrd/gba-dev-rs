// SPDX-License-Identifier: MPL-2.0
// SPDX-FileCopyrightText: 2021 Tim Crawford <crawfxrd@gmail.com>

#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)*) => (
        $crate::mgba::log($level, format_args!($($arg)*))
    )
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => (
        $crate::log!($crate::mgba::Level::Debug, $($arg)*)
    )
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => (
        $crate::log!($crate::mgba::Level::Info, $($arg)*)
    )
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => (
        $crate::log!($crate::mgba::Level::Warn, $($arg)*)
    )
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => (
        $crate::log!($crate::mgba::Level::Error, $($arg)*)
    )
}

#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => (
        $crate::log!($crate::mgba::Level::Fatal, $($arg)*)
    )
}
