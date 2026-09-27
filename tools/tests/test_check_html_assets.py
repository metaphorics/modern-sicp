# SPDX-License-Identifier: MIT

from pathlib import Path

import pytest

from check_html_assets import main


def write_file(path: Path, content: str = "") -> Path:
    """Create one synthetic asset, including its parent directories."""
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")
    return path


def test_clean_tree_reports_local_images_and_fonts(
    tmp_path: Path, capsys: pytest.CaptureFixture[str]
) -> None:
    """Local image/object and CSS font references resolve across nested pages."""
    html = tmp_path / "html"
    write_file(html / "fig" / "a.svg", "<svg />")
    write_file(html / "fig" / "b.svg", "<svg />")
    write_file(html / "fonts" / "x.woff", "font")
    write_file(html / "css" / "m.woff2", "font")
    write_file(
        html / "css" / "book.css",
        """@font-face {
  src: url('../fonts/x.woff');
}
@font-face {
  src: url(m.woff2);
}
.icon { background: url(data:image/svg+xml;base64,AAAA); }
.remote { background: url(https://cdn.example.test/icon.svg); }
""",
    )
    write_file(
        html / "page.html",
        """<html><head><link rel="stylesheet" href="css/book.css"></head>
<body><img src="fig/a.svg"><object data="fig/b.svg"></object>
<img src="https://example.test/remote.svg">
<object data="data:image/svg+xml;base64,AAAA"></object></body></html>
""",
    )
    write_file(
        html / "sub" / "page.html",
        """<html><head><link rel="stylesheet" href="../css/book.css"></head>
<body><img src="../fig/a.svg"></body></html>
""",
    )

    assert main(["--html", str(html)]) == 0
    assert capsys.readouterr().out == "pages=2 images=3 fonts=2\n"


def test_dangling_image_fails_with_page_and_reference(
    tmp_path: Path, capsys: pytest.CaptureFixture[str]
) -> None:
    """The first unresolved image is reported on stderr."""
    html = tmp_path / "html"
    write_file(html / "page.html", '<img src="fig/missing.svg">')

    assert main(["--html", str(html)]) == 1
    assert capsys.readouterr().err == "page.html: fig/missing.svg\n"


def test_dangling_css_url_fails_with_page_and_reference(
    tmp_path: Path, capsys: pytest.CaptureFixture[str]
) -> None:
    """A missing CSS URL is attributed to the HTML page that linked its sheet."""
    html = tmp_path / "html"
    write_file(html / "css" / "book.css", ".icon { src: url('../fonts/missing.woff'); }\n")
    write_file(html / "page.html", '<link rel="stylesheet" href="css/book.css">')

    assert main(["--html", str(html)]) == 1
    assert capsys.readouterr().err == "page.html: ../fonts/missing.woff\n"


def test_rooted_reference_uses_site_root(
    tmp_path: Path, capsys: pytest.CaptureFixture[str]
) -> None:
    """A leading slash is resolved against --site-root rather than the page directory."""
    html = tmp_path / "html"
    site_root = tmp_path / "site"
    write_file(site_root / "assets" / "picture.svg", "<svg />")
    write_file(html / "page.html", '<img src="/assets/picture.svg">')

    assert main(["--html", str(html), "--site-root", str(site_root)]) == 0
    assert capsys.readouterr().out == "pages=1 images=1 fonts=0\n"

    assert main(["--html", str(html)]) == 1
    assert capsys.readouterr().err == "page.html: /assets/picture.svg\n"


def test_missing_stylesheet_fails(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    """A stylesheet href is checked before its CSS contents are read."""
    html = tmp_path / "html"
    write_file(html / "page.html", '<link rel="stylesheet alternate" href="css/missing.css">')

    assert main(["--html", str(html)]) == 1
    assert capsys.readouterr().err == "page.html: css/missing.css\n"


def test_non_html_files_are_ignored(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    """Only files ending in .html contribute pages or references."""
    html = tmp_path / "html"
    write_file(html / "notes.txt", '<img src="missing.svg">')

    assert main(["--html", str(html)]) == 0
    assert capsys.readouterr().out == "pages=0 images=0 fonts=0\n"


def test_missing_html_directory_fails(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    """A missing input directory returns the defect exit code."""
    missing = tmp_path / "missing"

    assert main(["--html", str(missing)]) == 1
    assert "not a directory" in capsys.readouterr().err


def test_help_is_available(capsys: pytest.CaptureFixture[str]) -> None:
    """Argparse exposes a successful help path."""
    with pytest.raises(SystemExit) as raised:
        main(["--help"])
    assert raised.value.code == 0
    assert "usage:" in capsys.readouterr().out
