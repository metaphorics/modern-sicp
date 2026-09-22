# Engineering review (oracle agent) — raw output

## Decision register

| Principle | Verdict | Reason (one sentence) |
|---|---|---|
| clarity | amend | The plan names tools and outputs but leaves format-specific image branches, corpus applicability, index regeneration, and the shared runtime contract underspecified. |
| impact | amend | Split, parity, corpus, and CI gates can pass while figures, math, exercise coverage, or evaluator variants are missing. |
| audience | amend | Several companion choices silently change the lesson (readers, streams, table keys, numeric bounds, search mechanism). |
| risk | amend | Tex4ht MathML, EPUB SVG/MathML manifest rules, SVG-to-PDF, fixed-width arithmetic, memoization, and backtracking each have untested failure paths. |
| sequencing | amend | Format probes, index-regeneration design, and the corpus contract must precede demolition and the language phases. |
| reversibility | accept | Retaining the original Texinfo source is reversible, provided generated assets and parity manifests are retained. |

## Amendments

1. **`Text pipeline and demolition`, steps 2 and 5, and `Verification` V2 — fix HTML/EPUB figure conditionals.** The SVG for Figure 1.1 sits inside `@iftex` (`sicp-pocket.texi:1441-1443`) and the existing Makefile passes `--iftex` for HTML; the planned commands omit it, so HTML/EPUB can ship the ASCII fallback with no figures. Amend: `split_texi.py` replaces the TeX-only branch with `@ifhtml` referencing the SVG (HTML and EPUB) and `@iftex` referencing the generated PDF; V2 compares the HTML/EPUB figure-ID set against the source anchor set.

2. **`Text pipeline and demolition`, step 4, and V2 — reconcile figure counts with named units.** `ex-fig-ref.pl:18-32` hardcodes 5+26+38+6+18 = 93 figures and `figures.texi` enumerates 1.1-5.18, while the 91 SVGs include cover and icons. Amend: parity reports `section_files=22`, `exercise_anchors=356`, `figure_anchors=93`, `figure_list_entries=93`, `numbered_image_refs=84`, `license_image_refs=3`, `svg_assets=91`, `footnotes=<source>`, `undefined_refs=0`; `figures=91` is renamed `svg_assets`; unreferenced assets (`Fig2.23a`, cover, unused icons) are allowlisted or removed; no numbered entry is deleted to force 91.

3. **`Text pipeline and demolition`, steps 3 and 4 — regenerate the two global lists, never hand-edit.** `sicp-pocket.texi:37921-37928` includes `exercises.texi` and `figures.texi` as global nodes under Top, and both are generated from hardcoded counts in `ex-fig-ref.pl`. `texi_indexes.py` must regenerate them from source anchors (or the global lists are kept whole in the split); parity then proves definition, list-entry, and reference sets agree. Also, `@float Figure` is not the scan key: the source uses bare `@float` plus an anchor, and Figures 5.3, 5.6, 5.8-5.10, 5.12-5.13, 5.17-5.18 are `@quotation` blocks with no float at all. Scan every `@anchor{Figure N.M}`.

4. **`Text pipeline and demolition`, step 2 — make the split lossless with a byte-offset manifest.** Split only at the 22 top-level section nodes (subsection nodes stay in their parent file); emit byte offsets; compare the assembled source to the original after only declared substitutions, plus node/anchor/footnote/image/include multiset comparison. Count-only parity cannot detect dropped prose. The `@image` path rewrite is relative and must resolve against each `<lang>/book` tree or be replaced by format-neutral references.

5. **`Toolchain pins` / `Quality gates and CI` / V2 — replace the `HTML_MATH=t4h` assumption with a verified Texinfo 7.3 invocation and fixture matrix.** The captured manual supports `l2h`, `t4h`, and `mathjax` for `HTML_MATH`; the gap is that this source carries LaTeX-specific `@math` and raw `@tex`, and tex4ht needs `T4H_MATH_CONVERSION=latex` (and possibly `T4H_TEX_CONVERSION`), and the local `texinfo.4ht` predates language tags. Amend: set both variables, run a fixture matrix over inline `@math`, `@displaymath`, and raw `@tex`, fail on conversion errors or raw-TeX leakage, and do not infer correctness from legacy HTML output.

