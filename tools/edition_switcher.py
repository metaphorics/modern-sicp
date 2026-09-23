# SPDX-License-Identifier: MIT
"""Inject the four-edition navigation into the built HTML pages.

Inserts a <nav class="editions"> element immediately after the body tag of
every page under --html: the current edition as a span, the others as links
to the same file name under each edition's site URL. An existing navigation
is replaced, never duplicated.
"""

import argparse
import re
import sys
from collections.abc import Sequence
from pathlib import Path

DEFAULT_EDITIONS = ("rust", "ocaml", "typescript", "kotlin")
DEFAULT_SITE_ROOT = "/modern-sicp"
BODY_PATTERN = re.compile(r"<body\b[^>]*>", re.IGNORECASE)
NAV_PATTERN = re.compile(r'<nav class="editions">.*?</nav>', re.DOTALL)


def build_nav(editions: Sequence[str], edition: str, page_name: str, site_root: str) -> str:
    """Render the navigation element for one page of one edition."""
    root = site_root.rstrip("/")
    items = []
    for name in editions:
        if name == edition:
            items.append(f'<span class="current">{name}</span>')
        else:
            items.append(f'<a href="{root}/{name}/{page_name}">{name}</a>')
    return '<nav class="editions">' + "".join(items) + "</nav>"


def switch_page(text: str, nav: str) -> str:
    """Drop any existing edition navigation and insert nav after the body tag."""
    text = NAV_PATTERN.sub("", text)
    return BODY_PATTERN.sub(lambda match: match.group(0) + nav, text, count=1)


def check_siblings(
    html_dir: Path,
    pages: list[Path],
    edition: str,
    editions: Sequence[str],
    siblings: Path,
) -> None:
    """Verify every other edition ships the same page names, or raise."""
    for page in pages:
        for name in editions:
            if name == edition:
                continue
            target = siblings / name / "html" / page.name
            if not target.is_file():
                raise ValueError(f"{html_dir / page.name}: missing sibling {target}")


def main(argv: Sequence[str] | None = None) -> int:
    """Add or refresh the edition navigation on every page under --html."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--html", type=Path, required=True, help="directory of built pages")
    parser.add_argument("--edition", required=True, help="the edition being built")
    parser.add_argument(
        "--editions",
        default=",".join(DEFAULT_EDITIONS),
        help="comma-separated edition names in nav order",
    )
    parser.add_argument("--site-root", default=DEFAULT_SITE_ROOT, help="URL root of the editions")
    parser.add_argument(
        "--siblings", type=Path, help="directory holding <edition>/html trees to verify"
    )
    args = parser.parse_args(argv)
    editions = [name.strip() for name in args.editions.split(",") if name.strip()]
    if not editions:
        print("--editions names no edition", file=sys.stderr)
        return 1
    if args.edition not in editions:
        parser.error(f"--edition {args.edition!r} is not listed in --editions {editions}")
    if not args.html.is_dir():
        print(f"{args.html}: not a directory", file=sys.stderr)
        return 1
    pages = sorted(args.html.rglob("*.html"))
    if not pages:
        print(f"{args.html}: no HTML pages", file=sys.stderr)
        return 1
    try:
        if args.siblings is not None:
            check_siblings(args.html, pages, args.edition, editions, args.siblings)
        for page in pages:
            nav = build_nav(editions, args.edition, page.name, args.site_root)
            text = page.read_text(encoding="utf-8")
            updated = switch_page(text, nav)
            if updated != text:
                page.write_text(updated, encoding="utf-8")
    except (OSError, ValueError) as exc:
        print(str(exc), file=sys.stderr)
        return 1
    else:
        print(f"pages={len(pages)}")
        return 0


if __name__ == "__main__":
    raise SystemExit(main())
