# IEEE binary64 samples and horizontal prediction

`inspect_raster` and `inspect_extended` accept 64-bit IEEE floating-point
samples (`BitsPerSample=64`, `SampleFormat=3` for every channel). This extends
the existing extended profile and its complete acceptance theorem; it is not
a separate fallback or an external decoder.

## Supported combinations

| Property | Accepted values |
| --- | --- |
| Samples | BlackIsZero grayscale, one channel; RGB, three channels |
| Storage | Strips or tiles; contiguous or separate planes; either TIFF byte order |
| Predictor 1 | Uncompressed, PackBits, LZW, Deflate 8/32946 |
| Predictor 2 | LZW and Deflate 8/32946 |
| Other transforms | FillOrder 1, Orientation 1, no alpha |

The original directory, codec and resource rules apply: sorted unique tags,
complete segment arrays, validated encoded streams, full edge-tile storage,
checked arithmetic, bounded dimensions/input/output/segments and work budgets.
Deflate retains its bounded short-final-strip padding policy. Padding is
validated and charged but is not returned as image samples.

Missing SampleFormat defaults to unsigned integer in TIFF and is therefore
rejected for this 64-bit profile. Every channel must have depth 64 and format
3; mixed types/depths are rejected. Float32, unsigned/signed 64-bit samples,
float alpha, WhiteIsZero floating-point images, and Predictor 3 (floating-point
byte shuffling) are not implemented. Predictor 2 remains unsupported for the
existing integer representations. Predictor 2 on uncompressed/PackBits input
is rejected, because those codecs do not implement that transform here.

## Exact bits and prediction

Output contains interleaved, row-major, **little-endian binary64 bytes**,
independent of input byte order. The core does not execute floating-point
arithmetic, normalize NaNs, clip values, rescale grayscale, or tone map.
Positive/negative zero, subnormals, infinities, signaling/quiet NaNs and their
payloads are transported bit for bit.

Predictor 2 is integer horizontal differencing of the underlying 64-bit
sample words. For each channel in each storage row:

```
output[x] = stored[x]                                      if local_x == 0
output[x] = (stored[x] + output[x - 1]) modulo 2^64         otherwise
```

Here `x - 1` means the preceding pixel's sample of the **same channel**.
Each tile starts a new horizontal sequence. Each row resets the sequence;
neither a preceding row nor an adjacent tile supplies the predecessor.
Separate planes and contiguous RGB therefore reconstruct identically.
The implementation follows the integer accumulation and endian handling in
checksum-pinned libtiff 4.7.1 `libtiff/tif_predict.c` (`horAcc64`,
`swabHorAcc64`), rather than performing floating-point addition.

Native decoded bytes are first scattered into caller scratch using the
existing proved segment geometry. Removed right-edge tile padding cannot
influence a visible predecessor. The float pass reads a sample word using
its input byte order, reconstructs it using a fixed four-word predecessor
array, and writes its eight canonical bytes. It resets at `x % block_width == 0`.
No image-sized allocation, unsafe code, C codec, or runtime recursion is added.

## API and CLI

`Raster::sample_format()` returns `SampleFormat::IeeeFloat` or
`SampleFormat::UnsignedInteger`. `Raster::predictor()` reports the source
predictor. Float64 has `bits_per_sample() == output_bits_per_sample() == 64`.
`ExtendedImage` also exposes `is_floating_point()` and `predictor()`.
`output_len()` and `scratch_len()` count bytes, not sample elements. Each is
`width * height * channels * 8`, independently limited by `max_output`.

A caller can interpret each eight-byte output slice with
`f64::from_bits(u64::from_le_bytes(bytes))`. Applications that need to retain
signaling NaNs should keep the integer bits or bytes and avoid operations that
may quiet them. The decoder itself never constructs an `f64`.

The CLI writes a NumPy **NPY 1.0** file for float64 input, regardless of the
output filename extension. Its descriptor is `<f8`, C order, with shape
`(height, width)` for grayscale or `(height, width, 3)` for RGB. This preserves
all bits and precision. Integer images continue to use PGM/PPM/PAM. Example:

```bash
target/release/verified-tiff input-float64.tif output.npy
```

The adapter writes the documented magic/version, little-endian header length,
ASCII dictionary and newline, padding the data offset to a 64-byte boundary.
See the [NumPy file-format specification](https://numpy.org/doc/stable/reference/generated/numpy.lib.format.html).
There is no NumPy runtime dependency. JSON adds `sample_format`, `predictor`,
and `output_byte_order`. File I/O and NPY packaging remain outside the verified
core, with independent header/shape/payload tests.

## Proof and resource scope

`float64.rs` specifies a source sample as the weighted sum of eight original
native bytes, then specifies the horizontal prefix sum modulo `2^64` and the
canonical little-endian digits. Executable loading, modular addition,
predecessor tracking, byte writing and conversion refine those definitions.
The conversion's output and suffix frame compose with the existing verified
codec, scratch and scatter contracts.

`ExtFormat`, `RasterLayout`, `supported_extended_profile` and their acceptance
proofs include these new combinations. The public raster union and original
smaller-budget guarantee are preserved. Successful inspection still implies
successful decoding with sufficient buffers and the reported decode quota.
Two concrete 160-byte LZW TIFF witnesses prove non-vacuity for Predictor 1/2;
the second reconstructs `u64::MAX + 2 == 1` modulo `2^64`. Matching runtime
fixtures check the exact bit patterns. Three negative gates reject zero sample
loads, omitted prediction and an inspector that rejects every float64 image.

The sufficient inspection formula is unchanged. The decode quota is also the
existing extended quota: scratch bytes + output bytes + segment codec work.
The added pass performs constant bounded work per eight-byte sample, covered
by output-byte work. It reserves the quota before writing either caller buffer.
Undersized buffers or insufficient work leave both buffers unchanged. Compiled
instruction counts, process RSS and compiler/OS correctness are not proved.

The proof is relative to these explicit bit-level models; it does not prove
floating-point arithmetic, the NPY writer, all TIFF extensions, or fidelity of
the model to every prose standard. Independent oracle comparisons remain
necessary evidence against specification mistakes.

## Validation and coverage

`tests/float64.py` exercises 256 format/layout/endian/codec combinations and
16 page/edge cases, with independent integer-bit expectations and live libtiff
native-sample comparisons. It also checks malformed metadata, every truncation
of small files, output/work limits and both original float64 fixtures.
`tests/float_api.rs` checks exact quotas, buffer frames, typed output metadata,
modular overflow and the concrete witnesses. Six committed float fixtures
join the deterministic mutation corpus.

Both `test_float64_predictor2_be_lzw.tif` and
`test_float64_predictor2_le_lzw.tif` match libtiff exactly. Original-file
coverage is **27/31 (87.1%)**: 23 exact comparisons and four JPEG comparisons
within the documented numerical tolerance. This measures each file's first
main image, not the fraction of all libtiff tests or features implemented.
