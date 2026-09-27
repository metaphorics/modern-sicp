# Contributing

Every version below is pinned in `docs/toolchain-pins.md`. The steps are executable on a fresh Linux machine without root; where a step needs root it says so.

## Bootstrap a fresh machine

1. Clone the repository and install `just` (https://just.systems).
2. Rust: install `rustup`; the checked-in `rust/rust-toolchain.toml` selects 1.98.1 with `rustfmt` and `clippy` on first use. Then `just setup-rust`, which installs `cargo-nextest` 0.9.146.
3. OCaml: install `opam` 2.6.0. In `ocaml/` run `opam switch create 5.5.1`, then `opam install . --deps-only --locked --with-dev-setup` (the lock file pins every library). Run every dune command through `opam exec --switch=5.5.1 --`, or select the switch in your shell.
4. TypeScript: install a Node version manager (`mise` or `nvm`) and run `mise install` or `nvm install` in `typescript/`; `.node-version` pins 24.21.0. Enable pnpm 12.5.1 with `corepack enable` or `npm i -g pnpm@12.5.1`. Then `just setup-typescript`, which runs `pnpm install --frozen-lockfile`.
5. Kotlin: install JDK 25 (Temurin or the distribution's OpenJDK) and export `JAVA_HOME`. No local Gradle: `./gradlew --version` in `kotlin/` downloads Gradle 9.7.0 through the committed wrapper. Then `just setup-kotlin`.
6. Books: `just setup-books` builds Texinfo 7.3 from the GNU tarball into the prefix `MODERN_SICP_PREFIX` (default `~/.local`; set it to a writable directory such as `~/.cache/modern-sicp/opt` when `~/.local/bin` is not writable), installs epubcheck 5.4.0 from its GitHub release and `rsvg-convert` from the Debian `librsvg2-bin` package into the same prefix, and checks that TeX Live (`pdftex`, `tex4ht`, `htlatex`) and Pygments are on PATH. TeX Live and Pygments come from the distribution (`texlive-latex-recommended texlive-fonts-recommended texlive-plain-generic tex4ht python3-pygments`); that step needs root. Put `<prefix>/bin` first on PATH.
7. Tools: install `uv` 0.12.17 (https://docs.astral.sh/uv/). `just setup-tools` syncs `tools/` and runs its tests.
8. Run `just check`, `just test`, and `just books`. All three must pass before any change.

## The repository gates

The root `justfile` fans out to the editions and adds the shared gates:

- `just check` runs each edition's `fmt` and `lint`, then `check-tools` (ruff format, ruff check and pyright over `tools/`) and `check-corpus`.
- `just test` runs each edition's tests, then the `tools/` pytest suite.
- `just books` builds HTML, EPUB 3 and PDF for each edition.
- `just scaffold` lists the pending scaffolds; it exits nonzero while one is unsolved, which is the report rather than a failure.

An edition whose `justfile` is absent fails these gates rather than being skipped, because a skipped edition reads as a passing repository.

Two gates guard the book build specifically. `tools/math_check.py` compares the math fragments `texi2any` requested against the MathML in the delivered pages, because `texi2any` exits 0 even when the TeX run dies and drops every equation. `tools/scheme_corpus_check.py` re-runs every Scheme corpus program through `spec/scheme-subset/runner.scm` and compares it to its expected file, so an expected file that no longer matches its program is caught.

`tools/texi2any_html.sh` prefers a prefix-installed Texinfo 7.3, checking `MODERN_SICP_PREFIX`, then `~/.local`, then the `~/.cache/modern-sicp/opt` fallback, so `just books` needs no manual PATH changes after `just setup-books`.

## One exercise, end to end

The unit of work is one section of one edition (`docs/plan/technical-modern-sicp-editions.md`, "Work breakdown"). Inside a section, each exercise has three artifacts (`docs/exercise-map.md`, "Policy"):

1. The statement lives in `<lang>/book/chN/N.M.texi` as the exercise's `@quotation` block with its `@anchor{Exercise N.M}`. An `R` row opens with the fixed replacement sentence; an addition opens with the fixed addition sentence.
2. The scaffold lives in `<lang>/exercises/chN/` with the signature and a test carrying the language's pending marker: `#[ignore = "pending solution"]` in Rust, the `scaffold` alias in OCaml, `test.todo` in TypeScript, the disabled Kotest case in Kotlin. `just scaffold` lists pending scaffolds; the language gate stays green while the scaffold is unsolved.
3. The solution lives in `<lang>/solutions/chN/` with the scaffold's test minus the marker, passing, and a rationale `ex_N_MM.md` beside it. A prose exercise (a diagram, a proof, a discussion) has only the rationale file and is marked `prose` in the map.

Then run the language gate (`just fmt`, `just lint`, `just test` from the language root), `uv run --project tools tools/exercise_map_check.py --lang <lang> --section N.M` from the repository root, and `just book` from the language root. Record any map change in the same commit as the code.

## Commits

One concern per commit: examples, prose, and exercises with solutions are three commits per section. Capitalized imperative subject at most 72 characters, no period; body wrapped at 72 explaining what and why. The layout rules for the build units are in `docs/decisions/0001-edition-unit-layout.md`.
