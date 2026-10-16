#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/env.sh"
cd "$TIFF_ROOT"
python3 scripts/webp-dependency.py
cargo test --locked
cargo build --locked --bin verified-tiff
python3 tests/integration.py
python3 tests/lzw_tail.py
python3 tests/lzw_legacy.py
python3 tests/alpha.py
python3 tests/extended.py
python3 tests/deflate.py
python3 tests/jpeg.py
python3 tests/fax.py
python3 tests/float64.py
python3 tests/webp.py
python3 tests/logluv.py
python3 tests/upstream.py

python3 scripts/zstd-dependency.py
cargo build --locked --release --example zstd-native
python3 tests/zstd.py

python3 scripts/lerc-dependency.py
cargo build --locked --example lerc-native --example webp-native --example vp8-native
python3 tests/lerc.py
python3 tests/webp_native.py
python3 tests/webp_extended.py
python3 tests/vp8_native.py
