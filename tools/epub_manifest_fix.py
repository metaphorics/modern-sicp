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
from html.parser import HTMLParser
from pathlib import Path
from urllib.parse import unquote
from xml.etree import ElementTree as ET

CONTAINER_NAME = "META-INF/container.xml"
CONTAINER_NS = "urn:oasis:names:tc:opendocument:xmlns:container"
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


VOID_ELEMENTS = frozenset(
    [
        "br",
        "img",
        "hr",
        "meta",
        "link",
        "input",
        "col",
        "area",
        "base",
        "embed",
        "source",
        "track",
        "wbr",
    ]
)


BLOCK_STARTS = frozenset(
    [
        "p",
        "div",
        "blockquote",
        "pre",
        "ul",
        "ol",
        "table",
        "h1",
        "h2",
        "h3",
        "h4",
        "h5",
        "h6",
        "li",
        "dd",
        "dt",
        "section",
        "figure",
        "hr",
    ]
)
NOPAR_PATTERN = re.compile(r'<p class="nopar"[^>]*>\s*(?=<)')


class _MarkupRepairer(HTMLParser):
    """Collect the edits that make t4h chapter markup well-formed.

    The t4h extension drops end tags around display formulas and emits a
    literal ``</p>`` where a quotation or list item should close. The repair
    follows the HTML implied-end-tag rules: a block-level start tag closes an
    open ``p``, a new ``li``/``blockquote``/``dd``/``dt`` closes the open
    sibling of the same name, and an end tag that names an element deeper in
    the stack inserts the missing closers in between. A ``</p>`` that names no
    open ``p`` is a mistyped ``</blockquote>`` when a quotation is on top, and
    is dropped otherwise.
    """

    def __init__(self) -> None:
        super().__init__(convert_charrefs=False)
        self.stack: list[str] = []
        self.edits: list[tuple[int, int, str, int]] = []
        self._offsets: list[int] = []
        self._sequence = 0

    def feed(self, data: str) -> None:
        line_starts = [0]
        for index, char in enumerate(data):
            if char == "\n":
                line_starts.append(index + 1)
        self._offsets = line_starts
        super().feed(data)

    def _position(self) -> int:
        line, column = self.getpos()
        return self._offsets[line - 1] + column

    def _emit(self, offset: int, replacement: str, span: int = 0) -> None:
        self.edits.append((offset, self._sequence, replacement, span))
        self._sequence += 1

    def _close_through(self, offset: int, stop: str, passable: frozenset[str]) -> None:
        """Insert the end tags needed to close an open ``stop`` element."""
        boundary = len(self.stack) - 1
        while boundary >= 0 and self.stack[boundary] != stop and self.stack[boundary] in passable:
            boundary -= 1
        if boundary < 0 or self.stack[boundary] != stop:
            return
        while self.stack:
            top = self.stack.pop()
            self._emit(offset, f"</{top}>")
            if top == stop:
                return

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:  # noqa: ARG002
        if tag in VOID_ELEMENTS:
            return
        offset = self._position()
        if tag in BLOCK_STARTS:
            while self.stack and self.stack[-1] == "p":
                self.stack.pop()
                self._emit(offset, "</p>")
        if tag in ("li", "dd", "dt"):
            self._close_through(offset, tag, frozenset({"p", "li", "dd", "dt"}))
        if tag == "blockquote":
            self._close_through(
                offset, "blockquote", frozenset({"p", "li", "dd", "dt", "ol", "ul"})
            )
        self.stack.append(tag)

    def handle_startendtag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:  # noqa: ARG002
        return

    def handle_endtag(self, tag: str) -> None:
        if tag in VOID_ELEMENTS:
            return
        offset = self._position()
        if tag not in self.stack:
            end = self.rawdata.find(">", offset)
            span = end + 1 - offset if end >= 0 else 0
            if tag == "p" and self.stack and self.stack[-1] == "blockquote":
                self._emit(offset, "</blockquote>", span)
                self.stack.pop()
                return
            self._emit(offset, "", span)
            return
        while self.stack and self.stack[-1] != tag:
            self._emit(offset, f"</{self.stack.pop()}>")
        if self.stack:
            self.stack.pop()


