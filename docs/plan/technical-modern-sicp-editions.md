# Modern SICP in four languages

This plan defines the four editions and their shared build rules. The approved [host-subset specification](host-subsets-specification.md) and [migration plan](host-subsets-migration.md) govern the language cutover. The edition companions are `idiom-rust.md`, `idiom-ocaml.md`, `idiom-typescript.md`, and `idiom-kotlin.md`. Exercise policy is in `../exercise-map.md`. The completed campaign and current execution evidence are recorded in `../../.outline/sdd/progress.md`. The original draft reviews remain provenance records, not authority over the approved migration.

## The answer

Maintain four self-contained SICP editions: Rust 2024 on Rust 1.98.1; OCaml 5.5.1 with Stdlib in the main text and Base and Core appendixes; TypeScript 7.0.2 with Effect 4.0.0-rc.117 on Node 24 LTS; and Kotlin 2.4.20 on JDK 25 with Arrow 2.2.3. Each edition covers all five chapters and the 356 original exercises. Its Chapter 0 primer teaches the language features used by the book. Keep section and exercise numbers. Adapt closure and ownership models in 3.2, concurrency in 3.4, and search in 4.3 without dropping their lessons. Section 5.3 remains a memory simulation. Chapters 4 and 5 interpret and compile the edition's real, checked host-language subset. They share lesson identities, not a source grammar or dynamic value model. Keep listings, pending exercise scaffolds, and reference solutions in their separate edition trees. Texinfo 7.3 builds HTML, EPUB 3, and PDF; HTML and EPUB carry MathML, and HTML uses Pygments. Preserve CC BY-SA 4.0 text and figures, GPL-3.0-only teaching code, and MIT original tooling. Execute the approved migration through dependency-ordered edition units and parent-owned shared integration. Do not publish partially migrated editions. The earlier bootstrap and release campaign is complete; its evidence does not verify this migration.

## Context

The user asked for modern versions of SICP in four languages, each with language-tailored exercises, examples, and solutions, with examples and solutions in separate directories. Later instructions: Effect v4 is the TypeScript pin; the plan covers every detail of the build rather than a tracking map alone; the plan text carries no AI tells.

The repository derives from `sarabander/sicp`, the HTML5 and EPUB3 Texinfo edition. The original interview established four complete editions, a Chapter 0 primer for each, all five chapters, and permission to adapt lessons to each language. The approved migration replaces the shared guest language with four host subsets. Remove archived prose only after its consumers move. Preserve useful assets, attribution, and Git history.

Bound skills and what each one owns:

- wayfinder tracks the multi-session build as a map with tickets on GitHub issues.
- askme (batch mode) ran the interview.
- plan classified this document and requires the answer first and sourced numbers.
- ground-latest pinned every toolchain choice from its release channel on 2026-09-22.
- breaking-driven governs the removal of the old build machinery: the text and figures are the contract, the machinery is residue.
- idiomatic-rust, ocaml, code-style, typescript-best-practices, and typescript-type-hardening set the code conventions; unslop sets the prose conventions.
- autoplan reviews this plan (CEO, design, DX, engineering) and derives the task ledger before the approval gate.

## Decision register

