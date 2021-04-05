# untitled

Embedded Rust development targeting the Nintendo Game Boy Advance.

## Dependencies

- [Rust] and Cargo
- An `arm-none-eabi` GCC toolchain (e.g., [devkitARM])
- [mGBA] for running the binary

```
sudo dnf -y install arm-none-eabi-gcc-cs arm-none-eabi-newlib
```

### Why GCC?

GCC is used for compiling the non-Rust code. Assembly must be used to write the
ROM header and the master ISR. Any addition ARM functions would also have to be
written in assembly or C, as Rust cannot mix Thumb and ARM states.

GCC is also required for linking the object files into the final ELF binary.
`rust-lld` cannot be used for linking because it will emit a `blx` instruction
for interworking. This instruction is not available in ARMv4T architecture.

## Building

Cargo is [configured][cargo-config] to run with the ARM7TDMI target
`thumbv4t-none-eabi`.

To make it a GBA ROM file, build in release mode and convert the target from
ELF to binary.

```
cargo build --release
arm-none-eabi-objcopy -O binary target/thumbv4t-none-eabi/release/untitled target/untitled.gba
```

## Running

Cargo is configured to run the target with mGBA, and assumes the binary is
called `mgba-qt`. If the mGBA binary uses a different name (e.g., `mgba`,
`mgba-sdl`), modify the `runner` value in `.cargo/config`.

```
cargo run --release -- -3
```

### Debugging

Run a debug build with a GDB session. mGBA will wait for a connection on the
localhost port 2345.

```
cargo run -- -g
```

In another window, attach to the session. The file [`.gdbinit`][gdbinit]
specifies the binary to load debug symbols from and the remote target. GDB must
be configured to allow loading this file (`local-gdbinit`, `safe-path`).

```
gdb -q -iex "set auto-load safe-path ."
0x00000000 in ?? ()
(gdb) c
Continuing.
```

## License

This software is made available under the terms of the Mozilla Public License,
version 2.0. See [LICENSE](./LICENSES/MPL-2.0.txt) for details.

[Rust]: https://www.rust-lang.org/
[cargo-config]: https://doc.rust-lang.org/cargo/reference/config.html
[devkitARM]: https://devkitpro.org/wiki/Getting_Started
[gdbinit]: https://sourceware.org/gdb/onlinedocs/gdb/gdbinit-man.html
[mGBA]: https://mgba.io/
