#!/usr/bin/env python3
"""Check the exercise map against the four editions' exercise artifacts.

Every row of the seven-column exercise tables in ``docs/exercise-map.md``
must have, in every edition: the statement anchor in the book TeXinfo
source, a scaffold under ``exercises/``, a solution under ``solutions/``,
and a rationale file beside the solution. Rows classified ``prose`` need
only the rationale. A row whose tailored-addition cell holds
``chosen: <lang>`` also needs the ``N.Ma`` artifacts in that language.
Exercise identifiers found on disk that the map does not list are reported
too.

Exit codes: 0 when clean, 1 on defects, 2 on usage errors (argparse).
"""

import argparse
import re
import sys
from dataclasses import dataclass
from pathlib import Path

LANGS: tuple[str, ...] = ("rust", "ocaml", "typescript", "kotlin")
_EXERCISE_COLUMNS = 7

_SECTION_HEADING = re.compile(r"^###\s+Section\s+(\d+\.\d+)(.*)$")
_HEADING = re.compile(r"^#{1,6}\s")
_DECLARED_COUNT = re.compile(r"(\d+)\s*(?:exercises|rows)")
_EXERCISE_NUM = re.compile(r"^\d+\.\d+$")
_CHOSEN = re.compile(r"chosen:\s*(rust|ocaml|typescript|kotlin)\b")
_NUM_PARTS = re.compile(r"(\d+)([a-z]*)")
_TOKEN = re.compile(r"ex_(\d+)_(\d+)([a-z]?)(?![0-9A-Za-z])")
_TS_NAME = re.compile(r"^ex_(\d+)_(\d+)([a-z]?)(?:\.test)?\.ts$")
_KT_NAME = re.compile(r"^E(\d+)_(\d+)([a-z]?)\.kt$")
_MD_NAME = re.compile(r"^ex_(\d+)_(\d+)([a-z]?)\.md$")


@dataclass(frozen=True)
class Row:
    """One exercise row of one section table."""

    section: str
    num: str
    topic: str
    addition: str

    @property
    def chosen_lang(self) -> str | None:
        """Language of a chosen tailored addition, or None.

        The pick is recorded anywhere in the addition cell, appended after
        the idea text, so the marker is searched, not anchored at the start.
        """
        match = _CHOSEN.search(self.addition)
        return match.group(1) if match else None

    @property
    def is_prose(self) -> bool:
        """Whether the exercise is prose-classified (rationale only)."""
        return self.topic.strip().lower() == "prose"


@dataclass(frozen=True)
class Section:
    """One ``### Section N.M`` heading with its declared count and rows."""

    name: str
    declared: int | None
    rows: list[Row]


def _is_exercise_header(cells: list[str]) -> bool:
    """Whether a table row is the seven-column exercise-table header.

    The second cell varies (``Topic``, ``Topic (8 words max)``); the gate is
    the first and last columns.
    """
    return (
        len(cells) == _EXERCISE_COLUMNS
        and cells[0].lower() == "exercise"
        and "addition" in cells[-1].lower()
    )


def _is_separator(cells: list[str]) -> bool:
    """Whether a table row is the ``|---|`` separator line."""
    return bool(cells) and all(re.fullmatch(r":?-+:?", cell) for cell in cells)


