#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/env.sh"
cd "$TIFF_ROOT"
test -x "$VERUS"
"$VERUS" --version | tee target/verus-version.txt
python3 scripts/audit.py
"$VERUS" --crate-type lib --triggers-mode silent --rlimit 100 --output-json src/lib.rs > target/verification.json
python3 scripts/check-verification.py target/verification.json
if "$VERUS" --crate-type lib tests/proof_failure/false_contract.rs > target/false-contract.log 2>&1; then
    echo 'ERROR: verifier accepted the intentionally false contract' >&2
    exit 1
fi
if ! rg -q 'postcondition not satisfied' target/false-contract.log; then
    cat target/false-contract.log >&2
    echo 'ERROR: negative check failed for an unexpected reason' >&2
    exit 1
fi
echo 'Negative proof check: correctly rejected false postcondition'
python3 scripts/reject-fast-check.py
for check in copy target; do
    if [ "$check" = copy ]; then function=copy_byte_run; else function=storage_target; fi
    if "$VERUS" --crate-type lib --triggers-mode silent --rlimit 100 \
        --verify-root --verify-function "$function" "target/reject-fast-$check/lib.rs" > "target/reject-fast-$check.log" 2>&1; then
        echo "ERROR: verifier accepted broken optimized $check" >&2; exit 1
    fi
    if ! rg -q 'postcondition not satisfied' "target/reject-fast-$check.log"; then
        cat "target/reject-fast-$check.log" >&2; echo "ERROR: unexpected optimized $check failure" >&2; exit 1
    fi
    echo "Optimized path semantic check: correctly rejected broken $check"
done
python3 scripts/reject-all-check.py
if "$VERUS" --crate-type lib --triggers-mode silent --rlimit 100 \
    --verify-root --verify-function inspect target/reject_all.rs > target/reject-all.log 2>&1; then
    echo 'ERROR: verifier accepted a parser that rejects every input' >&2
    exit 1
fi
if ! rg -q 'postcondition not satisfied' target/reject-all.log; then
    cat target/reject-all.log >&2
    echo 'ERROR: reject-all proof check failed for an unexpected reason' >&2
    exit 1
fi
echo 'Acceptance proof check: correctly rejected always-error parser'
python3 scripts/reject-deflate-check.py
if "$VERUS" --crate-type lib --triggers-mode silent --rlimit 100 \
    --verify-only-module deflate --verify-function validate target/reject-deflate/lib.rs > target/reject-deflate.log 2>&1; then
    echo 'ERROR: verifier accepted a codec that rejects every stream' >&2
    exit 1
fi
if ! rg -q 'postcondition not satisfied' target/reject-deflate.log; then
    cat target/reject-deflate.log >&2
    echo 'ERROR: codec acceptance check failed for an unexpected reason' >&2
    exit 1
fi
echo 'Deflate acceptance check: correctly rejected always-error codec'

python3 scripts/reject-profile-check.py
for check in huffman stored; do
    if [ "$check" = huffman ]; then function='Huff::find'; else function='Bits::read'; fi
    if "$VERUS" --crate-type lib --triggers-mode silent --rlimit 100 \
        --verify-only-module deflate_bits --verify-function "$function" "target/reject-profile-$check/lib.rs" > "target/reject-profile-$check.log" 2>&1; then
        echo "ERROR: verifier accepted broken Deflate $check shortcut" >&2; exit 1
    fi
    if ! rg -q 'postcondition not satisfied' "target/reject-profile-$check.log"; then
        cat "target/reject-profile-$check.log" >&2; echo "ERROR: unexpected Deflate $check failure" >&2; exit 1
    fi
    echo "Deflate shortcut semantic check: correctly rejected broken $check"
done

