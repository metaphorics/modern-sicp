# SPDX-License-Identifier: MIT
from pathlib import Path

from texi_indexes import main


def test_additions_sort_after_base_and_duplicates_do_not_repeat(tmp_path: Path) -> None:
    chapter = tmp_path / "ch2"
    chapter.mkdir()
    (chapter / "2.1.texi").write_text(
        "@anchor{Exercise 2.10}\n@anchor{Exercise 2.9a}\n"
        "@anchor{Exercise 2.9}\n@anchor{Exercise 2.9}\n@anchor{Figure 2.1}\n"
    )
    assert main(["--tree", str(tmp_path)]) == 0
    index = tmp_path / "back/exercises.texi"
    content = index.read_text()
    assert content.index("2.9,,") < content.index("2.9a,,") < content.index("2.10,,")
    assert content.count("@ref{Exercise 2.9,,") == 1
    assert main(["--tree", str(tmp_path), "--check"]) == 0
    index.write_text("stale")
    assert main(["--tree", str(tmp_path), "--check"]) == 1
    assert index.read_text() == "stale"


def test_missing_tree_fails_without_creating_it(tmp_path: Path) -> None:
    absent = tmp_path / "absent"
    assert main(["--tree", str(absent)]) == 1
    assert not absent.exists()
