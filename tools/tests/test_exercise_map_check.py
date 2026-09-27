"""Tests for tools/exercise_map_check.py."""

import importlib.util
import re
import sys
from pathlib import Path

import pytest

_TOOLS_DIR = Path(__file__).resolve().parents[1]
_SPEC = importlib.util.spec_from_file_location(
    "exercise_map_check", _TOOLS_DIR / "exercise_map_check.py"
)
if _SPEC is None or _SPEC.loader is None:
    raise ImportError(f"cannot load {_TOOLS_DIR / 'exercise_map_check.py'}")
exercise_map_check = importlib.util.module_from_spec(_SPEC)
sys.modules.setdefault("exercise_map_check", exercise_map_check)
_SPEC.loader.exec_module(exercise_map_check)

main = exercise_map_check.main
parse_map = exercise_map_check.parse_map

LANGS = ("rust", "ocaml", "typescript", "kotlin")
HEADER = "| Exercise | Topic | Rust | OCaml | TypeScript | Kotlin | Tailored addition idea |"
HEADER_CH5 = (
    "| Exercise | Topic (8 words max) | Rust | OCaml | TypeScript | Kotlin | "
    "Tailored addition idea |"
)
SEPARATOR = "|---|---|---|---|---|---|---|"
_NUM_TAIL = re.compile(r"(\d+)([a-z]*)")


def map_path(tmp_path: Path) -> Path:
    """Default map location: <tmp>/docs/exercise-map.md."""
    return tmp_path / "docs" / "exercise-map.md"


def split_num(num: str) -> tuple[str, str, str]:
    """Chapter, zero-padded exercise digits, and lowercase suffix."""
    chapter, _, tail = num.partition(".")
    match = _NUM_TAIL.fullmatch(tail)
    if match is None:
        raise ValueError(f"bad exercise number: {num}")
    return chapter, match.group(1).zfill(2), match.group(2)


def chapter_dir(lang: str, chapter: str) -> str:
    """Rust chapter directories are zero-padded; the others are plain."""
    return f"ch{int(chapter):02d}" if lang == "rust" else f"ch{chapter}"


def exercise_paths(
    root: Path, lang: str, num: str, section: str | None = None
) -> tuple[Path, Path, Path, Path]:
    """Scaffold, solution, rationale, and statement paths for one exercise."""
    chapter, digits, suffix = split_num(num)
    token = f"ex_{chapter}_{digits}{suffix}"
    ch = chapter_dir(lang, chapter)
    base = root / lang
    sec = section if section is not None else f"{chapter}.{int(digits)}"
    schapter, _, sdigits = sec.partition(".")
    statement = base / "book" / f"ch{int(schapter)}" / f"{int(schapter)}.{int(sdigits)}.texi"
    if lang == "typescript":
        scaffold = base / "exercises" / ch / f"{token}.ts"
        solution = base / "solutions" / ch / f"{token}.ts"
    elif lang == "kotlin":
        scaffold = base / "exercises" / ch / f"E{chapter}_{digits}{suffix}.kt"
        solution = base / "solutions" / ch / f"E{chapter}_{digits}{suffix}.kt"
    else:
        extension = "rs" if lang == "rust" else "ml"
        section_file = f"sec_{chapter}_{int(digits)}.{extension}"
        scaffold = base / "exercises" / ch / section_file
        solution = base / "solutions" / ch / section_file
    rationale = base / "solutions" / ch / f"{token}.md"
    return scaffold, solution, rationale, statement


def add_exercise(root: Path, lang: str, num: str, section: str | None = None) -> None:
    """Create the four artifacts of one exercise; safe to call repeatedly."""
    chapter, digits, suffix = split_num(num)
    token = f"ex_{chapter}_{digits}{suffix}"
    scaffold, solution, rationale, statement = exercise_paths(root, lang, num, section)
    for path in (scaffold, solution, rationale, statement):
        path.parent.mkdir(parents=True, exist_ok=True)
    with statement.open("a", encoding="utf-8") as handle:
        plain = f"{chapter}.{int(digits)}{suffix}"
        handle.write(f"@anchor{{Exercise {plain}}} Exercise {plain}: topic.\n")
    if lang == "rust":
        body = f"#[test]\nfn {token}() {{}}\n"
    elif lang == "ocaml":
        body = f"let {token} = ()\n"
    else:
        body = f"export const {token} = () => null;\n"
    for path in (scaffold, solution):
        with path.open("a", encoding="utf-8") as handle:
            handle.write(body)
    rationale.write_text(f"# Exercise {num}\nRationale.\n", encoding="utf-8")


