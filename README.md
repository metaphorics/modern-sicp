# Modern SICP

Four editions of "Structure and Interpretation of Computer Programs" by Abelson, Sussman, and Sussman, one each in Rust, OCaml, TypeScript, and Kotlin. Each edition keeps the book's section and exercise numbers, opens with a Chapter 0 primer that teaches the language subset the book uses, and ships every listing, exercise scaffold, and reference solution as code that runs under the language's test gate.

The text comes from the lineage of the [Unofficial Texinfo Format](https://www.neilvandyke.org/sicp-texi/) through the [HTML5 and EPUB3 edition](https://github.com/sarabander/sicp). See `NOTICE.md` for attribution and the changes made here.

The host-subset migration is in progress. Chapters 4 and 5 are being changed
to interpret and compile checked subsets of each edition's own language.
The [approved specification](docs/plan/host-subsets-specification.md) defines
the required behavior; the [execution plan](docs/plan/host-subsets-migration.md)
defines the work and verification order. Published editions are not evidence
that this migration has passed its gates.

Read online: https://metaphorics.github.io/modern-sicp/

## Layout

|Path|Holds|
|---|---|
|`text/original/figures/`|Shared SVG sources and their PDF counterparts|
|`spec/book-inventory.json`|The preserved section, exercise, figure, and reference inventory|
|`rust/`, `ocaml/`, `typescript/`, `kotlin/`|One edition each: `book/` (Texinfo), `examples/`, `exercises/`, `solutions/`|
|`spec/host-subsets/`|Four host-subset grammars and the preserved conformance case identities|
|`tools/`|Python build scripts, run with `uv`|
|`docs/`|The plan, the idiom companions, the exercise map, the toolchain pins|
|`site/`|The landing page|

## Building

Install the toolchains once with `just setup` (see `CONTRIBUTING.md` for the per-language steps), then:

```
just check     # format and lint every root and the tools
just test      # the four language gates and the tools tests
just books     # HTML, EPUB 3, and PDF for every edition
just scaffold  # list the unsolved exercise scaffolds
```

Each language root has the same six recipes: `setup`, `fmt`, `lint`, `test`, `scaffold`, `book`. A book build writes to `<lang>/book/_build/`:

```
<lang>/book/_build/html/index.html
<lang>/book/_build/<lang>-sicp.epub
<lang>/book/_build/<lang>-sicp.pdf
```

Building needs Texinfo 7.3, TeX Live with pdfTeX and tex4ht, Pygments,
librsvg, and epubcheck. `just setup-books` installs Texinfo, librsvg,
and epubcheck. Install the TeX programs and Pygments before running it;
the recipe checks for them but does not install them.

## Licenses

|Path|License|
|---|---|
|`text/`, `*/book/`, the figures|CC BY-SA 4.0|
|`*/examples/`, `*/exercises/`, `*/solutions/`, `*/appendix/`, `spec/`|GPL-3.0-only|
|`tools/`, `site/`, the justfiles, `.github/`|MIT|
|`text/assets/css/fonts/`|SIL OFL 1.1|
|`LICENSE.src`|The upstream build scripts' GPL-3.0 notice, retained|

The prose may be reused under CC BY-SA 4.0 and the code under GPL-3.0-only; the two never mix in one file. The scope map is `LICENSE`; the full texts are under `LICENSES/`; attribution is in `NOTICE.md`.
