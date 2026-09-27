# SPDX-License-Identifier: MIT
"""Check that every math fragment the build requested reached the HTML as MathML.

texi2any exits 0 when the tex4ht run fails: it drops the math from the output
and leaves the only trace in a `*_tex4ht_*.log` file. A build that loses every
equation therefore looks green, so the output is checked here instead.

The count to compare against is the one texi2any itself states. For each math
construct it converts, it writes a `tex4ht_begin ... <kind> <n>` marker into the
TeX file it hands to tex4ht, so that file records exactly what the build asked
for. Counting `@math` in the sources instead would overcount, because the HTML
build skips constructs inside `@iftex` and other ignored regions.
"""

import argparse
import re
import sys
from pathlib import Path

MATH_ELEMENT = re.compile(r"<math[\s>]")
REQUEST = re.compile(r"tex4ht_begin \S+ (?:math|displaymath|tex|latex) \d+")
SIDECAR = re.compile(r"_tex4ht_[^.]*\.html$")
PNG_FALLBACK = re.compile(r"""<img[^>]+src=["'][^"']*_tex4ht_[^"']*\.png""")
UNSUPPORTED = re.compile(r"\\text\s*\{|\\frac\s*\{")
TEX_ERROR = re.compile(r"^! (.+)$", re.MULTILINE)


def scan_sources(tree: Path) -> list[str]:
    """Report sources using math macros that the plain-TeX run cannot resolve."""
    defects: list[str] = []
    for path in sorted(tree.rglob("*.texi")):
        text = path.read_text(encoding="utf-8", errors="replace")
        for number, line in enumerate(text.splitlines(), start=1):
            found = UNSUPPORTED.search(line)
            if found is None:
                continue
            macro = found[0].strip()
            replacement = "\\hbox{...}" if macro.startswith("\\text") else "{a \\over b}"
            defects.append(
                f"{path}:{number}: {macro}...}} is undefined in the TeX math run; use {replacement}"
            )
    return defects


def count_requests(tree: Path) -> int:
    """Count the math fragments texi2any handed to tex4ht."""
    total = 0
    for path in sorted(tree.rglob("*_tex4ht_*.tex")):
        total += len(REQUEST.findall(path.read_text(encoding="utf-8", errors="replace")))
    return total


def scan_html(tree: Path) -> tuple[int, list[str]]:
    """Count MathML elements in the delivered pages, ignoring tex4ht sidecars.

    tex4ht writes its own `<basename>_tex4ht_<format>.html` beside the output.
    That sidecar carries the MathML even on builds where texi2any fails to fold
    it into the delivered page, so counting it would hide the very defect this
    check exists to catch.
    """
    total = 0
    defects: list[str] = []
    for path in sorted(tree.rglob("*.html")):
        if SIDECAR.search(path.name):
            continue
        text = path.read_text(encoding="utf-8", errors="replace")
        total += len(MATH_ELEMENT.findall(text))
        if PNG_FALLBACK.search(text):
            defects.append(
                f"{path}: math rendered as a PNG image; the tex4ht mathml option is missing"
            )
    return total, defects


def scan_logs(tree: Path) -> list[str]:
    """Report the TeX errors that texi2any swallows."""
    defects: list[str] = []
    for path in sorted(tree.rglob("*_tex4ht_*.log")):
        text = path.read_text(encoding="utf-8", errors="replace")
        defects.extend(
            f"{path}: TeX error: {message.strip()}" for message in TEX_ERROR.findall(text)
        )
    return defects


def main(argv: list[str] | None = None) -> int:
    """Run the checks and return the process exit status."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--html", required=True, type=Path, help="directory holding the generated HTML"
    )
    parser.add_argument("--source", type=Path, help="Texinfo tree to scan for banned macros")
    args = parser.parse_args(argv)

    if not args.html.is_dir():
        print(f"math_check: not a directory: {args.html}", file=sys.stderr)
        return 1

    elements, defects = scan_html(args.html)
    defects.extend(scan_logs(args.html))
    requested = count_requests(args.html)
    if elements < requested:
        defects.append(
            f"math dropped: the build requested {requested} fragment(s), "
            f"the delivered pages hold {elements}"
        )

    if args.source is not None:
        if not args.source.is_dir():
            print(f"math_check: not a directory: {args.source}", file=sys.stderr)
            return 1
        defects.extend(scan_sources(args.source))

    for defect in defects:
        print(defect, file=sys.stderr)
    print(f"requested={requested} elements={elements} defects={len(defects)}")
    return 1 if defects else 0


if __name__ == "__main__":
    raise SystemExit(main())
