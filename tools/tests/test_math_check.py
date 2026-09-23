# SPDX-License-Identifier: MIT
from pathlib import Path

import pytest

from math_check import main

MATHML = "<math display='inline'><mi>x</mi></math>"


def build(tmp_path: Path, **files: str) -> tuple[Path, Path]:
    """Lay out a source tree and an HTML output tree from named fragments."""
    source = tmp_path / "src"
    out = tmp_path / "html"
    source.mkdir()
    out.mkdir()
    (source / "1.1.texi").write_text(files.get("texi", ""))
    (out / "index.html").write_text(files.get("html", ""))
    for key, name in (
        ("sidecar_tex", "index_tex4ht_tex.tex"),
        ("sidecar_html", "index_tex4ht_tex.html"),
        ("log", "index_tex4ht_tex.log"),
    ):
        if files.get(key):
            (out / name).write_text(files[key])
    return source, out


def request(count: int) -> str:
    return "".join(
        f"<!-- tex4ht_begin index_tex4ht_tex math {n} -->\n" for n in range(1, count + 1)
    )


def test_every_requested_fragment_delivered_passes(tmp_path: Path) -> None:
    _, out = build(tmp_path, html=f"<p>{MATHML}{MATHML}</p>", sidecar_tex=request(2))
    assert main(["--html", str(out)]) == 0


def test_dropped_fragment_fails(tmp_path: Path) -> None:
    _, out = build(tmp_path, html="<p>nothing</p>", sidecar_tex=request(2))
    assert main(["--html", str(out)]) == 1


def test_sidecar_does_not_satisfy_the_count(tmp_path: Path) -> None:
    # The sidecar holds the MathML even when the delivered page lost it.
    _, out = build(
        tmp_path,
        html="<p>nothing</p>",
        sidecar_tex=request(1),
        sidecar_html=f"<p>{MATHML}</p>",
    )
    assert main(["--html", str(out)]) == 1


def test_png_fallback_fails(tmp_path: Path) -> None:
    _, out = build(tmp_path, html='<p><img src="index_tex4ht_tex0x.png" alt="x"></p>')
    assert main(["--html", str(out)]) == 1


def test_swallowed_tex_error_fails(tmp_path: Path) -> None:
    _, out = build(tmp_path, html="<p>fine</p>", log="! Undefined control sequence.\n")
    assert main(["--html", str(out)]) == 1


def test_unsupported_macros_fail(tmp_path: Path) -> None:
    source, out = build(
        tmp_path,
        texi="Inline @math{\\text{ if } x} and @math{\\frac{1}{2}}.\n",
        html=f"<p>{MATHML}</p>",
        sidecar_tex=request(1),
    )
    assert main(["--html", str(out), "--source", str(source)]) == 1


def test_source_without_banned_macros_passes(tmp_path: Path) -> None:
    source, out = build(
        tmp_path,
        texi="Inline @math{\\hbox{ if } x} and @math{{1 \\over 2}}.\n",
        html=f"<p>{MATHML}{MATHML}</p>",
        sidecar_tex=request(2),
    )
    assert main(["--html", str(out), "--source", str(source)]) == 0


def test_missing_directory_and_usage(tmp_path: Path) -> None:
    assert main(["--html", str(tmp_path / "absent")]) == 1
    with pytest.raises(SystemExit) as raised:
        main([])
    assert raised.value.code == 2