python3 scripts/reject-lzw-tail-check.py
for check in acceptance soundness; do
    if "$VERUS" --crate-type lib --triggers-mode silent --rlimit 100 \
        --verify-root --verify-function lz_validate "target/reject-lzw-tail-$check/lib.rs" > "target/reject-lzw-tail-$check.log" 2>&1; then
        echo "ERROR: verifier accepted broken LZW tail $check" >&2; exit 1
    fi
    if ! rg -q 'postcondition not satisfied' "target/reject-lzw-tail-$check.log"; then
        cat "target/reject-lzw-tail-$check.log" >&2; echo "ERROR: unexpected LZW tail $check failure" >&2; exit 1
    fi
    echo "LZW tail semantic check: correctly rejected broken $check"
done

python3 scripts/reject-lzw-legacy-check.py
for check in selection width full; do
    case "$check" in
        selection) function=lz_is_legacy;;
        width) function='LzTable::width';;
        full) function='LzTable::data';;
    esac
    if "$VERUS" --crate-type lib --triggers-mode silent --rlimit 100 \
        --verify-root --verify-function "$function" "target/reject-lzw-legacy-$check/lib.rs" > "target/reject-lzw-legacy-$check.log" 2>&1; then
        echo "ERROR: verifier accepted broken legacy LZW $check" >&2; exit 1
    fi
    if ! rg -q 'postcondition not satisfied' "target/reject-lzw-legacy-$check.log"; then
        cat "target/reject-lzw-legacy-$check.log" >&2; echo "ERROR: unexpected legacy LZW $check failure" >&2; exit 1
    fi
    echo "Legacy LZW semantic check: correctly rejected broken $check"
done

python3 scripts/reject-extended-check.py
for inspector in extended raster; do
    if [ "$inspector" = extended ]; then
        module='extended::image'; function=inspect_extended
    else
        module=reader; function=inspect_raster
    fi
    if "$VERUS" --crate-type lib --triggers-mode silent --rlimit 100 \
        --verify-only-module "$module" --verify-function "$function" "target/reject-$inspector/lib.rs" > "target/reject-$inspector.log" 2>&1; then
        echo "ERROR: verifier accepted broken $inspector inspection" >&2
        exit 1
    fi
    if ! rg -q 'postcondition not satisfied' "target/reject-$inspector.log"; then
        cat "target/reject-$inspector.log" >&2
        echo "ERROR: $inspector acceptance check failed for an unexpected reason" >&2
        exit 1
    fi
    echo "Extended acceptance check: correctly rejected broken $inspector inspection"
done

python3 scripts/reject-jpeg-check.py
for check in scan image idct raster; do
    case "$check" in
        scan) module=jpeg_scan; function=validate;;
        image) module=jpeg_tiff; function=inspect_jpeg_tiff;;
        idct) module=jpeg_transform; function=idct;;
        raster) module=reader; function=inspect_raster;;
    esac
    if "$VERUS" --crate-type lib --triggers-mode silent --rlimit 100 \
        --verify-only-module "$module" --verify-function "$function" "target/reject-jpeg-$check/lib.rs" > "target/reject-jpeg-$check.log" 2>&1; then
        echo "ERROR: verifier accepted broken JPEG $check" >&2; exit 1
    fi
    if ! rg -q 'postcondition not satisfied' "target/reject-jpeg-$check.log"; then
        cat "target/reject-jpeg-$check.log" >&2; echo "ERROR: unexpected JPEG $check failure" >&2; exit 1
    fi
    echo "JPEG semantic check: correctly rejected broken $check"
done

python3 scripts/reject-fax-check.py
for check in stream image pixels raster; do
    case "$check" in
        stream) module=fax; function=process;;
        image) module=fax_tiff; function=inspect_fax_tiff;;
        pixels) module=fax; function=render;;
        raster) module=reader; function=inspect_raster;;
    esac
    if "$VERUS" --crate-type lib --triggers-mode silent --rlimit 100 \
        --verify-only-module "$module" --verify-function "$function" "target/reject-fax-$check/lib.rs" > "target/reject-fax-$check.log" 2>&1; then
        echo "ERROR: verifier accepted broken fax $check" >&2; exit 1
    fi
    if ! rg -q 'postcondition not satisfied' "target/reject-fax-$check.log"; then
        cat "target/reject-fax-$check.log" >&2; echo "ERROR: unexpected fax $check failure" >&2; exit 1
    fi
    echo "Fax semantic check: correctly rejected broken $check"
