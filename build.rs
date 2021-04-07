// SPDX-FileCopyrightText: NONE
// SPDX-License-Identifier: CC0-1.0

fn main() {
    println!("cargo:rustc-link-arg=-Tsrc/linker.ld");
    println!("cargo:rustc-link-arg=-Map=target/output.map");

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
