// SPDX-FileCopyrightText: NONE
// SPDX-License-Identifier: CC0-1.0

use std::env;

fn main() {
    let cwd = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-arg=-T{}/src/linker.ld", cwd);
    let profile = env::var("PROFILE").unwrap();
    println!("cargo:rustc-link-arg-bins=-Map=target/{}.map", profile);
}
