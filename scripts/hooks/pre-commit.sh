#!/usr/bin/env bash
# SPDX-License-Identifier: CC0-1.0
# SPDX-FileCopyrightText: NONE

passed() {
    echo -e "\x1B[32mPASSED\x1B[0m"
}

skipped() {
    local reason=$1
    echo -e "\x1B[33mSKIPPED\x1B[0m ($reason)"
}

failed() {
    echo -e "\x1B[31mFAILED\x1B[0m"
}

echo -n "Checking for formatting issues..."
if command -v cargo > /dev/null; then
    needs_fmt=$(cargo fmt -- --check -l)
    if [[ "$?" = "0" ]]; then
        passed
    else
        failed
        for file in $needs_fmt; do
            echo "- $file"
        done
        echo
        exit 1
    fi
else
    skipped "cargo not found"
fi