def parse_map(text: str) -> dict[str, Section]:
    """Parse the map into sections keyed by ``N.M``, in file order.

    Rows belong to the ``### Section N.M`` heading they fall under and are
    accepted only while a table opened by the seven-column exercise header
    is contiguous: a separator row continues the table, any other foreign
    row (a counts row, another table's header or data) ends it, so counts,
    dependency, and classification tables never leak rows into a section.
    """
    sections: dict[str, Section] = {}
    current: Section | None = None
    in_exercise_table = False
    for line in text.splitlines():
        heading = _SECTION_HEADING.match(line)
        if heading is not None:
            counts = _DECLARED_COUNT.findall(heading.group(2))
            current = Section(heading.group(1), int(counts[-1]) if counts else None, [])
            sections[current.name] = current
            in_exercise_table = False
            continue
        if _HEADING.match(line) is not None:
            current = None
            in_exercise_table = False
            continue
        if not line.lstrip().startswith("|"):
            in_exercise_table = False
            continue
        cells = [cell.strip() for cell in line.strip().strip("|").split("|")]
        if _is_exercise_header(cells):
            in_exercise_table = True
            continue
        if _is_separator(cells):
            continue
        if (
            in_exercise_table
            and current is not None
            and len(cells) == _EXERCISE_COLUMNS
            and _EXERCISE_NUM.match(cells[0]) is not None
        ):
            current.rows.append(
                Row(
                    section=current.name,
                    num=cells[0],
                    topic=cells[1],
                    addition=cells[6],
                )
            )
            continue
        in_exercise_table = False
    return sections


def _num_parts(num: str) -> tuple[str, str, str]:
    """Split ``1.2a`` into chapter ``1``, zero-padded exercise ``02``, suffix ``a``."""
    chapter, _, tail = num.partition(".")
    match = _NUM_PARTS.fullmatch(tail)
    digits, suffix = (match.group(1), match.group(2)) if match else (tail, "")
    return chapter, digits.zfill(2), suffix


def _normalize(num: str) -> str:
    """Return the normalized ``N.M[a]`` display form of ``num``."""
    chapter, digits, suffix = _num_parts(num)
    return f"{int(chapter)}.{int(digits)}{suffix}"


def _sort_key(num: str) -> tuple[int, int, str]:
    """Sort key for display numbers (``1.10a`` sorts after ``1.10``)."""
    chapter, _, tail = num.partition(".")
    match = _NUM_PARTS.fullmatch(tail)
    digits, suffix = (match.group(1), match.group(2)) if match else (tail, "")
    return int(chapter), int(digits), suffix


def _chapter_dir(lang: str, chapter: str) -> str:
    """Chapter directory name: zero-padded in Rust, plain elsewhere."""
    if lang == "rust":
        return f"ch{int(chapter):02d}"
    return f"ch{chapter}"


def _book_dir(chapter: str) -> str:
    """Book-tree chapter directory: plain ``chN`` in every edition."""
    return f"ch{int(chapter)}"


def _token(num: str) -> str:
    """Return the rust/ocaml exercise identifier for ``num`` (``ex_1_02a``)."""
    chapter, digits, suffix = _num_parts(num)
    return f"ex_{chapter}_{digits}{suffix}"


def _token_in(path: Path, token: str) -> bool:
    """Whether ``path`` names the exercise ``token`` and not a longer one."""
    try:
        text = path.read_text(encoding="utf-8", errors="replace")
    except OSError:
        return False
    return re.search(rf"\b{re.escape(token)}(?![0-9A-Za-z])", text) is not None


def _code_path(root: Path, lang: str, num: str, kind: str) -> Path:
    """Scaffold (``kind='exercises'``) or solution path for ``num``.

    TypeScript and Kotlin keep one file per exercise; Rust and OCaml group
    a section into one file that must name the exercise.
    """
    chapter, digits, suffix = _num_parts(num)
    base = root / lang / kind / _chapter_dir(lang, chapter)
    if lang == "typescript":
        return base / f"ex_{chapter}_{digits}{suffix}.ts"
    if lang == "kotlin":
        return base / f"E{chapter}_{digits}{suffix}.kt"
    extension = "rs" if lang == "rust" else "ml"
    return base / f"sec_{chapter}_{int(digits)}.{extension}"


def _scaffold_path(root: Path, lang: str, num: str) -> Path:
    """Scaffold location for ``num`` in ``lang``."""
    return _code_path(root, lang, num, "exercises")


