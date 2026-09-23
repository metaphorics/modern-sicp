# SPDX-License-Identifier: MIT
import zipfile
from pathlib import Path
from xml.etree import ElementTree as ET

import pytest

from epub_manifest_fix import main

CONTAINER = b"""<?xml version="1.0" encoding="UTF-8"?>
<container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container:1.0">
  <rootfiles>
    <rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml"/>
  </rootfiles>
</container>
"""

OPF = b"""<?xml version="1.0" encoding="UTF-8"?>
<package xmlns="http://www.idpf.org/2007/opf" xmlns:dc="http://purl.org/dc/elements/1.1/"
         version="3.0" unique-identifier="pub-id">
  <metadata>
    <dc:identifier id="pub-id">urn:uuid:test</dc:identifier>
    <dc:title>SICP</dc:title>
  </metadata>
  <manifest>
    <item id="ch1" href="ch1.xhtml" media-type="application/xhtml+xml"/>
    <item id="ch2" href="ch2.xhtml" media-type="application/xhtml+xml" properties="mathml"/>
    <item id="ch3" href="ch3.xhtml" media-type="application/xhtml+xml"/>
    <item id="fig1" href="fig1.svg" media-type="image/png"/>
    <item id="fig2" href="fig2.svg" media-type="image/svg+xml"/>
    <item id="css" href="style.css" media-type="text/css"/>
  </manifest>
</package>
"""

CH1 = b"<html><body><math><mi>x</mi></math></body></html>"
CH2 = b"<html><body><math/></body></html>"
CH3 = b"<html><body><p>plain</p></body></html>"
SVG = b"<svg xmlns='http://www.w3.org/2000/svg'/>"
CSS = b"body { color: black }"
OPF_ITEM = "{http://www.idpf.org/2007/opf}item"


def build_epub(path: Path, opf: bytes = OPF) -> None:
    with zipfile.ZipFile(path, "w") as archive:
        archive.writestr("mimetype", b"application/epub+zip", compress_type=zipfile.ZIP_STORED)
        archive.writestr("META-INF/container.xml", CONTAINER, compress_type=zipfile.ZIP_DEFLATED)
        archive.writestr("OEBPS/content.opf", opf, compress_type=zipfile.ZIP_DEFLATED)
        archive.writestr("OEBPS/ch1.xhtml", CH1, compress_type=zipfile.ZIP_DEFLATED)
        archive.writestr("OEBPS/ch2.xhtml", CH2, compress_type=zipfile.ZIP_STORED)
        archive.writestr("OEBPS/ch3.xhtml", CH3, compress_type=zipfile.ZIP_DEFLATED)
        archive.writestr("OEBPS/fig1.svg", SVG, compress_type=zipfile.ZIP_DEFLATED)
        archive.writestr("OEBPS/fig2.svg", SVG, compress_type=zipfile.ZIP_STORED)
        archive.writestr("OEBPS/style.css", CSS, compress_type=zipfile.ZIP_DEFLATED)


def read_zip(path: Path) -> dict[str, tuple[bytes, int]]:
    with zipfile.ZipFile(path) as archive:
        return {
            info.filename: (archive.read(info), info.compress_type) for info in archive.infolist()
        }


def opf_items(path: Path) -> dict[str, dict[str, str]]:
    # Parses the test's own OPF fixture bytes, not untrusted input.
    root = ET.fromstring(read_zip(path)["OEBPS/content.opf"][0])  # noqa: S314
    return {item.get("href", ""): dict(item.attrib) for item in root.iter(OPF_ITEM)}


