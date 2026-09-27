# SPDX-License-Identifier: MIT
import re
from collections.abc import Sequence
from pathlib import Path

import pytest

from edition_switcher import main

PAGES = ("index.html", "1.1.html", "1.2.html")
BODIES = {
    "index.html": "<html><head><title>t</title></head>\n<body>\n<h1>home</h1>\n</body>\n</html>\n",
    "1.1.html": '<html>\n<body lang="en" id="sec-1_1">\n<p>x</p>\n</body>\n</html>\n',
    "1.2.html": "<html><body><p>y</p></body></html>\n",
}
EDITIONS: tuple[str, ...] = ("rust", "ocaml", "typescript", "kotlin")


def write_site(root: Path, names: Sequence[str] = PAGES) -> Path:
    html_dir = root / "html"
    html_dir.mkdir(parents=True)
    for name in names:
        (html_dir / name).write_text(BODIES[name], encoding="utf-8")
    return html_dir


def write_siblings(
    root: Path, names: Sequence[str] = PAGES, editions: Sequence[str] = EDITIONS
) -> Path:
    for edition in editions:
        target = root / edition / "html"
        target.mkdir(parents=True)
        for name in names:
            (target / name).write_text("<html><body></body></html>\n", encoding="utf-8")
    return root


def read_pages(html_dir: Path) -> dict[str, str]:
    return {page.name: page.read_text(encoding="utf-8") for page in sorted(html_dir.glob("*.html"))}


def nav_of(content: str) -> str:
    match = re.search(r'(<nav class="editions">.*?</nav>)', content, re.DOTALL)
    assert match is not None
    return match[1]


def test_inserts_nav_after_body_tags_with_and_without_attributes(
    tmp_path: Path, capsys: pytest.CaptureFixture[str]
) -> None:
    html_dir = write_site(tmp_path)
    assert main(["--html", str(html_dir), "--edition", "rust"]) == 0
    captured = capsys.readouterr()
    assert captured.out == "pages=3\n"
    assert captured.err == ""

    def expected_nav(page_name: str) -> str:
        return (
            '<nav class="editions"><span class="current">rust</span>'
            f'<a href="/modern-sicp/ocaml/{page_name}">ocaml</a>'
            f'<a href="/modern-sicp/typescript/{page_name}">typescript</a>'
            f'<a href="/modern-sicp/kotlin/{page_name}">kotlin</a></nav>'
        )

    plain = (html_dir / "index.html").read_text(encoding="utf-8")
    assert "<body>" + expected_nav("index.html") + "\n<h1>home</h1>" in plain
    attributed = (html_dir / "1.1.html").read_text(encoding="utf-8")
    assert '<body lang="en" id="sec-1_1">' + expected_nav("1.1.html") in attributed
    for name in PAGES:
        content = (html_dir / name).read_text(encoding="utf-8")
        assert content.count('<nav class="editions">') == 1


def test_second_run_is_byte_identical(tmp_path: Path) -> None:
    html_dir = write_site(tmp_path)
    argv = ["--html", str(html_dir), "--edition", "rust"]
    assert main(argv) == 0
    first = read_pages(html_dir)
    assert main(argv) == 0
    second = read_pages(html_dir)
    assert second == first
    for name, text in second.items():
        assert text.count('<nav class="editions">') == 1
        assert re.search(r'<body\b[^>]*><nav class="editions">', text), name


def test_existing_nav_is_replaced_not_duplicated(tmp_path: Path) -> None:
    html_dir = write_site(tmp_path)
    page = html_dir / "1.2.html"
    page.write_text(
        page.read_text(encoding="utf-8").replace(
            "<body>", '<body><nav class="editions">STALE</nav>'
        ),
        encoding="utf-8",
    )
    assert main(["--html", str(html_dir), "--edition", "kotlin"]) == 0
    updated = page.read_text(encoding="utf-8")
    assert "STALE" not in updated
    assert updated.count('<nav class="editions">') == 1
    assert '<span class="current">kotlin</span>' in updated


def test_sibling_check_passes(tmp_path: Path) -> None:
    html_dir = write_site(tmp_path)
    siblings = write_siblings(tmp_path / "editions")
    before = read_pages(html_dir)
    assert main(["--html", str(html_dir), "--edition", "rust", "--siblings", str(siblings)]) == 0
    assert read_pages(html_dir) != before


def test_missing_sibling_fails_without_touching_pages(
    tmp_path: Path, capsys: pytest.CaptureFixture[str]
) -> None:
    html_dir = write_site(tmp_path)
    siblings = write_siblings(tmp_path / "editions")
    (siblings / "ocaml" / "html" / "1.2.html").unlink()
    before = read_pages(html_dir)
    assert main(["--html", str(html_dir), "--edition", "rust", "--siblings", str(siblings)]) == 1
    error = capsys.readouterr().err
    assert "1.2.html" in error
    assert "ocaml" in error
    assert read_pages(html_dir) == before


def test_custom_edition_order_and_site_root(tmp_path: Path) -> None:
    html_dir = write_site(tmp_path)
    siblings = write_siblings(tmp_path / "editions", editions=("kotlin", "rust"))
    argv = [
        "--html",
        str(html_dir),
        "--edition",
        "typescript",
        "--editions",
        "kotlin,rust,typescript",
        "--site-root",
        "/editions",
        "--siblings",
        str(siblings),
    ]
    assert main(argv) == 0
    assert nav_of((html_dir / "1.1.html").read_text(encoding="utf-8")) == (
        '<nav class="editions">'
        '<a href="/editions/kotlin/1.1.html">kotlin</a>'
        '<a href="/editions/rust/1.1.html">rust</a>'
        '<span class="current">typescript</span></nav>'
    )


def test_edition_outside_editions_fails(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    html_dir = write_site(tmp_path)
    with pytest.raises(SystemExit) as raised:
        main(["--html", str(html_dir), "--edition", "haskell"])
    assert raised.value.code == 2
    assert "haskell" in capsys.readouterr().err


def test_help_exits_zero(capsys: pytest.CaptureFixture[str]) -> None:
    with pytest.raises(SystemExit) as raised:
        main(["--help"])
    assert raised.value.code == 0
    documented = capsys.readouterr().out
    assert "--edition" in documented
    assert "--siblings" in documented
