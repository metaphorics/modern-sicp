# SPDX-License-Identifier: MIT
"""Check numbered teaching material and references against the migration inventory."""

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
from collections.abc import Mapping, Sequence
from html.parser import HTMLParser
from pathlib import Path
from typing import TypeIs, override

from texi_indexes import anchors, source_files

EDITIONS = ("rust", "ocaml", "typescript", "kotlin")
IDENTITIES = ("sections", "exercises", "figures")
PRESERVED_FIELDS = ("parts", *IDENTITIES, "references", "images")
EXACT_FIELDS = ("parts", *IDENTITIES, "images")


class RenderedAnchors(HTMLParser):
    """Collect anchor targets from the delivered HTML."""

    def __init__(self) -> None:
        """Start a separate target inventory for each converted book."""
        super().__init__()
        self.identifiers: set[str] = set()

    @override
    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        for name, value in attrs:
            if name == "id" and value is not None:
                self.identifiers.add(value)


def is_object(value: object) -> TypeIs[dict[object, object]]:
    """Narrow a decoded container without assuming its contents."""
    return isinstance(value, dict)


def is_array(value: object) -> TypeIs[list[object]]:
    """Narrow a decoded array before validating its entries."""
    return isinstance(value, list)


def read_inventory(path: Path, edition: str) -> dict[str, set[str]]:
    """Read one edition's preserved fields; reject incomplete inventories."""
    document: object = json.loads(path.read_text(encoding="utf-8"))
    for key in ("sources", edition):
        if not is_object(document):
            raise ValueError(f"{path}: expected an object containing {key}")
        document = document.get(key)
    if not is_object(document):
        raise ValueError(f"{path}: missing inventory for {edition}")
    fields = document
    result: dict[str, set[str]] = {}
    for kind in PRESERVED_FIELDS:
        entries = fields.get(kind)
        if not is_array(entries) or (kind in ("parts", *IDENTITIES) and not entries):
            raise ValueError(f"{path}: {edition}.{kind} must be a nonempty list")
        values: set[str] = set()
        for entry in entries:
            if not isinstance(entry, str) or not entry:
                raise ValueError(f"{path}: invalid {edition}.{kind} entry: {entry!r}")
            if kind in IDENTITIES and re.fullmatch(r"\d+\.\d+[a-z]?", entry) is None:
                raise ValueError(f"{path}: invalid {edition}.{kind} identity: {entry!r}")
            if entry in values:
                raise ValueError(f"{path}: duplicate {edition}.{kind} entry: {entry}")
            values.add(entry)
        result[kind] = values
    return result


def identity_defects(tree: Path, expected: Mapping[str, set[str]]) -> list[str]:
    """Report missing preserved targets and unexpected exact-inventory entries."""
    parts = source_files(tree)
    if not parts:
        raise ValueError(f"No Texinfo source in {tree}")
    text = "\n".join(path.read_text(encoding="utf-8") for path in parts)
    reference_text = "\n".join(
        [text, (tree / "main.texi").read_text(encoding="utf-8")]
        if (tree / "main.texi").is_file()
        else [text]
    )
    references = {
        " ".join(reference.split())
        for reference in re.findall(r"^@node[ \t]+([^,]+)", reference_text, re.MULTILINE)
    }
    references.update(
        " ".join(reference.split())
        for reference in re.findall(r"@anchor\{\s*([^}]+)", reference_text)
    )
    references.update(
        " ".join(reference.split())
        for reference in re.findall(
            r"@(?:ref|xref|pxref)\s*\{\s*([^,}]+)", reference_text, re.DOTALL
        )
    )
    # Adapted prose may change link usage, but preserved targets must remain available.
    actual = {
        "parts": {path.relative_to(tree).as_posix() for path in parts},
        "sections": set(re.findall(r"^@node[ \t]+(\d+\.\d+)(?=,|\s*$)", text, re.MULTILINE)),
        "exercises": set(anchors(text, "Exercise")),
        "figures": set(anchors(text, "Figure")),
        "references": references,
        "images": {image.strip() for image in re.findall(r"@image\{\s*([^,}]+)", reference_text)},
    }
    defects: list[str] = []
    for kind in PRESERVED_FIELDS:
        defects.extend(f"{kind}: missing {item}" for item in sorted(expected[kind] - actual[kind]))
    for kind in EXACT_FIELDS:
        defects.extend(
            f"{kind}: unexpected {item}" for item in sorted(actual[kind] - expected[kind])
        )
    return defects


