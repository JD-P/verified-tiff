#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/env.sh"
cd "$TIFF_ROOT"
mkdir -p target/benchmark
cargo build --locked --release --example benchmark
bash scripts/build-compat.sh
cc -O3 -std=c11 -Wall -Wextra -Werror -Icompat/include -DVERIFIED_ADAPTER benchmarks/libtiff.c -Ltarget/compat -Wl,-rpath,"$PWD/target/compat" -ltiff -o target/benchmark/adapter
for source in libtiff encode; do
 .tools/tiff-4.7.1/libtool --mode=link --tag=CC cc -O3 -std=c11 -Wall -Wextra -Werror -I.tools/tiff-4.7.1/libtiff "benchmarks/$source.c" .tools/tiff-4.7.1/libtiff/libtiff.la -o "target/benchmark/$source"
done
cc -O3 -std=c11 -Wall -Wextra -Werror -I.tools/tiff-4.7.1/libtiff benchmarks/libtiff.c -Wl,-l:libtiff.so.6 -o target/benchmark/system
