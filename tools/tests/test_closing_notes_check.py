# SPDX-License-Identifier: MIT
from pathlib import Path
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    import pytest

from closing_notes_check import main

NOTE_SECTIONS = ("ch2/2.4.texi", "ch2/2.5.texi", "ch3/3.3.texi", "ch4/4.1.texi")
NOTE = "@strong{In this language: first-class procedures.}"


def write_section(tree: Path, relative: str, body: str) -> None:
    path = tree / relative
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(body)


def good_section() -> str:
    return f"@section 2.4\n@quotation\n{NOTE}\n@end quotation\n"


def test_all_present_passes(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    tree = tmp_path
    for relative in NOTE_SECTIONS:
        write_section(tree, relative, good_section())
    # Blank lines between the quotation and its first line still count.
    write_section(tree, "ch4/4.1.texi", f"@quotation\n\n{NOTE}\n@end quotation\n")
    assert main(["--tree", str(tree)]) == 0
    assert capsys.readouterr().out == "notes=4 missing=0\n"


def test_one_missing_fails(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    tree = tmp_path
    for relative in NOTE_SECTIONS:
        write_section(tree, relative, good_section())
    write_section(tree, "ch2/2.5.texi", "@section 2.5\nNo note here.\n")
    assert main(["--tree", str(tree)]) == 1
    captured = capsys.readouterr()
    assert "ch2/2.5.texi: missing closing note" in captured.err
    assert captured.out == "notes=4 missing=1\n"


def test_quotation_with_wrong_first_line_fails(
    tmp_path: Path, capsys: pytest.CaptureFixture[str]
) -> None:
    tree = tmp_path
    for relative in NOTE_SECTIONS:
        write_section(tree, relative, good_section())
    # The note text appears only as a later line inside an exercise quotation.
    write_section(
        tree,
        "ch3/3.3.texi",
        "@quotation\n@strong{Exercise 3.12:}\n\n"
        f"{NOTE}\n@end quotation\n\n@quotation\n@strong{{Something else}}\n@end quotation\n",
    )
    assert main(["--tree", str(tree)]) == 1
    captured = capsys.readouterr()
    assert "ch3/3.3.texi: missing closing note" in captured.err
    assert captured.out == "notes=4 missing=1\n"


def test_absent_file_counts_as_missing(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    tree = tmp_path
    write_section(tree, "ch2/2.4.texi", good_section())
    assert main(["--tree", str(tree)]) == 1
    captured = capsys.readouterr()
    assert captured.out == "notes=4 missing=3\n"
    assert "ch4/4.1.texi: missing closing note" in captured.err
