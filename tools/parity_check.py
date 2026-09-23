# SPDX-License-Identifier: MIT
"""Check lossless text splitting, structural parity, and Texinfo references."""

import argparse
import os
import re
import subprocess
import sys
import tempfile
from collections.abc import Sequence
from pathlib import Path

from split_texi import compare, join
from texi_indexes import anchors, source_files


def image_ref(ref: str) -> str:
    """Reduce an @image path to its figure identity across source layouts."""
    ref = re.sub(r"\.std$", "", ref)
    ref = ref.rsplit("figures/", 1)[-1]
    return ref.removeprefix("fig/").removeprefix("pdf/")


def counts(text: str) -> dict[str, int]:
    """Measure source constructs before macro expansion."""
    return {
        "exercise_anchors": len(re.findall(r"@anchor\{Exercise \d+\.\d+[a-z]?\}", text)),
        "figure_anchors": len(re.findall(r"@anchor\{Figure \d+\.\d+\}", text)),
        "floats": len(re.findall(r"^@float(?:\s|$)", text, re.MULTILINE)),
        "displaymath": len(re.findall(r"^@displaymath(?:\s|$)", text, re.MULTILINE)),
        "raw_tex": len(re.findall(r"^@tex$", text, re.MULTILINE)),
        "image_refs": len({image_ref(ref) for ref in re.findall(r"@image\{([^,}]+)", text)}),
        "footnotes": text.count("@footnote{"),
        "index_entries": len(re.findall(r"^@cindex\b", text, re.MULTILINE))
        + text.count("@newterm{"),
    }


def undefined_references(tree: Path, texi2any: str) -> int:
    """Run the real converter; reject any non-reference conversion failure."""
    with tempfile.TemporaryDirectory(prefix="sicp-parity-") as directory:
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
    errors = result.stderr
    missing = len(
        re.findall(
            r"(?:undefined|nonexistent|does not exist).*?(?:node|reference)|"
            r"(?:node|reference).*?(?:undefined|nonexistent|does not exist)",
            errors,
            re.IGNORECASE,
        )
    )
    if errors:
        print(errors, file=sys.stderr, end="")
    if result.returncode and not missing:
        raise ValueError(f"texi2any failed with exit {result.returncode}")
    return missing


def main(argv: Sequence[str] | None = None) -> int:
    """Compare the generated split against the preserved provenance source."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--split", type=Path, default=Path("text/split"))
    parser.add_argument("--src", type=Path, default=Path("text/original/sicp-pocket.texi"))
    parser.add_argument("--figures", type=Path, default=Path("text/original/figures"))
    parser.add_argument("--tree", type=Path)
    parser.add_argument("--texi2any", default=os.environ.get("TEXI2ANY", "texi2any"))
    args = parser.parse_args(argv)
    try:
        version = subprocess.run(
            [args.texi2any, "--version"], capture_output=True, text=True, check=True, timeout=30
        ).stdout.splitlines()[0]
        if re.search(r"\b7\.3(?:\s|$)", version) is None:
            raise ValueError(f"Texinfo 7.3 required: {version}")
        original = args.src.read_bytes()
        compare(original, join(args.split))
        source = original.decode()
        parts = source_files(args.split)
        text = "\n".join(path.read_text(encoding="utf-8") for path in parts)
        expected = counts(source)
        actual = counts(text)
        expected["displaymath"] = expected["raw_tex"]
        expected["raw_tex"] = 0
        defects = [name for name in expected if expected[name] != actual[name]]
        defects.extend(
            f"{kind} anchor set"
            for kind in ("Exercise", "Figure")
            if anchors(source, kind) != anchors(text, kind)
        )
        sections = sum(bool(re.fullmatch(r"\d+\.\d+\.texi", path.name)) for path in parts)
        expected_sections = len(re.findall(r"^@node[ \t]+\d+\.\d+,", source, re.MULTILINE))
        if sections != expected_sections:
            defects.append("section_files")
        if not args.figures.is_dir():
            raise ValueError(f"Figure directory missing: {args.figures}")
        svg_assets = len(list(args.figures.rglob("*.svg")))
        undefined = undefined_references(args.tree, args.texi2any) if args.tree else 0
        report = {"section_files": sections, **actual}
        report = {
            key: report[key]
            for key in (
                "section_files",
                "exercise_anchors",
                "figure_anchors",
                "floats",
                "displaymath",
                "raw_tex",
                "image_refs",
            )
        }
        report.update(
            svg_assets=svg_assets,
            footnotes=actual["footnotes"],
            index_entries=actual["index_entries"],
            undefined_refs=undefined,
        )
        print(" ".join(f"{key}={value}" for key, value in report.items()))
        if defects:
            print(f"Parity differs: {', '.join(defects)}", file=sys.stderr)
        return int(bool(defects) or undefined != 0)
    except (OSError, ValueError, subprocess.SubprocessError) as exc:
        print(str(exc), file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
