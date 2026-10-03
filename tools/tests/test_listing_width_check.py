# SPDX-License-Identifier: MIT
import shutil
from pathlib import Path

import pytest

from listing_width_check import expand, main

pytestmark = pytest.mark.skipif(shutil.which("makeinfo") is None, reason="needs makeinfo")

LISTING = "const tooWide = (alpha: number, beta: number): number => alpha * beta;"
PROSE = "A paragraph whose inline code cannot break."
AFTER = "The line right after the listing is prose again."
CHAPTER = f"""@node Top
@top Book

@example
{LISTING}
@end example
{AFTER}

{PROSE}
"""


@pytest.fixture
def book(tmp_path: Path) -> Path:
    root = tmp_path / "typescript" / "book"
    (root / "ch1").mkdir(parents=True)
    (root / "main.texi").write_text("\\input texinfo\n@include ch1/1.1.texi\n@bye\n")
    (root / "ch1" / "1.1.texi").write_text(CHAPTER)
    return root


def overfull(book: Path, text: str, width: float) -> str:
    line = expand(book, "makeinfo").index(text) + 1
    return f"Overfull \\hbox ({width}pt too wide) in paragraph at lines {line}--{line}\n"


def test_listing_overflow_is_reported_at_its_source(
    book: Path, capsys: pytest.CaptureFixture[str]
) -> None:
    log = overfull(book, LISTING, 32.5) + overfull(book, PROSE, 40.0) + overfull(book, AFTER, 9.0)
    (book / "main.log").write_text(log)
    assert main(["--root", str(book.parents[1]), "--edition", "typescript"]) == 1
    out = capsys.readouterr().out
    assert f"typescript: ch1/1.1.texi:5: 32.5pt too wide: {LISTING}" in out
    assert "typescript: listing overflows=1" in out


def test_prose_and_sub_tolerance_overflows_pass(
    book: Path, capsys: pytest.CaptureFixture[str]
) -> None:
    log = overfull(book, LISTING, 0.8) + overfull(book, PROSE, 40.0) + overfull(book, AFTER, 9.0)
    (book / "main.log").write_text(log)
    assert main(["--root", str(book.parents[1]), "--edition", "typescript"]) == 0
    assert "typescript: listing overflows=0" in capsys.readouterr().out
