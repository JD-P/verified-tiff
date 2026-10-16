#!/usr/bin/env bash
set -euo pipefail
source scripts/env.sh
cd "$TIFF_ROOT"
out="$TIFF_ROOT/target/reader-benchmark"
ref="$out/tiff-4.7.1"
cc -std=c11 -O3 -g -Wall -Wextra -Werror -DVERIFIED_ADAPTER -Icompat/include benchmarks/reader.c -Ltarget/compat -Wl,-rpath,"$PWD/target/compat" -ltiff -o "$out/verified"
for pair in reader:reference reader-encode:encode; do
 src=${pair%:*}
 bin=${pair#*:}
 "$ref/libtool" --mode=link --tag=CC cc -std=c11 -O3 -g -Wall -Wextra -Werror -I"$ref/libtiff" "benchmarks/$src.c" "$ref/libtiff/libtiff.la" -Wl,-rpath,"$PWD/target/lerc-reference" -Wl,-rpath,"$PWD/target/zstd-reference/lib" -o "$out/$bin"
done
