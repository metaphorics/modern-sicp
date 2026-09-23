# Repository gates. CONTRIBUTING.md step 8 requires `just check`, `just test`
# and `just books` to pass before any change.
#
# Each edition exposes the same six recipes (setup, fmt, lint, test, scaffold,
# book); this file only fans out to them and adds the `tools/` gates. An
# edition that has not landed its justfile yet fails loudly here rather than
# being skipped, because a silently skipped edition reads as a passing repo.

editions := "rust ocaml typescript kotlin"

default:
    @just --list

# Formatting and linting across every edition and the Python tools.
check:
    #!/usr/bin/env bash
    set -euo pipefail
    for edition in {{editions}}; do
        just --justfile "$edition/justfile" --working-directory "$edition" fmt
        just --justfile "$edition/justfile" --working-directory "$edition" lint
    done
    just check-tools
    just check-corpus

# Tests across every edition and the Python tools.
test:
    #!/usr/bin/env bash
    set -euo pipefail
    for edition in {{editions}}; do
        just --justfile "$edition/justfile" --working-directory "$edition" test
    done
    just test-tools

# The pending scaffolds per edition; nonzero while one is unsolved, which is the report.
scaffold:
    #!/usr/bin/env bash
    for edition in {{editions}}; do
        just --justfile "$edition/justfile" --working-directory "$edition" scaffold
    done

# HTML, EPUB 3 and PDF for every edition.
books:
    #!/usr/bin/env bash
    set -euo pipefail
    for edition in {{editions}}; do
        just --justfile "$edition/justfile" --working-directory "$edition" book
    done

check-tools:
    uv run --project tools ruff format --check tools
    uv run --project tools ruff check tools
    uv run --project tools pyright tools

test-tools:
    uv run --project tools pytest tools/tests

# Re-runs every Scheme corpus program and compares it to its expected file.
check-corpus:
    uv run --project tools python tools/scheme_corpus_check.py --root spec/scheme-subset

# CONTRIBUTING.md states that this syncs tools/ and runs its tests.
setup-tools:
    uv sync --project tools
    just test-tools

setup-rust:
    just --justfile rust/justfile --working-directory rust setup

setup-ocaml:
    just --justfile ocaml/justfile --working-directory ocaml setup

setup-typescript:
    just --justfile typescript/justfile --working-directory typescript setup

setup-kotlin:
    just --justfile kotlin/justfile --working-directory kotlin setup

# The book toolchain (CONTRIBUTING step 6): Texinfo 7.3 from the GNU tarball,
# epubcheck 5.4.0, and rsvg-convert from the Debian package, all under one
# prefix. MODERN_SICP_PREFIX defaults to ~/.local; when ~/.local/bin is not
# writable, set it to a writable directory such as ~/.cache/modern-sicp/opt.
books-prefix := env_var_or_default("MODERN_SICP_PREFIX", env_var("HOME") / ".local")
texinfo-sha256 := "51f74eb0f51cfa9873b85264dfdd5d46e8957ec95b88f0fb762f63d9e164c72e"
epubcheck-sha256 := "33350c61038e71dfb3d45a76aed04bf5481e6d5500cb780f6e98db8bbd15a28c"
archive-zip-sha256 := "984e185d785baf6129c6e75f8eb44411745ac00bf6122fb1c8e822a3861ec650"
librsvg2-bin-version := "2.61.3+dfsg-3"

setup-books:
    #!/usr/bin/env bash
    set -euo pipefail
    prefix='{{books-prefix}}'
    mkdir -p "$prefix/bin" "$prefix/opt" "$prefix/src"

    if ! [ -x "$prefix/bin/texi2any" ] || ! "$prefix/bin/texi2any" --version | head -1 | grep -q '7\.3'; then
        cd "$prefix/src"
        curl -fsSLO https://ftp.gnu.org/gnu/texinfo/texinfo-7.3.tar.xz
        echo '{{texinfo-sha256}}  texinfo-7.3.tar.xz' | sha256sum -c -
        tar xf texinfo-7.3.tar.xz
        cd texinfo-7.3
        ./configure --prefix="$prefix"
        make -j"$(nproc)"
        make install
    fi

    if ! PERL5LIB="$prefix/lib/perl5" perl -MArchive::Zip -e 1 >/dev/null 2>&1; then
        cd "$prefix/src"
        curl -fsSLO https://www.cpan.org/modules/by-module/Archive/Archive-Zip-1.68.tar.gz
        echo '{{archive-zip-sha256}}  Archive-Zip-1.68.tar.gz' | sha256sum -c -
        tar xf Archive-Zip-1.68.tar.gz
        (cd Archive-Zip-1.68 && perl Makefile.PL INSTALL_BASE="$prefix" && make -j"$(nproc)" && make install)
    fi

    if ! [ -x "$prefix/bin/epubcheck" ]; then
        curl -fsSL -o "$prefix/src/epubcheck-5.4.0.zip" \
            https://github.com/w3c/epubcheck/releases/download/v5.4.0/epubcheck-5.4.0.zip
        (cd "$prefix/src" && echo '{{epubcheck-sha256}}  epubcheck-5.4.0.zip' | sha256sum -c -)
        unzip -q -o "$prefix/src/epubcheck-5.4.0.zip" -d "$prefix/opt"
        printf '#!/bin/sh\nexec java -jar %s/epubcheck.jar "$@"\n' \
            "$prefix/opt/epubcheck-5.4.0/epubcheck.jar" > "$prefix/bin/epubcheck"
        chmod +x "$prefix/bin/epubcheck"
    fi

    if ! [ -x "$prefix/bin/rsvg-convert" ]; then
        debdir=$(mktemp -d)
        (cd "$debdir" && apt-get download "librsvg2-bin={{librsvg2-bin-version}}")
        dpkg -x "$debdir"/librsvg2-bin_*.deb "$prefix/opt/librsvg2-bin"
        ln -sfn "$prefix/opt/librsvg2-bin/usr/bin/rsvg-convert" "$prefix/bin/rsvg-convert"
    fi

    "$prefix/bin/texi2any" --version | head -1
    "$prefix/bin/epubcheck" --version
    "$prefix/bin/rsvg-convert" --version
    for required in pdftex tex4ht htlatex pygmentize; do
        command -v "$required" >/dev/null 2>&1 || { echo "setup-books: $required is not on PATH" >&2; exit 1; }
    done

# Every toolchain this repository pins, in one command.
setup: setup-rust setup-ocaml setup-typescript setup-kotlin setup-books setup-tools
