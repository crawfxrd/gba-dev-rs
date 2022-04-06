#!/usr/bin/env bash
# SPDX-License-Identifier: CC0-1.0
# SPDX-FileCopyrightText: NONE

set -e

NAME="untitled"
TARGET="thumbv4t-none-eabi"
DEBUG_DIR="target/${TARGET}/debug"
RELEASE_DIR="target/${TARGET}/release"

cargo build
arm-none-eabi-objcopy -O binary ${DEBUG_DIR}/${NAME} ${DEBUG_DIR}/${NAME}.gba

cargo build --release
arm-none-eabi-objcopy -O binary ${RELEASE_DIR}/${NAME} ${RELEASE_DIR}/${NAME}.gba
