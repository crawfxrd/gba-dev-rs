#!/usr/bin/env bash
# SPDX-License-Identifier: CC0-1.0
# SPDX-FileCopyrightText: NONE

set -e

cargo build
rust-objcopy -O binary ./target/armv4t-none-eabi/debug/untitled ./target/debug.gba

cargo build --release
rust-objcopy -O binary ./target/armv4t-none-eabi/release/untitled ./target/release.gba
