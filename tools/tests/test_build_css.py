# SPDX-License-Identifier: MIT

from pathlib import Path

import pytest

from build_css import main

STYLE = """@import url(fonts/fonts.css);

body { margin: 0; }
"""

FONTS = """@font-face {
  font-family: 'Test';
  src: url('test-r.woff');
}
@font-face {
  font-family: 'Test';
  src: url(test-ri.woff2);
  src: url( test-spaced.woff );
  font-style: italic;
}
"""

HIGHLIGHT = ".k { color: #5a3696; }\n"


def write_assets(root: Path, style: str = STYLE) -> Path:
    """Write a synthetic CSS asset tree and return its assets directory."""
    assets = root / "assets"
    fonts = assets / "fonts"
    fonts.mkdir(parents=True)
    (assets / "style.css").write_text(style, encoding="utf-8")
    (fonts / "fonts.css").write_text(FONTS, encoding="utf-8")
    (assets / "highlight.css").write_text(HIGHLIGHT, encoding="utf-8")
    return assets


def test_book_css_inlines_fonts_and_preserves_order(tmp_path: Path) -> None:
    """The screen stylesheet prefixes font URLs and concatenates the three inputs."""
    assets = write_assets(tmp_path)
    output = tmp_path / "book"

    assert main(["--assets", str(assets), "--out", str(output)]) == 0

    css = (output / "book.css").read_text(encoding="utf-8")
    assert "url('fonts/test-r.woff')" in css
    assert "url(fonts/test-ri.woff2)" in css
    assert "url(fonts/test-spaced.woff)" in css
    assert "@import" not in css
    assert css.index("test-r.woff") < css.index("body { margin: 0; }") < css.index(".k {")


def test_custom_font_url_prefix_is_used(tmp_path: Path) -> None:
    """The optional prefix is inserted without changing the font URL quotes."""
    assets = write_assets(tmp_path)
    output = tmp_path / "book"

    assert (
        main(
            [
                "--assets",
                str(assets),
                "--out",
                str(output),
                "--font-url-prefix",
                "../fonts/",
            ]
        )
        == 0
    )

    css = (output / "book.css").read_text(encoding="utf-8")
    assert "url('../fonts/test-r.woff')" in css


def test_epub_css_removes_import_fonts_and_navigation_rules(tmp_path: Path) -> None:
    """The EPUB stylesheet drops embedded fonts and fixed navigation rules only."""
    style = """@import url(fonts/fonts.css);
@font-face {
  font-family: 'Inline';
  src: url('inline.woff');
}
body { margin: 0; position: relative; }
.jump {
  font-family: "DejaVu-Arrows";
  position: fixed;
  right: 0;
}
.jump a[href] {
  color: rgba(220, 220, 220, .5);
}
.fixed {
  position: fixed;
}
p { hyphens: auto; }
"""
    assets = write_assets(tmp_path, style)
    output = tmp_path / "book"

    assert main(["--assets", str(assets), "--out", str(output)]) == 0

    book_css = (output / "book.css").read_text(encoding="utf-8")
    epub_css = (output / "book-epub.css").read_text(encoding="utf-8")
    assert "inline.woff" in book_css
    assert epub_css.startswith("html { font-size: 100% }")
    assert "@import" not in epub_css
    assert "@font-face" not in epub_css
    assert "inline.woff" not in epub_css
    assert ".fixed" not in epub_css
    assert "position: fixed" not in epub_css
    assert "body { margin: 0; position: relative; }" in epub_css
    assert "p { hyphens: auto; }" in epub_css
    assert HIGHLIGHT.strip() in epub_css


def test_epub_strips_nested_navigation_rules_without_dropping_media(tmp_path: Path) -> None:
    """A media wrapper remains when only one nested rule is removed."""
    style = """@import url(fonts/fonts.css);
@media (max-width: 480px) {
  .jump { position: fixed; }
  .kept { color: red; }
}
"""
    assets = write_assets(tmp_path, style)
    output = tmp_path / "book"

    assert main(["--assets", str(assets), "--out", str(output)]) == 0

    epub_css = (output / "book-epub.css").read_text(encoding="utf-8")
    assert "@media (max-width: 480px)" in epub_css
    assert ".kept { color: red; }" in epub_css
    assert ".jump" not in epub_css


def test_report_contains_written_byte_counts(
    tmp_path: Path, capsys: pytest.CaptureFixture[str]
) -> None:
    """The report uses the exact UTF-8 byte counts of both output files."""
    assets = write_assets(tmp_path)
    output = tmp_path / "book"

    assert main(["--assets", str(assets), "--out", str(output)]) == 0

    expected = (
        f"book_css={(output / 'book.css').stat().st_size} "
        f"epub_css={(output / 'book-epub.css').stat().st_size}\n"
    )
    assert capsys.readouterr().out == expected


def test_surviving_import_fails_the_check(
    tmp_path: Path, capsys: pytest.CaptureFixture[str]
) -> None:
    """A literal import marker that survives transformation fails closed."""
    assets = write_assets(tmp_path, "/* @import must not survive */\nbody { color: red; }\n")
    output = tmp_path / "book"

    assert main(["--assets", str(assets), "--out", str(output)]) == 1
    assert "@import survived" in capsys.readouterr().err


def test_missing_asset_is_reported(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    """A missing source stylesheet returns the defect exit code."""
    assets = tmp_path / "assets"
    (assets / "fonts").mkdir(parents=True)
    (assets / "style.css").write_text(STYLE, encoding="utf-8")
    (assets / "fonts" / "fonts.css").write_text(FONTS, encoding="utf-8")

    assert main(["--assets", str(assets), "--out", str(tmp_path / "book")]) == 1
    assert "highlight.css" in capsys.readouterr().err


def test_help_is_available(capsys: pytest.CaptureFixture[str]) -> None:
    """Argparse exposes a successful help path."""
    with pytest.raises(SystemExit) as raised:
        main(["--help"])
    assert raised.value.code == 0
    assert "usage:" in capsys.readouterr().out