| Id | Decision | Source |
|---|---|---|
| D1 | Four self-contained editions, one per language | Interview Q1 |
| D2 | The reader knows nothing of the language beyond what Chapter 0 teaches; Chapter 0 is about thirty pages per edition | Interview Q2 |
| D3 | All five chapters, 356 exercises, in every edition | Interview Q3 |
| D4 | Section and exercise numbers stay 1:1 with SICP; prose may be re-cut where the language changes the lesson; a re-cut that changes exercise numbering must be recorded in the exercise map | Interview Q4 |
| D5 | OCaml main text uses Stdlib; each chapter ends with a Base and Core appendix in a fixed six-subsection format | Default Q5, OCaml companion |
| D6 | The original campaign used GitHub issues on `metaphorics/modern-sicp`. The approved migration plan and `.outline/sdd/progress.md` track the current cutover. | Original interview and approved migration plan |
| D7 | Monorepo; each language root holds `book/`, `examples/`, `exercises/`, `solutions/` | Default Q7 and the literal ask |
| D8 | Text, figures, and exercise statements (`text/`, `*/book/`) are CC BY-SA 4.0; `*/examples/`, `*/exercises/`, `*/solutions/` are GPL-3.0-only, the BY-SA Compatible License Creative Commons declared on 2015-10-08 (Section 3(b)(1)); `tools/` is MIT; `LICENSE.src` is retained | Default Q8 amended by the CEO review: Creative Commons advises against its licenses for software, and SICP JS uses the same split |
| D9 | Chapters 4 and 5 interpret and compile each edition's documented, type-checked host subset. Native execution and every teaching engine must agree on supported core observations. | Approved host-subset specification |
| D10 | TypeScript runs on Node 24 LTS with pnpm; Bun is not used | Default Q10 and grounding |
| D11 | Kotlin uses Arrow 2.2.3 (`arrow-core`, `arrow-fx-coroutines`) | Default Q11 |
| D12 | Edition roots may proceed in parallel. Within each edition, source checking and runtime contracts precede evaluators, machines, compilers, and dependent teaching consumers. The parent owns shared integration, gates, and commits. | Approved migration plan |
| D13 | Effect 4.0.0-rc.117 is pinned exactly although it is a release candidate | User interjection |
| D14 | Rust uses no numeric crates. Earlier chapters build checked `i128` arithmetic and their rational abstractions. The chapter-4/5 guest has the separate numeric contract in `spec/host-subsets/rust/grammar.md`; do not inherit the earlier numeric tower into it. | Original numeric policy, scoped by the approved Rust grammar |
| D15 | Kotlin drops detekt (stale stable line); ktlint plus `allWarningsAsErrors` and `explicitApi()` form the lint gate | Grounding |
| D16 | Gradle is pinned to 9.7.0, inside the Kotlin Gradle plugin's tested window, instead of 9.7.1 | Grounding compatibility rule |
| D17 | OCaml pins 5.5.1 (current stable); the machine switch moves from 5.4.0; Zarith is not used anywhere, bounded `int` with stated bounds replaces it | Grounding, OCaml companion |
| D18 | Chapter 3 builds a memoized stream abstraction in each edition. Memoization is the default; exercises 3.50 to 3.52 expose it. Unmemoized thunks appear only in named probes. Cold host sequences are comparisons, not substitutes for this abstraction. Chapter 4 lazy, search, and query experiments follow their separate approved contracts. | Prior art, fidelity, engineering review, and approved host-subset specification |
| D19 | The `put` and `get` operation table stays a real table in every edition, keyed by the operation name plus the ordered list of type tags; `get` on a missing key returns the absent option; `put` on an existing key overwrites; handlers return the edition's typed error, never a raw value; typed alternatives appear as closing notes, appendixes, and tailored exercises | Prior art lesson 6, all companions, engineering review |
| D20 | Use explicit host data types for lists, mutable pairs, syntax, and environments. Keep their distinct empty states and identity rules. Each guest runtime follows its own grammar; no shared dynamic value representation may bypass source typing or ownership. | Approved host-subset specification |
| D21 | Texinfo 7.3 builds HTML, EPUB 3, and PDF. HTML and EPUB use MathML through tex4ht. HTML alone uses Pygments; its markup is invalid inside EPUB example blocks. `tools/epub_manifest_fix.py` repairs SVG media types and MathML properties. `epubcheck` is mandatory. | Verified book pipeline |
| D22 | Repository tooling under `tools/` is Python 3.14 run with `uv`, standard library only, typed, tested with pytest | Installed `uv` 0.12.17, AGENTS.md Python rules |
| D23 | Each edition supplies one checked host-source interface shared by its evaluators and compiler. Preserve all case identities in `spec/host-subsets/cases.json`. Core expectations come from checked native execution; named experiments use independent finite models or reviewed derivations. The translated evaluator must itself be valid guest source and run through the teaching evaluator and compiled machine for the self-interpretation lessons. | Approved host-subset specification |
| D24 | Keep every original exercise number. Record changed semantic premises as `A` reasons or same-number `R` replacements in the exercise map. Tailored additions retain their separate suffixes. | Approved host-subset specification and exercise policy |
| D25 | Section 4.3 teaches named search experiments through each edition's host mechanisms. It must not claim these experiments are native core-language semantics. Preserve exercises 4.35 to 4.54, search order, state restoration, and resumable alternatives. | Approved host-subset specification |
| D26 | The original campaign tracked chapter tickets and landed section-sized commits. The host-subset migration uses the units and dependency order in the approved migration plan. Each commit contains one verified concern. | Original CEO review and approved migration plan |
| D27 | The original incremental publication campaign is complete. This migration requires all edition, conformance, book, and ordered audit gates before publication. Remote publication needs separate authority. | Approved migration plan |
| D28 | Every exercise scaffold in `exercises/` carries the language's pending marker, so the language gate stays green while a scaffold is unsolved; `just scaffold` surfaces pending work and never runs in CI | DX review |
| D29 | Interaction display: one `@example <language>` block per interaction; results are comment lines with a `=>` marker (`// => 441` in Rust, TypeScript, and Kotlin; `(* => 441 *)` in OCaml); the same value is asserted in `examples/` | Design review |
| D30 | At most one tailored addition per section per edition, chosen at unit start from the map's idea column; `R` replacements and additions open with the fixed signpost sentences | CEO and design reviews |
| D31 | No `rand` crate and no host PRNG in the book's code: every runtime ships a seeded xorshift64* `random`, so 1.24 and the Monte Carlo sections are deterministic under test | Engineering review, D14 |
| D32 | `parallel-execute` returns a handle whose `halt` sets a shared flag that workers poll, in all four editions, so exercise 3.48 onward reads the same in every edition | Engineering review |
| D33 | Chapters 1 and 2 of the TypeScript edition import no Effect module; Effect enters at 3.1 under its own names, with no adapter layer; an rc bump happens only at a chapter boundary in one commit that touches `examples/`, the lockfile, and the companion | User interjection, CEO review (adapter rejected) |
| D34 | Every GitHub Action is pinned to an immutable commit SHA with its release tag recorded in `docs/toolchain-pins.md`; every install is frozen (`--locked`, `--frozen-lockfile`, the opam lock file, the Gradle lockfile) | DX and engineering reviews |
| D35 | The site is the project Pages site `https://metaphorics.github.io/modern-sicp/` with the editions at `/rust/`, `/ocaml/`, `/typescript/`, `/kotlin/`; no custom domain; the EPUB has no cover beyond the generated title page; the PDF uses the `texinfo.tex` defaults (Computer Modern text, typewriter code, letter paper) and no syntax highlighting | CEO open questions, decided here |
| D36 | The fixture matrix at A3 settles `T4H_MATH_CONVERSION` and `T4H_TEX_CONVERSION` (`tex` by default; `latex` if the source's `@math` needs it); the chosen values are recorded in `docs/toolchain-pins.md` and used in every build command | Engineering review, Texinfo manual |

## Original repository snapshot (2026-09-22)

- Path `/home/alpha/book/modern-sicp`, origin `https://github.com/metaphorics/modern-sicp`, branch `master`, clean tree.
- Fork state: local HEAD `bda03f7` equals the upstream `master` head; 101 commits, none of them ours; GitHub Pages is not enabled (the Pages API returns 404); no `plans/`, `docs/`, or `.outline/` directory exists.
- Text: `sicp-pocket.texi`, 37,952 lines of Texinfo. The text and the `html/fig` illustrations are CC BY-SA 4.0 (`LICENSE`, `README.md`). The build scripts are GPL-3.0 (`LICENSE.src`). The fonts under `html/css/fonts` are SIL OFL 1.1 (Libertine, Biolinum, Inconsolata, DejaVu, STIX) except jsMath, which is GPL; `style.css` line 1 is `@import url(fonts/fonts.css)`.
- Exercise statements are `@quotation` blocks that begin `@strong{@anchor{Exercise N.M}Exercise N.M:}`; listings are `@lisp … @end lisp`; interpreter results appear as `@i{…}` lines after the expression; inline math is `@math{…}` (845 lines) and displayed math is 82 raw `@tex` blocks (`\[ … \]`, some with `eqnarray` or `array`), no `@displaymath`; 93 `@anchor{Figure N.M}` anchors, of which 84 sit in bare `@float` blocks (never `@float Figure`) that carry an `@ifinfo` ASCII fallback and the `@image` inside `@iftex` only, and 9 in chapter 5 are `@quotation` blocks with no image; no `@ifhtml` exists anywhere, and the old Makefile passes `--iftex` to get figures into HTML; 87 `@image` lines (84 figures, 3 license icons); 339 `@footnote` lines; the term index uses `@cindex`; `exercises.texi` and `figures.texi` are generated global nodes included under Top at lines 37923 and 37928.
- Section start lines: 1.1 at 1107, 1.2 at 2630, 1.3 at 4192, 2.1 at 5905, 2.2 at 6781, 2.3 at 9608, 2.4 at 11234, 2.5 at 12310, 3.1 at 14039, 3.2 at 15026, 3.3 at 15999, 3.4 at 18791, 3.5 at 19855, 4.1 at 22672, 4.2 at 24872, 4.3 at 25622, 4.4 at 27116, 5.1 at 30358, 5.2 at 31562, 5.3 at 32786, 5.4 at 33544, 5.5 at 34641.
- Old machinery: `Makefile` runs a vendored, patched Texinfo 5.1 `texi2any` (`texi2any`, `lib/Texinfo/Convert/HTML.pm`) to produce `html/*.xhtml`; `get-math.js`, `put-math.js`, and `mathcell.xhtml` drive MathJax under PhantomJS to turn LaTeX into MathML; `batch-prettify.js` with `prettify.js` and `lang-lisp.js` highlights Scheme; `ex-fig-ref.pl` generates `exercises.texi` and `figures.texi` from hard-coded per-chapter counts (46, 97, 82, 79, 52 exercises; 5, 26, 38, 6, 18 figures); `create_metafiles.rb` (Nokogiri) writes `content.opf` and `toc.xhtml`; `svg_cleanup.rb` normalizes figures; a zip step produces `../sicp.epub`.
- Installed: rustc and cargo 1.98.1; OCaml 5.4.0, opam 2.6.0, dune 3.24.2; Node v26.10.0, Bun 1.4.0, pnpm 12.5.1, no `tsc`; JDK 25, 21, and 17 under `/usr/lib/jvm` with 25 as default (the `java` on PATH reports 27 from a separate install); no Gradle; Texinfo 7.2 (`texi2any`, `texi2pdf`, `/usr/share/texi2any/ext/epub3.pm`); TeX Live 2025 with pdfTeX and tex4ht; `pygmentize`; Guile and Racket; Python 3.14.7 and `uv` 0.12.17; `just` 1.58.0; `gh` 2.101.0, authenticated. Not installed: `rsvg-convert`, `epubcheck`.

## Book inventory

| Chapter | Title | Sections and exercise ranges | Exercises |
|---|---|---|---|
| 1 | Building Abstractions with Procedures | 1.1 (1.1 to 1.8), 1.2 (1.9 to 1.28), 1.3 (1.29 to 1.46) | 46 |
| 2 | Building Abstractions with Data | 2.1 (2.1 to 2.16), 2.2 (2.17 to 2.52), 2.3 (2.53 to 2.72), 2.4 (2.73 to 2.76), 2.5 (2.77 to 2.97) | 97 |
| 3 | Modularity, Objects, and State | 3.1 (3.1 to 3.8), 3.2 (3.9 to 3.11), 3.3 (3.12 to 3.37), 3.4 (3.38 to 3.49), 3.5 (3.50 to 3.82) | 82 |
| 4 | Metalinguistic Abstraction | 4.1 (4.1 to 4.24), 4.2 (4.25 to 4.34), 4.3 (4.35 to 4.54), 4.4 (4.55 to 4.79) | 79 |
| 5 | Computing with Register Machines | 5.1 (5.1 to 5.6), 5.2 (5.7 to 5.19), 5.3 (5.20 to 5.22), 5.4 (5.23 to 5.30), 5.5 (5.31 to 5.52) | 52 |

Totals: 22 sections, 356 exercises (225 in chapters 1 to 3, 131 in chapters 4 and 5), 93 numbered figures (5, 26, 38, 6, 18 per chapter) of which 84 have SVG art and 9 in chapter 5 are text-only. SVG assets under `html/fig/`: 92 files, of which 85 are figure files (84 referenced plus the unreferenced `chap2/Fig2.23a.std.svg`), 6 are icons (3 referenced), and 1 is the cover. Front and back matter: Foreword, two Prefaces, Acknowledgments, References, Term Index, List of Exercises, List of Figures, UTF appendix. All five chapter ranges are verified against the exercise map (V3).

## Toolchain pins

Every pin below was read at its release channel on 2026-09-22; `docs/toolchain-pins.md` carries this table with the source links. Rule applied: latest stable LTS where the project runs an LTS track, else latest stable; a compatibility window or a user decision overrides and is named.

### Rust

| Entry | Pin | Released | Note |
|---|---|---|---|
| rustc, cargo | 1.98.1 stable | 2026-09-03 | No LTS track; machine matches; `rust-toolchain.toml` pins the channel with `rustfmt` and `clippy` |
| edition | 2024 | stable since 1.85.0 | Newest edition; `rust-version = "1.98"` |
| cargo-nextest | 0.9.146 | 2026-09-21 | Not a rustup component: installed with `cargo install cargo-nextest --locked --version 0.9.146` by `just setup-rust` and by the CI `rust` job |
| thiserror | 2.0.20 | 2026-08-08 | Library error types |
| anyhow | 1.0.104 | 2026-07-18 | Binaries only |
| proptest | 1.11.0 | 2026-03-24 | Property tests |
| insta | 1.48.0 | 2026-06-11 | Snapshots of interpreter transcripts and machine traces |
| num-bigint, num-rational, num-traits | dropped | last releases 2024 | Stale beyond the 12-month rule and not needed (D14) |

### OCaml

| Entry | Pin | Released | Note |
|---|---|---|---|
| OCaml | 5.5.1 | 2026-09-05 | Current stable; the opam entry `5.6.0` is a placeholder without a base compiler; `opam switch create 5.5.1` |
| opam | 2.6.0 | 2026-09-17 | Machine matches |
| dune | 3.24.2 | 2026-08-03 | `(lang dune 3.24)` on the first line of `dune-project` |
| base, core | v0.17.3, v0.17.2 | 2025-06-13, 2026-03-26 | Appendix only; the v0.17 series is the shipped series |
| ppx_jane, ppx_expect, ppx_inline_test | v0.17.0, v0.17.3, v0.17.1 | 2024 to 2025 | Appendix tests |
| alcotest | 1.9.1 | 2025-10-01 | Main-text tests |
| qcheck-core | 0.91 | 2025-12-28 | Property tests; the `qcheck` compat package is not used |
| ocamlformat | 0.29.0 | 2026-03-17 | `.ocamlformat`: `profile = janestreet`, `version = 0.29.0` |
| odoc | 3.2.1 | 2026-05-12 | Requires OCaml below 5.6 |
| ocaml-lsp-server | 1.27.0 | 2026-06-23 | The `1.28.0-506~preview` entry is a pre-release |
| zarith | dropped | last release 2024-07-15 | Stale and not needed (D17) |

### TypeScript

| Entry | Pin | Released | Note |
|---|---|---|---|
| typescript | 7.0.2 | 2026-07-08 | GA; `@typescript/native-preview` is retired |
| effect | 4.0.0-rc.117 | 2026-09-21 | Release candidate pinned by user decision (D13); the stable line is 3.22.2 |
| @effect/vitest | 4.0.0-rc.117 | 2026-09-21 | Peers `effect ^4.0.0-rc.117` and `vitest >=5.0.0 <6.0.0` |
| vitest | 5.0.1 | 2026-09-15 | Engines `^22.12.0 || ^24.0.0 || >=26.0.0` |
| @effect/platform-node | 4.0.0-rc.117 | 2026-09-21 | The `rc` dist-tag read on 2026-09-22; pinned exactly like `effect` |
| @biomejs/biome | 2.5.14 | 2026-09-16 | Lint and format |
| Node.js | 24.21.0 Active LTS | 2026-09-08, EOL 2028-04-30 | Node 26 is Current until 2026-10-28; `.node-version` and `engines.node` carry `24.21.0`; installed with a version manager |
| pnpm | 12.5.1 | 2026-09-18 | `packageManager` field; machine matches |
| @types/node | highest 24.x | resolved at A2 | `npm view @types/node versions --json`, take the highest `24.x`; the value and date go into `docs/toolchain-pins.md` |
| fast-check | latest stable | resolved at A2 | `npm view fast-check version`; property tests; Effect 4's `Arbitrary` lives under `effect/unstable` and is not used |

### Kotlin

| Entry | Pin | Released | Note |
|---|---|---|---|
| Kotlin | 2.4.20 | 2026-09-07 | Latest stable; K2 |
| JDK | 25 LTS (Temurin 25.0.4.1+1) | 2026-08-19 | `jvmToolchain(25)` with the foojay resolver convention; Gradle runs on JVM 17 to 26 only |
| Gradle | 9.7.0 | 9.7 line, 2026-08 | Inside the Kotlin Gradle plugin's tested window (7.6.3 to 9.7.0); pinned by `gradle-wrapper.properties` (D16) |
| arrow-core, arrow-fx-coroutines | 2.2.3 | 2026-06-04 | `2.3.0-alpha.4` is a pre-release |
| kotlinx.coroutines | 1.11.0 | 2026-05-07 | |
| kotlinx.collections.immutable | 0.5.2 | 2026-08-28 | Persistent lists and maps |
| Kotest | 6.2.5 | 2026-09-10 | `kotest-runner-junit5`, assertions, property testing |
| ktlint | 1.8.0 with ktlint-gradle 14.2.0 | 2025-11-14, 2026-03-12 | Style gate |
| detekt | dropped | 1.23.8 from 2025-02-21 | Stale beyond the 12-month rule; only 2.0 alphas exist (D15) |

### Book build

| Entry | Pin | Released | Note |
|---|---|---|---|
| Texinfo | 7.3 | 2026-03-02 (info-gnu announcement) | `texi2any --html`, `texi2any --epub3` (EPUB 3.3), `texi2pdf`; `HIGHLIGHT_SYNTAX` left experimental status in 7.3; machine has 7.2, so `just setup-books` builds 7.3 from `ftp.gnu.org/gnu/texinfo/texinfo-7.3.tar.xz` into `~/.local` (CI: `/opt/texinfo`, cached by tarball SHA256); `T4H_MATH_CONVERSION` and `T4H_TEX_CONVERSION` take the values the A3 fixture matrix settles (D36) |
| TeX Live | 2025 (installed) | | pdfTeX for `texi2pdf`; tex4ht for `HTML_MATH=t4h` |
| Pygments | recorded in `docs/toolchain-pins.md` | | `pygmentize --version`; lexers `rust`, `ocaml`, `typescript`, `kotlin`; HTML only |
| librsvg (`rsvg-convert`) | resolved at A2 | | SVG to PDF for the PDF build's figures; `librsvg2-bin` from apt; not installed on the machine |
| Python, uv | 3.14.7, 0.12.17 (installed) | | `tools/` scripts (D22) |
| epubcheck | resolved at A2 | | `epubcheck --version`; the `epubcheck` apt package; mandatory in `just books` and in CI, absence fails the build |
| GitHub Actions | commit SHAs resolved at A5 | | `actions/checkout`, `Swatinem/rust-cache`, `ocaml/setup-ocaml`, `pnpm/action-setup`, `actions/setup-node`, `actions/setup-java`, `gradle/actions/setup-gradle`, `astral-sh/setup-uv`, `actions/cache`, `actions/upload-pages-artifact`, `actions/deploy-pages`; each pinned to a commit SHA with the tag beside it (D34) |
| mdBook 0.5.4 with mdbook-katex 0.10.0 | dropped | | Needs a lossy Texinfo conversion and leaves EPUB math unsolved |
| Typst | dropped | | No EPUB output |
| Quarto, Pandoc pipeline | dropped | | Pandoc has no Texinfo reader; the only path is `texi2any --docbook` then `pandoc -f docbook`, lossy for anchors |
| PhantomJS, vendored Texinfo 5.1, prettify, Nokogiri scripts | dropped | PhantomJS unmaintained since 2018 | Replaced by stock Texinfo 7.3 features |

## Repository layout

```
modern-sicp/
  README.md                 project overview, build commands, output paths, license map
  CONTRIBUTING.md           fresh-machine bootstrap and the one-exercise workflow
  AGENTS.md                 pointer: conventions in docs/plan/, gates in the justfiles
  LICENSE                   scope map by path (no full text)
  LICENSES/                 CC-BY-SA-4.0.txt, GPL-3.0-only.txt, MIT.txt
  NOTICE.md                 attribution notice, modification statement, adapter's license URIs
  LICENSE.src               retained upstream notice, unchanged
  justfile                  root recipes: setup, check, test, books, scaffold, site
  .github/workflows/ci.yml  jobs: rust, ocaml, typescript, kotlin, tools, books-html-epub, books-pdf, pages
  text/original/figures/              the 92 SVG assets (from html/fig) and pdf/ (generated by tools/figures_pdf.py)
  text/assets/css/                    style.css, prettify.css, fonts.css, fonts/, OFL-1.1.txt, LICENCE.txt (from html/css, jsMath removed); highlight.css (new)
  spec/book-inventory.json            preserved pre-migration section, exercise, figure and reference inventory
  spec/host-subsets/                  shared lesson identities and four edition-owned grammars, sources and provenance
  site/                               landing page source (index.html, site.css) deployed beside the four editions
  rust/       rust-toolchain.toml, Cargo.toml, Cargo.lock, justfile, book/, examples/, exercises/, solutions/
  ocaml/      dune-project, .ocamlformat, modern-sicp.opam, modern-sicp.opam.locked, justfile, book/, examples/, exercises/, solutions/, appendix/
  typescript/ package.json, pnpm-lock.yaml, pnpm-workspace.yaml, tsconfig.base.json, vitest.config.ts, biome.json, justfile, book/, examples/, exercises/, solutions/
  kotlin/     settings.gradle.kts, build.gradle.kts, gradle/ (wrapper, libs.versions.toml, lockfiles), justfile, book/, examples/, exercises/, solutions/
  tools/      MIT, Python: book_structure_check.py, texi_indexes.py, figures_pdf.py, exercise_map_check.py, build_css.py, check_html_assets.py, epub_manifest_fix.py, edition_switcher.py, closing_notes_check.py, tests/
  docs/       plan/ (this plan, the idiom companions, wayfinder-map.md, reviews/), exercise-map.md, toolchain-pins.md, decisions/
  tasks/      autoplan-ledger.jsonl
```

Rules that hold across all four language roots:

- `book/` is an edition-owned Texinfo tree: `main.texi`, Chapter 0, the numbered chapter sections, front matter, references, and generated exercise and figure indexes. `tools/texi_indexes.py` owns those indexes. `tools/build_css.py` owns `book.css` and `book-epub.css`. Builds land in the ignored `book/_build/` directory. Do not recreate an edition from the removed archive.
- `examples/` holds every listing that appears in the text, one module per section, with the printed outputs asserted in tests; the text never shows code that is not in `examples/`.
- `exercises/` holds one pending scaffold per exercise: the signature, a test with the language's pending marker (D28), and a doc comment naming the exercise and its section. The statement lives only in `book/`.
- `solutions/` holds one reference solution per exercise with passing tests and a short rationale file; the solution's test is the scaffold's test with the marker removed.
- A file for exercise N.M is named with a zero-padded number (`ex_1_03`, `ex_2_20a`) so listings sort in book order.
- The language-internal layout (crates, dune libraries, pnpm packages, Gradle subprojects) follows the edition's idiom companion: Rust has one crate per chapter plus `sicp-runtime`; OCaml has one library per chapter plus `common` and `appendix`; TypeScript has one package per chapter under `packages/`; Kotlin has one subproject per chapter. Each root's `justfile` has exactly `setup`, `fmt`, `lint`, `test`, `scaffold`, `book`.

## License and attribution

Findings read from the CC BY-SA 4.0 legal code on 2026-09-22: rewriting the prose produces Adapted Material (Section 1(a)); the Adapter's License must be CC BY-SA 4.0 or later or a BY-SA Compatible License (Section 3(b)(1)); the compatible list holds only Free Art License 1.3 and GPLv3 (one way, version 3 only); Creative Commons' FAQ treats translation as generally an adaptation, and Source Academy licenses its translated SICP JS programs under GPLv3 while its prose stays CC BY-SA. The prudent posture: every listing that corresponds to a program printed in the book is Adapted Material.

Decisions (D8):

- Root `LICENSE` is a scope map: `text/`, `*/book/`, and the figures are CC BY-SA 4.0; `*/examples/`, `*/exercises/`, `*/solutions/` are GPL-3.0-only, applied as the BY-SA Compatible License that Section 3(b)(1) permits for Adapted Material (translated listings) and chosen for original code too so each tree has one license; `tools/` and `site/` are MIT; the fonts stay SIL OFL 1.1 and the GPL jsMath font is removed because MathML replaces its only use; `LICENSE.src` is retained unchanged as the prior-modification record. GPL-3.0-only, not or-later, because the compatibility declaration names version 3.
- `NOTICE.md` carries the notice in Creative Commons' recommended form: the title; the authors (Harold Abelson and Gerald Jay Sussman with Julie Sussman); the MIT Press license statement; the lineage (MIT Press HTML, Neil Van Dyke's Unofficial Texinfo Format, sarabander); the changes made (prose rewritten for four languages, programs re-expressed, figures re-rendered); the CC BY-SA 4.0 URI; the warranty disclaimer reference (Section 5); the adapter's license URIs for prose and for code.
- Every edition README and every code directory README carries a one-line pointer to `NOTICE.md`. Prose files carry no headers. Every code file carries a two-line header: `SPDX-License-Identifier: GPL-3.0-only` and either `Adapted from the Scheme program <name> in SICP section <n.m>` or `Original exercise` (Chapter 0 exercises, tailored additions, runtime code). Original tooling under `tools/` and `site/` carries `SPDX-License-Identifier: MIT`.
- The README license map and `NOTICE.md` say in one sentence that the prose may be reused under CC BY-SA 4.0 and the code under GPL-3.0-only, and that the two never mix in one file.

