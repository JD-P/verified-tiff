#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/env.sh"
cd "$TIFF_ROOT"
out="$TIFF_ROOT/target/webp-simd/scalar"
mkdir -p "$out"
cargo test --locked --release -p verified-tiff-simd --features force-scalar
cargo test --locked --features force-scalar --test vp8_primitives --test webp_simd
cargo build --locked --features force-scalar --example webp-native --example vp8-native
cp target/debug/examples/webp-native target/debug/examples/vp8-native "$out/"
# Keep matrix binaries separate from ordinary builds and change only each
# test module's explicit executable parameter. Expectations remain libwebp.
python3 - <<'PY'
import importlib.util,pathlib,unittest,sys
root=pathlib.Path('.').resolve()
sys.path.insert(0,str(root/'tests'))
for name in ['webp_native','webp_extended','vp8_native']:
 spec=importlib.util.spec_from_file_location(name,root/'tests'/(name+'.py'))
 module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
 module.EXE=root/'target/webp-simd/scalar'/('vp8-native' if name=='vp8_native' else 'webp-native')
 result=unittest.TextTestRunner(verbosity=2).run(unittest.defaultTestLoader.loadTestsFromModule(module))
 assert result.wasSuccessful(),name
PY
# Restore normal production artifacts after the forced-scalar build.
cargo build --locked --example webp-native --example vp8-native
bash scripts/build-compat.sh
