# SPDX-License-Identifier: MIT
"""Convert SVG figures to PDF, skipping outputs already newer than their sources."""

import argparse
import os
import shutil
import subprocess
import sys
from collections.abc import Sequence
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

DEFAULT_BINARY = "rsvg-convert"


def convert_pairs(src: Path, out: Path) -> list[tuple[Path, Path]]:
    """Pair every SVG under src with its mirrored PDF path under out."""
    return [
        (svg, out / svg.relative_to(src).with_suffix(".pdf")) for svg in sorted(src.rglob("*.svg"))
    ]


def is_current(svg: Path, pdf: Path) -> bool:
    """Say whether the PDF exists and is not older than its SVG."""
    return pdf.is_file() and pdf.stat().st_mtime >= svg.stat().st_mtime


def run_convert(binary: str, svg: Path, pdf: Path) -> None:
    """Convert one SVG, creating the output directory first."""
    pdf.parent.mkdir(parents=True, exist_ok=True)
    result = subprocess.run(
        [binary, "-f", "pdf", "-o", str(pdf), str(svg)],
        capture_output=True,
        text=True,
        check=False,
    )
    if result.returncode != 0:
        raise ValueError(f"{binary} failed on {svg}: {result.stderr.strip()}")


def main(argv: Sequence[str] | None = None) -> int:
    """Convert pending SVGs, or with --check report missing and stale PDFs without converting."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--src", type=Path, required=True, help="directory of SVG figures")
    parser.add_argument("--out", type=Path, required=True, help="directory receiving PDF figures")
    parser.add_argument(
        "--check", action="store_true", help="list missing and stale PDFs, convert nothing"
    )
    parser.add_argument("--jobs", type=int, default=None, help="parallel conversions")
    args = parser.parse_args(argv)
    try:
        binary = os.environ.get("RSVG_CONVERT", DEFAULT_BINARY)
        if shutil.which(binary) is None:
            print(f"{binary}: converter not found on PATH", file=sys.stderr)
            return 1
        if not args.src.is_dir():
            raise ValueError(f"No figure directory at {args.src}")
        current: list[Path] = []
        pending: list[tuple[Path, Path]] = []
        for svg, pdf in convert_pairs(args.src, args.out):
            if is_current(svg, pdf):
                current.append(pdf)
            else:
                pending.append((svg, pdf))
        if args.check:
            for _, pdf in pending:
                state = "stale" if pdf.exists() else "missing"
                print(f"{pdf}: {state} pdf", file=sys.stderr)
            print(f"converted=0 skipped={len(current)} stale={len(pending)}")
            return int(bool(pending))
        workers = max(1, args.jobs or os.process_cpu_count() or 1)

        def convert(pair: tuple[Path, Path]) -> None:
            run_convert(binary, pair[0], pair[1])

        with ThreadPoolExecutor(max_workers=workers) as executor:
            list(executor.map(convert, pending))
        print(f"converted={len(pending)} skipped={len(current)} stale=0")
    except (OSError, ValueError) as exc:
        print(str(exc), file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
