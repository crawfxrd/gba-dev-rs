# untitled

Embedded Rust development targeting the Nintendo Game Boy Advance.

## Dependencies

- [Rust] and Cargo
- [cargo-xbuild]
- An `arm-none-eabi` GCC toolchain (e.g., [devkitARM])
- [mGBA] for running the binary

## Building

Cargo is [configured][cargo-config] to run with the provided ARM7TDMI target
file `armv4t-none-eabi.json`.

```
cargo xbuild
```

To make it a GBA ROM file, build in release mode and convert the target from
ELF to binary.

```
cargo xbuild --release
arm-none-eabi-objcopy -O binary target/armv4t-none-eabi/release/untitled target/untitled.gba
```

## Running

Cargo is configured to run the target with mGBA, and assumes the binary is
called `mgba`. If the mGBA binary uses a different name (e.g., `mgba-sdl`,
`mgba-qt`), modify the `runner` value in `.cargo/config`.

```
cargo xrun -- -3
```

## License

This software is made available under the terms of the Mozilla Public License,
version 2.0. See [LICENSE](./Licenses/MPL-2.0.txt) for details.

[Rust]: https://www.rust-lang.org/
[cargo-config]: https://doc.rust-lang.org/cargo/reference/config.html
[cargo-xbuild]: https://github.com/rust-osdev/cargo-xbuild
[devkitARM]: https://devkitpro.org/wiki/Getting_Started
[mGBA]: https://mgba.io/