NOPAR_PATTERN = re.compile(r'<p class="nopar"[^>]*>\s*(?=<)')
MTABLE_MSPACE_PATTERN = re.compile(r"(<mtable[^>]*>)\s*<mspace[^>]*/>")
SVG_STYLESHEET_PATTERN = re.compile(r"<\?xml-stylesheet[^?]*\?>\s*")
CSS_IMPORT_PATTERN = re.compile(r'@import\s+"([^"]+)"')


def drop_spurious_end_tags(text: str) -> tuple[str, int]:
    """Return the document with t4h markup defects repaired, and the edit count."""
    text = NOPAR_PATTERN.sub("", text)
    repairer = _MarkupRepairer()
    repairer.feed(text)
    repairer.close()
    for offset, _sequence, replacement, span in sorted(repairer.edits, reverse=True):
        text = text[:offset] + replacement + text[offset + span :]
    return text, len(repairer.edits)


def repair_markup(package: dict[str, bytes]) -> int:
    """Repair t4h markup in every chapter document; return the edit count."""
    repairs = 0
    for name in list(package):
        if not name.endswith(".xhtml"):
            continue
        text, count = drop_spurious_end_tags(package[name].decode("utf-8"))
        text, spaces = MTABLE_MSPACE_PATTERN.subn(r"\1", text)
        count += spaces
        if count:
            package[name] = text.encode("utf-8")
            repairs += count
    return repairs


def strip_svg_stylesheets(package: dict[str, bytes]) -> int:
    """Remove the xml-stylesheet reference t4h leaves pointing at a missing css."""
    stripped = 0
    for name in list(package):
        if not name.endswith(".svg"):
            continue
        text = package[name].decode("utf-8")
        text, count = SVG_STYLESHEET_PATTERN.subn("", text)
        if count:
            package[name] = text.encode("utf-8")
            stripped += count
    return stripped


def declare_css_imports(opf_name: str, opf: bytes, package: dict[str, bytes]) -> tuple[bytes, int]:
    """Declare in the OPF manifest every css file an xhtml document imports."""
    register_namespaces(opf.decode("utf-8"))
    root = ET.fromstring(opf)  # noqa: S314
    manifest = root.find(f"{{{OPF_NS}}}manifest")
    if manifest is None:
        raise ValueError(f"{opf_name}: no manifest element")
    opf_dir = posixpath.dirname(opf_name)
    declared = {item.get("href") for item in manifest.findall(f"{{{OPF_NS}}}item")}
    added = 0
    for name, blob in package.items():
        if not name.endswith(".xhtml"):
            continue
        for match in CSS_IMPORT_PATTERN.finditer(blob.decode("utf-8")):
            href = match.group(1)
            if href in declared:
                continue
            resolved = posixpath.normpath(posixpath.join(posixpath.dirname(name), href))
            if resolved not in package:
                continue
            item = ET.SubElement(manifest, f"{{{OPF_NS}}}item")
            item.set("id", f"css-{added}")
            item.set("href", posixpath.relpath(resolved, opf_dir) if opf_dir else resolved)
            item.set("media-type", "text/css")
            declared.add(href)
            added += 1
    return ET.tostring(root, encoding="UTF-8", xml_declaration=True), added


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
        closed_dups = repair_markup(package)
        stripped = strip_svg_stylesheets(package)
        opf, svg_items, mathml_docs = fix_manifest(opf_name, package[opf_name], package)
        opf, declared = declare_css_imports(opf_name, opf, package)
        entries = [(info, package[info.filename]) for info, _ in entries]
        rewrite_zip(args.epub, entries, opf_name, opf)
    except (OSError, ValueError, zipfile.BadZipFile, ET.ParseError) as exc:
        print(str(exc), file=sys.stderr)
        return 1
    else:
        print(
            f"svg_items={svg_items} mathml_docs={mathml_docs} "
            f"closed_dups={closed_dups} svg_css={stripped} css_items={declared}"
        )
        return 0


if __name__ == "__main__":
    raise SystemExit(main())