## Text pipeline and demolition

The old machinery is interior (no consumers outside this project) and is cut once the new build reproduces the essentials. Contract: the text with its structure and cross-references, exercise statements and numbers, the 93 numbered figures with captions, the displayed and inline math, footnotes, the term index, the references, and the attribution chain. Essentials the new build reproduces: section pages with cross-references, highlighted code blocks, MathML math in HTML and EPUB, figures with captions and a figure list in all three formats, exercises with anchors and an exercise list, footnotes, the term index, references, EPUB 3 packaging that passes `epubcheck`, PDF, and responsive HTML with the embedded fonts.

Migration requirements:

1. Keep useful assets under `text/original/figures/` and `text/assets/css/`. Rewrite teaching labels without losing values, graph edges, evaluation steps, or legal metadata. Regenerate the PDF mirror after SVG changes. Use edition-specific figures when a shared diagram would misstate host semantics.
2. Keep all numbered sections, exercises, and figures in the edition sources. `tools/texi_indexes.py` regenerates the exercise and figure indexes from their anchors. An addition follows its base exercise. Do not hand-edit generated indexes.
3. `tools/book_structure_check.py` compares exact section, exercise, and figure identities with `spec/book-inventory.json`. It invokes Texinfo 7.3, rejects conversion and reference errors, and checks that every required target occurs in the rendered HTML. Do not regenerate the baseline to accept a lost lesson. The old byte-for-byte split/join contract ends with its transcription tool.
4. Build through each edition's `book` recipe. Keep section pages, cross-references, captions, alternative text, footnotes, the term index, and references. HTML uses MathML and Pygments. EPUB uses MathML without Pygments, repaired media types and MathML properties, and mandatory `epubcheck`. PDF uses converted figures and the existing plain-TeX math pipeline. Run asset and math checks; inspect rendered pages as well as command exits.
5. Each executable listing uses its edition's source language. Quotation and symbolic-computation lessons use explicit syntax data, not embedded old source. Preserve conceptual machine and query notation without claiming it is native host syntax.
6. Replace all corpus and archive consumers before removing obsolete text, parsers, runners, dependencies, and split/parity tools. Preserve licenses, useful figures, and Git history. Do not delete a mixed directory wholesale. The ordered migration plan defines the removal and verification barriers.

