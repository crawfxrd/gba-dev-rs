#!/usr/bin/env bash
# SPDX-License-Identifier: CC0-1.0
# SPDX-FileCopyrightText: NONE

set -eE

source /etc/os-release

echo ">> Installing system dependencies"
if [[ "${ID}" = "fedora" ]] || [[ "${ID_LIKE}" = "fedora" ]]; then
    sudo dnf -y install \
        arm-none-eabi-gcc-cs \
        arm-none-eabi-newlib
elif [[ "${ID}" = "ubuntu" ]] || [[ "${ID_LIKE}" = "debian" ]]; then
    sudo apt install --no-install-recommends --assume-yes \
        binutils-arm-none-eabi \
        gcc-arm-none-eabi
fi

if which rustup &> /dev/null; then
    echo ">> Updating rustup"
    # XXX: Need to ensure rustup 1.24.0+
    rustup self update
else
    echo ">> Installing Rust"
    # XXX: Implicitly trust the script
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
        | sh -s -- -y --default-toolchain none

    echo ">> Loading Rust environment"
    source "${HOME}/.cargo/env"
fi

echo ">> Installing pinned Rust toolchain and components"
# XXX: `rustup update` tries to install everything?
rustup show
