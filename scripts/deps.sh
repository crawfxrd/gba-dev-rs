#!/usr/bin/env bash
# SPDX-License-Identifier: CC0-1.0
# SPDX-FileCopyrightText: NONE

set -eE

# Print bolded message to stdout
msg() {
  echo -e "\x1B[1m$*\x1B[0m"
}

source /etc/os-release

msg ">> Installing system dependencies"
if [[ "${ID}" = "fedora" ]] || [[ "${ID_LIKE}" =~ "fedora" ]]; then
    sudo dnf -y install \
        arm-none-eabi-gcc-cs \
        arm-none-eabi-newlib
elif [[ "${ID}" = "ubuntu" ]] || [[ "${ID_LIKE}" =~ "debian" ]]; then
    sudo apt install --no-install-recommends --assume-yes \
        binutils-arm-none-eabi \
        gcc-arm-none-eabi
elif [[ "${ID}" = "arch" ]] || [[ "${ID_LIKE}" =~ "arch" ]]; then
    sudo pacman -S --noconfirm \
        arm-none-eabi-binutils \
        arm-none-eabi-gcc
fi

if which rustup &> /dev/null; then
    msg ">> Updating rustup"
    rustup self update
else
    msg ">> Installing Rust"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
        | sh -s -- -y --default-toolchain none

    msg ">> Loading Rust environment"
    source "${HOME}/.cargo/env"
fi

msg ">> Installing pinned Rust toolchain and components"
rustup show
