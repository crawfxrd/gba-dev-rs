// SPDX-License-Identifier: CC0-1.0

fn main() {
    cc::Build::new()
        .compiler("arm-none-eabi-gcc")
        .archiver("arm-none-eabi-ar")
        .no_default_flags(true)
        .warnings_into_errors(true)
        .flag("-mcpu=arm7tdmi")
        .out_dir("target")
        .file("src/entry.S")
        .compile("entry.o");

    cc::Build::new()
        .compiler("arm-none-eabi-gcc")
        .archiver("arm-none-eabi-ar")
        .no_default_flags(true)
        .warnings_into_errors(true)
        .flag("-mcpu=arm7tdmi")
        .out_dir("target")
        .file("src/interrupt.S")
        .compile("interrupt.o");
}
