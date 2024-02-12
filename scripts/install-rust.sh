#!/bin/sh
# SPDX-License-Identifier: CC0-1.0
# SPDX-FileCopyrightText: NONE

# Install Rust via rustup, along with the pinned toolchain and required
# programs via cargo.

# shellcheck shell=dash
# shellcheck disable=SC1091

set -Ee

if ! command -v rustup >/dev/null 2>&1; then
    curl --proto '=https' --tlsv1.3 -sSf https://sh.rustup.rs \
        | sh -s -- -y --profile minimal --default-toolchain none
    . "${HOME}/.cargo/env"
fi

# rustup has no command to specifically install a toolchain from a TOML file.
# Rely on the fact that `show` will install the default toolchain.
rustup show