def check_references(tree: Path, texi2any: str, expected: Mapping[str, set[str]]) -> list[str]:
    """Reject conversion errors and report numbered material absent from the output."""
    rendered = RenderedAnchors()
    with tempfile.TemporaryDirectory(prefix="sicp-structure-") as directory:
        result = subprocess.run(
            [
                texi2any,
                "--html",
                "--no-split",
                "-o",
                str(Path(directory) / "index.html"),
                "main.texi",
            ],
            cwd=tree,
            capture_output=True,
            text=True,
            check=False,
            timeout=180,
        )
        if result.returncode == 0:
            rendered.feed((Path(directory) / "index.html").read_text(encoding="utf-8"))
            rendered.close()
    if result.stderr:
        print(result.stderr, file=sys.stderr, end="")
    if result.returncode:
        raise ValueError(f"{tree}: texi2any failed with exit {result.returncode}")
    if re.search(
        r"(?:undefined|nonexistent|does not exist).*?(?:node|reference)|"
        r"(?:node|reference).*?(?:undefined|nonexistent|does not exist)",
        result.stderr,
        re.IGNORECASE,
    ):
        raise ValueError(f"{tree}: unresolved Texinfo reference")
    defects: list[str] = []
    for kind, prefix in (("sections", "g_t"), ("exercises", "Exercise-"), ("figures", "Figure-")):
        for number in sorted(expected[kind]):
            identifier = prefix + number.replace(".", "_002e")
            if identifier not in rendered.identifiers:
                defects.append(f"{kind}: not rendered {number}")
    return defects


def main(argv: Sequence[str] | None = None) -> int:
    """Check selected editions without changing sources or the preserved inventory."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path())
    parser.add_argument("--edition", choices=EDITIONS)
    parser.add_argument("--inventory", type=Path)
    parser.add_argument("--texi2any", default=os.environ.get("TEXI2ANY", "texi2any"))
    args = parser.parse_args(argv)
    inventory = args.inventory or args.root / "spec/book-inventory.json"
    editions = (args.edition,) if args.edition else EDITIONS
    try:
        converter = shutil.which(args.texi2any)
        if converter is None:
            raise ValueError(f"Texinfo executable not found: {args.texi2any}")
        texi2any = str(Path(converter).absolute())
        version = subprocess.run(
            [texi2any, "--version"], capture_output=True, text=True, check=True, timeout=30
        ).stdout
        first_line = version.partition("\n")[0]
        if re.search(r"\b7\.3(?:\s|$)", first_line) is None:
            raise ValueError(f"Texinfo 7.3 required: {first_line}")
        defects: list[str] = []
        for edition in editions:
            expected = read_inventory(inventory, edition)
            tree = args.root / edition / "book"
            defects.extend(f"{edition}: {item}" for item in identity_defects(tree, expected))
            defects.extend(
                f"{edition}: {item}" for item in check_references(tree, texi2any, expected)
            )
        for defect in defects:
            print(defect, file=sys.stderr)
        print(f"editions={len(editions)} structural_defects={len(defects)}")
    except (OSError, ValueError, subprocess.SubprocessError) as exc:
        print(str(exc), file=sys.stderr)
        return 1
    return int(bool(defects))


if __name__ == "__main__":
    raise SystemExit(main())
