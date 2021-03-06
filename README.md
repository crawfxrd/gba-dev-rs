# untitled

Embedded Rust development targeting the Nintendo Game Boy Advance.

## Dependencies

- [Rust] and Cargo
- An `arm-none-eabi` GCC toolchain (e.g., [devkitARM])
- [mGBA] for running the binary

A script is provided to install the dependencies for Fedora and Ubuntu.

```
./scripts/deps.sh
```

### Why GCC?

GCC is used for compiling the non-Rust code. Assembly must be used to write the
ROM header and the master ISR. Any addition ARM functions would also have to be
written in assembly or C, as mixing Thumb and ARM states in Rust is
[unusably inefficient][instruction_set].

GCC is also required for linking the object files into the final ELF binary.
`rust-lld` cannot be used for linking because it will emit a `blx` instruction
for interworking (Ref: [`arm-blx-v4t.s`]). This instruction is not available on
ARMv4T.

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

## Resources

- [AGB Programming Manual v1.1][manual]
- [GBATEK]
- [TONC]

## License

This software is made available under the terms of the Mozilla Public License,
version 2.0. See [LICENSE](./LICENSES/MPL-2.0.txt) for details.


[Rust]: https://www.rust-lang.org/
[cargo-config]: https://doc.rust-lang.org/cargo/reference/config.html
[devkitARM]: https://devkitpro.org/wiki/Getting_Started
[gdbinit]: https://sourceware.org/gdb/onlinedocs/gdb/gdbinit-man.html
[mGBA]: https://mgba.io/
[manual]: https://archive.org/details/NintendoGbaManualV1.1
[GBATEK]: https://problemkaputt.de/gbatek.htm
[TONC]: https://www.coranac.com/tonc/text/toc.htm
[instruction_set]: https://github.com/rust-lang/rust/issues/74727
[`arm-blx-v4t.s`]: https://github.com/rust-lang/llvm-project/blob/ea6bb2615f04d53db11b6a43a14be5c9d1eaebe1/lld/test/ELF/arm-blx-v4t.s
