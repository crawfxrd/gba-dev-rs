// SPDX-FileCopyrightText: NONE
// SPDX-License-Identifier: CC0-1.0

use std::env;

fn main() {
    let out = env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search={}", out);
    println!("cargo:rustc-link-arg=-Tgba.ld");

    let target = env::var("TARGET").unwrap();
    let profile = env::var("PROFILE").unwrap();
    let name = env::var("CARGO_PKG_NAME").unwrap();
    println!("cargo:rustc-link-arg-bins=-Map=target/{}/{}/{}.map", target, profile, name);
}
