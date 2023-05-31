# Development documentation

## Environment

Linux is the only supported environment. Development is possible on a variety
of distros.

Any editor that has support for [EditorConfig] may be used.

## Dependencies

A script is provided to install dependencies.

```
./scripts/deps.sh
```

Additionally, mGBA is required for the `run` target.

### Rust toolchain

A [nightly channel][channels] is used to build the project. This is required to
use **unstable** features, such as the [`build-std`] Cargo feature, Rust
language features, and rustfmt features

### GCC

An `arm-none-eabi` GCC toolchain is required.

GCC is used for compiling the non-Rust code. Assembly must be used to write the
ROM header and the master ISR. Any addition ARM functions would also have to be
written in assembly or C, as mixing Thumb and ARM states in Rust is
[unusably inefficient][instruction_set].

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


[EditorConfig]: https://editorconfig.org/
[Rust]: https://www.rust-lang.org/
[cargo-config]: https://doc.rust-lang.org/cargo/reference/config.html
[channels]: https://rust-lang.github.io/rustup/concepts/channels.html
[devkitARM]: https://devkitpro.org/wiki/Getting_Started
[gdbinit]: https://sourceware.org/gdb/onlinedocs/gdb/gdbinit-man.html
[instruction_set]: https://github.com/rust-lang/rust/issues/74727
[mGBA]: https://mgba.io/
[`build-std`]: https://doc.rust-lang.org/cargo/reference/unstable.html#build-std
