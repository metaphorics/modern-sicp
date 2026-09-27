# SPDX-License-Identifier: MIT
"""Check that assets referenced by built HTML pages resolve to files."""

import argparse
import re
import sys
from html.parser import HTMLParser
from pathlib import Path
from typing import Final
from urllib.parse import urlparse

FONT_SUFFIXES: Final[frozenset[str]] = frozenset({".woff", ".woff2", ".ttf", ".otf"})
SKIPPED_SCHEMES: Final[frozenset[str]] = frozenset({"http", "https", "data"})
CSS_URL_RE: Final[re.Pattern[str]] = re.compile(
    r"""url\(\s*(?:"""
    r"""(?P<single>'[^']*')|(?P<double>"[^"]*")|"""
    r"""(?P<bare>[^\s)]+))\s*\)"""
)


class AssetError(Exception):
    """An asset reference that does not resolve to an existing file."""


class ReferenceCollector(HTMLParser):
    """Collect local image, object, and stylesheet references from one HTML page."""

    def __init__(self) -> None:
        """Initialize an empty reference list."""
        super().__init__(convert_charrefs=True)
        self.references: list[tuple[str, str]] = []

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        """Record the supported references from one start tag."""
        values = {name: value for name, value in attrs if value is not None}
        if tag == "img" and values.get("src"):
            self.references.append(("image", values["src"]))
        elif tag == "object" and values.get("data"):
            self.references.append(("image", values["data"]))
        elif tag == "link":
            rel_tokens = (values.get("rel") or "").lower().split()
            if "stylesheet" in rel_tokens and values.get("href"):
                self.references.append(("stylesheet", values["href"]))


def is_skipped_reference(reference: str) -> bool:
    """Return whether a URL is external, data-backed, or a document fragment."""
    return reference.startswith("#") or urlparse(reference).scheme in SKIPPED_SCHEMES


def resolve_reference(reference: str, base_file: Path, site_root: Path | None) -> Path | None:
    """Resolve one URL against its file, or against the configured site root."""
    if reference.startswith("/"):
        if site_root is None:
            return None
        return (site_root / reference.lstrip("/")).resolve(strict=False)
    return (base_file.parent / reference).resolve(strict=False)


def display_page(page: Path, html_root: Path) -> str:
    """Format a page path relative to the checked HTML root."""
    try:
        return page.relative_to(html_root).as_posix()
    except ValueError:
        return page.as_posix()


def unresolved(page: Path, html_root: Path, reference: str) -> AssetError:
    """Create the required diagnostic for one unresolved reference."""
    return AssetError(f"{display_page(page, html_root)}: {reference}")


def css_url_target(match: re.Match[str]) -> str:
    """Extract one CSS URL while accepting quoted and unquoted forms."""
    single = match.group("single")
    double = match.group("double")
    if single is not None:
        return single[1:-1]
    if double is not None:
        return double[1:-1]
    bare = match.group("bare")
    if bare is None:
        raise ValueError("CSS URL has no target")
    return bare


def collect_css_urls(css_file: Path) -> list[str]:
    """Return CSS ``url(...)`` references in source order."""
    text = css_file.read_text(encoding="utf-8")
    return [css_url_target(match) for match in CSS_URL_RE.finditer(text)]


def check_stylesheet(
    css_file: Path,
    page: Path,
    html_root: Path,
    site_root: Path | None,
) -> int:
    """Validate one stylesheet and return its number of local font URLs."""
    font_count = 0
    for css_reference in collect_css_urls(css_file):
        if is_skipped_reference(css_reference):
            continue
        css_target = resolve_reference(css_reference, css_file, site_root)
        if css_target is None or not css_target.is_file():
            raise unresolved(page, html_root, css_reference)
        if css_target.suffix.lower() in FONT_SUFFIXES:
            font_count += 1
    return font_count


def check_tree(html_root: Path, site_root: Path | None) -> tuple[int, int, int]:
    """Validate one HTML tree and return page, image, and font counts."""
    pages = sorted(path for path in html_root.rglob("*.html") if path.is_file())
    images = 0
    fonts = 0
    checked_stylesheets: set[Path] = set()
    for page in pages:
        collector = ReferenceCollector()
        collector.feed(page.read_text(encoding="utf-8"))
        collector.close()
        for kind, reference in collector.references:
            if is_skipped_reference(reference):
                continue
            target = resolve_reference(reference, page, site_root)
            if target is None or not target.is_file():
                raise unresolved(page, html_root, reference)
            if kind == "image":
                images += 1
                continue
            if target in checked_stylesheets:
                continue
            checked_stylesheets.add(target)
            fonts += check_stylesheet(target, page, html_root, site_root)
    return len(pages), images, fonts


def parse_args(argv: list[str] | None) -> argparse.Namespace:
    """Parse command-line arguments for the HTML asset checker."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--html",
        type=Path,
        required=True,
        help="directory containing built HTML pages",
    )
    parser.add_argument(
        "--site-root",
        type=Path,
        default=None,
        help="filesystem root used to resolve references beginning with /",
    )
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    """Validate HTML assets and print the required summary on success."""
    args = parse_args(argv)
    if not args.html.is_dir():
        print(f"check_html_assets: {args.html}: not a directory", file=sys.stderr)
        return 1
    try:
        pages, images, fonts = check_tree(args.html, args.site_root)
    except (AssetError, OSError) as error:
        print(error, file=sys.stderr)
        return 1
    print(f"pages={pages} images={images} fonts={fonts}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
