#!/usr/bin/env bash

set -e

IMG=cyberpunk

grit ./${IMG}.png -gb -gu16 -ftc
rm "${IMG}.h"
mv "${IMG}.c" "${IMG}.rs"
sed -i '/const unsigned short/c\pub const DATA: &[u16] =' "${IMG}.rs"
sed -i 's/^{/\&[/' "${IMG}.rs"
sed -i 's/^}/]/' "${IMG}.rs"
sed -i '/^\/\//d' "${IMG}.rs"
sed -i '/^$/d' "${IMG}.rs"
rustfmt "${IMG}.rs"
