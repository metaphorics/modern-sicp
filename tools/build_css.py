# SPDX-License-Identifier: MIT
"""Assemble the screen and EPUB stylesheets from shared CSS assets."""

import argparse
import re
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Final

DEFAULT_FONT_URL_PREFIX: Final[str] = "fonts/"
EPUB_HTML_RULE: Final[str] = "html { font-size: 100% }"
FIXED_POSITION_RE: Final[re.Pattern[str]] = re.compile(r"position\s*:\s*fixed\b")
URL_RE: Final[re.Pattern[str]] = re.compile(
    r"""url\(\s*(?:"""
    r"""(?P<single>'[^']*')|(?P<double>"[^"]*")|"""
    r"""(?P<bare>[^\s)]+))\s*\)"""
)


@dataclass(frozen=True)
class CssSegment:
    """One top-level CSS text, comment, statement, or brace-delimited block."""

    kind: str
    raw: str
    prelude: str = ""
    body: str = ""


def skip_string(css: str, start: int) -> int:
    """Return the first offset after the quoted string beginning at ``start``."""
    quote = css[start]
    index = start + 1
    while index < len(css):
        if css[index] == "\\":
            index += 2
            continue
        if css[index] == quote:
            return index + 1
        index += 1
    return len(css)


def matching_brace(css: str, opening: int) -> int:
    """Return the closing brace matching the opening brace at ``opening``."""
    depth = 0
    index = opening
    while index < len(css):
        character = css[index]
        if character in "'\"":
            index = skip_string(css, index)
            continue
        if css.startswith("/*", index):
            comment_end = css.find("*/", index + 2)
            index = len(css) if comment_end == -1 else comment_end + 2
            continue
        if character == "{":
            depth += 1
        elif character == "}":
            depth -= 1
            if depth == 0:
                return index
        index += 1
    return len(css) - 1


def parse_segments(css: str) -> list[CssSegment]:
    """Split CSS into segments while preserving each segment's original bytes."""
    segments: list[CssSegment] = []
    text_start = 0
    index = 0
    while index < len(css):
        character = css[index]
        if css.startswith("/*", index):
            comment_end = css.find("*/", index + 2)
            comment_end = len(css) if comment_end == -1 else comment_end + 2
            if index > text_start:
                segments.append(CssSegment("text", css[text_start:index]))
            segments.append(CssSegment("comment", css[index:comment_end]))
            index = comment_end
            text_start = index
        elif character in "'\"":
            index = skip_string(css, index)
        elif character == "{":
            closing = matching_brace(css, index)
            segments.append(
                CssSegment(
                    "block",
                    css[text_start : closing + 1],
                    css[text_start:index],
                    css[index + 1 : closing],
                )
            )
            index = closing + 1
            text_start = index
        elif character == ";":
            prelude = css[text_start:index]
            if prelude.lstrip().startswith("@"):
                segments.append(CssSegment("statement", css[text_start : index + 1], prelude))
                index += 1
                text_start = index
            else:
                index += 1
        else:
            index += 1
    if text_start < len(css):
        segments.append(CssSegment("text", css[text_start:]))
    return segments


def transform_css(
    css: str,
    *,
    strip_import: bool = False,
    strip_font_face: bool = False,
    strip_fixed_rules: bool = False,
) -> str:
    """Apply the requested removals while retaining all untouched CSS text."""
    output: list[str] = []
    dropped_previous = False
    for segment in parse_segments(css):
        drop = False
        raw = segment.raw
        if segment.kind == "statement":
            drop = strip_import and segment.prelude.lstrip().startswith("@import")
        elif segment.kind == "block":
            prelude = segment.prelude.strip()
            is_at_rule = prelude.startswith("@")
            if strip_font_face and prelude.startswith("@font-face"):
                drop = True
            elif is_at_rule and (strip_font_face or strip_fixed_rules):
                body = transform_css(
                    segment.body,
                    strip_import=strip_import,
                    strip_font_face=strip_font_face,
                    strip_fixed_rules=strip_fixed_rules,
                )
                raw = f"{segment.prelude}{{{body}}}"
            elif strip_fixed_rules and (
                ".jump" in prelude or FIXED_POSITION_RE.search(segment.body) is not None
            ):
                drop = True
        if drop:
            dropped_previous = True
            continue
        if dropped_previous and segment.kind == "text" and not segment.raw.strip():
            dropped_previous = False
            continue
        dropped_previous = False
        output.append(raw)
    return "".join(output)


def prefix_font_urls(css: str, prefix: str) -> str:
    """Prefix every local URL in an inlined ``fonts.css`` stylesheet."""

    def replace(match: re.Match[str]) -> str:
        """Build one URL while retaining its quote style."""
        single = match.group("single")
        double = match.group("double")
        if single is not None:
            return f"url('{prefix}{single[1:-1]}')"
        if double is not None:
            return f'url("{prefix}{double[1:-1]}")'
        bare = match.group("bare")
        if bare is None:
            raise ValueError("CSS URL has no target")
        return f"url({prefix}{bare})"

    return URL_RE.sub(replace, css)


def ensure_newline(text: str) -> str:
    """Return ``text`` with one terminating newline when it lacks one."""
    return text if text.endswith("\n") else f"{text}\n"


def build_stylesheets(assets: Path, font_url_prefix: str) -> tuple[str, str]:
    """Read source assets and return the assembled screen and EPUB CSS text."""
    style = (assets / "style.css").read_text(encoding="utf-8")
    fonts = (assets / "fonts" / "fonts.css").read_text(encoding="utf-8")
    highlight = (assets / "highlight.css").read_text(encoding="utf-8")

    book_css = "".join(
        (
            ensure_newline(prefix_font_urls(fonts, font_url_prefix)),
            ensure_newline(transform_css(style, strip_import=True)),
            ensure_newline(highlight),
        )
    )
    epub_style = transform_css(
        style,
        strip_import=True,
        strip_font_face=True,
        strip_fixed_rules=True,
    )
    epub_css = "".join(
        (
            f"{EPUB_HTML_RULE}\n\n",
            ensure_newline(epub_style),
            ensure_newline(highlight),
        )
    )
    return book_css, epub_css


def parse_args(argv: list[str] | None) -> argparse.Namespace:
    """Parse command-line arguments for the stylesheet builder."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--assets",
        type=Path,
        required=True,
        help="directory containing style.css, fonts/fonts.css, and highlight.css",
    )
    parser.add_argument(
        "--out",
        type=Path,
        required=True,
        help="directory receiving the two CSS files",
    )
    parser.add_argument(
        "--font-url-prefix",
        default=DEFAULT_FONT_URL_PREFIX,
        help="prefix inserted into fonts.css URLs (default: %(default)s)",
    )
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    """Build both stylesheets and print their byte counts."""
    args = parse_args(argv)
    try:
        book_css, epub_css = build_stylesheets(args.assets, args.font_url_prefix)
    except OSError as error:
        print(f"build_css: {error}", file=sys.stderr)
        return 1

    outputs = (("book.css", book_css), ("book-epub.css", epub_css))
    for name, content in outputs:
        if "@import" in content:
            print(f"build_css: @import survived in {name}", file=sys.stderr)
            return 1

    args.out.mkdir(parents=True, exist_ok=True)
    book_bytes = (args.out / "book.css").write_bytes(book_css.encode("utf-8"))
    epub_bytes = (args.out / "book-epub.css").write_bytes(epub_css.encode("utf-8"))
    print(f"book_css={book_bytes} epub_css={epub_bytes}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