## Edition design

### Shared decisions across the four editions

- Chapter-4/5 source contracts (D9): use `spec/host-subsets/<edition>/grammar.md`. Each edition owns its grammar, source typing, values, diagnostics, and native observation protocol. Core programs run through direct evaluation, analysis, explicit control, and compilation. Compare them with checked native execution. Named lazy/search experiments use independent models. Queries and machines are typed host domain data. Preserve every identity in `spec/host-subsets/cases.json`, including the evaluator executed as guest source.
- Checked source interface (D23): one parser and type contract serves all execution engines within an edition. Supplied parsing code may be explained in an appendix. Reject invalid syntax and types before any guest effect. Report host-valid excluded constructs as unsupported; a late runtime failure is not source-type agreement.
- Pairs and empty states (D20): use the host's immutable data in early chapters and explicit mutable structures where aliasing is the lesson. Keep list emptiness, optional environments, and initialization states distinct. Identity comparisons follow the declared reference type; they are not a universal equality rule.
- Recursive bindings: enforce the host's initialization and recursive-definition restrictions. Represent an uninitialized binding explicitly where required. Never substitute a default value or execute a host-invalid recursive program.
- Streams (D18): memoized host thunks retain the demand-driven lesson in 3.5.1. A stream constructor receives its tail as a thunk. Compare cold host sequences without replacing the book's stream model. Chapter 4's altered evaluation policies are named experiments, not changes to core host semantics.
- Data-directed dispatch (D19): `put` and `get` on a table keyed by the operation name plus the ordered list of type tags; `get` on a missing key returns the absent option, never `false`; `put` overwrites; handlers return the edition's typed error; `apply-generic` raises it.
- Numbers: earlier chapters retain their stated fixed-width, floating-point, and rational abstractions: checked Rust `i128` and `f64`; OCaml `int` and `float`; TypeScript `number` and explicit `bigint` lessons; Kotlin checked arithmetic abstractions and `BigInteger` rationals. Chapters 4 and 5 instead obey each accepted guest grammar's native numeric rules. Do not impose one numeric tower on all guests. Keep the overflow demonstrations in exercises 1.25 and 1.26 and seeded xorshift64* experiments (D31).
- Tail calls: OCaml guarantees them; Kotlin uses `tailrec`; Rust and TypeScript express iterative processes as loops and say so in 1.2; the chapter 4 evaluators in Rust and TypeScript say that they are not tail recursive, as SICP JS does, and 5.4 restores tail recursion in the explicit-control evaluator.
- Errors: SICP's `error` becomes the edition's typed error: `Result<T, E>` with `thiserror`, `result` with a variant, Effect's error channel with `Schema.TaggedError` (a plain `Result` in chapters 1 and 2), Arrow `Either` with `Raise`. The rule holds in chapter 5: `Machine.run` returns the typed error for an absent operation or an invalid instruction, and the corpus has a case for each.
- Concurrency (3.4, re-cut): Rust `std::thread::scope`, `Arc<Mutex>`, `AtomicBool::compare_exchange`, `Condvar`; OCaml `Domain.spawn` and `Mutex.protect`; TypeScript Effect fibers, `Effect.all` with concurrency, `Semaphore`, and `Effect.tx` with `TxRef`; Kotlin structured coroutines, `Mutex`, `AtomicBoolean.compareAndSet`, a token channel for the 3.47 semaphore. The serializer is one mutex around a function in all four; `parallel-execute` returns a handle whose `halt` sets a shared flag the workers poll (D32); exercises 3.38 to 3.49 keep their numbers.
- Environment model (3.2, re-cut): each edition draws frames for its own closure model; Rust adds ownership and borrowing of captured state; exercises 3.9 to 3.11 keep their numbers and become closure-capture and pointer-identity drawings.
- Nondeterminism (4.3, D25): use a named search experiment with explicit admission rules. A solution iterator resumes alternatives. Preserve search order, failure continuations, backtrackable state, and explicit permanent effects. Keep exercises 4.35 to 4.54.
- Picture language (2.2.4): painters are functions from a frame to segments. Generated SVG figures, where used by the book, are checked in so the book builds without running examples. Kotlin uses the reference figure assets; its picture-language tests check geometry in memory and write no files.
- Symbols and quotation (2.3.1): represent symbolic data through each host's declared domain types and constructors. Keep the distinction between a name, a value, and a syntax tree. Do not encode executable old source in strings or lists.
- Interaction display (D29): show the definition, call, and observed result in one `@example <language>` block. Results are comments with a `=>` marker: `// => 441` in Rust, TypeScript, and Kotlin; `(* => 441 *)` in OCaml. Keep printed effects in order and separate them from the returned value. Use the edition's declared value and diagnostic display, not a shared old-language printer. Section 0.8 explains the convention. Runnable examples must assert the observed values or effects. Do not imply that a comment is execution evidence.
- Highlighting theme: `text/assets/css/highlight.css` is the one Pygments theme for all four languages, assembled into both stylesheets. Result lines are comments, so Pygments renders them in the comment style; that is intended. Class map: keyword `.k .kd .kr` #5a3696; type and class `.kt .nc` #7a3e00; string `.s .s1 .s2 .sc` #2f6b2f; number `.m .mi .mf .mh` #8a4b08; builtin `.nb` #0b5fa5; function name `.nf .fm` #383838 bold; comment and results `.c .c1 .cm .cp` #6a737d italic; operator and punctuation `.o .p` #383838; lexer error `.err` #b00020. Every color meets WCAG AA contrast against the page background. Code keeps the book faces (Inconsolata LGC at the existing size).
- Chapter appendix per edition: sections 2.4, 2.5, 3.3, and 4.1 close with an "In this language" note set as a quotation block whose first line is `@strong{In this language: <topic>.}`, where `<topic>` names the host mechanism. The note runs at most fifteen lines: one paragraph naming the host mechanism, one short typed sketch, one sentence saying when the book's table mechanism still wins. It sits after the section's last prose paragraph and before its first exercise; `tools/closing_notes_check.py` fails `just book` when a required note is missing. The OCaml edition closes every chapter with the Base and Core appendix in this fixed six-subsection format: Base translation; Core translation; Errors; Collections; Data interchange; Tests (D5).
- Term index: the adapted prose keeps every `@newterm` and `@cindex` of the source paragraph and adds `@newterm` for each host-language concept the section introduces, so the Term Index stays complete per edition.

