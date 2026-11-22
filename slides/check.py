import html
import re
import sys
import textwrap
from pathlib import Path

root = Path(__file__).resolve().parent.parent
slides = (root / "slides/index.html").read_text(encoding="utf-8")
failures = 0

for m in re.finditer(r'data-source="([^"]+)">\n(.*?)\n</code>', slides, re.S):
    path, snippet = m.group(1), html.unescape(m.group(2))
    lines = [
        l
        for l in (root / path).read_text(encoding="utf-8").splitlines()
        if not l.strip().startswith("///")
    ]
    want = snippet.splitlines()
    found = any(
        textwrap.dedent("\n".join(lines[i : i + len(want)])) == snippet
        for i in range(len(lines) - len(want) + 1)
    )
    if not found:
        failures += 1
        print(f"out of date: {path}: {want[0]}")

if failures:
    sys.exit(1)
print("slides match the code")