def test_fixes_svg_media_type_and_adds_mathml(
    tmp_path: Path, capsys: pytest.CaptureFixture[str]
) -> None:
    epub = tmp_path / "book.epub"
    build_epub(epub)
    assert main(["--epub", str(epub)]) == 0
    assert capsys.readouterr().out == "svg_items=2 mathml_docs=2\n"
    items = opf_items(epub)
    assert items["fig1.svg"]["media-type"] == "image/svg+xml"
    assert items["fig2.svg"]["media-type"] == "image/svg+xml"
    assert items["ch1.xhtml"]["properties"] == "mathml"
    assert items["ch2.xhtml"]["properties"] == "mathml"
    assert "mathml mathml" not in read_zip(epub)["OEBPS/content.opf"][0].decode()
    assert "properties" not in items["ch3.xhtml"]
    assert items["style.css"]["media-type"] == "text/css"
    assert items["ch3.xhtml"]["media-type"] == "application/xhtml+xml"


def test_mimetype_stays_first_stored_and_other_entries_keep_bytes(
    tmp_path: Path,
) -> None:
    epub = tmp_path / "book.epub"
    build_epub(epub)
    assert main(["--epub", str(epub)]) == 0
    entries = read_zip(epub)
    with zipfile.ZipFile(epub) as archive:
        infos = archive.infolist()
    assert infos[0].filename == "mimetype"
    assert infos[0].compress_type == zipfile.ZIP_STORED
    assert entries["mimetype"][0] == b"application/epub+zip"
    assert entries["OEBPS/ch2.xhtml"][1] == zipfile.ZIP_STORED
    assert entries["OEBPS/fig2.svg"][1] == zipfile.ZIP_STORED
    assert entries["OEBPS/ch1.xhtml"][1] == zipfile.ZIP_DEFLATED
    assert entries["OEBPS/ch1.xhtml"][0] == CH1
    assert entries["OEBPS/ch2.xhtml"][0] == CH2
    assert entries["OEBPS/ch3.xhtml"][0] == CH3
    assert entries["OEBPS/fig1.svg"][0] == SVG
    assert entries["OEBPS/style.css"][0] == CSS


def test_second_run_is_byte_identical(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    epub = tmp_path / "book.epub"
    build_epub(epub)
    assert main(["--epub", str(epub)]) == 0
    first = read_zip(epub)
    assert main(["--epub", str(epub)]) == 0
    assert read_zip(epub) == first
    assert capsys.readouterr().out == "svg_items=2 mathml_docs=2\nsvg_items=2 mathml_docs=2\n"


def test_missing_container_fails(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    epub = tmp_path / "book.epub"
    with zipfile.ZipFile(epub, "w") as archive:
        archive.writestr("mimetype", b"application/epub+zip", compress_type=zipfile.ZIP_STORED)
        archive.writestr("OEBPS/content.opf", OPF, compress_type=zipfile.ZIP_DEFLATED)
    assert main(["--epub", str(epub)]) == 1
    assert "META-INF/container.xml" in capsys.readouterr().err


def test_missing_package_file_fails(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    epub = tmp_path / "book.epub"
    container = CONTAINER.replace(b"OEBPS/content.opf", b"OEBPS/absent.opf")
    with zipfile.ZipFile(epub, "w") as archive:
        archive.writestr("mimetype", b"application/epub+zip", compress_type=zipfile.ZIP_STORED)
        archive.writestr("META-INF/container.xml", container, compress_type=zipfile.ZIP_DEFLATED)
    assert main(["--epub", str(epub)]) == 1
    assert "OEBPS/absent.opf" in capsys.readouterr().err


def test_not_a_zip_fails(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    epub = tmp_path / "book.epub"
    epub.write_bytes(b"plain text, no zip here")
    assert main(["--epub", str(epub)]) == 1
    assert capsys.readouterr().err


def test_missing_file_fails(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    assert main(["--epub", str(tmp_path / "nope.epub")]) == 1
    assert capsys.readouterr().err


def test_help_exits_zero(capsys: pytest.CaptureFixture[str]) -> None:
    with pytest.raises(SystemExit) as raised:
        main(["--help"])
    assert raised.value.code == 0
    assert "--epub" in capsys.readouterr().out