### Chapter 0 primer (per edition, about thirty pages)

Common skeleton; each companion refines the content and lists its exercises 0.1 onward:

| Section | Teaches |
|---|---|
| 0.1 Running this book | The pinned toolchain, `just` recipes, the layout of `examples/`, `exercises/`, `solutions/`, how a test proves an exercise; running the code is optional and the book reads standalone in every format |
| 0.2 Values, names, and functions | Literals, bindings, functions, closures, higher-order functions; booleans, comparison operators, and conditionals; operator precedence in infix languages; integers, floats, and the division rule used in this book |
| 0.3 Recursion and the call stack | Recursive and iterative processes in this language (tail calls or loops) |
| 0.4 Data | Tuples, lists, the language's sum type, pattern matching, exhaustiveness |
| 0.5 Mutation and ownership | The language's mutable cell, aliasing, and identity; Rust ownership and borrowing; OCaml refs and mutable fields; TypeScript assignment and `Ref` (Effect enters at 3.1, not here); Kotlin captured `var` |
| 0.6 Errors as values | The edition's error type and how the book uses it |
| 0.7 Testing | The test framework, property tests, how solutions are checked, the pending marker and `just scaffold` |
| 0.8 Reading this book | The interaction convention (D29) with one worked example per format, how exercises map to SICP's numbers, the signpost sentences, the exercise map |

