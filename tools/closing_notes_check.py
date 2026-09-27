# SPDX-License-Identifier: MIT
"""Check that the required sections close with an In this language note."""

import argparse
import sys
from collections.abc import Sequence
from pathlib import Path

NOTE_SECTIONS = ("ch2/2.4.texi", "ch2/2.5.texi", "ch3/3.3.texi", "ch4/4.1.texi")
NOTE_PREFIX = "@strong{In this language:"


def has_closing_note(text: str) -> bool:
    """Find a quotation whose first non-blank line opens the note."""
    lines = text.splitlines()
    for index, line in enumerate(lines):
        if line.strip() != "@quotation":
            continue
        for following in lines[index + 1 :]:
            stripped = following.strip()
            if not stripped:
                continue
            if stripped.startswith(NOTE_PREFIX):
                return True
            break
    return False


def main(argv: Sequence[str] | None = None) -> int:
    """Report the required closing notes, one stderr line per missing note."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--tree", type=Path, required=True, help="edition book tree")
    args = parser.parse_args(argv)
    missing = 0
    for relative in NOTE_SECTIONS:
        try:
            present = has_closing_note((args.tree / relative).read_text(encoding="utf-8"))
        except OSError:
            present = False
        if not present:
            print(f"{relative}: missing closing note", file=sys.stderr)
            missing += 1
    print(f"notes={len(NOTE_SECTIONS)} missing={missing}")
    return int(missing != 0)


if __name__ == "__main__":
    raise SystemExit(main())
