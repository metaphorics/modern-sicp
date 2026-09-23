# SPDX-License-Identifier: MIT
"""Repair the SVG media type and the MathML property in an EPUB package manifest.

Reads the OPF named by META-INF/container.xml, sets image/svg+xml on every
manifest item whose href ends in .svg, and adds mathml to the properties of
every application/xhtml+xml item whose document bytes contain a <math element.
The zip is rewritten in place with mimetype kept as the first, stored entry.
"""

import argparse
import os
import posixpath
import re
import sys
import tempfile
import zipfile
from collections.abc import Sequence
from pathlib import Path
from urllib.parse import unquote
from xml.etree import ElementTree as ET

CONTAINER_NAME = "META-INF/container.xml"
CONTAINER_NS = "urn:oasis:names:tc:opendocument:xmlns:container:1.0"
OPF_NS = "http://www.idpf.org/2007/opf"
OPF_MEDIA_TYPE = "application/oebps-package+xml"
SVG_MEDIA_TYPE = "image/svg+xml"
XHTML_MEDIA_TYPE = "application/xhtml+xml"
MIMETYPE_NAME = "mimetype"
MIMETYPE_CONTENT = b"application/epub+zip"
MATH_MARKER = b"<math"
DATA_DESCRIPTOR_FLAG = 0x8
XMLNS_PATTERN = re.compile(r'xmlns(?::([A-Za-z_][\w.-]*))?\s*=\s*["\']([^"\']+)["\']')


def register_namespaces(source: str) -> None:
    """Register every namespace declared in the OPF so serialization keeps prefixes."""
    for prefix, uri in XMLNS_PATTERN.findall(source):
        ET.register_namespace(prefix, uri)


def resolve_href(opf_dir: str, href: str) -> str:
    """Resolve a manifest href against the OPF directory inside the zip."""
    ref = unquote(href)
    if ref.startswith("/"):
        return posixpath.normpath(ref)
    return posixpath.normpath(posixpath.join(opf_dir, ref))


def package_path(container: bytes) -> str:
    """Return the OPF zip path named by the container's rootfile elements."""
    # Parses the container.xml this build produced, not untrusted input.
    root = ET.fromstring(container)  # noqa: S314
    rootfiles = list(root.iter(f"{{{CONTAINER_NS}}}rootfile"))
    if not rootfiles:
        raise ValueError(f"{CONTAINER_NAME}: no rootfile element")
    for element in rootfiles:
        full_path = element.get("full-path")
        if element.get("media-type") == OPF_MEDIA_TYPE and full_path is not None:
            return unquote(full_path)
    fallback = rootfiles[0].get("full-path") or ""
    return unquote(fallback)


def fix_manifest(opf_name: str, opf: bytes, package: dict[str, bytes]) -> tuple[bytes, int, int]:
    """Fix manifest attributes; return the new OPF bytes and both item counts."""
    register_namespaces(opf.decode("utf-8"))
    # Parses the OPF this build produced, not untrusted input.
    root = ET.fromstring(opf)  # noqa: S314
    manifest = root.find(f"{{{OPF_NS}}}manifest")
    if manifest is None:
        raise ValueError(f"{opf_name}: no manifest element")
    opf_dir = posixpath.dirname(opf_name)
    svg_items = 0
    mathml_docs = 0
    for item in manifest.findall(f"{{{OPF_NS}}}item"):
        href = item.get("href")
        if href is None:
            continue
        if href.lower().endswith(".svg"):
            svg_items += 1
            if item.get("media-type") != SVG_MEDIA_TYPE:
                item.set("media-type", SVG_MEDIA_TYPE)
        elif item.get("media-type") == XHTML_MEDIA_TYPE:
            document = package.get(resolve_href(opf_dir, href))
            if document is None or MATH_MARKER not in document:
                continue
            mathml_docs += 1
            tokens = (item.get("properties") or "").split()
            if "mathml" not in tokens:
                tokens.append("mathml")
                item.set("properties", " ".join(tokens))
    return ET.tostring(root, encoding="UTF-8", xml_declaration=True), svg_items, mathml_docs


def rewrite_zip(
    path: Path, entries: list[tuple[zipfile.ZipInfo, bytes]], opf_name: str, opf: bytes
) -> None:
    """Write a new zip beside path with the OPF replaced, then swap it in."""
    mimetype = [(info, blob) for info, blob in entries if info.filename == MIMETYPE_NAME]
    rest = [(info, blob) for info, blob in entries if info.filename != MIMETYPE_NAME]
    if mimetype:
        info, blob = mimetype[0]
        info.compress_type = zipfile.ZIP_STORED
        head: list[tuple[zipfile.ZipInfo, bytes]] = [(info, blob)]
    else:
        head = [(zipfile.ZipInfo(MIMETYPE_NAME), MIMETYPE_CONTENT)]
    handle, tmp_name = tempfile.mkstemp(dir=path.parent, prefix=f"{path.name}.", suffix=".tmp")
    os.close(handle)
    tmp = Path(tmp_name)
    try:
        with zipfile.ZipFile(tmp, "w") as out:
            for entry_info, blob in [*head, *rest]:
                # A seekable rewrite never emits a data descriptor, so the stale
                # flag from a streamed source zip must not survive the copy.
                entry_info.flag_bits &= ~DATA_DESCRIPTOR_FLAG
                out.writestr(entry_info, opf if entry_info.filename == opf_name else blob)
        tmp.replace(path)
    finally:
        tmp.unlink(missing_ok=True)


def main(argv: Sequence[str] | None = None) -> int:
    """Fix the manifest of --epub in place, or report why it cannot be fixed."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--epub", type=Path, required=True, help="EPUB file to repair in place")
    args = parser.parse_args(argv)
    try:
        if not args.epub.is_file():
            raise ValueError(f"{args.epub}: not a file")
        with zipfile.ZipFile(args.epub) as archive:
            entries = [(info, archive.read(info)) for info in archive.infolist()]
        package = {info.filename: blob for info, blob in entries}
        container = package.get(CONTAINER_NAME)
        if container is None:
            raise ValueError(f"missing {CONTAINER_NAME}")
        opf_name = package_path(container)
        if opf_name not in package:
            raise ValueError(f"missing package file {opf_name}")
        opf, svg_items, mathml_docs = fix_manifest(opf_name, package[opf_name], package)
        rewrite_zip(args.epub, entries, opf_name, opf)
    except (OSError, ValueError, zipfile.BadZipFile, ET.ParseError) as exc:
        print(str(exc), file=sys.stderr)
        return 1
    else:
        print(f"svg_items={svg_items} mathml_docs={mathml_docs}")
        return 0


if __name__ == "__main__":
    raise SystemExit(main())
