#!/usr/bin/env bash
# SPDX-License-Identifier: CC0-1.0
# SPDX-FileCopyrightText: NONE

set -e

cargo build
arm-none-eabi-objcopy -O binary ./target/thumbv4t-none-eabi/debug/untitled ./target/debug.gba

cargo build --release
arm-none-eabi-objcopy -O binary ./target/thumbv4t-none-eabi/release/untitled ./target/release.gba
