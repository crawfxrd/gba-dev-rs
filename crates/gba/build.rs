// SPDX-FileCopyrightText: NONE
// SPDX-License-Identifier: CC0-1.0

use std::{env, fs};

fn main() {
    let out = env::var("OUT_DIR").unwrap();
    fs::copy("gba.ld", format!("{out}/gba.ld")).unwrap();

    println!("cargo:rustc-link-search={out}");
    println!("cargo:rustc-link-arg-examples=-Tgba.ld");

    cc::Build::new()
        .compiler("arm-none-eabi-gcc")
        .no_default_flags(true)
        .warnings_into_errors(true)
        .flag("-mcpu=arm7tdmi")
        .file("src/entry.S")
        .compile("entry");

    cc::Build::new()
        .compiler("arm-none-eabi-gcc")
        .no_default_flags(true)
        .warnings_into_errors(true)
        .flag("-mcpu=arm7tdmi")
        .file("src/interrupt.S")
        .compile("interrupt");
}