def _solution_path(root: Path, lang: str, num: str) -> Path:
    """Solution location for ``num`` in ``lang``."""
    return _code_path(root, lang, num, "solutions")


def _statement_path(root: Path, lang: str, section: str) -> Path:
    """Section TeXinfo file holding the section's exercise statements."""
    chapter, _, digits = section.partition(".")
    book_dir = root / lang / "book" / _book_dir(chapter)
    return book_dir / f"{int(chapter)}.{int(digits)}.texi"


def _rationale_path(root: Path, lang: str, num: str) -> Path:
    """Rationale markdown beside the solution of ``num``."""
    chapter, _, _ = _num_parts(num)
    solutions_dir = root / lang / "solutions" / _chapter_dir(lang, chapter)
    return solutions_dir / f"{_token(num)}.md"


def _statement_ok(root: Path, lang: str, section: str, num: str) -> bool:
    """Whether the section file exists and anchors ``num``."""
    path = _statement_path(root, lang, section)
    if not path.is_file():
        return False
    try:
        text = path.read_text(encoding="utf-8", errors="replace")
    except OSError:
        return False
    return f"@anchor{{Exercise {num}}}" in text


def _scaffold_ok(root: Path, lang: str, num: str) -> bool:
    """Whether the scaffold exists for ``num`` in ``lang``."""
    path = _scaffold_path(root, lang, num)
    if lang in ("typescript", "kotlin"):
        return path.is_file()
    return path.is_file() and _token_in(path, _token(num))


def _solution_ok(root: Path, lang: str, num: str) -> bool:
    """Whether the solution exists for ``num`` in ``lang``."""
    path = _solution_path(root, lang, num)
    if lang in ("typescript", "kotlin"):
        return path.is_file()
    return path.is_file() and _token_in(path, _token(num))


def _missing_artifacts(root: Path, lang: str, section: str, num: str) -> list[str]:
    """Defect lines for the statement, scaffold, solution, and rationale."""
    problems: list[str] = []
    if not _statement_ok(root, lang, section, num):
        problems.append(f"{lang} {num}: missing statement")
    if not _scaffold_ok(root, lang, num):
        problems.append(f"{lang} {num}: missing scaffold")
    if not _solution_ok(root, lang, num):
        problems.append(f"{lang} {num}: missing solution")
    if not _rationale_path(root, lang, num).is_file():
        problems.append(f"{lang} {num}: missing rationale")
    return problems


def _check_row(root: Path, lang: str, row: Row, defects: list[str]) -> int:
    """Check one row in one edition, plus its addition when chosen there."""
    numbers = [row.num]
    if row.chosen_lang == lang:
        numbers.append(row.num + "a")
    checked = 0
    for num in numbers:
        checked += 1
        if row.is_prose:
            if not _rationale_path(root, lang, num).is_file():
                defects.append(f"{lang} {num}: missing rationale")
            continue
        defects.extend(_missing_artifacts(root, lang, row.section, num))
    return checked


def _check_editions(
    root: Path,
    sections: dict[str, Section],
    langs: tuple[str, ...],
    section_filter: str | None,
    defects: list[str],
) -> int:
    """Check every exercise of every edition; return the checks made."""
    checked = 0
    for lang in langs:
        for section in sections.values():
            if section_filter is not None and section.name != section_filter:
                continue
            for row in section.rows:
                checked += _check_row(root, lang, row, defects)
    return checked


def _groups_display(match: re.Match[str]) -> str:
    """Display number for a three-group identifier match."""
    chapter, exercise, suffix = match.group(1, 2, 3)
    return f"{int(chapter)}.{int(exercise)}{suffix}"


