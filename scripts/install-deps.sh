#!/usr/bin/env bash
# SPDX-License-Identifier: CC0-1.0
# SPDX-FileCopyrightText: NONE

# Install system dependencies for building the project.

# shellcheck shell=dash
# shellcheck disable=SC1091

set -Ee

. /etc/os-release
if [ "${ID}" = "arch" ]; then
    sudo pacman -S --noconfirm \
        arm-none-eabi-binutils \
        arm-none-eabi-gcc \
        curl
elif [ "${ID}" = "fedora" ]; then
    sudo dnf -y install \
        arm-none-eabi-gcc-cs \
        arm-none-eabi-newlib \
        curl
elif [ "${ID}" = "ubuntu" ]; then
    sudo apt -q update
    sudo apt -q install \
        --no-install-recommends --assume-yes \
        binutils-arm-none-eabi \
        curl \
        gcc-arm-none-eabi
else
    printf "\x1B[1;31munsupported host:\x1B[0m %s\n" "${ID}"
    exit 1
fi
