#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/env.sh"
cd "$TIFF_ROOT"
output="${VTIFF_COMPAT_OUTPUT:-$TIFF_ROOT/target/compat}"
compiler="${CC:-cc}"
extra=()
if [ "${VTIFF_SANITIZE:-0}" = 1 ]; then extra=(-fsanitize=address,undefined,float-cast-overflow -fno-omit-frame-pointer -no-pie); fi
wrap=()
for name in vt_error_code vt_error_message vt_next vt_free_directory vt_write_check vt_rgba_ok vt_select vt_set_offset vt_mem_claim vt_unset vt_clientdata vt_info vt_count_dirs vt_size vt_scalar vt_array vt_field vt_typed vt_real vt_pair vt_strile vt_read_size vt_read vt_region vt_commit vt_put vt_put_real vt_mode vt_custom_dir vt_directory_bytes vt_read_buffer; do wrap+=("-Wl,--wrap=$name"); done
"$compiler" -std=c11 -O2 -g -Wall -Wextra -Werror "${extra[@]}" -Icompat -Icompat/include compat/tests/shim_hardening.c target/release/libverified_tiff_capi.a "${wrap[@]}" -ldl -lpthread -lm -o "$output/shim-hardening"
chmod u+x "$output/shim-hardening"
python3 - "$output/hardening-planar.tif" <<'PYFIXTURE'
import pathlib,sys
sys.path.insert(0,'tests')
from extended import make_extended
pathlib.Path(sys.argv[1]).write_bytes(make_extended(w=17,h=19,ch=3,bits=16,separate=True,tiled=True)[0])
PYFIXTURE
"$output/shim-hardening" tests/fixtures/gray-strip.tif tests/fixtures/rgb-be-tile.tif "$output/hardening-planar.tif"
