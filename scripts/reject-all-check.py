#!/usr/bin/env python3
"""Generate an intentionally broken parser while preserving its contracts."""
import pathlib, shutil

root = pathlib.Path(__file__).resolve().parents[1]
source = (root / 'src/lib.rs').read_text()
start = source.index("pub fn inspect<'a>")
body = source.index('\n{\n', start)
# inspect is the final item in verus!. Fail closed if that structure changes.
depth = 0
end = None
for pos in range(body + 1, len(source)):
    if source[pos] == '{':
        depth += 1
    elif source[pos] == '}':
        depth -= 1
        if depth == 0:
            end = pos + 1
            break
if end is None or source[end:].strip() != '}':
    raise SystemExit('Update reject-all mutation for the changed source layout')
mutated = source[:body] + '\n{\n    Err(Error::Unsupported)\n}' + source[end:]
(root / 'target/reject_all.rs').write_text(mutated)

# The negative fixture imports exactly the current verified modules.
for module in (root / "src").glob("*.rs"):
    if module.name not in ("lib.rs", "main.rs"):
        shutil.copyfile(module, root / "target" / module.name)
