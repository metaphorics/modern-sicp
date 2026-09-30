# Wayfinder map and tickets for modern-sicp

> Historical record of the completed first campaign. The Scheme subset in `spec/scheme-subset/` named below was replaced by `spec/host-subsets/`. The current authority is `docs/plan/host-subsets-specification.md` and `docs/plan/host-subsets-migration.md`.

Storage: GitHub issues on `metaphorics/modern-sicp`. Labels: `wayfinder:map`, `wayfinder:research`, `wayfinder:prototype`, `wayfinder:grilling`, `wayfinder:task`. Create the labels with `gh label create <name> --color <hex>`: map `1d76db`, research `0e8a16`, prototype `fbca04`, grilling `d93f0b`, task `5319e7`. Create the map issue first, then the tickets, then edit each ticket's `## Blocked by` with the real issue numbers.

## Map issue

Title: `Wayfinder map: modern SICP in four languages`

Body:

```
## Destination

Four self-contained editions of SICP live in this repository, one each in Rust (edition 2024), OCaml (5.5.1 with a Base and Core appendix per chapter), TypeScript (7.0.2 with Effect 4) and Kotlin (2.4.20 with Arrow 2.2.3). Each edition covers all five chapters and all 356 exercises, opens with a Chapter 0 primer, keeps SICP's section and exercise numbers, and re-cuts sections 3.2, 3.4 and 4.3 for its own language. Every listing and reference solution runs under the language's test gate; a scaffold is pending until solved; a prose-only exercise (diagram, proof, discussion) has a Markdown answer and no test. Chapters 4 and 5 implement the shared Scheme subset in `spec/scheme-subset/`. Each edition builds to HTML, EPUB3 and PDF with `just books`. The map is done when every phase A to H unit in `docs/plan/technical-modern-sicp-editions.md` is closed and the four editions are published.

## Notes

- The plan is `docs/plan/technical-modern-sicp-editions.md`; its decision register (D1 to D36) binds every ticket. Companions: `docs/plan/idiom-<language>.md`, `docs/exercise-map.md`, `docs/toolchain-pins.md`.
- Unit of work: one section of one edition, executed with the unit contract in the plan; one section unit per session. Unit of tracking: one chapter of one edition, a ticket with a section checklist. Rust writes each section first; the other editions follow its section note (D12).
- Tailored additions are numbered as the exercise they extend plus a lowercase letter. Replacements keep the original number.
- Prose is plain English without AI tells; code follows the edition's style contract in the plan.
- Licenses: text and figures CC BY-SA 4.0; `examples/`, `exercises/`, `solutions/` GPL-3.0-only; `tools/` and `site/` MIT; see `NOTICE.md`.

## Decisions so far

(empty)

## Not yet specified

- Per-edition preface text (Phase H).
- Constant folding as a compiler extension in the typed editions (a 5.5 tailored addition candidate).

## Out of scope

- Human-language translations.
- A Scheme edition.
- An interactive online REPL.
- Video.
- Upstream contributions to `sarabander/sicp`.
- Bun support.
- Parity with SICP Python.
```

## Tickets

### Prototype: section 1.1 in all four editions

Label `wayfinder:prototype`. Title: `Prototype: section 1.1 through the full pipeline in four editions`.

```
## Question

Does the unit contract hold end to end? Write section 1.1 (The Elements of Programming, exercises 1.1 to 1.8) in the Rust, OCaml, TypeScript and Kotlin editions: examples with asserted outputs, adapted prose, exercise scaffolds, reference solutions with tests, the exercise-map rows, and a book build per edition. Record what the contract must change.

## Blocked by

(none)

## Done when

Each language gate passes; `tools/exercise_map_check.py --section 1.1` passes; `just books` renders section 1.1 in each edition with eight exercise anchors; any change to the unit contract is a comment on this ticket and an edit to the plan.
```

### Chapter 0 primer tickets (one per edition)

Label `wayfinder:task`. Titles: `Rust Chapter 0: The language of this book`, `OCaml Chapter 0: The language of this book`, `TypeScript Chapter 0: The language of this book`, `Kotlin Chapter 0: The language of this book`.

```
## Question

Write the Chapter 0 primer for the <edition> edition from the outline in `docs/plan/idiom-<language>.md` and the common skeleton in the plan (0.1 Running this book to 0.8 Reading this book), about thirty pages, with exercises 0.1 onward as original work.

## Blocked by

- Prototype: section 1.1 through the full pipeline in four editions

## Done when

`book/ch0/0.1.texi` to `book/ch0/0.8.texi` exist with the eight sections; every listing is under `examples/ch0/` with asserted outputs; exercises 0.1 onward have scaffolds and solutions; the language gate passes; the edition builds.
```

### Chapter tickets (one per chapter and edition, created when the previous phase closes)

Label `wayfinder:task`. Titles: `<Edition> chapter N: <chapter title>` (for example `Rust chapter 1: Building Abstractions with Procedures`). Twenty tickets over the programme, four per phase C to G.

```
## Question

Write chapter N of the <edition> edition, one section unit at a time under the unit contract in `docs/plan/technical-modern-sicp-editions.md`. Rust leads each section and posts its section note here; the other editions start a section from that note.

## Sections

- [ ] N.1 <title> (<size>)
- [ ] N.2 <title> (<size>); L units list their session slices: N.2 examples, N.2 prose, N.2 exercises and solutions
- [ ] ...

## Map rows

<the chapter's rows from docs/exercise-map.md for this edition>

## Blocked by

- <Edition> chapter N-1 (or the Chapter 0 ticket for chapter 1)
- Rust chapter N, per section (follower editions only)

## Done when

Every section is ticked; `tools/exercise_map_check.py --lang <lang>` passes for the chapter; `just book` builds the edition; the chapter close commit reconciled the four editions' map rows and the sync note is posted here; the `pages` job published the chapter.
```
