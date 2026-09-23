# SPDX-License-Identifier: MIT
import os
from pathlib import Path

import pytest

from figures_pdf import main

# Mimics rsvg-convert: -f pdf -o OUT IN — copy the input file to the output path.
FAKE_CONVERTER = """#!/bin/sh
out=
prev=
for arg in "$@"; do
  if [ "$prev" = "-o" ]; then out=$arg; fi
  prev=$arg
done
for last in "$@"; do :; done
cp "$last" "$out"
"""


@pytest.fixture
def converter(monkeypatch: pytest.MonkeyPatch, tmp_path: Path) -> Path:
    script = tmp_path / "fake-rsvg-convert"
    script.write_text(FAKE_CONVERTER)
    script.chmod(0o755)
    monkeypatch.setenv("RSVG_CONVERT", str(script))
    return script


def write_svg(src: Path, relative: str, body: str = "<svg/>") -> Path:
    svg = src / relative
    svg.parent.mkdir(parents=True, exist_ok=True)
    svg.write_text(body)
    return svg


@pytest.mark.usefixtures("converter")
def test_fresh_conversion_mirrors_nested_dirs(
    tmp_path: Path, capsys: pytest.CaptureFixture[str]
) -> None:
    src = tmp_path / "figures"
    out = tmp_path / "pdf"
    write_svg(src, "Fig1.1.svg", "<svg>a</svg>")
    write_svg(src, "chap2/sub/Fig2.3.svg", "<svg>b</svg>")
    assert main(["--src", str(src), "--out", str(out)]) == 0
    assert (out / "Fig1.1.pdf").read_text() == "<svg>a</svg>"
    assert (out / "chap2/sub/Fig2.3.pdf").read_text() == "<svg>b</svg>"
    assert capsys.readouterr().out == "converted=2 skipped=0 stale=0\n"


@pytest.mark.usefixtures("converter")
def test_skips_when_pdf_is_newer_and_reconverts_when_stale(
    tmp_path: Path, capsys: pytest.CaptureFixture[str]
) -> None:
    src = tmp_path / "figures"
    out = tmp_path / "pdf"
    svg = write_svg(src, "Fig1.1.svg")
    argv = ["--src", str(src), "--out", str(out)]
    assert main(argv) == 0
    assert main(argv) == 0  # the PDF the converter just wrote is newer than the SVG
    assert capsys.readouterr().out.splitlines()[-1] == "converted=0 skipped=1 stale=0"
    future = svg.stat().st_mtime + 100
    os.utime(svg, (future, future))
    assert main(argv) == 0
    assert capsys.readouterr().out == "converted=1 skipped=0 stale=0\n"


@pytest.mark.usefixtures("converter")
def test_check_lists_missing_and_stale_without_converting(
    tmp_path: Path, capsys: pytest.CaptureFixture[str]
) -> None:
    src = tmp_path / "figures"
    out = tmp_path / "pdf"
    svg = write_svg(src, "chap1/Fig1.1.svg")
    argv = ["--src", str(src), "--out", str(out), "--check"]
    assert main(argv) == 1
    captured = capsys.readouterr()
    assert "missing" in captured.err
    assert "Fig1.1.pdf" in captured.err
    assert not (out / "chap1/Fig1.1.pdf").exists()
    assert main(argv[:-1]) == 0
    future = svg.stat().st_mtime + 100
    os.utime(svg, (future, future))
    assert main(argv) == 1
    captured = capsys.readouterr()
    assert "stale" in captured.err
    assert "converted=0" in captured.out


@pytest.mark.usefixtures("converter")
def test_binary_from_rsvg_convert_env(tmp_path: Path) -> None:
    src = tmp_path / "figures"
    out = tmp_path / "pdf"
    write_svg(src, "Fig5.2.svg")
    assert main(["--src", str(src), "--out", str(out)]) == 0
    assert (out / "Fig5.2.pdf").exists()


def test_binary_from_path_when_env_unset(tmp_path: Path, monkeypatch: pytest.MonkeyPatch) -> None:
    bindir = tmp_path / "bin"
    bindir.mkdir()
    script = bindir / "rsvg-convert"
    script.write_text(FAKE_CONVERTER)
    script.chmod(0o755)
    monkeypatch.setenv("PATH", f"{bindir}{os.pathsep}{os.environ['PATH']}")
    monkeypatch.delenv("RSVG_CONVERT", raising=False)
    src = tmp_path / "figures"
    out = tmp_path / "pdf"
    write_svg(src, "Fig1.2.svg")
    assert main(["--src", str(src), "--out", str(out)]) == 0
    assert (out / "Fig1.2.pdf").exists()


def test_missing_binary_fails_without_creating_output(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch, capsys: pytest.CaptureFixture[str]
) -> None:
    monkeypatch.setenv("RSVG_CONVERT", "no-such-rsvg-anywhere")
    src = tmp_path / "figures"
    out = tmp_path / "pdf"
    write_svg(src, "Fig1.1.svg")
    assert main(["--src", str(src), "--out", str(out)]) == 1
    assert "no-such-rsvg-anywhere" in capsys.readouterr().err
    assert not out.exists()


def test_missing_source_directory_fails(tmp_path: Path) -> None:
    assert main(["--src", str(tmp_path / "absent"), "--out", str(tmp_path / "pdf")]) == 1


@pytest.mark.usefixtures("converter")
def test_jobs_flag_converts_in_threads(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    src = tmp_path / "figures"
    out = tmp_path / "pdf"
    write_svg(src, "Fig1.1.svg")
    write_svg(src, "chap3/Fig3.1.svg")
    argv = ["--src", str(src), "--out", str(out), "--jobs", "2"]
    assert main(argv) == 0
    assert (out / "chap3/Fig3.1.pdf").exists()
    assert capsys.readouterr().out == "converted=2 skipped=0 stale=0\n"
