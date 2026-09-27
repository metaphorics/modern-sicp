# SPDX-License-Identifier: MIT
from pathlib import Path

import pytest

from split_texi import join, main, plain_alt

SOURCE = r"""\input texinfo
@node Top
@top Test
Unicode: λ and é.
@node 1.1, , , Top
@section First
@anchor{Exercise 1.1}
@footnote{Keep this.}
@float
@anchor{Figure 1.1}
@ifinfo
ASCII art.
@end ifinfo
@iftex
@image{fig/chap1/Fig1.1,,20mm,,.std.svg}
@caption{@strong{Figure 1.1:} A @code{nested} caption, with a comma.}
@end iftex
@end float
@ifinfo
ASCII equation.
@end ifinfo
@tex
\[
x^2
\]
@end tex
@node 1.2, , , Top
@section Second
@anchor{Exercise 1.2}
@node References, , , Top
@unnumbered References
@bye
"""


def test_split_join_preserves_bytes_and_renders_figures(tmp_path: Path) -> None:
    src = tmp_path / "source.texi"
    src.write_bytes(SOURCE.encode())
    out = tmp_path / "split"
    assert main(["--src", str(src), "--out", str(out)]) == 0
    assert join(out) == src.read_bytes()
    section = (out / "ch1/1.1.texi").read_text()
    assert "@displaymath\nx^2\n@end displaymath\n@ifinfo" in section
    assert "@tex\n" not in section
    assert "@ifhtml\n@image{" in section
    assert "Figure 1.1: A nested caption@comma{} with a comma." in section
    assert section.count("@caption{") == 1


def test_plain_alt_reduces_caption_markup_to_text() -> None:
    """The @image alt carries no markup: elements inside attributes break XML."""
    assert (
        plain_alt("set @math{{\\{1@comma{} 3\\}}} seen in @ref{Figure 2.5}")
        == "set @{1@comma{} 3@} seen in Figure 2.5"
    )
    assert plain_alt("a @code{car} cell") == "a car cell"


TWO_EDITS = r"""\input texinfo
@node Top
@top Test
@node 1.1, , , Top
@section First
@tex
\[
a
\]
@end tex
Between the two blocks.
@tex
\[
a_{1} + a_{2} + a_{3}
\]
@end tex
@node References, , , Top
@unnumbered References
@bye
"""


def test_join_restores_two_unequal_replacements_in_one_part(tmp_path: Path) -> None:
    src = tmp_path / "source.texi"
    src.write_bytes(TWO_EDITS.encode())
    out = tmp_path / "split"
    assert main(["--src", str(src), "--out", str(out)]) == 0
    section = (out / "ch1/1.1.texi").read_text()
    assert section.count("@displaymath") == 2
    assert join(out) == src.read_bytes()


PLAIN_TEX = r"""\input texinfo
@node Top
@top Test
@node 1.1, , , Top
@section First
Nested: @math{\frac{5 + \frac{4}{5}}{3}} and text @math{\text{Fib}(n)}.
@displaymath
\frac{2\cdot 4}
     {3\cdot 3}
@end displaymath
@node References, , , Top
@unnumbered References
@bye
"""


def test_plain_tex_rewrites_nested_and_split_fractions(tmp_path: Path) -> None:
    src = tmp_path / "source.texi"
    src.write_bytes(PLAIN_TEX.encode())
    out = tmp_path / "split"
    assert main(["--src", str(src), "--out", str(out)]) == 0
    section = (out / "ch1/1.1.texi").read_text()
    # Plain TeX defines none of these; leaving one behind aborts the TeX math run.
    assert "\\frac" not in section
    assert "\\text{" not in section
    assert "{5 + {4 \\over 5} \\over 3}" in section
    assert "\\hbox{Fib}(n)" in section
    # The two arguments of the display fraction are separated by a line break.
    assert "{2\\cdot 4 \\over 3\\cdot 3}" in section
    assert join(out) == src.read_bytes()


def test_check_rejects_modified_split_without_overwriting(tmp_path: Path) -> None:
    src = tmp_path / "source.texi"
    src.write_text(SOURCE)
    out = tmp_path / "split"
    args = ["--src", str(src), "--out", str(out)]
    assert main(args) == 0
    part = out / "ch1/1.2.texi"
    changed = part.read_text().replace("Second", "Changed")
    part.write_text(changed)
    assert main([*args, "--check"]) == 1
    assert part.read_text() == changed


def test_missing_source_and_usage(tmp_path: Path) -> None:
    assert main(["--src", str(tmp_path / "missing"), "--out", str(tmp_path)]) == 1
    with pytest.raises(SystemExit) as raised:
        main([])
    assert raised.value.code == 2
