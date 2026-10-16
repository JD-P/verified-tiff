# Codec metadata and ink-name consistency

This pass fixes the two gaps found during the C shim CVE reassessment: known
codec tags falling through to the anonymous-tag ABI, and inconsistent InkNames
and NumberOfInks. It also corrects the InkNames getter's calling convention.

## Registered tag schemas

`compat/generate_fields.py` now accounts for every row in the pinned libtiff
4.7.1 standard and codec field tables. It imports all **15 on-file codec tag
schemas**: the five fax fields, seven OJPEG fields, JPEGTables, Predictor and
LercParameters. Pseudo tags and explicitly ignored fields are omitted. Existing
DNG/EP declarations remain supported. The parser also handles TRUE/FALSE flags,
which recovers four previously missed standard fax reception fields:
FaxRecvParams, FaxSubAddress, FaxRecvTime and FaxDcs.

Known tags retain their registered host type and argument shape independently
of the file's stored type. BadFaxLines, for example, takes a `uint32_t *` output,
never an anonymous tag's count and pointer outputs. Integer conversions either
preserve the mathematical value or reject it; malformed scalar counts and
unrepresentable values leave outputs unchanged. Custom registration cannot
replace a built-in schema. Missing fields return absence without consuming a
second, nonexistent output argument.

Every C library build checks that the generated table exactly matches its
pinned source. Unparsed table rows, unfamiliar codec calling shapes, conflicting
codec definitions, or a changed codec inventory fail generation. Declarative
schema extraction and C varargs handling remain outside the formal proof.

## Ink policy and ABI

For image directories, InkNames must be ASCII with **1–65,535 bytes**, and its
exact declared payload must end in NUL. The extra terminator reserved by metadata
storage does not make an unterminated file payload valid. Empty individual names
are accepted, as in libtiff, and each NUL terminates one name.

The executable `adapter_meta::ink_names` function proves exact acceptance of
that predicate, an exact count of terminating NULs, and a result between 1 and
65,535. Directory loading derives NumberOfInks from that result, inserts the
field when absent, and corrects a mismatched declaration. This follows the
observable normalization behavior of libtiff 4.7.1.

Setting InkNames validates both fields on candidate storage before publishing
them. Setting NumberOfInks inconsistently fails and preserves the old metadata.
Unsetting NumberOfInks while InkNames exists also fails; unsetting InkNames
allows a standalone count. Without names, NumberOfInks must have one unsigned
value representable as a uint16. Allocation and metadata/tag limits still apply.
The same rules cover the metadata-only temporary writer used by read clients.
EXIF/GPS/custom directory namespaces do not acquire TIFF image ink semantics.

The public getter is `TIFFGetField(t, TIFFTAG_INKNAMES, &names)` with a single
`char **` output. Its setter remains `TIFFSetField(t, TIFFTAG_INKNAMES,
byte_count, names)`. The field descriptor's counted-setter metadata must not be
used to infer this special getter's argument list.

## Recorded validation

The current evidence is in `docs/evidence/metadata-contracts/summary.json`.

* Whole-crate Verus: **1,809 verified obligations, zero errors**. No additional
  trusted primitives or axioms. Three new negative proof mutations corrupt
  the count, reject every valid input, and reject the accepted maximum length.
* **71 Rust tests**, including exhaustive short-string comparisons against an
  independent iterator model, plus **97 core Python tests**.
* **70 C integration tests** in optimized, sanitizer and coverage runs, plus
  the C boundary fault-injection harness. The six new test methods exercise
  the 15 codec and four fax reception schemas, scalar/output guards, absent
  fields, conflicting registration, malformed types/counts/narrowing, and
  Classic TIFF/BigTIFF in both byte orders. Positive ABI and ink cases use an
  independently linked libtiff 4.7.1 consumer as an oracle.
* Ink regressions cover missing/mismatched counts, empty individual names,
  maximum-length strings, missing terminators, oversized data, invalid type,
  atomic setters, failed directory selection, namespaces, and reopening
  metadata produced by all four temporary writer byte-order/header modes.
* ASan, UBSan and float-cast-overflow: passing C boundary/integration runs.
  LeakSanitizer remains unavailable in this container; the C allocation ledger
  returns to zero in the boundary harness.
* Production C coverage: **242/242 functions, 1,351/1,351 executable lines,
  1,455/1,455 branch outcomes, all 198 public API functions**. No exclusions.
  The small denominator reduction comes from removing the now-redundant
  handwritten LercParameters lookup, not from excluding code from coverage.
* **20,000 deterministic mutation smoke iterations** completed. This is not
  a coverage-guided fuzz campaign or a replay of every historical CVE exploit.

The verified counting helper does not prove all C metadata composition, pointer
validity, callback behavior or varargs extraction. The integration regressions
and 100% C execution coverage supplement those local contracts; neither is an
end-to-end security proof.

To reproduce, bootstrap the pinned dependencies, then run `scripts/test.sh`,
`scripts/test-compat.sh`, `scripts/verify.sh` (or `verify-parallel.py`), and
`scripts/fuzz-smoke.sh 20000`. Run the compatibility suite again with
`VTIFF_SANITIZE=1`, `ASAN_OPTIONS=detect_leaks=0`, and a separate
`VTIFF_COMPAT_OUTPUT`, then `scripts/test-c-coverage.sh`.
