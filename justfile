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
