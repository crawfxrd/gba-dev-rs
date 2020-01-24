// SPDX-FileCopyrightText: NONE
// SPDX-License-Identifier: CC0-1.0

use std::env;

fn main() {
    let cwd = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-arg-bins=-T{}/src/linker.ld", cwd);
    let profile = env::var("PROFILE").unwrap();
    println!("cargo:rustc-link-arg-bins=-Map=target/{}.map", profile);

    cc::Build::new()
        .compiler("clang")
        .no_default_flags(true)
        .warnings_into_errors(true)
        .flag("--target=arm-none-eabi")
        .file("src/entry.S")
        .compile("entry");

    cc::Build::new()
        .compiler("clang")
        .no_default_flags(true)
        .warnings_into_errors(true)
        .flag("--target=arm-none-eabi")
        .file("src/interrupt.S")
        .compile("interrupt");
}
