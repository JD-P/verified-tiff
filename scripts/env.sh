#!/usr/bin/env bash
set -euo pipefail
TIFF_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
export CARGO_HOME="$TIFF_ROOT/.tools/cargo"
export RUSTUP_HOME="$TIFF_ROOT/.tools/rustup"
export PATH="$CARGO_HOME/bin:$PATH"
VERUS="$TIFF_ROOT/scripts/verus-core.py"
mkdir -p "$TIFF_ROOT/target"

