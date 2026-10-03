# Host-subset migration execution plan

Status: Implements the user's explicit approval of the full specification and documented repairs. New scope or dependency choices still require approval.

Anchor: Christopher Alexander, Tony Hoare, David Parnas

## Global constraints

Preserve all chapters, section and exercise numbers, and learning objectives. The approved specification is [host-subsets-specification.md](host-subsets-specification.md). Remove the old source language from the current tree, not just labels. Preserve legal material, needed figures, and Git history. Admit only valid host-subset programs. Keep experimental lazy/search semantics explicit. All self-interpreters must be guest programs.

Workers edit only assigned files. They do not run builds, tests, linters, formatters, or commits. The orchestrator executes checks and records evidence. Use agent-tier selection explicitly. The orchestrator alone stages and commits after green gates. Review each completed concern independently. Resolve blocking findings before dependent work. Do not publish partially migrated work.

Offense stays inside the requested surface. Formal rigor covers parsers, types, ownership, state, and compiler equivalence. Minimalism keeps every required lesson. Keep one deep source-language interface per edition, not adapters back to obsolete representations.

## Phase 1: Repair baseline and fix language contracts
Anchor: Tony Hoare, John Carmack

### Task 1: Repair Rust exercise 3.46
Anchor: Tony Hoare, Edsger W. Dijkstra
Scope: rust/solutions/ch03; corresponding exercise statement in rust/book/ch3/3.4.texi if required.
Start each trial with a free cell. Replace scheduler-dependent required evidence with an explicit load/load/store/store handshake. Preserve a meaningful atomic comparison. Update the rationale. Do not widen the number of yields. Parent checks the focused test, repeated full nextest runs, Rust lint/format/doc gates, and a runtime smoke.

### Task 2: Stop Kotlin figure tests from writing outside the edition
Anchor: John Carmack, Tony Hoare
Scope: kotlin/examples/ch2/S2_2_4APictureLanguage.kt; kotlin/examples/ch2/Painters.kt only if needed; kotlin/book/ch2/2.2.texi for read-only consumer checks.
Identify the real canonical figure source before selecting an output path. The edition currently has no kotlin/book/figures directory. Do not invent a checked-in reference from the test's own output. Remove the write/read-back pseudo-check. Test meaningful rendering or geometry behavior without dirtying the repository. Parent checks the focused suite and verifies no generated root book/ directory.

### Task 3: Specify the Rust guest subset
Anchor: Tony Hoare, David Parnas
Scope: new spec/host-subsets/rust/grammar.md; read rust/crates/sicp-runtime, rust/crates/ch04, rust/crates/ch05, rust/solutions/ch05.
Enumerate syntax, types, numeric rules, moves/borrows/capture, rejection categories, observable output, guest-level self-interpretation, and native-oracle invocation. Explicitly map the chapter-4/5 lesson families to admitted forms. No code changes.

### Task 4: Specify the OCaml guest subset
Anchor: Tony Hoare, David Parnas
Scope: new spec/host-subsets/ocaml/grammar.md; read ocaml/common, ocaml/examples/ch4, ocaml/examples/ch5, ocaml/solutions/ch5.
Enumerate syntax, type inference boundary, recursive bindings, data/patterns, mutation/capture, output, guest self-interpreter, and native oracle. Map all lesson families. No code changes.

### Task 5: Specify the TypeScript guest subset
Anchor: Tony Hoare, Otl Aicher
Scope: new spec/host-subsets/typescript/grammar.md; read typescript/packages/ch4, typescript/packages/ch5, typescript/solutions/ch5, typescript/tsconfig.base.json.
Enumerate source typing, JS runtime behavior, structured/recursive values, closures/mutation, experimental extensions, self-interpretation, and native oracle. Both chapter-4 and chapter-5 parser paths must converge on this contract. No code changes.

### Task 6: Specify the Kotlin guest subset
Anchor: Tony Hoare, Otl Aicher
Scope: new spec/host-subsets/kotlin/grammar.md; read kotlin/ch4, kotlin/ch5, kotlin/runtime, kotlin/solutions/ch5.
Enumerate numeric inference/division/overflow, Boolean conditions, nullability, binding mutability/capture, structured data, guest self-interpretation, and native oracle. Map all lesson families. No code changes.