def write_map(
    tmp_path: Path,
    sections: list[tuple[str, int | None, list[tuple[str, str, str]]]],
    header: str = HEADER,
) -> Path:
    """Write a synthetic map with one table per section."""
    lines: list[str] = ["# Exercise map", ""]
    for name, declared, rows in sections:
        count = f", {declared} exercises" if declared is not None else ""
        lines += [
            f"### Section {name} (lines 1 to 99{count})",
            "",
            header,
            SEPARATOR,
        ]
        lines += [
            f"| {num} | {topic} | T | T | T | T | {addition} |" for num, topic, addition in rows
        ]
        lines.append("")
    path = map_path(tmp_path)
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text("\n".join(lines) + "\n", encoding="utf-8")
    return path


def run(tmp_path: Path, *extra: str) -> int:
    """Run the check against the synthetic map."""
    return main(["--map", str(map_path(tmp_path)), *extra])


def build_all(tmp_path: Path, *numbers: str, root: Path | None = None) -> None:
    """Build every artifact of the given exercises in all four editions."""
    target = tmp_path if root is None else root
    sections = parse_map(map_path(tmp_path).read_text(encoding="utf-8"))
    section_of = {row.num: name for name, sec in sections.items() for row in sec.rows}
    for lang in LANGS:
        for num in numbers:
            add_exercise(target, lang, num, section_of.get(num))
            if num.endswith("a"):
                continue


def test_help(capsys: pytest.CaptureFixture[str]) -> None:
    with pytest.raises(SystemExit) as excinfo:
        main(["--help"])
    assert excinfo.value.code == 0
    assert "usage" in capsys.readouterr().out


