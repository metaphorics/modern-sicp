# SPDX-License-Identifier: MIT
"""Cap overflowing figure widths in edition books, with snapshot guardrails.

A height-only ``@image{..., ,Hmm,...}`` scales wide figures past the
measured 433.62pt text width (TS main.log: up to 2279pt boxes). For each
such line whose natural aspect times the specified height exceeds the
text width, rewrite to width-only 147mm (418.3pt, eval-verified in TeX
points), which always fits and preserves aspect. Portrait height-specs
are never touched: pdfTeX would scale both dims disproportionately.

Guardrails (shared tree, workers may be editing other lines in the
same files): snapshot every touched file; skip files whose mtime moved
between snapshot and write; after patching, report any file whose git
diff covers non-@image lines (snapshots are never auto-restored).
Limit: the 147mm cap assumes full text width; floats nested inside
indented environments (e.g. @quotation) need per-site smaller widths
such as the Fig3.33a 120mm fix. `--check` reports, `--apply` patches.
"""

import argparse
import pathlib
import re
import shutil
import subprocess
import sys
import tempfile

TEXT_WIDTH_PT = 433.62
TARGET_MM = "147mm"
TE_PT_PER_MM = 72.27 / 25.4


def natural_sizes(pdfdir: pathlib.Path) -> dict[str, tuple[float, float]]:
    """Map sized-PDF keys to (width, height) points via pdfinfo."""
    sizes: dict[str, tuple[float, float]] = {}
    for pdf in sorted(pdfdir.rglob("*.pdf")):
        try:
            out = subprocess.run(
                ["pdfinfo", str(pdf)], capture_output=True, text=True, timeout=30, check=False
            ).stdout
        except OSError, subprocess.SubprocessError:
            continue
        match = re.search(r"Page size:\s+([\d.]+) x ([\d.]+)", out)
        if match:
            key = pdf.relative_to(pdfdir).with_suffix("").as_posix()
            sizes[key] = (float(match.group(1)), float(match.group(2)))
    return sizes


def pdf_key(name: str, sizes: dict[str, tuple[float, float]]) -> str | None:
    """Resolve an @image file field to a sized-PDF key."""
    if "figures/pdf/" in name:
        key = name.rsplit("figures/pdf/", maxsplit=1)[-1]
        return key if key in sizes else None
    base = name.rsplit("figures/", maxsplit=1)[-1].rsplit(".", 2)[0]
    for key in sizes:
        if key.endswith("/" + base) or key == base:
            return key
    return None


def overflowing_targets(
    book: pathlib.Path, sizes: dict[str, tuple[float, float]]
) -> list[tuple[pathlib.Path, int, str, str]]:
    """Return (file, lineno, key, line) for height-only pdf lines that overflow."""
    found = []
    for path in sorted(book.rglob("*.texi")):
        for lineno, line in enumerate(path.read_text().splitlines(), 1):
            for match in re.finditer(r"@image\{([^,}]+),([^,]*),([^,]*),", line):
                name, width, height = (
                    match.group(1),
                    match.group(2).strip(),
                    match.group(3).strip(),
                )
                if width:
                    continue
                key = pdf_key(name, sizes)
                if key is None or key not in sizes:
                    continue
                natural_w, natural_h = sizes[key]
                if natural_h <= 0:
                    continue
                height_match = re.match(r"([\d.]+)mm$", height)
                if height_match:
                    scaled = float(height_match.group(1)) * TE_PT_PER_MM * (natural_w / natural_h)
                    if scaled <= TEXT_WIDTH_PT:
                        continue
                elif height == "" and natural_w > TEXT_WIDTH_PT:
                    pass
                else:
                    continue
                found.append((path, lineno, key, line))
    return found


def unresolvable_lines(
    root: pathlib.Path, editions: list[str], sizes: dict[str, tuple[float, float]]
) -> list[str]:
    """List height-only or bare @image lines with no resolvable figure size."""
    missing = []
    for edition in editions:
        book = root / edition / "book"
        for path in sorted(book.rglob("*.texi")):
            for lineno, line in enumerate(path.read_text().splitlines(), 1):
                for match in re.finditer(r"@image\{([^,}]+),([^,]*),([^,]*),", line):
                    name = match.group(1)
                    if match.group(2).strip():
                        continue
                    height = match.group(3).strip()
                    if not re.match(r"([\d.]+)mm$", height) and height != "":
                        continue
                    if pdf_key(name, sizes) is None:
                        missing.append(f"{path.relative_to(root).as_posix()}:{lineno} {name}")
    return missing


def sibling_svg_line(lines: list[str], lineno: int, key: str) -> int | None:
    """Find the paired height-only svg @image line near a pdf line."""
    base = key.rsplit(".", 1)[0]
    for delta in range(11):
        for candidate in (lineno - 1 - delta, lineno - 1 + delta):
            if 0 <= candidate < len(lines) and "figures/" in lines[candidate]:
                text = lines[candidate]
                if base in text and re.search(r"@image\{[^,}]+,,[\d.]+mm,", text):
                    return candidate + 1
    return None