6. **V2 — add EPUB SVG and MathML manifest assertions.** The legacy EPUB converter maps only PNG/JPEG/GIF; unknown `.svg` falls back to a type EPUBCheck rejects, and the native EPUB extension emits manifest items without scanning for MathML. Amend: require SVG-to-raster conversion for EPUB (or a verified 7.3 manifest fix), assert every image item has a valid media type, assert MathML documents carry `properties="mathml"` in the OPF, and make epubcheck mandatory (absence is failure, not a skip).

7. **`Edition design` D23 and `Approach` A4 — capability-partitioned corpus.** One undifferentiated `programs/*.scm` plus `expected/*.txt` cannot cover strict, lazy forcing, amb choice paths, query frames, machine traces, and compiled code; `amb` forms (`require`, `ramb`, `if-fail`, `permanent-set!`) are absent from the listed grammar and the query engine is not the 4.1 evaluator. Amend: core grammar plus lazy/amb/query/machine subgrammars; a manifest assigns each program a capability (`core`, `lazy`, `amb`, `query`, `machine`, `compiler`), output mode, and required evaluators; capability-specific expected outputs; a printer contract fixing booleans, symbols, strings, pairs, empties, procedures, errors, frames, solution order, line endings; deterministic amb order and declared query fairness; unexplained skips fail.

8. **`Edition design` D18-D20 and the error bullet — define the cross-language runtime contract.** D20 demands one empty value, but Rust has `Nil` plus a non-empty `Stream<T>` and `Option<Env>`, OCaml has `Nil`, `Cons`-only streams and frame lists, Kotlin defines both `VNil` and `SNil`; D18 permits cold host sequences and an unmemoized thunk without fixing tested semantics. Amend: one object-language `nil` for the empty list; each host defines its own empty-stream and environment-root representations with documented boundaries; one canonical memoized `delay`; cold `Sequence`/`Seq`/iterator/Effect adapters are comparison-only; unmemoized thunks only in named probe cases.

9. **`Decision register` / `Per-edition digests` / `Exercise policy` — reconcile every companion contradiction in a decision ledger:**

