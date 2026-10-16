#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/env.sh"
cd "$TIFF_ROOT"
python3 compat/generate_fields.py --check
compiler="${CC:-cc}"
output="${VTIFF_COMPAT_OUTPUT:-$TIFF_ROOT/target/compat}"
mkdir -p "$output"
cargo build --locked --release -p verified-tiff-capi
extra=()
if [ "${VTIFF_SANITIZE:-0}" = 1 ]; then extra=(-fsanitize=address,undefined,float-cast-overflow -fno-omit-frame-pointer); fi
"$compiler" -std=c11 -O2 -g -fPIC -Wall -Wextra -Werror "${extra[@]}" -Icompat -Icompat/include -c compat/tiff_compat.c -o "$output/tiff_compat.o"
python3 compat/generate_stubs.py "$output" "$compiler"
"$compiler" -std=c11 -O2 -g -fPIC -Wall -Wextra -Werror "${extra[@]}" -Icompat -Icompat/include -c "$output/stubs.c" -o "$output/stubs.o"
"$compiler" -shared "${extra[@]}" -Wl,-z,defs,-z,relro,-z,now -Wl,-soname,libtiff.so.6 -Wl,--version-script="$output/exports.map" \
 "$output/tiff_compat.o" "$output/stubs.o" target/release/libverified_tiff_capi.a -ldl -lpthread -lm -o "$output/libtiff.so.6"
ln -sfn libtiff.so.6 "$output/libtiff.so"
python3 compat/check_exports.py "$output"
echo "Compatibility library: $output/libtiff.so.6"
