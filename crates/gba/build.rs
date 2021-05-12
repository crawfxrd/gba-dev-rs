// SPDX-FileCopyrightText: NONE
// SPDX-License-Identifier: CC0-1.0

use std::env;

fn main() {
    let cwd = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-arg=-T{}/src/linker.ld", cwd);
    let profile = env::var("PROFILE").unwrap();
    println!("cargo:rustc-link-arg-bins=-Map=target/{}.map", profile);

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
