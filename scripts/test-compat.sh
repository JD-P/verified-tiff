#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/env.sh"
cd "$TIFF_ROOT"
output="${VTIFF_COMPAT_OUTPUT:-$TIFF_ROOT/target/compat}"
compiler="${CC:-cc}"
bash scripts/build-compat.sh
# coverage.py compares the adapter with the original Rust decoder as well.
# Build that executable here so a clean compatibility-only run is reproducible.
cargo build --locked --bin verified-tiff
extra=()
if [ "${VTIFF_SANITIZE:-0}" = 1 ]; then extra=(-fsanitize=address,undefined,float-cast-overflow -fno-omit-frame-pointer -no-pie); fi
"$compiler" -std=c11 -O2 -g -Wall -Wextra -Werror "${extra[@]}" -Icompat/include compat/tests/api.c -L"$output" -Wl,-rpath,"$output" -ltiff -lpthread -o "$output/api-test"
"$compiler" -std=c11 -O2 -g -Wall -Wextra -Werror "${extra[@]}" -Icompat/include "$output/stub_smoke.c" -L"$output" -Wl,-rpath,"$output" -ltiff -o "$output/stub-test"
# Compile the consumer against the real pinned public headers, not our copy.
"$compiler" -std=c11 -O2 -g "${extra[@]}" -I.tools/tiff-4.7.1/libtiff compat/tests/dump.c -L"$output" -Wl,-rpath,"$output" -ltiff -o "$output/dump"
.tools/tiff-4.7.1/libtool --mode=link --tag=CC "$compiler" -std=c11 -O2 -I.tools/tiff-4.7.1/libtiff compat/tests/dump.c .tools/tiff-4.7.1/libtiff/libtiff.la -o "$output/dump-libtiff"
# An existing binary with LIBTIFF_* version requirements, initially linked to
# the system shared library; only its loader path changes during the test.
"$compiler" -std=c11 -O2 -g "${extra[@]}" -I.tools/tiff-4.7.1/libtiff compat/tests/dump.c -Wl,-l:libtiff.so.6 -o "$output/prelinked-dump"
"$compiler" -std=c11 -O2 -g -Wall -Wextra -Werror "${extra[@]}" -Icompat/include compat/tests/mutate.c -L"$output" -Wl,-rpath,"$output" -ltiff -o "$output/mutate-test"
"$compiler" -std=c11 -O2 -g -Wall -Wextra -Werror "${extra[@]}" -Icompat/include compat/tests/coverage.c -L"$output" -Wl,-rpath,"$output" -ltiff -o "$output/coverage-test"
"$compiler" -std=c11 -O2 -g -Wall -Wextra -Werror "${extra[@]}" -Icompat/include compat/tests/consumers.c -L"$output" -Wl,-rpath,"$output" -ltiff -lm -o "$output/consumer-test"
# Keep the independent oracle free of adapter-only extensions.
.tools/tiff-4.7.1/libtool --mode=link --tag=CC "$compiler" -std=c11 -O2 -I.tools/tiff-4.7.1/libtiff compat/tests/logluv_oracle.c .tools/tiff-4.7.1/libtiff/libtiff.la -lm -o "$output/consumer-libtiff"
"$compiler" -std=c11 -O2 -g -Wall -Wextra -Werror "${extra[@]}" -Icompat/include compat/tests/reader_features.c -L"$output" -Wl,-rpath,"$output" -ltiff -lm -o "$output/reader-features"
.tools/tiff-4.7.1/libtool --mode=link --tag=CC "$compiler" -std=c11 -O2 -DORACLE -I.tools/tiff-4.7.1/libtiff compat/tests/reader_features.c .tools/tiff-4.7.1/libtiff/libtiff.la -lm -o "$output/reader-features-libtiff"
# Ensure rebuilt consumers remain executable on materialized workspaces.
chmod u+x "$output"/{api-test,stub-test,dump,dump-libtiff,prelinked-dump,mutate-test,coverage-test,consumer-test,consumer-libtiff,reader-features,reader-features-libtiff}
python3 compat/tests/reader_features.py
python3 compat/tests/consumers.py
python3 compat/tests/test.py
python3 compat/tests/coverage.py

python3 scripts/zstd-tiff-dependency.py
zstd_tiff="$TIFF_ROOT/.tools/zstd-tiff-source/tiff-4.7.1"
"$zstd_tiff/libtool" --mode=link --tag=CC "$compiler" -std=c11 -O2 -I"$zstd_tiff/libtiff" compat/tests/dump.c "$zstd_tiff/libtiff/libtiff.la" -Wl,-rpath,"$TIFF_ROOT/target/zstd-reference/lib" -o "$output/dump-libtiff-zstd"
chmod u+x "$output/dump-libtiff-zstd"
python3 compat/tests/zstd_compat.py

python3 scripts/webp-dependency.py
python3 scripts/lerc-tiff-dependency.py
lerc_tiff="$TIFF_ROOT/.tools/lerc-tiff-source/tiff-4.7.1"
"$lerc_tiff/libtool" --mode=link --tag=CC "$compiler" -std=c11 -O2 -I"$lerc_tiff/libtiff" compat/tests/dump.c "$lerc_tiff/libtiff/libtiff.la" -Wl,-rpath,"$TIFF_ROOT/target/zstd-reference/lib" -Wl,-rpath,"$TIFF_ROOT/target/lerc-reference" -o "$output/dump-libtiff-lerc"
chmod u+x "$output/dump-libtiff-lerc"
python3 compat/tests/lerc_compat.py
python3 compat/tests/webp_compat.py
python3 compat/tests/codec_malformed.py

"$compiler" -std=c11 -O2 -g -Wall -Wextra -Werror "${extra[@]}" -Icompat/include compat/tests/adapter_proofs.c -L"$output" -Wl,-rpath,"$output" -ltiff -lm -o "$output/adapter-proofs"
python3 compat/tests/adapter_proofs.py

"$compiler" -std=c11 -O2 -g -Wall -Wextra -Werror "${extra[@]}" -I.tools/tiff-4.7.1/libtiff compat/tests/metadata_contracts.c -L"$output" -Wl,-rpath,"$output" -ltiff -o "$output/metadata-contracts"
"$lerc_tiff/libtool" --mode=link --tag=CC "$compiler" -std=c11 -O2 -DORACLE -I"$lerc_tiff/libtiff" compat/tests/metadata_contracts.c "$lerc_tiff/libtiff/libtiff.la" -Wl,-rpath,"$TIFF_ROOT/target/zstd-reference/lib" -Wl,-rpath,"$TIFF_ROOT/target/lerc-reference" -o "$output/metadata-contracts-libtiff"
python3 compat/tests/metadata_contracts.py

bash scripts/test-shim-hardening.sh