def _file_identifiers(path: Path, lang: str) -> set[str]:
    """Exercise identifiers a code-tree file contributes, if any."""
    if path.suffix == ".md":
        match = _MD_NAME.match(path.name)
        return {_groups_display(match)} if match else set()
    if lang == "typescript":
        match = _TS_NAME.match(path.name)
        return {_groups_display(match)} if match else set()
    if lang == "kotlin":
        match = _KT_NAME.match(path.name)
        return {_groups_display(match)} if match else set()
    if path.suffix not in (".rs", ".ml", ".mli"):
        return set()
    text = path.read_text(encoding="utf-8", errors="replace")
    return {_groups_display(match) for match in _TOKEN.finditer(text)}


def _dir_identifiers(top: Path, lang: str) -> set[str]:
    """Exercise identifiers under one edition's exercises or solutions tree."""
    if not top.is_dir():
        return set()
    names: set[str] = set()
    for path in sorted(top.rglob("*")):
        if path.is_file():
            names |= _file_identifiers(path, lang)
    return names


def _identifiers_on_disk(root: Path, lang: str) -> set[str]:
    """Every exercise identifier present under ``lang``'s two code trees."""
    names: set[str] = set()
    for kind in ("exercises", "solutions"):
        names |= _dir_identifiers(root / lang / kind, lang)
    return names


def _scan_unlisted(
    root: Path,
    sections: dict[str, Section],
    langs: tuple[str, ...],
    defects: list[str],
) -> None:
    """Report identifiers on disk that the map does not list for the edition."""
    for lang in langs:
        known: set[str] = set()
        for section in sections.values():
            for row in section.rows:
                known.add(_normalize(row.num))
                if row.chosen_lang == lang:
                    known.add(_normalize(row.num) + "a")
        unlisted = _identifiers_on_disk(root, lang) - known
        defects.extend(f"{lang} {num}: not in map" for num in sorted(unlisted, key=_sort_key))


def main(argv: list[str] | None = None) -> int:
    """Run the check; return the process exit code."""
    parser = argparse.ArgumentParser(
        prog="exercise_map_check",
        description=(
            "Check docs/exercise-map.md rows against the four editions' "
            "statement, scaffold, solution, and rationale artifacts."
        ),
    )
    parser.add_argument(
        "--map",
        type=Path,
        default=Path("docs/exercise-map.md"),
        help="exercise map file (default: docs/exercise-map.md)",
    )
    parser.add_argument(
        "--root",
        type=Path,
        default=None,
        help=(
            "repository root holding the edition trees "
            "(default: the map file's grandparent directory)"
        ),
    )
    parser.add_argument(
        "--lang",
        choices=LANGS,
        help="check one edition instead of all four",
    )
    parser.add_argument(
        "--section",
        metavar="N.M",
        help="check one section instead of all",
    )
    args = parser.parse_args(argv)

    map_path: Path = args.map
    if not map_path.is_file():
        print(f"exercise_map_check: map file not found: {map_path}", file=sys.stderr)
        return 1
    root = args.root if args.root is not None else map_path.resolve().parent.parent
    sections = parse_map(map_path.read_text(encoding="utf-8"))

    if args.section is not None and args.section not in sections:
        print(
            f"exercise_map_check: section {args.section} not found in {map_path}",
            file=sys.stderr,
        )
        return 1

    defects: list[str] = []
    for section in sections.values():
        if args.section is not None and section.name != args.section:
            continue
        if section.declared is None:
            defects.append(f"section {section.name}: no exercise count in heading")
        elif section.declared != len(section.rows):
            defects.append(
                f"section {section.name}: declared {section.declared} exercises, "
                f"found {len(section.rows)}"
            )
    if defects:
        for line in defects:
            print(line)
        print(f"checked=0 defects={len(defects)}")
        return 1

    langs = (args.lang,) if args.lang else LANGS
    checked = _check_editions(root, sections, langs, args.section, defects)
    _scan_unlisted(root, sections, langs, defects)
    for line in defects:
        print(line)
    print(f"checked={checked} defects={len(defects)}")
    return 1 if defects else 0


if __name__ == "__main__":
    raise SystemExit(main())