done

python3 scripts/reject-float-check.py
for check in bits prediction format; do
    case "$check" in
        bits) module=float64; function=load;;
        prediction) module=float64; function=accumulate;;
        format) module=extended; function=read_ext_format;;
    esac
    if "$VERUS" --crate-type lib --triggers-mode silent --rlimit 100 \
        --verify-only-module "$module" --verify-function "$function" "target/reject-float-$check/lib.rs" > "target/reject-float-$check.log" 2>&1; then
        echo "ERROR: verifier accepted broken float64 $check" >&2; exit 1
    fi
    if ! rg -q 'postcondition not satisfied' "target/reject-float-$check.log"; then
        cat "target/reject-float-$check.log" >&2; echo "ERROR: unexpected float64 $check failure" >&2; exit 1
    fi
    echo "Float64 semantic check: correctly rejected broken $check"
done

python3 scripts/reject-webp-check.py
for check in stream image pixels cache raster; do
    case "$check" in
        stream) module=webp; function=process;;
        image) module=webp_tiff; function=inspect_webp_tiff;;
        pixels) module=webp; function=inverse;;
        cache) module=webp; function=hash;;
        raster) module=reader; function=inspect_raster;;
    esac
    if "$VERUS" --crate-type lib --triggers-mode silent --rlimit 100 \
        --verify-only-module "$module" --verify-function "$function" "target/reject-webp-$check/lib.rs" > "target/reject-webp-$check.log" 2>&1; then
        echo "ERROR: verifier accepted broken webp $check" >&2; exit 1
    fi
    if ! rg -q 'postcondition not satisfied' "target/reject-webp-$check.log"; then
        cat "target/reject-webp-$check.log" >&2; echo "ERROR: unexpected webp $check failure" >&2; exit 1
    fi
    echo "WebP semantic check: correctly rejected broken $check"
done

python3 scripts/reject-logluv-check.py
for check in stream image pixels chroma raster; do
    case "$check" in
        stream) module=logluv; function=sg_validate;;
        image) module=logluv_tiff; function=inspect_logluv_tiff;;
        pixels) module=logluv_segment; function=convert;;
        chroma) module=logluv_segment; function=expand_uv;;
        raster) module=reader; function=inspect_raster;;
    esac
    if "$VERUS" --crate-type lib --triggers-mode silent --rlimit 100 \
        --verify-only-module "$module" --verify-function "$function" "target/reject-logluv-$check/lib.rs" > "target/reject-logluv-$check.log" 2>&1; then
        echo "ERROR: verifier accepted broken logluv $check" >&2; exit 1
    fi
    if ! rg -q 'postcondition not satisfied' "target/reject-logluv-$check.log"; then
        cat "target/reject-logluv-$check.log" >&2; echo "ERROR: unexpected logluv $check failure" >&2; exit 1
    fi
    echo "LogLuv semantic check: correctly rejected broken $check"
done

VERUS="$VERUS" python3 scripts/reject-throughput-check.py

VERUS="$VERUS" python3 scripts/reject-native-check.py

VERUS="$VERUS" python3 scripts/reject-zstd-check.py

VERUS="$VERUS" python3 scripts/reject-codecs-check.py

VERUS="$VERUS" python3 scripts/reject-reader-optimization.py
VERUS="$VERUS" python3 scripts/reject-rgba.py

VERUS="$VERUS" python3 scripts/reject-kernel-optimization.py
VERUS="$VERUS" python3 scripts/reject-lerc-arithmetic.py

VERUS="$VERUS" python3 scripts/reject-native-float.py
VERUS="$VERUS" python3 scripts/reject-lerc-throughput.py

VERUS="$VERUS" python3 scripts/reject-webp-simd.py

VERUS="$VERUS" python3 scripts/reject-webp-spans.py

VERUS="$VERUS" python3 scripts/reject-adapter.py

VERUS="$VERUS" python3 scripts/reject-run-optimization.py