The interaction convention taught in 0.8 applies from the first listing in 0.2. Each companion lists its 0.x exercises; the exercises use only constructs the primer has taught.

### Per-edition digests

The companions hold the concept map, the per-section notes for all 22 sections, the conventions, the Chapter 0 outline, the re-cut list, and the architecture sketches for the chapter 4 evaluator, the register-machine simulator, and the compiler. Decisions fixed from them:

- Rust: use `List<T>` with `Rc` sharing where the lesson requires it and `Vec` where it does not. Chapter 3 introduces explicit shared mutable cells; `Weak` handles break circuit and connector cycles. Operation tables use operation names and ordered tag lists. Keep stream and iterator distinctions. The guest must enforce its declared moves, borrows, captures, and numeric rules before execution; runtime `Rc` values do not authorize invalid source aliases. Preserve resumable search frames and undo trails, the assembly boundary, simulated semispace storage, and the stop-and-copy collector. Keep one crate per chapter plus `sicp-runtime`.
- OCaml: keep Stdlib in the main text, mutable records where aliasing is the lesson, closure records for message passing, and `Hashtbl` operation tables. Base and Core appendixes compare their module abstractions. Keep `Lazy.t` streams with `Seq` as a comparison, and `Domain.spawn`/`Mutex.protect` concurrency without Eio. The checked guest obeys OCaml recursive-binding, type, reference, and evaluation-order rules. Named search experiments retain resumable alternatives; do not reuse one-shot continuations illegally. Chapters 4 and 5 share the accepted syntax contract. Chapter 5 retains explicit simulated memory and collection.
- TypeScript: earlier host lessons use `number` and explicit `bigint` arithmetic where exactness requires it, including rational and polynomial examples. Keep the reader-built `List<A>`, pure chapters 1 and 2, and the later Effect/Ref boundary. Operation tables use `Map`; closed symbolic variants use exhaustive matching. Preserve memoized host streams and library comparisons. The guest core instead follows the binary64-only grammar and returns typed outcome/transcript data. Effect services remain at driver boundaries, not guest primitives. Lazy evaluation and search are named experiments; continuation-passing remains the objective of 4.35a. Vitest, `@effect/vitest`, fast-check, and chapter `test.projects` remain the test conventions.
- Kotlin: keep persistent collections, captured `var`, explicit mutable pairs, and operation tables keyed by operation and tag sequence. Earlier chapters retain checked arithmetic abstractions, exact `BigInteger` lessons, `tailrec`/loops, and host recursion facilities. Guest arithmetic follows the separate native `Int`/`Long`/`Double` contract, including wrap, division, conversions, and nullability. Keep memoized host streams and the cold-sequence comparison. Named search and query models remain distinct. Typed controller data serves the simulator, explicit-control evaluator, and compiler; operations return typed errors rather than defaults.

Companion constraints:

- The accepted grammar controls guest syntax and semantics. Earlier host-language lessons may use documented facilities outside that guest grammar. Do not narrow those lessons to fit the guest.
- Apply the repository's existing pins, including Gradle 9.7.0. Do not adopt a different version from an old sketch.
- Operation tables use an operation name plus an ordered tag list. Rust agenda events order by time and sequence, not by their callback. Invoke `FnOnce` thunks once. Normalize rational signs with checked arithmetic.
- Concurrency examples preserve cancellation handles (D32). Randomized teaching probes use the seeded generator (D31). Pending TypeScript exercise tests use `test.todo` (D28).
- Machine jumps support both label and register targets. Keep typed assembly and runtime errors. An unchecked cast from every target to a label is not a valid implementation.
- A conceptual sketch is not runnable proof. Replace obsolete sketches with the accepted contract or checked source; do not keep known-broken code as an implementation template.

## Exercise policy

