# SPDX-License-Identifier: MIT
"""Fail when a code listing runs past the PDF text measure.

TeX reports every overfull line in the book's log, but an overfull code
line and a 3 pt prose or page-furniture line look alike there. This check
reproduces the macro expansion that `texi2dvi --expand` gives to TeX, so
the log's line numbers index it exactly, keeps only the lines inside a
listing environment, and maps each one back to its chapter source.
"""

import argparse
import re
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path

EDITIONS = ("rust", "ocaml", "typescript", "kotlin")
LISTINGS = frozenset({"example", "smallexample", "lisp", "smalllisp", "verbatim"})
OVERFULL = re.compile(
    r"^Overfull \\hbox \(([0-9.]+)pt too wide\) in paragraph at lines (\d+)--\d+", re.MULTILINE
)
BOUNDARY = re.compile(r"@(end\s+)?(\w+)\b")


@dataclass(frozen=True)
class Overflow:
    """One listing line that TeX set wider than the measure."""

    width: float
    text: str
    sources: tuple[tuple[str, int], ...]


def expand(book: Path, makeinfo: str) -> list[str]:
    """Macro-expand the book exactly as `texi2dvi --expand` does before TeX."""
    result = subprocess.run(
        [
            makeinfo,
            "-c",
            "TEXINFO_OUTPUT_FORMAT=plaintexinfo",
            "--iftex",
            "--no-ifinfo",
            "--footnote-style=end",
            "-I",
            ".",
            "main.texi",
        ],
        cwd=book,
        capture_output=True,
        text=True,
        check=False,
    )
    if result.returncode != 0:
        raise RuntimeError(f"{makeinfo} failed on {book}/main.texi:\n{result.stderr}")
    return result.stdout.split("\n")


def in_listing(expanded: list[str], line: int) -> bool:
    """Whether the expanded line sits inside a listing environment.

    Listings hold no other block environment, so the nearest listing
    boundary above the line decides: an opening means inside, an end means
    outside.
    """
    for k in range(line - 1, 0, -1):
        boundary = BOUNDARY.match(expanded[k - 1].strip())
        if boundary and boundary.group(2) in LISTINGS:
            return boundary.group(1) is None
    return False


def source_index(book: Path) -> dict[str, list[tuple[str, int]]]:
    """Map each chapter source line's text to where it occurs."""
    index: dict[str, list[tuple[str, int]]] = {}
    for path in sorted(book.rglob("*.texi")):
        if "_build" in path.parts:
            continue
        name = str(path.relative_to(book))
        for number, text in enumerate(path.read_text(encoding="utf-8").split("\n"), 1):
            index.setdefault(text, []).append((name, number))
    return index


def listing_overflows(book: Path, makeinfo: str, tolerance: float) -> list[Overflow]:
    """Every listing line in the last PDF build that exceeds the measure."""
    log = (book / "main.log").read_text(encoding="latin-1")
    expanded = expand(book, makeinfo)
    index = source_index(book)
    found: list[Overflow] = []
    for match in OVERFULL.finditer(log):
        width, line = float(match.group(1)), int(match.group(2))
        if width <= tolerance or not in_listing(expanded, line):
            continue
        text = expanded[line - 1]
        found.append(Overflow(width, text, tuple(index.get(text, []))))
    return found


def main(argv: list[str] | None = None) -> int:
    """Report listing overflows per edition; exit 1 when any remain."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path())
    parser.add_argument("--edition", choices=EDITIONS, action="append")
    parser.add_argument("--makeinfo", default="makeinfo")
    parser.add_argument("--tolerance", type=float, default=1.0, help="points of overflow to allow")
    args = parser.parse_args(argv)
    failures = 0
    for edition in args.edition or EDITIONS:
        book = args.root / edition / "book"
        if not (book / "main.log").is_file():
            print(f"{edition}: missing {book}/main.log; build the PDF first", file=sys.stderr)
            failures += 1
            continue
        newest = max(p.stat().st_mtime for p in book.rglob("*.texi") if "_build" not in p.parts)
        if (book / "main.log").stat().st_mtime < newest:
            print(
                f"{edition}: {book}/main.log predates the book source; rebuild the PDF",
                file=sys.stderr,
            )
            failures += 1
            continue
        overflows = listing_overflows(book, args.makeinfo, args.tolerance)
        for item in overflows:
            where = ", ".join(f"{name}:{number}" for name, number in item.sources) or "unmapped"
            print(f"{edition}: {where}: {item.width:.1f}pt too wide: {item.text.strip()}")
        print(f"{edition}: listing overflows={len(overflows)}")
        failures += len(overflows)
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
