# SPDX-License-Identifier: MIT
"""Generate exercise and figure indexes from numbered anchors."""

import argparse
import re
import sys
from collections.abc import Sequence
from pathlib import Path


def source_files(tree: Path) -> list[Path]:
    """Return source parts, excluding generated indexes and the assembly file."""
    excluded = {tree / "main.texi", tree / "back/exercises.texi", tree / "back/figures.texi"}
    return sorted(
        path
        for path in tree.rglob("*.texi")
        if path not in excluded and "_build" not in path.relative_to(tree).parts
    )


def number_key(number: str) -> tuple[int, int, str]:
    """Order additions immediately after their original exercise."""
    match = re.fullmatch(r"(\d+)\.(\d+)([a-z]?)", number)
    if match is None:
        raise ValueError(f"Invalid exercise or figure number: {number}")
    return int(match[1]), int(match[2]), match[3]


def anchors(text: str, kind: str) -> list[str]:
    """Collect unique numbered anchors in book order."""
    return sorted(set(re.findall(rf"@anchor\{{{kind} (\d+\.\d+[a-z]?)\}}", text)), key=number_key)


def index_text(numbers: list[str], kind: str) -> str:
    """Render an eight-column chapter table with no hard-coded chapter counts."""
    chapters = sorted({number_key(number)[0] for number in numbers})
    lines: list[str] = []
    for chapter in chapters:
        lines.extend(
            [
                f"@subsubheading Chapter {chapter}",
                "",
                "@multitable @columnfractions " + " ".join(["0.12"] * 8),
            ]
        )
        group = [number for number in numbers if number_key(number)[0] == chapter]
        for start in range(0, len(group), 8):
            row = group[start : start + 8]
            lines.append("@item " + " @tab ".join(f"@ref{{{kind} {n},,{n}}}" for n in row))
        lines.extend(["@end multitable", ""])
    return "\n".join(lines)


def main(argv: Sequence[str] | None = None) -> int:
    """Write indexes, or reject missing and stale files without changing them."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--tree", type=Path, required=True)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args(argv)
    try:
        files = source_files(args.tree)
        if not files:
            raise ValueError(f"No Texinfo source in {args.tree}")
        text = "\n".join(path.read_text(encoding="utf-8") for path in files)
        counts: dict[str, int] = {}
        stale = 0
        for kind, name in (("Exercise", "exercises"), ("Figure", "figures")):
            numbers = anchors(text, kind)
            counts[name] = len(numbers)
            content = index_text(numbers, kind)
            path = args.tree / "back" / f"{name}.texi"
            if args.check:
                if not path.is_file() or path.read_text(encoding="utf-8") != content:
                    print(f"{path}: stale index", file=sys.stderr)
                    stale += 1
            else:
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text(content, encoding="utf-8")
        print(f"exercises={counts['exercises']} figures={counts['figures']} stale={stale}")
    except (OSError, ValueError) as exc:
        print(str(exc), file=sys.stderr)
        return 1
    return int(stale != 0)


if __name__ == "__main__":
    raise SystemExit(main())
