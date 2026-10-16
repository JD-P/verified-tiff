#!/usr/bin/env bash
# GCC coverage for production C, combining real-backend integration with the
# boundary unit harness. All emitted branches count; no exclusion annotations.
set -euo pipefail
source "$(dirname "$0")/env.sh"
cd "$TIFF_ROOT"
output="${VTIFF_COVERAGE_OUTPUT:-$TIFF_ROOT/target/c-coverage}"
mkdir -p "$output"
# Remove only counters in this dedicated output directory; retain other files.
python3 - "$output" <<'PY'
import pathlib,sys
p=pathlib.Path(sys.argv[1]).resolve()
for f in p.glob('*.gcda'):f.unlink()
PY
cat > "$output/cc-coverage" <<'COMPILER'
#!/usr/bin/env bash
exec gcc "$@" -O0 --coverage -fprofile-abs-path -fprofile-update=atomic
COMPILER
chmod u+x "$output/cc-coverage"
CC="$output/cc-coverage" VTIFF_COMPAT_OUTPUT="$output" bash scripts/test-compat.sh > "$output/suite.log" 2>&1
python3 scripts/report-c-coverage.py "$output" --require-complete