| Plan location | Companion location | Failure scenario and exact resolution |
|---|---|---|
| D14/D17 `Numbers`; map overrides | `exercise-map-ch1.md` out-of-stdlib rows 1.19/1.24; `exercise-map-ch2.md` row 2.5 and out-of-stdlib table; `idiom-ocaml.md` `Numbers` | Builds introduce prohibited `num-bigint`/Zarith or change the overflow lesson. Replace map rows with bounded `i128`/`int` behavior; drop numeric-library requirements and the Zarith appendix dependency unless D17 changes. (Note: `exercise-map-ch1.md` line 90 names the `rand` crate for Rust 1.24 and line 91 `i128` for 1.19; `rand` is a real unpinned dependency to resolve under D14 or pin.) |
| D16 Kotlin pins | `idiom-kotlin.md` section 0 pins Gradle 9.7.1 | 9.7.1 is outside the plugin window ending at 9.7.0. Pin 9.7.0 in companion, wrapper, V1, and CI. |
| D20 `Value` bullet | `idiom-rust.md` Part 4; `idiom-ocaml.md` value/stream/env sketches; `idiom-kotlin.md` sketches 1 and 4 | Empty/root behavior untypeable; OCaml's immutable `Pair` and Kotlin's immutable `VPair` violate the mutable-pair rule. Adopt amendment 8 wording; mark each case mutable, empty, unassigned, or separate. |
| D23 reader (plan line 216) | `idiom-kotlin.md` lines 682 and 688 (no reader, prebuilt `Value` lists) | Kotlin cannot run the unchanged corpus. D23 overrides the companion: add the reader and printer, delete the no-reader exception, or amend D23 to split a corpus-fixture reader from evaluator drivers. |
| D19 tag-list keys (plan line 220) | `idiom-ocaml.md` lines 93-96 (correct shape); `idiom-rust.md` lines 126-129 and `idiom-kotlin.md` lines 132-134 (`(op, tag)` pairs) | Binary keys cannot represent arbitrary tag lists. Adopt operation plus ordered tag-list everywhere; define multi-argument lookup, overwrite, and missing-entry behavior; handlers return the typed error channel (Kotlin's raw `Value` violates the error bullet). |
| Errors bullet | `idiom-kotlin.md` sketch 7 and chapter 5 `Machine.run` | Machine failures escape as raw values or `Unit`. Typed errors required; add absent-operation and invalid-instruction corpus cases. |
| D18 streams | `idiom-kotlin.md` 3.5/4.2 (`VThunkNoMemo`, cold `Sequence`) | Cold or unmemoized representations change 3.51, 3.57, 3.63 and query termination. Canonical memoized stream required; host pipelines comparison-only. |
| D25 Kotlin `sequence {}` | `idiom-kotlin.md` sketch 8 and section 4.3 reject the builder, choose `Sequence` composition | Wrong mechanism and teaching text. Replace D25's Kotlin phrase with `Sequence` composition over persistent environments. |
| Concurrency halt handle | `idiom-ocaml.md` 3.4; `idiom-kotlin.md` 3.4 | OCaml join/scope and Kotlin launch/scope lack the promised halt handle. Add cancellation handles or narrow the shared decision; test in all four editions. |
| D25 re-cut list | `wayfinder-map.md` `Destination` (3.2, 3.4, 5.3) | Plan says 5.3 unchanged, 4.3 re-cut. Replace map text with 3.2, 3.4, 4.3. |
| register D1-D25 | `wayfinder-map.md` `Notes` (D1 to D20) | Tickets can omit D21-D25. Replace with D1 to D25. |
| prose-only solutions | `wayfinder-map.md` `Destination` (every solution runs under the gate) | Diagram/proof exercises are allowed no test. Replace with the exercise-map prose rule. |
| OCaml 5.5 stdlib claim | `idiom-ocaml.md` cites `Pqueue` as 5.5 stdlib | Pqueue moved out of Stdlib into a separate package in OCaml 5; the agenda note would not build on the pins. Verify 5.5 availability or drop the margin note; keep the hand-built agenda. |
| Rust 1.98.1 pin | `Toolchain pins`, Rust table (stable, released 2026-09-03) | Release date is in the future relative to today; unverified against release notes and releases.rs. Verify before committing the pin, or mark the pin provisional in `docs/toolchain-pins.md`. |

10. **`Toolchain pins` / CI / V1 — reproducible pins.** The Effect/vitest/vitest peer set is consistent (`@effect/vitest@4.0.0-rc.117` peers `effect ^4.0.0-rc.117`, `vitest >=5 <6`); the real gaps are unpinned `@effect/platform-node`, `@types/node`, fast-check, Pygments, TeX Live/tex4ht image, librsvg, epubcheck, GitHub Actions, and cargo-nextest (not a rustup component; needs a CI install step). Record exact versions and lockfile hashes; use frozen installs; TypeScript records actual tsconfig flags; V1 separates bootstrap checks from implementation gates (an empty workspace proves nothing about evaluators, books, or corpus).

11. **V2/V6 — close false-green paths.** V2 can pass on one `<math` token and one highlighted `<pre>` while EPUB validation is skipped, PDF figures are absent, other languages are unhighlighted, or cross-references fail only in EPUB/PDF. Books CI builds all four editions in all three formats, validates HTML links and figure IDs, EPUB with epubcheck plus SVG/Media-type and MathML-properties checks, PDF figure inclusion, and compares source/output counts for sections, exercises, figures, footnotes, index entries, and math blocks. V6 adds a representative exercise, figure, math block, footnote, cross-reference, highlighted listing, and corpus capability case per edition.

12. **V3/V4 and `Approach` step 1 — missing inputs and the Scheme oracle.** Only `exercise-map-ch1.md`-`ch3.md` and three idiom companions exist; V3 claims five maps and the approach copies four companions. Guile/Racket cannot validate Effect, OCaml effects, Kotlin sequences, query, machine, or compiler variants. Require the TypeScript companion and chapter 4/5 maps before approval (or A4 generates them first); V3 checks unique set equality against all 356 source anchors; V4's oracle is the repository harness or a checked-in reference interpreter, with Guile/Racket advisory only for exactly matching core cases.

13. **`Per-edition digests` and sketches — compile probes.** The Rust agenda sketch puts `Rc<dyn Fn()>` inside `BinaryHeap<Reverse<(u64, u64, Rc<dyn Fn()>)>>` (callbacks lack `Ord`); the lazy sketch calls `call_mut` on a `FnOnce`; the rational constructor rejects negative denominators instead of normalizing; the Kotlin machine casts every `GotoTarget` to `Lbl`, ignoring register gotos. Load-bearing sketches get compile-only fixtures: an agenda `Event` ordering on `(time, sequence)` with the callback as a non-compared field; `FnOnce` invoked as `f()`; rational sign normalization with checked arithmetic; both goto cases implemented. Non-compiling sketches are pseudocode, not contracts.

## Tasks

1. Phase A: replace implicit `@iftex` HTML figures with explicit SVG-for-HTML/EPUB and PDF-for-TeX branches; acceptance compares all 93 figure IDs across source, HTML, EPUB, and PDF.
2. Phase A: implement the node-bounded byte-offset split and parity manifest; regenerate the two global lists from source anchors; acceptance reports 22 sections, 356 exercise anchors, 93 figure anchors and list entries, 84 numbered image references, 3 license images, 91 SVG assets, footnote parity, zero undefined refs.
3. Phase A: run the Texinfo 7.3 fixture matrix (inline `@math`, `@displaymath`, raw `@tex`, language-tagged examples) with `HTML_MATH=t4h` and `T4H_MATH_CONVERSION=latex`; acceptance fails on conversion errors or raw-TeX leakage.
4. Phase A: add EPUB checks — SVG rasterization or verified manifest fix, valid image media types, `properties="mathml"` on MathML documents, mandatory epubcheck.
5. Phase A: publish the canonical Value, error, empty-value, memoized-delay, table-key, printer, and concurrency contracts; acceptance is a decision ledger resolving every listed contradiction, including the Rust date and OCaml Pqueue verifications.
6. Phase A: partition the corpus by core, lazy, amb, query, machine, and compiler capability; acceptance requires deterministic normalized output and no unexplained skips.
7. Phase A: supply or generate the missing TypeScript and chapter 4/5 companion maps; acceptance is unique exercise-set equality against all 356 source anchors.
8. Phase F/G: run evaluator, explicit-control, and compiler equivalence corpus cases covering typed errors, memoization traces, amb solution order, query fairness, and machine/compiler agreement.

## Open questions

- Which exact `@math` and `@tex` constructs convert to MathML without rewriting under pinned Texinfo 7.3 plus tex4ht?
- Does the selected TeX path accept every generated PDF figure with correct dimensions and no clipping?
- Is the canonical printer byte-identical across hosts, or semantic with host-specific forms for procedures, errors, frames, and traces?
- Are the missing TypeScript companion and chapter 4/5 exercise maps intentionally absent, or must they be supplied before approval?
- Is Rust 1.98.1 (2026-09-03) verified against the official release notes, or provisional until checked?

### Critical Files for Implementation
- `local://modern-sicp-editions-plan.md` — decision register, pipeline, phases, V1-V6.
- `modern-sicp/sicp-pocket.texi` — source nodes, conditionals, math, image references, anchors, includes.
- `modern-sicp/ex-fig-ref.pl` — hardcoded 5/26/38/6/18 counts generating both list files.
- `modern-sicp/Makefile` — existing `--iftex`, MathML, highlighting, EPUB build behavior.
- `local://idiom-rust.md`, `local://idiom-ocaml.md`, `local://idiom-kotlin.md` — companion runtime and toolchain choices requiring reconciliation.