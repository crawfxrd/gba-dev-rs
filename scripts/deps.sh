#!/usr/bin/env bash
# SPDX-License-Identifier: CC0-1.0
# SPDX-FileCopyrightText: NONE

# Install dependencies for building the project.

set -eE

# Print bolded message to stdout
msg() {
  echo -e "\x1B[1m$*\x1B[0m"
}

msg ">> Installing system dependencies"
source /etc/os-release
if [[ "${ID}" = "arch" ]] || [[ "${ID_LIKE}" =~ "arch" ]]; then
    sudo pacman -S --noconfirm \
        arm-none-eabi-binutils \
        arm-none-eabi-gcc \
        curl
elif [[ "${ID}" = "fedora" ]] || [[ "${ID_LIKE}" =~ "fedora" ]]; then
    sudo dnf -y install \
        arm-none-eabi-gcc-cs \
        arm-none-eabi-newlib \
        curl
elif [[ "${ID}" = "ubuntu" ]] || [[ "${ID_LIKE}" =~ "debian" ]]; then
    sudo apt install --no-install-recommends --assume-yes \
        binutils-arm-none-eabi \
        curl \
        gcc-arm-none-eabi
else
    msg "\x1B[31m>> Unknown host:\x1B[0m ${ID}"
    exit 1
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
# rustup has no command to specifically install a toolchain from a TOML file.
# Rely on the fact that `show` will install the default toolchain.
rustup show

msg "\x1B[32m>> Successfully installed dependencies\n"