- Numbering is 1:1 with SICP in every edition (D4, D24). A tailored addition is numbered as the exercise it extends plus a lowercase letter and sits right after it; at most one addition per section per edition, chosen at unit start from the map's idea column and recorded in the map (D30); a second addition per section never happens.
- Every exercise has a per-language class in `docs/exercise-map.md`: `T` needs only syntax changes; `A` keeps the idea with a changed statement and recorded reason; `R` supplies a host-language replacement under the original number and section objective.
- An `R` statement opens with `@emph{This edition replaces SICP exercise N.M with a host-language exercise that keeps its number and teaching objective.}` An addition opens with `@emph{Exercise N.Ma is added by this edition and extends exercise N.M; SICP numbers stop at N.M.}` Keep the matching numbered anchor. The exercise map owns this wording.
- Map overrides are recorded in the policy header: 2.5 uses checked `u128` in Rust and bounded OCaml `int`, with bounds in the statements; TypeScript uses `bigint` and Kotlin uses `BigInteger` for those lessons. Exercise 2.86 uses the section's trait, not `num-traits`. TypeScript 2.74 and 2.75 use plain unions, not `Schema`. Exercises 5.50 and 5.52 use the edition's checked guest evaluator source. Review each changed chapter-4 classification against its actual objective.
- Every exercise has three artifacts in every edition: the statement in `book/`, a pending scaffold in `exercises/`, and a reference solution with passing tests and a rationale in `solutions/`. Tests are property, boundary, or transition tests that a plausible bug would fail; a solution with no observable contract (a diagram, a proof, a discussion) has a Markdown answer and no test, and the map marks it `prose`.
- Scaffold contract (D28). Rust: `crates/chNN/exercises/sec_X_Y.rs` compiled by a dedicated `[[test]] name = "scaffold_sec_X_Y"` target, each test `#[ignore = "pending solution"]`. OCaml: the signature in the `sicp_chN_exercises` library plus a pending executable under `(alias (name scaffold))` that `dune runtest` never runs. TypeScript: `packages/chN/exercises/ex_N_MM.test.ts` with `test.todo`. Kotlin: `src/exercises/kotlin/.../E*.kt` with an exercises test source set, `@Disabled("pending solution")`; the `exercises` and `solutions` source sets are attached to `check` by explicit compile tasks (compile-only for exercises), because a source set with no attached task compiles nothing. The language gates treat pending scaffolds as absent. `just scaffold` from a language root surfaces them: `cargo nextest run --workspace --ignored`, `dune build @scaffold`, and the vitest and Gradle todo listings; the Rust and OCaml runs are expected to fail while unsolved; `just scaffold` never runs in CI.
- `tools/exercise_map_check.py` fails when any edition lacks any of the three artifacts for any of the 356 numbers plus its chosen additions, when an exercise exists on disk that the map does not list, or when a section's counts differ from the map.
- Chapter 0 exercises (0.1 onward) and tailored additions are original work and are marked as such in the map.

## Work breakdown

The approved migration plan defines the active tasks and their acceptance criteria. Keep one concern per commit. A concern may cross sections when a shared source or runtime contract requires all callers to move together. Do not commit a broken intermediate cutover.

Within each edition, implement in this order:

1. Checked source, typed syntax, diagnostics, and runtime values.
2. Direct and analyzed evaluation.
3. Named lazy/search experiments and typed query data.
4. Register machines, assembly, instrumentation, and simulated storage.
5. Explicit-control evaluation, compilation, compiled/interpreted calls, guest self-interpretation, and C-backend lessons.
6. All dependent examples, scaffolds, solutions, rationales, and book passages.

Independent edition roots may run in parallel. A consumer may start after its owner publishes a stable interface and releases the consumer's paths. Shared files have one integration owner.

For each unit, read the current edition section, exercise-map rows, idiom companion, and accepted grammar. Preserve every instructional objective, numbered identity, term-index entry, and required closing note. Keep a runnable example for each executable listing. Migrate pending scaffolds and reference solutions together. Record classification changes in the exercise map.

The parent runs language gates, meaningful behavior checks, exercise coverage, and book checks. It records the exact evidence and resolves review findings before committing. Workers do not run gates, formatters, or commits.

## Quality gates and CI

| Gate | Command (from the language root) |
|---|---|
| Rust | `cargo fmt --check`, `cargo clippy --all-targets --all-features --locked -- -D warnings`, `cargo nextest run --workspace --locked`, `cargo test --doc --locked` |
| OCaml | `dune build @fmt`, `dune build` with warnings as errors in the dev profile, `dune runtest` (the `scaffold` alias is excluded) |
| TypeScript | `pnpm install --frozen-lockfile`, `pnpm biome check .`, `pnpm -r exec tsc --noEmit` (every chapter package), `pnpm vitest run` (the root `vitest.config.ts` lists `packages/*` under `test.projects`) |
| Kotlin | `./gradlew ktlintCheck`, `./gradlew build --write-locks` only when pins change, `./gradlew build` with `allWarningsAsErrors = true` and `explicitApi()` on the runtime module, `./gradlew test` |
| Tools | From the repository root: `just check-tools` runs Ruff format/check and strict Pyright; `just test-tools` runs pytest under `uv run --project tools`. |
| Books | `just books` builds HTML, EPUB 3, and PDF for each edition, then runs `just check-book-structure`. Edition recipes run their asset, math, EPUB, and closing-note checks. Run the exercise-map check as a separate required gate. |
| Conformance | `just test-conformance` runs every case in `spec/host-subsets/cases.json` through each edition's toolchain and compares the result with the native toolchain or the independent reference model. Preserve all case identities. Core outputs need checked-native provenance; experiments need independent models. Compare direct, analyzed, explicit-control, and compiled execution where required. |

The root `setup` recipe invokes each edition's setup plus book and shared-tool setup. `check` invokes edition formatting and linting, `check-tools`, and `check-exercise-map`. `test` invokes edition tests, shared-tool tests, and `test-conformance`. `books` builds all formats and checks structural preservation. `scaffold` reports pending student work and does not run in CI. Each edition exposes its own setup, format, lint, test, scaffold, and book recipes; the checked-in justfiles are the command authority.

The checked-in GitHub Actions workflow defines the language, tool, HTML/EPUB, PDF, and site jobs. HTML/EPUB and PDF failures are blocking. The HTML/EPUB job also runs the preserved-structure check. Keep immutable action SHA pins and frozen dependency installs (D34). Keep existing toolchain and cache configuration unless the migration requires a change. The site job remains gated by its book dependency and the existing push-to-master condition. Editing this plan does not authorize a remote dispatch or deployment.

## Tool interfaces

Run Python tools from the repository root with `uv run --project tools python tools/<name>.py`. Their command-line interfaces use `--help`, stdout reports, and stderr diagnostics. Check commands return 0 on success, 1 on a defect, and 2 on an argument error. Permanent tests must expose plausible behavior, boundary, or invariant failures; do not pin incidental report wording.