def test_clean_tree(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    write_map(
        tmp_path,
        [("1.1", 2, [("1.1", "evaluate in order", ""), ("1.2", "prefix", "")])],
    )
    build_all(tmp_path, "1.1", "1.2")
    code = run(tmp_path)
    out = capsys.readouterr().out
    assert code == 0
    assert "checked=8 defects=0" in out


def test_missing_artifacts(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    write_map(tmp_path, [("1.1", 1, [("1.1", "topic", "")])])
    build_all(tmp_path, "1.1")
    exercise_paths(tmp_path, "rust", "1.1")[3].unlink()
    exercise_paths(tmp_path, "ocaml", "1.1")[0].unlink()
    exercise_paths(tmp_path, "typescript", "1.1")[1].unlink()
    exercise_paths(tmp_path, "kotlin", "1.1")[2].unlink()
    code = run(tmp_path)
    out = capsys.readouterr().out
    assert code == 1
    assert "rust 1.1: missing statement" in out
    assert "ocaml 1.1: missing scaffold" in out
    assert "typescript 1.1: missing solution" in out
    assert "kotlin 1.1: missing rationale" in out
    assert "defects=4" in out


def test_statement_requires_anchor(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    write_map(tmp_path, [("1.1", 1, [("1.1", "topic", "")])])
    build_all(tmp_path, "1.1")
    exercise_paths(tmp_path, "rust", "1.1")[3].write_text("Exercise 1.1: no anchor.\n")
    code = run(tmp_path, "--lang", "rust")
    out = capsys.readouterr().out
    assert code == 1
    assert "rust 1.1: missing statement" in out
    assert "missing scaffold" not in out
    assert "missing solution" not in out
    assert "missing rationale" not in out


def test_prose_row_needs_only_rationale(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    write_map(tmp_path, [("1.1", 1, [("1.1", "prose", "")])])
    build_all(tmp_path, "1.1")
    code = run(tmp_path)
    out = capsys.readouterr().out
    assert code == 0
    assert "checked=4 defects=0" in out


def test_prose_row_without_rationale(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    write_map(tmp_path, [("1.1", 1, [("1.1", "prose", "")])])
    code = run(tmp_path)
    out = capsys.readouterr().out
    assert code == 1
    assert out.count(": missing rationale") == 4
    assert "missing statement" not in out
    assert "missing scaffold" not in out
    assert "missing solution" not in out


def test_count_mismatch(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    write_map(tmp_path, [("1.1", 3, [("1.1", "topic", ""), ("1.2", "topic", "")])])
    code = run(tmp_path)
    out = capsys.readouterr().out
    assert code == 1
    assert "section 1.1: declared 3 exercises, found 2" in out
    assert "defects=1" in out


def test_heading_without_count(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    write_map(tmp_path, [("1.1", None, [("1.1", "topic", "")])])
    code = run(tmp_path)
    out = capsys.readouterr().out
    assert code == 1
    assert "section 1.1: no exercise count in heading" in out


def test_chosen_addition(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    rows = [
        ("1.1", "topic", ""),
        ("1.2", "topic", "1.2a: compare versions; chosen: rust"),
    ]
    write_map(tmp_path, [("1.1", 2, rows)])
    build_all(tmp_path, "1.1", "1.2")
    add_exercise(tmp_path, "rust", "1.2a", "1.1")
    code = run(tmp_path)
    out = capsys.readouterr().out
    assert code == 0
    assert "checked=9 defects=0" in out
    exercise_paths(tmp_path, "rust", "1.2a")[2].unlink()
    code = run(tmp_path)
    out = capsys.readouterr().out
    assert code == 1
    assert "rust 1.2a: missing rationale" in out
    assert "typescript 1.2a" not in out


def test_not_in_map(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    write_map(tmp_path, [("1.1", 1, [("1.1", "topic", "")])])
    build_all(tmp_path, "1.1")
    ts_scaffold = exercise_paths(tmp_path, "typescript", "1.1")[0]
    (ts_scaffold.parent / "ex_1_09.ts").write_text("export const stray = 1;\n")
    (tmp_path / "typescript/solutions/ch1/ex_9_99.md").write_text(
        "Stray rationale.\n", encoding="utf-8"
    )
    rust_scaffold = exercise_paths(tmp_path, "rust", "1.1")[0]
    with rust_scaffold.open("a", encoding="utf-8") as handle:
        handle.write("fn ex_2_17() {}\n")
    code = run(tmp_path)
    out = capsys.readouterr().out
    assert code == 1
    assert "typescript 1.9: not in map" in out
    assert "typescript 9.99: not in map" in out
    assert "rust 2.17: not in map" in out


def test_lang_filter(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    write_map(tmp_path, [("1.1", 2, [("1.1", "topic", ""), ("1.2", "topic", "")])])
    build_all(tmp_path, "1.1", "1.2")
    exercise_paths(tmp_path, "kotlin", "1.1")[0].unlink()
    code = run(tmp_path, "--lang", "typescript")
    out = capsys.readouterr().out
    assert code == 0
    assert "checked=2 defects=0" in out
    code = run(tmp_path, "--lang", "kotlin")
    out = capsys.readouterr().out
    assert code == 1
    assert "kotlin 1.1: missing scaffold" in out
    assert "checked=2 defects=1" in out


def test_section_filter(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    write_map(
        tmp_path,
        [
            ("1.1", 3, [("1.1", "topic", ""), ("1.2", "topic", "")]),
            ("1.2", 1, [("1.9", "topic", "")]),
        ],
    )
    build_all(tmp_path, "1.1", "1.2", "1.9")
    code = run(tmp_path, "--section", "1.2")
    out = capsys.readouterr().out
    assert code == 0
    assert "checked=4 defects=0" in out
    code = run(tmp_path, "--section", "1.1")
    out = capsys.readouterr().out
    assert code == 1
    assert "section 1.1: declared 3 exercises, found 2" in out
    code = run(tmp_path)
    out = capsys.readouterr().out
    assert code == 1
    assert "section 1.1: declared 3 exercises, found 2" in out
    assert "missing statement" not in out


def test_root_override(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    write_map(tmp_path, [("1.1", 1, [("1.1", "topic", "")])])
    tree = tmp_path / "tree"
    build_all(tmp_path, "1.1", root=tree)
    code = main(["--map", str(map_path(tmp_path)), "--root", str(tree)])
    out = capsys.readouterr().out
    assert code == 0
    assert "checked=4 defects=0" in out
    code = run(tmp_path)
    assert code == 1


def test_header_gates_other_tables(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    lines = [
        "# Exercise map",
        "",
        "### Section 5.1 (lines 1 to 9, 2 exercises)",
        "",
        "Some prose note between the heading and the table.",
        "",
        HEADER_CH5,
        SEPARATOR,
        "| 5.1 | design machine | T | T | T | T | |",
        "| 5.2 | write iterative factorial | T | T | T | T | |",
        "| Counts, summing to 2 per language: all T |",
        "",
        "| Section | Needs |",
        "|---|---|",
        "| 5.2 | 5.1 |",
        "| 5.9 | rogue row | T | T | T | T | |",
        "",
        "| 5.8 | rogue row after blank line | T | T | T | T | |",
        "",
    ]
    docs = tmp_path / "docs"
    docs.mkdir(parents=True)
    (docs / "exercise-map.md").write_text("\n".join(lines) + "\n", encoding="utf-8")
    build_all(tmp_path, "5.1", "5.2")
    code = run(tmp_path)
    out = capsys.readouterr().out
    assert code == 0
    assert "checked=8 defects=0" in out
    assert "5.9" not in out
    assert "5.8" not in out


def test_missing_map_file(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    code = main(["--map", str(tmp_path / "nope.md")])
    assert code == 1
    assert "not found" in capsys.readouterr().err


def test_unknown_section(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    write_map(tmp_path, [("1.1", 1, [("1.1", "topic", "")])])
    code = run(tmp_path, "--section", "9.9")
    assert code == 1
    assert "section 9.9 not found" in capsys.readouterr().err
