#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/env.sh"
cd "$TIFF_ROOT"
cargo run --locked --release --bin fuzz-smoke -- "${1:-20000}" | tee target/fuzz-smoke.log