Barrier: parent verifies QC repairs, native positive/negative contract witnesses, and reviews all grammar contracts for completeness. A grammar that cannot express its self-interpreter is not accepted.

## Phase 2: Replace edition execution and teaching surfaces
Anchor: Christopher Alexander, David Parnas

### Task 7: Migrate the Rust edition
Anchor: Tony Hoare, Christopher Alexander
Scope: rust/ and spec/host-subsets/rust/. Implement the accepted grammar, source checking, runtime, evaluators, experimental engines, query and machine representations, compiler, guest evaluator, and C-backend exercise consumers. Migrate every example, test, exercise scaffold, solution, rationale, book listing, historical passage, and affected figure label. Delete obsolete parsers/types/aliases and source files. Parent runs Rust gates and native/interpreter/compiler conformance plus all book formats.

### Task 8: Migrate the OCaml edition
Anchor: Tony Hoare, Christopher Alexander
Scope: ocaml/ and spec/host-subsets/ocaml/. Same complete consumer cutover, retaining paired interfaces, OCaml typing and evaluation rules. Parent runs OCaml gates and native/interpreter/compiler conformance plus all book formats.

### Task 9: Migrate the TypeScript edition
Anchor: Tony Hoare, Christopher Alexander
Scope: typescript/ and spec/host-subsets/typescript/. Same complete cutover, including both independent old parser paths, Effect error boundaries, metacircular source, and C-backend exercises. Parent runs TypeScript gates and native/interpreter/compiler conformance plus all book formats.

### Task 10: Migrate the Kotlin edition
Anchor: Tony Hoare, Christopher Alexander
Scope: kotlin/ and spec/host-subsets/kotlin/. Same complete cutover, including runtime module and all source sets. Parent runs Kotlin gates and native/interpreter/compiler conformance plus all book formats.

Each edition task must be decomposed into dependency-ordered runtime, evaluator, compiler, and teaching-material units before write dispatch. The edition roots are disjoint; dependent units inside one edition are not parallel. Shared files remain owned by the orchestrator's integration task.

## Phase 3: Replace shared infrastructure and remove archives
Anchor: Christopher Alexander, Dieter Rams

### Task 11: Replace corpus and structural QC
Anchor: Tony Hoare, Richard Feynman
Scope: tools/, spec/, root justfile. Preserve current semantic case coverage. Record fixture provenance per case. Replace the Guile runner and legacy file inventory with native-host conformance. Experimental expectations require independent reference-model provenance. Replace split-source parity with section/exercise/figure/reference inventories derived before deletion. Remove obsolete split/parity code and associated obsolete tests only after replacements cover the useful checks.

#### Executable conformance interface

`tools/host_conformance_check.py` reads the preserved case inventory and each edition's `manifest.tsv` and `driver.json`. Each manifest row has five tab-separated fields: case ID, source artifact, engines, provenance, and lesson note. Each artifact must be a repository file. Each preserved case must occur once.

Core and compiler cases use the native, direct, analyzed, eceval, and compiled engines. This includes the guest self-interpreter. Their provenance is `native`. Other cases use an independent `reference` engine plus `lazy`, `search`, `query`, or `machine`. Their provenance is `reference`. These labels select a required oracle; they do not claim that a check passed.

`driver.json` contains `build` (a list of argv arrays), `run`, `native_compile`, and `native_run` (one argv array each). Commands run from the edition root without a shell. Run commands substitute `{source}`, `{work}`, `{case}`, and `{engine}`. The source is a temporary copy named `program` with the artifact's suffix. Native fixtures must be standalone. Compiler output must stay under `{work}`. The native compiler must check `{source}`. Build commands have no substitutions.

The teaching driver must receive `{source}`, `{case}`, and `{engine}`. It writes one JSON object with `termination` and `stdout`. Termination is `value`, `error`, or `rejected`. The stdout field contains the exact ordered observation transcript. Driver diagnostics go to stderr. Driver failures exit nonzero; guest runtime errors use an `error` observation with exit zero. Source rejection cannot satisfy the runtime-error case.

Native compilation must succeed before native execution. Native and teaching results compare termination and transcript, not diagnostic text or internal stack counts. Independent reference models must not call the engine under test. The migration review checks that boundary.

The CLI accepts `--root`, `--edition`, `--case`, and `--report`. A scoped run still validates full inventory coverage. The JSON report records source hashes, argv, exit status, and output. The root recipe `test-conformance` runs this checker; it replaced the Guile `check-corpus` recipe, and `check-exercise-map` runs the exercise inventory.

