# Source provenance

`libwebp/`: COPYRIGHT/BSD license, patent grant and authors from libwebp 1.6.0.
The VP8 probability/quantization tables and integer codec algorithms in
`src/vp8_*` and `src/webp_*` were translated with reference to that pinned
source. `scripts/vp8-tables.py` regenerates the imported numeric tables.

`lerc/`: Apache 2.0 license and NOTICE from Esri LERC 4.0.0. The new Lerc2 v4
bitstream models and decoder were implemented with reference to its header,
mask, bit-stuffing, Huffman and quantization algorithms.

Exact downloaded archive hashes are in `toolchain-lock.json`. Native upstream
libraries are independent test dependencies only. No native LERC or WebP
library is included or linked into the production decoder.

`compat/color_api.inc` and `compat/logluv_api.inc` adapt the public numerical
color helpers from pinned libtiff 4.7.1 `tif_color.c` and `tif_luv.c`;
`compat/uv_table.inc` contains its Greg Ward Larson chromaticity table. Their
original permissive notices are preserved in those files. These are unverified
C presentation helpers, not codec parsers or additions to the verified core.
