# Original whole-image lossless WebP profile

The C reader now additionally supports full static VP8/VP8L through the
separate [native WebP engine](webp-native.md). This page documents the retained
original `Raster`/CLI profile and its whole-image theorem.

`inspect_raster` and the direct `inspect_webp_tiff` / `WebpImage` API accept
TIFF Compression 50001 under the explicit VP8L profile below. The decoder is
implemented in Rust inside Verus. It does not link libwebp or libtiff.

## TIFF representation

- Classic TIFF, either byte order; positive, bounded dimensions.
- Eight-bit unsigned RGB or RGBA, contiguous samples, Photometric 2.
  BitsPerSample has one value per channel; SampleFormat is absent or all 1.
- RGB has no ExtraSamples. RGBA has one trailing ExtraSamples=1/2 channel.
  Output uses the existing straight-alpha policy, including nearest/clipped
  unassociation and zero color at associated alpha zero.
- Predictor, FillOrder, Orientation and PlanarConfiguration are all 1.
- Strips and tiles are independent RIFF WebP images. A strip encodes exactly
  its actual rows; a tile encodes its full padded rectangle. WebP dimensions
  must match the storage rectangle. Each segment dimension is at most 16,384.
  Whole tiled images can be larger within the caller's limits.
- Tile dimensions are multiples of 16. Segment arrays have exactly the
  computed number of entries; every byte range is checked.
- The directory reader allows unordered tags and all-zero tombstones, and
  rejects duplicate recognized tags. Main-directory traversal is bounded.

The VP8L `alpha_is_used` bit is a hint, not the output channel count. TIFF
SamplesPerPixel/ExtraSamples determine RGB versus RGBA. In particular, the
original `webp_lossless_rgba_alpha_fully_opaque.tif` remains RGBA even though
its VP8L hint is zero. All 400 alpha samples decode to 255.

## Supported VP8L syntax

One complete simple RIFF/WEBP container with one VP8L chunk is required.
RIFF and chunk sizes must agree exactly with the TIFF segment byte count;
odd chunks require their zero pad byte. Signature 0x2f and version zero are
required. Lossy VP8, VP8X, animation, additional RIFF chunks and raw payloads
without the container are outside this profile.

The stream accepts no transform or exactly one subtract-green transform.
There is one Huffman group. Color caches are absent or use 1 through 11 bits,
including initialized transparent-black entries and replacement collisions.
Predictor, color-transform and color-indexing transforms, and entropy
meta-images/multiple Huffman groups, are not implemented. This is a useful
bounded subset of lossless WebP, not general WebP support.

Simple and normal canonical Huffman descriptions are supported, with code
lengths up to 15. A single length-one leaf consumes zero data bits. Duplicate
simple symbols collapse to that single leaf. Empty, oversubscribed or
incomplete multi-leaf trees, and out-of-alphabet simple symbols, are rejected.
Normal code-length descriptions implement the token-count limit, default
previous nonzero length 8, and repeats 16, 17 and 18. Zero literal lengths do
not reset the previous nonzero length. Unfilled alphabet entries are zero.

The raster supports literals, every length prefix 0..23, every distance
prefix 0..39, all 120 plane-distance mappings, and linear distances. Copies
may overlap; references must point to already produced pixels and cannot
exceed the required output. Every produced pixel updates the color cache,
including pixels from copies and cache lookups. Cache hashing uses the
specified ARGB word and modulo-2^32 multiplication. Inverse subtract-green
runs after entropy decoding, with separate modulo-256 red and blue sums.

Exactly the required pixel count must be produced. No whole unused byte may
remain in the VP8L payload; unused bits in its final byte are ignored. The
RIFF odd-byte pad is separate. This trailing-data policy is intentionally
stricter than permissive decoders. Unsupported transforms are rejected before
attempting to interpret their metadata as ordinary raster data.

## Memory and work

The core borrows input, has no heap allocation or runtime recursion, and uses
five fixed-capacity Huffman tables (2,328 lengths and 16 counts each) and a
2,048-entry RGBA cache. Header parsing temporarily uses additional bounded
arrays. Exact stack-frame placement and stack exhaustion are not proved.

Inspection needs no caller raster scratch. The control decisions in this
profile depend on symbols, lengths and distances, not on decoded pixel
values. The validator tracks the complete pixel/cache history only as erased
proof state. Decoding stores real history in caller scratch. This distinction
is checked against the same byte-stream model, not a separate weaker parser.

`scratch_len()` is the maximum `4 * segment_width * segment_height` across
segments, even for RGB output. The RGB conversion compacts that RGBA storage
in place. Both scratch and final output are independently limited by
`Limits::max_output`, so total caller raster storage is at most twice the
limit. Output/scratch suffixes are preserved.

For encoded segment size `N` and RGBA scratch size `P`, the reserved abstract
work fee is `1,000,000 + 64*N + 3,000*P`. The fixed term covers bounded table
construction; the remaining terms conservatively charge stream decoding,
cache/copy work, conversion and scattering. These units are not CPU
instructions or a timing guarantee. Decode reserves the complete sum before
writing. Either undersized buffer or insufficient work leaves both unchanged.

With `D=(page+1)*(max_tags+1)`, the sufficient direct inspection bound is:

`D + 128 + max_segments*(64*max_input + 3000*max_output + 1000000)`.

`webp_inspection_work_limit` computes it with checked arithmetic and succeeds
exactly when it fits `usize`. The public raster calculator adds all six
profile bounds and retains the original profile's smaller-budget guarantee.
The CLI still uses its explicit shared work quota; large WebP images may
need `--work` raised above the default 100 million units.

## Proof and validation boundaries

The checked modules specify/refine canonical trees, code-length descriptions,
container fields, symbol/length/distance tokens, pixel/cache transitions,
exact termination, inverse transform, alpha/RGB conversion, TIFF geometry,
segment scattering and buffer frames. `supported_webp_profile` is a predicate
over source bytes and limits. Sufficient budget implies successful inspection;
a well-formed image decodes successfully with the reported buffers and quota.
The public `supported_raster_profile` is now the union of six TIFF profiles, including native SGILog32/LogLuv16.

A concrete 160-byte TIFF witness proves this profile is inhabited and is
also decoded by a Rust API test. Five new negative gates reject an always-error
stream decoder, an always-error TIFF inspector, omitted inverse-transform
writes, a constant-zero cache hash and removed WebP public dispatch.

Tests compare known pixels with separately built, checksum-pinned libwebp
1.6.0 and libtiff 4.7.1. The 192-case TIFF matrix covers byte orders, strips,
tiles, segment placement, RGB, both alpha conventions, subtract-green and
multiple cache sizes. Additional cases cover all 120 plane distances, long
linear distances, overlapping copies, all cache widths, maximum Huffman
length, normal/simple single leaves, repeats/token limits, pages, strict
termination, truncation and malformed metadata/trees. The unchanged upstream
opaque-alpha fixture is exact. See `validation.md` for the recorded full run.

Normative model references:

- https://developers.google.com/speed/webp/docs/webp_lossless_bitstream_specification
- https://developers.google.com/speed/webp/docs/riff_container
- libtiff 4.7.1 `libtiff/tif_webp.c` for TIFF framing/channel integration.

Proofs establish the implemented mathematical model. Independent review of
that model against these specifications remains necessary. The CLI, test
encoders/oracles, Rust compiler, Verus, solver and hardware retain their
existing trust boundaries.