### Task 12: Cut over CI and book assets
Anchor: John Carmack, Christopher Alexander
Scope: .github/workflows/ci.yml, edition justfiles, text/, site/. Remove the old interpreter dependency. Update active figure paths without discarding useful assets. Delete original and split archival text after their consumers move. Verify site assembly, fonts, image links, mathematical rendering, EPUB, and PDF. No deployment during partial migration.

### Task 13: Reconcile plans, exercise map, and attribution
Anchor: Richard Feynman, J.R.R. Tolkien
Scope: docs/, README.md, CONTRIBUTING.md, NOTICE.md, LICENSE. Replace the old plan's active shared-language mandate. Preserve the campaign record in Git and the execution ledger. Keep legal attribution accurate. Reconcile every exercise row and run command with the new editions. Do not preserve obsolete names in active guidance.

## Phase 4: Audit, simplify, and verify
Anchor: Tony Hoare, John Carmack

### Task 14: Audit the migration surface
Anchor: Tony Hoare, Edward Tufte
Apply the invoked audit-project workflow to the actual migration diff. Resolve all blocking findings. Check no old source remains in unlabelled blocks, embedded strings, fixtures, generated output, or figures. Verify native type rejection before effects and guest-level self-interpretation. Run a behavior-preserving simplification pass only after correctness is established.

### Task 15: Audit the entire repository
Anchor: Richard Feynman, John Carmack
After Task 14 passes, run the requested repository-wide audit. Include every edition, shared tool, CI recipe, publication asset, and documentation surface. Resolve findings within approved scope; surface genuine expansions rather than silently omitting defects.

### Task 16: Verify and land the complete migration
Anchor: Tony Hoare, John Carmack
Run all language gates, tool gates, conformance, exercise inventory, HTML/EPUB/PDF builds, rendered smoke checks, and final independent branch review. Keep review packages and evidence per atomic concern. Commit only green concerns. Resolve publishing authority before any remote mutation; never rewrite history or delete old releases.

### Audit record: TypeScript contract kernel vs engines
The direct and analyzed dispatchers and compiled switch lowering originally stopped at empty labels in the kernel's `add`/`subtract`/`multiply` group instead of reaching the arithmetic body. Both evaluator paths now continue through normally completing clauses, and `compileSwitch` links each body to the next label. `execSwitch` keeps one frame per switch, matching the static checker, native ECMAScript, and the compiled engine; the kernel's repeated `cell` declarations sit inside separate `block` nodes, so the shared-frame theory did not explain the initial failure, and the checker rejects same-name declarations across clauses. `memberGet` reports `unknown-field` for absent record keys while preserving a present `undefined` value, as the explicit-control and compiled engines already do; string-keyed index reads keep the contract's possibly-absent `Record<string, T>` semantics. The manifest points to `witnesses/section-9-kernel.ts`, whose body is the §9 listing verbatim behind the file header; the separate adapted `metacircular-evaluator.ts` remains for the Chapter 5 exercises. Native, direct, analyzed, explicit-control, and compiled runs all print `120`.

### Decision record: Exercise 5.50 step ratio
The Rust solution for section 5.50 asserts that the interpreted run takes more steps than the direct run and prints the ratio. It does not assert a 100x bound. The measured ratio is 7405 steps against 146, about 50.7x. The book text states no 100x figure, and the OCaml, TypeScript, and Kotlin editions assert no bound. A 100x assertion fails against the measurement, so the orchestrator kept the measured behavior on the evidence.

### Audit record: Footnote 323 preservation
The structure checker, extended to compare literal preserved targets, found that the OCaml and Kotlin editions dropped `Footnote 323` when section 5.5 was re-cut with inline adapted prose. The footnote anchor remains in Rust and TypeScript; Rust also retains an explicit `@ref` to it. The inventory now records each edition's actual preserved targets. The open question was resolved by restoring the footnote in both editions: the OCaml edition anchors it at the compiled-closure application text (`make-compiled-procedure` operation), and the Kotlin edition anchors it at the compiled-procedure description (`compiled-const`/`compiled-capture` building the tagged `GValue.VObject`). Neither edition has a 5.5.7-style interfacing paragraph, so no `@ref` back-reference was added.