def patch_line(line: str) -> str | None:
    """Swap a height-only or bare spec to width-only TARGET_MM, else None."""
    patched, count = re.subn(
        r"(@image\{[^,}]+),,([\d.]+mm),",
        r"\1," + TARGET_MM + ",,",
        line,
        count=1,
    )
    if count == 1:
        return patched
    patched, count = re.subn(
        r"(@image\{[^,}]+),,,",
        r"\1," + TARGET_MM + ",,",
        line,
        count=1,
    )
    return patched if count == 1 else None


def collect_edits(
    root: pathlib.Path, editions: list[str], sizes: dict[str, tuple[float, float]]
) -> list[tuple[pathlib.Path, dict[int, str], str]]:
    """Pair each overflowing pdf line with its svg sibling edit."""
    work = []
    for edition in editions:
        book = root / edition / "book"
        for path, lineno, key, _ in overflowing_targets(book, sizes):
            lines = path.read_text().splitlines()
            replacement = patch_line(lines[lineno - 1])
            if replacement is None:
                continue
            edits = {lineno: replacement}
            svg_lineno = sibling_svg_line(lines, lineno, key)
            if svg_lineno is not None and svg_lineno not in edits:
                svg_replacement = patch_line(lines[svg_lineno - 1])
                if svg_replacement is not None:
                    edits[svg_lineno] = svg_replacement
            work.append((path, edits, key))
    return work


def apply_work(
    root: pathlib.Path,
    work: list[tuple[pathlib.Path, dict[int, str], str]],
    snapdir: pathlib.Path,
    *,
    dry: bool,
) -> tuple[list[str], list[str]]:
    """Write edits behind mtime checks; report applied and deferred."""
    applied: list[str] = []
    deferred: list[str] = []
    for path, edits, key in work:
        rel = path.relative_to(root).as_posix()
        if dry:
            for target in sorted(edits):
                print(f"would-patch {rel}:{target} {key}")
            continue
        snapshot = snapdir / rel.replace("/", "_")
        if not snapshot.exists():
            shutil.copy2(path, snapshot)
        before_mtime = path.stat().st_mtime_ns
        current = path.read_text().splitlines()
        if path.stat().st_mtime_ns != before_mtime:
            deferred.append(f"{rel} moved-underfoot")
            continue
        if any(
            not re.search(r"@image\{[^,}]+,,([\d.]+mm)?,", current[target - 1]) for target in edits
        ):
            deferred.append(f"{rel} already-fixed")
            continue
        for target, text in edits.items():
            current[target - 1] = text
        path.write_text("\n".join(current) + "\n")
        applied.append(f"{rel}:{sorted(edits)} {key}")
    return applied, deferred


def verify_scope(root: pathlib.Path, applied: list[str], snapdir: pathlib.Path) -> None:
    """Report any touched file whose git diff covers non-@image lines."""
    if not applied:
        return
    dirty = subprocess.run(
        ["git", "--no-pager", "diff", "--unified=0", "--", "*/book/ch*/*.texi"],
        capture_output=True,
        text=True,
        check=False,
        cwd=root,
    ).stdout
    touched: dict[str, list[str]] = {}
    current_file = ""
    for line in dirty.splitlines():
        if line.startswith("+++ b/"):
            current_file = line[6:]
        elif line.startswith(("+", "-")) and not line.startswith(("+++", "---")):
            touched.setdefault(current_file, []).append(line)
    for rel, changes in touched.items():
        if changes and all("@image{" in change for change in changes if change not in ("+", "-")):
            continue
        print(f"scope-violation {rel} (hand-check; snapshots kept,")
        print(f"  never auto-restored: {snapdir})")


def main(argv: list[str] | None = None) -> int:
    """Report (--check default) or apply width caps to overflowing figures."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=pathlib.Path, required=True)
    parser.add_argument("--apply", action="store_true")
    parser.add_argument("--editions", default="rust,ocaml,typescript,kotlin")
    args = parser.parse_args(argv)
    sizes = natural_sizes(args.root / "text/original/figures/pdf")
    if not sizes:
        print("no figure sizes resolvable", file=sys.stderr)
        return 1
    work = collect_edits(args.root, args.editions.split(","), sizes)
    if not args.apply:
        missing = unresolvable_lines(args.root, args.editions.split(","), sizes)
        print(f"targets={len(work)} unresolvable={len(missing)}")
        for item in missing:
            print(f"unresolvable {item}")
    snapdir = pathlib.Path(tempfile.mkdtemp(prefix="figsweep-"))
    applied, deferred = apply_work(args.root, work, snapdir, dry=not args.apply)
    if args.apply:
        print(f"applied={len(applied)} deferred={len(deferred)}")
        for item in deferred:
            print(f"deferred {item}")
        verify_scope(args.root, applied, snapdir)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