- `texi_indexes.py --tree <lang>/book [--check]`: regenerates `back/exercises.texi` and `back/figures.texi`; `--check` writes nothing and exits 1 when either file is stale.
- `book_structure_check.py [--root .] [--edition rust|ocaml|typescript|kotlin] [--inventory <path>] [--texi2any <path>]`: checks exact numbered identities and the actual rendered HTML against the preserved inventory. Texinfo 7.3 is required. Conversion errors, unresolved references, and targets missing from rendered output fail the check. Without `--texi2any`, use `TEXI2ANY` or the command on PATH.
- `figures_pdf.py --src text/original/figures --out text/original/figures/pdf [--check] [--jobs N]`: `rsvg-convert -f pdf` per SVG, skipping outputs newer than the source; `--check` exits 1 on missing or stale PDFs; exits 1 when `rsvg-convert` is absent.
- `exercise_map_check.py [--lang rust|ocaml|typescript|kotlin] [--section N.M]`: parses `docs/exercise-map.md` and checks the three artifacts per number for all editions or the named one, including Chapter 0 exercises and chosen additions; the statement check is the `Exercise N.M` anchor in `<lang>/book/ch*/`; one line per defect (`rust 1.7: missing solutions test`); exits 1 for an exercise on disk that the map does not list.
- `build_css.py --assets text/assets/css --out <lang>/book [--font-url-prefix <prefix>]`: writes `book.css` and `book-epub.css` as step 5 describes; exits 1 when an `@import` survives.
- `check_html_assets.py --html <lang>/book/_build/html`: every `img` or `object` `src` and every CSS `url()` resolves to an existing file; prints `pages=<n> images=<n> fonts=<n>`; exits 1 on the first unresolved reference. The prototype ticket records the one chosen URL convention (root-relative for the site) and this script enforces it from then on.
- `epub_manifest_fix.py --epub <file>`: rewrites the OPF inside the zip in place: `image/svg+xml` for `.svg` items, `properties="mathml"` on XHTML items containing `<math`, `mimetype` kept first and stored; prints `svg_items=<n> mathml_docs=<n>`; idempotent.
- `edition_switcher.py --html <lang>/book/_build/html --edition <lang>`: injects `<nav class="editions">` into every page linking to the same section file in the other three editions; the mapping is mechanical because section numbers and node names are 1:1; exits 1 when a page has no counterpart.
- `closing_notes_check.py --tree <lang>/book`: sections 2.4, 2.5, 3.3, and 4.1 contain a quotation block starting `@strong{In this language:`; exits 1 otherwise.

## Wayfinder map and tickets

The original campaign used GitHub issues on `metaphorics/modern-sicp`. Preserve that history and `wayfinder-map.md`. The current migration's execution briefs, ownership boundaries, review records, and evidence belong under `.outline/sdd/`. Follow `host-subsets-migration.md`; do not recreate bootstrap tickets or infer new remote authority from the old campaign.

## Approach

Execute [host-subsets-migration.md](host-subsets-migration.md). It starts with the reported QC repairs and accepted grammar contracts, then moves each edition through its dependent implementation units. Shared conformance, structural checks, CI, and assets follow their required handoffs.

Remove archived prose and obsolete tools only after every affected consumer has a working replacement. Reconcile the exercise map, active guidance, command documentation, and attribution. Audit the migration surface first, then audit the repository. Resolve findings before the final language, native-conformance, book, and rendered checks.

Keep the existing toolchain pins, strict compiler options, source-set wiring, and frozen lockfiles. Use `CONTRIBUTING.md` and the checked-in setup recipes rather than repeating the completed bootstrap. Preserve pending student scaffolds without hiding incomplete reference implementations. No remote publication follows from this local execution plan.

## Verification

Record actual commands, exit statuses, and observable results. The earlier campaign's green gates are historical evidence, not proof of the migration.

- Language gates: run the pinned Rust, OCaml, TypeScript, and Kotlin gates. Keep all packages and source sets included. Pending student work must remain visible through `just scaffold`, not make a completed solution appear tested.
- Core conformance: type-check each supported source with the native host toolchain before execution. Compare values, ordered effects, and error categories across native, direct, analyzed, explicit-control, and compiled paths. Reject host-invalid and unsupported source before effects. Do not compare unstable diagnostic wording.
- Experimental semantics: use named modes, explicit admission rules, independent finite models, and invariants for lazy evaluation and search. Native eager execution is not their oracle.
- Self-interpretation and C: run the translated evaluator as checked guest source through the teaching evaluator and compiled machine. Preserve C evaluator and compiler-backend execution. Native helper calls, generated fake output, or hidden old source do not satisfy these lessons.
- Coverage: preserve every identity in `spec/host-subsets/cases.json` and `spec/book-inventory.json`. Run the exercise-map check for all editions. Review changed exercise classifications and their statement, scaffold, solution, and rationale as one contract.
- Books: produce each edition's HTML, EPUB, and PDF. Run structure/reference, asset, math, and EPUB checks. Inspect representative rendered pages, including chapters 4 and 5. Check figures, captions, navigation, fonts, footnotes, indexes, inline/display math, and executable listings. Search generated output as well as source for removed teaching material.
- Integration: audit the migration diff before the repository-wide audit. Resolve all blocking findings and run an independent final review. Preserve license text and attribution. Do not deploy or change old releases without separate authority.

## Original planning sources (2026-09-22)

- Texinfo 7.3 release announcement, 2026-03-02: https://lists.gnu.org/archive/html/info-gnu/2026-03/msg00000.html
- Texinfo manual, syntax highlighting and EPUB generation nodes: https://www.gnu.org/software/texinfo/manual/texinfo/html_node/Syntax-Highlighting.html and the neighbouring `Generating-EPUB.html`
- Rust release notes and Cargo reference: https://doc.rust-lang.org/stable/releases.html, https://doc.rust-lang.org/cargo/reference/workspaces.html, https://endoflife.date/api/v1/products/rust/
- OCaml releases and opam packages: https://ocaml.org/releases, https://opam.ocaml.org/packages/
- TypeScript 7.0 announcement and npm dist-tags: https://devblogs.microsoft.com/typescript/announcing-typescript-7-0/, https://registry.npmjs.org/typescript, https://registry.npmjs.org/effect
- Effect 4 documentation: https://effect.website/docs/v4/getting-started/installation
- Node.js support windows: https://endoflife.date/api/v1/products/nodejs/
- Kotlin, Gradle, and JDK: https://kotlinlang.org/docs/releases.html, https://docs.gradle.org/9.7.1/userguide/compatibility.html, https://endoflife.date/api/v1/products/eclipse-temurin/
- CC BY-SA 4.0 legal code and compatible licenses: https://creativecommons.org/licenses/by-sa/4.0/legalcode.en, https://creativecommons.org/compatible-licenses/
- SICP JS comparison edition and license: https://sicp.sourceacademy.org/, https://github.com/source-academy/sicp
- Upstream lineage: https://github.com/sarabander/sicp
- Vitest 4 migration guide, `workspace` replaced by `projects`: https://v4.vitest.dev/guide/migration
- OCaml 5.5 `Pqueue` (in Stdlib since 5.4): https://ocaml.org/manual/5.5/api/Pqueue.html
- Texinfo 7.2 on the machine: `/usr/share/texi2any/ext/epub3.pm` (image media-type map, manifest properties) and `texinfo.info` (`T4H_MATH_CONVERSION`, `T4H_TEX_CONVERSION`)
- Repository counts measured on 2026-09-22 with `grep -c` and `find` over `sicp-pocket.texi` and `html/fig/`

## Constraints and contingencies

- The user approved the host-subset specification and ordered migration plan. New dependencies, unrelated public-contract changes, and further data removal still require approval.
- Preserve the existing language and library pins. A required toolchain change needs a demonstrated migration failure and an explicit decision; a newly released version is not a reason to upgrade this task.
- If math conversion fails, repair the source or converter against the supported Texinfo math forms. EPUB must not depend on JavaScript. Do not hide a lost equation behind a successful converter exit.
- Keep EPUB media-type and MathML-property repair while the pinned converter needs it. Prove a replacement preserves valid packaging before removing that repair.
- If a structural check fails, repair the missing or hidden material. Do not regenerate the inventory, skip an identity, or accept the same count with different identities.
- Keep every original exercise number. Adapt its objective to the host or use a same-number replacement with its reason in the map.
- Keep one owner per file and explicit dependency handoffs. Another edition may proceed while one edition is blocked; no dependent consumer may guess an unpublished interface.
- A custom domain, new cover art, PDF syntax highlighting, and extra tailored additions are outside this migration.
